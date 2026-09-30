import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'login_screen.dart';

final class OpenMuseGoTruePlugin
    implements
        OpenMusePlugin,
        OpenMuseAuthenticationContributor,
        OpenMuseSettingsContributor {
  OpenMuseGoTruePlugin({
    required this.authentication,
    this.cloudLabel,
    this.onSettings,
    this.termsUri,
    this.privacyUri,
  });

  @override
  final OpenMuseAuthenticationController authentication;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final Uri? termsUri;
  final Uri? privacyUri;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.auth.gotrue',
    name: 'OpenMuse GoTrue Authentication',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'network.auth', 'secrets.session'},
  );

  @override
  Future<void> activate(OpenMusePluginContext context) =>
      authentication.restore();

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildAuthenticationGate(
    BuildContext context, {
    required Widget authenticatedChild,
  }) => AnimatedBuilder(
    animation: authentication,
    builder: (context, _) {
      final snapshot = authentication.snapshot;
      if (snapshot.isAuthenticated) return authenticatedChild;
      if (snapshot.phase == OpenMuseAuthenticationPhase.restoring) {
        return const Center(child: CircularProgressIndicator());
      }
      return OpenMuseLoginScreen(
        authentication: authentication,
        cloudLabel: cloudLabel,
        onSettings: onSettings,
        termsUri: termsUri,
        privacyUri: privacyUri,
      );
    },
  );

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;

  @override
  Widget buildSettings(BuildContext context) => AnimatedBuilder(
    animation: authentication,
    builder: (context, _) {
      final identity = authentication.snapshot.identity;
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            '登录账号',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          Text(identity?.email ?? '未登录', key: const ValueKey('account-email')),
          if (identity != null) ...[
            const SizedBox(height: 10),
            OutlinedButton.icon(
              key: const ValueKey('desktop-sign-out'),
              onPressed: authentication.signOut,
              icon: const Icon(Icons.logout),
              label: const Text('退出登录'),
            ),
          ],
        ],
      );
    },
  );
}
