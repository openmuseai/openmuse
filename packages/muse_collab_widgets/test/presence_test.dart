import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_collab_widgets/muse_collab_widgets.dart';

void main() {
  testWidgets(
    'shared presence entry and live member sheet stay bounded on mobile with many sessions',
    (t) async {
      t.view.physicalSize = const Size(390, 750);
      t.view.devicePixelRatio = 1;
      addTearDown(t.view.resetPhysicalSize);
      addTearDown(t.view.resetDevicePixelRatio);
      final changed = ChangeNotifier();
      var online = true;
      var people = List.generate(
        40,
        (i) => CollaborationPerson(
          name: '成员 $i',
          uid: '$i',
          sessionId: 'remote-$i',
        ),
      );
      await t.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Center(
              child: CollaboratorsButton(
                listenable: changed,
                selfUid: () => online ? 'self' : null,
                selfSessionId: 'self-window',
                people: () => people,
                connected: () => online,
              ),
            ),
          ),
        ),
      );
      expect(find.text('41 人'), findsOneWidget);
      await t.tap(find.text('41 人'));
      await t.pumpAndSettle();
      expect(find.text('在线协作者'), findsOneWidget);
      expect(t.takeException(), isNull);
      people = [];
      online = false;
      changed.notifyListeners();
      await t.pumpAndSettle();
      expect(find.text('暂无其他在线协作者'), findsOneWidget);
      expect(find.text('正在重新连接，本机修改仍保留'), findsOneWidget);
      expect(t.takeException(), isNull);
      await t.pumpWidget(const SizedBox());
      changed.dispose();
    },
  );
}
