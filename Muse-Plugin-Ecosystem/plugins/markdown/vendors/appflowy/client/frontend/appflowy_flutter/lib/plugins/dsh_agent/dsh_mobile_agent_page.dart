import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_capability_host.dart';
import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_control_host.dart';
import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_file_chooser_host.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

/// AppFlowy route wrapper. Shell implementation lives in `muse_dsh_mobile`.
class DshMobileAgentPage extends StatelessWidget {
  const DshMobileAgentPage({
    super.key,
    required this.workspaceId,
    required this.workspaceTitle,
    required this.accountRef,
    required this.isCloudAccount,
    required this.isCurrentScope,
    this.accessToken,
    this.cloudOrigin,
    this.sessionApi,
    this.sessionDeviceId,
  });

  final String workspaceId;
  final String workspaceTitle;
  final String accountRef;
  final bool isCloudAccount;
  final bool Function() isCurrentScope;
  final String? accessToken;
  final String? cloudOrigin;
  final DshSessionApi? sessionApi;
  final String? sessionDeviceId;

  @override
  Widget build(BuildContext context) {
    final origin = (cloudOrigin ?? _cloudOriginFromEnv()).trim();
    final token = (accessToken ?? '').trim();
    final api = sessionApi ??
        ((token.isNotEmpty && origin.isNotEmpty)
            ? DshSessionApi(
                cloudOrigin: Uri.parse(origin),
                accessToken: token,
                post: dshSessionHttpPost,
              )
            : null);
    return DshMobileShellPage(
      scope: DshMobileScope(
        workspaceRef: workspaceId,
        workspaceTitle: workspaceTitle,
        accountRef: accountRef,
        isCloudAccount: isCloudAccount,
        isCurrentScope: isCurrentScope,
      ),
      controlHost: AppFlowyDshControlHost(
        navigator: () => Navigator.of(context),
      ),
      capabilityHost: AppFlowyDshCapabilityHost(),
      fileChooserHost: AppFlowyDshFileChooserHost(
        context: () => context,
      ),
      endpoint: origin.isEmpty ? null : DshRemoteConfig.fromWebUrl('$origin/'),
      sessionApi: api,
      accessToken: token.isEmpty ? null : token,
      sessionDeviceId: sessionDeviceId,
      requireRemoteSession: true,
    );
  }

  static String _cloudOriginFromEnv() {
    try {
      return getIt<AppFlowyCloudSharedEnv>().appflowyCloudConfig.base_url;
    } catch (_) {
      return '';
    }
  }
}
