import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

// Capture public HTTPS responses immediately before running this opt-in test.
// The installed SDK remains responsible for signature, identity, expiry,
// resource integrity and offline restoration. No account session is used.
final class CapturedPublicTransport implements ConfigTransport {
  CapturedPublicTransport(this.responses);
  final Map<String, dynamic> responses;
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    final encoded = responses[uri.toString()];
    if (encoded is! String) throw const SocketException('Not captured');
    final bytes = base64Decode(encoded);
    if (bytes.length > maxBytes) throw const FormatException('Too large');
    return bytes;
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  const fixture = String.fromEnvironment('OPENMUSE_PUBLICATION_FIXTURE');
  test(
    'current public US config activates and restores using installed trust',
    () async {
      final capture = jsonDecode(await File(fixture).readAsString()) as Map;
      final capturedAt = DateTime.parse(capture['capturedAt'] as String);
      expect(
        DateTime.now().difference(capturedAt).abs(),
        lessThan(const Duration(minutes: 15)),
      );
      final bundle = DeploymentBundle.parse(
        await File('assets/deployments/global-us.json').readAsString(),
      );
      final store = MemoryConfigStore();
      final client = RuntimeConfigClient(
        bundle: bundle,
        transport: CapturedPublicTransport(
          Map<String, dynamic>.from(capture['responses'] as Map),
        ),
        store: store,
        build: 11,
        capabilities: const {
          'config.v1',
          'messages.v1',
          'messages.dynamic.v1',
          'settings.v1',
        },
      );
      final active = await client.refresh();
      expect(active.sequence, greaterThan(0));
      expect(active.identity, bundle.snapshot.identity);
      expect(active.expiresAt!.isAfter(DateTime.now()), isTrue);
      final restarted = RuntimeConfigClient(
        bundle: bundle,
        transport: CapturedPublicTransport({}),
        store: store,
        build: 11,
        capabilities: client.capabilities,
      );
      await restarted.restore();
      expect(restarted.current.releaseId, active.releaseId);
      expect(restarted.resources, client.resources);
    },
    skip: fixture.isEmpty
        ? 'Opt-in: capture current public HTTPS responses'
        : false,
  );
}
