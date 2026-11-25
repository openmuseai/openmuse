import 'dart:convert';
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:openmuse_runtime_config/openmuse_runtime_config_core.dart';

// Verify actual HTTPS publications with the same verifier/cache as the app.
Future<void> main(List<String> args) async {
  if (args.length != 2) {
    stderr.writeln(
      'Usage: verify_public_release installed-release.json report.json',
    );
    exitCode = 64;
    return;
  }
  final results = <Map<String, Object?>>[];
  final client = http.Client();
  try {
    final release = InstalledRelease.parse(await File(args[0]).readAsString());
    for (final region in release.regions.where((r) => r.available)) {
      final bundle = region.bundle!;
      final cache = MemoryConfigStore();
      RuntimeConfigClient runtime() => RuntimeConfigClient(
        bundle: bundle,
        transport: HttpConfigTransport(
          client: client,
          timeout: const Duration(seconds: 20),
        ),
        store: cache,
        build: 17,
        capabilities: bundle.snapshot.requiredCapabilities.toSet(),
      );
      final live = runtime();
      final snapshot = await live.refresh();
      if (snapshot.sequence < 1 || live.networkUnavailable) {
        throw StateError(
          'No authenticated publication accepted for ${region.id}',
        );
      }
      final restored = runtime();
      await restored.restore();
      if (restored.current.releaseId != snapshot.releaseId ||
          restored.networkUnavailable) {
        throw StateError('Authenticated cache restoration failed');
      }
      results.add({
        'region': region.id,
        'releaseId': snapshot.releaseId,
        'sequence': snapshot.sequence,
        'expiresAt': snapshot.expiresAt?.toIso8601String(),
        'signatureAndInstalledTrustVerified': true,
        'cacheRestorationVerified': true,
      });
    }
  } catch (error) {
    results.add({'error': error.toString()});
    exitCode = 1;
  } finally {
    client.close();
    await File(args[1]).writeAsString(
      const JsonEncoder.withIndent('  ').convert({
        'checkedAt': DateTime.now().toUtc().toIso8601String(),
        'passed': exitCode == 0,
        'regions': results,
      }),
    );
    stdout.writeln(
      'Public release verification: ${exitCode == 0 ? 'PASS' : 'FAIL'}',
    );
  }
}
