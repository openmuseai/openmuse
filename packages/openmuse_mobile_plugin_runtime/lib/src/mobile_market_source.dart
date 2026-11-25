import 'dart:convert';
import 'dart:io';

import 'package:cryptography/cryptography.dart';

import 'mobile_plugin_store.dart';

abstract interface class MobileMarketSource {
  Future<List<MobileMarketEntry>> list();
  Future<List<int>> artifact(MobileMarketEntry entry);
}

final class BundledMobileMarketSource implements MobileMarketSource {
  const BundledMobileMarketSource({this.officialRelease = false});
  final bool officialRelease;

  @override
  Future<List<MobileMarketEntry>> list() => officialRelease
      ? loadBundledMobileMarket(
          catalogAsset:
              'packages/openmuse_mobile_plugin_runtime/assets/official-market.json',
        )
      : loadBundledMobileMarket();

  @override
  Future<List<int>> artifact(MobileMarketEntry entry) =>
      loadBundledMobileArtifact(entry);
}

/// Signed remote index: {"payload":base64(JSON), "signature":base64(Ed25519)}.
/// The signed payload supplies immutable SHA-256 digests for plugin artifacts.
final class SignedHttpMobileMarketSource implements MobileMarketSource {
  SignedHttpMobileMarketSource({
    required this.indexUri,
    required this.publicKey,
    this.sequenceFile,
    this.allowLoopbackHttpForTesting = false,
  });

  final Uri indexUri;
  final List<int> publicKey;
  final File? sequenceFile;
  final bool allowLoopbackHttpForTesting;
  final Map<String, String> _trustedArtifacts = {};
  int _lastSequence = -1;

  bool _allowed(Uri uri) =>
      uri.scheme == 'https' ||
      (allowLoopbackHttpForTesting &&
          uri.scheme == 'http' &&
          (uri.host == '127.0.0.1' || uri.host == 'localhost'));

  @override
  Future<List<MobileMarketEntry>> list() async {
    if (!_allowed(indexUri) || publicKey.length != 32) {
      throw const FormatException('untrusted Mobile market configuration');
    }
    final envelope = jsonDecode(utf8.decode(await _fetch(indexUri))) as Map;
    final payload = base64Decode(envelope['payload'] as String);
    final signature = base64Decode(envelope['signature'] as String);
    final valid = await Ed25519().verify(
      payload,
      signature: Signature(
        signature,
        publicKey: SimplePublicKey(publicKey, type: KeyPairType.ed25519),
      ),
    );
    if (!valid) throw const FormatException('Mobile market signature invalid');
    final data = jsonDecode(utf8.decode(payload)) as Map;
    if (data['schema'] != 'openmuse-mobile-market/v1' ||
        data['sequence'] is! int ||
        (data['sequence'] as int) < 0 ||
        DateTime.parse(
          data['expiresAt'] as String,
        ).isBefore(DateTime.now().toUtc())) {
      throw const FormatException('Mobile market expired or unsupported');
    }
    final sequence = data['sequence'] as int;
    final persisted = sequenceFile;
    if (persisted != null && await persisted.exists()) {
      final saved = int.tryParse(await persisted.readAsString());
      if (saved != null && saved > _lastSequence) _lastSequence = saved;
    }
    if (sequence < _lastSequence) {
      throw const FormatException('Mobile market catalog rollback rejected');
    }
    final entries = (data['plugins'] as List)
        .map(
          (item) => MobileMarketEntry.fromJson(
            Map<String, Object?>.from(item as Map),
          ),
        )
        .toList(growable: false);
    for (final entry in entries) {
      final artifactUri = Uri.tryParse(entry.bundle);
      if (artifactUri == null ||
          !_allowed(artifactUri) ||
          artifactUri.origin != indexUri.origin ||
          !RegExp(r'^[0-9a-f]{64}$').hasMatch(entry.sha256)) {
        throw const FormatException(
          'Mobile market artifact origin or digest invalid',
        );
      }
      for (final media in entry.media) {
        for (final location in [
          media.uri,
          if (media.poster != null) media.poster!,
        ]) {
          final mediaUri = Uri.tryParse(location);
          if (mediaUri == null ||
              !_allowed(mediaUri) ||
              mediaUri.origin != indexUri.origin) {
            throw const FormatException('Mobile market media origin invalid');
          }
        }
      }
    }
    _trustedArtifacts
      ..clear()
      ..addEntries(entries.map((e) => MapEntry(e.bundle, e.sha256)));
    if (sequence > _lastSequence && persisted != null) {
      await persisted.parent.create(recursive: true);
      final temporary = File('${persisted.path}.tmp');
      await temporary.writeAsString('$sequence', flush: true);
      await temporary.rename(persisted.path);
    }
    _lastSequence = sequence;
    return entries;
  }

  @override
  Future<List<int>> artifact(MobileMarketEntry entry) async {
    final uri = Uri.parse(entry.bundle);
    if (!_allowed(uri) || uri.origin != indexUri.origin) {
      throw const FormatException('artifact origin mismatch');
    }
    if (_trustedArtifacts[entry.bundle] != entry.sha256) {
      throw const FormatException('artifact absent from verified catalog');
    }
    return _fetch(uri);
  }

  Future<List<int>> _fetch(Uri uri) async {
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 10);
    try {
      final request = await client.getUrl(uri);
      request.followRedirects = false;
      final response = await request.close().timeout(
        const Duration(seconds: 15),
      );
      if (response.statusCode != HttpStatus.ok ||
          response.contentLength > 1024 * 1024) {
        throw const FormatException('Mobile market fetch rejected');
      }
      final output = <int>[];
      await for (final chunk in response) {
        output.addAll(chunk);
        if (output.length > 1024 * 1024) {
          throw const FormatException('Mobile market payload too large');
        }
      }
      return output;
    } finally {
      client.close(force: true);
    }
  }
}
