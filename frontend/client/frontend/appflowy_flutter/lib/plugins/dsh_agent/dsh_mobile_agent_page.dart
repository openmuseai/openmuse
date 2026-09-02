import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_capability_host.dart';
import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_control_host.dart';
import 'package:appflowy/plugins/dsh_agent/appflowy_dsh_file_chooser_host.dart';
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
  });

  final String workspaceId;
  final String workspaceTitle;
  final String accountRef;
  final bool isCloudAccount;
  final bool Function() isCurrentScope;

  @override
  Widget build(BuildContext context) {
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
    );
  }
}
