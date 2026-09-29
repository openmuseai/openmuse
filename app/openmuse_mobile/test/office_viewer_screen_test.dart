import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/office_viewer_screen.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  testWidgets('XLSX viewer is capability-gated and has no save action', (
    tester,
  ) async {
    final handle = ResourceHandle(
      resourceRef: 'resource:xlsx',
      revision: 'r1',
      audience: 'openmuse-mobile-office',
      generation: 3,
      expiresAtMs: 2000,
      size: 3,
      mediaType:
          'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    );
    await tester.pumpWidget(
      MaterialApp(
        home: OfficeViewerScreen(
          title: 'Budget.xlsx',
          format: OfficeFormat.sheet,
          handle: handle,
          engine: _ViewerEngine(),
          ranges: const _Ranges(),
          generation: 3,
          nowMs: () => 1000,
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('office-view-only')), findsOneWidget);
    expect(find.textContaining('Sheet A'), findsOneWidget);
    expect(find.byIcon(Icons.save), findsNothing);
  });
}

final class _ViewerEngine implements OfficeEnginePort {
  @override
  String get abi => 'test-viewer@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async => const OfficeEngineInspection(
    format: OfficeFormat.sheet,
    profile: 'view-only',
    paragraphs: ['Sheet A\t42'],
    capabilities: {OfficeCapability.view},
  );

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw UnsupportedError('view only');
}

final class _Ranges implements ResourceRangePort {
  const _Ranges();
  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async => const [1, 2, 3].sublist(start, endExclusive);
}
