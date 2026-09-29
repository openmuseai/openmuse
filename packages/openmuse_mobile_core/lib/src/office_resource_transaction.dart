import 'office_formats.dart';
import 'resource_client.dart';

final class OfficeResourceCommitReceipt {
  const OfficeResourceCommitReceipt({
    required this.commitRef,
    required this.resourceRef,
    required this.previousRevision,
    required this.newRevision,
    required this.generation,
  });

  final String commitRef;
  final String resourceRef;
  final String previousRevision;
  final String newRevision;
  final int generation;
}

/// A narrow CAS capability. The implementation may stage bytes in Cloud,
/// paired Desktop, or local storage, but it must not expose provider secrets to
/// the Office engine.
abstract interface class OfficeResourceCommitPort {
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  });
}

final class OfficeDocumentSession {
  const OfficeDocumentSession({
    required this.handle,
    required this.inspection,
    required this.bytes,
  });

  final ResourceHandle handle;
  final OfficeEngineInspection inspection;
  final List<int> bytes;
}

/// Coordinates bounded Resource I/O and DOCX export without giving the engine
/// a Resource, Workspace, network, account, or storage-provider capability.
final class OfficeResourceTransaction {
  OfficeResourceTransaction({
    required this.engine,
    required this.ranges,
    required this.commits,
    this.audience = 'openmuse-mobile-office',
    this.maxDocumentBytes = 64 * 1024 * 1024,
    this.chunkBytes = 512 * 1024,
  }) {
    if (maxDocumentBytes <= 0 || chunkBytes <= 0) {
      throw ArgumentError('invalid Office Resource limits');
    }
  }

  final OfficeEnginePort engine;
  final ResourceRangePort ranges;
  final OfficeResourceCommitPort commits;
  final String audience;
  final int maxDocumentBytes;
  final int chunkBytes;

  Future<OfficeDocumentSession> open(
    ResourceHandle handle, {
    required int generation,
    required int nowMs,
  }) async {
    _validateHandle(handle, generation: generation, nowMs: nowMs);
    final bytes = <int>[];
    for (var start = 0; start < handle.size; start += chunkBytes) {
      final end = (start + chunkBytes).clamp(0, handle.size);
      final chunk = await ranges.read(handle, start, end);
      if (chunk.length != end - start ||
          chunk.any((value) => value < 0 || value > 255)) {
        throw StateError('invalid Office Resource range');
      }
      bytes.addAll(chunk);
    }
    if (bytes.length != handle.size) {
      throw StateError('incomplete Office Resource');
    }
    final inspection = await engine.inspect(OfficeFormat.word, bytes);
    if (inspection.format != OfficeFormat.word ||
        !inspection.capabilities.contains(OfficeCapability.view)) {
      throw StateError('DOCX view capability unavailable');
    }
    return OfficeDocumentSession(
      handle: handle,
      inspection: inspection,
      bytes: List<int>.unmodifiable(bytes),
    );
  }

  Future<OfficeResourceCommitReceipt> saveSimple({
    required OfficeDocumentSession session,
    required List<String> paragraphs,
    required String expectedRevision,
    required String idempotencyKey,
    required int generation,
    required int nowMs,
  }) async {
    _validateHandle(session.handle, generation: generation, nowMs: nowMs);
    if (expectedRevision != session.handle.revision ||
        idempotencyKey.trim().isEmpty ||
        !session.inspection.capabilities.contains(OfficeCapability.edit) ||
        !session.inspection.capabilities.contains(OfficeCapability.export)) {
      throw StateError('DOCX export not admitted');
    }
    final output = await engine.exportSimple(
      OfficeFormat.word,
      session.bytes,
      paragraphs,
    );
    if (output.isEmpty || output.length > maxDocumentBytes) {
      throw StateError('invalid DOCX export');
    }
    final receipt = await commits.commit(
      resourceRef: session.handle.resourceRef,
      expectedRevision: expectedRevision,
      bytes: output,
      idempotencyKey: idempotencyKey,
      generation: generation,
    );
    if (receipt.commitRef.isEmpty ||
        receipt.resourceRef != session.handle.resourceRef ||
        receipt.previousRevision != expectedRevision ||
        receipt.newRevision.isEmpty ||
        receipt.newRevision == expectedRevision ||
        receipt.generation != generation) {
      throw StateError('invalid Office commit receipt');
    }
    return receipt;
  }

  void _validateHandle(
    ResourceHandle handle, {
    required int generation,
    required int nowMs,
  }) {
    if (handle.audience != audience ||
        handle.generation != generation ||
        nowMs >= handle.expiresAtMs ||
        handle.size <= 0 ||
        handle.size > maxDocumentBytes ||
        (handle.mediaType !=
                'application/vnd.openxmlformats-officedocument.wordprocessingml.document' &&
            handle.mediaType != 'application/docx')) {
      throw StateError('Office Resource handle denied');
    }
  }
}
