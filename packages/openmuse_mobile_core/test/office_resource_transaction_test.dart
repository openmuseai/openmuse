import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  const bytes = [1, 2, 3, 4, 5];
  final handle = ResourceHandle(
    resourceRef: 'resource:docx',
    revision: 'r1',
    audience: 'openmuse-mobile-office',
    generation: 7,
    expiresAtMs: 2000,
    size: bytes.length,
    mediaType:
        'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  );

  test('opens by bounded ranges and commits with CAS receipt', () async {
    final ranges = _Ranges(bytes);
    final commits = _Commits();
    final transaction = OfficeResourceTransaction(
      engine: _Engine(editable: true),
      ranges: ranges,
      commits: commits,
      chunkBytes: 2,
    );
    final session = await transaction.open(handle, generation: 7, nowMs: 1000);
    expect(ranges.requests, [(0, 2), (2, 4), (4, 5)]);
    expect(session.inspection.paragraphs, ['original']);

    final receipt = await transaction.saveSimple(
      session: session,
      paragraphs: const ['changed'],
      expectedRevision: 'r1',
      idempotencyKey: 'save:1',
      generation: 7,
      nowMs: 1001,
    );
    expect(receipt.newRevision, 'r2');
    expect(commits.bytes, [9, 8, 7]);
  });

  test('expired, cross-generation, oversized, and short ranges fail', () async {
    Future<void> denied(ResourceHandle value, _Ranges ranges) async {
      final transaction = OfficeResourceTransaction(
        engine: _Engine(editable: true),
        ranges: ranges,
        commits: _Commits(),
        maxDocumentBytes: 8,
      );
      await expectLater(
        transaction.open(value, generation: 7, nowMs: 2000),
        throwsStateError,
      );
    }

    await denied(handle, _Ranges(bytes));
    await denied(
      ResourceHandle(
        resourceRef: handle.resourceRef,
        revision: handle.revision,
        audience: handle.audience,
        generation: 8,
        expiresAtMs: 3000,
        size: handle.size,
        mediaType: handle.mediaType,
      ),
      _Ranges(bytes),
    );
    await denied(
      ResourceHandle(
        resourceRef: handle.resourceRef,
        revision: handle.revision,
        audience: handle.audience,
        generation: 7,
        expiresAtMs: 3000,
        size: 9,
        mediaType: handle.mediaType,
      ),
      _Ranges(bytes),
    );
    await expectLater(
      OfficeResourceTransaction(
        engine: _Engine(editable: true),
        ranges: _Ranges(bytes, short: true),
        commits: _Commits(),
      ).open(handle, generation: 7, nowMs: 1000),
      throwsStateError,
    );
  });

  test(
    'view-only documents and stale or invalid receipts never save',
    () async {
      final viewOnly = OfficeResourceTransaction(
        engine: _Engine(editable: false),
        ranges: _Ranges(bytes),
        commits: _Commits(),
      );
      final session = await viewOnly.open(handle, generation: 7, nowMs: 1000);
      await expectLater(
        viewOnly.saveSimple(
          session: session,
          paragraphs: const ['changed'],
          expectedRevision: 'r1',
          idempotencyKey: 'save:2',
          generation: 7,
          nowMs: 1001,
        ),
        throwsStateError,
      );

      final invalidReceipt = OfficeResourceTransaction(
        engine: _Engine(editable: true),
        ranges: _Ranges(bytes),
        commits: _Commits(invalid: true),
      );
      final editable = await invalidReceipt.open(
        handle,
        generation: 7,
        nowMs: 1000,
      );
      await expectLater(
        invalidReceipt.saveSimple(
          session: editable,
          paragraphs: const ['changed'],
          expectedRevision: 'r1',
          idempotencyKey: 'save:3',
          generation: 7,
          nowMs: 1001,
        ),
        throwsStateError,
      );
    },
  );

  test('sheet format admits only sheet media and dispatches sheet', () async {
    final sheetHandle = ResourceHandle(
      resourceRef: 'resource:xlsx',
      revision: 'r1',
      audience: 'openmuse-mobile-office',
      generation: 7,
      expiresAtMs: 2000,
      size: bytes.length,
      mediaType:
          'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    );
    final transaction = OfficeResourceTransaction(
      engine: _Engine(editable: false),
      ranges: _Ranges(bytes),
      commits: _Commits(),
      format: OfficeFormat.sheet,
    );
    final session = await transaction.open(
      sheetHandle,
      generation: 7,
      nowMs: 1000,
    );
    expect(session.inspection.format, OfficeFormat.sheet);
    await expectLater(
      transaction.open(handle, generation: 7, nowMs: 1000),
      throwsStateError,
    );
  });
}

final class _Ranges implements ResourceRangePort {
  _Ranges(this.bytes, {this.short = false});
  final List<int> bytes;
  final bool short;
  final List<(int, int)> requests = [];

  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async {
    requests.add((start, endExclusive));
    final result = bytes.sublist(start, endExclusive);
    return short ? result.take(result.length - 1).toList() : result;
  }
}

final class _Engine implements OfficeEnginePort {
  _Engine({required this.editable});
  final bool editable;

  @override
  String get abi => 'openmuse-docx-ffi@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => OfficeEngineInspection(
    format: format,
    profile: editable ? 'simple-text' : 'view-only',
    paragraphs: const ['original'],
    capabilities: editable
        ? const {
            OfficeCapability.view,
            OfficeCapability.edit,
            OfficeCapability.export,
          }
        : const {OfficeCapability.view},
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) async => const [9, 8, 7];
}

final class _Commits implements OfficeResourceCommitPort {
  _Commits({this.invalid = false});
  final bool invalid;
  List<int>? bytes;

  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) async {
    this.bytes = List<int>.from(bytes);
    return OfficeResourceCommitReceipt(
      commitRef: 'commit:1',
      resourceRef: resourceRef,
      previousRevision: expectedRevision,
      newRevision: invalid ? expectedRevision : 'r2',
      generation: generation,
    );
  }
}
