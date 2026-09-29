import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/docx_editor_screen.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  testWidgets('editable DOCX saves only through a revision receipt', (
    tester,
  ) async {
    final commits = _Commits();
    await tester.pumpWidget(
      MaterialApp(
        home: DocxEditorScreen(
          title: 'Document.docx',
          handle: _handle,
          engine: _Engine(editable: true),
          ranges: _Ranges(),
          commits: commits,
          generation: 1,
          nowMs: () => 100,
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('DOCX · simple-text'), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('docx-paragraph-0')),
      'changed',
    );
    await tester.tap(find.byKey(const ValueKey('docx-save')));
    await tester.pumpAndSettle();
    expect(find.text('已保存 · r2'), findsOneWidget);
    expect(commits.expectedRevision, 'r1');
    expect(commits.bytes, [7, 8]);
  });

  testWidgets('view-only DOCX has no save entry', (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: DocxEditorScreen(
          title: 'Complex.docx',
          handle: _handle,
          engine: _Engine(editable: false),
          ranges: _Ranges(),
          commits: _Commits(),
          generation: 1,
          nowMs: () => 100,
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('docx-save')), findsNothing);
    expect(find.text('此文档包含未认证结构，仅支持查看'), findsOneWidget);
  });
}

const _handle = ResourceHandle(
  resourceRef: 'resource:docx',
  revision: 'r1',
  audience: 'openmuse-mobile-office',
  generation: 1,
  expiresAtMs: 1000,
  size: 3,
  mediaType:
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
);

final class _Ranges implements ResourceRangePort {
  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async => const [1, 2, 3].sublist(start, endExclusive);
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
    format: OfficeFormat.word,
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
  ) async => const [7, 8];
}

final class _Commits implements OfficeResourceCommitPort {
  String? expectedRevision;
  List<int>? bytes;
  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) async {
    this.expectedRevision = expectedRevision;
    this.bytes = bytes;
    return OfficeResourceCommitReceipt(
      commitRef: 'commit:1',
      resourceRef: resourceRef,
      previousRevision: expectedRevision,
      newRevision: 'r2',
      generation: generation,
    );
  }
}
