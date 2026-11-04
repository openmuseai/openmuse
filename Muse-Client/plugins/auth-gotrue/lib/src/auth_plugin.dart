import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'auth_controller.dart';
import 'auth_localizations.dart';
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
    this.allowAnonymous = false,
    this.onDeleteAccount,
  });

  @override
  final OpenMuseAuthenticationController authentication;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final Uri? termsUri;
  final Uri? privacyUri;
  final bool allowAnonymous;
  final Future<void> Function()? onDeleteAccount;

  Future<void> _confirmDeletion(BuildContext context) async {
    final chinese = Localizations.localeOf(context).languageCode == 'zh';
    var busy = false;
    String? error;
    await showDialog<void>(
      context: context,
      barrierDismissible: false,
      builder: (dialog) => StatefulBuilder(
        builder: (context, update) => AlertDialog(
          title: Text(chinese ? '删除账号' : 'Delete account'),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                chinese
                    ? '这将永久删除账号及关联的云端数据。本地文档可单独管理。此操作无法撤销。'
                    : 'This permanently deletes your account and associated cloud data. Local documents can be managed separately. This cannot be undone.',
              ),
              if (error != null)
                Padding(
                  padding: const EdgeInsets.only(top: 12),
                  child: Text(
                    error!,
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ),
            ],
          ),
          actions: [
            TextButton(
              onPressed: busy ? null : () => Navigator.pop(dialog),
              child: Text(chinese ? '取消' : 'Cancel'),
            ),
            FilledButton(
              onPressed: busy
                  ? null
                  : () async {
                      update(() {
                        busy = true;
                        error = null;
                      });
                      try {
                        await onDeleteAccount!();
                        if (dialog.mounted) Navigator.pop(dialog);
                      } catch (_) {
                        if (dialog.mounted)
                          update(() {
                            busy = false;
                            error = chinese
                                ? '删除未完成，请重试或联系支持。'
                                : 'Deletion did not complete. Please retry or contact support.';
                          });
                      }
                    },
              child: Text(
                busy
                    ? (chinese ? '处理中…' : 'Deleting…')
                    : (chinese ? '永久删除' : 'Delete permanently'),
              ),
            ),
          ],
        ),
      ),
    );
  }

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
      final gotrue = authentication;
      return OpenMuseLoginScreen(
        authentication: authentication,
        cloudLabel: cloudLabel,
        onSettings: onSettings,
        termsUri:
            RuntimeConfigScope.maybeOf(
              context,
            )?.linkUri(LinkId.terms, locale: stringsOf(context).locale) ??
            termsUri,
        privacyUri:
            RuntimeConfigScope.maybeOf(
              context,
            )?.linkUri(LinkId.privacy, locale: stringsOf(context).locale) ??
            privacyUri,
        onAnonymous: allowAnonymous && gotrue is GoTrueAuthenticationController
            ? () => unawaited(gotrue.signInAnonymously())
            : null,
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
      final l10n = openMuseAuthLocalizations(context);
      final identity = authentication.snapshot.identity;
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            l10n.accountSectionTitle,
            style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          Text(
            identity?.email ?? l10n.notSignedIn,
            key: const ValueKey('account-email'),
          ),
          if (identity != null) ...[
            const SizedBox(height: 10),
            OutlinedButton.icon(
              key: const ValueKey('desktop-sign-out'),
              onPressed: authentication.signOut,
              icon: const Icon(Icons.logout),
              label: Text(l10n.signOut),
            ),
            if (onDeleteAccount != null &&
                identity.subject != localAnonymousUserId)
              TextButton.icon(
                key: const ValueKey('desktop-delete-account'),
                onPressed: () => _confirmDeletion(context),
                icon: const Icon(Icons.delete_forever_outlined),
                label: Text(
                  Localizations.localeOf(context).languageCode == 'zh'
                      ? '删除账号'
                      : 'Delete account',
                ),
              ),
          ],
        ],
      );
    },
  );
}
