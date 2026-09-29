import 'dart:async';

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

import 'docx_editor_screen.dart';
import 'office_viewer_screen.dart';

final String _runtimeMobileDeviceId =
    'mobile.flutter.${DateTime.now().microsecondsSinceEpoch}';

void main() {
  final officeEngine = _loadPackagedOfficeEngine();
  final endpoints = MobileEndpointConfig.fromEnvironment();
  final authentication = GoTrueAuthenticationController(
    provider: GoTrueHttpClient(
      config: GoTrueClientConfig(
        origin: endpoints.gotrueOrigin,
        allowInsecureLoopback: endpoints.allowInsecureLoopback,
      ),
    ),
    store: const SecureAuthSessionStore(values: FlutterSecureValueStore()),
  );
  final authPlugin = OpenMuseGoTruePlugin(
    authentication: authentication,
    cloudLabel: endpoints.cloudOrigin.toString(),
  );
  final cloudPlugin = OpenMuseCloudWorkspacePlugin(
    authentication: authentication,
    cloudOrigin: endpoints.cloudOrigin,
    deviceId: _runtimeMobileDeviceId,
    allowInsecureLoopback: endpoints.allowInsecureLoopback,
  );
  runApp(
    OpenMuseMobileApplication(
      authenticationPlugin: authPlugin,
      cloudWorkspacePlugin: cloudPlugin,
      officeEngine: officeEngine,
    ),
  );
}

@immutable
final class MobileEndpointConfig {
  const MobileEndpointConfig({
    required this.gotrueOrigin,
    required this.cloudOrigin,
    required this.allowInsecureLoopback,
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
  );

  final Uri gotrueOrigin;
  final Uri cloudOrigin;
  final bool allowInsecureLoopback;
}

final class OpenMuseMobileApplication extends StatefulWidget {
  const OpenMuseMobileApplication({
    super.key,
    required this.authenticationPlugin,
    required this.cloudWorkspacePlugin,
    this.officeEngine,
  });

  final OpenMuseGoTruePlugin authenticationPlugin;
  final OpenMuseCloudWorkspacePlugin cloudWorkspacePlugin;
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
          ..install(widget.cloudWorkspacePlugin);
    unawaited(() async {
      await _plugins.activate(widget.authenticationPlugin.descriptor.id);
      await _plugins.activate(widget.cloudWorkspacePlugin.descriptor.id);
    }());
  }

  @override
  void dispose() {
    unawaited(
      Future.wait([
        _plugins.deactivate(widget.cloudWorkspacePlugin.descriptor.id),
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
          officeEngine: widget.officeEngine,
        ),
      ),
    ),
  );
}

final class _AuthenticatedMobileHost extends StatelessWidget {
  const _AuthenticatedMobileHost({
    required this.authentication,
    required this.cloudService,
    this.officeEngine,
  });

  final OpenMuseAuthenticationController authentication;
  final AppFlowyCloudWorkspaceService cloudService;
  final OfficeEnginePort? officeEngine;

  @override
  Widget build(BuildContext context) {
    final identity = authentication.snapshot.identity;
    if (identity == null) return const SizedBox.shrink();
    return OpenMuseHostShell(
      composition: connectedCloudMobileComposition(
        session: MobileAccountSession.authenticated(identity.email),
        apiOrigin: cloudService.baseUri,
        accessToken: () => authentication.accessToken(),
        refreshAccessToken: () =>
            authentication.accessToken(forceRefresh: true),
        service: cloudService,
        officeEngine: officeEngine,
      ),
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
}) {
  final catalog = _CloudCatalogAdapter(cloudService);
  return OpenMuseHostComposition(
    platform: OpenMuseHostPlatform.mobile,
    session: session,
    workspaceCatalog: catalog,
    capabilitySnapshot: _MobileCapabilities(
      cloudService != null,
      _supportsFormat(officeEngine, OfficeFormat.word),
      _supportsFormat(officeEngine, OfficeFormat.sheet),
      _supportsFormat(officeEngine, OfficeFormat.slides),
      _supportsFormat(officeEngine, OfficeFormat.pdf),
    ),
    workspaceBuilder: (context, workspace) {
      final record = catalog.record(workspace.workspaceRef);
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

final class _CloudCatalogAdapter implements WorkspaceCatalogPort {
  _CloudCatalogAdapter(this.service);
  final CloudWorkspaceService? service;
  final Map<String, CloudWorkspaceRecord> _records = {};

  CloudWorkspaceRecord? record(String workspaceRef) => _records[workspaceRef];

  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async {
    final provider = service;
    if (provider == null) return const [];
    final values = await provider.listWorkspaces();
    final sessions = provider is DshSessionCatalogPort
        ? await (provider as DshSessionCatalogPort).listSessions()
        : const <DshSessionSummary>[];
    final runningByWorkspace = {
      for (final session in sessions.where((value) => value.isRunning))
        session.workspaceRef: session.sessionRef,
    };
    _records
      ..clear()
      ..addEntries(values.map((value) => MapEntry(value.workspaceRef, value)));
    return values
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
        .toList(growable: false);
  }
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
    this.docxEngineConnected,
    this.xlsxEngineConnected,
    this.pptxEngineConnected,
    this.pdfEngineConnected,
  );
  final bool cloudConnected;
  final bool docxEngineConnected;
  final bool xlsxEngineConnected;
  final bool pptxEngineConnected;
  final bool pdfEngineConnected;
  @override
  Set<String> get capabilities => {
    'resource.viewer',
    if (cloudConnected) 'workspace.cloud',
    if (cloudConnected) 'dsh.remote',
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
                '${session.origin}${session.path}',
                key: const ValueKey('remote-dsh-session'),
              ),
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
