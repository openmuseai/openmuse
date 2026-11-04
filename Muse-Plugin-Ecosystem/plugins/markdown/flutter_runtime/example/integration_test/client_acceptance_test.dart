import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';
import 'package:archive/archive.dart';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';
import 'package:path_provider/path_provider.dart';
import 'package:webview_flutter/webview_flutter.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'original client, local notebook, blocks, search, export and UI save',
    (tester) async {
      final checks = <String, Object?>{};
      final kernel = SiYuanKernel();
      WebViewController? view;
      Uint8List? exportedBytes;
      Object? failure;
      final client = HttpClient();
      Future<T> io<T>(Future<T> Function() action) async {
        T? value;
        Object? error;
        StackTrace? stack;
        await tester.runAsync(() async {
          try {
            value = await action();
          } catch (caught, trace) {
            error = caught;
            stack = trace;
          }
        });
        if (error != null) Error.throwWithStackTrace(error!, stack!);
        return value as T;
      }

      try {
        final origin = await io(() => kernel.start());
        expect(origin.host, '127.0.0.1');
        checks['localKernel'] = true;
        Future<dynamic> api(
          String path, [
          Map<String, Object?> body = const {},
        ]) async {
          final request = await client.postUrl(origin.resolve(path));
          kernel.localSessionHeaders.forEach(request.headers.set);
          request.headers.contentType = ContentType.json;
          request.write(jsonEncode(body));
          final response = await request.close();
          final raw = await utf8.decoder.bind(response).join();
          if (response.statusCode != 200) {
            throw StateError('$path: ${response.statusCode} $raw');
          }
          final result = jsonDecode(raw) as Map;
          if (result['code'] != 0) throw StateError('$path: $result');
          return result['data'];
        }

        await io(() async {
          final denied = await (await client.getUrl(
            origin.resolve('/api/system/version'),
          )).close();
          expect(denied.statusCode, 403);
          await denied.drain<void>();
          checks['unauthorizedLoopbackDenied'] = true;
          final exit = await client.postUrl(origin.resolve('/api/system/exit'));
          kernel.localSessionHeaders.forEach(exit.headers.set);
          exit.headers.contentType = ContentType.json;
          exit.write('{}');
          final closed = await exit.close();
          final result =
              jsonDecode(await utf8.decoder.bind(closed).join()) as Map;
          expect(result['msg'], 'PLUGIN_LIFECYCLE_MANAGED');
          checks['hostLifecycleProtected'] = true;
        });
        final stamp = DateTime.now().microsecondsSinceEpoch.toString();
        final notebook = await io(
          () => api('/api/notebook/createNotebook', {
            'name': 'Plugin acceptance $stamp',
          }),
        );
        final box = (notebook as Map)['notebook']['id'] as String;
        final createdAt = DateTime(
          int.parse(box.substring(0, 4)),
          int.parse(box.substring(4, 6)),
          int.parse(box.substring(6, 8)),
          int.parse(box.substring(8, 10)),
          int.parse(box.substring(10, 12)),
          int.parse(box.substring(12, 14)),
        );
        expect(
          DateTime.now().difference(createdAt).abs(),
          lessThan(const Duration(minutes: 2)),
        );
        checks['kernelLocalTimezone'] = true;
        Future<void> metadataReady(String box) async {
          for (var attempt = 0; attempt < 100; attempt++) {
            final configuration =
                await api('/api/notebook/getNotebookConf', {'notebook': box})
                    as Map;
            if ((configuration['documentCapabilities']
                    as Map?)?['indexState'] ==
                'ready') {
              return;
            }
            await Future<void>.delayed(const Duration(milliseconds: 100));
          }
          throw StateError('New notebook metadata index did not become ready');
        }

        await io(() => metadataReady(box));
        final document = await io(
          () => api('/api/filetree/createDocWithMd', {
            'notebook': box,
            'path': '/Local client $stamp',
            'markdown':
                '# SiYuan 客户端验收\n\nOriginal editor content.\n\n- [ ] task\n\n| A | B |\n| --- | --- |\n| 1 | 2 |\n\n```js\nconst local = true;\n```\n',
          }),
        );
        final doc = document as String;
        checks['notebookAndMarkdownImport'] = true;
        await io(() async {
          await api('/api/block/appendBlock', {
            'dataType': 'markdown',
            'data': 'Plugin block $stamp',
            'parentID': doc,
          });
          await api('/api/sqlite/flushTransaction');
          final rows = await api('/api/query/sql', {
            'stmt':
                "SELECT id FROM blocks WHERE root_id='$doc' AND content LIKE '%Plugin block%'",
          });
          expect(rows, isNotEmpty);
          checks['blocksAndSearchIndex'] = true;
          final exported =
              await api('/api/export/exportMdContent', {'id': doc}) as Map;
          expect(exported['content'], contains('Plugin block $stamp'));
          expect(exported['content'], contains('SiYuan 客户端验收'));
          checks['markdownExport'] = true;
        });
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: SiYuanClientView(
                kernel: kernel,
                exportWriter: (name, bytes) async {
                  exportedBytes = bytes;
                  return true;
                },
                onReady: (controller) => view = controller,
              ),
            ),
          ),
        );
        for (var i = 0; i < 180 && view == null; i++) {
          await tester.pump(const Duration(milliseconds: 200));
          await io(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
        }
        expect(view, isNotNull, reason: 'Original frontend failed to load');
        await io(
          () => view!.loadRequest(
            origin.resolve(
              Platform.isMacOS
                  ? '/stage/build/desktop/?id=$doc'
                  : '/stage/build/mobile/?id=$doc&url=siyuan://blocks/$doc',
            ),
            headers: kernel.localSessionHeaders,
          ),
        );
        Future<Object> evaluate(String script) =>
            view!.runJavaScriptReturningResult(script);
        var rendered = false;
        for (var i = 0; i < 180; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          final value = await io(
            () => evaluate(
              "Boolean(document.querySelector('.protyle-wysiwyg [data-node-id]'))",
            ),
          );
          if (value == true || value == 'true') {
            rendered = true;
            break;
          }
          await io(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
        }
        expect(
          rendered,
          true,
          reason: 'The original Protyle editor did not render the document',
        );
        checks['originalProtyleFrontend'] = true;
        var backend = await io(
          () => evaluate('window.siyuan.config.system.container'),
        );
        if (backend is String && backend.startsWith('"')) {
          backend = jsonDecode(backend);
        }
        expect(backend, Platform.isMacOS ? 'std' : Platform.operatingSystem);
        checks['kernelPlatform'] = true;
        final marker = ' UI_INPUT_$stamp';
        await io(
          () => view!.runJavaScript('''
        (() => {
          const editor = document.querySelector('.protyle-wysiwyg');
          const block = editor.querySelector('[data-type="NodeParagraph"] [contenteditable="true"]') || editor.querySelector('[contenteditable="true"]');
          if (!block) throw Error('No editable paragraph');
          block.focus();
          const range = document.createRange(); range.selectNodeContents(block); range.collapse(false);
          const selection = getSelection(); selection.removeAllRanges(); selection.addRange(range);
          if (!document.execCommand('insertText', false, ${jsonEncode(marker)})) throw Error('Browser input rejected');
        })();
      '''),
        );
        var saved = false;
        for (var i = 0; i < 100; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          saved = await io(() async {
            await Future<void>.delayed(const Duration(milliseconds: 100));
            final exported =
                await api('/api/export/exportMdContent', {'id': doc}) as Map;
            return (exported['content'] as String).contains(marker.trim());
          });
          if (saved) break;
        }
        expect(
          saved,
          true,
          reason: 'UI input did not reach the local notebook',
        );
        checks['uiInputPersisted'] = true;
        await io(() async {
          final archive = await api('/api/export/exportMd', {'id': doc}) as Map;
          final path = archive['zip'] as String;
          await view!.runJavaScript('''
            (async () => {
              const response = await fetch(${jsonEncode(path)});
              const reader = new FileReader();
              reader.onload = () => SiYuanClient.postMessage(JSON.stringify({id:'acceptance-export',method:'saveExport',name:'notes.zip',blob:String(reader.result).split(',')[1]}));
              reader.readAsDataURL(await response.blob());
            })();
          ''');
        });
        for (var i = 0; i < 100 && exportedBytes == null; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          await io(
            () => Future<void>.delayed(const Duration(milliseconds: 50)),
          );
        }
        expect(exportedBytes, isNotNull);
        final exportedFiles = ZipDecoder().decodeBytes(exportedBytes!).files;
        expect(
          exportedFiles
              .where((file) => file.name.endsWith('.md'))
              .any((file) => utf8.decode(file.content).contains(marker.trim())),
          true,
        );
        checks['nativeExportBridge'] = true;
        await io(() async {
          const password = 'SiYuan-local-plugin-acceptance-2026';
          final status =
              await api('/api/notebook/getEncryptedNotebookStatus') as Map;
          if (status['enabled'] != true) {
            await api('/api/notebook/enableEncryptedNotebooks', {
              'password': password,
            });
          }
          final created =
              await api('/api/notebook/createEncryptedNotebook', {
                    'name': 'Encrypted $stamp',
                    'password': password,
                  })
                  as Map;
          final encrypted = created['notebook']['id'] as String;
          await metadataReady(encrypted);
          final secret =
              '${DateTime.now().toIso8601String().replaceAll(RegExp(r'\D'), '').substring(0, 14)}-crypt01';
          await api('/api/filetree/createDocByParent', {
            'notebook': encrypted,
            'parentID': null,
            'id': secret,
            'title': {'format': 'plain', 'text': 'Encrypted document'},
            'markdown': 'Secret $stamp',
          });
          await api('/api/notebook/lockNotebook', {'notebook': encrypted});
          await api('/api/notebook/unlockAndOpenNotebook', {
            'notebook': encrypted,
            'password': password,
          });
          await metadataReady(encrypted);
          final restored =
              await api('/api/export/exportMdContent', {'id': secret}) as Map;
          expect(restored['content'], contains('Secret $stamp'));
          await api('/api/notebook/lockNotebook', {'notebook': encrypted});
          checks['encryptedNotebookRoundTrip'] = true;
          final ocr = await api('/api/asset/getOCRConfig') as Map;
          expect(
            (ocr['providers'] as List).whereType<Map>().any(
              (provider) =>
                  provider['id'] == 'paddleocr' &&
                  provider['available'] == true,
            ),
            true,
            reason: 'Bundled local OCR runtime unavailable',
          );
          checks['localOcrRuntime'] = true;
        });
        await io(() async {
          await kernel.suspend();
          final suspended = await client.getUrl(
            origin.resolve('/api/system/version'),
          );
          kernel.localSessionHeaders.forEach(suspended.headers.set);
          final response = await suspended.close();
          expect(response.statusCode, 503);
          await response.drain<void>();
          await kernel.resume();
          await kernel.flush();
          final restored =
              await api('/api/export/exportMdContent', {'id': doc}) as Map;
          expect(restored['content'], contains(marker.trim()));
          checks['deactivateReactivatePreservesData'] = true;
        });
      } catch (error, stack) {
        failure = error;
        checks['error'] = '$error';
        checks['stack'] = '$stack';
      } finally {
        client.close(force: true);
        await io(() async {
          final directory = await getApplicationSupportDirectory();
          await File('${directory.path}/siyuan-acceptance.json').writeAsString(
            jsonEncode({
              'platform': Platform.operatingSystem,
              'passed': failure == null,
              'checks': checks,
              'at': DateTime.now().toUtc().toIso8601String(),
            }),
            flush: true,
          );
          debugPrint(
            'SIYUAN_ACCEPTANCE_RESULT ${jsonEncode({'platform': Platform.operatingSystem, 'passed': failure == null, 'checks': checks, 'at': DateTime.now().toUtc().toIso8601String()})}',
          );
        });
      }
      if (failure != null) fail('$failure');
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
