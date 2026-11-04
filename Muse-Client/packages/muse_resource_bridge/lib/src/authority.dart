import 'dart:async';
import 'package:crypto/crypto.dart';
import 'package:muse_resource_contract/muse_resource_contract.dart';

enum MuseWorkspaceProviderKind { local, cloud, paired }

enum MuseAuthorityCommitResult { committed, noChange, conflict }

final class MuseAuthorityException implements Exception {
  const MuseAuthorityException(this.code);
  final String code;
  @override
  String toString() => 'MuseAuthorityException($code)';
}

final class MuseResourcePage {
  const MuseResourcePage({required this.items, this.nextCursor});
  final List<MuseResourceDescriptorV1> items;
  final String? nextCursor;
}

final class MuseAuthorityMaterialization {
  const MuseAuthorityMaterialization({
    required this.value,
    required this.audience,
    required this.generation,
  });
  final MuseResourceMaterializationV1 value;
  final String audience;
  final int generation;
}

final class MuseWorkingCopy {
  const MuseWorkingCopy({
    required this.draftRef,
    required this.resourceRef,
    required this.baseRevision,
    required this.audience,
    required this.generation,
    required this.expiresAt,
  });
  final String draftRef, resourceRef, baseRevision, audience;
  final int generation, expiresAt;
}

final class MuseAuthorityCommitReceipt {
  const MuseAuthorityCommitReceipt({
    required this.commitRef,
    required this.resourceRef,
    required this.result,
    required this.providerReceiptRef,
    this.newRevision,
    this.currentRevision,
  });
  final String commitRef, resourceRef, providerReceiptRef;
  final MuseAuthorityCommitResult result;
  final String? newRevision, currentRevision;
}

final class MuseAuthorityEvent {
  const MuseAuthorityEvent({
    required this.eventRef,
    required this.workspaceRef,
    required this.resourceRef,
    required this.beforeRevision,
    required this.afterRevision,
  });
  final String eventRef, workspaceRef, resourceRef;
  final String beforeRevision, afterRevision;
}

abstract interface class MuseWorkspaceAuthority {
  String get workspaceRef;
  String get mountRef;
  MuseWorkspaceProviderKind get providerKind;
  Future<MuseResourcePage> list({String? cursor, int limit = 50});
  Future<MuseResourceDescriptorV1> describe(String resourceRef);
  Future<MuseAuthorityMaterialization> materialize(
    String resourceRef, {
    required String audience,
    required MuseAccessMode accessMode,
    required int generation,
    required int ttlMs,
  });
  Future<MuseWorkingCopy> createDraft(
    String resourceRef, {
    required String audience,
    required int generation,
    required int ttlMs,
  });
  Future<void> writeDraft(
    String draftRef,
    List<int> bytes, {
    required String audience,
    required int generation,
  });
  Future<MuseAuthorityCommitReceipt> commit({
    required String commitRef,
    required String resourceRef,
    required String expectedRevision,
    required String draftRef,
    required String audience,
    required int generation,
    required String idempotencyKey,
  });
  Stream<MuseAuthorityEvent> subscribe();
}

final class MuseInMemoryWorkspaceAuthority implements MuseWorkspaceAuthority {
  MuseInMemoryWorkspaceAuthority({
    required this.workspaceRef,
    required this.mountRef,
    required this.providerKind,
    int Function()? clock,
  }) : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch) {
    _requireOpaqueRef(workspaceRef);
    _requireOpaqueRef(mountRef);
  }

  @override
  final String workspaceRef;
  @override
  final String mountRef;
  @override
  final MuseWorkspaceProviderKind providerKind;
  final int Function() _clock;
  final _resources = <String, _StoredResource>{};
  final _handles = <String, _Handle>{};
  final _drafts = <String, _Draft>{};
  final _receipts = <String, _IdempotentReceipt>{};
  final _blobs = <String, List<int>>{};
  final _events = StreamController<MuseAuthorityEvent>.broadcast(sync: true);
  int _sequence = 1;
  bool failNextMetadataCommit = false;

  int get orphanBlobCount => _blobs.keys
      .where(
          (digest) => !_resources.values.any((item) => item.digest == digest))
      .length;

  void seed({
    required String resourceRef,
    required String displayName,
    required String mediaType,
    required List<int> bytes,
  }) {
    _requireOpaqueRef(resourceRef);
    if (_resources.containsKey(resourceRef)) {
      throw const MuseAuthorityException('CONFLICT');
    }
    final digest = _digest(bytes);
    _blobs[digest] = List.unmodifiable(bytes);
    _resources[resourceRef] = _StoredResource(
      resourceRef: resourceRef,
      revision: 'revision.${_sequence++}',
      displayName: displayName,
      mediaType: mediaType,
      digest: digest,
    );
  }

  @override
  Future<MuseResourcePage> list({String? cursor, int limit = 50}) async {
    if (limit < 1 || limit > 200) {
      throw const MuseAuthorityException('INVALID_LIMIT');
    }
    final offset = cursor == null ? 0 : int.tryParse(cursor);
    if (offset == null || offset < 0) {
      throw const MuseAuthorityException('INVALID_CURSOR');
    }
    final values = _resources.values.toList()
      ..sort((a, b) => a.resourceRef.compareTo(b.resourceRef));
    final end = (offset + limit).clamp(0, values.length);
    return MuseResourcePage(
      items: values
          .sublist(offset.clamp(0, values.length), end)
          .map(_descriptor)
          .toList(),
      nextCursor: end < values.length ? '$end' : null,
    );
  }

  @override
  Future<MuseResourceDescriptorV1> describe(String resourceRef) async {
    _requireOpaqueRef(resourceRef);
    return _descriptor(_resource(resourceRef));
  }

  @override
  Future<MuseAuthorityMaterialization> materialize(
    String resourceRef, {
    required String audience,
    required MuseAccessMode accessMode,
    required int generation,
    required int ttlMs,
  }) async {
    _requireOpaqueRef(resourceRef);
    if (providerKind != MuseWorkspaceProviderKind.local &&
        accessMode == MuseAccessMode.readWrite) {
      throw const MuseAuthorityException('DENIED');
    }
    final resource = _resource(resourceRef);
    _validateGrant(audience, generation, ttlMs);
    final ref = 'handle.${_sequence++}';
    final expiresAt = _clock() + ttlMs;
    _handles[ref] = _Handle(
      resourceRef: resourceRef,
      revision: resource.revision,
      digest: resource.digest,
      audience: audience,
      generation: generation,
      expiresAt: expiresAt,
    );
    return MuseAuthorityMaterialization(
      value: MuseResourceMaterializationV1(
        handleRef: ref,
        resourceRef: resourceRef,
        revision: resource.revision,
        consumerAdapterRef: audience,
        kind: providerKind == MuseWorkspaceProviderKind.local
            ? MuseMaterializationKind.bytesHandle
            : MuseMaterializationKind.streamHandle,
        accessMode: accessMode,
        expiresAt: expiresAt,
        sizeBytes: _blobs[resource.digest]!.length,
        digest: resource.digest,
      ),
      audience: audience,
      generation: generation,
    );
  }

  List<int> readMaterialization(
    String handleRef, {
    required String audience,
    required int generation,
  }) {
    final handle = _handles[handleRef];
    if (handle == null) throw const MuseAuthorityException('NOT_FOUND');
    _authorize(handle.audience, handle.generation, handle.expiresAt, audience,
        generation);
    return List.unmodifiable(_blobs[handle.digest]!);
  }

  @override
  Future<MuseWorkingCopy> createDraft(
    String resourceRef, {
    required String audience,
    required int generation,
    required int ttlMs,
  }) async {
    _requireOpaqueRef(resourceRef);
    final resource = _resource(resourceRef);
    _validateGrant(audience, generation, ttlMs);
    final ref = 'draft.${_sequence++}';
    final draft = _Draft(
      resourceRef: resourceRef,
      baseRevision: resource.revision,
      audience: audience,
      generation: generation,
      expiresAt: _clock() + ttlMs,
      bytes: List.of(_blobs[resource.digest]!),
    );
    _drafts[ref] = draft;
    return MuseWorkingCopy(
      draftRef: ref,
      resourceRef: resourceRef,
      baseRevision: resource.revision,
      audience: audience,
      generation: generation,
      expiresAt: draft.expiresAt,
    );
  }

  @override
  Future<void> writeDraft(
    String draftRef,
    List<int> bytes, {
    required String audience,
    required int generation,
  }) async {
    final draft = _drafts[draftRef];
    if (draft == null) throw const MuseAuthorityException('NOT_FOUND');
    _authorize(draft.audience, draft.generation, draft.expiresAt, audience,
        generation);
    draft.bytes = List.of(bytes);
  }

  @override
  Future<MuseAuthorityCommitReceipt> commit({
    required String commitRef,
    required String resourceRef,
    required String expectedRevision,
    required String draftRef,
    required String audience,
    required int generation,
    required String idempotencyKey,
  }) async {
    final fingerprint =
        '$commitRef|$resourceRef|$expectedRevision|$draftRef|$audience|$generation';
    final prior = _receipts[idempotencyKey];
    if (prior != null) {
      if (prior.fingerprint != fingerprint) {
        throw const MuseAuthorityException('CONFLICT');
      }
      return prior.receipt;
    }
    final resource = _resource(resourceRef);
    final draft = _drafts[draftRef];
    if (draft == null || draft.resourceRef != resourceRef) {
      throw const MuseAuthorityException('NOT_FOUND');
    }
    _authorize(draft.audience, draft.generation, draft.expiresAt, audience,
        generation);
    if (resource.revision != expectedRevision ||
        draft.baseRevision != expectedRevision) {
      final receipt = MuseAuthorityCommitReceipt(
        commitRef: commitRef,
        resourceRef: resourceRef,
        result: MuseAuthorityCommitResult.conflict,
        providerReceiptRef: 'receipt.${_sequence++}',
        currentRevision: resource.revision,
      );
      _receipts[idempotencyKey] = _IdempotentReceipt(fingerprint, receipt);
      return receipt;
    }
    final digest = _digest(draft.bytes);
    if (digest == resource.digest) {
      final receipt = MuseAuthorityCommitReceipt(
        commitRef: commitRef,
        resourceRef: resourceRef,
        result: MuseAuthorityCommitResult.noChange,
        providerReceiptRef: 'receipt.${_sequence++}',
        newRevision: resource.revision,
      );
      _receipts[idempotencyKey] = _IdempotentReceipt(fingerprint, receipt);
      return receipt;
    }
    _blobs[digest] = List.unmodifiable(draft.bytes);
    if (failNextMetadataCommit) {
      failNextMetadataCommit = false;
      throw const MuseAuthorityException('METADATA_COMMIT_FAILED');
    }
    final before = resource.revision;
    resource.revision = 'revision.${_sequence++}';
    resource.digest = digest;
    final receipt = MuseAuthorityCommitReceipt(
      commitRef: commitRef,
      resourceRef: resourceRef,
      result: MuseAuthorityCommitResult.committed,
      providerReceiptRef: 'receipt.${_sequence++}',
      newRevision: resource.revision,
    );
    _receipts[idempotencyKey] = _IdempotentReceipt(fingerprint, receipt);
    _events.add(MuseAuthorityEvent(
      eventRef: 'event.${_sequence++}',
      workspaceRef: workspaceRef,
      resourceRef: resourceRef,
      beforeRevision: before,
      afterRevision: resource.revision,
    ));
    return receipt;
  }

  @override
  Stream<MuseAuthorityEvent> subscribe() => _events.stream;

  _StoredResource _resource(String ref) =>
      _resources[ref] ?? (throw const MuseAuthorityException('NOT_FOUND'));

  MuseResourceDescriptorV1 _descriptor(_StoredResource resource) =>
      MuseResourceDescriptorV1(
        resourceRef: resource.resourceRef,
        revision: resource.revision,
        displayName: resource.displayName,
        mediaType: resource.mediaType,
        sizeBytes: _blobs[resource.digest]!.length,
        format: const MuseResourceFormat(
          formatId: 'binary.unknown',
          confidence: MuseFormatConfidence.claimed,
        ),
        capabilities: const [
          MuseResourceCapability.describe,
          MuseResourceCapability.materialize,
          MuseResourceCapability.commit,
          MuseResourceCapability.subscribe,
        ],
        security: const MuseResourceSecurity(
          classification: MuseSecurityClassification.internal,
          activeContent: MuseActiveContent.unknown,
        ),
      );

  void _validateGrant(String audience, int generation, int ttlMs) {
    if (audience.isEmpty || generation < 1 || ttlMs < 1) {
      throw const MuseAuthorityException('DENIED');
    }
  }

  void _authorize(
    String expectedAudience,
    int expectedGeneration,
    int expiresAt,
    String audience,
    int generation,
  ) {
    if (expectedAudience != audience) {
      throw const MuseAuthorityException('DENIED');
    }
    if (expectedGeneration != generation) {
      throw const MuseAuthorityException('STALE_GENERATION');
    }
    if (_clock() >= expiresAt) {
      throw const MuseAuthorityException('EXPIRED');
    }
  }

  static String _digest(List<int> bytes) => 'sha256:${sha256.convert(bytes)}';

  static void _requireOpaqueRef(String value) {
    final lower = value.toLowerCase();
    if (value.isEmpty ||
        value.startsWith('/') ||
        RegExp(r'^[a-zA-Z]:[\\/]').hasMatch(value) ||
        lower.startsWith('file://') ||
        lower.startsWith('s3://')) {
      throw const MuseAuthorityException('DENIED');
    }
  }
}

final class _StoredResource {
  _StoredResource({
    required this.resourceRef,
    required this.revision,
    required this.displayName,
    required this.mediaType,
    required this.digest,
  });
  final String resourceRef, displayName, mediaType;
  String revision, digest;
}

final class _Handle {
  const _Handle({
    required this.resourceRef,
    required this.revision,
    required this.digest,
    required this.audience,
    required this.generation,
    required this.expiresAt,
  });
  final String resourceRef, revision, digest, audience;
  final int generation, expiresAt;
}

final class _Draft {
  _Draft({
    required this.resourceRef,
    required this.baseRevision,
    required this.audience,
    required this.generation,
    required this.expiresAt,
    required this.bytes,
  });
  final String resourceRef, baseRevision, audience;
  final int generation, expiresAt;
  List<int> bytes;
}

final class _IdempotentReceipt {
  const _IdempotentReceipt(this.fingerprint, this.receipt);
  final String fingerprint;
  final MuseAuthorityCommitReceipt receipt;
}
