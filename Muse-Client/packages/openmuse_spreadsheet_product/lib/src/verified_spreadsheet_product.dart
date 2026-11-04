import 'dart:convert';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:cryptography/cryptography.dart';
import 'package:spreadsheet_editor/spreadsheet_editor.dart';

/// A verified, immutable product snapshot. It contains no document state or
/// executable editor code and can be loaded from an installed directory or an
/// app-bundled bootstrap resource using the same signature policy.
final class VerifiedSpreadsheetProduct {
  const VerifiedSpreadsheetProduct({
    required this.product,
    required this.chrome,
    required this.commands,
    required this.enabledFeatures,
  });

  final Map<String, dynamic> product;
  final SpreadsheetChromeDefinition chrome;
  final Map<String, Map<String, dynamic>> commands;
  final Set<String> enabledFeatures;

  static Future<VerifiedSpreadsheetProduct> loadBundle({
    required Uint8List bundle,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    String? expectedVersion,
  }) async {
    if (bundle.length > 8 * 1024 * 1024) {
      throw const FormatException('Spreadsheet product bundle too large');
    }
    final archive = ZipDecoder().decodeBytes(bundle, verify: true);
    final entries = <String, ArchiveFile>{};
    if (archive.length > 256 ||
        archive.any(
          (file) =>
              !file.isFile ||
              file.size > 4 * 1024 * 1024 ||
              file.isSymbolicLink ||
              file.name.startsWith('/') ||
              file.name.contains('\\') ||
              file.name.split('/').contains('..') ||
              entries.putIfAbsent(file.name, () => file) != file,
        )) {
      throw const FormatException('Spreadsheet product archive invalid');
    }
    return load(
      read: (path) async {
        final file = entries[path];
        if (file == null || !file.isFile) {
          throw FormatException('Spreadsheet product resource missing: $path');
        }
        final content = file.content;
        if (content is Uint8List) return content;
        if (content is List<int>) return Uint8List.fromList(content);
        throw FormatException('Spreadsheet resource unreadable: $path');
      },
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: expectedVersion,
    );
  }

  static Future<VerifiedSpreadsheetProduct> load({
    required Future<Uint8List> Function(String relativePath) read,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    String? expectedVersion,
  }) async {
    final catalogBytes = await read('catalog.json');
    final signature = _object(
      jsonDecode(utf8.decode(await read('catalog.signature.json'))),
    );
    if (signature['algorithm'] != 'Ed25519' ||
        signature['keyId'] != expectedKeyId ||
        signature['signature'] is! String) {
      throw const FormatException('Spreadsheet product signature metadata invalid');
    }
    final der = base64Decode(
      trustedPublicKeyPem
          .replaceAll(RegExp(r'-----[^-]+-----'), '')
          .replaceAll(RegExp(r'\s+'), ''),
    );
    const prefix = [
      0x30,
      0x2a,
      0x30,
      0x05,
      0x06,
      0x03,
      0x2b,
      0x65,
      0x70,
      0x03,
      0x21,
      0x00,
    ];
    if (der.length != 44 ||
        !List.generate(
          prefix.length,
          (index) => der[index] == prefix[index],
        ).every((value) => value)) {
      throw const FormatException(
        'Spreadsheet product public key is not Ed25519 SPKI',
      );
    }
    final verified = await Ed25519().verify(
      catalogBytes,
      signature: Signature(
        base64Decode(signature['signature'] as String),
        publicKey: SimplePublicKey(der.sublist(12), type: KeyPairType.ed25519),
      ),
    );
    if (!verified)
      throw const FormatException('Spreadsheet product signature invalid');

    final catalog = _object(jsonDecode(utf8.decode(catalogBytes)));
    final version = catalog['version'];
    if (catalog['profile'] != 'openmuse-signed-resources/1' ||
        catalog['productId'] != 'io.openmuse/spreadsheet' ||
        version is! String ||
        !RegExp(r'^\d+\.\d+\.\d+$').hasMatch(version) ||
        (expectedVersion != null && version != expectedVersion) ||
        catalog['entries'] is! List) {
      throw const FormatException('Spreadsheet product catalog invalid');
    }
    final resources = <String, Map<String, dynamic>>{};
    final seen = <String>{};
    for (final raw in catalog['entries'] as List) {
      final item = _object(raw);
      final path = item['path'];
      if (path is! String ||
          path.startsWith('/') ||
          path.contains('\\') ||
          path.split('/').contains('..') ||
          !seen.add(path)) {
        throw const FormatException('Spreadsheet product catalog path invalid');
      }
      final bytes = await read(path);
      if (bytes.length != item['bytes'] ||
          sha256.convert(bytes).toString() != item['sha256']) {
        throw FormatException('Spreadsheet product resource digest invalid: $path');
      }
      if (path.endsWith('.json') && path.startsWith('product/')) {
        resources[path] = _object(jsonDecode(utf8.decode(bytes)));
      }
    }
    Map<String, dynamic> resource(String path) =>
        resources[path] ??
        (throw FormatException('Spreadsheet resource missing: $path'));

    final product = resource('product/product.json');
    if (product['profile'] != 'openmuse-spreadsheet-product/1' ||
        product['id'] != 'io.openmuse/spreadsheet' ||
        product['version'] != version ||
        _object(product['requires'])['spreadsheetAbiMajor'] != 1 ||
        _object(product['requires'])['surfaceMajor'] != 1) {
      throw const FormatException('Spreadsheet product API incompatible');
    }
    final paths = _object(product['resources']);
    final chrome = resource(paths['chrome'] as String);
    final iconData = paths['icons'] is String
        ? resource(paths['icons'] as String)
        : <String, dynamic>{};
    final commandData = resource(paths['commands'] as String);
    final featureData = resource(paths['features'] as String);
    if (chrome['profile'] != 'openmuse-spreadsheet-chrome/1' ||
        (iconData.isNotEmpty && iconData['profile'] != 'openmuse-spreadsheet-icons/1') ||
        commandData['profile'] != 'openmuse-spreadsheet-commands/1' ||
        featureData['profile'] != 'openmuse-spreadsheet-features/1') {
      throw const FormatException('Spreadsheet product resource profile incompatible');
    }
    return VerifiedSpreadsheetProduct(
      product: product,
      chrome: SpreadsheetChromeDefinition.fromJson(chrome, iconData: iconData),
      commands: {
        for (final raw in commandData['commands'] as List)
          _object(raw)['id'] as String: _object(raw),
      },
      enabledFeatures: {
        for (final raw in featureData['features'] as List)
          if (_object(raw)['defaultEnabled'] == true)
            _object(raw)['id'] as String,
      },
    );
  }
}

Map<String, dynamic> _object(Object? value) {
  if (value is! Map) throw const FormatException('Expected JSON object');
  return Map<String, dynamic>.from(value);
}
