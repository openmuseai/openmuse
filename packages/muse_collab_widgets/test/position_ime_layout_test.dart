import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

void main() {
  testWidgets(
    'native position action remains visible and usable above an open keyboard',
    (t) async {
      t.view.devicePixelRatio = 1;
      t.view.physicalSize = const Size(360, 800);
      t.view.viewInsets = const FakeViewPadding(bottom: 300);
      addTearDown(t.view.resetDevicePixelRatio);
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetViewInsets);
      final focus = FocusNode();
      final text = TextEditingController(text: '原光标正文');
      final changed = ChangeNotifier();
      var called = 0;
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Column(
              children: [
                TextField(
                  focusNode: focus,
                  controller: text,
                  selectAllOnFocus: false,
                ),
                CollaboratorsButton(
                  compact: true,
                  listenable: changed,
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
      await t.tap(find.text('2 人'));
      await t.pumpAndSettle();
      final action = find.byKey(const ValueKey('collaboration.member.bob'));
      final rect = t.getRect(action);
      expect(rect.top, greaterThanOrEqualTo(0));
      expect(
        rect.bottom,
        lessThanOrEqualTo(500),
        reason: 'A semantics-only action behind the 300px keyboard is unusable',
      );
      await t.tap(action);
      await t.pumpAndSettle();
      expect(called, 1);
      expect(text.text, '原光标正文');
      expect(text.selection, const TextSelection.collapsed(offset: 2));
      expect(focus.hasFocus, isTrue);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox());
      focus.dispose();
      text.dispose();
      changed.dispose();
    },
    skip: kIsWeb,
    variant: TargetPlatformVariant({
      TargetPlatform.android,
      TargetPlatform.iOS,
    }),
  );
}
