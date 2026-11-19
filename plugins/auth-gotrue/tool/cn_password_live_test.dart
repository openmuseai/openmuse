// Explicit live check; excluded from the default unit-test discovery.
// OPENMUSE_AUTH_ACCOUNT_FILE points to the operator-authorized local file.
import 'dart:io';
import '../../../scripts/ci/local_account_file.dart';

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
      final account = parseLocalAccount(await File(path).readAsString());
      final email = account.email;
      final password = account.password;
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
