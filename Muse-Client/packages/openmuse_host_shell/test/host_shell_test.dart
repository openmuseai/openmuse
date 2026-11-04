import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';

final class _Session implements OpenMuseSessionPort {
  @override
  String? get accountLabel => 'user@example.test';
  @override
  bool get signedIn => true;
}

final class _Catalog implements WorkspaceCatalogPort {
  int calls = 0;

  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async {
    calls += 1;
    return const [
      WorkspaceSummary(
        workspaceRef: 'cloud:one',
        title: 'Cloud Project',
        placement: WorkspacePlacement.cloud,
        writable: true,
        runningSessionRef: 'session-1',
      ),
    ];
  }
}

final class _FailingCatalog implements WorkspaceCatalogPort {
  var shouldFail = true;

  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async {
    if (shouldFail) throw StateError('catalog unavailable');
    return const [];
  }
}

final class _Capabilities implements CapabilitySnapshotPort {
  @override
  Set<String> get capabilities => const {'workspace.catalog'};
}

void main() {
  testWidgets('shared shell renders an injected workspace', (tester) async {
    final catalog = _Catalog();
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: OpenMuseHostComposition(
          platform: OpenMuseHostPlatform.mobile,
          session: _Session(),
          workspaceCatalog: catalog,
          capabilitySnapshot: _Capabilities(),
          workspaceBuilder: (_, workspace) =>
              Scaffold(body: Text(workspace.workspaceRef)),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('Cloud Project'), findsOneWidget);
    expect(find.text('Cloud Workspace · DSH running'), findsOneWidget);
    expect(find.text('user@example.test'), findsOneWidget);
    expect(catalog.calls, 1);

    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: OpenMuseHostComposition(
          platform: OpenMuseHostPlatform.mobile,
          session: _Session(),
          workspaceCatalog: catalog,
          capabilitySnapshot: _Capabilities(),
          workspaceBuilder: (_, workspace) =>
              Scaffold(body: Text(workspace.workspaceRef)),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(catalog.calls, 1);

    await tester.tap(find.byKey(const ValueKey('workspace-refresh')));
    await tester.pumpAndSettle();
    expect(catalog.calls, 2);
  });

  testWidgets('workspace load failures are visible and retryable', (
    tester,
  ) async {
    final catalog = _FailingCatalog();
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: OpenMuseHostComposition(
          platform: OpenMuseHostPlatform.mobile,
          session: _Session(),
          workspaceCatalog: catalog,
          capabilitySnapshot: _Capabilities(),
          workspaceBuilder: (_, workspace) =>
              Scaffold(body: Text(workspace.workspaceRef)),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('工作区加载失败'), findsOneWidget);
    expect(find.textContaining('catalog unavailable'), findsOneWidget);

    catalog.shouldFail = false;
    await tester.tap(find.byKey(const ValueKey('workspace-retry')));
    await tester.pumpAndSettle();
    expect(find.text('没有可用的 Workspace'), findsOneWidget);
  });
}
