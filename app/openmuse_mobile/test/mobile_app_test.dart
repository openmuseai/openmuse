import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile/main.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

void main() {
  testWidgets('mobile distribution loads the GoTrue login plugin by default', (
    tester,
  ) async {
    final authentication = _FakeAuthenticationController();
    final cloudPlugin = OpenMuseCloudWorkspacePlugin(
      authentication: authentication,
      cloudOrigin: Uri.parse('https://cloud.openmuse.test'),
      deviceId: 'mobile-test',
    );
    await tester.pumpWidget(
      OpenMuseMobileApplication(
        authenticationPlugin: OpenMuseGoTruePlugin(
          authentication: authentication,
          cloudLabel: 'https://cloud.openmuse.test',
        ),
        cloudWorkspacePlugin: cloudPlugin,
        pairedDesktopPlugin: OpenMusePairedDesktopMobilePlugin(
          client: PairedDesktopClient(
            origin: Uri.parse('https://desktop.openmuse.test'),
            accessToken: authentication.accessToken,
            deviceRef: 'mobile-test',
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(authentication.restoreCalls, 1);
    expect(find.text('Welcome to OpenMuse'), findsOneWidget);
    expect(find.byKey(const ValueKey('auth.email')), findsOneWidget);
    expect(find.text('请登录以访问 Cloud Workspace'), findsNothing);
  });

  test('DOCX capability is advertised only when an engine is injected', () {
    expect(
      mobileComposition().capabilitySnapshot.capabilities,
      isNot(contains('office.docx.engine')),
    );
    final viewers = MultiFormatOfficeEngine({
      OfficeFormat.pdf: _FakePdfEngine(),
    });
    expect(
      mobileComposition(officeEngine: viewers).capabilitySnapshot.capabilities,
      contains('office.pdf.engine'),
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

  testWidgets('same-account Desktop screen has no pairing-code step', (
    tester,
  ) async {
    final controller = PairedDesktopMobileController.direct(
      PairedDesktopClient(
        origin: Uri.parse('https://desktop.openmuse.test'),
        accessToken: () async => 'token',
        deviceRef: 'mobile.test',
      ),
    );
    final device = AccountDevice.fromJson({
      'deviceId': 'desktop.test',
      'displayName': 'MacBook Pro',
      'platform': 'macos',
      'deviceKind': 'desktop',
      'capabilities': ['paired-desktop.transport'],
      'transportOrigin': 'https://desktop.openmuse.test',
      'lastSeenAt': DateTime.now().millisecondsSinceEpoch,
      'online': true,
    });
    await tester.pumpWidget(
      MaterialApp(
        home: PairedDesktopConnectScreen(
          controller: controller,
          initialDevice: device,
        ),
      ),
    );

    expect(find.text('选择电脑'), findsOneWidget);
    expect(find.text('进入 MacBook Pro'), findsOneWidget);
    expect(find.byKey(const ValueKey('paired-desktop-code')), findsNothing);
    controller.dispose();
  });

  testWidgets('Desktop workspace hub mirrors device, tasks and spaces', (
    tester,
  ) async {
    const connection = PairedDesktopConnection(
      accountRef: 'account.1',
      deviceRef: 'desktop.1',
      deviceName: 'DESKTOP-FBRL8RL',
      workspaceRef: 'workspace.1',
      workspaceTitle: 'openmuse-io',
      grantRef: 'grant.1',
      expiresAtMs: 4102444800000,
      session: DshSessionDescriptor(
        sessionRef: 'session.1',
        origin: 'https://desktop.openmuse.test',
        path: '/u/grant.1',
        generation: 1,
      ),
    );
    await tester.pumpWidget(
      const MaterialApp(
        home: PairedDesktopWorkspaceScreen(connection: connection),
      ),
    );

    expect(find.text('DESKTOP-FBRL8RL'), findsOneWidget);
    expect(find.text('新建任务'), findsOneWidget);
    expect(find.text('全部对话'), findsOneWidget);
    expect(find.text('进行中、等待输入与已完成任务实时同步'), findsOneWidget);
    expect(find.text('openmuse-io'), findsOneWidget);
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
    expect(find.byKey(const ValueKey('open-remote-dsh')), findsOneWidget);
    expect(find.text('打开 Agent'), findsOneWidget);
    expect(find.textContaining('dsh.example.test'), findsNothing);
    expect(find.textContaining('/session/s1'), findsNothing);
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

  testWidgets('authorized XLSX catalog route remains view only', (
    tester,
  ) async {
    final service = _FakeCloudService(xlsx: true);
    final engine = MultiFormatOfficeEngine({
      OfficeFormat.word: _FakeOfficeEngine(),
      OfficeFormat.sheet: _FakeSheetEngine(),
    });
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: mobileComposition(
          session: const MobileAccountSession.authenticated('Test Account'),
          cloudService: service,
          dshConnector: service,
          resources: service,
          resourceCatalog: service,
          officeCommits: service,
          officeEngine: engine,
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Cloud Project'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Budget.xlsx'));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('office-view-only')), findsOneWidget);
    expect(find.textContaining('Sheet A'), findsOneWidget);
    expect(find.byIcon(Icons.save), findsNothing);
  });

  testWidgets('authorized PPTX catalog route remains view only', (
    tester,
  ) async {
    final service = _FakeCloudService(pptx: true);
    final viewers = _FakeSlidesEngine();
    final engine = MultiFormatOfficeEngine({
      OfficeFormat.word: _FakeOfficeEngine(),
      OfficeFormat.slides: viewers,
    });
    await tester.pumpWidget(
      OpenMuseHostShell(
        composition: mobileComposition(
          session: const MobileAccountSession.authenticated('Test Account'),
          cloudService: service,
          dshConnector: service,
          resources: service,
          resourceCatalog: service,
          officeCommits: service,
          officeEngine: engine,
        ),
      ),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.text('Cloud Project'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Deck.pptx'));
    await tester.pumpAndSettle();
    expect(find.text('PPTX · 只读兼容视图'), findsOneWidget);
    expect(find.textContaining('Slide 1'), findsOneWidget);
    expect(find.byIcon(Icons.save), findsNothing);
  });

  testWidgets(
    'authorized PDF catalog route is a text-only compatibility view',
    (tester) async {
      final service = _FakeCloudService(pdf: true);
      final engine = MultiFormatOfficeEngine({
        OfficeFormat.pdf: _FakePdfEngine(),
      });
      await tester.pumpWidget(
        OpenMuseHostShell(
          composition: mobileComposition(
            session: const MobileAccountSession.authenticated('Test Account'),
            cloudService: service,
            dshConnector: service,
            resources: service,
            resourceCatalog: service,
            officeEngine: engine,
          ),
        ),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('Cloud Project'));
      await tester.pumpAndSettle();
      await tester.tap(find.text('Notes.pdf'));
      await tester.pumpAndSettle();
      expect(find.text('PDF · 文本兼容只读视图'), findsOneWidget);
      expect(find.textContaining('Page 1'), findsOneWidget);
      expect(find.byIcon(Icons.save), findsNothing);
    },
  );
}

final class _FakeAuthenticationController extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  int restoreCalls = 0;

  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot.signedOut();

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;

  @override
  Future<void> restore() async {
    restoreCalls++;
  }

  @override
  Future<void> signInWithPassword(String email, String password) async {}

  @override
  Future<void> signOut() async {}
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

final class _FakeSheetEngine implements OfficeEnginePort {
  @override
  String get abi => 'openmuse-office-viewers-ffi@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => const OfficeEngineInspection(
    format: OfficeFormat.sheet,
    profile: 'view-only',
    paragraphs: ['Sheet A\t42'],
    capabilities: {OfficeCapability.view},
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw UnsupportedError('view only');
}

final class _FakeSlidesEngine implements OfficeEnginePort {
  @override
  String get abi => 'openmuse-office-viewers-ffi@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => const OfficeEngineInspection(
    format: OfficeFormat.slides,
    profile: 'view-only',
    paragraphs: ['Slide 1\tHello'],
    capabilities: {OfficeCapability.view},
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw UnsupportedError('view only');
}

final class _FakePdfEngine implements OfficeEnginePort {
  @override
  String get abi => 'openmuse-office-viewers-ffi@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => const OfficeEngineInspection(
    format: OfficeFormat.pdf,
    profile: 'text-view-only',
    paragraphs: ['Page 1\tHello PDF'],
    capabilities: {OfficeCapability.view},
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw UnsupportedError('view only');
}

final class _FakeCloudService
    implements
        CloudWorkspaceService,
        CloudResourceCatalogPort,
        DshRuntimeConnector,
        ResourceRangePort,
        OfficeResourceCommitPort {
  _FakeCloudService({this.xlsx = false, this.pptx = false, this.pdf = false});
  final bool xlsx;
  final bool pptx;
  final bool pdf;

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
  }) async => [
    if (!xlsx && !pptx && !pdf)
      const CloudResourceRecord(
        resourceRef: 'resource:docx',
        title: 'Document.docx',
        revision: 'docx-r1',
        size: 3,
        mediaType:
            'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
        writable: true,
      ),
    if (xlsx)
      const CloudResourceRecord(
        resourceRef: 'resource:xlsx',
        title: 'Budget.xlsx',
        revision: 'xlsx-r1',
        size: 3,
        mediaType:
            'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
        writable: false,
      ),
    if (pptx)
      const CloudResourceRecord(
        resourceRef: 'resource:pptx',
        title: 'Deck.pptx',
        revision: 'pptx-r1',
        size: 3,
        mediaType:
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
        writable: false,
      ),
    if (pdf)
      const CloudResourceRecord(
        resourceRef: 'resource:pdf',
        title: 'Notes.pdf',
        revision: 'pdf-r1',
        size: 3,
        mediaType: 'application/pdf',
        writable: false,
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
    mediaType: switch (resourceRef) {
      'resource:xlsx' =>
        'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      'resource:pptx' =>
        'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      'resource:pdf' => 'application/pdf',
      _ =>
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    },
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
