import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:appflowy/plugins/resource_surface/surfaces/helix_resource_surface.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/material.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:path/path.dart' as p;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets(
    'retained Helix tabs keep only the selected tab LSP alive',
    (tester) async {
      if (!Platform.isMacOS && !Platform.isLinux) {
        markTestSkipped('Fake language-server process probe is POSIX-only');
        return;
      }

      await getIt.reset();
      final temp = await Directory.systemTemp.createTemp('muse-lsp-budget-');
      final installerRoot = Directory(p.join(temp.path, 'language-servers'));
      final dartBin =
          File(p.join(installerRoot.path, 'dart-sdk', 'bin', 'dart'));
      final events = File(p.join(installerRoot.path, 'lsp-events.log'));
      await dartBin.parent.create(recursive: true);
      await dartBin.writeAsString('''#!/usr/bin/env python3
import json
import os
import signal
import sys

events = ${jsonEncode(events.path)}
pid = os.getpid()
with open(events, "a", encoding="utf-8") as log:
    log.write("start %d\\n" % pid)

stopped = False
def stop(*_args):
    global stopped
    if not stopped:
        stopped = True
        with open(events, "a", encoding="utf-8") as log:
            log.write("stop %d\\n" % pid)
    raise SystemExit(0)

for sig in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
    signal.signal(sig, stop)

def reply(message):
    encoded = json.dumps(message, separators=(",", ":")).encode("utf-8")
    sys.stdout.buffer.write(
        ("Content-Length: %d\\r\\n\\r\\n" % len(encoded)).encode("ascii")
    )
    sys.stdout.buffer.write(encoded)
    sys.stdout.buffer.flush()

while True:
    length = None
    while True:
        line = sys.stdin.buffer.readline()
        if not line:
            stop()
        if line in (b"\\r\\n", b"\\n"):
            break
        name, value = line.decode("ascii").split(":", 1)
        if name.lower() == "content-length":
            length = int(value.strip())
    if length is None:
        continue
    message = json.loads(sys.stdin.buffer.read(length))
    method = message.get("method")
    if method == "initialize":
        reply({"jsonrpc": "2.0", "id": message["id"], "result": {"capabilities": {}}})
    elif method == "shutdown":
        reply({"jsonrpc": "2.0", "id": message["id"], "result": None})
    elif method == "exit":
        stop()
''');
      final chmod = await Process.run('chmod', ['+x', dartBin.path]);
      expect(chmod.exitCode, 0);

      final project = Directory(p.join(temp.path, 'project'));
      await project.create(recursive: true);
      await File(p.join(project.path, 'pubspec.yaml')).writeAsString(
        'name: lsp_budget_fixture\nenvironment:\n  sdk: ">=3.0.0 <4.0.0"\n',
      );
      final first = File(p.join(project.path, 'first.dart'));
      final second = File(p.join(project.path, 'second.dart'));
      await first.writeAsString('void first() {}\n');
      await second.writeAsString('void second() {}\n');

      final installer = HelixLanguageServerInstaller(root: installerRoot);
      final settings = HelixSettingsController(installer)..loaded = true;
      getIt.registerSingleton<HelixSettingsController>(settings);
      await Pty.prewarmAsync();

      try {
        await tester.pumpWidget(_tabs(first, second, active: 0));
        final firstLive = await _waitForSingleLiveServer(tester, events);

        await tester.pumpWidget(_tabs(first, second, active: 1));
        final secondLive = await _waitForSingleLiveServer(
          tester,
          events,
          differentFrom: firstLive,
        );
        expect(secondLive, isNot(firstLive));

        await tester.pumpWidget(const SizedBox.shrink());
        await _waitForNoLiveServers(tester, events);
      } finally {
        await Future<void>.delayed(const Duration(milliseconds: 500));
        await getIt.reset();
        settings.dispose();
        await temp.delete(recursive: true);
      }
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}

Widget _tabs(File first, File second, {required int active}) => MaterialApp(
      home: IndexedStack(
        index: active,
        children: [
          HelixResourceSurface(
            key: ValueKey(first.path),
            file: first,
            isActive: active == 0,
          ),
          HelixResourceSurface(
            key: ValueKey(second.path),
            file: second,
            isActive: active == 1,
          ),
        ],
      ),
    );

Future<int> _waitForSingleLiveServer(
  WidgetTester tester,
  File events, {
  int? differentFrom,
}) async {
  final deadline = Stopwatch()..start();
  while (deadline.elapsed < const Duration(seconds: 15)) {
    // Integration-test bindings only execute post-frame callbacks when the
    // test advances a frame. The production app schedules these normally.
    await tester.pump();
    await Future<void>.delayed(const Duration(milliseconds: 200));
    final live = await _liveServerPids(events);
    if (live.length == 1 &&
        (differentFrom == null || live.single != differentFrom)) {
      return live.single;
    }
  }
  fail(
    'Expected exactly one selected-tab LSP; events=${await _events(events)}',
  );
}

Future<void> _waitForNoLiveServers(WidgetTester tester, File events) async {
  final deadline = Stopwatch()..start();
  while (deadline.elapsed < const Duration(seconds: 10)) {
    await tester.pump();
    await Future<void>.delayed(const Duration(milliseconds: 200));
    if ((await _liveServerPids(events)).isEmpty) return;
  }
  fail(
    'Language servers survived closed tabs; events=${await _events(events)}',
  );
}

Future<List<int>> _liveServerPids(File events) async {
  final pids = <int>{
    for (final line in await _events(events))
      if (line.startsWith('start ')) int.parse(line.substring(6)),
  };
  final live = <int>[];
  for (final pid in pids) {
    final result = await Process.run('ps', ['-p', '$pid', '-o', 'stat=']);
    final state = '${result.stdout}'.trim();
    if (result.exitCode == 0 && state.isNotEmpty && !state.startsWith('Z')) {
      live.add(pid);
    }
  }
  return live;
}

Future<List<String>> _events(File file) async {
  if (!file.existsSync()) return const <String>[];
  return file.readAsLines();
}
