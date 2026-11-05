import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/muse_presentation_dispatch.dart';
import 'package:appflowy/plugins/resource_surface/resource_file_plugin.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/startup/tasks/app_widget.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_persistence.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as p;

const String _mountRef = 'mount:test';

/// These tests drive the real `MuseResourceSurfaceOpener.open` — real resource
/// router, real containment check, real observer — with only the app tab
/// registry (an infrastructure detail, not the seam) stood in for: the real
/// `TabsBloc` opens the tab by asking Rust for the latest view, and a unit test
/// has no `dart_ffi.dll`.
void main() {
  late Directory mount;
  late _FakeTabsBloc tabs;

  setUp(() async {
    mount = await Directory.systemTemp.createTemp('openmuse-surface-');
    await Directory(p.join(mount.path, 'notes')).create(recursive: true);
    await File(p.join(mount.path, 'notes', 'spec.md')).writeAsString('# spec');
    tabs = _FakeTabsBloc();
    getIt.registerSingleton<TabsBloc>(tabs);
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
      dispose: (controller) => controller.dispose(),
    );
  });

  tearDown(() async {
    await getIt.reset();
    if (mount.existsSync()) await mount.delete(recursive: true);
  });

  /// The Host window the dispatcher presents into.
  Future<void> pumpHostWindow(WidgetTester tester) => tester.pumpWidget(
        MaterialApp(
          navigatorKey: AppGlobals.rootNavKey,
          home: const Scaffold(body: SizedBox.shrink()),
        ),
      );

  /// The shared router resolves real files, so dispatches run outside the
  /// widget test's fake async zone.
  Future<MusePresentationOutcome?> dispatch(
    WidgetTester tester,
    String payload,
  ) async {
    final dispatcher = MuseResourceSurfaceDispatcher();
    return tester.runAsync(() => dispatcher.handleWire(payload));
  }

  testWidgets('a dispatch opens the file through MuseResourceSurfaceOpener',
      (tester) async {
    await pumpHostWindow(tester);

    final outcome = await dispatch(tester, _wireJson());

    expect(outcome, isNotNull);
    expect(outcome!.isFailure, isFalse);
    expect(outcome.result, 'opened');
    expect(outcome.layout, 'resource');
    expect(outcome.title, 'spec.md');
    expect(outcome.effectiveMode, 'view');
    expect(outcome.revision, 'revision.7');
    expect(outcome.selectedAdapterRef, museHostResolvedAdapterRef);

    // The tab the Host asked for really reached the tab registry, with the
    // resolved file and the Mount as its session.
    expect(tabs.opened, hasLength(1));
    final plugin = tabs.opened.single;
    expect(plugin, isA<MuseResourceFilePlugin>());
    final resource = (plugin as MuseResourceFilePlugin).resource;
    expect(resource.file.path, p.join(mount.path, 'notes', 'spec.md'));
    expect(resource.origin, MuseResourceOpenOrigin.dshConversation);

    // What the Host gets back is opaque refs and a readable title only.
    final reply = jsonDecode(outcome.toWireJson()!) as Map<String, dynamic>;
    expect(reply['surfaceInstanceRef'], outcome.surfaceInstanceRef);
    expect(reply['selectedAdapterRef'], museHostResolvedAdapterRef);
    expect(reply['revision'], 'revision.7');
    expect(
      reply['surfaceInstanceRef'],
      isNot(startsWith('surface.muse.resource.resource:')),
    );
    final wire = outcome.toWireJson()!;
    expect(wire, isNot(contains('\\')));
    expect(wire, isNot(contains(':/')));
    expect(wire, isNot(contains(mount.path)));
  });

  testWidgets('a missing file is refused with RESOURCE_NOT_FOUND',
      (tester) async {
    await pumpHostWindow(tester);

    final outcome = await dispatch(
      tester,
      _wireJson(relativePath: 'notes/gone.md'),
    );

    expect(outcome!.isFailure, isTrue);
    expect(outcome.reasonCode, MusePresentationReason.resourceNotFound);
    expect(outcome.toWireJson(), isNull);
    expect(tabs.opened, isEmpty);
  });

  testWidgets('an escaped Mount path is refused with RESOURCE_REF_INVALID',
      (tester) async {
    await pumpHostWindow(tester);

    final outcome = await dispatch(
      tester,
      _wireJson(relativePath: '../outside.md'),
    );

    expect(outcome!.reasonCode, MusePresentationReason.resourceRefInvalid);
    expect(outcome.toWireJson(), isNull);
    expect(tabs.opened, isEmpty);
  });

  testWidgets('a surface that cannot open a tab reports SURFACE_UNAVAILABLE',
      (tester) async {
    tabs.failToOpen = true;
    await pumpHostWindow(tester);

    final outcome = await dispatch(tester, _wireJson());

    expect(outcome!.reasonCode, MusePresentationReason.surfaceUnavailable);
    expect(outcome.toWireJson(), isNull);
  });
}

/// Stands in for the app tab registry. The seam's contract is the open request
/// it hands over, not the tab widget tree.
final class _FakeTabsBloc implements TabsBloc {
  final List<Plugin> opened = <Plugin>[];
  bool failToOpen = false;

  @override
  void openExternalPlugin(Plugin plugin) {
    if (failToOpen) throw StateError('this surface cannot open a tab');
    opened.add(plugin);
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnsupportedError(
        'the seam must not reach TabsBloc.${invocation.memberName}',
      );
}

/// One `muse.presentation/dispatch/v1` payload.
String _wireJson({
  String? mountRef = _mountRef,
  String? relativePath = 'notes/spec.md',
}) =>
    jsonEncode(<String, Object?>{
      'protocol': musePresentationDispatchProtocol,
      'requestRef': 'request.1',
      'resourceRef': 'resource.abc',
      'workspaceId': 'workspace.1',
      'disposition': 'open',
      'requestedMode': 'view',
      'placementHint': 'current-window',
      'adapterRef': museHostResolvedAdapterRef,
      'effectiveMode': 'view',
      'sessionRef': 'session.1',
      'revision': 'revision.7',
      'causeKind': 'deliverable',
      'resolved': true,
      if (mountRef != null) 'mountRef': mountRef,
      if (relativePath != null) 'relativePath': relativePath,
    });
