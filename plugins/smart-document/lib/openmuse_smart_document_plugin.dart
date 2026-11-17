library;

import 'dart:async';
import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'src/platform_storage.dart';
export 'src/platform_storage.dart' show createSmartDocumentStore;
import 'package:uuid/uuid.dart';
import 'src/resource_checkpoint.dart';
export 'src/resource_checkpoint.dart';

typedef SmartDocumentResourceLease =
    ResourceSessionLease<
      SmartDocumentController,
      SmartDocumentResourceCheckpoint,
      DocumentRealtimeSession?
    >;

const smartDocumentMediaType = 'application/vnd.openmuse.smart-document+json';
typedef SmartDocumentProviderFactory =
    CollaborationProvider? Function(ResourceRecord record);
typedef SmartDocumentAssetFactory =
    Future<AssetRepository?> Function(ResourceEditorSession session);
typedef SmartDocumentOwnerAssetFactory =
    Future<AssetRepository?> Function(
      ResourceEditorSession session,
      SmartDocumentController owner,
    );
typedef SmartDocumentStoreFactory =
    Future<DocumentStore> Function(ResourceEditorSession session);

/// Observes the existing content owner without acquiring another resource,
/// controller, checkpoint or transport. Cleanup must detach its listeners.
typedef SmartDocumentContentObserverFactory =
    VoidCallback? Function(
      ResourceEditorSession session,
      SmartDocumentController controller,
    );

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
        OpenMuseResourceEmbedContributor,
        OpenMuseResourceEmbedValidator,
        OpenMuseResourceContextContributor {
  OpenMuseSmartDocumentPlugin({
    DocumentCore? core,
    this.providerFactory,
    this.realtimeFactory,
    this.assetFactory,
    this.ownerAssetFactory,
    this.draftStoreFactory,
    this.replacementDraftStoreFactory,
    this.linkOpener,
    this.imagePicker,
    this.networkImageReader,
    this.embedHostFactory,
    this.headerActions,
    this.contentObserverFactory,
    SmartDocumentStoreFactory? storeFactory,
  }) : _coreOverride = core,
       _storeFactory = storeFactory ?? _defaultStore;
  final DocumentCore? _coreOverride;
  final SmartDocumentContentObserverFactory? contentObserverFactory;
  final _contentObserverStops = <ResourceSessionScope, VoidCallback>{};
  DocumentCore? _core;
  final SmartDocumentProviderFactory? providerFactory;
  final SmartDocumentRealtimeFactory? realtimeFactory;
  final Map<ResourceSessionScope, DocumentRealtimeSession> _realtime = {};
  DocumentRealtimeSession? realtimeFor(
    ResourceIdentity identity, {
    ResourceVersion? snapshotVersion,
  }) =>
      _realtime[ResourceSessionScope(
        identity,
        snapshotVersion: snapshotVersion,
      )];
  final SmartDocumentAssetFactory? assetFactory;
  final SmartDocumentOwnerAssetFactory? ownerAssetFactory;
  final SmartDocumentStoreFactory? draftStoreFactory;
  final SmartDocumentStoreFactory? replacementDraftStoreFactory;
  final DocumentLinkOpener? linkOpener;
  final DocumentImagePicker? imagePicker;
  final DocumentNetworkImageReader? networkImageReader;
  final ResourceEmbedHost? Function(ResourceEditorSession)? embedHostFactory;
  final List<Widget> Function(BuildContext, ResourceEditorSession)?
  headerActions;
  final Map<
    ResourceSessionScope,
    ({ResourceSessionLifetime? lifetime, VoidCallback callback})
  >
  _authorityListeners = {};
  final Map<ResourceSessionScope, DocumentMediaSession> _media = {};
  DocumentMediaSession? mediaFor(
    ResourceIdentity identity, {
    ResourceVersion? snapshotVersion,
  }) =>
      _media[ResourceSessionScope(identity, snapshotVersion: snapshotVersion)];
  final SmartDocumentStoreFactory _storeFactory;
  late final _owners =
      ResourceSessionOwners<
        SmartDocumentController,
        SmartDocumentResourceCheckpoint,
        DocumentRealtimeSession?
      >(
        open: openController,
        release: _releaseController,
        objectId: (bytes) => decode(bytes)['objectId'] as String,
        realtime: (scope) => _realtime[scope],
        createCheckpoint: (c, session) => SmartDocumentResourceCheckpoint(
          c,
          session,
          savedState: decode(session.bytes)['docState'] as String,
          encode: _export,
        ),
        controllerObjectId: (c) => c.objectId,
        controllerEditable: (c) => c.editable,
        supportsPrivateRecovery: true,
        prepareAuthority: (c, next) async {
          final seed = decode(next.bytes);
          if (seed['objectId'] != c.objectId)
            throw const ResourceFailure('RESOURCE_CONTENT_CHANGED');
          final ticket = await c.prepareWriteAuthority(
            seed['docState'] as String,
          );
          return ResourceOwnerAuthorityPreparation(
            active: () => ticket.isActive,
            commit: ticket.commit,
            cancel: ticket.cancel,
            complete: ticket.complete,
          );
        },
        prepareClose: (c) async {
          final ticket = await c.prepareClose();
          DocumentMediaClosePreparation? media;
          try {
            final scope = _controllers.entries
                .singleWhere((e) => identical(e.value, c))
                .key;
            media = await _media[scope]?.prepareClose();
            return ResourceOwnerPreparation(
              active: () => ticket.isActive && (media?.isActive ?? true),
              cancel: () {
                media?.cancel();
                ticket.cancel();
              },
            );
          } on Object {
            media?.cancel();
            ticket.cancel();
            rethrow;
          }
        },
        authorityEnded: (c, realtime) {
          c.invalidateAuthority();
          if (realtime != null)
            unawaited(realtime.close().catchError((Object _) {}));
        },
      );
  Future<SmartDocumentResourceLease> acquireResourceSession(
    ResourceEditorSession session,
  ) => _owners.acquire(session);
  bool hasActiveResourceOwner(ResourceSessionScope scope) =>
      _owners.hasActiveOwner(scope);
  final _primaryEditors = <ResourceSessionScope, Object>{};
  bool _claimPrimary(
    ResourceSessionScope scope,
    SmartDocumentController c,
    Object user,
  ) {
    if (c.isClosed || !c.canRead || _primaryEditors.containsKey(scope))
      return false;
    _primaryEditors[scope] = user;
    c.editor.selection = null;
    return true;
  }

  void _releasePrimary(ResourceSessionScope scope, Object user) {
    if (identical(_primaryEditors[scope], user)) _primaryEditors.remove(scope);
  }

  final Set<ResourceSessionScope> _open = {};
  final Map<ResourceSessionScope, SmartDocumentController> _controllers = {};
  SmartDocumentController? activeController(
    ResourceIdentity identity, {
    ResourceVersion? snapshotVersion,
  }) =>
      _controllers[ResourceSessionScope(
        identity,
        snapshotVersion: snapshotVersion,
      )];
  DocumentCore get core =>
      _core ??= _coreOverride ?? createPlatformDocumentCore();
  Future<DocumentCore> _loadCore() async =>
      _core ??= _coreOverride ?? await loadDocumentCore();
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
  Future<void> activate(OpenMusePluginContext context) async {
    _owners.activate();
  }

  @override
  Future<void> deactivate() async {
    await _owners.close();
    for (final stop in _contentObserverStops.values.toList()) {
      stop();
    }
    _contentObserverStops.clear();
    for (final listener in _authorityListeners.values) {
      listener.lifetime?.removeListener(listener.callback);
    }
    _authorityListeners.clear();
    for (final session in _realtime.values.toList()) {
      await session.close();
    }
    _realtime.clear();
    for (final entry in _controllers.entries.toList()) {
      DocumentMediaClosePreparation? prepared;
      try {
        final media = _media[entry.key];
        if (media != null && !media.isPreparingClose)
          prepared = await media.prepareClose();
        await entry.value.close();
      } on Object {
        prepared?.cancel();
        rethrow;
      }
    }
    for (final m in _media.values) {
      m.dispose();
    }
    _media.clear();
    _controllers.clear();
    _primaryEditors.clear();
    _open.clear();
  }

  @override
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'smart-document.document',
      supportsSharedPresentations: true,
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
  Set<String> get resourceEmbedSurfaces => const {'smart-document.document'};
  @override
  Future<void> validateResourceEmbedTarget(
    OpenedResource opened,
    ResourceEmbedTarget target,
  ) async {
    if (target.surface != 'smart-document.document' ||
        target.selector.length != 1 ||
        target.selector['objectId'] !=
            decode(opened.content.bytes)['objectId']) {
      throw const ResourceFailure('EMBED_RESOURCE_MISMATCH');
    }
  }

  @override
  Future<List<({String title, ResourceEmbedTarget target})>>
  resourceEmbedTargets(OpenedResource opened, String surface) async {
    if (!resourceEmbedSurfaces.contains(surface)) return const [];
    final seed = decode(opened.content.bytes);
    return [
      (
        title: opened.content.record.name,
        target: ResourceEmbedTarget(
          reference: opened.content.record.reference(
            // Hub records the observed version even for a live resolve. Only
            // an explicit snapshot opening selects a historical child surface.
            pin: opened.fromSnapshot,
          ),
          surface: surface,
          selector: {'objectId': seed['objectId'] as String},
        ),
      ),
    ];
  }

  @override
  Widget buildResourceEmbed(
    BuildContext context,
    ResourceEditorSession session,
    ResourceEmbedTarget target,
    ResourceEmbedPolicy policy,
  ) => _SmartDocumentResourceView(
    plugin: this,
    session: session,
    target: target,
    policy: policy,
  );
  @override
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  ) async {
    if (creatorId != 'smart-document.document')
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    final core = await _loadCore();
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

  static List<int> encodeSnapshot(Map<String, dynamic> state) => utf8.encode(
    jsonEncode({
      'profile': 'muse.smart-document/phase1',
      'objectId': state['objectId'],
      'docState': state['docState'],
    }),
  );
  static List<int> _export(Map<String, dynamic> state) => encodeSnapshot(state);
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
    final identity =
        '${session.record.identity.key}:${session.opened.fromSnapshot ? session.record.version.digest : 'live'}';
    final key = sha256.convert(utf8.encode(identity));
    return createSmartDocumentStore('smart-document', '$key');
  }

  Future<SmartDocumentController> openController(
    ResourceEditorSession session,
  ) async {
    final key = session.ownershipScope;
    if (!_open.add(key)) throw const ResourceFailure('RESOURCE_ALREADY_OPEN');
    try {
      final seed = decode(session.bytes);
      final provider = session.opened.fromSnapshot
          ? null
          : providerFactory?.call(session.record);
      final c = await SmartDocumentController.open(
        core: await _loadCore(),
        store: session.opened.fromSnapshot
            ? _SnapshotMemory()
            : await _storeFactory(session),
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
        final cacheKey = sha256
            .convert(utf8.encode(session.record.identity.key))
            .toString();
        final cache = DocumentStoreAssetCache(
          (key) =>
              createSmartDocumentStore('smart-document-assets/$cacheKey', key),
        );
        final repository =
            (await ownerAssetFactory?.call(session, c)) ??
            (await assetFactory?.call(session)) ??
            LocalAssetRepository(scope: cacheKey, cache: cache);
        final media = DocumentMediaSession(
          picker: imagePicker,
          networkReader: networkImageReader,
          controller: c,
          repository: repository,
          replacementDraftStore: await (session.opened.fromSnapshot
              ? Future<DocumentStore>.value(_SnapshotMemory())
              : replacementDraftStoreFactory?.call(session) ??
                    Future.value(
                      createSmartDocumentStore(
                        'smart-document-assets/$cacheKey',
                        'replacement-draft.v1',
                      ),
                    )),
          draftStore: await (session.opened.fromSnapshot
              ? Future<DocumentStore>.value(_SnapshotMemory())
              : draftStoreFactory?.call(session) ??
                    Future.value(
                      createSmartDocumentStore(
                        'smart-document-assets/$cacheKey',
                        'draft',
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
        final stop = contentObserverFactory?.call(session, c);
        if (stop != null) _contentObserverStops[key] = stop;
      } on Object {
        await _realtime.remove(key)?.close();
        _media.remove(key)?.dispose();
        await c.close();
        rethrow;
      }
      _controllers[key] = c;
      void checkAuthority() {
        if (session.isActive) return;
        c.invalidateAuthority();
        final realtime = _realtime[key];
        if (realtime != null) unawaited(realtime.close());
      }

      session.lifetime?.addListener(checkAuthority);
      _authorityListeners[key] = (
        lifetime: session.lifetime,
        callback: checkAuthority,
      );
      checkAuthority();
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
    if (_owners.owns(session.ownershipScope))
      throw const ResourceFailure('RESOURCE_SHARED_OWNER');
    await _releaseController(session, c);
  }

  Future<void> _releaseController(
    ResourceEditorSession session,
    SmartDocumentController c, {
    Future<void> Function()? beforeRelease,
  }) async {
    final key = session.ownershipScope;
    if (!identical(_controllers[key], c))
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    DocumentMediaClosePreparation? mediaPreparation;
    try {
      await c.close(
        beforeRelease: () async {
          final media = _media[key];
          if (media != null && !media.isPreparingClose) {
            mediaPreparation = await media.prepareClose();
          }
          await beforeRelease?.call();
          await _realtime[key]?.close();
        },
      );
    } on Object {
      mediaPreparation?.cancel();
      rethrow;
    }
    final listener = _authorityListeners.remove(key);
    _contentObserverStops.remove(key)?.call();
    if (listener != null) listener.lifetime?.removeListener(listener.callback);
    _realtime.remove(key);
    _media.remove(key)?.dispose();
    _controllers.remove(key);
    _open.remove(key);
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
    final core = await _loadCore();
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
    this.target,
    this.policy,
  });
  final OpenMuseSmartDocumentPlugin plugin;
  final ResourceEditorSession session;
  final ResourceEmbedTarget? target;
  final ResourceEmbedPolicy? policy;
  @override
  State<_SmartDocumentResourceView> createState() =>
      _SmartDocumentResourceViewState();
}

final class _SmartDocumentResourceViewState
    extends State<_SmartDocumentResourceView> {
  SmartDocumentController? controller;
  Object? failure;
  SmartDocumentResourceLease? lease;
  SmartDocumentResourceCheckpoint? checkpoint;
  SmartDocumentPresentation? presentation;
  bool usesPrimary = false;
  DocumentSelectionBookmark? _selectionBookmark;
  Listenable? _selectionChanges;
  Future<void>? _restoringPlacement;
  late final Future<void> Function() _placementCallback = _restorePlacement;

  bool _focusWithinView() {
    var inside = false;
    final focusContext = FocusManager.instance.primaryFocus?.context;
    if (identical(focusContext, context)) return true;
    focusContext?.visitAncestorElements((element) {
      if (identical(element, context)) {
        inside = true;
        return false;
      }
      return true;
    });
    return inside;
  }

  void _rememberSelection() {
    final owner = controller;
    if (!mounted || owner == null || !owner.canRead) {
      _selectionBookmark = null;
      return;
    }
    final editor = presentation?.editor ?? owner.editor;
    if (editor.selection != null) {
      if (owner.editorProjectionCurrent(editor)) {
        _selectionBookmark = owner.captureSelectionBookmark(
          presentation: presentation,
        );
      }
    } else {
      // Escape or a deliberate deselection while still inside this editor
      // clears the bookmark. Blur into Host chrome retains the last position.
      if (_focusWithinView()) _selectionBookmark = null;
    }
  }

  Future<void> _restorePlacement() => _restoringPlacement ??=
      _restoreLocalPlacement().whenComplete(() => _restoringPlacement = null);

  Future<void> _restoreLocalPlacement() async {
    final owner = controller;
    if (!mounted ||
        owner == null ||
        _selectionBookmark == null ||
        !widget.session.isPresentationActive)
      return;
    await owner.projectionReady;
    if (!mounted ||
        !identical(controller, owner) ||
        !widget.session.isPresentationActive ||
        !owner.canRead)
      return;
    // A new selection or explicit deselection can arrive while projection is
    // pending. Never restore the bookmark captured before that newer intent.
    final bookmark = _selectionBookmark;
    if (bookmark == null) return;
    final editor = presentation?.editor ?? owner.editor;
    if (editor.selection == null)
      owner.restoreSelectionBookmark(bookmark, presentation: presentation);
  }

  @override
  void initState() {
    super.initState();
    widget.session.addListener(_displayChanged);
    unawaited(_start());
  }

  void _displayChanged() {
    if (mounted) setState(() {});
  }

  @override
  void didUpdateWidget(_SmartDocumentResourceView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!identical(oldWidget.session, widget.session)) {
      oldWidget.session.removeListener(_displayChanged);
      widget.session.addListener(_displayChanged);
    }
    presentation?.setEditingPolicy(widget.policy?.allowEditing ?? true);
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
      final acquired = await widget.plugin.acquireResourceSession(
        widget.session,
      );
      if (target != null &&
          (target.surface != 'smart-document.document' ||
              target.selector.length != 1 ||
              target.selector['objectId'] != acquired.controller.objectId)) {
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
      controller = acquired.controller;
      usesPrimary =
          widget.target == null &&
          acquired.allowEditing &&
          controller!.editable &&
          widget.plugin._claimPrimary(
            widget.session.ownershipScope,
            controller!,
            this,
          );
      if (!usesPrimary) {
        presentation = controller!.openPresentation(
          allowEditing: acquired.allowEditing,
        );
        presentation!.setEditingPolicy(widget.policy?.allowEditing ?? true);
      }
      _selectionChanges =
          (presentation?.editor ?? controller!.editor).selectionNotifier;
      _selectionChanges!.addListener(_rememberSelection);
      widget.session.onPlacementRestored = _placementCallback;
      _rememberSelection();
      checkpoint = acquired.checkpoint..addListener(_changed);
      setState(() {});
    } on Object catch (error, stack) {
      assert(() {
        debugPrint('SmartDocument open failed: ${error.runtimeType}'
          '${error is SmartDocumentFailure ? ' ${error.code}' : ''}'
          '${error is RangeError ? ' $error' : ''}');
        debugPrintStack(stackTrace: stack, maxFrames: 12);
        return true;
      }());
      if (usesPrimary)
        widget.plugin._releasePrimary(widget.session.ownershipScope, this);
      usesPrimary = false;
      controller = null;
      if (mounted) setState(() => failure = error);
    }
  }

  void _changed() {
    if (controller?.canRead != true) _selectionBookmark = null;
    if (mounted) setState(() => failure = checkpoint?.failure);
  }

  Future<void> _flush() => lease?.flush() ?? Future.value();

  Future<bool> _openLink(Uri target) async {
    final custom = widget.plugin.linkOpener;
    if (custom != null) return custom(target);
    if (target.scheme == 'openmuse-resource') {
      // Use the shared SDK parser and current Host router. No provider or
      // account credential is passed through the document's href mark.
      final reference = ResourceReference.fromUri(target.toString());
      final routing = ResourceLinkRouting.maybeOf(context);
      if (routing == null) return false;
      await routing.open(reference.toUri().toString());
      return true;
    }
    return openDocumentLink(controller!, target.toString());
  }

  @override
  void dispose() {
    widget.session.removeListener(_displayChanged);
    _selectionChanges?.removeListener(_rememberSelection);
    _selectionBookmark = null;
    if (identical(widget.session.onPlacementRestored, _placementCallback))
      widget.session.onPlacementRestored = null;
    checkpoint?.removeListener(_changed);
    final acquired = lease;
    if (usesPrimary)
      widget.plugin._releasePrimary(widget.session.ownershipScope, this);
    final projection = presentation;
    if (acquired != null)
      unawaited(() async {
        await projection?.close();
        await acquired.close();
      }());
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
            presentation: presentation,
            title: widget.session.record.name,
            headerActions:
                widget.plugin.headerActions?.call(context, widget.session) ??
                const [],
            media: widget.plugin._media[widget.session.ownershipScope],
            realtime: widget.plugin._realtime[widget.session.ownershipScope],
            linkOpener: _openLink,
            embeds: widget.session.opened.fromSnapshot
                ? null
                : widget.plugin.embedHostFactory?.call(widget.session) ??
                      ResourceEmbedScope.of(context),
          ),
        ),
      ],
    );
  }
}

/// Historical previews never consult or overwrite a mutable recovery checkpoint.
final class _SnapshotMemory implements DocumentStore {
  Map<String, dynamic>? _value;
  @override
  Future<Map<String, dynamic>?> read() async => _value;
  @override
  Future<void> write(Map<String, dynamic> value) async => _value = value;
}
