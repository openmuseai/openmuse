import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/ai_content_reporting.dart';

void main() {
  testWidgets('cancel does not fetch a token or send the selected response', (
    tester,
  ) async {
    var requests = 0;
    final client = AiContentReportClient(
      origin: Uri.parse('https://unused.example'),
      accessToken: () async {
        requests++;
        return null;
      },
      currentAccount: () => 'a',
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Builder(
          builder: (context) => Scaffold(
            body: TextButton(
              onPressed: () => showAiContentReport(
                context: context,
                client: client,
                response: 'QA response',
                provider: 'QA',
                model: 'qa',
              ),
              child: const Text('Open report'),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.text('Open report'));
    await tester.pumpAndSettle();
    expect(find.text('QA response'), findsOneWidget);
    expect(requests, 0);
    await tester.tap(find.text('Cancel'));
    await tester.pumpAndSettle();
    expect(find.byType(AlertDialog), findsNothing);
    expect(requests, 0);
  });

  testWidgets(
    'failed submission stays open and does not show a success receipt',
    (tester) async {
      final client = AiContentReportClient(
        origin: Uri.parse('https://unused.example'),
        accessToken: () async => null,
        currentAccount: () => 'a',
      );
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: AiContentReportDialog(
              client: client,
              account: 'a',
              response: 'QA response',
              provider: 'QA',
              model: 'qa',
            ),
          ),
        ),
      );
      await tester.tap(find.byKey(const ValueKey('ai-report-submit')));
      await tester.pumpAndSettle();
      expect(find.byType(AlertDialog), findsOneWidget);
      expect(find.textContaining('Sign in again'), findsOneWidget);
      expect(find.text('Report submitted for safety review.'), findsNothing);
      expect(
        tester
            .widget<FilledButton>(
              find.byKey(const ValueKey('ai-report-submit')),
            )
            .onPressed,
        isNotNull,
      );
    },
  );
}
