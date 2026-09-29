import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile/main.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  test('DOCX capability is advertised only when an engine is injected', () {
    expect(
      mobileComposition().capabilitySnapshot.capabilities,
      isNot(contains('office.docx.engine')),
    );
    expect(
      mobileComposition(
        officeEngine: _FakeOfficeEngine(),
      ).capabilitySnapshot.capabilities,
      contains('office.docx.engine'),
    );
  });

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

  testWidgets('authorized DOCX catalog route opens and commits by receipt', (
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
          resourceCatalog: service,
          officeCommits: service,
          officeEngine: _FakeOfficeEngine(),
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Cloud Project'));
    await tester.pumpAndSettle();
    expect(find.text('Document.docx'), findsOneWidget);
    await tester.tap(find.text('Document.docx'));
    await tester.pumpAndSettle();
    expect(find.text('DOCX · simple-text'), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('docx-paragraph-0')),
      'changed',
    );
    await tester.tap(find.byKey(const ValueKey('docx-save')));
    await tester.pumpAndSettle();
    expect(find.text('已保存 · docx-r2'), findsOneWidget);
  });
}

final class _FakeOfficeEngine implements OfficeEnginePort {
  @override
  String get abi => 'openmuse-docx-ffi@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => const OfficeEngineInspection(
    format: OfficeFormat.word,
    profile: 'simple-text',
    paragraphs: ['original'],
    capabilities: {
      OfficeCapability.view,
      OfficeCapability.edit,
      OfficeCapability.export,
    },
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) async => const [9, 8, 7];
}

final class _FakeCloudService
    implements
        CloudWorkspaceService,
        CloudResourceCatalogPort,
        DshRuntimeConnector,
        ResourceRangePort,
        OfficeResourceCommitPort {
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
  Future<List<CloudResourceRecord>> listResources({
    required String workspaceRef,
    required String revision,
    required int generation,
  }) async => const [
    CloudResourceRecord(
      resourceRef: 'resource:docx',
      title: 'Document.docx',
      revision: 'docx-r1',
      size: 3,
      mediaType:
          'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      writable: true,
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
  }) async => ResourceHandle(
    resourceRef: resourceRef,
    revision: revision,
    audience: audience,
    generation: generation,
    expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
    size: 3,
    mediaType:
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  );

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
      Future.value(const [1, 2, 3].sublist(start, endExclusive));

  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) async => OfficeResourceCommitReceipt(
    commitRef: 'commit:docx',
    resourceRef: resourceRef,
    previousRevision: expectedRevision,
    newRevision: 'docx-r2',
    generation: generation,
  );
}
