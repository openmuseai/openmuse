import 'dart:async';
import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'workspace_access_controller.dart';
import 'owner_invitations_panel.dart';
import 'workspace_invite_code_panel.dart';

Future<void> showWorkspaceAccessDialog(
  BuildContext context, {
  required OpenMuseAuthenticationController authentication,
  required AppFlowyCloudWorkspaceService service,
  required String workspaceId,
  required String title,
  String? viewId,
  List<String> ancestors = const [],
}) async {
  final controller = WorkspaceAccessController(
    authentication: authentication,
    service: service,
    workspaceId: workspaceId,
    viewId: viewId,
    ancestors: ancestors,
  );
  try {
    await showDialog<void>(
      context: context,
      builder: (_) =>
          WorkspaceAccessDialog(controller: controller, title: title),
    );
  } finally {
    controller.dispose();
  }
}

/// Theme-driven surface over the original workspace access model. It contains
/// no content engine or local ACL and can be used by any Cloud view editor.
class WorkspaceAccessDialog extends StatefulWidget {
  const WorkspaceAccessDialog({
    super.key,
    required this.controller,
    required this.title,
  });
  final WorkspaceAccessController controller;
  final String title;
  @override
  State<WorkspaceAccessDialog> createState() => _WorkspaceAccessDialogState();
}

class _WorkspaceAccessDialogState extends State<WorkspaceAccessDialog> {
  final _email = TextEditingController();
  final _emailFocus = FocusNode();
  final _form = GlobalKey<FormState>();
  AppFlowyAccessLevel _access = AppFlowyAccessLevel.readOnly;
  AppFlowyWorkspaceRole _role = AppFlowyWorkspaceRole.member;
  WorkspaceAccessController get c => widget.controller;
  bool get page => c.viewId != null;
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  @override
  void initState() {
    super.initState();
    unawaited(c.refresh());
  }

  @override
  void dispose() {
    _email.dispose();
    _emailFocus.dispose();
    super.dispose();
  }

  String roleName(AppFlowyWorkspaceRole role) => switch (role) {
    AppFlowyWorkspaceRole.owner => tr('所有者', 'Owner'),
    AppFlowyWorkspaceRole.member => tr('成员', 'Member'),
    AppFlowyWorkspaceRole.guest => tr('访客', 'Guest'),
  };
  String levelName(AppFlowyAccessLevel level) => switch (level) {
    AppFlowyAccessLevel.readOnly => tr('只读', 'Read only'),
    AppFlowyAccessLevel.comment => tr('可评论', 'Can comment'),
    AppFlowyAccessLevel.edit => tr('可编辑', 'Can edit'),
    AppFlowyAccessLevel.full => tr('完全访问', 'Full access'),
  };
  Widget emailField() => TextFormField(
    key: const ValueKey('cloud-access.email'),
    controller: _email,
    focusNode: _emailFocus,
    enabled: !c.busy,
    onChanged: (_) => setState(() {}),
    keyboardType: TextInputType.emailAddress,
    decoration: InputDecoration(labelText: tr('邮箱', 'Email')),
    validator: (value) =>
        RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]+$').hasMatch(value?.trim() ?? '')
        ? null
        : tr('请输入有效邮箱', 'Enter a valid email'),
  );
  Widget form() => LayoutBuilder(
    builder: (context, constraints) {
      final controls = Wrap(
        spacing: 8,
        runSpacing: 8,
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          if (page)
            DropdownButton<AppFlowyAccessLevel>(
              key: const ValueKey('cloud-access.level'),
              value: _access,
              items: [
                for (final level in AppFlowyAccessLevel.values)
                  DropdownMenuItem(
                    key: ValueKey('cloud-access.level.${level.wire}'),
                    value: level,
                    child: Text(levelName(level)),
                  ),
              ],
              onChanged: c.busy
                  ? null
                  : (value) => setState(() => _access = value!),
            )
          else
            DropdownButton<AppFlowyWorkspaceRole>(
              key: const ValueKey('cloud-access.role'),
              value: _role,
              items: [
                for (final role in [
                  AppFlowyWorkspaceRole.member,
                  AppFlowyWorkspaceRole.guest,
                ])
                  DropdownMenuItem(value: role, child: Text(roleName(role))),
              ],
              onChanged: c.busy
                  ? null
                  : (value) => setState(() => _role = value!),
            ),
          FilledButton(
            key: const ValueKey('cloud-access.submit'),
            style: FilledButton.styleFrom(minimumSize: const Size(72, 44)),
            onPressed: !c.canMutateTarget(_email.text)
                ? null
                : () async {
                    if (!_form.currentState!.validate()) return;
                    if (page) {
                      await c.share(_email.text.trim(), _access);
                    } else {
                      await c.invite(_email.text.trim(), _role);
                    }
                  },
            child: Text(page ? tr('分享', 'Share') : tr('邀请', 'Invite')),
          ),
          if (!page)
            OutlinedButton(
              key: const ValueKey('cloud-access.update-role'),
              style: OutlinedButton.styleFrom(minimumSize: const Size(72, 44)),
              onPressed: !c.canMutateTarget(_email.text)
                  ? null
                  : () async {
                      if (!_form.currentState!.validate()) return;
                      await c.changeRole(_email.text.trim(), _role);
                    },
              child: Text(tr('更新角色', 'Update role')),
            ),
        ],
      );
      return Form(
        key: _form,
        child: constraints.maxWidth < 560
            ? Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [emailField(), const SizedBox(height: 12), controls],
              )
            : Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Expanded(child: emailField()),
                  const SizedBox(width: 12),
                  controls,
                ],
              ),
      );
    },
  );
  Widget tile({
    required Key key,
    required String title,
    required String subtitle,
    Widget? actions,
  }) => Padding(
    key: key,
    padding: const EdgeInsets.symmetric(vertical: 12),
    child: LayoutBuilder(
      builder: (context, constraints) {
        final text = Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(title, maxLines: 2, overflow: TextOverflow.ellipsis),
            const SizedBox(height: 4),
            Text(
              subtitle,
              style: Theme.of(context).textTheme.bodySmall,
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
          ],
        );
        return constraints.maxWidth < 520
            ? Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  text,
                  if (actions != null)
                    Align(alignment: Alignment.centerRight, child: actions),
                ],
              )
            : Row(
                children: [
                  Expanded(child: text),
                  if (actions != null) ...[const SizedBox(width: 12), actions],
                ],
              );
      },
    ),
  );
  Widget personActions(AppFlowySharedPerson person) => person.viewId != c.viewId
      ? TextButton.icon(
          key: ValueKey('cloud-access.source.${person.viewId}.${person.email}'),
          style: TextButton.styleFrom(minimumSize: const Size(80, 44)),
          onPressed: c.busy || !c.canShare
              ? null
              : () async {
                  await showWorkspaceAccessDialog(
                    context,
                    authentication: c.authentication,
                    service: c.service,
                    workspaceId: c.workspaceId,
                    viewId: person.viewId,
                    title: person.sourceViewName?.isNotEmpty == true
                        ? person.sourceViewName!
                        : tr('上级页面', 'Ancestor page'),
                  );
                  if (mounted) await c.refresh();
                },
          icon: const Icon(Icons.manage_accounts_outlined, size: 18),
          label: Text(tr('管理来源', 'Manage source')),
        )
      : Wrap(
          spacing: 4,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            if (person.role == AppFlowyWorkspaceRole.guest)
              DropdownButton<AppFlowyAccessLevel>(
                key: ValueKey('cloud-access.person-permission.${person.email}'),
                value: person.access,
                items: [
                  for (final level in AppFlowyAccessLevel.values)
                    DropdownMenuItem(
                      key: ValueKey(
                        'cloud-access.person-level.${person.email}.${level.wire}',
                      ),
                      value: level,
                      child: Text(levelName(level)),
                    ),
                ],
                onChanged: !c.canMutateTarget(person.email) || !c.canShare
                    ? null
                    : (level) => c.share(person.email, level!),
              ),
            IconButton(
              key: ValueKey('cloud-access.revoke.${person.email}'),
              tooltip: tr('撤销页面分享', 'Revoke page sharing'),
              onPressed: !c.canMutateTarget(person.email) || !c.canShare
                  ? null
                  : () => c.revoke(person),
              icon: const Icon(Icons.person_remove_outlined),
            ),
          ],
        );
  Widget memberActions(AppFlowyWorkspaceMember member) =>
      PopupMenuButton<String>(
        key: ValueKey('cloud-access.member-menu.${member.email}'),
        enabled: c.canMutateTarget(member.email),
        onSelected: (action) => action == 'remove'
            ? c.removeMember(member.email)
            : c.changeRole(
                member.email,
                action == 'member'
                    ? AppFlowyWorkspaceRole.member
                    : AppFlowyWorkspaceRole.guest,
              ),
        itemBuilder: (_) => [
          PopupMenuItem(
            key: const ValueKey('cloud-access.member-action.member'),
            value: 'member',
            child: Text(tr('设为成员', 'Set as member')),
          ),
          PopupMenuItem(
            key: const ValueKey('cloud-access.member-action.guest'),
            value: 'guest',
            child: Text(tr('设为访客', 'Set as guest')),
          ),
          PopupMenuItem(
            key: const ValueKey('cloud-access.member-action.remove'),
            value: 'remove',
            child: Text(tr('移出群组', 'Remove from workspace')),
          ),
        ],
      );
  List<Widget> entries() => page
      ? [
          for (final person in c.people)
            tile(
              key: ValueKey(
                'cloud-access.person.${person.email}.${person.viewId}',
              ),
              title: person.email,
              subtitle:
                  '${levelName(person.access)} · ${roleName(person.role)}'
                  '${person.pending ? ' · ${tr('待接受', 'Pending')}' : ''}'
                  '${person.viewId != c.viewId ? ' · ${tr('继承自上级页面', 'Inherited from ancestor')}${person.sourceViewName?.isNotEmpty == true ? '「${person.sourceViewName}」' : ''}' : ''}',
              actions: personActions(person),
            ),
          if (!c.busy && c.people.isEmpty && c.canShare)
            Text(tr('尚无单独分享', 'No explicit page grants')),
          if (!c.busy && c.role == AppFlowyWorkspaceRole.guest)
            Text(
              tr('仅群组成员可以管理分享。', 'Only workspace members can manage sharing.'),
            ),
        ]
      : [
          for (final member in c.members)
            tile(
              key: ValueKey('cloud-access.member.${member.email}'),
              title: member.email,
              subtitle: roleName(member.role),
              actions:
                  !c.canManageMembers ||
                      member.role == AppFlowyWorkspaceRole.owner
                  ? null
                  : memberActions(member),
            ),
        ];
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: c,
    builder: (context, _) => Theme(
      data: Theme.of(context).copyWith(
        visualDensity: VisualDensity.standard,
        materialTapTargetSize: MaterialTapTargetSize.padded,
      ),
      child: Dialog(
        key: const ValueKey('cloud-access.dialog'),
        insetPadding: const EdgeInsets.all(16),
        child: SizedBox(
          width: 720,
          height: math.max(
            0.0,
            math.min(
              560.0,
              MediaQuery.sizeOf(context).height -
                  MediaQuery.viewInsetsOf(context).vertical -
                  32,
            ),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(16, 12, 8, 8),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        '${page ? tr('分享', 'Share') : tr('群组成员', 'Workspace members')} · ${widget.title}',
                        style: Theme.of(context).textTheme.titleLarge,
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    IconButton(
                      key: const ValueKey('cloud-access.dismiss'),
                      tooltip: tr('关闭', 'Close'),
                      onPressed: () => Navigator.pop(context),
                      icon: const Icon(Icons.close),
                    ),
                  ],
                ),
              ),
              if (c.busy)
                const LinearProgressIndicator(
                  key: ValueKey('cloud-access.busy'),
                  minHeight: 2,
                ),
              Expanded(
                child: CustomScrollView(
                  key: const ValueKey('cloud-access.contents'),
                  slivers: [
                    SliverPadding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      sliver: SliverToBoxAdapter(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            if (c.failure != null)
                              Padding(
                                padding: const EdgeInsets.symmetric(
                                  vertical: 8,
                                ),
                                child: Text(switch (c.failure) {
                                  'account_changed' => tr(
                                    '账号已切换，请关闭后重新打开。',
                                    'Account changed. Close and reopen this dialog.',
                                  ),
                                  'unconfirmed' => tr(
                                    '操作结果尚未确认。请刷新核对，暂不重复提交此目标。',
                                    'The result is unconfirmed. Refresh to check before submitting this target again.',
                                  ),
                                  'refresh_unavailable' => tr(
                                    '服务器已接受操作，暂时无法读取最新权限。请刷新核对。',
                                    'The server accepted the operation. Refresh to read the latest access details.',
                                  ),
                                  'review_required' => tr(
                                    '此目标的先前操作需要核对，请刷新。',
                                    'Review the previous operation for this target. Refresh to check.',
                                  ),
                                  'rejected' => tr(
                                    '操作未被接受，请核对输入和当前权限。',
                                    'The operation was not accepted. Check the input and current access.',
                                  ),
                                  _ => tr(
                                    '暂时无法读取权限，请刷新重试。',
                                    'Access details are unavailable. Refresh to retry.',
                                  ),
                                }, key: const ValueKey('cloud-access.failure')),
                              ),
                            for (final intent in c.pendingMutations)
                              Padding(
                                key: ValueKey(
                                  'cloud-access.pending.${intent.kind.name}.${intent.targets.join(',')}',
                                ),
                                padding: const EdgeInsets.symmetric(
                                  vertical: 6,
                                ),
                                child: Text(
                                  '${intent.targets.join(', ')} · ${intent.status == AppFlowyAccessMutationStatus.accepted ? tr('已接受，等待核对', 'Accepted, awaiting review') : tr('等待核对', 'Awaiting review')}',
                                  maxLines: 3,
                                  overflow: TextOverflow.ellipsis,
                                ),
                              ),
                            if (page)
                              Padding(
                                padding: const EdgeInsets.symmetric(
                                  vertical: 8,
                                ),
                                child: Text(
                                  tr(
                                    '群组成员继承工作区访问权限。单页分享不会降低群组成员的权限；访客按页面授权。',
                                    'Members inherit workspace access. Page sharing cannot reduce member access; guests use page grants.',
                                  ),
                                  style: Theme.of(context).textTheme.bodySmall,
                                ),
                              ),
                            if (page ? c.canShare : c.canManageMembers) form(),
                            const SizedBox(height: 12),
                          ],
                        ),
                      ),
                    ),
                    SliverPadding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      sliver: SliverList.list(children: entries()),
                    ),
                    if (!page && c.canManageMembers)
                      SliverPadding(
                        padding: const EdgeInsets.symmetric(horizontal: 16),
                        sliver: SliverToBoxAdapter(
                          child: OwnerInvitationsPanel(
                            key: ValueKey('cloud-sent.owner.${c.workspaceId}'),
                            access: c,
                          ),
                        ),
                      ),
                    if (!page && c.canManageMembers)
                      SliverPadding(
                        padding: const EdgeInsets.symmetric(horizontal: 16),
                        sliver: SliverToBoxAdapter(
                          child: WorkspaceInviteCodePanel(access: c),
                        ),
                      ),
                    if (!page)
                      SliverPadding(
                        padding: const EdgeInsets.all(16),
                        sliver: SliverToBoxAdapter(
                          child: Text(
                            tr(
                              '此列表显示所有者和成员。访客在页面分享中管理，也可输入邮箱更新已有群组角色。',
                              'This list shows owners and members. Manage guests through page sharing, or enter an email to update an existing role.',
                            ),
                            style: Theme.of(context).textTheme.bodySmall,
                          ),
                        ),
                      ),
                  ],
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(8),
                child: Wrap(
                  alignment: WrapAlignment.end,
                  spacing: 8,
                  children: [
                    TextButton(
                      key: const ValueKey('cloud-access.refresh'),
                      style: TextButton.styleFrom(
                        minimumSize: const Size(72, 44),
                      ),
                      onPressed: c.busy || c.retired ? null : c.refresh,
                      child: Text(tr('刷新', 'Refresh')),
                    ),
                    TextButton(
                      key: const ValueKey('cloud-access.close'),
                      style: TextButton.styleFrom(
                        minimumSize: const Size(72, 44),
                      ),
                      onPressed: () => Navigator.pop(context),
                      child: Text(tr('关闭', 'Close')),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    ),
  );
}
