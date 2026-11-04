import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

const _origin = String.fromEnvironment('DSH_E2E_ORIGIN');
const _bootstrapPath = String.fromEnvironment('DSH_E2E_BOOTSTRAP_PATH');
const _bridgeToken = String.fromEnvironment('DSH_E2E_BRIDGE_TOKEN');
const _sessionId = String.fromEnvironment('DSH_E2E_SESSION_ID');
const _holdMilliseconds = int.fromEnvironment(
  'DSH_E2E_HOLD_MS',
  defaultValue: 0,
);
const _desktopPrompt = String.fromEnvironment(
  'DSH_E2E_DESKTOP_PROMPT',
  defaultValue: 'Desktop 与 Mobile 原生对话流对齐验证',
);
const _mobilePrompt = String.fromEnvironment(
  'DSH_E2E_MOBILE_PROMPT',
  defaultValue: 'Mobile 原生回传到 Desktop 验证',
);
const _assistantMarker = String.fromEnvironment(
  'DSH_E2E_ASSISTANT_MARKER',
  defaultValue: 'E2E_NATIVE_SYNC_OK',
);

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('real Desktop DSH session synchronizes both ways', (
    tester,
  ) async {
    expect(_origin, isNotEmpty, reason: 'DSH_E2E_ORIGIN is required');
    expect(
      _bootstrapPath,
      isNotEmpty,
      reason: 'DSH_E2E_BOOTSTRAP_PATH is required',
    );
    expect(
      _bridgeToken,
      isNotEmpty,
      reason: 'DSH_E2E_BRIDGE_TOKEN is required',
    );
    expect(_sessionId, isNotEmpty, reason: 'DSH_E2E_SESSION_ID is required');

    final client = DshNativeGatewayClient(
      origin: Uri.parse(_origin),
      bootstrapPath: _bootstrapPath,
      allowInsecureLoopback: true,
      headers: {'x-openmuse-bridge-token': _bridgeToken},
    );
    final controller = DshConversationController(client: client);
    addTearDown(controller.dispose);

    final hello = await client.initialize();
    expect(hello.supported, isTrue);
    final negotiation = await client.negotiate(
      DshNativeCapabilities.standard(),
    );
    expect(negotiation.requiresWebFallback, isFalse);
    final sessions = await client.listSessions();
    expect(sessions.map((session) => session.sessionId), contains(_sessionId));

    await controller.start(_sessionId);
    await _waitUntil(
      () => controller.store.snapshot.phase == DshConnectionPhase.live,
      description: 'initial DSH snapshot',
    );
    var rows = controller.store.snapshot.rows;
    expect(rows.map((row) => row.text), contains(_desktopPrompt));
    expect(
      rows.any(
        (row) =>
            row.kind == DshConversationRowKind.assistant &&
            row.text.contains(_assistantMarker),
      ),
      isTrue,
      reason: 'Desktop prompt must already have a durable assistant answer',
    );
    expect(
      rows.any((row) => row.text.contains('Current runtime context')),
      isFalse,
      reason: 'runtime-context messages are not visible conversation rows',
    );
    expect(
      rows.where((row) => row.kind == DshConversationRowKind.error),
      isEmpty,
    );
    final assistantAnswersBefore = rows
        .where(
          (row) =>
              row.kind == DshConversationRowKind.assistant &&
              row.text.contains(_assistantMarker),
        )
        .length;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DshConversationSurface(
            store: controller.store,
            nativeContributions: negotiation.contributions,
            onSend: controller.send,
            onCancel: controller.cancel,
            onFallbackRequested: () => fail('unexpected Web fallback'),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text(_desktopPrompt), findsOneWidget);
    expect(find.textContaining(_assistantMarker), findsWidgets);

    await tester.enterText(
      find.byKey(const ValueKey('dsh-native.composer')),
      _mobilePrompt,
    );
    await tester.tap(find.byKey(const ValueKey('dsh-native.send')));
    await tester.pump();
    await _waitUntil(
      () {
        rows = controller.store.snapshot.rows;
        final durableMobilePrompt = rows.any(
          (row) =>
              row.kind == DshConversationRowKind.user &&
              row.text == _mobilePrompt,
        );
        final assistantAnswers = rows
            .where(
              (row) =>
                  row.kind == DshConversationRowKind.assistant &&
                  row.text.contains(_assistantMarker),
            )
            .length;
        return durableMobilePrompt && assistantAnswers > assistantAnswersBefore;
      },
      description: 'durable Mobile prompt and assistant answer in DSH journal',
    );
    await tester.pumpAndSettle();

    expect(find.text(_mobilePrompt), findsOneWidget);
    expect(
      controller.store.snapshot.rows
          .where((row) => row.text == _mobilePrompt)
          .length,
      1,
      reason: 'optimistic echo retires when DSH commits the prompt',
    );
    if (_holdMilliseconds > 0) {
      await Future<void>.delayed(Duration(milliseconds: _holdMilliseconds));
    }
  });
}

Future<void> _waitUntil(
  bool Function() predicate, {
  required String description,
  Duration timeout = const Duration(seconds: 20),
}) async {
  final deadline = DateTime.now().add(timeout);
  while (!predicate()) {
    if (DateTime.now().isAfter(deadline)) {
      throw TimeoutException('Timed out waiting for $description', timeout);
    }
    await Future<void>.delayed(const Duration(milliseconds: 100));
  }
}
