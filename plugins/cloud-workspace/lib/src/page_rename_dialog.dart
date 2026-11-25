import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show MaxLengthEnforcement;
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'page_rename_controller.dart';

Future<AppFlowyWorkspacePage?> showPageRenameDialog(
  BuildContext context,
  PageRenameController controller,
) => showDialog<AppFlowyWorkspacePage>(
  context: context,
  barrierDismissible: false,
  builder: (_) => _PageRenameDialog(controller),
);

class _PageRenameDialog extends StatefulWidget {
  const _PageRenameDialog(this.controller);
  final PageRenameController controller;
  @override
  State<_PageRenameDialog> createState() => _PageRenameDialogState();
}

class _PageRenameDialogState extends State<_PageRenameDialog> {
  PageRenameController get c => widget.controller;
  late final input = TextEditingController(text: c.name)
    ..selection = TextSelection(baseOffset: 0, extentOffset: c.name.length);
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  @override
  void initState() {
    super.initState();
    c.addListener(_changed);
    unawaited(c.load());
  }

  void _changed() {
    if (input.text != c.name)
      input.value = TextEditingValue(
        text: c.name,
        selection: c.target != null && c.pending == null
            ? TextSelection(baseOffset: 0, extentOffset: c.name.length)
            : TextSelection.collapsed(offset: c.name.length),
      );
  }

  @override
  void dispose() {
    c.removeListener(_changed);
    input.dispose();
    super.dispose();
  }

  Future<void> submit() async {
    final page = await c.submit();
    if (mounted && c.active && page != null) {
      final before = c.pending;
      if (before != null) c.finishConfirmed(before);
      Navigator.pop(context, page);
    }
  }

  String error(String code) => switch (code) {
    'readonly' => tr(
      '当前工作区角色无法修改文档名称。',
      'Your workspace role cannot rename documents.',
    ),
    'changed' => tr(
      '页面或来源已变化，请刷新后核对。',
      'The page or source changed. Refresh and review it.',
    ),
    'name' => tr(
      '名称过长或含有不支持的换行，请修改。',
      'Shorten the name or remove line breaks.',
    ),
    'review' => tr('请先核对上一次改名结果。', 'Review the previous rename first.'),
    _ => tr(
      '暂时无法读取原名称，请重试。',
      'The current name is unavailable. Retry to refresh.',
    ),
  };
  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: c,
    builder: (context, _) {
      final p = c.pending;
      return Theme(
        data: Theme.of(context).copyWith(
          visualDensity: VisualDensity.standard,
          materialTapTargetSize: MaterialTapTargetSize.padded,
        ),
        child: PopScope(
          canPop: !c.busy || c.retired,
          child: AlertDialog(
            key: const ValueKey('cloud-rename.dialog'),
            title: Text(tr('重命名文档', 'Rename document')),
            content: SizedBox(
              width: 480,
              child: SingleChildScrollView(
                child: c.retired
                    ? Text(
                        tr(
                          '账号或来源已变化，请关闭后重新打开。',
                          'Account or source changed. Close and reopen.',
                        ),
                      )
                    : Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          if (c.busy)
                            const LinearProgressIndicator(
                              key: ValueKey('cloud-rename.busy'),
                            ),
                          if (c.failure.isNotEmpty)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 12),
                              child: Text(error(c.failure)),
                            ),
                          if (p == null)
                            TextField(
                              key: const ValueKey('cloud-rename.name'),
                              controller: input,
                              enabled: !c.busy && c.target != null,
                              maxLength: 512,
                              maxLengthEnforcement: MaxLengthEnforcement.none,
                              buildCounter:
                                  (
                                    context, {
                                    required currentLength,
                                    required isFocused,
                                    maxLength,
                                  }) {
                                    final count = input.text.runes.length;
                                    return Text(
                                      '$count/512',
                                      key: const ValueKey(
                                        'cloud-rename.name-counter',
                                      ),
                                      style: count > 512
                                          ? TextStyle(
                                              color: Theme.of(
                                                context,
                                              ).colorScheme.error,
                                            )
                                          : null,
                                    );
                                  },
                              textInputAction: TextInputAction.done,
                              decoration: InputDecoration(
                                labelText: tr('文档名称', 'Document name'),
                              ),
                              onChanged: c.setName,
                              onSubmitted: (_) {
                                if (c.canSubmit) unawaited(submit());
                              },
                            )
                          else ...[
                            Text(tr('提交的名称', 'Submitted name')),
                            Text(
                              p.name,
                              key: const ValueKey('cloud-rename.submitted'),
                            ),
                            const SizedBox(height: 12),
                            if (p.observed != null) ...[
                              Text(tr('目录中的当前名称', 'Current catalog name')),
                              Text(
                                p.observed!.name,
                                key: const ValueKey('cloud-rename.observed'),
                              ),
                              const SizedBox(height: 12),
                            ],
                            Text(
                              p.busy
                                  ? tr(
                                      '正在修改并核对目录…',
                                      'Renaming and checking the catalog…',
                                    )
                                  : p.status == AppFlowyPageRenameStatus.ready
                                  ? tr(
                                      '名称已在原目录确认。',
                                      'The name is confirmed in the original catalog.',
                                    )
                                  : tr(
                                      '本次名称尚未确认。请先核对，系统不会自动重发改名请求。',
                                      'The requested name is not confirmed. Check the result; the rename will not be resent automatically.',
                                    ),
                              key: const ValueKey('cloud-rename.outcome'),
                            ),
                          ],
                        ],
                      ),
              ),
            ),
            actions: [
              if (!c.retired && p == null)
                TextButton(
                  key: const ValueKey('cloud-rename.refresh'),
                  onPressed: c.busy ? null : c.load,
                  child: Text(tr('刷新原名称', 'Refresh current name')),
                ),
              if (!c.retired &&
                  p?.status == AppFlowyPageRenameStatus.unconfirmed) ...[
                TextButton(
                  key: const ValueKey('cloud-rename.review-finish'),
                  onPressed: c.busy ? null : () => c.finishReview(p!),
                  child: Text(
                    tr('我已核对，结束本次处理', 'I reviewed the result; finish handling'),
                  ),
                ),
                FilledButton(
                  key: const ValueKey('cloud-rename.check'),
                  onPressed: c.busy ? null : () => c.check(p!),
                  child: Text(tr('核对结果', 'Check result')),
                ),
              ],
              TextButton(
                key: const ValueKey('cloud-rename.close'),
                onPressed: c.busy && !c.retired
                    ? null
                    : () {
                        if (p != null && !c.retired) c.finishConfirmed(p);
                        Navigator.pop(
                          context,
                          p?.status == AppFlowyPageRenameStatus.ready
                              ? p?.observed
                              : null,
                        );
                      },
                child: Text(
                  tr(p == null ? '取消' : '关闭', p == null ? 'Cancel' : 'Close'),
                ),
              ),
              if (!c.retired && p == null)
                FilledButton(
                  key: const ValueKey('cloud-rename.submit'),
                  onPressed: c.canSubmit ? submit : null,
                  child: Text(tr('保存名称', 'Save name')),
                ),
            ],
          ),
        ),
      );
    },
  );
}
