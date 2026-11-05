import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/muse_presentation_dispatch.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_dialog.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_persistence.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as p;

const String _mountRef = 'mount:test';

/// The dispatch JSON `MusePresentationDispatch::to_wire()` emits. The same
/// literal is asserted from Rust in
/// `rust-lib/dart-ffi/src/muse_presentation_ffi.rs`, so a key rename on either
/// side fails a test instead of silently degrading every presentation to
/// `SURFACE_UNAVAILABLE`.
const String hostDispatchFixture =
    '{"protocol":"muse.presentation/dispatch/v1","requestRef":"request.9f",'
    '"resourceRef":"resource.7a3","workspaceId":"workspace.appflowy.1",'
    '"disposition":"open","requestedMode":"view",'
    '"placementHint":"current-window","adapterRef":"muse.adapter.host-resolved",'
    '"effectiveMode":"view","sessionRef":"session.4c","revision":"revision.12",'
    '"causeKind":"deliverable","resolved":true,"mountRef":"mount:test",'
    '"relativePath":"notes/spec.md"}';

/// The reply JSON `MusePresentationOutcome.toWireJson()` sends back, pinned
/// byte for byte against `MusePresentationOutcome::from_wire` in the Rust test.
const String dartOpenedReplyFixture =
    '{"result":"opened",'
    '"surfaceInstanceRef":"surface.muse.resource.0f1e2d3c4b5a69788796a5b4c3d2e1f0",'
    '"selectedAdapterRef":"muse.adapter.host-resolved",'
    '"effectiveMode":"view","revision":"revision.12",'
    '"warnings":[],"layout":"resource","title":"spec.md"}';

void main() {
  late Directory mount;
  late Directory outside;

  setUp(() async {
    mount = await Directory.systemTemp.createTemp('openmuse-dispatch-');
    outside = await Directory.systemTemp.createTemp('openmuse-dispatch-out-');
    await Directory(p.join(mount.path, 'notes')).create(recursive: true);
    await File(p.join(mount.path, 'notes', 'spec.md')).writeAsString('# spec');
    getIt.registerSingleton<MuseWorkspaceController>(
      MuseWorkspaceController(
        providers: MuseWorkspaceProviderRegistry()
          ..register(MuseLocalWorkspaceProvider()),
        persistence: MuseWorkspacePersistence(),
      )..mounts.add(
          MuseWorkspaceMount(
            mountRef: _mountRef,
            providerId: MuseLocalWorkspaceProvider.providerId,
            bindingKey: 'device-local:$_mountRef',
            displayName: 'mount',
            rootLocator: mount.path,
            readOnly: false,
            order: 0,
          ),
        ),
    );
  });

  tearDown(() async {
    await getIt.reset();
    if (mount.existsSync()) await mount.delete(recursive: true);
    if (outside.existsSync()) await outside.delete(recursive: true);
  });

  test('routes one Mount dispatch into the opener with the Host request',
      () async {
    final presenter = _RecordingPresenter();
    final dispatcher = MuseResourceSurfaceDispatcher(presenter: presenter.call);

    final outcome = await dispatcher.handleWire(_wireJson());

    // The dispatch reached the opener as exactly the request the seam owns:
    // Mount root, Mount-relative path, DSH origin and the Mount as session cwd.
    expect(presenter.requests, hasLength(1));
    final request = presenter.requests.single;
    expect(request.path, p.join(mount.path, 'notes', 'spec.md'));
    expect(request.origin, MuseResourceOpenOrigin.dshConversation);
    expect(request.sessionCwd, mount.path);

    expect(outcome.isFailure, isFalse);
    expect(outcome.result, 'opened');
    expect(outcome.layout, 'resource');
    expect(outcome.title, 'spec.md');
    expect(outcome.effectiveMode, 'view');
    expect(outcome.revision, 'revision.7');
    expect(outcome.selectedAdapterRef, museHostResolvedAdapterRef);
    expect(isOpaqueMuseRef(outcome.surfaceInstanceRef!), isTrue);
    expect(outcome.surfaceInstanceRef, startsWith('surface.muse.resource.'));

    // Only opaque refs cross back: no device path may be sent to the Host.
    final reply = outcome.toWireJson()!;
    expect(reply, isNot(contains('\\')));
    expect(reply, isNot(contains(':/')));
    expect(reply, isNot(contains(mount.path)));
    expect(
      jsonDecode(reply),
      containsPair('surfaceInstanceRef', outcome.surfaceInstanceRef),
    );
  });

  test('refuses an unbound Mount with MOUNT_NOT_BOUND', () async {
    final presenter = _RecordingPresenter();
    final dispatcher = MuseResourceSurfaceDispatcher(presenter: presenter.call);

    final outcome = await dispatcher.handleWire(
      _wireJson(mountRef: 'mount:missing'),
    );

    expect(outcome.isFailure, isTrue);
    expect(outcome.reasonCode, MusePresentationReason.mountNotBound);
    expect(presenter.requests, isEmpty);
    expect(outcome.toWireJson(), isNull);
  });

  test('refuses a path that escapes the Mount and never opens anything',
      () async {
    final presenter = _RecordingPresenter();
    final dispatcher = MuseResourceSurfaceDispatcher(presenter: presenter.call);

    for (final escaping in [
      '../secret.md',
      'notes/../../secret.md',
      '..',
      'notes/./spec.md',
    ]) {
      final outcome = await dispatcher.handleWire(
        _wireJson(relativePath: escaping),
      );
      expect(
        outcome.reasonCode,
        MusePresentationReason.resourceRefInvalid,
        reason: escaping,
      );
      expect(outcome.toWireJson(), isNull, reason: escaping);
    }
    expect(presenter.requests, isEmpty);
  });

  test('never opens a device path handed over by the bridge', () async {
    final presenter = _RecordingPresenter();
    final dispatcher = MuseResourceSurfaceDispatcher(presenter: presenter.call);

    for (final devicePath in [
      r'C:\Windows\win.ini',
      'C:/Windows/win.ini',
      r'\\server\share\secret.md',
      '/etc/passwd',
      'notes\\spec.md',
    ]) {
      final outcome = await dispatcher.handleWire(
        _wireJson(relativePath: devicePath),
      );
      expect(
        outcome.reasonCode,
        MusePresentationReason.resourceRefInvalid,
        reason: devicePath,
      );
    }
    expect(presenter.requests, isEmpty);
  });

  test('reports a missing file or a directory as RESOURCE_NOT_FOUND', () async {
    // The presenter resolves with the shared router, as the real opener does.
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: _routerPresenter,
    );

    final missing = await dispatcher.handleWire(
      _wireJson(relativePath: 'notes/gone.md'),
    );
    expect(missing.reasonCode, MusePresentationReason.resourceNotFound);

    final directory = await dispatcher.handleWire(
      _wireJson(relativePath: 'notes'),
    );
    expect(directory.reasonCode, MusePresentationReason.resourceNotFound);
  });

  test('keeps the containment check for a link that leaves the Mount',
      () async {
    final secret = await File(p.join(outside.path, 'secret.md'))
        .writeAsString('secret');
    await Link(p.join(mount.path, 'linked.md')).create(secret.path);
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: _routerPresenter,
    );

    final outcome = await dispatcher.handleWire(
      _wireJson(relativePath: 'linked.md'),
    );

    expect(outcome.isFailure, isTrue);
    expect(outcome.reasonCode, MusePresentationReason.resourceRefInvalid);
    expect(outcome.toWireJson(), isNull);
  });

  test('reports SURFACE_UNAVAILABLE when the surface cannot open a tab',
      () async {
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: (_) async => const MuseResourceSurfaceOpenResult.unavailable(),
    );

    final outcome = await dispatcher.handleWire(_wireJson());

    expect(outcome.reasonCode, MusePresentationReason.surfaceUnavailable);
    expect(outcome.toWireJson(), isNull);
  });

  test('fails closed on a payload that is not a dispatch', () async {
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: _RecordingPresenter().call,
    );

    for (final payload in <Object?>[
      null,
      const <String, Object?>{},
      _wireJson(protocol: 'muse.presentation/dispatch/v0'),
      _wireJson(adapterRef: null),
      _wireJson(requestRef: null),
      utf8.encode('{not json'),
      '{not json',
      '[]',
    ]) {
      final outcome = await dispatcher.handleWire(payload);
      expect(
        outcome.reasonCode,
        MusePresentationReason.resourceRefInvalid,
        reason: payload.toString(),
      );
      expect(outcome.toWireJson(), isNull);
    }
  });

  test('tells an unresolved ref from a Host view the surface cannot present',
      () async {
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: _RecordingPresenter().call,
    );

    final unresolved = await dispatcher.dispatch(
      MusePresentationDispatch.fromWire(
        _wire(mountRef: null, relativePath: null, resolved: false),
      )!,
    );
    expect(unresolved.reasonCode, MusePresentationReason.resourceNotFound);

    final hostView = await dispatcher.dispatch(
      MusePresentationDispatch.fromWire(
        _wire(
          mountRef: null,
          relativePath: null,
          viewId: 'view.1',
          layout: 'document',
        ),
      )!,
    );
    expect(hostView.reasonCode, MusePresentationReason.surfaceUnavailable);
  });

  test('parses the dispatch literal the Host actually emits', () {
    final dispatch = MusePresentationDispatch.fromWire(
      jsonDecode(hostDispatchFixture),
    );

    expect(dispatch, isNotNull);
    expect(dispatch!.requestRef, 'request.9f');
    expect(dispatch.resourceRef, 'resource.7a3');
    expect(dispatch.mountRef, 'mount:test');
    expect(dispatch.relativePath, 'notes/spec.md');
    expect(dispatch.resolved, isTrue);
    expect(dispatch.effectiveMode, 'view');
    expect(dispatch.revision, 'revision.12');
  });

  test('answers with the reply literal the Host accepts', () {
    const outcome = MusePresentationOutcome.opened(
      surfaceInstanceRef:
          'surface.muse.resource.0f1e2d3c4b5a69788796a5b4c3d2e1f0',
      selectedAdapterRef: museHostResolvedAdapterRef,
      effectiveMode: 'view',
      revision: 'revision.12',
      title: 'spec.md',
    );

    expect(outcome.toWireJson(), dartOpenedReplyFixture);
  });

  test('lets the surface revision win and drops an inadmissible one', () async {
    final revision = <String?>['revision.surface.9', r'C:\tmp\spec.md'];
    var index = 0;
    final dispatcher = MuseResourceSurfaceDispatcher(
      presenter: (request) async => MuseResourceSurfaceOpenResult.opened(
        resource: _resolved(request),
        surfaceKey: 'resource:${request.path}',
        revision: revision[index++],
      ),
    );

    final fromSurface = await dispatcher.handleWire(_wireJson());
    expect(fromSurface.revision, 'revision.surface.9');

    final fromDispatch = await dispatcher.handleWire(_wireJson());
    expect(fromDispatch.revision, 'revision.7');
    expect(fromDispatch.toWireJson(), isNot(contains('tmp')));
  });
}

/// A presenter that records the request it was handed, as the opener does.
final class _RecordingPresenter {
  final List<MuseResourceOpenRequest> requests = [];

  Future<MuseResourceSurfaceOpenResult> call(
    MuseResourceOpenRequest request,
  ) async {
    requests.add(request);
    return MuseResourceSurfaceOpenResult.opened(
      resource: _resolved(request),
      surfaceKey: 'resource:${request.path}',
    );
  }
}

/// The resolve-and-map half of `MuseResourceSurfaceOpener.open`, without the
/// Flutter tab: it keeps the shared containment check under test.
Future<MuseResourceSurfaceOpenResult> _routerPresenter(
  MuseResourceOpenRequest request,
) async {
  try {
    final resolved = await MuseLocalResourceRouter().resolve(request);
    return MuseResourceSurfaceOpenResult.opened(
      resource: resolved,
      surfaceKey: 'resource:${resolved.file.path}',
    );
  } on MuseResourceOpenException catch (error) {
    return MuseResourceSurfaceOpenResult.failed(error.code);
  }
}

MuseResolvedResource _resolved(MuseResourceOpenRequest request) =>
    MuseResolvedResource(
      file: File(request.path),
      engine: MuseLocalEngine.helix,
      origin: request.origin,
    );

/// One `muse.presentation/dispatch/v1` document, as `to_wire` projects it.
Map<String, Object?> _wire({
  String? protocol,
  String? requestRef = 'request.1',
  String? resourceRef = 'resource.abc',
  String? adapterRef = museHostResolvedAdapterRef,
  String? mountRef = _mountRef,
  String? relativePath = 'notes/spec.md',
  bool resolved = true,
  String? viewId,
  String? layout,
  String effectiveMode = 'view',
  String revision = 'revision.7',
}) =>
    <String, Object?>{
      'protocol': protocol ?? musePresentationDispatchProtocol,
      'requestRef': requestRef,
      'resourceRef': resourceRef,
      'workspaceId': 'workspace.1',
      'disposition': 'open',
      'requestedMode': 'view',
      'placementHint': 'current-window',
      'adapterRef': adapterRef,
      'effectiveMode': effectiveMode,
      'sessionRef': 'session.1',
      'revision': revision,
      'causeKind': 'deliverable',
      'resolved': resolved,
      if (viewId != null) 'viewId': viewId,
      if (layout != null) 'layout': layout,
      if (mountRef != null) 'mountRef': mountRef,
      if (relativePath != null) 'relativePath': relativePath,
    };

String _wireJson({
  String? protocol,
  String? requestRef = 'request.1',
  String? resourceRef = 'resource.abc',
  String? adapterRef = museHostResolvedAdapterRef,
  String? mountRef = _mountRef,
  String? relativePath = 'notes/spec.md',
  bool resolved = true,
  String? viewId,
  String? layout,
  String effectiveMode = 'view',
  String revision = 'revision.7',
}) =>
    jsonEncode(
      _wire(
        protocol: protocol,
        requestRef: requestRef,
        resourceRef: resourceRef,
        adapterRef: adapterRef,
        mountRef: mountRef,
        relativePath: relativePath,
        resolved: resolved,
        viewId: viewId,
        layout: layout,
        effectiveMode: effectiveMode,
        revision: revision,
      ),
    );
