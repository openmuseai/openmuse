import 'dart:convert';

import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_dialog.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/startup/tasks/app_widget.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;

/// Protocol tag of one Host→Flutter presentation dispatch. Mirrors
/// `MUSE_PRESENTATION_DISPATCH_PROTOCOL` in
/// `flowy-core/src/muse_presentation.rs`.
const String musePresentationDispatchProtocol = 'muse.presentation/dispatch/v1';

/// Adapter the Host labels a dispatch with when it could not resolve the ref
/// itself: the Flutter surface owns the Project Workspace Mount catalog.
const String museHostResolvedAdapterRef = 'muse.adapter.host-resolved';

/// Reason codes of the `muse.resource-presentation` family, mirrored from the
/// Rust provider so a refused dispatch names the same condition on both sides.
abstract final class MusePresentationReason {
  /// The ref resolved but this Host build cannot present it.
  static const String surfaceUnavailable = 'SURFACE_UNAVAILABLE';

  /// The opaque ref names nothing this Host can present.
  static const String resourceNotFound = 'RESOURCE_NOT_FOUND';

  /// The Host has no binding for the Mount the ref lives in.
  static const String mountNotBound = 'MOUNT_NOT_BOUND';

  /// The ref (or the Mount-relative locator it carried) is not admissible.
  static const String resourceRefInvalid = 'RESOURCE_REF_INVALID';
}

/// Shared opaque-reference grammar of the presentation contract:
/// `^[A-Za-z0-9._~-]{1,128}$`.
final RegExp _opaqueRefPattern = RegExp(r'^[A-Za-z0-9._~-]{1,128}$');

/// True when `value` may cross the bridge as an opaque ref.
bool isOpaqueMuseRef(String value) => _opaqueRefPattern.hasMatch(value);

/// Longest Mount scope reference the Host accepts, matching Rust.
const int maxMuseMountRefChars = 256;

/// Longest Mount-relative path the Host accepts, matching Rust.
const int maxMuseRelativePathChars = 4096;

/// Whether `relativePath` is a Mount-relative POSIX path. Mirrors
/// `relative_path_is_mount_relative` in `flowy-core/src/muse_presentation.rs`:
/// no device path, no absolute path, no traversal and no empty segment. The
/// empty path is the Mount root.
bool isMountRelativePath(String relativePath) {
  if (relativePath.isEmpty) return true;
  if (relativePath.length > maxMuseRelativePathChars) return false;
  if (relativePath.startsWith('/')) return false;
  if (relativePath.contains(r'\')) return false;
  if (relativePath.codeUnits.any((unit) => unit < 0x20 || (unit >= 0x7F && unit <= 0x9F))) {
    return false;
  }
  if (relativePath.length >= 2 &&
      _driveLetterPattern.hasMatch(relativePath.substring(0, 2))) {
    return false;
  }
  return relativePath
      .split('/')
      .every((segment) => segment.isNotEmpty && segment != '.' && segment != '..');
}

final RegExp _driveLetterPattern = RegExp(r'^[A-Za-z]:$');

/// One Host-authorized presentation handed to the Flutter surface.
///
/// Every field is Host-derived and the payload carries only opaque refs: the
/// Mount identity (`mountRef`) plus a Mount-relative POSIX path. It never
/// carries a device path.
final class MusePresentationDispatch {
  const MusePresentationDispatch({
    required this.requestRef,
    required this.resourceRef,
    required this.workspaceId,
    required this.adapterRef,
    required this.effectiveMode,
    required this.revision,
    this.disposition = 'open',
    this.requestedMode = 'view',
    this.placementHint = 'current-window',
    this.sessionRef = '',
    this.causeKind = '',
    this.viewId,
    this.layout,
    this.title,
    this.mountRef,
    this.relativePath,
    this.resolved = false,
  });

  /// Opaque `requestRef` of the v2 request; the answer is routed by this ref.
  final String requestRef;

  /// Opaque `resourceRef` exactly as the DSH requested it.
  final String resourceRef;

  /// Host workspace (Mount) identity the request was authorized against.
  final String workspaceId;

  /// Adapter the Host selected for the resolved target.
  final String adapterRef;

  /// `view`, `ephemeral-edit` or `edit` the Host will try to establish.
  final String effectiveMode;

  /// Host-known revision marker of the resolved target.
  final String revision;

  final String disposition;
  final String requestedMode;
  final String placementHint;
  final String sessionRef;
  final String causeKind;
  final String? viewId;
  final String? layout;
  final String? title;

  /// DSH `mountRef` the Host located the resource in. Opaque.
  final String? mountRef;

  /// Mount-relative POSIX path inside that Mount; empty is the Mount root.
  final String? relativePath;

  /// True when the Host resolved `resourceRef` to one of its own targets.
  final bool resolved;

  /// Decode one `muse.presentation/dispatch/v1` document.
  ///
  /// Strict on purpose: an unknown protocol or a missing identity field is not
  /// a dispatch, so the caller fails closed instead of opening something the
  /// Host did not authorize.
  static MusePresentationDispatch? fromWire(Object? value) {
    if (value is! Map) return null;
    if (value['protocol'] != musePresentationDispatchProtocol) return null;
    String? text(String key) {
      final field = value[key];
      return field is String && field.isNotEmpty ? field : null;
    }

    final requestRef = text('requestRef');
    final resourceRef = text('resourceRef');
    final workspaceId = text('workspaceId');
    final adapterRef = text('adapterRef');
    final effectiveMode = text('effectiveMode');
    final revision = text('revision');
    if (requestRef == null ||
        resourceRef == null ||
        workspaceId == null ||
        adapterRef == null ||
        effectiveMode == null ||
        revision == null) {
      return null;
    }
    final mountRef = text('mountRef');
    final relativePathValue = value['relativePath'];
    final relativePath =
        relativePathValue is String ? relativePathValue : null;
    if (mountRef != null && mountRef.length > maxMuseMountRefChars) return null;
    return MusePresentationDispatch(
      requestRef: requestRef,
      resourceRef: resourceRef,
      workspaceId: workspaceId,
      adapterRef: adapterRef,
      effectiveMode: effectiveMode,
      revision: revision,
      disposition: text('disposition') ?? 'open',
      requestedMode: text('requestedMode') ?? 'view',
      placementHint: text('placementHint') ?? 'current-window',
      sessionRef: text('sessionRef') ?? '',
      causeKind: text('causeKind') ?? '',
      viewId: text('viewId'),
      layout: text('layout'),
      title: text('title'),
      mountRef: mountRef,
      relativePath: relativePath,
      resolved: value['resolved'] == true,
    );
  }
}

/// What the Flutter surface established, or why it refused.
final class MusePresentationOutcome {
  const MusePresentationOutcome.opened({
    required this.surfaceInstanceRef,
    required this.selectedAdapterRef,
    required this.effectiveMode,
    required this.revision,
    this.result = 'opened',
    this.warnings = const <String>[],
    this.layout = 'resource',
    this.title,
  })  : reasonCode = null,
        detail = null;

  const MusePresentationOutcome.failed(this.reasonCode, this.detail)
      : result = 'failed',
        surfaceInstanceRef = null,
        selectedAdapterRef = null,
        effectiveMode = null,
        revision = null,
        warnings = const <String>[],
        layout = null,
        title = null;

  /// `opened`, `focused` or `fallback`; `failed` for a refusal.
  final String result;

  /// Opaque ref of the surface that now presents the resource.
  final String? surfaceInstanceRef;

  /// Adapter that performed the presentation.
  final String? selectedAdapterRef;

  /// Mode effectively established.
  final String? effectiveMode;

  /// Host-known revision of what is displayed.
  final String? revision;

  final List<String> warnings;
  final String? layout;
  final String? title;

  /// Mirrored reason code; non-null exactly when this outcome is a refusal.
  final String? reasonCode;

  /// Readable detail for logs and tests.
  final String? detail;

  bool get isFailure => reasonCode != null;

  /// The reply the Host accepts, or `null` when this outcome must fail closed.
  ///
  /// The Rust provider only projects `opened`/`focused`/`fallback` with an
  /// admissible mode and opaque refs into a receipt, so anything else is
  /// answered with `null` — the provider then fails closed instead of
  /// fabricating success.
  Map<String, Object?>? toWire() {
    if (isFailure) return null;
    final surface = surfaceInstanceRef;
    final adapter = selectedAdapterRef;
    final mode = effectiveMode;
    final known = revision;
    if (surface == null || adapter == null || mode == null || known == null) {
      return null;
    }
    if (!isOpaqueMuseRef(surface) ||
        !isOpaqueMuseRef(adapter) ||
        !isOpaqueMuseRef(known)) {
      return null;
    }
    if (!const {'opened', 'focused', 'fallback'}.contains(result)) return null;
    if (!const {'view', 'ephemeral-edit', 'edit'}.contains(mode)) return null;
    final boundedWarnings = warnings
        .where((warning) => warning.isNotEmpty && warning.length <= 256)
        .take(16)
        .toList(growable: false);
    return <String, Object?>{
      'result': result,
      'surfaceInstanceRef': surface,
      'selectedAdapterRef': adapter,
      'effectiveMode': mode,
      'revision': known,
      'warnings': boundedWarnings,
      if (layout != null) 'layout': layout,
      if (title != null) 'title': title,
    };
  }

  /// JSON reply for the Host, or `null` when nothing may be sent back.
  String? toWireJson() {
    final wire = toWire();
    return wire == null ? null : jsonEncode(wire);
  }
}

/// Resolves one Mount ref to the Host-side directory it is bound to.
typedef MuseMountRootResolver = String? Function(String mountRef);

/// Presents one already-authorized request in the Host UI.
typedef MuseResourcePresenter = Future<MuseResourceSurfaceOpenResult> Function(
  MuseResourceOpenRequest request,
);

/// The Flutter half of the `muse.resource-presentation` family.
///
/// It turns a Host dispatch (opaque Mount ref + Mount-relative path) into the
/// request `MuseResourceSurfaceOpener.open` already knows how to route, and
/// reports what actually happened. Nothing here fabricates success: a missing
/// Mount, a path that escapes the granted Mount, a missing file or an
/// unreachable surface all end in a refusal carrying a mirrored reason code.
final class MuseResourceSurfaceDispatcher {
  MuseResourceSurfaceDispatcher({
    MuseMountRootResolver? mountRoot,
    MuseResourcePresenter? presenter,
  })  : _mountRoot = mountRoot ?? resolveBoundMountRoot,
        _presenter = presenter ?? presentMuseResourceSurface;

  final MuseMountRootResolver _mountRoot;
  final MuseResourcePresenter _presenter;

  /// Present one dispatch, or refuse with a mirrored reason code.
  Future<MusePresentationOutcome> dispatch(
    MusePresentationDispatch dispatch,
  ) async {
    final mountRef = dispatch.mountRef;
    final relativePath = dispatch.relativePath;
    if (mountRef == null || mountRef.isEmpty) {
      if (!dispatch.resolved) {
        return MusePresentationOutcome.failed(
          MusePresentationReason.resourceNotFound,
          'the Host resolved ${dispatch.resourceRef} to nothing presentable',
        );
      }
      return MusePresentationOutcome.failed(
        MusePresentationReason.surfaceUnavailable,
        'no Flutter surface is bound to ${dispatch.resourceRef}',
      );
    }
    if (relativePath == null || !isMountRelativePath(relativePath)) {
      return MusePresentationOutcome.failed(
        MusePresentationReason.resourceRefInvalid,
        'the dispatch carried no admissible Mount-relative path',
      );
    }
    final root = _mountRoot(mountRef);
    if (root == null || root.trim().isEmpty) {
      return MusePresentationOutcome.failed(
        MusePresentationReason.mountNotBound,
        'Mount $mountRef is not bound in this Host',
      );
    }
    final segments =
        relativePath.isEmpty ? const <String>[] : relativePath.split('/');
    final request = MuseResourceOpenRequest(
      path: p.joinAll(<String>[root, ...segments]),
      origin: MuseResourceOpenOrigin.dshConversation,
      sessionCwd: root,
    );
    final MuseResourceSurfaceOpenResult result;
    try {
      result = await _presenter(request);
    } on Object catch (error) {
      return MusePresentationOutcome.failed(
        MusePresentationReason.surfaceUnavailable,
        'the Host surface threw: $error',
      );
    }
    switch (result.status) {
      case MuseResourceSurfaceOpenStatus.opened:
        final surfaceKey = result.surfaceKey;
        if (surfaceKey == null || surfaceKey.isEmpty) {
          return MusePresentationOutcome.failed(
            MusePresentationReason.surfaceUnavailable,
            'the surface reported no tab identity',
          );
        }
        return MusePresentationOutcome.opened(
          surfaceInstanceRef: museResourceSurfaceRef(surfaceKey),
          selectedAdapterRef: isOpaqueMuseRef(dispatch.adapterRef)
              ? dispatch.adapterRef
              : museHostResolvedAdapterRef,
          effectiveMode: dispatch.effectiveMode,
          // The surface's own revision wins when it reports an admissible one;
          // otherwise the Host revision carried by the dispatch stands.
          revision: _admissibleRevision(result.revision) ?? dispatch.revision,
          title: museResourceDispatchTitle(relativePath),
        );
      case MuseResourceSurfaceOpenStatus.unavailable:
        return MusePresentationOutcome.failed(
          MusePresentationReason.surfaceUnavailable,
          'the Host could not open a tab for $mountRef/$relativePath',
        );
      case MuseResourceSurfaceOpenStatus.failed:
        return MusePresentationOutcome.failed(
          museReasonForRouterCode(result.errorCode),
          'the resource router refused $mountRef/$relativePath'
          '${result.errorCode == null ? '' : ' (${result.errorCode})'}',
        );
    }
  }

  /// Decode and answer one Host payload. A payload that is not a dispatch is
  /// refused with `RESOURCE_REF_INVALID`.
  Future<MusePresentationOutcome> handleWire(Object? payload) async {
    final decoded = decodeMusePresentationWire(payload);
    final dispatch = MusePresentationDispatch.fromWire(decoded);
    if (dispatch == null) {
      return MusePresentationOutcome.failed(
        MusePresentationReason.resourceRefInvalid,
        'the payload is not a $musePresentationDispatchProtocol document',
      );
    }
    return this.dispatch(dispatch);
  }
}

/// Mount root the Host has granted for `mountRef`, or `null` when unbound.
String? resolveBoundMountRoot(String mountRef) {
  try {
    final root = getIt<MuseWorkspaceController>().requireMount(mountRef).rootLocator;
    return root.trim().isEmpty ? null : root;
  } on Object {
    return null;
  }
}

/// Routes one request through the existing Host resource surface path.
Future<MuseResourceSurfaceOpenResult> presentMuseResourceSurface(
  MuseResourceOpenRequest request,
) async {
  final context = AppGlobals.rootNavKey.currentContext;
  if (context == null) {
    return const MuseResourceSurfaceOpenResult.unavailable();
  }
  MuseResourceSurfaceOpenResult? reported;
  await MuseResourceSurfaceOpener.open(
    context,
    request,
    observer: (result) => reported = result,
  );
  return reported ?? const MuseResourceSurfaceOpenResult.unavailable();
}

/// Opaque surface-instance ref for one opened resource tab.
///
/// Derived from the tab identity (which carries a device path) so the ref is
/// stable per resource and still admits no device path across the bridge.
String museResourceSurfaceRef(String surfaceKey) =>
    'surface.muse.resource.'
    '${sha256.convert(utf8.encode(surfaceKey)).toString().substring(0, 24)}';

/// Display title of one Mount-relative locator, matching the Host's own rule.
String? museResourceDispatchTitle(String relativePath) {
  final name = p.posix.basename(relativePath);
  return name.isEmpty ? null : name;
}

/// Maps a `MuseLocalResourceRouter` code onto a mirrored Host reason code.
String museReasonForRouterCode(String? code) {
  switch (code) {
    case 'NOT_A_FILE':
      return MusePresentationReason.resourceNotFound;
    case 'OUTSIDE_SESSION_WORKSPACE':
      return MusePresentationReason.resourceRefInvalid;
    case 'EMPTY_PATH':
      return MusePresentationReason.resourceRefInvalid;
    case 'DSH_CWD_REQUIRED':
      return MusePresentationReason.mountNotBound;
    default:
      return MusePresentationReason.surfaceUnavailable;
  }
}

/// The surface-reported revision, but only when it is an opaque ref: a surface
/// that reports a path-shaped revision falls back to the Host's own.
String? _admissibleRevision(String? revision) =>
    revision != null && isOpaqueMuseRef(revision) ? revision : null;

/// Decodes a dispatch payload coming off the Flutter↔Rust channel: raw bytes,
/// a JSON string, or an already-decoded document.
Object? decodeMusePresentationWire(Object? payload) {
  if (payload is String) {
    try {
      return jsonDecode(payload);
    } on Object {
      return null;
    }
  }
  if (payload is List<int>) {
    try {
      return jsonDecode(utf8.decode(payload));
    } on Object {
      return null;
    }
  }
  return payload;
}
