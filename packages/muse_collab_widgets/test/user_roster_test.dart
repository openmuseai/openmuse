import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

void main() {
  for (final width in [320.0, 430.0, 1100.0]) {
    testWidgets(
      'same-user windows count once and live leave updates expanded locations at $width',
      (t) async {
        t.view.physicalSize = Size(width, 850);
        t.view.devicePixelRatio = 1;
        addTearDown(t.view.resetPhysicalSize);
        addTearDown(t.view.resetDevicePixelRatio);
        final changed = ChangeNotifier();
        String? self = 'alice';
        var online = true;
        var people = [
          const CollaborationPerson(
            uid: 'alice',
            sessionId: 'alice-other',
            name: 'Alice',
            location: '第 2 块',
          ),
          const CollaborationPerson(
            uid: 'bob',
            sessionId: 'bob-one',
            name: 'Bob',
            location: '表格 · 标题',
          ),
          const CollaborationPerson(
            uid: 'bob',
            sessionId: 'bob-two',
            name: 'Bob',
            location: '第 3 块',
          ),
        ];
        await t.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Center(
                child: CollaboratorsButton(
                  listenable: changed,
                  people: () => people,
                  connected: () => online,
                  selfUid: () => self,
                  selfSessionId: 'alice-current',
                ),
              ),
            ),
          ),
        );
        expect(find.text('2 人'), findsOneWidget);
        await t.tap(find.text('2 人'));
        await t.pumpAndSettle();
        expect(find.text('我'), findsOneWidget);
        expect(find.text('Bob'), findsOneWidget);
        expect(find.text('2 处在线'), findsNWidgets(2));
        await t.tap(find.byKey(const ValueKey('collaboration.member.bob')));
        await t.pumpAndSettle();
        expect(find.text('表格 · 标题'), findsOneWidget);
        expect(find.text('第 3 块'), findsOneWidget);
        people = people.where((p) => p.sessionId != 'bob-one').toList();
        changed.notifyListeners();
        await t.pumpAndSettle();
        expect(find.text('2 人'), findsOneWidget);
        expect(find.text('Bob'), findsOneWidget);
        expect(find.text('第 3 块'), findsOneWidget);
        people = people.where((p) => p.uid != 'bob').toList();
        changed.notifyListeners();
        await t.pumpAndSettle();
        expect(find.text('1 人'), findsOneWidget);
        expect(find.text('Bob'), findsNothing);
        self = null;
        changed.notifyListeners();
        await t.pumpAndSettle();
        expect(find.text('1 人'), findsNothing);
        expect(find.text('身份尚未确认，在线人数待确认'), findsOneWidget);
        online = false;
        people = [];
        changed.notifyListeners();
        await t.pumpAndSettle();
        expect(find.text('正在重新连接，本机修改仍保留'), findsOneWidget);
        expect(t.takeException(), isNull);
        await t.pumpWidget(const SizedBox());
        changed.dispose();
      },
    );
  }
  testWidgets(
    'conflicting slot ownership shows unavailable roster instead of a fabricated count',
    (t) async {
      final changed = ChangeNotifier();
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: CollaboratorsButton(
              listenable: changed,
              people: () => const [
                CollaborationPerson(
                  uid: 'bob',
                  sessionId: 'shared',
                  name: 'Bob',
                ),
                CollaborationPerson(
                  uid: 'mallory',
                  sessionId: 'shared',
                  name: 'Spoof',
                ),
              ],
              connected: () => true,
              selfUid: () => 'alice',
              selfSessionId: 'self',
            ),
          ),
        ),
      );
      expect(find.text('在线协作'), findsOneWidget);
      await t.tap(find.text('在线协作'));
      await t.pumpAndSettle();
      expect(find.text('在线名单暂不可用'), findsOneWidget);
      expect(find.text('3 人'), findsNothing);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox());
      changed.dispose();
    },
  );
}
