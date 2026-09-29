import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile/main.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  testWidgets('signed-out production composition has no fixture account', (
    tester,
  ) async {
    await tester.pumpWidget(
      OpenMuseHostShell(composition: mobileComposition()),
    );
    await tester.pumpAndSettle();
    expect(find.text('未登录'), findsOneWidget);
    expect(find.text('请登录以访问 Cloud Workspace'), findsOneWidget);
    expect(find.text('OpenMuse Cloud'), findsNothing);
  });

  testWidgets('service-backed composition opens a Remote DSH session', (
    tester,
  ) async {
    final service = _FakeCloudService();
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: mobileComposition(
          session: const MobileAccountSession.authenticated('Test Account'),
          cloudService: service,
          dshConnector: service,
          resources: service,
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('Cloud Project'), findsOneWidget);
    await tester.tap(find.text('Cloud Project'));
    await tester.pumpAndSettle();
    expect(find.text('Cloud Workspace · Remote DSH'), findsOneWidget);
    expect(find.text('Revision r1'), findsOneWidget);
    expect(find.text('Storage · 可写'), findsOneWidget);
    expect(find.text('DSH · binding'), findsOneWidget);
    expect(find.byKey(const ValueKey('remote-dsh-session')), findsOneWidget);
  });
}

final class _FakeCloudService
    implements CloudWorkspaceService, DshRuntimeConnector, ResourceRangePort {
  @override
  DshPlacement get placement => DshPlacement.cloudRemote;

  @override
  Future<List<CloudWorkspaceRecord>> listWorkspaces() async => const [
    CloudWorkspaceRecord(
      workspaceRef: 'cloud:w1',
      title: 'Cloud Project',
      revision: 'r1',
      writable: true,
      storageState: CloudStorageState.available,
    ),
  ];

  @override
  Future<DshSessionDescriptor> open(
    String workspaceRef,
    int generation,
  ) async => DshSessionDescriptor(
    sessionRef: 'dsh:s1',
    origin: 'https://dsh.example.test',
    path: '/session/s1',
    generation: generation,
  );

  @override
  Future<void> close(String sessionRef) async {}

  @override
  Future<ResourceHandle> issueResourceHandle({
    required String workspaceRef,
    required String resourceRef,
    required String revision,
    required String audience,
    required int generation,
  }) => throw UnimplementedError();

  @override
  Future<CloudChangeProposal> propose({
    required String workspaceRef,
    required String expectedRevision,
    required String instruction,
    required int generation,
  }) => throw UnimplementedError();

  @override
  Future<CloudApplyReceipt> approve({
    required CloudChangeProposal proposal,
    required int generation,
  }) => throw UnimplementedError();

  @override
  Future<List<int>> read(ResourceHandle handle, int start, int endExclusive) =>
      throw UnimplementedError();
}
