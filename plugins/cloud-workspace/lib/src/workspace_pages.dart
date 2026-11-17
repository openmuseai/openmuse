import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'workspace_access_dialog.dart';
import 'workspace_catalog_controller.dart';
import 'document_creation_controller.dart';
import 'document_creation_dialog.dart';
import 'page_rename_controller.dart';
import 'page_rename_dialog.dart';

typedef OpenCloudDocument = Future<void> Function(AppFlowyDocumentPage page);
typedef OpenCloudDatabase = Future<void> Function(AppFlowyDatabasePage page);
typedef OpenCloudPage = Future<void> Function(AppFlowyWorkspacePage page);

class CloudWorkspacePages extends StatefulWidget {
  const CloudWorkspacePages({
    super.key,
    required this.authentication,
    required this.service,
    required this.workspaceId,
    required this.title,
    this.onOpen,
    this.supportsDocumentCreation = true,
  });
  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService service;
  final String workspaceId, title;
  final OpenCloudPage? onOpen;
  final bool supportsDocumentCreation;
  @override
  State<CloudWorkspacePages> createState() => _CloudWorkspacePagesState();
}

class _CloudWorkspacePagesState extends State<CloudWorkspacePages> {
  late WorkspaceCatalogController _catalog;
  StreamSubscription<void>? _documentSubscription;
  DocumentCreationController? _creationController;
  PageRenameController? _renameController;
  StreamSubscription<void>? _renameSubscription;
  final _search = TextEditingController();
  String tr(String zh, String en) =>
      Localizations.localeOf(context).languageCode == 'zh' ? zh : en;

  @override
  void initState() {
    super.initState();
    _bind();
  }

  void _bind() {
    _catalog = WorkspaceCatalogController(
      authentication: widget.authentication,
      service: widget.service,
      workspaceId: widget.workspaceId,
    )..addListener(_changed);
    final catalog = _catalog;
    _renameSubscription = widget.service.pageRenameChanges.listen((_) {
      if (mounted && identical(catalog, _catalog)) setState(() {});
    });
    _documentSubscription = widget.service.documentCreationChanges.listen(
      (_) {
        if (mounted && identical(catalog, _catalog)) setState(() {});
      },
      onDone: () {
        if (mounted && identical(catalog, _catalog)) {
          _creationController?.retire();
          _renameController?.retire();
          catalog.retire();
        }
      },
    );
    _search.clear();
    unawaited(_catalog.refresh());
  }

  void _changed() {
    if (mounted) {
      if (_catalog.retired) _search.clear();
      setState(() {});
    }
  }

  @override
  void didUpdateWidget(CloudWorkspacePages oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.workspaceId != widget.workspaceId ||
        !identical(oldWidget.service, widget.service) ||
        !identical(oldWidget.authentication, widget.authentication)) {
      _creationController?.retire(deferNotification: true);
      _renameController?.retire(deferNotification: true);
      unawaited(_renameSubscription?.cancel());
      unawaited(_documentSubscription?.cancel());
      _catalog.removeListener(_changed);
      _catalog.dispose();
      _bind();
    } else if (!widget.supportsDocumentCreation || widget.onOpen == null) {
      _creationController?.retire(deferNotification: true);
    }
  }

  @override
  void dispose() {
    _renameController?.retire(deferNotification: true);
    unawaited(_renameSubscription?.cancel());
    _creationController?.retire(deferNotification: true);
    unawaited(_documentSubscription?.cancel());
    _catalog.removeListener(_changed);
    _catalog.dispose();
    _search.dispose();
    super.dispose();
  }

  Future<void> _create(WorkspaceCatalogController catalog) async {
    if (!identical(catalog, _catalog) ||
        !catalog.active ||
        catalog.busy ||
        catalog.openingId != null ||
        _creationController != null ||
        _renameController != null)
      return;
    final before = catalog.service.pendingDocumentCreation(catalog.workspaceId);
    final canOpen = widget.onOpen != null && widget.supportsDocumentCreation;
    if (before == null && (!catalog.canCreateDocument || !canOpen)) return;
    final controller = DocumentCreationController(
      authentication: catalog.authentication,
      service: catalog.service,
      workspaceId: catalog.workspaceId,
    );
    setState(() => _creationController = controller);
    try {
      final page = await showDocumentCreationDialog(
        context,
        controller,
        allowOpen: canOpen,
      );
      if (!mounted || !identical(catalog, _catalog) || !catalog.active) return;
      await catalog.refresh();
      if (!mounted ||
          !identical(catalog, _catalog) ||
          !catalog.active ||
          page == null)
        return;
      final current = catalog.pageForId(page.viewId);
      if (current is! AppFlowyDocumentPage ||
          widget.onOpen == null ||
          !widget.supportsDocumentCreation) {
        catalog.reportCreatedOpenFailure();
        return;
      }
      final pending = catalog.service.pendingDocumentCreation(
        catalog.workspaceId,
      );
      if (pending?.documentId == page.viewId &&
          pending?.status == AppFlowyDocumentCreationStatus.ready) {
        catalog.service.finishDocumentCreation(pending!);
      }
      await _open(catalog, current);
      if (catalog.failure == 'open') catalog.reportCreatedOpenFailure();
    } finally {
      controller.dispose();
      if (identical(_creationController, controller)) {
        _creationController = null;
        if (mounted) setState(() {});
      }
    }
  }

  Future<void> _open(
    WorkspaceCatalogController catalog,
    AppFlowyWorkspacePage page,
  ) async {
    final onOpen = widget.onOpen;
    if (onOpen == null || !identical(catalog, _catalog)) return;
    final completed = await catalog.open(page, onOpen);
    if (completed && mounted && identical(catalog, _catalog)) {
      Navigator.of(context).popUntil(
        (route) => !{
          'openmuse.cloud-pages',
          'openmuse.cloud-workspaces',
        }.contains(route.settings.name),
      );
    }
  }

  Future<void> _rename(
    WorkspaceCatalogController catalog,
    AppFlowyDocumentPage page,
  ) async {
    if (!identical(catalog, _catalog) ||
        !catalog.owns(page) ||
        catalog.busy ||
        catalog.openingId != null ||
        _creationController != null ||
        _renameController != null)
      return;
    final pending = catalog.service.pendingPageRename(
      page.workspaceId,
      page.viewId,
    );
    if (pending == null && !catalog.canCreateDocument) return;
    final controller = PageRenameController(
      authentication: catalog.authentication,
      service: catalog.service,
      page: page,
    );
    setState(() => _renameController = controller);
    try {
      await showPageRenameDialog(context, controller);
      if (mounted && identical(catalog, _catalog) && catalog.active)
        await catalog.refresh();
    } finally {
      controller.dispose();
      if (identical(_renameController, controller)) {
        _renameController = null;
        if (mounted) setState(() {});
      }
    }
  }

  Future<void> _share(
    WorkspaceCatalogController catalog,
    AppFlowyWorkspacePage page,
  ) async {
    if (!identical(catalog, _catalog) ||
        !catalog.owns(page) ||
        catalog.busy ||
        catalog.openingId != null)
      return;
    await showWorkspaceAccessDialog(
      context,
      authentication: catalog.authentication,
      service: catalog.service,
      workspaceId: page.workspaceId,
      viewId: page.viewId,
      ancestors: page.ancestors,
      title: page.name,
    );
    if (mounted && identical(catalog, _catalog)) {
      await catalog.refresh();
    }
  }

  @override
  Widget build(BuildContext context) {
    final catalog = _catalog;
    final entries = catalog.entries;
    final creation = catalog.active
        ? catalog.service.pendingDocumentCreation(catalog.workspaceId)
        : null;
    final enabled =
        catalog.active && !catalog.busy && catalog.openingId == null;
    return Theme(
      data: Theme.of(context).copyWith(
        visualDensity: VisualDensity.standard,
        materialTapTargetSize: MaterialTapTargetSize.padded,
      ),
      child: Scaffold(
        appBar: AppBar(
          title: Text(
            widget.title,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          actions: [
            if (widget.supportsDocumentCreation &&
                widget.onOpen != null &&
                catalog.role != AppFlowyWorkspaceRole.guest)
              IconButton(
                key: const ValueKey('cloud-pages.create'),
                tooltip: tr('新建文档', 'New document'),
                icon: const Icon(Icons.note_add_outlined),
                onPressed:
                    catalog.canCreateDocument &&
                        _creationController == null &&
                        _renameController == null
                    ? () => _create(catalog)
                    : null,
              ),
            IconButton(
              key: const ValueKey('cloud-pages.refresh'),
              tooltip: tr('刷新', 'Refresh'),
              onPressed: enabled ? catalog.refresh : null,
              icon: const Icon(Icons.refresh),
            ),
          ],
        ),
        body: Column(
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
              child: TextField(
                key: const ValueKey('cloud-pages.search'),
                controller: _search,
                enabled: catalog.active,
                maxLength: 256,
                decoration: InputDecoration(
                  labelText: tr('查找文档和数据库', 'Find documents and databases'),
                  prefixIcon: const Icon(Icons.search),
                  counterText: '',
                  suffixIcon: catalog.query.isEmpty
                      ? null
                      : IconButton(
                          key: const ValueKey('cloud-pages.clear-search'),
                          tooltip: tr('清空搜索', 'Clear search'),
                          onPressed: () {
                            _search.clear();
                            catalog.search('');
                          },
                          icon: const Icon(Icons.close),
                        ),
                ),
                onChanged: catalog.search,
              ),
            ),
            if (creation != null)
              ListTile(
                key: const ValueKey('cloud-pages.creation-pending'),
                title: Text(tr('新建文档处理', 'Document creation')),
                subtitle: Text(
                  creation.name,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
                trailing: TextButton(
                  key: const ValueKey('cloud-pages.creation-resume'),
                  onPressed: enabled && _creationController == null
                      ? () => _create(catalog)
                      : null,
                  child: Text(tr('继续核对', 'Continue')),
                ),
              ),
            if (catalog.busy || catalog.openingId != null)
              const LinearProgressIndicator(),
            if (catalog.failure != null)
              Padding(
                padding: const EdgeInsets.all(16),
                child: Text(
                  catalog.failure == 'created_open'
                      ? tr(
                          '文档已创建，暂时无法打开，请刷新目录后重试。',
                          'The document was created but could not be opened. Refresh the catalog to retry.',
                        )
                      : catalog.retired
                      ? tr(
                          '账号已切换，请重新打开工作区。',
                          'Account changed. Reopen the workspace.',
                        )
                      : catalog.failure == 'open'
                      ? tr(
                          '无法打开此资源，请刷新后重试。',
                          'Resource unavailable. Refresh to retry.',
                        )
                      : tr(
                          '目录暂时不可用，请刷新。',
                          'Catalog unavailable. Refresh to retry.',
                        ),
                ),
              ),
            Expanded(
              child: entries.isEmpty && !catalog.busy && catalog.failure == null
                  ? Center(
                      child: Padding(
                        padding: const EdgeInsets.all(24),
                        child: Text(
                          catalog.query.isNotEmpty
                              ? tr(
                                  '没有匹配的文档或数据库',
                                  'No matching documents or databases',
                                )
                              : tr(
                                  '此工作区暂无可用文档或数据库',
                                  'No documents or databases available in this workspace',
                                ),
                        ),
                      ),
                    )
                  : ListView.builder(
                      itemCount: entries.length,
                      itemBuilder: (context, index) {
                        final entry = entries[index], page = entry.page;
                        return ListTile(
                          key: ValueKey('cloud-page.${page.viewId}'),
                          contentPadding: EdgeInsets.only(
                            left: 12 + entry.depth.clamp(0, 4) * 12.0,
                            right: 8,
                          ),
                          leading: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              if (entry.hasChildren)
                                IconButton(
                                  key: ValueKey(
                                    'cloud-page.toggle.${page.viewId}',
                                  ),
                                  tooltip: entry.expanded
                                      ? tr('收起子页面', 'Collapse child pages')
                                      : tr('展开子页面', 'Expand child pages'),
                                  onPressed: enabled && catalog.query.isEmpty
                                      ? () => catalog.toggle(page.viewId)
                                      : null,
                                  icon: Icon(
                                    entry.expanded
                                        ? Icons.expand_more
                                        : Icons.chevron_right,
                                  ),
                                )
                              else
                                const SizedBox(width: 48),
                              Icon(
                                page is AppFlowyDatabasePage
                                    ? Icons.table_chart_outlined
                                    : Icons.description_outlined,
                                size: 20,
                              ),
                            ],
                          ),
                          title: Text(
                            page.name,
                            maxLines: 2,
                            overflow: TextOverflow.ellipsis,
                          ),
                          subtitle: entry.breadcrumb.isEmpty
                              ? null
                              : Text(
                                  entry.breadcrumb,
                                  maxLines: 1,
                                  overflow: TextOverflow.ellipsis,
                                ),
                          onTap: enabled && widget.onOpen != null
                              ? () => _open(catalog, page)
                              : null,
                          trailing: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              if (page is AppFlowyDocumentPage &&
                                  (catalog.canCreateDocument ||
                                      catalog.service.pendingPageRename(
                                            page.workspaceId,
                                            page.viewId,
                                          ) !=
                                          null))
                                IconButton(
                                  key: ValueKey(
                                    'cloud-page.rename.${page.viewId}',
                                  ),
                                  tooltip:
                                      catalog.service.pendingPageRename(
                                            page.workspaceId,
                                            page.viewId,
                                          ) !=
                                          null
                                      ? tr('核对改名结果', 'Review rename result')
                                      : tr('重命名', 'Rename'),
                                  icon: Icon(
                                    catalog.service.pendingPageRename(
                                              page.workspaceId,
                                              page.viewId,
                                            ) !=
                                            null
                                        ? Icons.pending_actions_outlined
                                        : Icons.drive_file_rename_outline,
                                  ),
                                  onPressed:
                                      enabled &&
                                          _renameController == null &&
                                          _creationController == null
                                      ? () => _rename(catalog, page)
                                      : null,
                                ),
                              IconButton(
                                key: ValueKey(
                                  'cloud-page.share.${page.viewId}',
                                ),
                                tooltip: tr('分享', 'Share'),
                                icon: const Icon(Icons.person_add_outlined),
                                onPressed: enabled
                                    ? () => _share(catalog, page)
                                    : null,
                              ),
                            ],
                          ),
                        );
                      },
                    ),
            ),
          ],
        ),
      ),
    );
  }
}
