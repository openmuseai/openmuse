import 'dsh_connector.dart';
import 'resource_client.dart';
import 'cloud_flow.dart';

enum CloudStorageState { available, readOnly, unavailable }

final class CloudWorkspaceRecord {
  const CloudWorkspaceRecord({
    required this.workspaceRef,
    required this.title,
    required this.revision,
    required this.writable,
    required this.storageState,
  });
  final String workspaceRef;
  final String title;
  final String revision;
  final bool writable;
  final CloudStorageState storageState;
}

final class CloudResourceRecord {
  const CloudResourceRecord({
    required this.resourceRef,
    required this.title,
    required this.revision,
    required this.size,
    required this.mediaType,
    required this.writable,
  });
  final String resourceRef;
  final String title;
  final String revision;
  final int size;
  final String mediaType;
  final bool writable;
}

abstract interface class CloudResourceCatalogPort {
  Future<List<CloudResourceRecord>> listResources({
    required String workspaceRef,
    required String revision,
    required int generation,
  });
}

final class CloudChangeProposal {
  const CloudChangeProposal({
    required this.proposalRef,
    required this.workspaceRef,
    required this.expectedRevision,
    required this.summary,
    required this.generation,
  });
  final String proposalRef;
  final String workspaceRef;
  final String expectedRevision;
  final String summary;
  final int generation;
}

final class CloudApplyReceipt {
  const CloudApplyReceipt({
    required this.receiptRef,
    required this.workspaceRef,
    required this.previousRevision,
    required this.newRevision,
    required this.generation,
  });
  final String receiptRef;
  final String workspaceRef;
  final String previousRevision;
  final String newRevision;
  final int generation;
}

enum CloudServiceErrorCode {
  unauthorized,
  storageUnavailable,
  staleRevision,
  unavailable,
  invalidResponse,
}

final class CloudServiceException implements Exception {
  const CloudServiceException(this.code, this.message);
  final CloudServiceErrorCode code;
  final String message;
  @override
  String toString() => 'CloudServiceException(${code.name}, $message)';
}

abstract interface class CloudWorkspaceService {
  Future<List<CloudWorkspaceRecord>> listWorkspaces();
  Future<ResourceHandle> issueResourceHandle({
    required String workspaceRef,
    required String resourceRef,
    required String revision,
    required String audience,
    required int generation,
  });
  Future<CloudChangeProposal> propose({
    required String workspaceRef,
    required String expectedRevision,
    required String instruction,
    required int generation,
  });
  Future<CloudApplyReceipt> approve({
    required CloudChangeProposal proposal,
    required int generation,
  });
}

/// Coordinates the M4 vertical slice without importing Flutter or an HTTP SDK.
/// The connector, resource port, and Cloud API may share one adapter, but remain
/// separate capabilities at this boundary.
final class CloudWorkspaceCoordinator {
  CloudWorkspaceCoordinator({
    required this.service,
    required this.connector,
    required ResourceRangePort resources,
    this.resourceAudience = 'openmuse-mobile-resource',
  }) : resourceClient = MobileResourceClient(resources);

  final CloudWorkspaceService service;
  final DshRuntimeConnector connector;
  final MobileResourceClient resourceClient;
  final String resourceAudience;
  final CloudWorkspaceFlow flow = CloudWorkspaceFlow();
  final DshPresentationController presentation = DshPresentationController();
  CloudWorkspaceRecord? workspace;
  CloudChangeProposal? proposal;
  CloudApplyReceipt? receipt;

  Future<List<CloudWorkspaceRecord>> loginAndLoadCatalog() async {
    flow.login();
    try {
      return await service.listWorkspaces();
    } on CloudServiceException catch (error) {
      _serviceFailure(error);
      rethrow;
    }
  }

  Future<void> select(CloudWorkspaceRecord value) async {
    workspace = value;
    proposal = null;
    receipt = null;
    flow.select(value.workspaceRef, value.revision);
    if (value.storageState == CloudStorageState.unavailable) {
      flow.storageFailed();
      return;
    }
    final expectedGeneration = flow.generation;
    try {
      await presentation.open(connector, value.workspaceRef);
      if (expectedGeneration != flow.generation ||
          presentation.generation != expectedGeneration ||
          presentation.session == null) {
        return;
      }
      flow.sessionOpened(expectedGeneration);
    } on CloudServiceException catch (error) {
      _serviceFailure(error);
      rethrow;
    }
  }

  void pageLoaded(int generation) {
    if (generation != flow.generation) return;
    presentation.pageLoaded(generation);
  }

  void bridgeBound(int generation) {
    if (generation != flow.generation) return;
    presentation.bridgeBound(generation);
  }

  void workspaceAttached(int generation, String workspaceRef) {
    if (generation != flow.generation ||
        workspaceRef != workspace?.workspaceRef) {
      return;
    }
    presentation.workspaceAttached(generation);
    if (presentation.state == DshPresentationState.ready) {
      flow.bound(generation, workspaceRef);
    }
  }

  Future<String> readTextResource(
    String resourceRef, {
    required int nowMs,
  }) async {
    final current = _readyWorkspace();
    final generation = flow.generation;
    final handle = await service.issueResourceHandle(
      workspaceRef: current.workspaceRef,
      resourceRef: resourceRef,
      revision: flow.revision!,
      audience: resourceAudience,
      generation: generation,
    );
    if (generation != flow.generation || handle.resourceRef != resourceRef) {
      throw StateError('late or cross-resource handle');
    }
    final bytes = await resourceClient.readRange(
      handle,
      audience: resourceAudience,
      generation: generation,
      nowMs: nowMs,
      start: 0,
      endExclusive: handle.size,
    );
    return String.fromCharCodes(bytes);
  }

  Future<CloudChangeProposal> propose(String instruction) async {
    final current = _readyWorkspace();
    final generation = flow.generation;
    final expectedRevision = flow.revision!;
    flow.propose(expectedRevision);
    try {
      final value = await service.propose(
        workspaceRef: current.workspaceRef,
        expectedRevision: expectedRevision,
        instruction: instruction,
        generation: generation,
      );
      if (generation != flow.generation ||
          value.generation != generation ||
          value.workspaceRef != current.workspaceRef ||
          value.expectedRevision != expectedRevision) {
        throw StateError('late, cross-workspace, or stale proposal');
      }
      return proposal = value;
    } on CloudServiceException catch (error) {
      _serviceFailure(error);
      rethrow;
    }
  }

  Future<CloudApplyReceipt> approve() async {
    final current = workspace;
    final pending = proposal;
    if (current == null ||
        pending == null ||
        flow.state != CloudFlowState.awaitingApproval) {
      throw StateError('approval unavailable');
    }
    final generation = flow.generation;
    try {
      final value = await service.approve(
        proposal: pending,
        generation: generation,
      );
      if (generation != flow.generation ||
          value.generation != generation ||
          value.workspaceRef != current.workspaceRef ||
          value.previousRevision != flow.revision) {
        throw StateError('late, cross-workspace, or stale receipt');
      }
      flow.approve(value.newRevision);
      return receipt = value;
    } on CloudServiceException catch (error) {
      _serviceFailure(error);
      rethrow;
    }
  }

  CloudWorkspaceRecord _readyWorkspace() {
    final value = workspace;
    if (value == null || flow.state != CloudFlowState.ready) {
      throw StateError('workspace is not ready');
    }
    return value;
  }

  void _serviceFailure(CloudServiceException error) {
    if (error.code == CloudServiceErrorCode.storageUnavailable) {
      flow.storageFailed();
    } else {
      flow.degraded();
    }
  }
}
