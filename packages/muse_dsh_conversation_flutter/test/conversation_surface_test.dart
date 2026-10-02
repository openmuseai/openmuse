import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

void main() {
  testWidgets('renders Desktop journal and sends from native composer', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: {'id': 's-1'},
        cursor: 1,
        records: [
          DshWireEvent.fromJson({
            'type': 'assistant/message',
            'seq': 1,
            'time': 1,
            'surfaceOp': 'append',
            'data': {
              'message': {
                'content': [
                  {'type': 'text', 'text': 'Desktop reply'},
                ],
              },
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );
    String? sent;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DshConversationSurface(
            store: store,
            onSend: (text, {mode = 'queue'}) async => sent = text,
            onCancel: () async {},
            onFallbackRequested: () {},
          ),
        ),
      ),
    );
    expect(find.text('Desktop reply'), findsOneWidget);
    await tester.enterText(
      find.byKey(const ValueKey('dsh-native.composer')),
      'Mobile prompt',
    );
    await tester.tap(find.byKey(const ValueKey('dsh-native.send')));
    await tester.pump();
    expect(sent, 'Mobile prompt');
  });

  testWidgets(
    'embedded conversation keeps timeline and composer without a duplicate header',
    (tester) async {
      final store = DshConversationStore();
      store.apply(
        DshSnapshotFrame(
          header: const {'id': 's-embedded'},
          cursor: 0,
          records: const [],
          hasMore: false,
          projections: const {},
        ),
      );
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            appBar: AppBar(title: const Text('Workspace header')),
            body: DshConversationSurface(
              store: store,
              showHeader: false,
              onSend: (_, {mode = 'queue'}) async {},
              onCancel: () async {},
              onFallbackRequested: () {},
            ),
          ),
        ),
      );

      expect(find.text('Workspace header'), findsOneWidget);
      expect(find.byKey(const ValueKey('dsh-native.header')), findsNothing);
      expect(find.byKey(const ValueKey('dsh-native.composer')), findsOneWidget);
    },
  );

  testWidgets('unknown required event stays in native flow with optional Web', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      DshEventFrame(
        DshWireEvent.fromJson({
          'type': 'plugin/chat-side-effect',
          'seq': 1,
          'time': 1,
          'data': <String, Object?>{},
        }),
      ),
    );
    var fallback = false;
    await tester.pumpWidget(
      MaterialApp(
        home: DshConversationSurface(
          store: store,
          onSend: (_, {mode = 'queue'}) async {},
          onCancel: () async {},
          onFallbackRequested: () => fallback = true,
        ),
      ),
    );
    expect(find.text('Mobile 暂不支持此 DSH 元素'), findsOneWidget);
    expect(find.byKey(const ValueKey('dsh-native.timeline')), findsOneWidget);
    await tester.tap(
      find.byKey(const ValueKey('dsh-native.open-web:incompatible:1')),
    );
    expect(fallback, isTrue);
  });

  testWidgets('embedded timeline can use the host composer only', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      const DshSnapshotFrame(
        header: {'id': 's-host-composer'},
        cursor: 0,
        records: [],
        hasMore: false,
        projections: {},
      ),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: DshConversationSurface(
          store: store,
          showHeader: false,
          showComposer: false,
          onSend: (_, {mode = 'queue'}) async {},
          onCancel: () async {},
          onFallbackRequested: () {},
        ),
      ),
    );
    expect(find.byKey(const ValueKey('dsh-native.timeline')), findsNothing);
    expect(find.byKey(const ValueKey('dsh-native.composer')), findsNothing);
    expect(find.text('探索未至之境'), findsOneWidget);
  });

  testWidgets('renders a negotiated tool contribution with safe bindings', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      DshEventFrame(
        DshWireEvent.fromJson({
          'type': 'tool/call',
          'seq': 1,
          'time': 1,
          'data': {
            'turn': 1,
            'step': 1,
            'callId': 'call-1',
            'name': 'weather',
            'arguments': '{"city":"上海"}',
          },
        }),
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
      ],
      'actions': <Object?>[],
    });
    await tester.pumpWidget(
      MaterialApp(
        home: DshConversationSurface(
          store: store,
          nativeContributions: [contribution],
          onSend: (_, {mode = 'queue'}) async {},
          onCancel: () async {},
          onFallbackRequested: () {},
        ),
      ),
    );
    expect(find.text('上海'), findsNWidgets(2));
    expect(
      find.byKey(const ValueKey('native-ui:tool.call.toolview:weather')),
      findsOneWidget,
    );
  });

  testWidgets('renders ask_user_question as a native question panel', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      DshEventFrame(
        DshWireEvent.fromJson({
          'type': 'tool/call',
          'seq': 1,
          'time': 1,
          'data': {
            'turn': 1,
            'step': 1,
            'callId': 'ask-1',
            'name': 'ask_user_question',
            'arguments':
                '{"questions":[{"id":"mode","header":"模式","question":"选择运行模式","options":[{"label":"安全","description":"只读执行"},{"label":"完整","description":"允许写入"}]}]}',
          },
        }),
      ),
    );
    List<JsonMap>? answered;
    await tester.pumpWidget(
      MaterialApp(
        home: DshConversationSurface(
          store: store,
          onQuestionAnswer: (answers) async => answered = answers,
          onSend: (_, {mode = 'queue'}) async {},
          onCancel: () async {},
          onFallbackRequested: () {},
        ),
      ),
    );

    expect(
      find.byKey(const ValueKey('dsh-native.question:event:1')),
      findsOneWidget,
    );
    expect(find.text('需要你的回答'), findsOneWidget);
    expect(find.text('选择运行模式'), findsOneWidget);
    expect(find.text('安全'), findsOneWidget);
    expect(find.text('只读执行'), findsOneWidget);
    await tester.tap(
      find.byKey(const ValueKey('dsh-native.question.option:mode:安全')),
    );
    await tester.pump();
    await tester.ensureVisible(
      find.byKey(const ValueKey('dsh-native.question.submit')),
    );
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('dsh-native.question.submit')));
    await tester.pump();
    expect(answered, [
      {
        'id': 'mode',
        'selected': ['安全'],
      },
    ]);
    expect(find.text('回答已发送到 Desktop，正在继续执行。'), findsOneWidget);
  });

  testWidgets(
    'assistant message renders markdown blocks instead of raw syntax',
    (tester) async {
      final store = DshConversationStore();
      store.apply(
        DshEventFrame(
          DshWireEvent.fromJson({
            'type': 'assistant/message',
            'seq': 1,
            'time': 1,
            'surfaceOp': 'append',
            'data': {
              'message': {
                'content': [
                  {'type': 'text', 'text': '# 标题\n\n- **重点**\n- `code`'},
                ],
              },
            },
          }),
        ),
      );
      await tester.pumpWidget(
        MaterialApp(
          home: DshConversationSurface(
            store: store,
            onSend: (_, {mode = 'queue'}) async {},
            onCancel: () async {},
            onFallbackRequested: () {},
          ),
        ),
      );
      expect(find.byKey(const ValueKey('dsh-native.markdown')), findsOneWidget);
      expect(find.textContaining('**重点**'), findsNothing);
    },
  );

  testWidgets('generic completed tool keeps both input and output', (
    tester,
  ) async {
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-tools'},
        cursor: 2,
        records: [
          DshWireEvent.fromJson({
            'type': 'tool/call',
            'seq': 1,
            'time': 1,
            'data': {
              'callId': 'call-1',
              'name': 'weather',
              'arguments': '{"city":"上海"}',
            },
          }),
          DshWireEvent.fromJson({
            'type': 'tool/result',
            'seq': 2,
            'time': 2,
            'data': {
              'message': {
                'toolCallId': 'call-1',
                'isError': false,
                'content': [
                  {'type': 'text', 'text': '{"temperature":26}'},
                ],
              },
            },
          }),
        ],
        hasMore: false,
        projections: const {},
      ),
    );
    await tester.pumpWidget(
      MaterialApp(
        home: DshConversationSurface(
          store: store,
          onSend: (_, {mode = 'queue'}) async {},
          onCancel: () async {},
          onFallbackRequested: () {},
        ),
      ),
    );

    expect(find.text('输入'), findsNothing);
    await tester.tap(find.text('weather 完成'));
    await tester.pumpAndSettle();
    expect(find.text('输入'), findsOneWidget);
    expect(find.textContaining('上海'), findsOneWidget);
    expect(find.text('输出'), findsOneWidget);
    expect(find.textContaining('temperature'), findsOneWidget);
  });

  testWidgets('workspace changes renders a native artifact and opens a file', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(430, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final store = DshConversationStore();
    store.apply(
      DshEventFrame(
        DshWireEvent.fromJson({
          'type': 'workspace/changes',
          'seq': 7,
          'time': 1,
          'data': {'turn': 2},
        }),
      ),
    );
    (int, int)? opened;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DshConversationSurface(
            store: store,
            onLoadWorkspaceChanges: (seq) async {
              expect(seq, 7);
              return const DshNativeWorkspaceChanges(
                turn: 2,
                total: 1,
                added: 5,
                deleted: 1,
                files: [
                  DshNativeChangedFile(
                    display: 'assets/README.md',
                    added: 5,
                    deleted: 1,
                    binary: false,
                    oversized: false,
                  ),
                ],
              );
            },
            onOpenWorkspaceChange: (seq, index) async => opened = (seq, index),
            onLoadWorkspaceChangePreview: (seq, index) async {
              expect((seq, index), (7, 0));
              return const DshNativeArtifactPreview(
                kind: DshNativeArtifactPreviewKind.text,
                display: 'assets/README.md',
                before: false,
                after: true,
                coarse: false,
                hunks: [
                  DshNativeDiffHunk(
                    oldStart: 1,
                    oldLines: 0,
                    newStart: 1,
                    newLines: 2,
                    lines: ['+# Artifact', '+Mobile preview'],
                  ),
                ],
              );
            },
            onSend: (_, {mode = 'queue'}) async {},
            onCancel: () async {},
            onFallbackRequested: () {},
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('已更新 1 个文件'), findsOneWidget);
    expect(find.text('assets/README.md'), findsOneWidget);
    expect(find.text('Mobile 暂不支持此 DSH 元素'), findsNothing);
    await tester.tap(find.text('assets/README.md'));
    await tester.pumpAndSettle();
    expect(find.text('产物预览'), findsOneWidget);
    expect(find.text('本轮新建的文件'), findsOneWidget);
    expect(
      find.byKey(const ValueKey('dsh-native.artifact-preview.unified')),
      findsOneWidget,
    );
    expect(find.text('# Artifact'), findsOneWidget);
    await tester.tap(find.byTooltip('双栏对比'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('dsh-native.artifact-preview.split')),
      findsOneWidget,
    );
    await tester.tap(find.byTooltip('单栏对比'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('dsh-native.artifact-preview.unified')),
      findsOneWidget,
    );
    await tester.tap(find.text('预览'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey('dsh-native.artifact-preview.markdown')),
      findsOneWidget,
    );
    await tester.tap(find.byTooltip('在 Desktop 打开'));
    await tester.pumpAndSettle();
    expect(opened, (7, 0));
  });

  testWidgets(
    'wide artifact preview defaults to split and arrows switch files',
    (tester) async {
      tester.view.physicalSize = const Size(700, 900);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      final store = DshConversationStore();
      store.apply(
        DshEventFrame(
          DshWireEvent.fromJson({
            'type': 'workspace/changes',
            'seq': 8,
            'time': 1,
            'data': {'turn': 3},
          }),
        ),
      );
      final loaded = <int>[];
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: DshConversationSurface(
              store: store,
              onLoadWorkspaceChanges: (_) async {
                return const DshNativeWorkspaceChanges(
                  turn: 3,
                  total: 2,
                  added: 2,
                  deleted: 2,
                  files: [
                    DshNativeChangedFile(
                      display: 'first.md',
                      added: 1,
                      deleted: 1,
                      binary: false,
                      oversized: false,
                    ),
                    DshNativeChangedFile(
                      display: 'second.dart',
                      added: 1,
                      deleted: 1,
                      binary: false,
                      oversized: false,
                    ),
                  ],
                );
              },
              onLoadWorkspaceChangePreview: (_, index) async {
                loaded.add(index);
                return DshNativeArtifactPreview(
                  kind: DshNativeArtifactPreviewKind.text,
                  display: index == 0 ? 'first.md' : 'second.dart',
                  before: true,
                  after: true,
                  coarse: false,
                  hunks: [
                    DshNativeDiffHunk(
                      oldStart: 10,
                      oldLines: 2,
                      newStart: 10,
                      newLines: 2,
                      lines: [
                        '-old value $index',
                        '+new value $index',
                        ' context',
                      ],
                    ),
                  ],
                );
              },
              onSend: (_, {mode = 'queue'}) async {},
              onCancel: () async {},
              onFallbackRequested: () {},
            ),
          ),
        ),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.text('first.md'));
      await tester.pumpAndSettle();

      expect(
        find.byKey(const ValueKey('dsh-native.artifact-preview.split')),
        findsOneWidget,
      );
      expect(find.text('old value 0'), findsOneWidget);
      expect(find.text('new value 0'), findsOneWidget);
      expect(find.text('1 / 2    +1  −1'), findsOneWidget);

      await tester.tap(find.byTooltip('下一个产物'));
      await tester.pumpAndSettle();
      expect(find.text('second.dart'), findsWidgets);
      expect(find.text('old value 1'), findsOneWidget);
      expect(find.text('new value 1'), findsOneWidget);
      expect(find.text('2 / 2    +1  −1'), findsOneWidget);
      expect(loaded, [0, 1]);

      await tester.tap(find.byTooltip('上一个产物'));
      await tester.pumpAndSettle();
      expect(find.text('old value 0'), findsOneWidget);
      expect(loaded, [0, 1, 0]);
    },
  );

  testWidgets('matches the pinned DSH conversation hierarchy on mobile', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(430, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);
    final store = DshConversationStore();
    store.apply(
      DshSnapshotFrame(
        header: const {'id': 's-parity'},
        cursor: 4,
        records: [
          DshWireEvent.fromJson({
            'type': 'user/message',
            'seq': 1,
            'time': 1790821558755,
            'surfaceOp': 'append',
            'data': {
              'content': [
                {'type': 'text', 'text': 'Desktop 与 Mobile 效果对齐'},
              ],
              'source': {'kind': 'user'},
            },
          }),
          DshWireEvent.fromJson({
            'type': 'step/end',
            'seq': 2,
            'time': 1790821558756,
            'data': {'step': 1},
          }),
          DshWireEvent.fromJson({
            'type': 'turn/end',
            'seq': 3,
            'time': 1790821558757,
            'data': {
              'turn': 1,
              'reason': {
                'kind': 'error',
                'error': {
                  'message': 'no API key for provider route',
                  'code': 'MISSING_CREDENTIAL',
                },
              },
            },
          }),
          DshWireEvent.fromJson({
            'type': 'session/title',
            'seq': 4,
            'time': 1790821558758,
            'data': {'title': 'Desktop 与 Mobile 原生对话流对齐'},
          }),
        ],
        hasMore: false,
        projections: const {
          'values': {
            'agentPreset': 'standard',
            'modelSelection': {
              'lastUsed': {
                'model': 'deepseek-flash',
                'reasoningEffort': 'high',
              },
            },
          },
        },
      ),
    );

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: DshConversationSurface(
            store: store,
            onSend: (_, {mode = 'queue'}) async {},
            onCancel: () async {},
            onFallbackRequested: () {},
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey('dsh-native.header')), findsOneWidget);
    expect(find.text('Desktop 与 Mobile 原生对话流对齐'), findsOneWidget);
    expect(find.text('标准模式'), findsOneWidget);
    expect(find.text('对话'), findsOneWidget);
    expect(find.text('轨迹'), findsOneWidget);
    expect(find.text('处理失败'), findsOneWidget);
    expect(find.textContaining('MISSING_CREDENTIAL'), findsOneWidget);
    expect(find.text('DeepSeek-V41-Flash · High'), findsOneWidget);
    expect(find.text('1 轮  1 步'), findsOneWidget);
    expect(find.byType(CircleAvatar), findsNothing);

    final headerBottom = tester
        .getBottomLeft(find.byKey(const ValueKey('dsh-native.header')))
        .dy;
    final messageTop = tester.getTopLeft(find.text('Desktop 与 Mobile 效果对齐')).dy;
    final composerTop = tester
        .getTopLeft(find.byKey(const ValueKey('dsh-native.composer-shell')))
        .dy;
    expect(headerBottom, lessThan(messageTop));
    expect(messageTop, lessThan(composerTop));
    expect(
      tester.getCenter(find.text('Desktop 与 Mobile 效果对齐')).dx,
      greaterThan(215),
      reason: 'DSH user prompts align to the right edge',
    );
  });
}
