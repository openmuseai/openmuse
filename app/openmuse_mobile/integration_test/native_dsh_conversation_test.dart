import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('renders a Desktop journal and native plugin contribution', (
    tester,
  ) async {
    final store = DshConversationStore();
    addTearDown(store.close);
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 'session-mobile-integration'},
        cursor: 3,
        records: [
          DshWireEvent.fromJson({
            'type': 'user/message',
            'seq': 1,
            'time': 1,
            'surfaceOp': 'append',
            'data': {
              'content': [
                {'type': 'text', 'text': '从 Desktop 发送'},
              ],
            },
          }),
          DshWireEvent.fromJson({
            'type': 'assistant/message',
            'seq': 2,
            'time': 2,
            'surfaceOp': 'append',
            'data': {
              'message': {
                'content': [
                  {'type': 'text', 'text': '已同步到 Flutter 原生对话流'},
                ],
              },
            },
          }),
          DshWireEvent.fromJson({
            'type': 'tool/call',
            'seq': 3,
            'time': 3,
            'surfaceOp': 'append',
            'data': {
              'turn': 1,
              'step': 1,
              'callId': 'weather-call-1',
              'name': 'weather',
              'arguments': '{"city":"上海"}',
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );

    final contribution = DshNativeContribution.fromJson({
      'slot': 'tool.call.toolview',
      'key': 'weather',
      'template': 'toolCard',
      'title': {'bind': 'tool.arguments.city'},
      'body': [
        {
          'component': 'keyValue',
          'label': '城市',
          'value': {'bind': 'tool.arguments.city'},
        },
        {'component': 'badge', 'text': '来自声明式原生 UI', 'tone': 'info'},
      ],
      'actions': <Object?>[],
    });
    String? sent;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DshConversationSurface(
            store: store,
            nativeContributions: [contribution],
            onSend: (text, {mode = 'queue'}) async => sent = text,
            onCancel: () async {},
            onFallbackRequested: () {},
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('从 Desktop 发送'), findsOneWidget);
    expect(find.text('已同步到 Flutter 原生对话流'), findsOneWidget);
    expect(find.text('上海'), findsNWidgets(2));
    expect(find.text('来自声明式原生 UI'), findsOneWidget);
    expect(
      find.byKey(const ValueKey('native-ui:tool.call.toolview:weather')),
      findsOneWidget,
    );

    await tester.enterText(
      find.byKey(const ValueKey('dsh-native.composer')),
      '从 Mobile 原生 Composer 发送',
    );
    await tester.tap(find.byKey(const ValueKey('dsh-native.send')));
    await tester.pump();

    expect(sent, '从 Mobile 原生 Composer 发送');
  });
}
