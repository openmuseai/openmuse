import 'dart:async';
import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'workspace_invitations_controller.dart';

Future<void> showWorkspaceInvitationsDialog(
  BuildContext context, {
  required OpenMuseAuthenticationController authentication,
  required AppFlowyCloudWorkspaceService service,
  Future<void> Function()? onJoined,
}) => showDialog<void>(
  context: context,
  builder: (_) => WorkspaceInvitationsDialog(
    authentication: authentication,
    service: service,
    onJoined: onJoined,
  ),
);

class WorkspaceInvitationsDialog extends StatefulWidget {
  const WorkspaceInvitationsDialog({
    super.key,
    required this.authentication,
    required this.service,
    this.onJoined,
  });
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final Future<void> Function()? onJoined;
  @override
  State<WorkspaceInvitationsDialog> createState() =>
      _WorkspaceInvitationsDialogState();
}

class _WorkspaceInvitationsDialogState
    extends State<WorkspaceInvitationsDialog> {
  late final c = WorkspaceInvitationsController(
    authentication: widget.authentication,
    service: widget.service,
    onJoined: widget.onJoined,
  );
  @override
  void initState() {
    super.initState();
    unawaited(c.refresh());
  }

  @override
  void dispose() {
    c.dispose();
    super.dispose();
  }

  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  String status(AppFlowyWorkspaceInvitation value) => switch (value.status) {
    AppFlowyWorkspaceInvitationStatus.pending =>
      c.unconfirmedIds.contains(value.id)
          ? tr('结果待确认', 'Result unconfirmed')
          : tr('待接受', 'Pending'),
    AppFlowyWorkspaceInvitationStatus.accepted => tr('已接受', 'Accepted'),
    AppFlowyWorkspaceInvitationStatus.rejected => tr('已关闭', 'Closed'),
    AppFlowyWorkspaceInvitationStatus.unsupported => tr(
      '暂不支持的邀请状态',
      'Unsupported invitation status',
    ),
  };
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: c,
    builder: (context, _) => Dialog(
      key: const ValueKey('cloud-invitations.dialog'),
      insetPadding: const EdgeInsets.all(16),
      child: SizedBox(
        width: 600,
        height: math.max(
          0,
          math.min(560, MediaQuery.sizeOf(context).height - 32),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 8, 8, 4),
              child: Row(
                children: [
                  Expanded(
                    child: Text(
                      tr('群组邀请', 'Workspace invitations'),
                      style: Theme.of(context).textTheme.titleLarge,
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  IconButton(
                    key: const ValueKey('cloud-invitations.close'),
                    tooltip: tr('关闭', 'Close'),
                    onPressed: () => Navigator.of(context).pop(),
                    icon: const Icon(Icons.close),
                  ),
                ],
              ),
            ),
            if (c.busy)
              const LinearProgressIndicator(
                key: ValueKey('cloud-invitations.busy'),
                minHeight: 2,
              ),
            if (c.failure != null)
              Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 8,
                ),
                child: Text(switch (c.failure) {
                  'account_changed' => tr(
                    '账号已切换，请关闭后重新打开。',
                    'Account changed. Close and reopen this dialog.',
                  ),
                  'unconfirmed' => tr(
                    '接受结果尚未确认，请刷新核对。',
                    'The result is unconfirmed. Refresh to check.',
                  ),
                  'invitation_changed' => tr(
                    '邀请已变更，请刷新后重试。',
                    'The invitation changed. Refresh to check.',
                  ),
                  _ => tr(
                    '暂时无法读取邀请，请刷新重试。',
                    'Invitations are unavailable. Refresh to retry.',
                  ),
                }, key: const ValueKey('cloud-invitations.failure')),
              ),
            Expanded(
              child: c.invitations.isEmpty
                  ? Center(
                      child: Text(
                        c.busy
                            ? tr('正在读取邀请…', 'Loading invitations…')
                            : c.retired
                            ? ''
                            : tr('暂无群组邀请', 'No workspace invitations'),
                        key: const ValueKey('cloud-invitations.empty'),
                      ),
                    )
                  : ListView.separated(
                      itemCount: c.invitations.length,
                      separatorBuilder: (_, _) => const Divider(height: 1),
                      itemBuilder: (context, index) {
                        final value = c.invitations[index];
                        final inviter = value.inviterName ?? value.inviterEmail;
                        return Padding(
                          key: ValueKey('cloud-invitations.row.${value.id}'),
                          padding: const EdgeInsets.all(16),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.stretch,
                            children: [
                              Text(
                                value.workspaceName ??
                                    tr('未命名工作区', 'Untitled workspace'),
                                style: Theme.of(context).textTheme.titleMedium,
                                maxLines: 2,
                                overflow: TextOverflow.ellipsis,
                              ),
                              const SizedBox(height: 4),
                              Text(
                                inviter == null
                                    ? tr(
                                        '邀请者信息暂不可用',
                                        'Inviter details unavailable',
                                      )
                                    : '${tr('邀请者', 'Invited by')}: $inviter',
                                maxLines: 2,
                                overflow: TextOverflow.ellipsis,
                              ),
                              const SizedBox(height: 8),
                              Wrap(
                                alignment: WrapAlignment.spaceBetween,
                                crossAxisAlignment: WrapCrossAlignment.center,
                                spacing: 12,
                                children: [
                                  Text(
                                    status(value),
                                    key: ValueKey(
                                      'cloud-invitations.status.${value.id}',
                                    ),
                                  ),
                                  if (value.status ==
                                      AppFlowyWorkspaceInvitationStatus.pending)
                                    TextButton(
                                      key: ValueKey(
                                        'cloud-invitations.accept.${value.id}',
                                      ),
                                      style: TextButton.styleFrom(
                                        minimumSize: const Size(80, 44),
                                      ),
                                      onPressed: c.canAccept(value)
                                          ? () => c.accept(value.id)
                                          : null,
                                      child: Text(
                                        tr('接受邀请', 'Accept invitation'),
                                      ),
                                    ),
                                ],
                              ),
                            ],
                          ),
                        );
                      },
                    ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              child: Align(
                alignment: Alignment.centerRight,
                child: TextButton.icon(
                  key: const ValueKey('cloud-invitations.refresh'),
                  style: TextButton.styleFrom(minimumSize: const Size(80, 44)),
                  onPressed: c.busy || c.retired ? null : c.refresh,
                  icon: const Icon(Icons.refresh),
                  label: Text(tr('刷新', 'Refresh')),
                ),
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
