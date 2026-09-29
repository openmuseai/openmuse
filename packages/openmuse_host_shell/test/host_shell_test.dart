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
  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async => const [
    WorkspaceSummary(
      workspaceRef: 'cloud:one',
      title: 'Cloud Project',
      placement: WorkspacePlacement.cloud,
      writable: true,
      runningSessionRef: 'session-1',
    ),
  ];
}

final class _Capabilities implements CapabilitySnapshotPort {
  @override
  Set<String> get capabilities => const {'workspace.catalog'};
}

void main() {
  testWidgets('shared shell renders an injected workspace', (tester) async {
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: OpenMuseHostComposition(
          platform: OpenMuseHostPlatform.mobile,
          session: _Session(),
          workspaceCatalog: _Catalog(),
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
  });
}
