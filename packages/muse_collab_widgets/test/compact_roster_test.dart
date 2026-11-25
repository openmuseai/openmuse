import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

void main() {
  testWidgets('bounded compact entry preserves identities and live roster', (
    t,
  ) async {
    t.view.devicePixelRatio = 1;
    t.view.physicalSize = const Size(320, 420);
    addTearDown(t.view.resetDevicePixelRatio);
    addTearDown(t.view.resetPhysicalSize);
    final changed = ChangeNotifier();
    String? self = 'alice';
    var online = true;
    var people = [
      const CollaborationPerson(
        uid: 'alice',
        sessionId: 'other',
        name: 'Alice',
      ),
      const CollaborationPerson(uid: 'bob', sessionId: 'bob', name: 'Bob'),
      const CollaborationPerson(
        uid: 'carol',
        sessionId: 'carol',
        name: 'Carol',
      ),
    ];
    var closes = 0;
    await t.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Row(
            children: [
              const SizedBox(width: 192),
              Expanded(
                child: CollaboratorsButton(
                  compact: true,
                  listenable: changed,
                  people: () => people,
                  connected: () => online,
                  selfUid: () => self,
                  selfSessionId: 'current',
                ),
              ),
              IconButton(
                tooltip: 'Close',
                onPressed: () => closes++,
                icon: const Icon(Icons.close),
              ),
            ],
          ),
        ),
      ),
    );
    expect(find.text('3 人'), findsOneWidget);
    expect(t.takeException(), isNull);
    final entry = find.byType(TextButton);
    expect(t.getSize(entry).width, greaterThanOrEqualTo(44));
    expect(t.getRect(find.byTooltip('Close')).right, lessThanOrEqualTo(320));
    await t.tap(find.text('3 人'));
    await t.pumpAndSettle();
    expect(find.text('我'), findsOneWidget);
    expect(find.text('2 处在线'), findsOneWidget);
    expect(find.text('Bob'), findsOneWidget);
    await t.scrollUntilVisible(
      find.text('Carol'),
      80,
      scrollable: find.descendant(
        of: find.byType(ListView),
        matching: find.byType(Scrollable),
      ),
    );
    expect(find.text('Carol'), findsOneWidget);
    people = people.where((p) => p.uid != 'bob').toList();
    changed.notifyListeners();
    await t.pumpAndSettle();
    expect(find.text('2 人'), findsOneWidget);
    expect(find.text('Bob'), findsNothing);
    Navigator.of(t.element(find.text('在线协作者'))).pop();
    await t.pumpAndSettle();
    self = null;
    online = false;
    people = [];
    changed.notifyListeners();
    await t.pumpAndSettle();
    expect(find.text('在线协作'), findsOneWidget);
    expect(find.text('0 人'), findsNothing);
    expect(t.takeException(), isNull);
    await t.tap(find.byTooltip('Close'));
    expect(closes, 1);
    await t.tap(find.text('在线协作'));
    await t.pumpAndSettle();
    expect(find.text('正在重新连接，本机修改仍保留'), findsOneWidget);
    expect(t.takeException(), isNull);
    await t.pumpWidget(const SizedBox());
    changed.dispose();
  });
}
