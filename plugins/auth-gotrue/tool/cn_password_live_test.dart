// Explicit live check; excluded from the default unit-test discovery.
// OPENMUSE_AUTH_ACCOUNT_FILE points to the operator-authorized local file.
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_io.dart';

void main() {
  test(
    'authorized CN password login, identity and workspace bootstrap',
    () async {
      final path = Platform.environment['OPENMUSE_AUTH_ACCOUNT_FILE'];
      if (path == null)
        throw StateError('OPENMUSE_AUTH_ACCOUNT_FILE is required');
      final lines = (await File(path).readAsLines())
          .map((line) => line.trim())
          .where((line) => line.isNotEmpty)
          .toList();
      Object? decode(String line) {
        try {
          return jsonDecode(line.replaceFirst(RegExp(r',$'), ''));
        } on FormatException {
          return line;
        }
      }

      final i = lines.indexWhere((line) {
        final value = decode(line);
        return value is String &&
            RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]+$').hasMatch(value);
      });
      if (i < 0 || i + 1 >= lines.length) {
        throw StateError('Local account file format is invalid');
      }
      final email = decode(lines[i]) as String;
      final password = decode(lines[i + 1]) as String;
      final origin = Uri.parse('https://cn.openmuseai.com');
      installServiceDnsFallback(
        origins: {origin},
        resolver: Uri.parse('https://dns.alidns.com/resolve'),
      );
      final provider = GoTrueHttpClient(
        config: GoTrueClientConfig(origin: origin.resolve('/gotrue')),
      );
      final session = await provider.signInWithPassword(email, password);
      final user = await provider.currentUser(session.accessToken);
      // Assert booleans so a failed assertion never prints private identifiers.
    expect(user.email.toLowerCase() == email.toLowerCase(), isTrue);
      expect(user.id == session.user.id, isTrue);
      await AppFlowyAccountBootstrapper(cloudOrigin: origin).bootstrap(session);
      // Do not save this diagnostic session or log credentials / token responses.
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
