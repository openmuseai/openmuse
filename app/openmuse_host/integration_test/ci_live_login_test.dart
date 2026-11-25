import 'dart:async';
import 'dart:io';
import '../../../scripts/ci/local_account_file.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_host/main.dart' as application;
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

// Explicit live diagnostic for the normal Desktop boot factory. Credentials
// stay in the authorized local file; assertions never print their contents.
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'native Desktop password form authenticates the authorized CN account',
    (t) async {
      final path = Platform.environment['OPENMUSE_AUTH_ACCOUNT_FILE'];
      if (path == null ||
          Platform.environment['OPENMUSE_E2E_SUPPORT_DIR'] == null) {
        throw StateError(
          'An authorized account file and isolated support directory are required',
        );
      }
      final account = parseLocalAccount(await File(path).readAsString());
      final email = account.email, password = account.password;
      final widget = await t.runAsync(() async {
        final release = await loadInstalledRelease();
        final region = release.regions.singleWhere((r) => r.id == 'cn');
        return application.bootOpenMuseHost(installation: region.bundle!);
      });
      await t.pumpWidget(widget!);
      Future<void> until(bool Function() ready, String label) async {
        final end = DateTime.now().add(const Duration(seconds: 60));
        while (!ready() && DateTime.now().isBefore(end)) {
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 200)),
          );
          await t.pump(const Duration(milliseconds: 200));
        }
        expect(ready(), isTrue, reason: label);
      }

      await until(
        () => find.byType(OpenMuseLoginScreen).evaluate().isNotEmpty,
        'normal Desktop login screen',
      );
      final login = t.widget<OpenMuseLoginScreen>(
        find.byType(OpenMuseLoginScreen).first,
      );
      expect(login.cloudLabel?.contains('cn.openmuseai.com'), isTrue);
      await t.enterText(find.byKey(const ValueKey('auth.email')), email);
      await t.tap(find.byKey(const ValueKey('auth.continue-password')));
      await t.pumpAndSettle();
      await t.enterText(find.byKey(const ValueKey('auth.password')), password);
      final input = t.widget<TextField>(
        find.byKey(const ValueKey('auth.password')),
      );
      expect(
        input.controller!.text == password,
        isTrue,
        reason: 'secure input preserves the complete local credential',
      );
      await t.tap(find.byKey(const ValueKey('auth.submit-password')));
      await t.pump();
      await until(
        () =>
            login.authentication.snapshot.isAuthenticated ||
            login.authentication.snapshot.failureCode != null,
        'server authentication result',
      );
      debugPrint(
        'CI_DESKTOP_AUTH_RESULT phase=${login.authentication.snapshot.phase.name} code=${login.authentication.snapshot.failureCode}',
      );
      expect(login.authentication.snapshot.isAuthenticated, isTrue);
      await until(
        () => find.text('Muse-CI').evaluate().isNotEmpty,
        'installed CI entry on normal Desktop home',
      );
      debugPrint(
        'CI_DESKTOP_AUTH_PASS actual native password form and normal home',
      );
    },
    timeout: const Timeout(Duration(minutes: 3)),
  );
}
