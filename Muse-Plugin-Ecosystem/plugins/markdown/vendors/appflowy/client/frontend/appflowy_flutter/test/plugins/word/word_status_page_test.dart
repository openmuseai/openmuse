import 'package:appflowy/plugins/word/word_status_page.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('P2-T7 / P5-T7 status pages render copy and are not blank',
      (tester) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: WordStatusPage(message: wordWebUnsupportedMessage),
      ),
    );
    expect(find.text(wordWebUnsupportedMessage), findsOneWidget);
    expect(find.byType(CircularProgressIndicator), findsNothing);

    await tester.pumpWidget(
      const MaterialApp(
        home: WordStatusPage(message: wordMissingBlobMessage),
      ),
    );
    expect(find.text(wordMissingBlobMessage), findsOneWidget);
  });
}
