import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'workspace_access_controller.dart';
import 'workspace_invite_code_controller.dart';

class WorkspaceInviteCodePanel extends StatefulWidget {
  const WorkspaceInviteCodePanel({super.key, required this.access});
  final WorkspaceAccessController access;
  @override
  State<WorkspaceInviteCodePanel> createState() =>
      _WorkspaceInviteCodePanelState();
}

class _WorkspaceInviteCodePanelState extends State<WorkspaceInviteCodePanel> {
  late final c = WorkspaceInviteCodeController(widget.access);
  int? hours = 168;
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  @override
  void dispose() {
    c.dispose();
    super.dispose();
  }

  Future<bool> confirm(bool disable) async =>
      await showDialog<bool>(
        context: context,
        builder: (ctx) => AlertDialog(
          title: Text(
            disable
                ? tr('停用邀请码', 'Disable invitation code')
                : tr('重置邀请码', 'Reset invitation code'),
          ),
          content: Text(
            tr(
              '旧邀请码将失效。已经加入的成员不受影响，仍通过群组成员管理权限。',
              'The previous code will stop working. Existing members keep their membership; manage their roles through workspace members.',
            ),
          ),
          actions: [
            TextButton(
              key: const ValueKey('cloud-code.confirm-cancel'),
              style: TextButton.styleFrom(minimumSize: const Size(64, 44)),
              onPressed: () => Navigator.pop(ctx, false),
              child: Text(tr('取消', 'Cancel')),
            ),
            TextButton(
              key: const ValueKey('cloud-code.confirm'),
              style: TextButton.styleFrom(minimumSize: const Size(64, 44)),
              onPressed: () => Navigator.pop(ctx, true),
              child: Text(disable ? tr('停用', 'Disable') : tr('重置', 'Reset')),
            ),
          ],
        ),
      ) ??
      false;
  Future<void> copy(AppFlowyWorkspaceInviteCode value) async {
    if (!c.canCopy(value)) return;
    await Clipboard.setData(ClipboardData(text: value.code!));
    if (mounted && c.canCopy(value))
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(tr('邀请码已复制', 'Invitation code copied'))),
      );
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: c,
    builder: (context, _) {
      final value = c.observation;
      return ExpansionTile(
        key: const ValueKey('cloud-code.panel'),
        tilePadding: EdgeInsets.zero,
        title: Text(tr('通过邀请码加入', 'Join with an invitation code')),
        subtitle: Text(
          tr('持有邀请码的用户可以加入群组', 'People with the code can join this workspace'),
        ),
        onExpansionChanged: (open) {
          if (open && value == null && !c.retired) c.refresh();
        },
        children: [
          Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              if (c.busy)
                const LinearProgressIndicator(key: ValueKey('cloud-code.busy')),
              if (c.retired)
                Text(
                  tr('会话已结束，请重新打开。', 'This session ended. Reopen the dialog.'),
                  key: const ValueKey('cloud-code.retired'),
                ),
              if (c.failure != null || c.unknown)
                Text(
                  c.unknown
                      ? tr(
                          '操作结果尚未确认。可以查询当前邀请码，暂不重复生成或停用。',
                          'The result is unconfirmed. Read the current code before any further changes.',
                        )
                      : tr(
                          '暂时无法读取或操作邀请码，请查询重试。',
                          'The invitation code is unavailable. Read again to review.',
                        ),
                  key: const ValueKey('cloud-code.failure'),
                ),
              Text(
                c.unknown && !c.fresh
                    ? tr('当前邀请码状态待确认', 'Current code state is unconfirmed')
                    : value == null
                    ? tr('尚未读取到邀请码', 'No invitation code loaded')
                    : value.code == null
                    ? tr('当前没有邀请码', 'No invitation code exists')
                    : tr(
                        '已读到邀请码，加入时会验证有效性',
                        'Code read. Its validity is checked when joining.',
                      ),
                key: const ValueKey('cloud-code.state'),
              ),
              if (c.notice != null)
                Text(
                  c.notice == 'disabled'
                      ? tr(
                          '邀请码已停用，已加入成员未被移除。',
                          'Code disabled. Existing members were kept.',
                        )
                      : tr(
                          '邀请码已生成，旧邀请码已失效。',
                          'Code generated. The previous code no longer works.',
                        ),
                  key: const ValueKey('cloud-code.notice'),
                ),
              Text(
                tr('生成 / 重置后的有效期', 'Validity for the next generated code'),
                key: const ValueKey('cloud-code.validity-label'),
                style: Theme.of(context).textTheme.bodySmall,
              ),
              DropdownButton<int>(
                key: const ValueKey('cloud-code.validity'),
                value: hours,
                hint: Text(tr('不限期', 'No expiry')),
                isExpanded: true,
                items: [
                  for (final item in <(int?, String)>[
                    (24, tr('1 天', '1 day')),
                    (168, tr('7 天', '7 days')),
                    (720, tr('30 天', '30 days')),
                    (null, tr('不限期', 'No expiry')),
                  ])
                    DropdownMenuItem<int>(value: item.$1, child: Text(item.$2)),
                ],
                onChanged: c.retired || c.busy
                    ? null
                    : (v) => setState(() => hours = v),
              ),
              Wrap(
                spacing: 8,
                children: [
                  TextButton(
                    key: const ValueKey('cloud-code.refresh'),
                    style: TextButton.styleFrom(
                      minimumSize: const Size(64, 44),
                    ),
                    onPressed: c.retired || c.busy ? null : c.refresh,
                    child: Text(tr('查询', 'Read')),
                  ),
                  TextButton(
                    key: const ValueKey('cloud-code.copy'),
                    style: TextButton.styleFrom(
                      minimumSize: const Size(64, 44),
                    ),
                    onPressed: value != null && c.canCopy(value)
                        ? () => copy(value)
                        : null,
                    child: Text(tr('复制邀请码', 'Copy code')),
                  ),
                  TextButton(
                    key: const ValueKey('cloud-code.generate'),
                    style: TextButton.styleFrom(
                      minimumSize: const Size(64, 44),
                    ),
                    onPressed: value != null && c.canWrite(value)
                        ? () async {
                            if (!c.canWrite(value)) return;
                            final requested = hours;
                            if (value.code == null || await confirm(false))
                              await c.generate(value, requested);
                          }
                        : null,
                    child: Text(
                      value?.code == null
                          ? tr('生成', 'Generate')
                          : tr('重置', 'Reset'),
                    ),
                  ),
                  if (value?.code != null)
                    TextButton(
                      key: const ValueKey('cloud-code.disable'),
                      style: TextButton.styleFrom(
                        minimumSize: const Size(64, 44),
                      ),
                      onPressed: value != null && c.canWrite(value)
                          ? () async {
                              if (!c.canWrite(value)) return;
                              if (await confirm(true)) await c.disable(value);
                            }
                          : null,
                      child: Text(tr('停用', 'Disable')),
                    ),
                ],
              ),
              Text(
                tr(
                  '停用邀请码不会移除成员；移除成员也不会停用邀请码。',
                  'Disabling a code keeps existing members. Removing a member does not disable the code.',
                ),
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ],
          ),
        ],
      );
    },
  );
}
