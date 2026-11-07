library;

import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:openmuse_dataset/openmuse_dataset.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:uuid/uuid.dart';
import 'src/resource_checkpoint.dart';
export 'src/resource_checkpoint.dart';

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
        OpenMuseResourceContextContributor {
  OpenMuseDatasetPlugin({
    required this.core,
    required this.storeFactory,
    this.collaborationFactory,
    this.realtimeFactory,
  });
  final DatasetCore core;
  final DatasetStoreFactory storeFactory;
  final DatasetCollaborationFactory? collaborationFactory;
  final DatasetRealtimeFactory? realtimeFactory;
  final Map<String, DatasetRealtimeSession> _realtime = {};
  DatasetRealtimeSession? realtimeFor(ResourceIdentity id) => _realtime[id.key];
  final Map<String, DatasetController> _controllers = {};
  final Set<String> _opening = {};
  DatasetController? activeController(ResourceIdentity id) =>
      _controllers[id.key];
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
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {
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
    final key = session.record.identity.key;
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
      );
      try {
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
    await _realtime[session.record.identity.key]?.close();
    await c.close();
    _realtime.remove(session.record.identity.key);
    _controllers.remove(session.record.identity.key);
  }

  /// The Host resolves/authorizes the child first. This method receives the same
  /// bounded SDK session as a standalone editor; no parent document authority
  /// or blanket resource provider crosses the boundary.
  Future<DatasetEmbedSession> openEmbeddedSession(
    ResourceEditorSession session,
  ) async {
    final c = await openController(session);
    final checkpoint = DatasetResourceCheckpoint(c, session);
    return DatasetEmbedSession(
      realtime: realtimeFor(session.record.identity),
      checkpointStatus: checkpoint,
      checkpointFailure: () => checkpoint.failure,
      flushCheckpoint: checkpoint.flush,
      controller: c,
      title: session.record.name,
      close: () async {
        await checkpoint.flush();
        if (!await session.prepareToClose())
          throw const ResourceFailure('UNSAVED_CHANGES');
        await release(session, c);
        checkpoint.dispose();
      },
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
  const _DatasetResourceView({required this.plugin, required this.session});
  final OpenMuseDatasetPlugin plugin;
  final ResourceEditorSession session;
  @override
  State<_DatasetResourceView> createState() => _DatasetResourceViewState();
}

class _DatasetResourceViewState extends State<_DatasetResourceView> {
  DatasetController? c;
  DatasetResourceCheckpoint? checkpoint;
  Object? failure;
  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() async {
    try {
      final controller = await widget.plugin.openController(widget.session);
      if (!mounted) {
        await widget.plugin.release(widget.session, controller);
        return;
      }
      c = controller;
      checkpoint = DatasetResourceCheckpoint(controller, widget.session)
        ..addListener(_changed);
      setState(() {});
    } on Object catch (e) {
      if (mounted) setState(() => failure = e);
    }
  }

  void _changed() {
    if (mounted) setState(() {});
  }

  @override
  void dispose() {
    final adapter = checkpoint, controller = c;
    adapter?.removeListener(_changed);
    if (controller != null)
      unawaited(
        (() async {
          await adapter?.flush();
          await widget.plugin.release(widget.session, controller);
          adapter?.dispose();
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
                        await checkpoint!.flush();
                        if (mounted) setState(() {});
                      } on Object {}
                    },
                    child: const Text('重试'),
                  ),
                ],
              ),
            Expanded(
              child: DatasetSurface(
                controller: c!,
                title: widget.session.record.name,
                realtime: widget.plugin.realtimeFor(
                  widget.session.record.identity,
                ),
              ),
            ),
          ],
        );
}
