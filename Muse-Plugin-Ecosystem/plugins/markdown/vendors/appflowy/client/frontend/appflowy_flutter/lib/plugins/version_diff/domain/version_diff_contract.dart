import 'dart:typed_data';

/// The universal version/diff contract.
///
/// This layer deliberately has no knowledge of lines, slides, cells, frames,
/// shapes, or any other domain object. Domain providers own those concepts.
enum MuseVersionKind { committed, working, proposal }

/// Stable actor classification used by audit renderers. Providers may attach
/// richer identity/tenant data outside this universal contract.
enum MuseActorKind { user, agent, system }

enum MuseProposalStatus { open, partiallyApplied, accepted, rejected, conflict }

enum MuseDecisionKind { accept, reject }

enum MuseApplyStatus { applied, rejected, conflict, stale, unsupported }

final class MuseRepositoryRef {
  const MuseRepositoryRef({required this.id, required this.providerId});

  final String id;
  final String providerId;
}

final class MuseResourceRef {
  const MuseResourceRef({
    required this.id,
    required this.repository,
    required this.locator,
    required this.mediaType,
    required this.displayName,
  });

  final String id;
  final MuseRepositoryRef repository;
  final String locator;
  final String mediaType;
  final String displayName;
}

final class MuseActorRef {
  const MuseActorRef({
    required this.id,
    required this.displayName,
    this.kind = MuseActorKind.user,
  });

  final String id;
  final String displayName;
  final MuseActorKind kind;
}

final class MuseVersionRef {
  const MuseVersionRef(this.id);

  final String id;
}

final class MuseVersion {
  const MuseVersion({
    required this.ref,
    required this.resource,
    required this.kind,
    required this.contentRef,
    required this.contentDigest,
    required this.byteLength,
    required this.createdAt,
    required this.actor,
    this.parents = const [],
    this.message,
  });

  final MuseVersionRef ref;
  final MuseResourceRef resource;
  final MuseVersionKind kind;
  final String contentRef;
  final String contentDigest;
  final int byteLength;
  final DateTime createdAt;
  final MuseActorRef actor;
  final List<MuseVersionRef> parents;
  final String? message;
}

final class MuseComparison {
  const MuseComparison({
    required this.id,
    required this.resource,
    required this.base,
    required this.target,
    required this.createdAt,
    required this.actor,
    required this.rendererType,
    this.proposalId,
  });

  final String id;
  final MuseResourceRef resource;
  final MuseVersionRef base;
  final MuseVersionRef target;
  final DateTime createdAt;
  final MuseActorRef actor;
  final String rendererType;
  final String? proposalId;
}

final class MuseResourceDiff<TPayload> {
  const MuseResourceDiff({
    required this.comparison,
    required this.payload,
    required this.changeCount,
    required this.summary,
  });

  final MuseComparison comparison;
  final TPayload payload;
  final int changeCount;
  final String summary;
}

final class MuseProposal {
  const MuseProposal({
    required this.id,
    required this.resource,
    required this.base,
    required this.proposed,
    required this.actor,
    required this.createdAt,
    required this.status,
    this.intent,
    this.origin,
  });

  final String id;
  final MuseResourceRef resource;
  final MuseVersionRef base;
  final MuseVersionRef proposed;
  final MuseActorRef actor;
  final DateTime createdAt;
  final MuseProposalStatus status;
  final String? intent;
  final String? origin;
}

final class MuseChangeDecision {
  const MuseChangeDecision({
    required this.proposalId,
    required this.changeId,
    required this.kind,
    required this.actor,
    required this.decidedAt,
  });

  final String proposalId;
  final String changeId;
  final MuseDecisionKind kind;
  final MuseActorRef actor;
  final DateTime decidedAt;
}

final class MuseApplyResult {
  const MuseApplyResult({
    required this.status,
    required this.message,
    this.version,
    this.conflictIds = const [],
  });

  final MuseApplyStatus status;
  final String message;
  final MuseVersionRef? version;
  final List<String> conflictIds;
}

final class MuseAuditEvent {
  const MuseAuditEvent({
    required this.id,
    required this.type,
    required this.repositoryId,
    required this.resourceId,
    required this.actor,
    required this.occurredAt,
    this.subjectId,
    this.metadata = const {},
  });

  final String id;
  final String type;
  final String repositoryId;
  final String resourceId;
  final MuseActorRef actor;
  final DateTime occurredAt;
  final String? subjectId;
  final Map<String, Object?> metadata;
}

abstract interface class MuseContentResolver {
  Future<Uint8List> resolve(MuseVersion version);
}

abstract interface class MuseDiffProvider<TPayload> {
  String get id;

  String get rendererType;

  bool supports(MuseResourceRef resource);

  Future<MuseResourceDiff<TPayload>> compare({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  });
}

/// Host-owned provider registry. Providers can be contributed by built-ins or
/// later external plugins without changing the universal protocol.
final class MuseDiffProviderRegistry {
  final List<MuseDiffProvider<Object>> _providers = [];

  void register<TPayload>(MuseDiffProvider<TPayload> provider) {
    _providers.removeWhere((candidate) => candidate.id == provider.id);
    _providers.add(provider as MuseDiffProvider<Object>);
  }

  void unregister(String id) {
    _providers.removeWhere((provider) => provider.id == id);
  }

  MuseDiffProvider<Object>? providerFor(MuseResourceRef resource) {
    for (final provider in _providers.reversed) {
      if (provider.supports(resource)) return provider;
    }
    return null;
  }
}
