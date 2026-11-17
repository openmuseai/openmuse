import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'owner_invitations_controller.dart';
import 'workspace_access_controller.dart';

class OwnerInvitationsPanel extends StatefulWidget {
  const OwnerInvitationsPanel({super.key, required this.access});
  final WorkspaceAccessController access;
  @override
  State<OwnerInvitationsPanel> createState() => _OwnerInvitationsPanelState();
}

class _OwnerInvitationsPanelState extends State<OwnerInvitationsPanel> {
  late final OwnerInvitationsController c;
  final _filter = TextEditingController();
  @override
  void initState() {
    super.initState();
    c = OwnerInvitationsController(widget.access);
    unawaited(c.refresh());
  }

  @override
  void dispose() {
    c.dispose();
    _filter.dispose();
    super.dispose();
  }

  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  String role(AppFlowyWorkspaceRole? r) => switch (r) {
    AppFlowyWorkspaceRole.owner => tr('所有者', 'Owner'),
    AppFlowyWorkspaceRole.member => tr('成员', 'Member'),
    AppFlowyWorkspaceRole.guest => tr('访客', 'Guest'),
    null => tr('暂不支持的角色', 'Unsupported role'),
  };
  String status(AppFlowyWorkspaceInvitationStatus s) => switch (s) {
    AppFlowyWorkspaceInvitationStatus.pending => tr('待接受', 'Pending'),
    AppFlowyWorkspaceInvitationStatus.accepted => tr('已接受', 'Accepted'),
    AppFlowyWorkspaceInvitationStatus.rejected => tr('已结束', 'Ended'),
    AppFlowyWorkspaceInvitationStatus.unsupported => tr(
      '暂不支持的状态',
      'Unsupported status',
    ),
  };
  Widget entry(AppFlowyOwnerWorkspaceInvitation row) => Padding(
    key: ValueKey('cloud-sent.row.${row.id}'),
    padding: const EdgeInsets.symmetric(vertical: 8),
    child: LayoutBuilder(
      builder: (context, box) {
        final info = Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(row.email, maxLines: 2, overflow: TextOverflow.ellipsis),
            Text(
              '${status(row.status)} · ${role(row.role)}',
              key: ValueKey('cloud-sent.status.${row.id}'),
              style: Theme.of(context).textTheme.bodySmall,
            ),
            Text(
              row.memberRoleId == null
                  ? tr('当前未加入', 'Not currently a member')
                  : '${tr('当前成员角色', 'Current membership')}: ${role(row.memberRole)}',
              key: ValueKey('cloud-sent.member.${row.id}'),
              style: Theme.of(context).textTheme.bodySmall,
            ),
            Text(
              MaterialLocalizations.of(
                context,
              ).formatShortDate(row.createdAt.toLocal()),
              style: Theme.of(context).textTheme.bodySmall,
            ),
          ],
        );
        final action = row.status == AppFlowyWorkspaceInvitationStatus.pending
            ? TextButton(
                key: ValueKey('cloud-sent.cancel.${row.id}'),
                style: TextButton.styleFrom(minimumSize: const Size(88, 44)),
                onPressed: c.canCancel(row) ? () => c.cancel(row) : null,
                child: Text(tr('撤回邀请', 'Withdraw invitation')),
              )
            : null;
        if (box.maxWidth < 440)
          return Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              info,
              if (action != null)
                Align(alignment: Alignment.centerRight, child: action),
            ],
          );
        return Row(
          children: [
            Expanded(child: info),
            if (action != null) action,
          ],
        );
      },
    ),
  );
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: c,
    builder: (context, _) {
      if (c.unsupported) return const SizedBox.shrink();
      if (c.retired)
        return Text(
          tr(
            '邀请管理会话已结束，请重新打开。',
            'Invitation management ended. Reopen the dialog.',
          ),
          key: const ValueKey('cloud-sent.retired'),
        );
      return Column(
        key: const ValueKey('cloud-sent.panel'),
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Divider(),
          Text(
            tr('已发邀请', 'Sent invitations'),
            style: Theme.of(context).textTheme.titleMedium,
          ),
          Text(
            tr(
              '撤回只会结束待接受邀请。已加入成员仍通过原成员管理调整权限。',
              'Withdrawal ends a pending invitation. Manage existing membership through workspace members.',
            ),
            style: Theme.of(context).textTheme.bodySmall,
          ),
          const SizedBox(height: 8),
          TextField(
            key: const ValueKey('cloud-sent.filter'),
            controller: _filter,
            enabled: !c.busy,
            keyboardType: TextInputType.emailAddress,
            decoration: InputDecoration(
              labelText: tr('按邮箱筛选邀请', 'Filter invitations by email'),
            ),
            onSubmitted: (_) => c.refresh(filter: _filter.text),
          ),
          Align(
            alignment: Alignment.centerRight,
            child: TextButton(
              key: const ValueKey('cloud-sent.refresh'),
              style: TextButton.styleFrom(minimumSize: const Size(88, 44)),
              onPressed: c.busy ? null : () => c.refresh(filter: _filter.text),
              child: Text(tr('查询邀请', 'Find invitations')),
            ),
          ),
          if (c.busy)
            const LinearProgressIndicator(key: ValueKey('cloud-sent.busy')),
          if (c.failure != null)
            Text(switch (c.failure) {
              'invalid_filter' => tr(
                '请输入有效邮箱，或清空后查询全部邀请。',
                'Enter a valid email, or clear it to read all invitations.',
              ),
              'unconfirmed' => tr(
                '撤回结果尚未确认，请查询当前邀请后再操作。',
                'Withdrawal is unconfirmed. Read its current state before acting again.',
              ),
              _ => tr(
                '暂时无法读取已发邀请，请重试。',
                'Sent invitations are unavailable. Retry the read.',
              ),
            }, key: const ValueKey('cloud-sent.failure')),
          if (c.notice != null)
            Text(switch (c.notice) {
              'cancelled' ||
              'ended' => tr('此邀请已结束。', 'This invitation has ended.'),
              'accepted' => tr(
                '此邀请已被接受，成员权限未被撤回。',
                'This invitation was accepted. Membership was not removed.',
              ),
              'changed' => tr(
                '邀请已变化，请重新查询后再操作。',
                'The invitation changed. Read its current state before acting.',
              ),
              _ => tr('此邀请暂不可操作。', 'This invitation cannot be managed.'),
            }, key: const ValueKey('cloud-sent.notice')),
          if (!c.busy && c.items.isEmpty && c.failure == null)
            Text(
              tr('暂无已发邀请', 'No sent invitations'),
              key: const ValueKey('cloud-sent.empty'),
            ),
          for (final row in c.items) entry(row),
          if (c.cursor != null)
            Align(
              alignment: Alignment.centerRight,
              child: TextButton(
                key: const ValueKey('cloud-sent.more'),
                style: TextButton.styleFrom(minimumSize: const Size(88, 44)),
                onPressed: c.canLoadMore ? c.loadMore : null,
                child: Text(tr('更多邀请', 'More invitations')),
              ),
            ),
          if (c.items.length >= 500 && c.cursor != null)
            Text(
              tr(
                '按邮箱筛选可查看更早的邀请。',
                'Filter by email to find older invitations.',
              ),
            ),
        ],
      );
    },
  );
}
