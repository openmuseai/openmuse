import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

DshSnapshotFrame page(int first, int last, bool more) => DshSnapshotFrame(
  header: const {'id': 's'},
  cursor: 30,
  hasMore: more,
  projections: const {},
  records: [
    for (var i = first; i <= last; i++)
      DshWireEvent.fromJson({
        'type': 'user/message',
        'seq': i,
        'time': i,
        'data': {
          'content': [
            {'type': 'text', 'text': 'message $i'},
          ],
        },
      }),
  ],
);
void main() {
  testWidgets(
    'initial tail does not fetch history; scrolling up prepends once',
    (tester) async {
      final store = DshConversationStore()..apply(page(10, 30, true));
      var loads = 0;
      await tester.pumpWidget(
        MaterialApp(
          locale: const Locale('en'),
          supportedLocales: const [Locale('en')],
          localizationsDelegates: [
            OpenMuseStringsDelegate(OpenMuseCatalog.builtIn),
            ...GlobalMaterialLocalizations.delegates,
          ],
          home: Scaffold(
            body: DshConversationSurface(
              store: store,
              showComposer: false,
              onSend: (text, {mode = 'queue'}) async {},
              onCancel: () async {},
              onFallbackRequested: () {},
              onLoadOlder: () async {
                loads++;
                store.prepend(page(1, 9, false));
              },
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      expect(loads, 0);
      await tester.drag(
        find.byKey(const ValueKey('dsh-native.timeline')),
        const Offset(0, 5000),
      );
      await tester.pumpAndSettle();
      expect(loads, 1);
      expect(store.snapshot.cursor, 30);
      expect(store.snapshot.rows.length, 30);
    },
  );
}
