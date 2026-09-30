import 'dart:async';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';
import 'package:openmuse_office_viewers/openmuse_office_viewers.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'docx_editor_screen.dart';
import 'office_viewer_screen.dart';
import 'remote_dsh_page.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final officeEngine = _loadPackagedOfficeEngine();
  final endpoints = MobileEndpointConfig.fromEnvironment();
  const secureValues = FlutterSecureValueStore();
  final deviceId = await _persistentMobileDeviceId(secureValues);
  final authentication = GoTrueAuthenticationController(
    provider: GoTrueHttpClient(
      config: GoTrueClientConfig(
        origin: endpoints.gotrueOrigin,
        allowInsecureLoopback: endpoints.allowInsecureLoopback,
      ),
    ),
    store: const SecureAuthSessionStore(values: secureValues),
  );
  final authPlugin = OpenMuseGoTruePlugin(
    authentication: authentication,
    cloudLabel: endpoints.cloudOrigin.toString(),
  );
  final cloudPlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authentication,
    cloudOrigin: endpoints.cloudOrigin,
    deviceId: deviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
  );
  final deviceDirectory = AccountDeviceDirectoryController(
    authentication: authentication,
    client: AccountDeviceDirectoryClient(
      cloudOrigin: endpoints.cloudOrigin,
      accessToken: authentication.accessToken,
      allowInsecureLoopback: endpoints.allowInsecureLoopback,
    ),
    registration: () => AccountDeviceRegistration(
      deviceRef: deviceId,
      displayName: 'OpenMuse Mobile',
      platform: defaultTargetPlatform.name,
      kind: AccountDeviceKind.mobile,
      capabilities: const {'workspace.cloud', 'paired-desktop.client'},
    ),
  );
  final pairedPlugin = OpenMusePairedDesktopMobilePlugin.discovered(
    directory: deviceDirectory,
    accessToken: authentication.accessToken,
    deviceRef: deviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
    allowInsecurePrivateNetworkForTesting:
        endpoints.allowInsecurePrivateNetworkForTesting,
  );
  runApp(
    OpenMuseMobileApplication(
      authenticationPlugin: authPlugin,
      cloudWorkspacePlugin: cloudPlugin,
      pairedDesktopPlugin: pairedPlugin,
      officeEngine: officeEngine,
    ),
  );
}

Future<String> _persistentMobileDeviceId(SecureValueStore values) async {
  const key = 'openmuse.device.id.v1';
  final existing = await values.read(key);
  if (existing != null &&
      existing.startsWith('mobile.') &&
      existing.length <= 160) {
    return existing;
  }
  final random = Random.secure();
  final suffix = List.generate(
    20,
    (_) => random.nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();
  final value = 'mobile.$suffix';
  await values.write(key, value);
  return value;
}

@immutable
final class MobileEndpointConfig {
  const MobileEndpointConfig({
    required this.gotrueOrigin,
    required this.cloudOrigin,
    required this.allowInsecureLoopback,
    required this.pairedDesktopOrigin,
    required this.allowInsecurePrivateNetworkForTesting,
  });

  factory MobileEndpointConfig.fromEnvironment() => MobileEndpointConfig(
    gotrueOrigin: Uri.parse(
      const String.fromEnvironment(
        'OPENMUSE_GOTRUE_ORIGIN',
        defaultValue: 'http://127.0.0.1:9999',
      ),
    ),
    cloudOrigin: Uri.parse(
      const String.fromEnvironment(
        'OPENMUSE_CLOUD_ORIGIN',
        defaultValue: 'http://127.0.0.1:8000',
      ),
    ),
    allowInsecureLoopback: const bool.fromEnvironment(
      'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
      defaultValue: !kReleaseMode,
    ),
    pairedDesktopOrigin: Uri.parse(
      const String.fromEnvironment(
        'OPENMUSE_PAIRED_DESKTOP_ORIGIN',
        defaultValue: 'http://127.0.0.1:13180',
      ),
    ),
    allowInsecurePrivateNetworkForTesting:
        !kReleaseMode &&
        const bool.fromEnvironment(
          'OPENMUSE_ALLOW_INSECURE_PRIVATE_NETWORK_FOR_TESTING',
          defaultValue: false,
        ),
  );

  final Uri gotrueOrigin;
  final Uri cloudOrigin;
  final bool allowInsecureLoopback;
  final Uri pairedDesktopOrigin;
  final bool allowInsecurePrivateNetworkForTesting;
}

final class OpenMuseMobileApplication extends StatefulWidget {
  const OpenMuseMobileApplication({
    super.key,
    required this.authenticationPlugin,
    required this.cloudWorkspacePlugin,
    required this.pairedDesktopPlugin,
    this.officeEngine,
  });

  final OpenMuseGoTruePlugin authenticationPlugin;
  final OpenMuseCloudWorkspacePlugin cloudWorkspacePlugin;
  final OpenMusePairedDesktopMobilePlugin pairedDesktopPlugin;
  final OfficeEnginePort? officeEngine;

  @override
  State<OpenMuseMobileApplication> createState() =>
      _OpenMuseMobileApplicationState();
}

final class _OpenMuseMobileApplicationState
    extends State<OpenMuseMobileApplication> {
  late final OpenMusePluginRegistry _plugins;

  @override
  void initState() {
    super.initState();
    _plugins =
        OpenMusePluginRegistry(
            context: OpenMusePluginContext(
              executeHostCommand: (_, _) async => null,
            ),
          )
          ..install(widget.authenticationPlugin)
          ..install(widget.cloudWorkspacePlugin)
          ..install(widget.pairedDesktopPlugin);
    unawaited(() async {
      await _plugins.activate(widget.authenticationPlugin.descriptor.id);
      await _plugins.activate(widget.cloudWorkspacePlugin.descriptor.id);
      await _plugins.activate(widget.pairedDesktopPlugin.descriptor.id);
    }());
  }

  @override
  void dispose() {
    unawaited(
      Future.wait([
        _plugins.deactivate(widget.cloudWorkspacePlugin.descriptor.id),
        _plugins.deactivate(widget.pairedDesktopPlugin.descriptor.id),
        _plugins.deactivate(widget.authenticationPlugin.descriptor.id),
      ]).whenComplete(_plugins.dispose),
    );
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'OpenMuse',
    home: Builder(
      builder: (context) => widget.authenticationPlugin.buildAuthenticationGate(
        context,
        authenticatedChild: _AuthenticatedMobileHost(
          authentication: widget.authenticationPlugin.authentication,
          cloudService: widget.cloudWorkspacePlugin.service,
          pairedDesktop: widget.pairedDesktopPlugin.controller,
          officeEngine: widget.officeEngine,
        ),
      ),
    ),
  );
}

final class _AuthenticatedMobileHost extends StatefulWidget {
  const _AuthenticatedMobileHost({
    required this.authentication,
    required this.cloudService,
    required this.pairedDesktop,
    this.officeEngine,
  });

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService cloudService;
  final PairedDesktopMobileController pairedDesktop;
  final OfficeEnginePort? officeEngine;

  @override
  State<_AuthenticatedMobileHost> createState() =>
      _AuthenticatedMobileHostState();
}

final class _AuthenticatedMobileHostState
    extends State<_AuthenticatedMobileHost> {
  late OpenMuseHostComposition _composition;
  String? _pairedGrantRef;

  @override
  void initState() {
    super.initState();
    _recompose();
    widget.pairedDesktop.addListener(_pairedChanged);
  }

  @override
  void didUpdateWidget(covariant _AuthenticatedMobileHost oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.pairedDesktop != widget.pairedDesktop) {
      oldWidget.pairedDesktop.removeListener(_pairedChanged);
      widget.pairedDesktop.addListener(_pairedChanged);
    }
    if (oldWidget.authentication != widget.authentication ||
        oldWidget.cloudService != widget.cloudService ||
        oldWidget.pairedDesktop != widget.pairedDesktop ||
        oldWidget.officeEngine != widget.officeEngine) {
      _recompose();
    }
  }

  void _pairedChanged() {
    final nextGrantRef = widget.pairedDesktop.snapshot.connection?.grantRef;
    if (!mounted || nextGrantRef == _pairedGrantRef) return;
    setState(_recompose);
  }

  void _recompose() {
    final identity = widget.authentication.snapshot.identity;
    _pairedGrantRef = widget.pairedDesktop.snapshot.connection?.grantRef;
    _composition = connectedCloudMobileComposition(
      session: identity == null
          ? const MobileAccountSession.signedOut()
          : MobileAccountSession.authenticated(identity.email),
      apiOrigin: widget.cloudService.baseUri,
      accessToken: () => widget.authentication.accessToken(),
      refreshAccessToken: () =>
          widget.authentication.accessToken(forceRefresh: true),
      service: widget.cloudService,
      officeEngine: widget.officeEngine,
      pairedDesktop: widget.pairedDesktop,
      onSignOut: widget.authentication.signOut,
    );
  }

  @override
  void dispose() {
    widget.pairedDesktop.removeListener(_pairedChanged);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final identity = widget.authentication.snapshot.identity;
    if (identity == null) return const SizedBox.shrink();
    return OpenMuseHostShell(
      // Directory heartbeats notify the paired controller every 20 seconds.
      // Keep the workspace request stable across those rebuilds, while a new
      // pairing grant deliberately creates a fresh shell and catalog snapshot.
      key: ValueKey(_pairedGrantRef ?? 'unpaired'),
      composition: _composition,
    );
  }
}

OfficeEnginePort? _loadPackagedOfficeEngine() {
  final engines = <OfficeFormat, OfficeEnginePort>{};
  try {
    engines[OfficeFormat.word] = DocxFfiEngine.open();
  } on Object {
    // A missing or incompatible artifact removes only this capability.
  }
  try {
    final viewers = OfficeViewersFfiEngine.open();
    engines[OfficeFormat.sheet] = viewers;
    engines[OfficeFormat.slides] = viewers;
    engines[OfficeFormat.pdf] = viewers;
  } on Object {
    // A missing or incompatible artifact removes only this capability.
  }
  if (engines.isEmpty) return null;
  if (engines.length == 1) return engines.values.single;
  return MultiFormatOfficeEngine(engines);
}

/// Composition entry used by the login/bootstrap layer after it has obtained
/// an account session and a short-lived access-token provider. No credential is
/// compiled into the Mobile artifact or stored by the Cloud adapter.
OpenMuseHostComposition connectedCloudMobileComposition({
  required OpenMuseSessionPort session,
  required Uri apiOrigin,
  required AccessTokenProvider accessToken,
  RefreshAccessTokenProvider? refreshAccessToken,
  String deviceId = 'mobile.openmuse',
  bool allowHttpForTesting = false,
  AppFlowyCloudWorkspaceService? service,
  OfficeEnginePort? officeEngine,
  PairedDesktopMobileController? pairedDesktop,
  Future<void> Function()? onSignOut,
}) {
  final cloud =
      service ??
      AppFlowyCloudWorkspaceService(
        baseUri: apiOrigin,
        accessToken: accessToken,
        refreshAccessToken: refreshAccessToken,
        deviceId: deviceId,
        allowHttpForTesting: allowHttpForTesting,
      );
  return mobileComposition(
    session: session,
    cloudService: cloud,
    dshConnector: cloud,
    resources: cloud,
    resourceCatalog: cloud,
    officeCommits: cloud,
    officeEngine: officeEngine ?? _loadPackagedOfficeEngine(),
    pairedDesktop: pairedDesktop,
    onSignOut: onSignOut,
  );
}

OpenMuseHostComposition mobileComposition({
  OpenMuseSessionPort session = const MobileAccountSession.signedOut(),
  CloudWorkspaceService? cloudService,
  DshRuntimeConnector? dshConnector,
  ResourceRangePort? resources,
  CloudResourceCatalogPort? resourceCatalog,
  OfficeResourceCommitPort? officeCommits,
  OfficeEnginePort? officeEngine,
  PairedDesktopMobileController? pairedDesktop,
  Future<void> Function()? onSignOut,
}) {
  final catalog = _MobileCatalogAdapter(cloudService, pairedDesktop);
  return OpenMuseHostComposition(
    platform: OpenMuseHostPlatform.mobile,
    session: session,
    workspaceCatalog: catalog,
    capabilitySnapshot: _MobileCapabilities(
      cloudService != null,
      pairedDesktop != null,
      _supportsFormat(officeEngine, OfficeFormat.word),
      _supportsFormat(officeEngine, OfficeFormat.sheet),
      _supportsFormat(officeEngine, OfficeFormat.slides),
      _supportsFormat(officeEngine, OfficeFormat.pdf),
    ),
    workspaceBuilder: (context, workspace) {
      if (workspace.placement == WorkspacePlacement.pairedDesktop) {
        final controller = pairedDesktop;
        final connection = controller?.snapshot.connection;
        if (controller == null) {
          return const Scaffold(
            body: Center(child: Text('Paired Desktop 插件未加载')),
          );
        }
        if (connection == null) {
          return PairedDesktopConnectScreen(controller: controller);
        }
        return PairedDesktopWorkspaceScreen(connection: connection);
      }
      final record = catalog.cloudRecord(workspace.workspaceRef);
      if (record == null ||
          cloudService == null ||
          dshConnector == null ||
          resources == null) {
        return Scaffold(
          appBar: AppBar(title: Text(workspace.title)),
          body: const Center(child: Text('Cloud Workspace 服务未连接')),
        );
      }
      return CloudWorkspaceScreen(
        record: record,
        service: cloudService,
        connector: dshConnector,
        resources: resources,
        resourceCatalog: resourceCatalog,
        officeEngine: officeEngine,
        officeCommits: officeCommits,
      );
    },
    accountDevicesBuilder: pairedDesktop?.directory == null
        ? null
        : (_) => AccountDevicesScreen(controller: pairedDesktop!),
    onSignOut: onSignOut,
  );
}

final class MobileAccountSession implements OpenMuseSessionPort {
  const MobileAccountSession.authenticated(this.accountLabel) : signedIn = true;
  const MobileAccountSession.signedOut()
    : signedIn = false,
      accountLabel = null;
  @override
  final bool signedIn;
  @override
  final String? accountLabel;
}

final class _MobileCatalogAdapter implements WorkspaceCatalogPort {
  _MobileCatalogAdapter(this.service, this.pairedDesktop);
  final CloudWorkspaceService? service;
  final PairedDesktopMobileController? pairedDesktop;
  final Map<String, CloudWorkspaceRecord> _records = {};

  CloudWorkspaceRecord? cloudRecord(String workspaceRef) =>
      _records[workspaceRef];

  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async {
    final provider = service;
    final values = provider == null
        ? const <CloudWorkspaceRecord>[]
        : await provider.listWorkspaces();
    var sessions = const <DshSessionSummary>[];
    if (provider is DshSessionCatalogPort) {
      try {
        sessions = await (provider as DshSessionCatalogPort).listSessions();
      } on Object {
        // Session presence enriches the catalog but does not own it. A DSH
        // pool outage must not hide otherwise usable Cloud workspaces.
      }
    }
    final runningByWorkspace = {
      for (final session in sessions.where((value) => value.isRunning))
        session.workspaceRef: session.sessionRef,
    };
    _records
      ..clear()
      ..addEntries(values.map((value) => MapEntry(value.workspaceRef, value)));
    final result = values
        .map(
          (value) => WorkspaceSummary(
            workspaceRef: value.workspaceRef,
            title: value.title,
            placement: WorkspacePlacement.cloud,
            writable:
                value.writable &&
                value.storageState == CloudStorageState.available,
            runningSessionRef: runningByWorkspace[value.workspaceRef],
          ),
        )
        .toList(growable: true);
    final paired = pairedDesktop;
    if (paired != null) {
      final connection = paired.snapshot.connection;
      result.add(
        WorkspaceSummary(
          workspaceRef: connection?.workspaceRef ?? 'openmuse.paired.connect',
          title: connection?.workspaceTitle ?? '连接本地 Desktop',
          placement: WorkspacePlacement.pairedDesktop,
          writable: false,
          runningSessionRef: connection?.session.sessionRef,
        ),
      );
    }
    return result;
  }
}

final class AccountDevicesScreen extends StatelessWidget {
  const AccountDevicesScreen({super.key, required this.controller});

  final PairedDesktopMobileController controller;

  @override
  Widget build(BuildContext context) {
    final directory = controller.directory;
    return Scaffold(
      appBar: AppBar(title: const Text('账号设备')),
      body: directory == null
          ? const Center(child: Text('设备目录插件未加载'))
          : ListView(
              padding: const EdgeInsets.all(20),
              children: [
                const Text('同一账号下的设备', style: TextStyle(fontSize: 20)),
                const SizedBox(height: 6),
                const Text('在线 Desktop 可以发起配对；离线设备仅供识别，不能连接。'),
                const SizedBox(height: 16),
                AccountDeviceList(
                  controller: directory,
                  excludeDeviceRef: controller.requesterDeviceRef,
                  onSelect: (device) => Navigator.of(context).push<void>(
                    MaterialPageRoute(
                      builder: (_) => PairedDesktopConnectScreen(
                        controller: controller,
                        initialDevice: device,
                      ),
                    ),
                  ),
                ),
              ],
            ),
    );
  }
}

final class PairedDesktopConnectScreen extends StatefulWidget {
  const PairedDesktopConnectScreen({
    super.key,
    required this.controller,
    this.initialDevice,
  });

  final PairedDesktopMobileController controller;
  final AccountDevice? initialDevice;

  @override
  State<PairedDesktopConnectScreen> createState() =>
      _PairedDesktopConnectScreenState();
}

final class _PairedDesktopConnectScreenState
    extends State<PairedDesktopConnectScreen> {
  final TextEditingController _code = TextEditingController();
  AccountDevice? _selectedDevice;

  @override
  void initState() {
    super.initState();
    _selectedDevice = widget.initialDevice;
  }

  @override
  void dispose() {
    _code.dispose();
    super.dispose();
  }

  Future<void> _pair() async {
    final target = _selectedDevice;
    if (target == null) return;
    final connected = await widget.controller.pairDevice(target, _code.text);
    if (connected && mounted) Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: const Text('连接本地 Desktop')),
    body: ListenableBuilder(
      listenable: widget.controller,
      builder: (context, _) {
        final snapshot = widget.controller.snapshot;
        return ListView(
          padding: const EdgeInsets.all(24),
          children: [
            const Icon(Icons.phonelink, size: 48),
            const SizedBox(height: 18),
            const Text(
              '先选择同账号下在线的 Desktop，再输入该 Desktop 显示的配对码。授权只覆盖当前本地 Workspace。',
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 18),
            if (widget.controller.directory case final directory?)
              AccountDeviceList(
                controller: directory,
                excludeDeviceRef: widget.controller.requesterDeviceRef,
                selectedDeviceRef: _selectedDevice?.deviceRef,
                onSelect: (device) => setState(() => _selectedDevice = device),
              ),
            const SizedBox(height: 24),
            TextField(
              key: const ValueKey('paired-desktop-code'),
              controller: _code,
              keyboardType: TextInputType.number,
              maxLength: 6,
              textAlign: TextAlign.center,
              decoration: const InputDecoration(
                labelText: '6 位配对码',
                border: OutlineInputBorder(),
              ),
              onChanged: (_) => setState(() {}),
              enabled: _selectedDevice?.online == true,
              onSubmitted: snapshot.connecting ? null : (_) => _pair(),
            ),
            const SizedBox(height: 12),
            FilledButton.icon(
              key: const ValueKey('paired-desktop-connect'),
              onPressed:
                  snapshot.connecting ||
                      _selectedDevice?.online != true ||
                      _code.text.trim().length != 6
                  ? null
                  : _pair,
              icon: snapshot.connecting
                  ? const SizedBox(
                      width: 18,
                      height: 18,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    )
                  : const Icon(Icons.link),
              label: Text(
                _selectedDevice == null
                    ? '请选择在线 Desktop'
                    : '连接 ${_selectedDevice!.displayName}',
              ),
            ),
            if (snapshot.failureMessage case final message?) ...[
              const SizedBox(height: 16),
              Text(
                message,
                key: const ValueKey('paired-desktop-error'),
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ],
          ],
        );
      },
    ),
  );
}

final class PairedDesktopWorkspaceScreen extends StatelessWidget {
  const PairedDesktopWorkspaceScreen({super.key, required this.connection});

  final PairedDesktopConnection connection;

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(connection.workspaceTitle)),
    body: ListView(
      padding: const EdgeInsets.all(20),
      children: [
        const Text(
          'Paired Desktop · Local Workspace',
          key: ValueKey('paired-workspace-placement'),
        ),
        const SizedBox(height: 8),
        const Text('账号 · 已验证'),
        const Text('Workspace Grant · 已授权'),
        const Text('Desktop DSH · 已连接'),
        const SizedBox(height: 12),
        FilledButton.icon(
          key: const ValueKey('open-paired-desktop-dsh'),
          onPressed: () => Navigator.of(context).push<void>(
            MaterialPageRoute(
              builder: (_) => RemoteDshPage(
                session: connection.session,
                workspaceTitle: connection.workspaceTitle,
              ),
            ),
          ),
          icon: const Icon(Icons.computer_outlined),
          label: const Text('打开 Desktop 会话'),
        ),
        const SizedBox(height: 16),
        const Text('会话列表、运行状态、历史和消息均来自 Desktop 正在使用的同一个 DSH runtime。'),
      ],
    ),
  );
}

bool _supportsFormat(OfficeEnginePort? engine, OfficeFormat format) {
  if (engine == null) return false;
  if (engine is MultiFormatOfficeEngine) return engine.formats.contains(format);
  if (engine is DocxFfiEngine) return format == OfficeFormat.word;
  if (engine is OfficeViewersFfiEngine) {
    return format == OfficeFormat.sheet ||
        format == OfficeFormat.slides ||
        format == OfficeFormat.pdf;
  }
  return format == OfficeFormat.word;
}

final class _MobileCapabilities implements CapabilitySnapshotPort {
  const _MobileCapabilities(
    this.cloudConnected,
    this.pairedDesktopConnected,
    this.docxEngineConnected,
    this.xlsxEngineConnected,
    this.pptxEngineConnected,
    this.pdfEngineConnected,
  );
  final bool cloudConnected;
  final bool pairedDesktopConnected;
  final bool docxEngineConnected;
  final bool xlsxEngineConnected;
  final bool pptxEngineConnected;
  final bool pdfEngineConnected;
  @override
  Set<String> get capabilities => {
    'resource.viewer',
    if (cloudConnected) 'workspace.cloud',
    if (cloudConnected) 'dsh.remote',
    if (pairedDesktopConnected) 'workspace.paired',
    if (pairedDesktopConnected) 'dsh.paired',
    if (docxEngineConnected) 'office.docx.engine',
    if (xlsxEngineConnected) 'office.xlsx.engine',
    if (pptxEngineConnected) 'office.pptx.engine',
    if (pdfEngineConnected) 'office.pdf.engine',
  };
}

final class CloudWorkspaceScreen extends StatefulWidget {
  const CloudWorkspaceScreen({
    super.key,
    required this.record,
    required this.service,
    required this.connector,
    required this.resources,
    this.resourceCatalog,
    this.officeEngine,
    this.officeCommits,
  });
  final CloudWorkspaceRecord record;
  final CloudWorkspaceService service;
  final DshRuntimeConnector connector;
  final ResourceRangePort resources;
  final CloudResourceCatalogPort? resourceCatalog;
  final OfficeEnginePort? officeEngine;
  final OfficeResourceCommitPort? officeCommits;

  @override
  State<CloudWorkspaceScreen> createState() => _CloudWorkspaceScreenState();
}

final class _CloudWorkspaceScreenState extends State<CloudWorkspaceScreen> {
  late final CloudWorkspaceCoordinator coordinator;
  late final Future<List<CloudResourceRecord>> opening;

  @override
  void initState() {
    super.initState();
    coordinator = CloudWorkspaceCoordinator(
      service: widget.service,
      connector: widget.connector,
      resources: widget.resources,
    );
    opening = coordinator.select(widget.record).then((_) {
      final catalog = widget.resourceCatalog;
      if (catalog == null) return const <CloudResourceRecord>[];
      return catalog.listResources(
        workspaceRef: widget.record.workspaceRef,
        revision: widget.record.revision,
        generation: coordinator.flow.generation,
      );
    });
  }

  bool _isDocx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.wordprocessingml.document' ||
      value.mediaType == 'application/docx';

  bool _isXlsx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' ||
      value.mediaType == 'application/xlsx';

  bool _isPptx(CloudResourceRecord value) =>
      value.mediaType ==
          'application/vnd.openxmlformats-officedocument.presentationml.presentation' ||
      value.mediaType == 'application/pptx';

  bool _isPdf(CloudResourceRecord value) =>
      value.mediaType == 'application/pdf';

  Future<void> _openDocx(CloudResourceRecord resource) async {
    final engine = widget.officeEngine;
    final commits = widget.officeCommits;
    if (engine == null || commits == null || !_isDocx(resource)) return;
    try {
      final generation = coordinator.flow.generation;
      final handle = await widget.service.issueResourceHandle(
        workspaceRef: widget.record.workspaceRef,
        resourceRef: resource.resourceRef,
        revision: resource.revision,
        audience: 'openmuse-mobile-office',
        generation: generation,
      );
      if (!mounted ||
          handle.resourceRef != resource.resourceRef ||
          handle.revision != resource.revision ||
          handle.generation != generation) {
        throw StateError('late or cross-resource Office handle');
      }
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => DocxEditorScreen(
            title: resource.title,
            handle: handle,
            engine: engine,
            ranges: widget.resources,
            commits: commits,
            generation: generation,
            nowMs: () => DateTime.now().millisecondsSinceEpoch,
          ),
        ),
      );
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('DOCX 打开失败')));
      }
    }
  }

  Future<void> _openViewer(
    CloudResourceRecord resource,
    OfficeFormat format,
  ) async {
    final engine = widget.officeEngine;
    if (engine == null ||
        (format == OfficeFormat.sheet && !_isXlsx(resource)) ||
        (format == OfficeFormat.slides && !_isPptx(resource)) ||
        (format == OfficeFormat.pdf && !_isPdf(resource))) {
      return;
    }
    try {
      final generation = coordinator.flow.generation;
      final handle = await widget.service.issueResourceHandle(
        workspaceRef: widget.record.workspaceRef,
        resourceRef: resource.resourceRef,
        revision: resource.revision,
        audience: 'openmuse-mobile-office',
        generation: generation,
      );
      if (!mounted ||
          handle.resourceRef != resource.resourceRef ||
          handle.revision != resource.revision ||
          handle.generation != generation) {
        throw StateError('late or cross-resource Office handle');
      }
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => OfficeViewerScreen(
            title: resource.title,
            format: format,
            handle: handle,
            engine: engine,
            ranges: widget.resources,
            generation: generation,
            nowMs: () => DateTime.now().millisecondsSinceEpoch,
          ),
        ),
      );
    } on Object {
      if (mounted) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('Office 文件打开失败')));
      }
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.record.title)),
    body: FutureBuilder<List<CloudResourceRecord>>(
      future: opening,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return const Center(child: Text('Cloud Workspace · Degraded'));
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(child: Text('Cloud Workspace · Queued'));
        }
        final session = coordinator.presentation.session;
        final resources = snapshot.data ?? const <CloudResourceRecord>[];
        return ListView(
          padding: const EdgeInsets.all(20),
          children: [
            const Text(
              'Cloud Workspace · Remote DSH',
              key: ValueKey('workspace-placement'),
            ),
            const SizedBox(height: 8),
            Text('Revision ${coordinator.flow.revision}'),
            Text(
              widget.record.storageState == CloudStorageState.available
                  ? (widget.record.writable ? 'Storage · 可写' : 'Storage · 只读')
                  : 'Storage · Unavailable',
            ),
            Text('DSH · ${coordinator.flow.state.name}'),
            if (session != null)
              Text(
                'Remote DSH · 已连接',
                key: const ValueKey('remote-dsh-session'),
              ),
            if (session != null) ...[
              const SizedBox(height: 12),
              FilledButton.icon(
                key: const ValueKey('open-remote-dsh'),
                onPressed: () => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) => RemoteDshPage(
                      session: session,
                      workspaceTitle: widget.record.title,
                    ),
                  ),
                ),
                icon: const Icon(Icons.auto_awesome_outlined),
                label: const Text('打开 Agent'),
              ),
            ],
            if (resources.isNotEmpty) const Divider(),
            for (final resource in resources)
              ListTile(
                key: ValueKey('resource-${resource.resourceRef}'),
                title: Text(resource.title),
                subtitle: Text(resource.mediaType),
                enabled:
                    (_isDocx(resource) &&
                        widget.officeEngine != null &&
                        widget.officeCommits != null) ||
                    (_isXlsx(resource) && widget.officeEngine != null) ||
                    (_isPptx(resource) && widget.officeEngine != null) ||
                    (_isPdf(resource) && widget.officeEngine != null),
                onTap: () {
                  if (_isXlsx(resource)) {
                    _openViewer(resource, OfficeFormat.sheet);
                  } else if (_isPptx(resource)) {
                    _openViewer(resource, OfficeFormat.slides);
                  } else if (_isPdf(resource)) {
                    _openViewer(resource, OfficeFormat.pdf);
                  } else {
                    _openDocx(resource);
                  }
                },
              ),
          ],
        );
      },
    ),
  );
}
