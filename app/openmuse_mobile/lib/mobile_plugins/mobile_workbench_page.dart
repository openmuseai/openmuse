import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import '../mobile_plugin_detail_page.dart';

final class MobileWorkbenchPage extends StatefulWidget {
  const MobileWorkbenchPage({
    super.key,
    required this.registry,
    required this.values,
    required this.subject,
    this.scope = 'default',
    this.onDiscover,
  });

  final OpenMusePluginRegistry registry;
  final SecureValueStore values;
  final String subject;
  final String scope;
  final VoidCallback? onDiscover;

  @override
  State<MobileWorkbenchPage> createState() => _MobileWorkbenchPageState();
}

final class _MobileWorkbenchPageState extends State<MobileWorkbenchPage> {
  bool _editing = false;
  List<String> _order = [];
  Set<String> _pinned = {};
  List<String> _beforeOrder = [];
  Set<String> _beforePinned = {};

  String get _key =>
      'openmuse.mobile.workbench.v1.${widget.subject}.${widget.scope}';

  @override
  void initState() {
    super.initState();
    widget.registry.addListener(_refresh);
    unawaited(_restore());
  }

  @override
  void didUpdateWidget(covariant MobileWorkbenchPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.registry != widget.registry) {
      oldWidget.registry.removeListener(_refresh);
      widget.registry.addListener(_refresh);
    }
    if (oldWidget.subject != widget.subject ||
        oldWidget.scope != widget.scope) {
      _order = [];
      _pinned = {};
      unawaited(_restore());
    }
  }

  void _refresh() {
    if (mounted) setState(() {});
  }

  Future<void> _restore() async {
    try {
      final key = _key;
      final encoded = await widget.values.read(key);
      if (!mounted || key != _key || encoded == null) return;
      final data = jsonDecode(encoded);
      if (data is! Map || data['version'] != 1) return;
      setState(() {
        _order = (data['order'] as List? ?? []).whereType<String>().toList();
        _pinned = (data['pinned'] as List? ?? []).whereType<String>().toSet();
      });
    } on Object {
      // Corrupt layout preferences fall back to plugin declaration order.
    }
  }

  Future<void> _save() async {
    try {
      await widget.values.write(
        _key,
        jsonEncode({
          'version': 1,
          'order': _order,
          'pinned': _pinned.toList()..sort(),
        }),
      );
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('布局保存失败，请稍后重试')));
      }
    }
  }

  void _beginEdit() {
    setState(() {
      _beforeOrder = [..._order];
      _beforePinned = {..._pinned};
      _editing = true;
    });
  }

  void _finishEdit({required bool save}) {
    setState(() {
      if (!save) {
        _order = _beforeOrder;
        _pinned = _beforePinned;
      }
      _editing = false;
    });
    if (save) unawaited(_save());
  }

  void _move(String moved, String target) {
    if (moved == target) return;
    setState(() {
      final ids = _visible().map((e) => e.descriptor.id).toList();
      ids.remove(moved);
      ids.insert(ids.indexOf(target), moved);
      _order = ids;
    });
  }

  List<OpenMusePlugin> _visible() {
    final plugins = <OpenMusePlugin>[
      for (final descriptor in widget.registry.descriptors)
        if (widget.registry.stateOf(descriptor.id) ==
            OpenMusePluginState.active)
          if (widget.registry.plugin(descriptor.id)
              case final OpenMuseMobileWorkbenchContributor contributor)
            contributor as OpenMusePlugin,
    ];
    final position = {for (var i = 0; i < _order.length; i++) _order[i]: i};
    plugins.sort((a, b) {
      final aPin = _pinned.contains(a.descriptor.id);
      final bPin = _pinned.contains(b.descriptor.id);
      if (aPin != bPin) return aPin ? -1 : 1;
      return (position[a.descriptor.id] ?? 100000).compareTo(
        position[b.descriptor.id] ?? 100000,
      );
    });
    return plugins;
  }

  void _open(OpenMusePlugin plugin) {
    final contribution = plugin as OpenMuseMobileWorkbenchContributor;
    Navigator.of(context).push<void>(
      MaterialPageRoute(
        builder: (_) =>
            MobilePluginDetailPage(plugin: plugin, contribution: contribution),
      ),
    );
  }

  @override
  void dispose() {
    widget.registry.removeListener(_refresh);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final plugins = _visible();
    final colors = Theme.of(context).colorScheme;
    return Scaffold(
      backgroundColor: Theme.of(context).scaffoldBackgroundColor,
      body: CustomScrollView(
        slivers: [
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(22, 22, 16, 18),
              child: Row(
                children: [
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          '工作台',
                          style: Theme.of(context).textTheme.headlineLarge
                              ?.copyWith(
                                fontWeight: FontWeight.w800,
                                letterSpacing: -1.2,
                              ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '常用插件，一眼掌握',
                          style: TextStyle(color: colors.onSurfaceVariant),
                        ),
                      ],
                    ),
                  ),
                  TextButton(
                    key: const ValueKey('workbench-edit'),
                    onPressed: _editing
                        ? () => _finishEdit(save: false)
                        : _beginEdit,
                    child: Text(_editing ? '取消编辑' : '编辑布局'),
                  ),
                  if (_editing)
                    FilledButton(
                      key: const ValueKey('workbench-done'),
                      onPressed: () => _finishEdit(save: true),
                      child: const Text('完成'),
                    ),
                ],
              ),
            ),
          ),
          if (plugins.isEmpty)
            SliverFillRemaining(
              hasScrollBody: false,
              child: Center(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Icon(
                      Icons.widgets_outlined,
                      size: 48,
                      color: colors.primary,
                    ),
                    const SizedBox(height: 12),
                    const Text('还没有可放在工作台的插件'),
                    TextButton(
                      onPressed: widget.onDiscover,
                      child: const Text('探索插件'),
                    ),
                  ],
                ),
              ),
            )
          else
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(16, 0, 16, 24),
                child: LayoutBuilder(
                  builder: (context, constraints) => constraints.maxWidth < 340
                      ? Column(
                          children: [
                            for (final plugin in plugins)
                              Padding(
                                padding: const EdgeInsets.only(bottom: 12),
                                child: _card(
                                  plugin,
                                  plugin as OpenMuseMobileWorkbenchContributor,
                                ),
                              ),
                          ],
                        )
                      : _masonry(plugins),
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _masonry(List<OpenMusePlugin> plugins) {
    final rows = <Widget>[];
    var left = <Widget>[];
    var right = <Widget>[];
    var leftHeight = 0;
    var rightHeight = 0;
    void flush() {
      if (left.isEmpty && right.isEmpty) return;
      rows.add(
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(child: Column(children: left)),
            const SizedBox(width: 12),
            Expanded(child: Column(children: right)),
          ],
        ),
      );
      left = [];
      right = [];
      leftHeight = 0;
      rightHeight = 0;
    }

    for (final plugin in plugins) {
      final contribution = plugin as OpenMuseMobileWorkbenchContributor;
      final card = contribution.workbenchCard;
      final item = Padding(
        padding: const EdgeInsets.only(bottom: 12),
        child: _card(plugin, contribution),
      );
      if (card.columns == 2) {
        flush();
        rows.add(item);
      } else if (leftHeight <= rightHeight) {
        left.add(item);
        leftHeight += _height(card.height).round();
      } else {
        right.add(item);
        rightHeight += _height(card.height).round();
      }
    }
    flush();
    return Column(children: rows);
  }

  double _height(OpenMuseWidgetHeight value) => switch (value) {
    OpenMuseWidgetHeight.small => 166,
    OpenMuseWidgetHeight.medium => 236,
    OpenMuseWidgetHeight.large => 308,
  };

  Widget _card(
    OpenMusePlugin plugin,
    OpenMuseMobileWorkbenchContributor contribution,
  ) {
    final id = plugin.descriptor.id;
    final card = contribution.workbenchCard;
    final colors = Theme.of(context).colorScheme;
    final content = Material(
      color: colors.surface,
      borderRadius: BorderRadius.circular(20),
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        key: ValueKey('workbench-card-$id'),
        onTap: _editing ? null : () => _open(plugin),
        child: SizedBox(
          height: _height(card.height),
          child: Padding(
            padding: const EdgeInsets.all(15),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Expanded(
                      child: Text(
                        card.title,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: const TextStyle(
                          fontWeight: FontWeight.w700,
                          fontSize: 16,
                        ),
                      ),
                    ),
                    if (_editing)
                      IconButton(
                        key: ValueKey('workbench-pin-$id'),
                        tooltip: _pinned.contains(id) ? '取消置顶' : '置顶',
                        onPressed: () => setState(() {
                          if (!_pinned.add(id)) _pinned.remove(id);
                        }),
                        icon: Icon(
                          _pinned.contains(id)
                              ? Icons.push_pin
                              : Icons.push_pin_outlined,
                          size: 20,
                        ),
                      )
                    else if (_pinned.contains(id))
                      Icon(Icons.push_pin, size: 18, color: colors.primary),
                  ],
                ),
                const SizedBox(height: 8),
                Expanded(
                  child: ClipRect(
                    child: MobileWidgetPreview(
                      key: ValueKey('workbench-preview-$id'),
                      messages: contribution.buildWorkbenchPreview(),
                      changes: contribution.workbenchChanges,
                      reload: contribution.buildWorkbenchPreview,
                    ),
                  ),
                ),
                if (_editing)
                  Align(
                    alignment: Alignment.bottomRight,
                    child: Icon(
                      Icons.drag_indicator,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
    if (!_editing) return content;
    return DragTarget<String>(
      onWillAcceptWithDetails: (details) => details.data != id,
      onAcceptWithDetails: (details) => _move(details.data, id),
      builder: (context, candidates, rejected) => LongPressDraggable<String>(
        data: id,
        feedback: Material(
          elevation: 12,
          borderRadius: BorderRadius.circular(20),
          child: SizedBox(
            width: 160,
            child: Padding(
              padding: const EdgeInsets.all(18),
              child: Text(card.title),
            ),
          ),
        ),
        childWhenDragging: Opacity(opacity: 0.35, child: content),
        child: DecoratedBox(
          decoration: BoxDecoration(
            border: candidates.isEmpty
                ? null
                : Border.all(color: colors.primary, width: 2),
            borderRadius: BorderRadius.circular(20),
          ),
          child: content,
        ),
      ),
    );
  }
}
