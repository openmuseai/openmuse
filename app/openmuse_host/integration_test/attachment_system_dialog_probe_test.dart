import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_attachment_source/muse_attachment_source.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'original system file dialogs with task-scoped input, no AX',
    (t) async {
      const fixturePath = String.fromEnvironment('SMARTDOC_DATABASE_FIXTURE');
      final fixture = jsonDecode(await File(fixturePath).readAsString()) as Map;
      final source = fixture['attachmentSourcePath'] as String;
      final root = File(fixturePath).parent.path;
      await t.pumpWidget(
        const MaterialApp(home: Scaffold(body: Text('系统文件面板探针'))),
      );
      Future<void> input(String path) async {
        try {
          await t.runAsync(
            () => const MethodChannel(
              'io.openmuse.smartDocumentHostAcceptance/file-dialog-input',
            ).invokeMethod<void>('input', {'root': root, 'path': path}),
          );
          debugPrint('TASK_DIALOG_PROBE input scheduled');
        } on PlatformException catch (e) {
          debugPrint('TASK_DIALOG_PROBE platform error code=${e.code}');
          rethrow;
        } on MissingPluginException {
          debugPrint('TASK_DIALOG_PROBE input channel missing');
          rethrow;
        }
      }

      Future<T> wait<T>(Future<T> Function() operation) async {
        T? value;
        Object? failure;
        bool done = false;
        await t.runAsync(() async {
          operation().then(
            (v) {
              value = v;
              done = true;
            },
            onError: (Object e) {
              failure = e;
              done = true;
            },
          );
        });
        for (var i = 0; i < 600 && !done; i++) {
          await t.pump(const Duration(milliseconds: 50));
          await t.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 20)),
          );
          if (i % 100 == 50) {
            Map<dynamic, dynamic>? status;
            await t.runAsync(() async {
              try {
                status = await const MethodChannel(
                  'io.openmuse.smartDocumentHostAcceptance/file-dialog-input',
                ).invokeMapMethod('status');
              } catch (e) {
                debugPrint(
                  'TASK_DIALOG_PROBE status failure type=${e.runtimeType}',
                );
              }
            });
            debugPrint('TASK_DIALOG_PROBE status=$status');
          }
        }
        expect(done, true, reason: 'Original system dialog did not complete');
        if (failure != null) {
          debugPrint(
            'TASK_DIALOG_PROBE IO failure type=${failure.runtimeType}',
          );
          throw failure!;
        }
        return value as T;
      }

      await input(source);
      final selected = await wait(pickAttachment);
      expect(selected, isNotNull);
      expect(selected!.bytes, await File(source).readAsBytes());
      debugPrint('TASK_DIALOG_PROBE actual file selection passed');
      final directory = await Directory('$root/probe-export').create();
      await input(directory.path);
      final destination = await wait(
        () => chooseAttachmentSaveTarget(selected.name),
      );
      expect(destination, isNotNull);
      await destination!.write(selected.bytes, mediaType: selected.mediaType);
      expect(
        await File('${directory.path}/${selected.name}').readAsBytes(),
        selected.bytes,
      );
      debugPrint('TASK_DIALOG_PROBE actual file save passed');
    },
    timeout: const Timeout(Duration(minutes: 2)),
  );
}
