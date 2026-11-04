import 'dart:async';
import 'package:openmuse_resource_clipboard/openmuse_resource_clipboard.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'controller.dart';
import 'strings.dart';
import 'resource_open.dart';

/// The resource surface is shared by the Mobile Home tab and Desktop entry.
/// It owns presentation, not resource formats, storage or transport semantics.
final class ResourceCenterView extends StatefulWidget {
  const ResourceCenterView({
    super.key,
    required this.controller,
    this.onAddToConversation,
    this.remoteLabel,
    this.title,
  });
  final ResourceCenterController controller;
  final Future<void> Function(ResourceRecord record)? onAddToConversation;
  final String? remoteLabel, title;
  @override
  State<ResourceCenterView> createState() => _ResourceCenterViewState();
}

final class _ResourceCenterViewState extends State<ResourceCenterView> {
  final search = TextEditingController();
  ResourceCenterController get c => widget.controller;
  @override
  void dispose() {
    search.dispose();
    super.dispose();
  }

  void _section(ResourceCenterSection section) {
    search.clear();
    c.selectSection(section);
  }

  Future<void> _menu(ResourceRecord record) async {
    final s = ResourceCenterStrings(context),
        colors = Theme.of(context).colorScheme;
    ResourceProvider? provider;
    try {
      provider = c.hub.provider(record.identity);
    } on ResourceFailure {
      /* Source removed; keep metadata actions. */
    }
    await showModalBottomSheet<void>(
      context: context,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      builder: (sheet) {
        Widget item(
          String key,
          String label,
          IconData icon,
          VoidCallback? action, {
          String? reason,
        }) => ListTile(
          key: ValueKey('resource-menu-$key'),
          contentPadding: EdgeInsets.zero,
          leading: Icon(icon, color: colors.onSurfaceVariant),
          title: Text(label),
          subtitle: reason == null
              ? null
              : Text(reason, style: const TextStyle(fontSize: 12)),
          enabled: action != null,
          onTap: action,
        );
        void dismiss() => Navigator.pop(sheet);
        return ConstrainedBox(
          constraints: BoxConstraints(
            maxHeight: MediaQuery.sizeOf(sheet).height * .84,
          ),
          child: SingleChildScrollView(
            padding: const EdgeInsets.fromLTRB(20, 0, 20, 24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                ResourceSummary(record: record),
                const Divider(),
                item('open', s.open, Icons.insert_drive_file_outlined, () {
                  dismiss();
                  unawaited(openResourcePage(context, c, record.reference()));
                }),
                item('open-with', s.openWith, Icons.apps_outlined, () {
                  dismiss();
                  unawaited(
                    openResourcePage(
                      context,
                      c,
                      record.reference(),
                      chooseHandler: true,
                    ),
                  );
                }),
                if (widget.onAddToConversation != null)
                  item('chat', s.addToChat, Icons.chat_bubble_outline, () {
                    dismiss();
                    unawaited(widget.onAddToConversation!(record));
                  }),
                item('copy', s.copy, Icons.copy_outlined, () {
                  dismiss();
                  unawaited(_copy(record));
                }),
                item('path', s.copyPath, Icons.link_outlined, () {
                  dismiss();
                  unawaited(_copy(record, path: true));
                }),
                item(
                  'rename',
                  s.rename,
                  Icons.drive_file_rename_outline,
                  provider?.source.canRename == true && provider!.available
                      ? () {
                          dismiss();
                          unawaited(_rename(record));
                        }
                      : null,
                  reason: provider?.source.canRename != true
                      ? s.sourceReadOnly
                      : provider?.available != true
                      ? s.sourceUnavailable
                      : null,
                ),
                if (c.creationTarget != null)
                  item('save-copy', s.saveCopy, Icons.download_outlined, () {
                    dismiss();
                    unawaited(_saveCopy(record));
                  }),
                if (c.section == ResourceCenterSection.recent)
                  item(
                    'remove-recent',
                    s.removeRecent,
                    Icons.history,
                    () async {
                      dismiss();
                      await c.hub.removeRecent(record.identity);
                      if (mounted)
                        ScaffoldMessenger.of(context).showSnackBar(
                          SnackBar(
                            content: Text(s.removedRecent),
                            action: SnackBarAction(
                              label: s.undo,
                              onPressed: () => unawaited(
                                c.hub.recordOpened(record, 'restored'),
                              ),
                            ),
                          ),
                        );
                    },
                  ),
              ],
            ),
          ),
        );
      },
    );
  }

  Future<void> _copy(ResourceRecord record, {bool path = false}) async {
    final s = ResourceCenterStrings(context);
    try {
      if (path) {
        c.hub.clearClipboard();
        await Clipboard.setData(
          ClipboardData(
            text: [
              record.sourceLabel,
              ...record.location,
              record.name,
            ].join(' / '),
          ),
        );
      } else {
        final ref = record.reference();
        c.hub.copy(ref);
        await ResourceClipboard.write(ref);
      }
      if (mounted)
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(path ? s.pathCopied : s.copied),
            behavior: SnackBarBehavior.floating,
            margin: const EdgeInsets.fromLTRB(16, 0, 16, 88),
          ),
        );
    } on Object catch (error) {
      if (mounted) showResourceError(context, error);
    }
  }

  Future<void> _saveCopy(ResourceRecord record) async {
    try {
      await c.saveCopy(record.reference(pin: true));
      if (mounted)
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(ResourceCenterStrings(context).imported)),
        );
    } on Object catch (error) {
      if (mounted) showResourceError(context, error);
    }
  }

  Future<void> _rename(ResourceRecord record) async {
    final strings = ResourceCenterStrings(context);
    await showDialog<bool>(
      context: context,
      builder: (_) => _ResourceForm<bool>(
        title: strings.rename,
        description: [record.sourceLabel, ...record.location].join(' / '),
        confirm: strings.save,
        fields: [
          _ResourceField(
            'name',
            strings.name,
            record.name,
            'resource-rename-input',
          ),
        ],
        confirmKey: 'resource-rename-save',
        submit: (values) async {
          await c.rename(record, values['name']!);
          return true;
        },
      ),
    );
  }

  Future<void> _new() async {
    final initial = c.creationTarget;
    if (initial == null) {
      showResourceError(context, const ResourceFailure('SOURCE_UNAVAILABLE'));
      return;
    }
    ResourceProvider target = initial;
    await showModalBottomSheet<void>(
      context: context,
      isScrollControlled: true,
      useSafeArea: true,
      showDragHandle: true,
      builder: (sheet) => StatefulBuilder(
        builder: (sheet, change) {
          final s = ResourceCenterStrings(sheet),
              colors = Theme.of(sheet).colorScheme;
          return ListenableBuilder(
            listenable: c,
            builder: (sheet, _) {
              final creators = c.creators
                  .where(
                    (choice) => !choice.spec.localOnly || target.source.local,
                  )
                  .toList();
              return ConstrainedBox(
                constraints: BoxConstraints(
                  maxHeight: MediaQuery.sizeOf(sheet).height * .84,
                ),
                child: SingleChildScrollView(
                  padding: const EdgeInsets.fromLTRB(20, 0, 20, 28),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Row(
                        children: [
                          Expanded(
                            child: Text(
                              s.createTitle,
                              style: Theme.of(sheet).textTheme.titleLarge,
                            ),
                          ),
                          IconButton(
                            tooltip: s.close,
                            onPressed: () => Navigator.pop(sheet),
                            icon: const Icon(Icons.close),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16),
                      if (c.selectedSource != null &&
                          !c.selectedSource!.source.canCreate)
                        Padding(
                          padding: const EdgeInsets.only(bottom: 16),
                          child: Text(
                            s.sourceReadOnly,
                            style: TextStyle(color: colors.onSurfaceVariant),
                          ),
                        ),
                      Material(
                        color: colors.surface,
                        borderRadius: BorderRadius.circular(14),
                        child: ListTile(
                          leading: const Icon(Icons.folder_outlined),
                          title: Text(
                            s.saveIn,
                            style: const TextStyle(fontSize: 12),
                          ),
                          subtitle: Text(
                            '${target.source.deviceLabel} / ${target.source.mountLabel}',
                          ),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () async {
                            final chosen = await showDialog<ResourceProvider>(
                              context: sheet,
                              builder: (dialog) => SimpleDialog(
                                title: Text(s.saveIn),
                                children: [
                                  for (final p in c.hub.visibleProviders)
                                    SimpleDialogOption(
                                      onPressed:
                                          p.source.canCreate && p.available
                                          ? () => Navigator.pop(dialog, p)
                                          : null,
                                      child: ListTile(
                                        contentPadding: EdgeInsets.zero,
                                        leading: Icon(
                                          p.source.local
                                              ? Icons.phone_iphone_outlined
                                              : Icons.desktop_windows_outlined,
                                        ),
                                        title: Text(
                                          '${p.source.deviceLabel} / ${p.source.mountLabel}',
                                        ),
                                        subtitle: p.source.canCreate
                                            ? null
                                            : Text(s.sourceReadOnly),
                                      ),
                                    ),
                                ],
                              ),
                            );
                            if (chosen != null && sheet.mounted)
                              change(() => target = chosen);
                          },
                        ),
                      ),
                      const SizedBox(height: 24),
                      Text(
                        s.createResource,
                        style: Theme.of(sheet).textTheme.labelMedium,
                      ),
                      const SizedBox(height: 12),
                      if (creators.isEmpty)
                        Padding(
                          padding: const EdgeInsets.symmetric(vertical: 24),
                          child: Text(s.noCreators),
                        )
                      else
                        LayoutBuilder(
                          builder: (context, constraints) {
                            final count = constraints.maxWidth < 280 ? 1 : 2;
                            return Wrap(
                              spacing: 10,
                              runSpacing: 10,
                              children: [
                                for (final choice in creators)
                                  SizedBox(
                                    width:
                                        (constraints.maxWidth -
                                            (count - 1) * 10) /
                                        count,
                                    child: Material(
                                      color: colors.surface,
                                      borderRadius: BorderRadius.circular(14),
                                      child: InkWell(
                                        key: ValueKey(
                                          'resource-creator-${choice.key}',
                                        ),
                                        borderRadius: BorderRadius.circular(14),
                                        onTap: () {
                                          Navigator.pop(sheet);
                                          unawaited(_create(choice, target));
                                        },
                                        child: Padding(
                                          padding: const EdgeInsets.all(16),
                                          child: Column(
                                            crossAxisAlignment:
                                                CrossAxisAlignment.start,
                                            children: [
                                              Icon(
                                                Icons.note_add_outlined,
                                                color: colors.primary,
                                                size: 25,
                                              ),
                                              const SizedBox(height: 14),
                                              Text(
                                                choice.spec.title,
                                                style: const TextStyle(
                                                  fontWeight: FontWeight.w600,
                                                ),
                                              ),
                                              const SizedBox(height: 5),
                                              Text(
                                                choice.plugin.descriptor.name,
                                                style: TextStyle(
                                                  fontSize: 11,
                                                  color:
                                                      colors.onSurfaceVariant,
                                                ),
                                              ),
                                            ],
                                          ),
                                        ),
                                      ),
                                    ),
                                  ),
                              ],
                            );
                          },
                        ),
                      const SizedBox(height: 24),
                      Text(
                        s.importOrganize,
                        style: Theme.of(sheet).textTheme.labelMedium,
                      ),
                      ListTile(
                        key: const ValueKey('resource-import'),
                        contentPadding: EdgeInsets.zero,
                        leading: const Icon(Icons.file_upload_outlined),
                        title: Text(s.importFile),
                        enabled: c.importFile != null,
                        onTap: () {
                          Navigator.pop(sheet);
                          unawaited(_import(target));
                        },
                      ),
                      ListTile(
                        key: const ValueKey('resource-new-folder'),
                        contentPadding: EdgeInsets.zero,
                        leading: const Icon(Icons.create_new_folder_outlined),
                        title: Text(s.newFolder),
                        onTap: () {
                          Navigator.pop(sheet);
                          unawaited(_folder(target));
                        },
                      ),
                    ],
                  ),
                ),
              );
            },
          );
        },
      ),
    );
  }

  Future<void> _create(
    ResourceCreatorChoice creator,
    ResourceProvider target,
  ) async {
    final strings = ResourceCenterStrings(context),
        parent = c.targetParent(target),
        operationKey = resourceOperationKey('create');
    final properties = creator.spec.inputSchema['properties'];
    final fields = [
      _ResourceField(
        'name',
        strings.name,
        creator.spec.defaultName,
        'resource-create-name',
      ),
      if (properties is Map)
        for (final key in properties.keys)
          if (key is String)
            _ResourceField(
              key,
              key == 'url' ? strings.url : key,
              '',
              'resource-create-field-$key',
            ),
    ];
    final created = await showDialog<ResourceRecord>(
      context: context,
      barrierDismissible: false,
      builder: (_) => _ResourceForm<ResourceRecord>(
        title: creator.spec.title,
        description:
            '${target.source.deviceLabel} / ${target.source.mountLabel}',
        confirm: strings.createOpen,
        fields: fields,
        confirmKey: 'resource-create-confirm',
        submit: (values) => c.create(creator, target, parent, values['name']!, {
          for (final e in values.entries)
            if (e.key != 'name') e.key: e.value,
        }, operationKey),
      ),
    );
    if (created != null && mounted) {
      final opened = await openResourcePage(context, c, created.reference());
      if (!opened && mounted)
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text(strings.createdOpenFailed)));
    }
  }

  Future<void> _import(ResourceProvider target) async {
    try {
      final record = await c.importResource(destination: target);
      if (record != null && mounted)
        await openResourcePage(context, c, record.reference());
    } on Object catch (error) {
      if (mounted) showResourceError(context, error);
    }
  }

  Future<void> _folder(ResourceProvider target) async {
    final strings = ResourceCenterStrings(context);
    await showDialog<bool>(
      context: context,
      builder: (_) => _ResourceForm<bool>(
        title: strings.newFolder,
        confirm: strings.createTitle,
        fields: [
          _ResourceField('name', strings.name, '', 'resource-folder-name'),
        ],
        confirmKey: 'resource-folder-confirm',
        submit: (values) async {
          await c.directory(values['name']!, destination: target);
          return true;
        },
      ),
    );
  }

  Future<ResourceRecord?> _describeEntry(ResourceDirectoryEntry entry) async {
    try {
      final identity = entry.deferredIdentity!;
      return await c.describe(identity);
    } on Object catch (error) {
      if (mounted) showResourceError(context, error);
      return null;
    }
  }

  Widget _deferredRow(ResourceDirectoryEntry entry) => ListTile(
    key: ValueKey('resource-row-${entry.nodeRef}'),
    contentPadding: EdgeInsets.zero,
    leading: _ResourceIcon(
      mediaType: entry.mediaType ?? 'application/octet-stream',
    ),
    title: Text(entry.name, maxLines: 2, overflow: TextOverflow.ellipsis),
    onTap: () async {
      final r = await _describeEntry(entry);
      if (r != null && mounted)
        await openResourcePage(context, c, r.reference());
    },
    onLongPress: () async {
      final r = await _describeEntry(entry);
      if (r != null && mounted) await _menu(r);
    },
    trailing: IconButton(
      tooltip: ResourceCenterStrings(context).more,
      icon: const Icon(Icons.more_vert),
      onPressed: () async {
        final r = await _describeEntry(entry);
        if (r != null && mounted) await _menu(r);
      },
    ),
  );
  @override
  Widget build(BuildContext context) => Material(
    type: MaterialType.transparency,
    child: ListenableBuilder(
      listenable: c,
      builder: (context, _) {
        final s = ResourceCenterStrings(context),
            colors = Theme.of(context).colorScheme;
        return PopScope(
          canPop: c.selectedSource == null,
          onPopInvokedWithResult: (popped, result) {
            if (!popped)
              unawaited(
                c.breadcrumb(
                  c.breadcrumbs.length > 1 ? c.breadcrumbs.length - 2 : -1,
                ),
              );
          },
          child: Stack(
            children: [
              Column(
                children: [
                  Padding(
                    padding: const EdgeInsets.fromLTRB(20, 15, 12, 8),
                    child: Row(
                      children: [
                        Expanded(
                          child: Text(
                            widget.title ?? s.home,
                            style: const TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.w600,
                              letterSpacing: -.5,
                            ),
                          ),
                        ),
                        IconButton(
                          key: const ValueKey('resource-refresh'),
                          tooltip: s.refresh,
                          onPressed: () => unawaited(c.refresh()),
                          icon: const Icon(Icons.refresh_rounded),
                        ),
                      ],
                    ),
                  ),
                  SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: Row(
                      children: [
                        for (final section in ResourceCenterSection.values)
                          Padding(
                            padding: const EdgeInsets.only(left: 20, right: 6),
                            child: Semantics(
                              selected: c.section == section,
                              child: InkWell(
                                key: ValueKey(
                                  'resource-section-${section.name}',
                                ),
                                onTap: () => _section(section),
                                child: Container(
                                  padding: const EdgeInsets.symmetric(
                                    vertical: 15,
                                  ),
                                  decoration: BoxDecoration(
                                    border: Border(
                                      bottom: BorderSide(
                                        width: 3,
                                        color: c.section == section
                                            ? colors.primary
                                            : Colors.transparent,
                                      ),
                                    ),
                                  ),
                                  child: Text(
                                    section == ResourceCenterSection.recent
                                        ? s.recent
                                        : section == ResourceCenterSection.local
                                        ? s.thisDevice
                                        : widget.remoteLabel ?? s.remote,
                                    style: TextStyle(
                                      fontSize: 19,
                                      fontWeight: c.section == section
                                          ? FontWeight.w600
                                          : FontWeight.w400,
                                      color: c.section == section
                                          ? colors.onSurface
                                          : colors.onSurfaceVariant,
                                    ),
                                  ),
                                ),
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                  Divider(height: 1, color: colors.outlineVariant),
                  Expanded(
                    child: RefreshIndicator(
                      onRefresh: c.refresh,
                      child: ListView(
                        key: PageStorageKey(
                          'resource-list-${c.section.name}-${c.selectedSourceKey}-${c.parentRef}',
                        ),
                        padding: const EdgeInsets.fromLTRB(20, 16, 20, 100),
                        children: [
                          TextField(
                            key: const ValueKey('resource-search'),
                            controller: search,
                            onChanged: c.setQuery,
                            decoration: InputDecoration(
                              hintText:
                                  c.section == ResourceCenterSection.recent
                                  ? s.searchRecent
                                  : s.search,
                              prefixIcon: const Icon(Icons.search, size: 21),
                              filled: true,
                              fillColor: colors.surfaceContainer,
                              border: OutlineInputBorder(
                                borderRadius: BorderRadius.circular(12),
                                borderSide: BorderSide.none,
                              ),
                              contentPadding: const EdgeInsets.symmetric(
                                vertical: 10,
                                horizontal: 12,
                              ),
                            ),
                          ),
                          if (c.selectedSource != null)
                            SingleChildScrollView(
                              scrollDirection: Axis.horizontal,
                              child: Row(
                                children: [
                                  TextButton(
                                    onPressed: () =>
                                        unawaited(c.breadcrumb(-1)),
                                    child: Text(
                                      c.selectedSource!.source.deviceLabel,
                                    ),
                                  ),
                                  for (
                                    var i = 0;
                                    i < c.breadcrumbs.length;
                                    i++
                                  ) ...[
                                    const Icon(Icons.chevron_right, size: 15),
                                    TextButton(
                                      onPressed: () =>
                                          unawaited(c.breadcrumb(i)),
                                      child: Text(c.breadcrumbs[i].name),
                                    ),
                                  ],
                                ],
                              ),
                            ),
                          const SizedBox(height: 20),
                          if (c.failure != null)
                            _StateMessage(
                              icon: Icons.cloud_off_outlined,
                              title: resourceErrorText(context, c.failure!),
                              action: s.retry,
                              onAction: () => unawaited(c.refresh()),
                            )
                          else if (c.loading && c.entries.isEmpty) ...[
                            for (var i = 0; i < 5; i++)
                              const _ResourceSkeleton(),
                          ] else if (c.section == ResourceCenterSection.recent)
                            ..._recent(context)
                          else if (c.selectedSource == null)
                            ..._sources(context)
                          else if (c.filteredEntries.isEmpty)
                            _StateMessage(
                              icon: Icons.folder_open_outlined,
                              title: c.query.isEmpty
                                  ? s.emptyFolder
                                  : s.noResults,
                              body: c.query.isEmpty
                                  ? s.emptyHint
                                  : s.noResultsHint,
                              action: c.creationTarget != null
                                  ? s.createTitle
                                  : null,
                              onAction: () => unawaited(_new()),
                            )
                          else ...[
                            for (final entry in c.filteredEntries)
                              if (entry.isDirectory)
                                ListTile(
                                  key: ValueKey(
                                    'resource-directory-${entry.nodeRef}',
                                  ),
                                  contentPadding: EdgeInsets.zero,
                                  leading: const _ResourceIcon(
                                    mediaType: 'inode/directory',
                                  ),
                                  title: Text(entry.name),
                                  trailing: const Icon(Icons.chevron_right),
                                  onTap: () => unawaited(c.enter(entry)),
                                )
                              else if (entry.resource != null)
                                _recordRow(entry.resource!)
                              else if (entry.deferredIdentity != null)
                                _deferredRow(entry),
                            if (c.cursor != null)
                              TextButton(
                                onPressed: c.loading
                                    ? null
                                    : () => unawaited(c.refresh(more: true)),
                                child: Text(s.refresh),
                              ),
                          ],
                        ],
                      ),
                    ),
                  ),
                ],
              ),
              if (c.creationTarget != null)
                Positioned(
                  right: 20,
                  bottom: 16,
                  child: FloatingActionButton(
                    key: const ValueKey('resource-new'),
                    heroTag: null,
                    onPressed: () => unawaited(_new()),
                    tooltip: s.createResource,
                    backgroundColor: colors.primary,
                    foregroundColor: colors.onPrimary,
                    child: const Icon(Icons.add_rounded, size: 29),
                  ),
                ),
            ],
          ),
        );
      },
    ),
  );
  List<Widget> _recent(BuildContext context) {
    final s = ResourceCenterStrings(context);
    if (c.recent.isEmpty)
      return [
        _StateMessage(
          icon: Icons.history,
          title: c.query.isEmpty ? s.noRecent : s.noResults,
          body: c.query.isEmpty ? s.noRecentHint : s.noResultsHint,
          action: s.browseLocal,
          onAction: () => _section(ResourceCenterSection.local),
        ),
      ];
    final now = DateTime.now();
    bool? lastToday;
    final widgets = <Widget>[];
    for (final recent in c.recent) {
      final date = DateTime.fromMillisecondsSinceEpoch(recent.openedAtMs);
      final today =
          date.year == now.year &&
          date.month == now.month &&
          date.day == now.day;
      if (lastToday != today) {
        lastToday = today;
        widgets.add(
          Padding(
            padding: const EdgeInsets.only(top: 12, bottom: 8),
            child: Text(
              today ? s.today : s.earlier,
              style: TextStyle(
                fontSize: 12,
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
            ),
          ),
        );
      }
      widgets.add(_recordRow(recent.record));
    }
    return widgets;
  }

  List<Widget> _sources(BuildContext context) {
    final strings = ResourceCenterStrings(context),
        colors = Theme.of(context).colorScheme;
    if (c.sources.isEmpty)
      return [
        _StateMessage(
          icon: Icons.devices_other_outlined,
          title: strings.noSources,
          body: c.section == ResourceCenterSection.remote
              ? strings.connectPc
              : strings.emptyHint,
          action: strings.refresh,
          onAction: () => unawaited(c.refresh()),
        ),
      ];
    final groups = <String, List<ResourceProvider>>{};
    for (final p in c.sources) {
      groups
          .putIfAbsent('${p.source.deviceRef}:${p.source.workspace}', () => [])
          .add(p);
    }
    return [
      for (final group in groups.values) ...[
        Padding(
          padding: const EdgeInsets.only(top: 8, bottom: 12),
          child: Row(
            children: [
              Icon(
                c.section == ResourceCenterSection.local
                    ? Icons.phone_iphone_outlined
                    : Icons.desktop_windows_outlined,
                size: 19,
                color: colors.onSurfaceVariant,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Text(
                  group.first.source.deviceLabel,
                  style: const TextStyle(fontWeight: FontWeight.w600),
                ),
              ),
            ],
          ),
        ),
        Material(
          color: colors.surface,
          borderRadius: BorderRadius.circular(16),
          clipBehavior: Clip.antiAlias,
          child: ExpansionTile(
            key: PageStorageKey<String>(
              'resource-workspace-${group.first.source.scope}-${group.first.source.deviceRef}-${group.first.source.workspace}',
            ),
            initiallyExpanded: true,
            shape: const Border(),
            collapsedShape: const Border(),
            leading: const Icon(Icons.workspaces_outline),
            title: Text(group.first.source.workspaceLabel),
            children: [
              for (final p in group)
                ListTile(
                  key: ValueKey('resource-source-${p.source.key}'),
                  contentPadding: const EdgeInsets.only(left: 54, right: 16),
                  leading: const Icon(Icons.folder_outlined, size: 20),
                  title: Text(p.source.mountLabel),
                  subtitle: Text(
                    p.available ? strings.online : strings.offline,
                  ),
                  trailing: const Icon(Icons.chevron_right),
                  onTap: () => unawaited(c.selectSource(p)),
                ),
            ],
          ),
        ),
        const SizedBox(height: 22),
      ],
    ];
  }

  Widget _recordRow(ResourceRecord record) => ResourceRow(
    record: record,
    onOpen: () => unawaited(openResourcePage(context, c, record.reference())),
    onMenu: () => unawaited(_menu(record)),
  );
}

final class ResourceRow extends StatelessWidget {
  const ResourceRow({
    super.key,
    required this.record,
    required this.onOpen,
    required this.onMenu,
  });
  final ResourceRecord record;
  final VoidCallback onOpen, onMenu;
  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme,
        s = ResourceCenterStrings(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: colors.outlineVariant.withValues(alpha: .6),
          ),
        ),
      ),
      child: Row(
        children: [
          Expanded(
            child: InkWell(
              key: ValueKey('resource-row-${record.identity.resource}'),
              onTap: onOpen,
              onLongPress: onMenu,
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 17),
                child: ResourceSummary(record: record),
              ),
            ),
          ),
          SizedBox(
            width: 44,
            height: 48,
            child: IconButton(
              key: ValueKey('resource-more-${record.identity.resource}'),
              tooltip: s.more,
              onPressed: onMenu,
              icon: Icon(
                Icons.more_vert,
                size: 21,
                color: colors.onSurfaceVariant,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

final class ResourceSummary extends StatelessWidget {
  const ResourceSummary({super.key, required this.record});
  final ResourceRecord record;
  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Row(
      children: [
        _ResourceIcon(mediaType: record.mediaType),
        const SizedBox(width: 13),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                record.name,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(
                  fontSize: 15.5,
                  fontWeight: FontWeight.w500,
                  height: 1.4,
                ),
              ),
              const SizedBox(height: 5),
              Text(
                [record.sourceLabel, ...record.location].join(' · '),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

final class _ResourceIcon extends StatelessWidget {
  const _ResourceIcon({required this.mediaType});
  final String mediaType;
  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme;
    final icon = mediaType == 'inode/directory'
        ? Icons.folder_outlined
        : mediaType.startsWith('image/')
        ? Icons.image_outlined
        : mediaType.startsWith('video/')
        ? Icons.video_file_outlined
        : mediaType == 'text/uri-list'
        ? Icons.link_rounded
        : Icons.description_outlined;
    return Container(
      width: 40,
      height: 44,
      decoration: BoxDecoration(
        color: c.primary.withValues(alpha: .09),
        borderRadius: BorderRadius.circular(10),
      ),
      child: Icon(icon, color: c.primary, size: 23),
    );
  }
}

final class _StateMessage extends StatelessWidget {
  const _StateMessage({
    required this.icon,
    required this.title,
    this.body,
    this.action,
    this.onAction,
  });
  final IconData icon;
  final String title;
  final String? body, action;
  final VoidCallback? onAction;
  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: 60, horizontal: 14),
    child: Column(
      children: [
        Icon(
          icon,
          size: 40,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
        const SizedBox(height: 22),
        Text(
          title,
          textAlign: TextAlign.center,
          style: const TextStyle(
            fontSize: 17,
            fontWeight: FontWeight.w500,
            height: 1.5,
          ),
        ),
        if (body != null) ...[
          const SizedBox(height: 9),
          Text(
            body!,
            textAlign: TextAlign.center,
            style: TextStyle(
              fontSize: 13,
              height: 1.7,
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
        ],
        if (action != null) ...[
          const SizedBox(height: 18),
          OutlinedButton(onPressed: onAction, child: Text(action!)),
        ],
      ],
    ),
  );
}

final class _ResourceSkeleton extends StatelessWidget {
  const _ResourceSkeleton();
  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.surfaceContainer;
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 17),
      child: Row(
        children: [
          Container(
            width: 40,
            height: 44,
            decoration: BoxDecoration(
              color: color,
              borderRadius: BorderRadius.circular(9),
            ),
          ),
          const SizedBox(width: 13),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                FractionallySizedBox(
                  widthFactor: .8,
                  child: Container(height: 13, color: color),
                ),
                const SizedBox(height: 11),
                FractionallySizedBox(
                  widthFactor: .55,
                  child: Container(height: 9, color: color),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

final class _ResourceField {
  const _ResourceField(this.id, this.label, this.initial, this.widgetKey);
  final String id, label, initial, widgetKey;
}

final class _ResourceForm<T> extends StatefulWidget {
  const _ResourceForm({
    required this.title,
    required this.confirm,
    required this.fields,
    required this.confirmKey,
    required this.submit,
    this.description,
  });
  final String title, confirm, confirmKey;
  final String? description;
  final List<_ResourceField> fields;
  final Future<T> Function(Map<String, String>) submit;
  @override
  State<_ResourceForm<T>> createState() => _ResourceFormState<T>();
}

final class _ResourceFormState<T> extends State<_ResourceForm<T>> {
  late final inputs = {
    for (final field in widget.fields)
      field.id: TextEditingController(text: field.initial),
  };
  bool busy = false;
  String? failure;
  @override
  void dispose() {
    for (final input in inputs.values) {
      input.dispose();
    }
    super.dispose();
  }

  Future<void> submit() async {
    setState(() {
      busy = true;
      failure = null;
    });
    try {
      final result = await widget.submit({
        for (final e in inputs.entries) e.key: e.value.text,
      });
      if (mounted) Navigator.pop(context, result);
    } on Object catch (error) {
      if (mounted) setState(() => failure = resourceErrorText(context, error));
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  @override
  Widget build(BuildContext context) => PopScope(
    canPop: !busy,
    child: AlertDialog(
      title: Text(widget.title),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (widget.description != null) ...[
              Text(
                widget.description!,
                style: Theme.of(context).textTheme.bodySmall,
              ),
              const SizedBox(height: 18),
            ],
            for (final field in widget.fields) ...[
              Text(field.label),
              const SizedBox(height: 8),
              TextField(
                key: ValueKey(field.widgetKey),
                controller: inputs[field.id],
                enabled: !busy,
                maxLength: field.id == 'name' ? 240 : 2048,
                decoration: const InputDecoration(border: OutlineInputBorder()),
              ),
              const SizedBox(height: 10),
            ],
            if (failure != null)
              Text(
                failure!,
                key: const ValueKey('resource-create-error'),
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: busy ? null : () => Navigator.pop(context),
          child: Text(ResourceCenterStrings(context).cancel),
        ),
        FilledButton(
          key: ValueKey(widget.confirmKey),
          onPressed: busy ? null : submit,
          child: Text(
            busy ? ResourceCenterStrings(context).creating : widget.confirm,
          ),
        ),
      ],
    ),
  );
}

/// A Host can initialize its private storage before a controller is available.
/// Keep navigation usable and expose a recoverable failure instead of spinning
/// indefinitely when the platform storage/keychain adapter cannot initialize.
final class ResourceCenterPlaceholder extends StatelessWidget {
  const ResourceCenterPlaceholder({super.key, this.error, this.onRetry});
  final Object? error;
  final VoidCallback? onRetry;
  @override
  Widget build(BuildContext context) {
    final strings = ResourceCenterStrings(context);
    return Material(
      type: MaterialType.transparency,
      child: Column(
        key: ValueKey(
          error == null
              ? 'resource-initializing'
              : 'resource-initialization-error',
        ),
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(20, 20, 20, 16),
            child: Text(
              strings.home,
              style: const TextStyle(fontSize: 24, fontWeight: FontWeight.w600),
            ),
          ),
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(20),
              child: error == null
                  ? const _ResourceSkeleton()
                  : _StateMessage(
                      icon: Icons.folder_off_outlined,
                      title: strings.failed,
                      body: resourceErrorText(context, error!),
                      action: strings.retry,
                      onAction: onRetry ?? () {},
                    ),
            ),
          ),
        ],
      ),
    );
  }
}
