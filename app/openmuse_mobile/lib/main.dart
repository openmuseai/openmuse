import 'package:flutter/material.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';

void main() => runApp(OpenMuseHostShell(composition: mobileComposition()));

OpenMuseHostComposition mobileComposition() => OpenMuseHostComposition(
  platform: OpenMuseHostPlatform.mobile,
  session: const _DevelopmentSession(),
  workspaceCatalog: const _FixtureWorkspaceCatalog(),
  capabilitySnapshot: const _MobileCapabilities(),
  workspaceBuilder: (context, workspace) => Scaffold(
    appBar: AppBar(title: Text(workspace.title)),
    body: Center(
      child: Text(
        workspace.placement == WorkspacePlacement.cloud
            ? 'Cloud Workspace · Remote DSH'
            : 'Paired Desktop · 等待设备连接',
        key: const ValueKey('workspace-placement'),
      ),
    ),
  ),
);

final class _DevelopmentSession implements OpenMuseSessionPort {
  const _DevelopmentSession();
  @override
  bool get signedIn => true;
  @override
  String get accountLabel => 'OpenMuse Account';
}

final class _FixtureWorkspaceCatalog implements WorkspaceCatalogPort {
  const _FixtureWorkspaceCatalog();
  @override
  Future<List<WorkspaceSummary>> listWorkspaces() async => const [
    WorkspaceSummary(
      workspaceRef: 'cloud:welcome',
      title: 'OpenMuse Cloud',
      placement: WorkspacePlacement.cloud,
      writable: true,
    ),
  ];
}

final class _MobileCapabilities implements CapabilitySnapshotPort {
  const _MobileCapabilities();
  @override
  Set<String> get capabilities => const {
    'workspace.cloud',
    'dsh.remote',
    'resource.viewer',
  };
}
