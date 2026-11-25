import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  for (final inlineCode in [false, true]) {
    testWidgets(
      'file ${inlineCode ? 'inline code' : 'link'} routes original hint and Session to owner',
      (tester) async {
        const target = 'src/main.dart:12:8';
        final store = DshConversationStore()
          ..apply(
            DshSnapshotFrame(
              header: const {'id': 's-file'},
              cursor: 1,
              records: [
                DshWireEvent.fromJson({
                  'type': 'assistant/message',
                  'seq': 1,
                  'time': 1,
                  'data': {
                    'message': {
                      'content': [
                        {
                          'type': 'text',
                          'text': inlineCode ? '`$target`' : '[Open]($target)',
                        },
                      ],
                    },
                  },
                }),
              ],
              hasMore: false,
              projections: const {},
            ),
          );
        ResourceLocator? opened;
        await tester.pumpWidget(
          MaterialApp(
            locale: const Locale('en'),
            supportedLocales: const [Locale('en')],
            localizationsDelegates: [
              OpenMuseStringsDelegate(OpenMuseCatalog.builtIn),
              ...GlobalMaterialLocalizations.delegates,
            ],
            home: ResourceLinkRouting(
              open: (_) async {},
              locate: (l) async {
                opened = l;
              },
              child: Scaffold(
                body: DshConversationSurface(
                  store: store,
                  showComposer: false,
                  onSend: (text, {mode = 'queue'}) async {},
                  onCancel: () async {},
                  onFallbackRequested: () {},
                ),
              ),
            ),
          ),
        );
        await tester.pumpAndSettle();
        await tester.tap(
          inlineCode
              ? find.byKey(const ValueKey('dsh-file-hint-$target'))
              : find.text('Open', findRichText: true),
        );
        await tester.pump();
        expect(opened?.owner, 'dsh');
        expect(opened?.target, target);
        expect(opened?.context['sessionId'], 's-file');
      },
    );
  }
  test('external URLs are not interpreted as host files', () {
    expect(isDshFileHint('https://example.com/a.dart'), false);
    expect(isDshFileHint('file:///workspace/a.dart#L12'), true);
    expect(isDshFileHint('dsh-resource://file/session/s/a.dart'), true);
    expect(isDshFileHint('void main() {}'), false);
  });
}
