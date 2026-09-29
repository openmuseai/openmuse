import 'package:flutter/material.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_office_docx/openmuse_office_docx.dart';

void main() {
  final docxEngine = _loadPackagedDocxEngine();
  runApp(
    OpenMuseHostShell(composition: mobileComposition(officeEngine: docxEngine)),
  );
}

OfficeEnginePort? _loadPackagedDocxEngine() {
  try {
    return DocxFfiEngine.open();
  } on Object {
    // Packaging and ABI mismatches fail closed: no Office capability is
    // advertised and the rest of the Mobile Host remains usable.
    return null;
  }
}

/// Composition entry used by the login/bootstrap layer after it has obtained
/// an account session and a short-lived access-token provider. No credential is
/// compiled into the Mobile artifact or stored by the Cloud adapter.
OpenMuseHostComposition connectedCloudMobileComposition({
  required OpenMuseSessionPort session,
  required Uri apiOrigin,
  required AccessTokenProvider accessToken,
  OfficeEnginePort? officeEngine,
}) {
  final service = HttpCloudWorkspaceService(
    baseUri: apiOrigin,
    accessToken: accessToken,
  );
  return mobileComposition(
    session: session,
    cloudService: service,
    dshConnector: service,
    resources: service,
    officeEngine: officeEngine ?? _loadPackagedDocxEngine(),
  );
}

OpenMuseHostComposition mobileComposition({
  OpenMuseSessionPort session = const MobileAccountSession.signedOut(),
  CloudWorkspaceService? cloudService,
  DshRuntimeConnector? dshConnector,
  ResourceRangePort? resources,
  OfficeEnginePort? officeEngine,
}) {
  final catalog = _CloudCatalogAdapter(cloudService);
  return OpenMuseHostComposition(
    platform: OpenMuseHostPlatform.mobile,
    session: session,
    workspaceCatalog: catalog,
    capabilitySnapshot: _MobileCapabilities(
      cloudService != null,
      officeEngine != null,
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
          ),
        )
        .toList(growable: false);
  }
}

final class _MobileCapabilities implements CapabilitySnapshotPort {
  const _MobileCapabilities(this.cloudConnected, this.docxEngineConnected);
  final bool cloudConnected;
  final bool docxEngineConnected;
  @override
  Set<String> get capabilities => {
    'resource.viewer',
    if (cloudConnected) 'workspace.cloud',
    if (cloudConnected) 'dsh.remote',
    if (docxEngineConnected) 'office.docx.engine',
  };
}

final class CloudWorkspaceScreen extends StatefulWidget {
  const CloudWorkspaceScreen({
    super.key,
    required this.record,
    required this.service,
    required this.connector,
    required this.resources,
  });
  final CloudWorkspaceRecord record;
  final CloudWorkspaceService service;
  final DshRuntimeConnector connector;
  final ResourceRangePort resources;

  @override
  State<CloudWorkspaceScreen> createState() => _CloudWorkspaceScreenState();
}

final class _CloudWorkspaceScreenState extends State<CloudWorkspaceScreen> {
  late final CloudWorkspaceCoordinator coordinator;
  late final Future<void> opening;

  @override
  void initState() {
    super.initState();
    coordinator = CloudWorkspaceCoordinator(
      service: widget.service,
      connector: widget.connector,
      resources: widget.resources,
    );
    opening = coordinator.select(widget.record);
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.record.title)),
    body: FutureBuilder<void>(
      future: opening,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return const Center(child: Text('Cloud Workspace · Degraded'));
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(child: Text('Cloud Workspace · Queued'));
        }
        final session = coordinator.presentation.session;
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
          ],
        );
      },
    ),
  );
}
