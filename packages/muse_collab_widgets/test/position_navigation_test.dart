import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

class _Observer extends NavigatorObserver {
  VoidCallback? afterPop;
  @override
  void didPop(Route<dynamic> route, Route<dynamic>? previousRoute) =>
      afterPop?.call();
}

void main() {
  testWidgets(
    'same-user remote position is selectable, local and unavailable positions stay inert',
    (t) async {
      final changes = ChangeNotifier();
      var connected = true;
      final selected = <String>[];
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: CollaboratorsButton(
              listenable: changes,
              people: () => const [
                CollaborationPerson(
                  uid: 'alice',
                  sessionId: 'other-device',
                  name: 'Alice',
                  location: '第 3 块',
                ),
                CollaborationPerson(
                  uid: 'bob',
                  sessionId: 'no-anchor',
                  name: 'Bob',
                ),
              ],
              connected: () => connected,
              selfUid: () => 'alice',
              selfSessionId: 'local',
              canSelectPosition: (p) => p.sessionId == 'other-device',
              onPositionSelected: (p) async =>
                  selected.add('${p.uid}/${p.sessionId}'),
            ),
          ),
        ),
      );
      await t.tap(find.text('2 人'));
      await t.pumpAndSettle();
      expect(
        t
            .widget<ListTile>(
              find.byKey(const ValueKey('collaboration.member.bob')),
            )
            .onTap,
        isNull,
      );
      await t.tap(find.text('2 处在线'));
      await t.pumpAndSettle();
      ListTile tile(String id) => t.widget<ListTile>(
        find.byKey(ValueKey('collaboration.position.$id')),
      );
      expect(tile('local').onTap, isNull);
      expect(tile('other-device').onTap, isNotNull);
      await t.tap(
        find.byKey(const ValueKey('collaboration.position.other-device')),
      );
      await t.pumpAndSettle();
      expect(selected, ['alice/other-device']);
      expect(find.text('在线协作者'), findsNothing);
      connected = false;
      changes.notifyListeners();
      await t.pump();
      await t.tap(find.text('在线协作'));
      await t.pumpAndSettle();
      expect(
        t
            .widget<ListTile>(
              find.byKey(const ValueKey('collaboration.member.alice')),
            )
            .onTap,
        isNull,
      );
      expect(selected, hasLength(1));
      await t.pumpWidget(const SizedBox());
      changes.dispose();
    },
  );

  testWidgets(
    'slot removed while the sheet dismisses cannot invoke a stale location',
    (t) async {
      final changes = ChangeNotifier(), observer = _Observer();
      var online = const [
        CollaborationPerson(
          uid: 'bob',
          sessionId: 'remote',
          name: 'Bob',
          location: '第 9 块',
        ),
      ];
      var called = 0;
      await t.pumpWidget(
        MaterialApp(
          navigatorObservers: [observer],
          home: Scaffold(
            body: CollaboratorsButton(
              listenable: changes,
              people: () => online,
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
      await t.tap(find.text('2 人'));
      await t.pumpAndSettle();
      observer.afterPop = () {
        online = const [];
      };
      await t.tap(find.byKey(const ValueKey('collaboration.member.bob')));
      await t.pumpAndSettle();
      expect(called, 0);
      expect(find.text('在线协作者'), findsNothing);
      expect(find.text('该在线位置暂不可用'), findsOneWidget);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox());
      changes.dispose();
    },
  );
}
