import 'package:flutter/widgets.dart';

enum OpenMuseHostPlatform { desktop, mobile }

enum WorkspacePlacement { cloud, pairedDesktop }

final class WorkspaceSummary {
  const WorkspaceSummary({
    required this.workspaceRef,
    required this.title,
    required this.placement,
    required this.writable,
  });
  final String workspaceRef;
  final String title;
  final WorkspacePlacement placement;
  final bool writable;
}

abstract interface class OpenMuseSessionPort {
  bool get signedIn;
  String? get accountLabel;
}

abstract interface class WorkspaceCatalogPort {
  Future<List<WorkspaceSummary>> listWorkspaces();
}

abstract interface class CapabilitySnapshotPort {
  Set<String> get capabilities;
}

final class OpenMuseHostComposition {
  const OpenMuseHostComposition({
    required this.platform,
    required this.session,
    required this.workspaceCatalog,
    required this.capabilitySnapshot,
    required this.workspaceBuilder,
  });
  final OpenMuseHostPlatform platform;
  final OpenMuseSessionPort session;
  final WorkspaceCatalogPort workspaceCatalog;
  final CapabilitySnapshotPort capabilitySnapshot;
  final Widget Function(BuildContext, WorkspaceSummary) workspaceBuilder;
}
