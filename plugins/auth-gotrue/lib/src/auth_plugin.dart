import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

final class OpenMuseGoTruePlugin
    implements OpenMusePlugin, OpenMuseAuthenticationContributor {
  OpenMuseGoTruePlugin({required this.authentication});

  @override
  final OpenMuseAuthenticationController authentication;

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
      return const Center(child: Text('Sign in to OpenMuse'));
    },
  );

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}
