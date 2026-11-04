import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

final class MobileWorkbenchPage extends StatefulWidget {
  const MobileWorkbenchPage({
    super.key,
    required this.registry,
    required this.values,
    required this.subject,
    this.scope = 'default',
    this.onDiscover,
    this.onManagePlugins,
    this.embedded = false,
    this.onEditingChanged,
  });

  final OpenMusePluginRegistry registry;
  final SecureValueStore values;
  final String subject;
  final String scope;
  final VoidCallback? onDiscover;
  final VoidCallback? onManagePlugins;
  final bool embedded;
  final ValueChanged<bool>? onEditingChanged;

  @override
  State<MobileWorkbenchPage> createState() => MobileWorkbenchPageState();
}

final class MobileWorkbenchPageState extends State<MobileWorkbenchPage> {
  bool _editing = false;
  bool get editing => _editing;
  final scroll = ScrollController();
  Set<String> _hidden = {}, _beforeHidden = {};
  String _layout() => jsonEncode([
    _order,
    _pinned.toList()..sort(),
    _heights.map((k, v) => MapEntry(k, v.name)),
    _columns,
    _hidden.toList()..sort(),
  ]);
  String? _editBaseline;
  bool get dirty => _editing && _editBaseline != _layout();
  void scrollToTop() {
    if (scroll.hasClients) {
      scroll.animateTo(
        0,
        duration: const Duration(milliseconds: 180),
        curve: Curves.easeOut,
      );
    }
  }

  List<String> _order = [];
  Set<String> _pinned = {};
  Map<String, OpenMuseWidgetHeight> _heights = {};
  Map<String, int> _columns = {};
  List<String> _beforeOrder = [];
  Set<String> _beforePinned = {};
  Map<String, OpenMuseWidgetHeight> _beforeHeights = {};
  Map<String, int> _beforeColumns = {};

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
      _hidden = {};
      _editing = false;
      _pinned = {};
      _heights = {};
      _columns = {};
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
      if (data is! Map ||
          (data['version'] != 1 &&
              data['version'] != 2 &&
              data['version'] != 3)) {
        return;
      }
      setState(() {
        _hidden = (data['hidden'] as List? ?? []).whereType<String>().toSet();
        _order = (data['order'] as List? ?? []).whereType<String>().toList();
        _pinned = (data['pinned'] as List? ?? []).whereType<String>().toSet();
        _heights = {
          for (final entry in (data['heights'] as Map? ?? {}).entries)
            if (entry.key is String && entry.value is String)
              for (final height in OpenMuseWidgetHeight.values)
                if (height.name == entry.value) entry.key as String: height,
        };
        _columns = {
          if (data['version'] != 3)
            for (final descriptor in widget.registry.descriptors)
              if (widget.registry.plugin(descriptor.id)
                  case final OpenMuseMobileWorkbenchContributor contributor)
                descriptor.id: contributor.workbenchCard.columns,
          for (final entry in (data['columns'] as Map? ?? {}).entries)
            if (entry.key is String && (entry.value == 1 || entry.value == 2))
              entry.key as String: entry.value as int,
        };
      });
    } on Object {
      // Corrupt layout preferences fall back to plugin declaration order.
    }
  }

  Future<bool> _save() async {
    final localeStrings = stringsOf(context);

    try {
      await widget.values.write(
        _key,
        jsonEncode({
          'version': 3,
          'hidden': _hidden.toList()..sort(),
          'order': _order,
          'pinned': _pinned.toList()..sort(),
          'heights': _heights.map((id, height) => MapEntry(id, height.name)),
          'columns': _columns,
        }),
      );
      return true;
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              (localeStrings
                  .mobileUnableToSaveTheLayoutTryAgainLater8661017f()),
            ),
          ),
        );
      }
      return false;
    }
  }

  void beginEdit() => _beginEdit();
  void _beginEdit() {
    setState(() {
      _editBaseline = _layout();
      _beforeHidden = {..._hidden};
      _beforeOrder = [..._order];
      _beforePinned = {..._pinned};
      _beforeHeights = {..._heights};
      _beforeColumns = {..._columns};
      _editing = true;
    });
    widget.onEditingChanged?.call(true);
  }

  Future<bool> finishEdit({required bool save}) => _finishEdit(save: save);
  Future<bool> _finishEdit({required bool save}) async {
    if (save && !await _save()) return false;
    if (!mounted) return false;
    setState(() {
      if (!save) {
        _hidden = _beforeHidden;
        _order = _beforeOrder;
        _pinned = _beforePinned;
        _heights = _beforeHeights;
        _columns = _beforeColumns;
      }
      _editing = false;
    });
    widget.onEditingChanged?.call(false);
    return true;
  }

  Future<bool> requestLeave() async {
    if (!_editing) return true;
    if (!dirty) return _finishEdit(save: false);
    final choice = await showDialog<String>(
      context: context,
      builder: (dialog) => AlertDialog(
        title: Text(stringsOf(context).mobileSaveLayoutChanges()),
        actions: [
          TextButton(
            key: const ValueKey('apps-edit-keep'),
            onPressed: () => Navigator.pop(dialog),
            child: Text(stringsOf(context).mobileKeepEditing()),
          ),
          TextButton(
            key: const ValueKey('apps-edit-discard'),
            onPressed: () => Navigator.pop(dialog, 'discard'),
            child: Text(stringsOf(context).mobileDiscardChanges()),
          ),
          FilledButton(
            key: const ValueKey('apps-edit-save'),
            onPressed: () => Navigator.pop(dialog, 'save'),
            child: Text(stringsOf(context).mobileSaveChanges()),
          ),
        ],
      ),
    );
    if (choice == null || !mounted) return false;
    return _finishEdit(save: choice == 'save');
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
        if (widget.registry.plugin(descriptor.id)
            case final OpenMuseMobileWorkbenchContributor contributor)
          if (!widget.embedded ||
              _editing ||
              (!_hidden.contains(descriptor.id) &&
                  widget.registry.stateOf(descriptor.id) ==
                      OpenMusePluginState.active))
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

  OpenMuseWidgetHeight _heightFor(String id, OpenMuseWorkbenchCard card) =>
      _heights[id] ?? card.height;

  int _columnsFor(String id, OpenMuseWorkbenchCard card) =>
      _columns[id] ?? (widget.embedded ? 2 : card.columns);

  void _resize(String id, String option) {
    setState(() {
      if (option == 'reset') {
        _heights.remove(id);
        _columns.remove(id);
      } else if (option.startsWith('height.')) {
        _heights[id] = OpenMuseWidgetHeight.values.byName(
          option.substring('height.'.length),
        );
      } else if (option == 'columns.1' || option == 'columns.2') {
        _columns[id] = option == 'columns.2' ? 2 : 1;
      }
    });
  }

  @override
  void dispose() {
    widget.registry.removeListener(_refresh);
    scroll.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final plugins = _visible();
    final colors = Theme.of(context).colorScheme;
    final body = CustomScrollView(
      controller: scroll,
      key: const PageStorageKey('apps-overview-scroll'),
      slivers: [
        if (!widget.embedded)
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
                          (stringsOf(context).mobileWorkbenchA6f1b51f()),
                          style: Theme.of(context).textTheme.headlineLarge
                              ?.copyWith(
                                fontWeight: FontWeight.w800,
                                letterSpacing: -1.2,
                              ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          (stringsOf(
                            context,
                          ).mobileYourToolsAtAGlance9f2e8073()),
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
                    child: Text(
                      _editing
                          ? (stringsOf(context).mobileCancelEditing3a1cf24b())
                          : (stringsOf(context).mobileEditLayoutBb3d555a()),
                    ),
                  ),
                  if (_editing)
                    FilledButton(
                      key: const ValueKey('workbench-done'),
                      onPressed: () => _finishEdit(save: true),
                      child: Text((stringsOf(context).mobileDone92e3eec4())),
                    ),
                ],
              ),
            ),
          ),
        if (widget.embedded)
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 6, 20, 22),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        Icons.grid_view_outlined,
                        size: 15,
                        color: colors.onSurfaceVariant,
                      ),
                      const SizedBox(width: 6),
                      Expanded(
                        child: Text(
                          stringsOf(context).mobileYourAppsArranged(),
                          style: TextStyle(
                            fontSize: 12,
                            color: colors.onSurfaceVariant,
                          ),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 23),
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          stringsOf(context).mobileYourOverview(),
                          style: const TextStyle(
                            fontSize: 15,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                      Text(
                        stringsOf(context).mobileThisDevice(),
                        style: TextStyle(
                          fontSize: 12,
                          color: colors.onSurfaceVariant,
                        ),
                      ),
                    ],
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
                  Icon(Icons.widgets_outlined, size: 48, color: colors.primary),
                  const SizedBox(height: 12),
                  Text(
                    (widget.embedded
                        ? stringsOf(context).mobileNoAppsTitle()
                        : stringsOf(
                            context,
                          ).mobileNoWorkbenchPluginsAreAvailable4cba5019()),
                  ),
                  if (widget.embedded)
                    Padding(
                      padding: const EdgeInsets.all(16),
                      child: Text(
                        stringsOf(context).mobileNoAppsDetail(),
                        textAlign: TextAlign.center,
                        style: TextStyle(color: colors.onSurfaceVariant),
                      ),
                    ),
                  TextButton(
                    onPressed: widget.embedded && _hidden.isNotEmpty
                        ? _beginEdit
                        : widget.onDiscover,
                    child: Text(
                      (widget.embedded && _hidden.isNotEmpty
                          ? stringsOf(context).mobileChooseVisibleApps()
                          : stringsOf(context).mobileExplorePlugins1ff2cee8()),
                    ),
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
        if (widget.embedded && plugins.isNotEmpty)
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 0, 20, 24),
              child: OutlinedButton.icon(
                onPressed: _editing ? null : widget.onDiscover,
                icon: const Icon(Icons.add, size: 18),
                label: Text(stringsOf(context).mobileFindMoreApps()),
              ),
            ),
          ),
      ],
    );
    return widget.embedded
        ? body
        : Scaffold(
            backgroundColor: Theme.of(context).scaffoldBackgroundColor,
            body: body,
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
      if (_columnsFor(plugin.descriptor.id, card) == 2) {
        flush();
        rows.add(item);
      } else if (leftHeight <= rightHeight) {
        left.add(item);
        leftHeight += _height(_heightFor(plugin.descriptor.id, card)).round();
      } else {
        right.add(item);
        rightHeight += _height(_heightFor(plugin.descriptor.id, card)).round();
      }
    }
    flush();
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: rows,
    );
  }

  double _height(OpenMuseWidgetHeight value) => switch (value) {
    OpenMuseWidgetHeight.small => 166,
    OpenMuseWidgetHeight.medium => widget.embedded ? 190 : 236,
    OpenMuseWidgetHeight.large => widget.embedded ? 264 : 308,
  };

  Widget _card(
    OpenMusePlugin plugin,
    OpenMuseMobileWorkbenchContributor contribution,
  ) {
    final id = plugin.descriptor.id;
    final card = contribution.workbenchCard;
    final active = widget.registry.stateOf(id) == OpenMusePluginState.active;
    final colors = Theme.of(context).colorScheme;
    final content = Material(
      color: colors.surface,
      borderRadius: BorderRadius.circular(20),
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        key: ValueKey('workbench-card-$id'),
        onTap: _editing || !active ? null : () => _open(plugin),
        child: SizedBox(
          height:
              _height(_heightFor(id, card)) +
              (widget.embedded && _editing ? 60 : 0),
          child: Padding(
            padding: const EdgeInsets.all(15),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    if (widget.embedded) ...[
                      Container(
                        width: 40,
                        height: 40,
                        decoration: BoxDecoration(
                          color: colors.primary.withValues(alpha: .09),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Icon(
                          Icons.widgets_outlined,
                          color: colors.primary,
                          size: 23,
                        ),
                      ),
                      const SizedBox(width: 12),
                    ],
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            card.title,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: const TextStyle(
                              fontWeight: FontWeight.w600,
                              fontSize: 15,
                            ),
                          ),
                          if (widget.embedded) ...[
                            const SizedBox(height: 4),
                            Text(
                              card.summary,
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                              style: TextStyle(
                                fontSize: 12,
                                color: colors.onSurfaceVariant,
                              ),
                            ),
                          ],
                        ],
                      ),
                    ),
                    if (!_editing && _pinned.contains(id))
                      Icon(Icons.push_pin, size: 18, color: colors.primary),
                  ],
                ),
                if (widget.embedded) ...[
                  const SizedBox(height: 12),
                  Divider(height: 1, color: colors.outlineVariant),
                  const SizedBox(height: 12),
                ] else
                  const SizedBox(height: 8),
                Expanded(
                  child: active
                      ? IgnorePointer(
                          ignoring: _editing,
                          child: ClipRect(
                            child: KeyedSubtree(
                              key: ValueKey(
                                Localizations.localeOf(context).toLanguageTag(),
                              ),
                              child: MobileWidgetPreview(
                                key: ValueKey('workbench-preview-$id'),
                                messages: contribution.buildWorkbenchPreview(),
                                changes: contribution.workbenchChanges,
                                reload: contribution.buildWorkbenchPreview,
                              ),
                            ),
                          ),
                        )
                      : Center(
                          child: Column(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Text(
                                (stringsOf(
                                  context,
                                ).mobileThisPluginIsDisabled64309558()),
                              ),
                              if (widget.onManagePlugins != null)
                                TextButton(
                                  onPressed: _editing
                                      ? null
                                      : widget.onManagePlugins,
                                  child: Text(
                                    (stringsOf(
                                      context,
                                    ).mobileManagePlugins1429ebfe()),
                                  ),
                                ),
                            ],
                          ),
                        ),
                ),
                if (_editing)
                  Wrap(
                    alignment: WrapAlignment.end,
                    children: [
                      if (widget.embedded) ...[
                        IconButton(
                          tooltip: stringsOf(context).mobileMoveUp(),
                          icon: const Icon(Icons.keyboard_arrow_up),
                          onPressed: () {
                            final ids = _visible();
                            final i = ids.indexOf(plugin);
                            if (i > 0) _move(id, ids[i - 1].descriptor.id);
                          },
                        ),
                        IconButton(
                          tooltip: stringsOf(context).mobileMoveDown(),
                          icon: const Icon(Icons.keyboard_arrow_down),
                          onPressed: () {
                            final ids = _visible();
                            final i = ids.indexOf(plugin);
                            if (i < ids.length - 1) {
                              final order = ids
                                  .map((p) => p.descriptor.id)
                                  .toList();
                              order.remove(id);
                              order.insert(i + 1, id);
                              setState(() => _order = order);
                            }
                          },
                        ),
                        IconButton(
                          key: ValueKey('workbench-visible-$id'),
                          tooltip: _hidden.contains(id)
                              ? stringsOf(context).mobileShowApp()
                              : stringsOf(context).mobileHideApp(),
                          icon: Icon(
                            _hidden.contains(id)
                                ? Icons.visibility_off_outlined
                                : Icons.visibility_outlined,
                          ),
                          onPressed: () => setState(() {
                            if (!_hidden.remove(id)) _hidden.add(id);
                          }),
                        ),
                      ],
                      PopupMenuButton<String>(
                        key: ValueKey('workbench-size-$id'),
                        tooltip: (stringsOf(context).mobileResizeC93a70d1()),
                        icon: const Icon(Icons.aspect_ratio, size: 20),
                        onSelected: (value) => _resize(id, value),
                        itemBuilder: (_) => [
                          PopupMenuItem(
                            value: 'height.small',
                            child: Text(
                              (stringsOf(
                                context,
                              ).mobileCompactHeight557e1f94()),
                            ),
                          ),
                          PopupMenuItem(
                            value: 'height.medium',
                            child: Text(
                              (stringsOf(
                                context,
                              ).mobileStandardHeight173970c3()),
                            ),
                          ),
                          PopupMenuItem(
                            value: 'height.large',
                            child: Text(
                              (stringsOf(
                                context,
                              ).mobileExpandedHeight61c3586f()),
                            ),
                          ),
                          PopupMenuDivider(),
                          PopupMenuItem(
                            value: 'columns.1',
                            child: Text(
                              (stringsOf(context).mobileOneColumn84e0d35c()),
                            ),
                          ),
                          PopupMenuItem(
                            value: 'columns.2',
                            child: Text(
                              (stringsOf(context).mobileTwoColumnsA45919ff()),
                            ),
                          ),
                          PopupMenuItem(
                            value: 'reset',
                            child: Text(
                              (stringsOf(
                                context,
                              ).mobileResetPluginSize1482e95f()),
                            ),
                          ),
                        ],
                      ),
                      IconButton(
                        key: ValueKey('workbench-pin-$id'),
                        tooltip: _pinned.contains(id)
                            ? (stringsOf(context).mobileUnpinEf1e845f())
                            : (stringsOf(context).mobilePinFc99471d()),
                        onPressed: () => setState(() {
                          if (!_pinned.add(id)) _pinned.remove(id);
                        }),
                        icon: Icon(
                          _pinned.contains(id)
                              ? Icons.push_pin
                              : Icons.push_pin_outlined,
                          size: 20,
                        ),
                      ),
                      Icon(
                        Icons.drag_indicator,
                        color: colors.onSurfaceVariant,
                      ),
                    ],
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
