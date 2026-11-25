import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show MaxLengthEnforcement;
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'document_creation_controller.dart';

Future<AppFlowyDocumentPage?> showDocumentCreationDialog(
  BuildContext context,
  DocumentCreationController controller, {
  bool allowOpen = true,
}) => showDialog<AppFlowyDocumentPage>(
  context: context,
  barrierDismissible: false,
  builder: (_) =>
      _DocumentCreationDialog(controller: controller, allowOpen: allowOpen),
);

class _DocumentCreationDialog extends StatefulWidget {
  const _DocumentCreationDialog({
    required this.controller,
    required this.allowOpen,
  });
  final DocumentCreationController controller;
  final bool allowOpen;
  @override
  State<_DocumentCreationDialog> createState() =>
      _DocumentCreationDialogState();
}

class _DocumentCreationDialogState extends State<_DocumentCreationDialog> {
  DocumentCreationController get c => widget.controller;
  late final input = TextEditingController(text: c.name);
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;
  @override
  void initState() {
    super.initState();
    unawaited(c.load());
  }

  @override
  void dispose() {
    input.dispose();
    super.dispose();
  }

  Future<void> submit() async {
    final page = await c.submit();
    if (mounted && c.active && page != null) Navigator.pop(context, page);
  }

  Future<void> finishReview(AppFlowyDocumentCreation before) async {
    await c.finishReview(before);
    if (mounted && c.pending == null) input.clear();
  }

  String failure(String code) => switch (code) {
    'readonly' => tr(
      '当前权限无法在此工作区新建文档。',
      'Your current role cannot create documents in this workspace.',
    ),
    'changed' => tr(
      '位置或来源已变化，请刷新后重新选择。',
      'The location or source changed. Refresh and select again.',
    ),
    'name' => tr(
      '名称过长或包含不支持的换行，请修改。',
      'Shorten the name or remove line breaks.',
    ),
    'review' => tr(
      '请先核对上一次新建的结果。',
      'Check the previous creation result first.',
    ),
    _ => tr('暂时无法加载创建位置，请重试。', 'Locations unavailable. Retry to refresh.'),
  };

  Widget pendingView(AppFlowyDocumentCreation p) => Column(
    mainAxisSize: MainAxisSize.min,
    crossAxisAlignment: CrossAxisAlignment.start,
    children: [
      Text(
        p.name,
        key: const ValueKey('cloud-document.pending-name'),
        style: Theme.of(context).textTheme.titleMedium,
      ),
      const SizedBox(height: 8),
      Text(
        [
          p.parent.breadcrumb,
          p.parent.name,
        ].where((s) => s.isNotEmpty).join(' / '),
      ),
      const SizedBox(height: 16),
      Text(
        p.busy
            ? tr('正在创建并确认文档位置…', 'Creating and checking the document location…')
            : p.status == AppFlowyDocumentCreationStatus.ready
            ? tr(
                '文档已出现在目录中，可以打开。',
                'The document is in the catalog and can be opened.',
              )
            : p.page != null
            ? tr(
                '目录中存在同一文档，但名称或位置已变化，请核对。',
                'The document exists, but its name or location changed. Please review it.',
              )
            : p.serverReturnedId
            ? tr(
                '文档已创建，目录暂未确认，请核对结果。',
                'Creation returned successfully. Check the catalog to confirm the result.',
              )
            : tr(
                '请求结果尚未确认，名称和位置已保留。请先核对，不会自动重发。',
                'The result is unconfirmed. Your name and location are retained. Check the result; the request will not be retried automatically.',
              ),
        key: const ValueKey('cloud-document.outcome'),
      ),
    ],
  );

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
            key: const ValueKey('cloud-document.dialog'),
            title: Text(tr('新建云端文档', 'New cloud document')),
            content: SizedBox(
              width: 480,
              child: SingleChildScrollView(
                child: c.retired
                    ? Text(
                        tr(
                          '账号或来源已变化，请关闭后重新打开工作区。',
                          'Account or source changed. Close and reopen the workspace.',
                        ),
                      )
                    : Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          if (c.busy)
                            const LinearProgressIndicator(
                              key: ValueKey('cloud-document.busy'),
                            ),
                          if (c.failure != null)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 12),
                              child: Text(failure(c.failure!)),
                            ),
                          if (p != null)
                            pendingView(p)
                          else ...[
                            TextField(
                              key: const ValueKey('cloud-document.name'),
                              controller: input,
                              enabled: !c.busy,
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
                                        'cloud-document.name-counter',
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
                            ),
                            const SizedBox(height: 12),
                            if (c.parents.isEmpty && !c.busy)
                              Text(
                                tr(
                                  '没有可用的创建位置。',
                                  'No creation locations available.',
                                ),
                              )
                            else if (c.parents.isNotEmpty)
                              DropdownButtonFormField<String>(
                                key: const ValueKey('cloud-document.parent'),
                                initialValue: c.parentId,
                                isExpanded: true,
                                decoration: InputDecoration(
                                  labelText: tr('创建位置', 'Location'),
                                ),
                                items: [
                                  for (final parent in c.parents)
                                    DropdownMenuItem(
                                      key: ValueKey(
                                        'cloud-document.parent.${parent.viewId}',
                                      ),
                                      value: parent.viewId,
                                      child: Text(
                                        [parent.breadcrumb, parent.name]
                                            .where((v) => v.isNotEmpty)
                                            .join(' / '),
                                        maxLines: 2,
                                        overflow: TextOverflow.ellipsis,
                                      ),
                                    ),
                                ],
                                onChanged: c.busy ? null : c.selectParent,
                              ),
                          ],
                        ],
                      ),
              ),
            ),
            actions: [
              if (!c.retired && p == null)
                TextButton(
                  key: const ValueKey('cloud-document.refresh'),
                  onPressed: c.busy ? null : c.load,
                  child: Text(tr('刷新位置', 'Refresh locations')),
                ),
              if (!c.retired &&
                  p != null &&
                  (p.status == AppFlowyDocumentCreationStatus.unconfirmed ||
                      (!widget.allowOpen &&
                          p.status ==
                              AppFlowyDocumentCreationStatus.ready))) ...[
                TextButton(
                  key: const ValueKey('cloud-document.review-finish'),
                  onPressed: c.busy ? null : () => finishReview(p),
                  child: Text(
                    tr('我已核对，结束本次处理', 'I reviewed the result; finish handling'),
                  ),
                ),
                FilledButton(
                  key: const ValueKey('cloud-document.check'),
                  onPressed: c.busy ? null : () => c.check(p),
                  child: Text(tr('核对结果', 'Check result')),
                ),
              ],
              if (!c.retired &&
                  widget.allowOpen &&
                  p?.status == AppFlowyDocumentCreationStatus.ready)
                FilledButton(
                  key: const ValueKey('cloud-document.open'),
                  onPressed: c.busy
                      ? null
                      : () {
                          final page = c.readyPage(p!);
                          if (page != null) Navigator.pop(context, page);
                        },
                  child: Text(tr('打开文档', 'Open document')),
                ),
              TextButton(
                key: const ValueKey('cloud-document.close'),
                onPressed: c.busy && !c.retired
                    ? null
                    : () => Navigator.pop(context),
                child: Text(
                  tr(p == null ? '取消' : '关闭', p == null ? 'Cancel' : 'Close'),
                ),
              ),
              if (!c.retired && p == null)
                FilledButton(
                  key: const ValueKey('cloud-document.create'),
                  onPressed: c.canSubmit ? submit : null,
                  child: Text(tr('新建并打开', 'Create and open')),
                ),
            ],
          ),
        ),
      );
    },
  );
}
