import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

void main() {
  for (final choose in [true, false]) {
    testWidgets(
      'position sheet preserves original input focus on ${choose ? "select" : "dismiss"}',
      (t) async {
        final focus = FocusNode();
        final text = TextEditingController(text: '原本地正文');
        final changes = ChangeNotifier();
        var called = 0;
        await t.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Column(
                children: [
                  // Match document input: refocus preserves a caret rather than
                  // the browser TextField default of selecting all text.
                  TextField(
                    focusNode: focus,
                    controller: text,
                    selectAllOnFocus: false,
                  ),
                  CollaboratorsButton(
                    listenable: changes,
                    people: () => const [
                      CollaborationPerson(
                        uid: 'bob',
                        sessionId: 'remote',
                        name: 'Bob',
                        location: '第 38 块',
                      ),
                    ],
                    connected: () => true,
                    selfUid: () => 'alice',
                    selfSessionId: 'local',
                    canSelectPosition: (_) => true,
                    onPositionSelected: (_) async {
                      called++;
                    },
                  ),
                ],
              ),
            ),
          ),
        );
        await t.tap(find.byType(TextField));
        await t.pump();
        text.selection = const TextSelection.collapsed(offset: 2);
        expect(focus.hasFocus, isTrue);
        await t.tap(find.text('2 人'));
        await t.pumpAndSettle();
        if (choose) {
          await t.tap(find.byKey(const ValueKey('collaboration.member.bob')));
        } else {
          await t.tapAt(const Offset(5, 5));
        }
        await t.pumpAndSettle();
        expect(called, choose ? 1 : 0);
        expect(text.text, '原本地正文');
        expect(text.selection, const TextSelection.collapsed(offset: 2));
        expect(
          focus.hasFocus,
          isTrue,
          reason:
              'Position navigation must leave input with the original editor',
        );
        await t.pumpWidget(const SizedBox());
        focus.dispose();
        text.dispose();
        changes.dispose();
      },
      variant: TargetPlatformVariant({
        TargetPlatform.macOS,
        TargetPlatform.android,
        TargetPlatform.iOS,
      }),
    );
  }
  testWidgets(
    'keyboard users can enter the position sheet and activate a location',
    (t) async {
      final changes = ChangeNotifier();
      var called = 0;
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: CollaboratorsButton(
              listenable: changes,
              people: () => const [
                CollaborationPerson(
                  uid: 'bob',
                  sessionId: 'remote',
                  name: 'Bob',
                  location: '第 38 块',
                ),
              ],
              connected: () => true,
              selfUid: () => 'alice',
              selfSessionId: 'local',
              canSelectPosition: (_) => true,
              onPositionSelected: (_) async {
                called++;
              },
            ),
          ),
        ),
      );
      await t.sendKeyEvent(LogicalKeyboardKey.tab);
      await t.pump();
      await t.sendKeyEvent(LogicalKeyboardKey.enter);
      await t.pumpAndSettle();
      expect(find.text('在线协作者'), findsOneWidget);
      var targetFocused = false;
      for (var step = 0; step < 6; step++) {
        await t.sendKeyEvent(LogicalKeyboardKey.tab);
        await t.pump();
        if (Focus.of(t.element(find.text('Bob'))).hasFocus) {
          targetFocused = true;
          break;
        }
      }
      expect(
        targetFocused,
        isTrue,
        reason: 'Position action remains reachable by keyboard',
      );
      await t.sendKeyEvent(LogicalKeyboardKey.enter);
      await t.pumpAndSettle();
      expect(called, 1);
      expect(find.text('在线协作者'), findsNothing);
      await t.pumpWidget(const SizedBox());
      changes.dispose();
    },
    variant: TargetPlatformVariant({TargetPlatform.macOS}),
  );
}
