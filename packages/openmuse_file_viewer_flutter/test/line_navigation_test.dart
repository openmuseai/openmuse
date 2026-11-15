import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_file_viewer_flutter/openmuse_file_viewer_flutter.dart';

void main() {
  testWidgets('owner-supplied line opens visibly in the read-only viewer', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 220,
            child: OpenMuseTextBody(
              contents: List.generate(
                120,
                (i) => 'source line ${i + 1}',
              ).join('\n'),
              initialLine: 100,
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('source line 100'), findsOneWidget);
    expect(find.text('source line 1'), findsNothing);
    expect(tester.takeException(), isNull);
  });
}
