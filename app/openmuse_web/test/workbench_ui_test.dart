import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';

import 'package:openmuse_web/workbench/web_workbench_page.dart';

void main() {
  testWidgets('Web uses the Desktop workbench chrome and pane actions', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(() {
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    });
    await tester.pumpWidget(
      MaterialApp(theme: buildOpenMuseTheme(), home: const WebWorkbenchPage()),
    );
    expect(find.text('OpenMuse'), findsOneWidget);
    expect(find.text('Project Workspace'), findsOneWidget);
    expect(find.text('Blank page'), findsOneWidget);
    expect(find.text('从工作区开始'), findsOneWidget);
    expect(find.text('Open File Viewer'), findsNothing);
    expect(find.byType(AppBar), findsNothing);
    expect(find.byKey(const Key('pane-resizer')), findsWidgets);

    await tester.tap(find.byKey(const Key('pane-menu-button:editor')));
    await tester.pumpAndSettle();
    expect(find.text('向右切分'), findsOneWidget);
    expect(find.text('与右侧交换'), findsOneWidget);
    await tester.tap(find.text('向右切分'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pane-menu-button:pane-1')), findsOneWidget);
  });

  testWidgets('paired Workspace loads children on expand and opens Viewer', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(() {
      tester.view.resetPhysicalSize();
      tester.view.resetDevicePixelRatio();
    });
    final port = _FakeMirrorPort();
    await tester.pumpWidget(
      MaterialApp(
        theme: buildOpenMuseTheme(),
        home: WebWorkbenchPage(
          desktopRef: 'desktop',
          workspaceRef: 'workspace',
          mirrorPort: port,
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(port.mountCalls, 1);
    expect(port.childCalls, 0);
    await tester.tap(find.text('Repo'));
    await tester.pumpAndSettle();
    expect(port.childCalls, 1);
    await tester.tap(find.text('readme.md'));
    await tester.pumpAndSettle();
    expect(find.byType(OpenMuseFileViewerBody), findsOneWidget);
    expect(find.text('Shared preview'), findsOneWidget);
    expect(port.resourceCalls, 1);
  });
}

final class _FakeMirrorPort
    implements WorkspaceMirrorPort, WorkspaceResourcePort {
  int mountCalls = 0;
  int childCalls = 0;
  int resourceCalls = 0;

  @override
  Future<List<WorkspaceMirrorMount>> listMounts(String workspaceRef) async {
    mountCalls++;
    return [
      const WorkspaceMirrorMount(
        mountRef: 'mount',
        rootRef: 'root',
        title: 'Repo',
      ),
    ];
  }

  @override
  Future<WorkspaceMirrorPage> listChildren({
    required String workspaceRef,
    required String mountRef,
    required String parentRef,
    required int limit,
    String? cursor,
  }) async {
    childCalls++;
    return const WorkspaceMirrorPage(
      entries: [
        WorkspaceMirrorEntry(
          nodeRef: 'file',
          name: 'readme.md',
          isDirectory: false,
          resourceRef: 'resource',
        ),
      ],
    );
  }

  @override
  Future<Uint8List> readResource({
    required String workspaceRef,
    required String resourceRef,
  }) async {
    resourceCalls++;
    return Uint8List.fromList(utf8.encode('# Shared preview'));
  }
}
