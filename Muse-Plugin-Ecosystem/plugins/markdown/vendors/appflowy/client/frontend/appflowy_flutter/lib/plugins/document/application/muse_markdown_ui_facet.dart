import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:appflowy_backend/dispatch/dispatch.dart';
import 'package:appflowy_backend/log.dart';
import 'package:appflowy_backend/protobuf/flowy-document/entities.pb.dart';
import 'package:appflowy_backend/rust_stream.dart';
import 'package:appflowy_editor/appflowy_editor.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

const _pluginId = 'muse.appflowy.markdown';
const _pluginVersion = '2.0.0';
const _facetRef = 'facet.flutter.markdown.1';
const _surfaceKind = 'markdown.document';
const _surfaceDigest =
    'sha256:780a1eed2737ec2f9e7dd1fe56cd74a3a7803aea0fc23e37837fd33d458c7b92';
const _selectionDigest =
    'sha256:39caf711d2a5cc6a452fb9e4b5aa2f0bf2f5c695a1a751c0d06de2e7255946c0';

/// Must match `MUSE_MARKDOWN_NOTIFICATION_SOURCE` / `_TY` in flowy-document.
const museMarkdownNotificationSource = 'MuseMarkdown';
const museMarkdownDomainChangeTy = 1;

final class MarkdownUiSurfaceBinding {
  MarkdownUiSurfaceBinding._({
    required this.documentId,
    required MuseUiSurfaceRuntime runtime,
    required MuseSurfaceLease lease,
  })  : _runtime = runtime,
        _lease = lease,
        _epochRef = 'epoch.flutter.${DateTime.now().microsecondsSinceEpoch}';

  final String documentId;
  final MuseUiSurfaceRuntime _runtime;
  final MuseSurfaceLease _lease;
  final String _epochRef;
  var _revision = 0;
  var _closed = false;
  StreamSubscription? _domainChangeSubscription;

  static MarkdownUiSurfaceBinding open(
    String documentId, {
    required Future<void> Function() onExternalReconcile,
  }) {
    final runtime = _MarkdownSurfaceRuntime.instance;
    final lease = runtime.openSurface(
      MuseOpenSurface(
        facetInstanceRef: _facetRef,
        surfaceKind: _surfaceKind,
        scopeRef: documentId,
        resourceRef: documentId,
        windowRef: 'window.primary',
      ),
      _MarkdownFacet(onExternalReconcile),
    );
    runtime
      ..setActive(lease, active: true)
      ..setFocused(lease, focused: true);
    final binding = MarkdownUiSurfaceBinding._(
      documentId: documentId,
      runtime: runtime,
      lease: lease,
    );
    binding._listenForDomainChanges();
    unawaited(binding.publishSurface());
    return binding;
  }

  Future<void> publishSurface() => _publish(
        type: 'markdown.surface',
        digest: _surfaceDigest,
        lane: MuseContextLane.control,
        ttl: const Duration(minutes: 5),
        payload: {
          'resourceRef': documentId,
          'viewRef': documentId,
          'title': documentId,
          'mode': 'edit',
          'readOnly': false,
        },
      );

  Future<void> publishSelection(EditorState editorState) async {
    if (_closed) return;
    final selection = editorState.selection;
    if (selection == null) return;
    final selectedText = String.fromCharCodes(
      editorState.getTextInSelection(selection).join('\n').runes.take(2048),
    );
    final refs = <String>{};
    final start = editorState.getNodeAtPath(selection.start.path)?.id;
    final end = editorState.getNodeAtPath(selection.end.path)?.id;
    if (start != null) refs.add(start);
    if (end != null) refs.add(end);
    await _publish(
      type: 'markdown.selection',
      digest: _selectionDigest,
      lane: MuseContextLane.control,
      ttl: const Duration(seconds: 15),
      payload: {
        'collapsed': selection.isCollapsed,
        'selectedText': selectedText,
        'selectedBlockRefs': refs.toList(),
      },
    );
  }

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    await _domainChangeSubscription?.cancel();
    _domainChangeSubscription = null;
    await _runtime.closeSurface(_lease);
  }

  void _listenForDomainChanges() {
    _domainChangeSubscription = RustStreamReceiver.listen((observable) {
      if (_closed) return;
      if (observable.source != museMarkdownNotificationSource) return;
      if (observable.id != documentId) return;
      if (observable.ty != museMarkdownDomainChangeTy) return;
      if (!observable.hasPayload()) return;
      try {
        final envelope = DocumentTextPB.fromBuffer(
          Uint8List.fromList(observable.payload),
        );
        final decoded = jsonDecode(envelope.text);
        validateMuseFacetValue(MuseFacetSchemaKind.domainChange, decoded);
        unawaited(
          _runtime.routeDomainChange(MuseDomainChangeV1.fromJson(decoded)),
        );
      } catch (error) {
        Log.warn('Ignored Muse Markdown domain change: $error');
      }
    });
  }

  Future<void> _publish({
    required String type,
    required String digest,
    required MuseContextLane lane,
    required Duration ttl,
    required Object payload,
  }) {
    final now = DateTime.now().millisecondsSinceEpoch;
    return _runtime.publishContext(
      _lease,
      MuseContextContributionV1(
        pluginId: _pluginId,
        pluginVersion: _pluginVersion,
        facetInstanceRef: _facetRef,
        surfaceInstanceRef: _lease.surfaceInstanceRef,
        surfaceKind: _surfaceKind,
        scopeRef: documentId,
        contextType: type,
        contextSchemaDigest: digest,
        contextRevision: (++_revision).toString(),
        epochRef: _epochRef,
        lane: lane,
        capturedAt: now,
        expiresAt: now + ttl.inMilliseconds,
        payload: payload,
      ),
    );
  }
}

final class _MarkdownSurfaceRuntime {
  static final MuseUiSurfaceRuntime instance = _make();

  static MuseUiSurfaceRuntime _make() {
    final runtime = MuseUiSurfaceRuntime(sink: _FlutterHostContextSink());
    runtime.registerFacet(
      const MuseFacetRegistration(
        pluginId: _pluginId,
        pluginVersion: _pluginVersion,
        facetInstanceRef: _facetRef,
        surfaceKinds: {_surfaceKind},
      ),
    );
    return runtime;
  }
}

final class _FlutterHostContextSink implements MuseSurfaceContextSink {
  @override
  Future<void> publish(MuseContextContributionV1 context) async {
    MuseSurfaceContextFeed.instance.publish(context);
    final result = await DocumentEventPublishMuseUiContext(
      DocumentTextPB(text: jsonEncode(context.toJson())),
    ).send();
    result.fold((_) => null, (error) => throw error);
  }

  @override
  Future<void> closeSurface(String surfaceInstanceRef, String scopeRef) async {
    MuseSurfaceContextFeed.instance.closeSurface(surfaceInstanceRef);
    final now = DateTime.now().millisecondsSinceEpoch;
    await publish(
      MuseContextContributionV1(
        pluginId: _pluginId,
        pluginVersion: _pluginVersion,
        facetInstanceRef: _facetRef,
        surfaceInstanceRef: surfaceInstanceRef,
        surfaceKind: _surfaceKind,
        scopeRef: scopeRef,
        contextType: 'muse.surface.closed',
        contextSchemaDigest: _surfaceDigest,
        contextRevision: '1',
        epochRef: 'epoch.close.$now',
        lane: MuseContextLane.control,
        capturedAt: now,
        expiresAt: now + 1000,
        payload: const {'closed': true},
      ),
    );
  }
}

final class _MarkdownFacet implements MuseUiFacet {
  _MarkdownFacet(this._reconcile);

  final Future<void> Function() _reconcile;
  String? _projectedRevision;

  @override
  Future<void> dispose() async {}

  @override
  Future<void> onDomainChange(MuseDomainChangeV1 change) async {
    if (change.origin == MuseMutationOrigin.uiOptimistic) return;
    if (_projectedRevision == change.domainRevision) return;
    await _reconcile();
    _projectedRevision = change.domainRevision;
  }
}
