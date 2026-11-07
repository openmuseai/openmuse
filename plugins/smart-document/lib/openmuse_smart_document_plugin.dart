library;

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:path_provider/path_provider.dart';
import 'package:uuid/uuid.dart';

const smartDocumentMediaType = 'application/vnd.openmuse.smart-document+json';
typedef SmartDocumentProviderFactory =
    CollaborationProvider? Function(ResourceRecord record);
typedef SmartDocumentAssetFactory =
    Future<AssetRepository> Function(ResourceEditorSession session);
typedef SmartDocumentStoreFactory =
    Future<DocumentStore> Function(ResourceEditorSession session);

typedef SmartDocumentRealtimeFactory =
    Future<DocumentRealtimeSession?> Function(
      ResourceEditorSession session,
      SmartDocumentController controller,
    );

/// All content knowledge stays in the plugin. Host needs no format branch,
/// account duplication or private Cloud token in a portable resource file.
final class OpenMuseSmartDocumentPlugin
    implements
        OpenMusePlugin,
        OpenMuseResourceHandlerContributor,
        OpenMuseResourceCreatorContributor,
        OpenMuseResourceContextContributor {
  OpenMuseSmartDocumentPlugin({
    DocumentCore? core,
    this.providerFactory,
    this.realtimeFactory,
    this.assetFactory,
    this.draftStoreFactory,
    this.linkOpener,
    this.imagePicker,
    this.embedHostFactory,
    SmartDocumentStoreFactory? storeFactory,
  }) : _coreOverride = core,
       _storeFactory = storeFactory ?? _defaultStore;
  final DocumentCore? _coreOverride;
  DocumentCore? _core;
  final SmartDocumentProviderFactory? providerFactory;
  final SmartDocumentRealtimeFactory? realtimeFactory;
  final Map<String, DocumentRealtimeSession> _realtime = {};
  DocumentRealtimeSession? realtimeFor(ResourceIdentity identity) =>
      _realtime[identity.key];
  final SmartDocumentAssetFactory? assetFactory;
  final SmartDocumentStoreFactory? draftStoreFactory;
  final DocumentLinkOpener? linkOpener;
  final DocumentImagePicker? imagePicker;
  final ResourceEmbedHost? Function(ResourceEditorSession)? embedHostFactory;
  final Map<String, DocumentMediaSession> _media = {};
  DocumentMediaSession? mediaFor(ResourceIdentity identity) =>
      _media[identity.key];
  final SmartDocumentStoreFactory _storeFactory;
  final Set<String> _open = {};
  final Map<String, SmartDocumentController> _controllers = {};
  SmartDocumentController? activeController(ResourceIdentity identity) =>
      _controllers[identity.key];
  DocumentCore get core => _core ??= _coreOverride ?? NativeDocumentCore();
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'io.openmuse.smart-document',
    name: '智能文档',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'resource.read', 'resource.create', 'resource.edit'},
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {
    for (final session in _realtime.values.toList()) {
      await session.close();
    }
    _realtime.clear();
    for (final c in _controllers.values.toList()) {
      await c.close();
    }
    for (final m in _media.values) {
      m.dispose();
    }
    _media.clear();
    _controllers.clear();
    _open.clear();
  }

  @override
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'smart-document.document',
      title: '智能文档',
      mediaTypes: const {smartDocumentMediaType},
      extensions: const {'musedoc'},
      editable: true,
      priority: 40,
    ),
  ];
  @override
  List<ResourceCreatorSpec> get resourceCreators => [
    ResourceCreatorSpec(
      id: 'smart-document.document',
      title: '智能文档',
      mediaType: smartDocumentMediaType,
      extension: 'musedoc',
      defaultName: '未命名.musedoc',
    ),
  ];
  @override
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  ) async {
    if (creatorId != 'smart-document.document')
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    final state = core.call({'action': 'open', 'objectId': const Uuid().v4()});
    try {
      return ResourceSeed(
        bytes: _export(state),
        mediaType: smartDocumentMediaType,
      );
    } finally {
      core.call({'action': 'close', 'handle': state['handle']});
    }
  }

  static List<int> _export(Map<String, dynamic> state) => utf8.encode(
    jsonEncode({
      'profile': 'muse.smart-document/phase1',
      'objectId': state['objectId'],
      'docState': state['docState'],
    }),
  );
  static Map<String, dynamic> decode(List<int> bytes) {
    if (bytes.length > 8 * 1024 * 1024)
      throw const ResourceFailure('CONTENT_TOO_LARGE');
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map ||
        value['profile'] != 'muse.smart-document/phase1' ||
        value['objectId'] is! String ||
        value['docState'] is! String ||
        value.keys.any(
          (k) => !{'profile', 'objectId', 'docState'}.contains(k),
        )) {
      throw const ResourceFailure('INVALID_SMART_DOCUMENT');
    }
    return value.cast<String, dynamic>();
  }

  static Future<DocumentStore> _defaultStore(
    ResourceEditorSession session,
  ) async {
    final base = await getApplicationSupportDirectory();
    final identity =
        '${session.record.identity.key}:${session.opened.fromSnapshot ? session.record.version.digest : 'live'}';
    final key = sha256.convert(utf8.encode(identity));
    return FileDocumentStore(
      File('${base.path}/openmuse/smart-document/$key.json'),
    );
  }

  Future<SmartDocumentController> openController(
    ResourceEditorSession session,
  ) async {
    final key = session.record.identity.key;
    if (!_open.add(key)) throw const ResourceFailure('RESOURCE_ALREADY_OPEN');
    try {
      final seed = decode(session.bytes);
      final provider = session.opened.fromSnapshot
          ? null
          : providerFactory?.call(session.record);
      final c = await SmartDocumentController.open(
        core: core,
        store: await _storeFactory(session),
        objectId: seed['objectId'] as String,
        initialDocState: seed['docState'] as String,
        provider: provider,
        editable: session.allowEditing && !session.opened.fromSnapshot,
        // First-phase resources are created through an explicit resource creator.
        // Cloud creation is explicit here and still authorized by Cloud membership.
        createRemote:
            provider != null &&
            session.allowEditing &&
            !session.opened.fromSnapshot,
      );
      try {
        final base = await getApplicationSupportDirectory();
        final cacheKey = sha256
            .convert(utf8.encode(session.record.identity.key))
            .toString();
        final cache = DocumentStoreAssetCache(
          (key) => FileDocumentStore(
            File(
              '${base.path}/openmuse/smart-document-assets/$cacheKey/$key.json',
            ),
          ),
        );
        final repository = assetFactory == null
            ? LocalAssetRepository(scope: cacheKey, cache: cache)
            : await assetFactory!(session);
        final media = DocumentMediaSession(
          picker: imagePicker,
          controller: c,
          repository: repository,
          draftStore:
              await (draftStoreFactory?.call(session) ??
                  Future.value(
                    FileDocumentStore(
                      File(
                        '${base.path}/openmuse/smart-document-assets/$cacheKey/draft.json',
                      ),
                    ),
                  )),
        );
        try {
          await media.restore();
        } on Object {
          media.dispose();
          rethrow;
        }
        _media[key] = media;
      } on Object {
        await c.close();
        rethrow;
      }
      try {
        if (!session.opened.fromSnapshot && realtimeFactory != null) {
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
        _media.remove(key)?.dispose();
        await c.close();
        rethrow;
      }
      _controllers[key] = c;
      return c;
    } on Object {
      _open.remove(key);
      rethrow;
    }
  }

  Future<void> release(
    ResourceEditorSession session,
    SmartDocumentController c,
  ) async {
    await _realtime[session.record.identity.key]?.close();
    await c.close();
    _realtime.remove(session.record.identity.key);
    _media.remove(session.record.identity.key)?.dispose();
    _controllers.remove(session.record.identity.key);
    _open.remove(session.record.identity.key);
  }

  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async =>
      record.mediaType == smartDocumentMediaType ||
      record.name.toLowerCase().endsWith('.musedoc');
  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  ) => _SmartDocumentResourceView(plugin: this, session: session);
  @override
  Future<String?> resourceConversationText(ResourceContent content) async {
    if (content.record.mediaType != smartDocumentMediaType) return null;
    final seed = decode(content.bytes);
    final state = core.call({
      'action': 'open',
      'objectId': seed['objectId'],
      'docState': seed['docState'],
    });
    try {
      final data = state['data'] as Map;
      final parts = <String>[];
      void collect(String id) {
        final block = data['blocks'][id] as Map;
        final external = block['external_id'];
        if (external != null) {
          final runs =
              jsonDecode(data['meta']['text_map'][external] as String? ?? '[]')
                  as List;
          parts.add(
            runs
                .whereType<Map>()
                .map((run) => run['insert'] as String? ?? '')
                .join(),
          );
        }
        for (final child
            in data['meta']['children_map'][block['children']] as List? ?? []) {
          collect(child as String);
        }
      }

      collect(data['page_id'] as String);
      final text = parts.join('\n');
      if (text.length > 128 * 1024)
        throw const ResourceFailure('CONTEXT_TOO_LARGE');
      return text;
    } finally {
      core.call({'action': 'close', 'handle': state['handle']});
    }
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Requires a ResourceEditorSession');
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _SmartDocumentResourceView extends StatefulWidget {
  const _SmartDocumentResourceView({
    required this.plugin,
    required this.session,
  });
  final OpenMuseSmartDocumentPlugin plugin;
  final ResourceEditorSession session;
  @override
  State<_SmartDocumentResourceView> createState() =>
      _SmartDocumentResourceViewState();
}

final class _SmartDocumentResourceViewState
    extends State<_SmartDocumentResourceView> {
  SmartDocumentController? controller;
  Object? failure;
  Timer? timer;
  Future<void>? saving;
  String? savedState;
  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() async {
    try {
      final c = await widget.plugin.openController(widget.session);
      if (!mounted) {
        await widget.plugin.release(widget.session, c);
        return;
      }
      controller = c;
      savedState =
          OpenMuseSmartDocumentPlugin.decode(widget.session.bytes)['docState']
              as String;
      if (widget.session.allowEditing && !widget.session.opened.fromSnapshot)
        widget.session.enterEditing();
      widget.session.flushPendingChanges = _flush;
      c.addListener(_changed);
      widget.session.reportOpened();
      setState(() {});
      _changed();
    } on Object catch (error) {
      if (mounted) setState(() => failure = error);
    }
  }

  void _changed() {
    final c = controller;
    if (c == null || !widget.session.editing) return;
    if (c.contentFailure != null) {
      widget.session.markDirty(true);
      return;
    }
    if (c.snapshot['docState'] == savedState) return;
    widget.session.markDirty(true);
    timer?.cancel();
    timer = Timer(const Duration(milliseconds: 250), () async {
      try {
        await _flush();
      } on Object catch (e) {
        if (mounted) setState(() => failure = e);
      }
    });
  }

  Future<void> _flush() => saving ??= (() async {
    final c = controller;
    if (c == null || !widget.session.editing) return;
    await c.flush();
    final state = c.snapshot;
    if (state['docState'] == savedState) {
      widget.session.markDirty(false);
      return;
    }
    await widget.session.commit(OpenMuseSmartDocumentPlugin._export(state));
    savedState = state['docState'] as String;
    if (c.snapshot['docState'] != savedState) {
      widget.session.markDirty(true);
      _changed();
    }
  })().whenComplete(() => saving = null);
  @override
  void dispose() {
    timer?.cancel();
    widget.session.flushPendingChanges = null;
    final c = controller;
    if (c != null) {
      c.removeListener(_changed);
      unawaited(widget.plugin.release(widget.session, c));
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (controller == null)
      return Center(
        child: failure == null
            ? const CircularProgressIndicator()
            : const Text('智能文档无法打开，请检查内容与编辑能力。'),
      );
    return Column(
      children: [
        if (failure != null)
          MaterialBanner(
            content: const Text('资源检查点保存失败；本机草稿保留，请重试。'),
            actions: [
              TextButton(
                onPressed: () async {
                  try {
                    await _flush();
                    if (mounted) setState(() => failure = null);
                  } on Object {}
                },
                child: const Text('重试'),
              ),
            ],
          ),
        Expanded(
          child: SmartDocumentView(
            controller: controller!,
            title: widget.session.record.name,
            media: widget.plugin.mediaFor(widget.session.record.identity),
            realtime: widget.plugin.realtimeFor(widget.session.record.identity),
            linkOpener: widget.plugin.linkOpener,
            embeds: widget.session.opened.fromSnapshot
                ? null
                : widget.plugin.embedHostFactory?.call(widget.session),
          ),
        ),
      ],
    );
  }
}
