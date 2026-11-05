import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/version_history_dialog.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('audit panel attributes and colors semantic change kinds',
      (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 900));
    addTearDown(() => tester.binding.setSurfaceSize(null));
    final comparison = MuseAuditEvent(
      id: 'audit:1',
      type: 'comparison.create',
      repositoryId: 'repository',
      resourceId: 'resource',
      actor: const MuseActorRef(
        id: 'agent:dsh',
        displayName: 'DSH Agent',
        kind: MuseActorKind.agent,
      ),
      occurredAt: DateTime.utc(2026),
      subjectId: 'comparison:1',
      metadata: const {
        'changeSummary': {
          'insertions': 1,
          'deletions': 1,
          'modifications': 1,
          'addedLines': 2,
          'deletedLines': 2,
        },
        'changes': [
          {
            'kind': 'replace',
            'semanticLabel': 'Plan',
            'newStart': 3,
            'before': ['Original scope'],
            'after': ['Expanded scope'],
          },
          {
            'kind': 'insert',
            'semanticLabel': 'Plan',
            'newStart': 8,
            'before': <String>[],
            'after': ['- audit'],
          },
          {
            'kind': 'delete',
            'semanticLabel': 'Plan',
            'newStart': 10,
            'before': ['obsolete'],
            'after': <String>[],
          },
        ],
        'changesTruncated': false,
      },
    );
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SingleChildScrollView(
            child: MuseChangeAuditEventCard(event: comparison),
          ),
        ),
      ),
    );
    await tester.pump();

    expect(find.text('DSH Agent 的变更'), findsOneWidget);
    expect(find.text('AGENT'), findsOneWidget);
    expect(find.text('新增 1'), findsOneWidget);
    expect(find.text('删除 1'), findsOneWidget);
    expect(find.text('修改 1'), findsOneWidget);
    expect(find.text('Expanded scope'), findsOneWidget);
    expect(find.text('Original scope'), findsOneWidget);
    expect(find.text('- audit'), findsOneWidget);
    expect(find.text('obsolete'), findsOneWidget);
  });
}
