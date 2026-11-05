import 'dart:io';

import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_graph.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_plugin.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:path/path.dart' as p;

Future<void> showMuseVersionHistoryDialog({
  required BuildContext context,
  required File file,
  required MuseTextVersionDiffService service,
}) {
  return showDialog<void>(
    context: context,
    builder: (_) => _VersionHistoryDialog(file: file, service: service),
  );
}

final class _VersionHistoryDialog extends StatefulWidget {
  const _VersionHistoryDialog({required this.file, required this.service});

  final File file;
  final MuseTextVersionDiffService service;

  @override
  State<_VersionHistoryDialog> createState() => _VersionHistoryDialogState();
}

final class _VersionHistoryDialogState extends State<_VersionHistoryDialog> {
  late Future<(List<MuseVersion>, List<MuseAuditEvent>)> _data;

  @override
  void initState() {
    super.initState();
    _reload();
  }

  void _reload() {
    _data = _load();
  }

  Future<(List<MuseVersion>, List<MuseAuditEvent>)> _load() async => (
        await widget.service.repository.listVersions(widget.file),
        await widget.service.repository.listAudit(widget.file),
      );

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return AlertDialog(
      title: Row(
        children: [
          Icon(Icons.history, color: scheme.onSurface),
          const SizedBox(width: 10),
          Expanded(
            child: Text(
              '${p.basename(widget.file.path)} · 版本与审计',
              style: TextStyle(
                color: scheme.onSurface,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ],
      ),
      content: SizedBox(
        width: 720,
        height: 520,
        child: FutureBuilder<(List<MuseVersion>, List<MuseAuditEvent>)>(
          future: _data,
          builder: (context, snapshot) {
            if (!snapshot.hasData) {
              return const Center(child: CircularProgressIndicator());
            }
            final (versions, audit) = snapshot.data!;
            return DefaultTabController(
              length: 2,
              child: Column(
                children: [
                  TabBar(
                    labelColor: scheme.onSurface,
                    unselectedLabelColor:
                        scheme.onSurface.withValues(alpha: 0.62),
                    indicatorColor: scheme.primary,
                    labelStyle: const TextStyle(fontWeight: FontWeight.w600),
                    tabs: const [
                      Tab(text: '版本'),
                      Tab(text: '审计'),
                    ],
                  ),
                  Expanded(
                    child: TabBarView(
                      children: [
                        _versions(context, versions),
                        _audit(context, audit),
                      ],
                    ),
                  ),
                ],
              ),
            );
          },
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(),
          style: TextButton.styleFrom(
            foregroundColor: scheme.onSurface,
          ),
          child: const Text('关闭'),
        ),
      ],
    );
  }

  Widget _versions(BuildContext context, List<MuseVersion> versions) {
    if (versions.isEmpty) {
      return const Center(child: Text('尚未保存版本'));
    }
    final graph = const MuseVersionGraphProjector().project(versions);
    return ListView.separated(
      padding: const EdgeInsets.only(top: 8),
      itemCount: graph.nodes.length,
      separatorBuilder: (_, __) => const Divider(height: 1),
      itemBuilder: (context, index) {
        final node = graph.nodes[index];
        final version = node.version;
        return ListTile(
          leading: SizedBox(
            width: 38,
            height: 52,
            child: CustomPaint(
              painter: _VersionGraphNodePainter(
                depth: node.depth,
                isHead: node.isHead,
                color: Theme.of(context).colorScheme.primary,
              ),
            ),
          ),
          title: Row(
            children: [
              Expanded(
                child: Text(
                  version.message ?? 'Saved version',
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.onSurface,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              if (node.isHead)
                Padding(
                  padding: const EdgeInsets.only(left: 8),
                  child: DecoratedBox(
                    decoration: BoxDecoration(
                      color: Theme.of(context).colorScheme.onSurface,
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                        horizontal: 8,
                        vertical: 3,
                      ),
                      child: Text(
                        'HEAD',
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.surface,
                          fontSize: 11,
                          fontWeight: FontWeight.w700,
                        ),
                      ),
                    ),
                  ),
                ),
            ],
          ),
          subtitle: Text(
            '${DateFormat('yyyy-MM-dd HH:mm:ss').format(version.createdAt.toLocal())}'
            '  ·  ${version.actor.displayName}'
            '  ·  ${version.kind.name}\n'
            '${version.contentDigest.substring(0, 12)}'
            '${node.missingParentIds.isEmpty ? '' : '  ·  缺失父版本 ${node.missingParentIds.length}'}',
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
          isThreeLine: true,
          trailing: FilledButton(
            onPressed: () => _compare(context, version),
            child: const Text('与当前比较'),
          ),
        );
      },
    );
  }

  Widget _audit(BuildContext context, List<MuseAuditEvent> events) {
    if (events.isEmpty) return const Center(child: Text('尚无审计事件'));
    return ListView.separated(
      padding: const EdgeInsets.fromLTRB(4, 10, 4, 16),
      itemCount: events.length,
      separatorBuilder: (_, __) => const SizedBox(height: 8),
      itemBuilder: (context, index) {
        final event = events[index];
        final change = _AuditChangeSet.from(event.metadata);
        if (event.type == 'comparison.create' && change != null) {
          return MuseChangeAuditEventCard(event: event);
        }
        return _BasicAuditTile(event: event);
      },
    );
  }

  Future<void> _compare(BuildContext context, MuseVersion version) async {
    final tabs = context.read<TabsBloc>();
    final navigator = Navigator.of(context);
    final document = await widget.service.compareVersionWithCurrent(
      widget.file,
      version,
    );
    if (!mounted) return;
    navigator.pop();
    tabs.openExternalPlugin(
      MuseTextDiffPlugin.compare(
        document: document,
        fileName: p.basename(widget.file.path),
      ),
    );
  }
}

final class _VersionGraphNodePainter extends CustomPainter {
  const _VersionGraphNodePainter({
    required this.depth,
    required this.isHead,
    required this.color,
  });

  final int depth;
  final bool isHead;
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final x = (9 + depth.clamp(0, 3) * 6).toDouble();
    final paint = Paint()
      ..color = color
      ..strokeWidth = 2
      ..style = PaintingStyle.stroke;
    canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
    canvas.drawCircle(
      Offset(x, size.height / 2),
      isHead ? 5.5 : 4.5,
      Paint()
        ..color = isHead ? color : color.withValues(alpha: 0.88)
        ..style = PaintingStyle.fill,
    );
    canvas.drawCircle(
      Offset(x, size.height / 2),
      isHead ? 5.5 : 4.5,
      Paint()
        ..color = color
        ..strokeWidth = 2
        ..style = PaintingStyle.stroke,
    );
  }

  @override
  bool shouldRepaint(covariant _VersionGraphNodePainter oldDelegate) =>
      oldDelegate.depth != depth ||
      oldDelegate.isHead != isHead ||
      oldDelegate.color != color;
}

final class _BasicAuditTile extends StatelessWidget {
  const _BasicAuditTile({required this.event});

  final MuseAuditEvent event;

  @override
  Widget build(BuildContext context) => Card(
        margin: EdgeInsets.zero,
        elevation: 0,
        child: ListTile(
          dense: true,
          leading: Icon(
            Icons.save_outlined,
            color: Theme.of(context).colorScheme.onSurface,
          ),
          title: const Text('保存版本'),
          subtitle: Text(
            '${DateFormat('yyyy-MM-dd HH:mm:ss').format(event.occurredAt.toLocal())}'
            '  ·  ${event.actor.displayName}',
          ),
          trailing: _ActorBadge(actor: event.actor),
        ),
      );
}

/// Provider-neutral audit event shell with text-provider change previews.
/// Other domains can contribute their own audit card while keeping the same
/// actor and change-summary metadata contract.
final class MuseChangeAuditEventCard extends StatelessWidget {
  const MuseChangeAuditEventCard({super.key, required this.event});

  final MuseAuditEvent event;

  @override
  Widget build(BuildContext context) {
    final change = _AuditChangeSet.from(event.metadata)!;
    final scheme = Theme.of(context).colorScheme;
    return Card(
      margin: EdgeInsets.zero,
      elevation: 0,
      clipBehavior: Clip.antiAlias,
      child: ExpansionTile(
        initiallyExpanded: true,
        leading: CircleAvatar(
          radius: 17,
          backgroundColor: _actorColor(event.actor).withValues(alpha: 0.13),
          child: Icon(
            event.actor.kind == MuseActorKind.agent
                ? Icons.smart_toy_outlined
                : Icons.person_outline,
            size: 18,
            color: _actorColor(event.actor),
          ),
        ),
        title: Row(
          children: [
            Expanded(
              child: Text(
                '${event.actor.displayName} 的变更',
                style: const TextStyle(fontWeight: FontWeight.w600),
              ),
            ),
            _ActorBadge(actor: event.actor),
          ],
        ),
        subtitle: Padding(
          padding: const EdgeInsets.only(top: 4),
          child: Text(
            DateFormat('yyyy-MM-dd HH:mm:ss')
                .format(event.occurredAt.toLocal()),
          ),
        ),
        children: [
          Divider(height: 1, color: scheme.outlineVariant),
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 10),
            child: Row(
              children: [
                _ChangeMetric(
                  label: '新增',
                  value: change.insertions,
                  color: Colors.green,
                  icon: Icons.add,
                ),
                const SizedBox(width: 8),
                _ChangeMetric(
                  label: '删除',
                  value: change.deletions,
                  color: Colors.red,
                  icon: Icons.remove,
                ),
                const SizedBox(width: 8),
                _ChangeMetric(
                  label: '修改',
                  value: change.modifications,
                  color: Colors.blue,
                  icon: Icons.edit_outlined,
                ),
                const Spacer(),
                Text(
                  '+${change.addedLines} / -${change.deletedLines} 行',
                  style: Theme.of(context).textTheme.labelSmall,
                ),
              ],
            ),
          ),
          for (final preview in change.previews)
            _AuditChangePreview(preview: preview),
          if (change.truncated)
            Padding(
              padding: const EdgeInsets.all(12),
              child: Text(
                '仅显示前 ${change.previews.length} 处变更',
                style: Theme.of(context).textTheme.labelSmall,
              ),
            ),
        ],
      ),
    );
  }
}

final class _ChangeMetric extends StatelessWidget {
  const _ChangeMetric({
    required this.label,
    required this.value,
    required this.color,
    required this.icon,
  });

  final String label;
  final int value;
  final Color color;
  final IconData icon;

  @override
  Widget build(BuildContext context) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 5),
        decoration: BoxDecoration(
          color: color.withValues(alpha: 0.11),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, size: 14, color: color),
            const SizedBox(width: 4),
            Text(
              '$label $value',
              style: TextStyle(
                color: color,
                fontWeight: FontWeight.w600,
                fontSize: 12,
              ),
            ),
          ],
        ),
      );
}

final class _AuditChangePreview extends StatelessWidget {
  const _AuditChangePreview({required this.preview});

  final _AuditChangePreviewData preview;

  @override
  Widget build(BuildContext context) {
    final color = switch (preview.kind) {
      'insert' => Colors.green,
      'delete' => Colors.red,
      _ => Colors.blue,
    };
    final label = switch (preview.kind) {
      'insert' => '新增',
      'delete' => '删除',
      _ => '修改',
    };
    return Container(
      margin: const EdgeInsets.fromLTRB(16, 0, 16, 10),
      decoration: BoxDecoration(
        border: Border.all(color: color.withValues(alpha: 0.32)),
        borderRadius: BorderRadius.circular(7),
        color: color.withValues(alpha: 0.045),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 6),
            color: color.withValues(alpha: 0.10),
            child: Row(
              children: [
                Container(width: 3, height: 16, color: color),
                const SizedBox(width: 7),
                Text(
                  label,
                  style: TextStyle(
                    color: color,
                    fontWeight: FontWeight.w700,
                    fontSize: 12,
                  ),
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    preview.semanticLabel.isEmpty
                        ? '第 ${preview.newStart} 行'
                        : preview.semanticLabel,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.labelMedium,
                  ),
                ),
              ],
            ),
          ),
          for (final line in preview.before)
            _AuditLine(prefix: '−', text: line, color: Colors.red),
          for (final line in preview.after)
            _AuditLine(prefix: '+', text: line, color: Colors.green),
        ],
      ),
    );
  }
}

final class _AuditLine extends StatelessWidget {
  const _AuditLine({
    required this.prefix,
    required this.text,
    required this.color,
  });

  final String prefix;
  final String text;
  final Color color;

  @override
  Widget build(BuildContext context) => ColoredBox(
        color: color.withValues(alpha: 0.075),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
          child: Row(
            children: [
              SizedBox(
                width: 22,
                child: Text(
                  prefix,
                  style: TextStyle(color: color, fontWeight: FontWeight.bold),
                ),
              ),
              Expanded(
                child: SelectableText(
                  text,
                  style: const TextStyle(fontFamily: 'monospace', fontSize: 12),
                ),
              ),
            ],
          ),
        ),
      );
}

final class _ActorBadge extends StatelessWidget {
  const _ActorBadge({required this.actor});

  final MuseActorRef actor;

  @override
  Widget build(BuildContext context) {
    final color = _actorColor(actor);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 3),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.18),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: color.withValues(alpha: 0.55)),
      ),
      child: Text(
        switch (actor.kind) {
          MuseActorKind.agent => 'AGENT',
          MuseActorKind.system => 'SYSTEM',
          MuseActorKind.user => 'USER',
        },
        style: TextStyle(
          color: color,
          fontSize: 10,
          fontWeight: FontWeight.w700,
        ),
      ),
    );
  }
}

Color _actorColor(MuseActorRef actor) => switch (actor.kind) {
      MuseActorKind.agent => Colors.deepPurple,
      MuseActorKind.system => Colors.orange,
      MuseActorKind.user => Colors.indigo,
    };

final class _AuditChangeSet {
  const _AuditChangeSet({
    required this.insertions,
    required this.deletions,
    required this.modifications,
    required this.addedLines,
    required this.deletedLines,
    required this.previews,
    required this.truncated,
  });

  final int insertions;
  final int deletions;
  final int modifications;
  final int addedLines;
  final int deletedLines;
  final List<_AuditChangePreviewData> previews;
  final bool truncated;

  static _AuditChangeSet? from(Map<String, Object?> metadata) {
    final summary = metadata['changeSummary'];
    if (summary is! Map) return null;
    int number(String key) =>
        summary[key] is num ? (summary[key] as num).toInt() : 0;
    final rawChanges = metadata['changes'];
    final previews = <_AuditChangePreviewData>[];
    if (rawChanges is List) {
      for (final value in rawChanges) {
        if (value is Map) previews.add(_AuditChangePreviewData.from(value));
      }
    }
    return _AuditChangeSet(
      insertions: number('insertions'),
      deletions: number('deletions'),
      modifications: number('modifications'),
      addedLines: number('addedLines'),
      deletedLines: number('deletedLines'),
      previews: previews,
      truncated: metadata['changesTruncated'] == true,
    );
  }
}

final class _AuditChangePreviewData {
  const _AuditChangePreviewData({
    required this.kind,
    required this.semanticLabel,
    required this.newStart,
    required this.before,
    required this.after,
  });

  factory _AuditChangePreviewData.from(Map<dynamic, dynamic> json) =>
      _AuditChangePreviewData(
        kind: json['kind'] as String? ?? 'replace',
        semanticLabel: json['semanticLabel'] as String? ?? '',
        newStart: (json['newStart'] as num?)?.toInt() ?? 0,
        before: (json['before'] as List? ?? const [])
            .whereType<String>()
            .toList(growable: false),
        after: (json['after'] as List? ?? const [])
            .whereType<String>()
            .toList(growable: false),
      );

  final String kind;
  final String semanticLabel;
  final int newStart;
  final List<String> before;
  final List<String> after;
}
