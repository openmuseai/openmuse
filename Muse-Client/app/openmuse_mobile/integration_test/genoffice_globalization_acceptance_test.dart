import 'support/device_result.dart';
import 'dart:convert';
import 'dart:io';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  testWidgets(
    'GenOffice 0.8.0 installs, edits, switches language, undoes and saves on device',
    (tester) async {
      const expected = String.fromEnvironment(
        'OPENMUSE_GENOFFICE_BUNDLE_SHA256',
      );
      expect(
        expected,
        hasLength(64),
        reason: 'Supply the reviewed bundle digest',
      );
      final support = await getApplicationSupportDirectory();
      final inputs = Directory('${support.path}/acceptance-inputs');
      final package = File('${inputs.path}/genoffice.mcpb');
      final fixture = File('${inputs.path}/sample.pptx');
      expect(
        await package.exists(),
        isTrue,
        reason: 'Stage the reviewed bundle in the test app private directory',
      );
      final bundle = await package.readAsBytes();
      expect(sha256.convert(bundle).toString(), expected);
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final serving = server.listen((request) async {
        if (request.uri.path != '/bundle.mcpb') {
          request.response.statusCode = 404;
          await request.response.close();
          return;
        }
        request.response.add(bundle);
        await request.response.close();
      });
      final origin = Uri.parse('http://127.0.0.1:${server.port}');
      final market = RemoteMcpMarket(
        origin,
        File(
          Platform.isAndroid
              ? '${support.path}/acceptance-genoffice-receipts.json'
              : '${support.path}/acceptance-genoffice/receipts.json',
        ),
      );
      final entry = RemoteMcpEntry.fromRecord({
        'server': {
          'name': 'io.openmuse/genoffice-pptx',
          'title': 'GenOffice PPTX',
          'version': '0.8.0',
          'packages': [
            {
              'registryType': 'mcpb',
              'identifier': '$origin/bundle.mcpb',
              'fileSha256': expected,
            },
          ],
        },
      });
      await market.install(entry);
      expect(await market.installed(), contains('${entry.name}@0.8.0'));
      final html = await market.mcpAppDocument(entry);
      expect(html, isNotNull);
      final destination = File(
        '${support.path}/acceptance-genoffice/result.pptx',
      );
      await destination.parent.create(recursive: true);
      final original = await fixture.readAsBytes();
      await destination.writeAsBytes(original, flush: true);
      final locale = OpenMuseLocaleController(
        defaultTag: 'en',
        initialTag: 'en',
      );
      RemoteMcpViewConnection? connection;
      final page = RemoteMcpOfflineAppPage(
        title: 'GenOffice PPTX',
        digest: expected,
        appHtml: html,
        fileName: 'result.pptx',
        filePath: destination.path,
        fileBytes: original,
        mediaType:
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
        onCreated: (value) => connection = value,
      );
      await tester.pumpWidget(
        ListenableBuilder(
          listenable: locale,
          builder: (context, _) => OpenMuseLocaleScope(
            controller: locale,
            child: MaterialApp(
              locale: locale.locale,
              supportedLocales: locale.supportedLocales,
              localizationsDelegates: [
                OpenMuseStringsDelegate(locale.catalog),
                ...GlobalMaterialLocalizations.delegates,
              ],
              home: page,
            ),
          ),
        ),
      );
      for (
        var i = 0;
        i < 240 && (connection == null || !connection!.ready.isCompleted);
        i++
      ) {
        await tester.pump(const Duration(milliseconds: 250));
      }
      expect(connection, isNotNull);
      expect(connection!.ready.isCompleted, isTrue);
      final view = connection!;
      Future<Map<String, dynamic>> tool(
        String name, [
        Map<String, Object?> arguments = const {},
      ]) async {
        final result = await view.request('tools/call', {
          'name': name,
          'arguments': arguments,
        });
        expect(
          result['isError'],
          isNot(true),
          reason: 'GenOffice tool $name must succeed: ${jsonEncode(result)}',
        );
        return Map<String, dynamic>.from(
          jsonDecode(
                ((result['content'] as List).first as Map)['text'] as String,
              )
              as Map,
        );
      }

      Map<String, dynamic>? deck;
      for (var i = 0; i < 60; i++) {
        final result = await view.request('tools/call', {
          'name': 'pptx_read_deck',
          'arguments': {},
        });
        if (result['isError'] != true) {
          final candidate =
              jsonDecode(
                    ((result['content'] as List).first as Map)['text']
                        as String,
                  )
                  as Map<String, dynamic>;
          if ((candidate['slides'] as List? ?? []).isNotEmpty) {
            deck = candidate;
            break;
          }
        }
        await tester.pump(const Duration(milliseconds: 500));
      }
      expect(deck, isNotNull);
      final slide = (deck!['slides'] as List).cast<Map>().firstWhere(
        (slide) => (slide['elements'] as List).cast<Map>().any(
          (el) => el['text'] is String && (el['text'] as String).isNotEmpty,
        ),
      );
      final element = (slide['elements'] as List).cast<Map>().firstWhere(
        (el) => el['text'] is String && (el['text'] as String).isNotEmpty,
      );
      final originalText = element['text'];
      const changedText = 'OpenMuse device acceptance 原始内容';
      final editArgs = <String, Object?>{
        'slideIndex': slide['index'],
        'elementId': element['elementId'],
        'expectedRevision': deck['revision'],
        'text': changedText,
      };
      await tool('pptx_edit_text', editArgs);
      final edited = await tool('pptx_read_deck');
      locale.selectTag('zh-Hans');
      await tester.pump(const Duration(seconds: 1));
      expect(connection, same(view));
      final afterLanguage = await tool('pptx_read_deck');
      expect(afterLanguage['revision'], edited['revision']);
      expect(jsonEncode(afterLanguage), contains(changedText));
      await tool('pptx_undo', {'expectedRevision': afterLanguage['revision']});
      final undone = await tool('pptx_read_deck');
      expect(jsonEncode(undone), contains(originalText));
      expect(jsonEncode(undone), isNot(contains(changedText)));
      final restoredSlide = (undone['slides'] as List).cast<Map>().firstWhere(
        (s) => s['index'] == slide['index'],
      );
      final restoredElement = (restoredSlide['elements'] as List)
          .cast<Map>()
          .firstWhere((e) => e['text'] == originalText);
      await tool('pptx_edit_text', {
        ...editArgs,
        'elementId': restoredElement['elementId'],
        'expectedRevision': undone['revision'],
      });
      final finalDeck = await tool('pptx_read_deck');
      await tool('pptx_save', {'expectedRevision': finalDeck['revision']});
      final saved = await destination.readAsBytes();
      expect(
        sha256.convert(saved).toString(),
        isNot(sha256.convert(original).toString()),
      );
      final archive = ZipDecoder().decodeBytes(saved);
      final xml = archive.files
          .where(
            (f) => f.name.startsWith('ppt/slides/') && f.name.endsWith('.xml'),
          )
          .map((f) => utf8.decode(f.content as List<int>))
          .join('\n');
      expect(xml, contains(changedText));
      debugPrint(
        'GENOFFICE_DEVICE_ACCEPTANCE_PASS platform=${Platform.operatingSystem} version=0.8.0 sha256=$expected edit=true locale=retained undo=true saved=true',
      );
      // Keep the final rendered editor available for a native screenshot.
      await tester.pump(const Duration(seconds: 2));
      if (Platform.isIOS) {
        final png = await binding.takeScreenshot('genoffice-zh-Hans');
        await File(
          '${support.path}/acceptance-genoffice/screenshot.png',
        ).writeAsBytes(png);
      }
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump();
      locale.dispose();
      await serving.cancel();
      await server.close(force: true);
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
