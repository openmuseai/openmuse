import 'dart:convert';
import 'dart:io';

// Must work before pub get, including on a clean build machine.
// ignore: avoid_relative_lib_imports
import '../lib/env/muse_cloud_profile.dart';

/// Emits only a validated public dart-define profile. Never reads auth secrets.
void main(List<String> arguments) {
  try {
    if (arguments.length != 1) {
      throw const FormatException('Expected a mobile deployment JSON path');
    }
    final values =
        (jsonDecode(File(arguments.single).readAsStringSync()) as Map)
            .cast<String, String>();
    const keys = {
      'MUSE_CLOUD_URL',
      'MUSE_CLOUD_GOTRUE_URL',
      'MUSE_CLOUD_WS_URL',
      'MUSE_CLOUD_WEB_URL',
      'MUSE_DSH_PUBLIC_URL',
    };
    if (values.keys.any((key) => !keys.contains(key))) {
      throw const FormatException(
        'Only public mobile endpoint keys are allowed',
      );
    }
    for (final key in keys) {
      final override = Platform.environment[key];
      if (override != null && override.isNotEmpty) values[key] = override;
    }
    final profile = MuseCloudProfile.fromMap(values);
    final dsh = Uri.tryParse(values['MUSE_DSH_PUBLIC_URL'] ?? '');
    if (dsh == null ||
        dsh.scheme != 'https' ||
        dsh.host.isEmpty ||
        dsh.userInfo.isNotEmpty ||
        dsh.hasQuery ||
        dsh.hasFragment ||
        (dsh.path.isNotEmpty && dsh.path != '/')) {
      throw const FormatException('MOBILE_DSH_HTTPS_ORIGIN_REQUIRED');
    }
    stdout.writeln(
      jsonEncode({
        'MUSE_CLOUD_URL': profile.cloud.origin,
        'MUSE_CLOUD_GOTRUE_URL': profile.gotrue.toString(),
        'MUSE_CLOUD_WS_URL': profile.websocket.toString(),
        'MUSE_CLOUD_WEB_URL': profile.web.origin,
        'MUSE_DSH_PUBLIC_URL': dsh.origin,
      }),
    );
  } catch (_) {
    stderr.writeln(
        'Invalid mobile profile: require matching HTTPS Cloud/GoTrue, '
        'native WSS /ws/v1, HTTPS Web/DSH; localhost and secrets are forbidden.');
    exitCode = 64;
  }
}
