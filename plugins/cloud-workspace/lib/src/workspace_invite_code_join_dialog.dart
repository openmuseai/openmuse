import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'workspace_invite_code_join_controller.dart';

Future<void> showWorkspaceInviteCodeJoinDialog(
  BuildContext context, {
  required OpenMuseAuthenticationController authentication,
  required AppFlowyCloudWorkspaceService service,
  Future<void> Function()? onJoined,
}) => showDialog<void>(
  context: context,
  builder: (_) => WorkspaceInviteCodeJoinDialog(
    authentication: authentication,
    service: service,
    onJoined: onJoined,
  ),
);

class WorkspaceInviteCodeJoinDialog extends StatefulWidget {
  const WorkspaceInviteCodeJoinDialog({
    super.key,
    required this.authentication,
    required this.service,
    this.onJoined,
  });
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final Future<void> Function()? onJoined;
  @override
  State<WorkspaceInviteCodeJoinDialog> createState() =>
      _WorkspaceInviteCodeJoinDialogState();
}

class _WorkspaceInviteCodeJoinDialogState
    extends State<WorkspaceInviteCodeJoinDialog> {
  final input = TextEditingController();
  late final c = WorkspaceInviteCodeJoinController(
    authentication: widget.authentication,
    service: widget.service,
    onJoined: widget.onJoined,
  );
  @override
  void initState() {
    super.initState();
    c.addListener(_changed);
  }

  void _changed() {
    if (c.retired) input.clear();
  }

  @override
  void dispose() {
    c.removeListener(_changed);
    c.dispose();
    input.dispose();
    super.dispose();
  }

  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  String role(AppFlowyWorkspaceRole role) => switch (role) {
    AppFlowyWorkspaceRole.owner => tr('所有者', 'Owner'),
    AppFlowyWorkspaceRole.member => tr('成员', 'Member'),
    AppFlowyWorkspaceRole.guest => tr('访客', 'Guest'),
  };
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: c,
    builder: (context, _) {
      final preview = c.preview;
      return Theme(
        data: Theme.of(context).copyWith(
          visualDensity: VisualDensity.standard,
          materialTapTargetSize: MaterialTapTargetSize.padded,
        ),
        child: Dialog(
          key: const ValueKey('cloud-code-join.dialog'),
          insetPadding: const EdgeInsets.all(16),
          child: SizedBox(
            width: 560,
            height: math.max(
              0,
              math.min(
                520,
                MediaQuery.sizeOf(context).height -
                    MediaQuery.viewInsetsOf(context).vertical -
                    32,
              ),
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
                          tr('通过邀请码加入群组', 'Join a workspace by code'),
                          style: Theme.of(context).textTheme.titleLarge,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                      IconButton(
                        key: const ValueKey('cloud-code-join.close'),
                        tooltip: tr('关闭', 'Close'),
                        onPressed: () => Navigator.pop(context),
                        icon: const Icon(Icons.close),
                      ),
                    ],
                  ),
                ),
                if (c.busy)
                  const LinearProgressIndicator(
                    key: ValueKey('cloud-code-join.busy'),
                  ),
                Expanded(
                  child: SingleChildScrollView(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        TextField(
                          key: const ValueKey('cloud-code-join.input'),
                          controller: input,
                          obscureText: true,
                          enableSuggestions: false,
                          autocorrect: false,
                          enabled: !c.retired && !c.busy,
                          maxLength: 16,
                          decoration: InputDecoration(
                            labelText: tr('邀请码', 'Invitation code'),
                          ),
                          onChanged: (_) => c.draftChanged(),
                          onSubmitted: c.retired || c.busy
                              ? null
                              : (s) => c.read(s),
                        ),
                        Align(
                          alignment: Alignment.centerRight,
                          child: TextButton(
                            key: const ValueKey('cloud-code-join.preview'),
                            style: TextButton.styleFrom(
                              minimumSize: const Size(88, 44),
                            ),
                            onPressed: c.retired || c.busy
                                ? null
                                : () => c.read(input.text),
                            child: Text(tr('预览群组', 'Preview workspace')),
                          ),
                        ),
                        if (c.failure != null)
                          Text(switch (c.failure) {
                            'account_changed' => tr(
                              '账号已切换，请关闭后重新打开。',
                              'Account changed. Close and reopen.',
                            ),
                            'unconfirmed' => tr(
                              '加入结果尚未确认，请查询群组，暂不重复加入。',
                              'Joining is unconfirmed. Review the workspace before trying again.',
                            ),
                            'catalog_unavailable' => tr(
                              '成员角色已确认，暂时无法刷新目录，请关闭后刷新。',
                              'Membership confirmed. Close and refresh the directory.',
                            ),
                            _ => tr(
                              '邀请码不可用或暂时无法读取，请核对后重试。',
                              'Code unavailable. Check the code and read again.',
                            ),
                          }, key: const ValueKey('cloud-code-join.failure')),
                        if (preview != null) ...[
                          Text(
                            preview.workspaceName,
                            key: const ValueKey('cloud-code-join.workspace'),
                            style: Theme.of(context).textTheme.titleMedium,
                          ),
                          Text('${tr('所有者', 'Owner')}: ${preview.ownerName}'),
                          Text(
                            '${preview.memberCount} ${tr('位群组成员', 'workspace members')}',
                          ),
                          if (preview.isMember || c.joinedRole != null)
                            Text(
                              c.joinedRole == null
                                  ? tr(
                                      '你已属于此群组。',
                                      'You already belong to this workspace.',
                                    )
                                  : '${tr('已加入，当前角色', 'Joined with role')}: ${role(c.joinedRole!)}',
                              key: const ValueKey('cloud-code-join.joined'),
                            ),
                          if (!preview.isMember && c.joinedRole == null)
                            TextButton(
                              key: const ValueKey('cloud-code-join.accept'),
                              style: TextButton.styleFrom(
                                minimumSize: const Size(88, 44),
                              ),
                              onPressed: c.canJoin(preview)
                                  ? () => c.join(preview)
                                  : null,
                              child: Text(tr('确认加入', 'Join workspace')),
                            ),
                        ],
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      );
    },
  );
}
