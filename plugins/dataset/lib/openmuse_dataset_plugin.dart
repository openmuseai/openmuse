library;

import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:uuid/uuid.dart';
import 'src/resource_checkpoint.dart';
import 'src/shared_sessions.dart';
export 'src/resource_checkpoint.dart';
export 'src/shared_sessions.dart' show DatasetResourceLease;

const datasetMediaType = 'application/vnd.openmuse.dataset+json';
typedef DatasetStoreFactory =
    Future<SnapshotStore> Function(ResourceEditorSession session);
typedef DatasetCollaborationFactory =
    DatasetProviderFactory? Function(ResourceRecord record);
typedef DatasetRealtimeFactory =
    Future<DatasetRealtimeSession?> Function(
      ResourceEditorSession session,
      DatasetController controller,
    );

/// Content and collaboration are separate dependencies. The Host supplies only
/// a bounded resource session, persistence and optional collaboration capability.
final class OpenMuseDatasetPlugin
    implements
        OpenMusePlugin,
        OpenMuseResourceHandlerContributor,
        OpenMuseResourceCreatorContributor,
        OpenMuseResourceContextContributor,
        OpenMuseResourceEmbedContributor,
        OpenMuseResourceEmbedValidator {
  OpenMuseDatasetPlugin({
    required this.core,
    required this.storeFactory,
    this.collaborationFactory,
    this.realtimeFactory,
    this.viewIdFor,
    this.headerActions,
    this.allowRowCreationFor,
    this.rowCreationFor,
    this.linkOpener,
    this.ownerAssetFactory,
    this.recordDocumentFactory,
  });
  final DatasetRecordDocumentPort? Function(
    ResourceEditorSession,
    DatasetController,
  )?
  recordDocumentFactory;
  final DatasetCore core;
  final DatasetStoreFactory storeFactory;
  final DatasetCollaborationFactory? collaborationFactory;
  final DatasetRealtimeFactory? realtimeFactory;
  final String? Function(ResourceRecord)? viewIdFor;
  final List<Widget> Function(BuildContext, ResourceEditorSession)?
  headerActions;
  final bool Function(ResourceRecord)? allowRowCreationFor;
  final DatasetRowCreationPort? Function(ResourceRecord)? rowCreationFor;
  final DatasetLinkOpener? linkOpener;
  final Future<DatasetAssetSource?> Function(
    ResourceEditorSession,
    DatasetController,
  )?
  ownerAssetFactory;
  final Map<ResourceSessionScope, DatasetAssetSource> _assets = {};
  late final _owners = DatasetSessionOwners(
    open: openController,
    release: _releaseController,
    objectId: (bytes) => decode(bytes)['objectId'] as String,
    realtime: (scope) => _realtime[scope],
    prepareAuthority: (c, next) async {
      final seed = decode(next.bytes);
      if (seed['objectId'] != c.objectId)
        throw const ResourceFailure('RESOURCE_CONTENT_CHANGED');
      final ticket = await c.prepareWriteAuthority(
        seed,
        rowCreation: rowCreationFor?.call(next.record),
      );
      return ResourceOwnerAuthorityPreparation(
        active: () => ticket.isActive,
        commit: ticket.commit,
        cancel: ticket.cancel,
        complete: ticket.complete,
      );
    },
  );
  Future<DatasetResourceLease> acquireResourceSession(
    ResourceEditorSession session,
  ) => _owners.acquire(session);
  final Map<ResourceSessionScope, DatasetRealtimeSession> _realtime = {};
  DatasetRealtimeSession? realtimeFor(
    ResourceIdentity id, {
    ResourceVersion? snapshotVersion,
  }) => _realtime[ResourceSessionScope(id, snapshotVersion: snapshotVersion)];
  final Map<ResourceSessionScope, DatasetController> _controllers = {};
  final Set<ResourceSessionScope> _opening = {};
  DatasetController? activeController(
    ResourceIdentity id, {
    ResourceVersion? snapshotVersion,
  }) =>
      _controllers[ResourceSessionScope(id, snapshotVersion: snapshotVersion)];
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'io.openmuse.dataset',
    name: '数据库',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'resource.read', 'resource.create', 'resource.edit'},
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _owners.activate();
  }

  @override
  Future<void> deactivate() async {
    await _owners.close();
    for (final rt in _realtime.values.toList()) {
      await rt.close();
    }
    _realtime.clear();
    for (final entry in _controllers.entries.toList()) {
      await entry.value.close();
      _controllers.remove(entry.key);
    }
  }

  @override
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'dataset.database',
      title: '数据库',
      mediaTypes: const {datasetMediaType},
      extensions: const {'musedataset'},
      editable: true,
      supportsSharedPresentations: true,
      priority: 40,
    ),
  ];
  @override
  List<ResourceCreatorSpec> get resourceCreators => [
    ResourceCreatorSpec(
      id: 'dataset.database',
      title: '数据库',
      mediaType: datasetMediaType,
      extension: 'musedataset',
      defaultName: '未命名.musedataset',
    ),
  ];
  static List<int> exportBundle(Map<String, dynamic> bundle) =>
      utf8.encode(jsonEncode(bundle));
  static Map<String, dynamic> decode(List<int> bytes) {
    if (bytes.length > 16 * 1024 * 1024)
      throw const ResourceFailure('CONTENT_TOO_LARGE');
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map ||
        value['profile'] != 'muse.dataset.bundle/0.1' ||
        value['objectId'] is! String ||
        value['objects'] is! List ||
        value.keys.any(
          (k) => !{'profile', 'objectId', 'objects'}.contains(k),
        )) {
      throw const ResourceFailure('INVALID_DATASET');
    }
    return value.cast<String, dynamic>();
  }

  @override
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  ) async {
    if (creatorId != 'dataset.database')
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    final state = core.call({'action': 'open', 'objectId': const Uuid().v4()});
    try {
      return ResourceSeed(
        bytes: exportBundle((state['bundle'] as Map).cast<String, dynamic>()),
        mediaType: datasetMediaType,
      );
    } finally {
      core.call({'action': 'close', 'handle': state['handle']});
    }
  }

  Future<DatasetController> openController(
    ResourceEditorSession session,
  ) async {
    final key = session.ownershipScope;
    if (_controllers.containsKey(key) || !_opening.add(key))
      throw const ResourceFailure('RESOURCE_ALREADY_OPEN');
    try {
      final bundle = decode(session.bytes);
      final historical = session.opened.fromSnapshot;
      final factory = historical
          ? null
          : collaborationFactory?.call(session.record);
      final c = await DatasetController.open(
        core: core,
        objectId: bundle['objectId'] as String,
        initialBundle: bundle,
        store: historical ? _SnapshotMemory() : await storeFactory(session),
        providerFactory: factory,
        editable: session.allowEditing && !historical,
        createRemote: factory != null && session.allowEditing && !historical,
        rowCreation: historical ? null : rowCreationFor?.call(session.record),
      );
      try {
        if (!historical && ownerAssetFactory != null) {
          final source = await ownerAssetFactory!(session, c);
          if (source != null) _assets[key] = source;
        }
        if (!historical && realtimeFactory != null) {
          final realtime = await realtimeFactory!(session, c);
          if (realtime != null) {
            if (!identical(realtime.controller, c)) {
              await realtime.close();
              throw const ResourceFailure('REALTIME_CONTROLLER_MISMATCH');
            }
            _realtime[key] = realtime;
            await realtime.start();
          }
        }
      } on Object {
        _assets.remove(key)?.close();
        await _realtime.remove(key)?.close();
        await c.close();
        rethrow;
      }
      _controllers[key] = c;
      return c;
    } finally {
      _opening.remove(key);
    }
  }

  Future<void> release(
    ResourceEditorSession session,
    DatasetController c,
  ) async {
    if (_owners.owns(session.ownershipScope))
      throw const ResourceFailure('RESOURCE_SHARED_OWNER');
    await _releaseController(session, c);
  }

  Future<void> _releaseController(
    ResourceEditorSession session,
    DatasetController c, {
    Future<void> Function()? beforeRelease,
  }) async {
    final key = session.ownershipScope;
    if (!identical(_controllers[key], c))
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    await c.close(
      beforeRelease: () async {
        await beforeRelease?.call();
        await _realtime[key]?.close();
      },
    );
    _assets.remove(key)?.close();
    _realtime.remove(key);
    _controllers.remove(key);
  }

  /// The Host resolves/authorizes the child first. This method receives the same
  /// bounded SDK session as a standalone editor; no parent document authority
  /// or blanket resource provider crosses the boundary.
  Future<DatasetEmbedSession> openEmbeddedSession(
    ResourceEditorSession session,
  ) async {
    final lease = await acquireResourceSession(session);
    final checkpoint = lease.checkpoint;
    return DatasetEmbedSession(
      assets: _assets[session.ownershipScope],
      realtime: lease.realtime,
      allowEditing: lease.allowEditing,
      checkpointStatus: checkpoint,
      checkpointFailure: () => checkpoint.failure,
      flushCheckpoint: lease.flush,
      controller: lease.controller,
      title: session.record.name,
      close: lease.close,
    );
  }

  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async =>
      record.mediaType == datasetMediaType ||
      record.name.toLowerCase().endsWith('.musedataset');
  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  ) => _DatasetResourceView(plugin: this, session: session);
  @override
  Set<String> get resourceEmbedSurfaces => const {'dataset.grid'};
  @override
  Future<void> validateResourceEmbedTarget(
    OpenedResource opened,
    ResourceEmbedTarget target,
  ) async {
    if (target.surface != 'dataset.grid' ||
        target.selector.length != 2 ||
        target.selector['objectId'] !=
            decode(opened.content.bytes)['objectId']) {
      throw const ResourceFailure('EMBED_RESOURCE_MISMATCH');
    }
    final targets = await resourceEmbedTargets(opened, target.surface);
    if (!targets.any(
      (candidate) =>
          candidate.target.selector['viewId'] == target.selector['viewId'],
    )) {
      throw const ResourceFailure('EMBED_RESOURCE_MISMATCH');
    }
  }

  @override
  Future<List<({String title, ResourceEmbedTarget target})>>
  resourceEmbedTargets(OpenedResource opened, String surface) async {
    if (!resourceEmbedSurfaces.contains(surface))
      throw const ResourceFailure('EMBED_UNAVAILABLE');
    final bundle = decode(opened.content.bytes);
    final state = core.call({
      'action': 'open',
      'objectId': bundle['objectId'],
      'bundle': bundle,
    });
    try {
      return [
        for (final view in (state['views'] as List).whereType<Map>())
          if (DatasetViewProjection.fromJson(view).supportedLayout)
            (
              title: view['name'] as String? ?? '表格',
              target: ResourceEmbedTarget(
                reference: ResourceReference(
                  identity: opened.content.record.identity,
                  version: opened.fromSnapshot
                      ? opened.content.record.version
                      : null,
                ),
                surface: surface,
                selector: {
                  'objectId': bundle['objectId'] as String,
                  'viewId': view['id'] as String,
                },
              ),
            ),
      ];
    } finally {
      core.call({'action': 'close', 'handle': state['handle']});
    }
  }

  @override
  Widget buildResourceEmbed(
    BuildContext context,
    ResourceEditorSession session,
    ResourceEmbedTarget target,
    ResourceEmbedPolicy policy,
  ) => _DatasetResourceView(
    plugin: this,
    session: session,
    target: target,
    policy: policy,
  );

  @override
  Future<String?> resourceConversationText(ResourceContent content) async {
    if (content.record.mediaType != datasetMediaType) return null;
    final bundle = decode(content.bytes);
    final state = core.call({
      'action': 'open',
      'objectId': bundle['objectId'],
      'bundle': bundle,
    });
    try {
      // One view avoids duplicating the same independently stored row in context.
      final views = state['views'] as List;
      if (views.isEmpty) return '';
      final view = views.first as Map;
      final columns = view['columns'] as List;
      final lines = <String>[columns.map((f) => f['name']).join('\t')];
      for (final row in view['rows'] as List) {
        if (row['loaded'] != true) continue;
        lines.add(
          columns
              .map((f) => jsonEncode(row['cells'][f['id']]['value']))
              .join('\t'),
        );
      }
      final text = lines.join('\n');
      if (utf8.encode(text).length > 128 * 1024)
        throw const ResourceFailure('CONTEXT_TOO_LARGE');
      return text;
    } finally {
      core.call({'action': 'close', 'handle': state['handle']});
    }
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Requires ResourceEditorSession');
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _SnapshotMemory implements SnapshotStore {
  Map<String, dynamic>? _value;
  @override
  Future<Map<String, dynamic>?> read() async => _value;
  @override
  Future<void> write(Map<String, dynamic> value) async => _value = value;
}

class _DatasetResourceView extends StatefulWidget {
  const _DatasetResourceView({
    required this.plugin,
    required this.session,
    this.target,
    this.policy,
  });
  final OpenMuseDatasetPlugin plugin;
  final ResourceEditorSession session;
  final ResourceEmbedTarget? target;
  final ResourceEmbedPolicy? policy;
  @override
  State<_DatasetResourceView> createState() => _DatasetResourceViewState();
}

class _DatasetResourceViewState extends State<_DatasetResourceView> {
  DatasetResourceLease? lease;
  DatasetController? c;
  DatasetResourceCheckpoint? checkpoint;
  String? resourceViewId;
  DatasetRecordDocumentPort? recordDocuments;
  Object? failure;
  bool get _canUseResourcePorts =>
      widget.session.isPresentationActive && c?.canRead == true;
  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() async {
    try {
      if (!mounted) return;
      if (!widget.session.isPresentationActive) {
        throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      }
      final target = widget.target;
      if (target != null) {
        await widget.plugin.validateResourceEmbedTarget(
          widget.session.opened,
          target,
        );
      }
      if (!mounted) return;
      if (!widget.session.isPresentationActive) {
        throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      }
      final selectedViewId =
          widget.target?.selector['viewId'] ??
          widget.plugin.viewIdFor?.call(widget.session.record);
      final acquired = await widget.plugin.acquireResourceSession(
        widget.session,
      );
      if (!widget.session.opened.fromSnapshot &&
          (widget.policy?.allowLive ?? true)) {
        recordDocuments = widget.plugin.recordDocumentFactory?.call(
          widget.session,
          acquired.controller,
        );
      }
      if (target != null &&
          (target.surface != 'dataset.grid' ||
              target.selector.length != 2 ||
              target.selector['objectId'] != acquired.controller.objectId ||
              !acquired.controller.views.any(
                (v) => v.id == target.selector['viewId'] && v.supportedLayout,
              ))) {
        await acquired.close();
        throw const ResourceFailure('EMBED_RESOURCE_MISMATCH');
      }
      if (!mounted ||
          acquired.isClosed ||
          !widget.session.isPresentationActive) {
        await acquired.close();
        if (mounted) throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
        return;
      }
      lease = acquired;
      c = acquired.controller;
      resourceViewId = selectedViewId;
      checkpoint = acquired.checkpoint..addListener(_changed);
      setState(() {});
    } on Object catch (e) {
      if (mounted) setState(() => failure = e);
    }
  }

  void _changed() {
    if (mounted) setState(() {});
  }

  Future<bool> _openLink(Uri target) async {
    if (!mounted || !_canUseResourcePorts || c?.isClosed != false) return false;
    if (target.scheme == 'openmuse-resource') {
      final reference = ResourceReference.fromUri(target.toString());
      final routing = ResourceLinkRouting.maybeOf(context);
      if (routing == null) return false;
      await routing.open(reference.toUri().toString());
      return true;
    }
    return widget.plugin.linkOpener?.call(target) ?? false;
  }

  @override
  void dispose() {
    final adapter = checkpoint, acquired = lease;
    adapter?.removeListener(_changed);
    if (acquired != null)
      unawaited(
        (() async {
          await acquired.close();
        })().catchError((Object e, StackTrace s) {
          FlutterError.reportError(
            FlutterErrorDetails(
              exception: e,
              stack: s,
              library: 'openmuse_dataset_plugin',
            ),
          );
        }),
      );
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => c == null
      ? Center(
          child: failure == null
              ? const CircularProgressIndicator()
              : const Text('数据库无法打开，请检查资源与访问权限'),
        )
      : Column(
          children: [
            if (checkpoint?.failure != null)
              MaterialBanner(
                content: const Text('资源保存失败，本机修改保留，请重试'),
                actions: [
                  TextButton(
                    onPressed: () async {
                      try {
                        await lease!.flush();
                        if (mounted) setState(() {});
                      } on Object {}
                    },
                    child: const Text('重试'),
                  ),
                ],
              ),
            Expanded(
              child: DatasetRecordDocumentScope(
                port: recordDocuments,
                child: DatasetAssetScope(
                  source: widget.plugin._assets[widget.session.ownershipScope],
                  child: DatasetSurface(
                    allowRecordNavigation: !widget.session.opened.fromSnapshot,
                    controller: c!,
                    title: widget.session.record.name,
                    linkCapabilities: DatasetLinkCapabilities(
                      open: _openLink,
                      isActive: () => mounted && _canUseResourcePorts,
                      isEditable: () =>
                          widget.session.allowEditing &&
                          !widget.session.opened.fromSnapshot &&
                          (widget.policy?.allowEditing ?? true),
                      changes: Listenable.merge([
                        widget.session,
                        if (widget.session.lifetime != null)
                          widget.session.lifetime!,
                      ]),
                    ),
                    realtime: _canUseResourcePorts ? lease?.realtime : null,
                    headerActions: !_canUseResourcePorts
                        ? const []
                        : widget.plugin.headerActions?.call(
                                context,
                                widget.session,
                              ) ??
                              const [],
                    allowRowCreation:
                        _canUseResourcePorts &&
                        (widget.plugin.allowRowCreationFor?.call(
                              widget.session.record,
                            ) ??
                            true),
                    embedded: widget.target != null,
                    viewId: resourceViewId,
                    allowEditing:
                        widget.session.isPresentationActive &&
                        widget.session.allowEditing &&
                        !widget.session.opened.fromSnapshot &&
                        (widget.policy?.allowEditing ?? true),
                  ),
                ),
              ),
            ),
          ],
        );
}
