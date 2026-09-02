import 'dart:convert';
import 'dart:io';

// Standalone build-guard tests: no Flutter engine or pub dependencies required.
void main() {
  final temp = Directory.systemTemp.createTempSync('muse-mobile-config-test-');
  final tool =
      Platform.script.resolve('prepare_mobile_config.dart').toFilePath();
  final valid = <String, String>{
    'MUSE_CLOUD_URL': 'https://openmuseai.com',
    'MUSE_CLOUD_GOTRUE_URL': 'https://openmuseai.com/gotrue',
    'MUSE_CLOUD_WS_URL': 'wss://openmuseai.com/ws/v1',
    'MUSE_CLOUD_WEB_URL': 'https://www.openmuseai.com',
    'MUSE_DSH_PUBLIC_URL': 'https://dsh.openmuseai.com',
  };
  var count = 0;
  void check(
    Map<String, String> profile,
    bool success, [
    Map<String, String> overrides = const {},
  ]) {
    final file = File('${temp.path}/profile.json')
      ..writeAsStringSync(jsonEncode(profile));
    final result = Process.runSync(
      Platform.resolvedExecutable,
      [tool, file.path],
      environment: {for (final key in valid.keys) key: '', ...overrides},
    );
    if ((result.exitCode == 0) != success) {
      throw StateError('Build guard case $count failed: ${result.exitCode}');
    }
    if (!success && (result.stdout as String).isNotEmpty) {
      throw StateError('Invalid configuration must not emit dart defines');
    }
    if (success) {
      final emitted = jsonDecode(result.stdout as String) as Map;
      if (emitted['MUSE_CLOUD_URL'] != 'https://openmuseai.com') {
        throw StateError('Wrong Cloud deployment');
      }
    }
    count++;
  }

  try {
    check(valid, true);
    check(
      valid,
      true,
      {'MUSE_DSH_PUBLIC_URL': 'https://test-dsh.example.com/'},
    );
    check(valid, false, {'MUSE_CLOUD_URL': 'http://localhost'});
    check(valid, false, {'MUSE_CLOUD_URL': 'https://foreign.example.com'});
    check(valid, false, {'MUSE_CLOUD_WS_URL': 'wss://openmuseai.com/ws/v2'});
    check({'MUSE_CLOUD_URL': 'https://openmuseai.com'}, false);
    check({...valid, 'API_TOKEN': 'not-a-real-token'}, false);
    check(
      valid,
      false,
      {'MUSE_DSH_PUBLIC_URL': 'https://user:password@example.com'},
    );
    stdout.writeln('$count mobile build guard tests passed');
  } finally {
    temp.deleteSync(recursive: true);
  }
}
