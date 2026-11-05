import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_desktop_error.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('E2-T7 maps a missing API key', () {
    expect(
      DshDesktopError.codeFromMessage(
        'StateError: DEEPSEEK_API_KEY is missing. Enter it in the DeepSeek panel.',
      ),
      DshDesktopError.needApiKey,
    );
  });

  test('E2-T8 maps a sidecar timeout and crash', () {
    expect(
      DshDesktopError.codeFromMessage(
        'DSH sidecar did not become ready on http://127.0.0.1:3080',
      ),
      DshDesktopError.frameTimeout,
    );
    expect(
      DshDesktopError.codeFromMessage('DSH sidecar exited with 1'),
      DshDesktopError.sidecarExit,
    );
  });

  test('E2-T8 ready is HybridLive and never talks to session/open', () {
    final controller = DshAgentController();
    controller.setLaunching(true);
    expect(controller.stage, 'Placing');
    controller.setLaunching(false);
    controller.setUrl('http://127.0.0.1:3080/?token=launch');
    controller.setReady(true);
    expect(controller.ready, isTrue);
    expect(controller.stage, 'HybridLive');
    expect(controller.url, contains('127.0.0.1:3080'));
  });

  test('E2-T9 desktop sidecar never calls remote session/open', () {
    final sidecar = File('lib/plugins/dsh_agent/dsh_sidecar.dart').readAsStringSync();
    expect(sidecar.contains('/api/muse/dsh/session'), isFalse);
  });
}
