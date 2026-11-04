import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';
import 'package:path_provider/path_provider.dart';
import 'package:archive/archive.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  testWidgets(
    'native archive resource and standalone Protyle roundtrip',
    (tester) async {
      Future<T> io<T>(Future<T> Function() action) async {
        T? value;
        Object? error;
        StackTrace? trace;
        await tester.runAsync(() async {
          try {
            value = await action();
          } catch (e, s) {
            error = e;
            trace = s;
          }
        });
        if (error != null) Error.throwWithStackTrace(error!, trace!);
        return value as T;
      }

      final kernel = SiYuanKernel();
      final api = SiYuanResourceApi(kernel);
      final seed = await io(() => api.seed('OpenMuse resource roundtrip'));
      expect(SiYuanDocumentArchive.decode(seed).documents.length, 1);
      final lease = await io(() => api.open(seed));
      SiYuanResourceController? controller;
      bool ready = false, dirty = false;
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: SiYuanResourceView(
              lease: lease,
              editing: true,
              onDirty: () => dirty = true,
              onReady: () => ready = true,
              onSave: () {},
              onController: (v) => controller = v,
            ),
          ),
        ),
      );
      for (var i = 0; i < 200 && !ready; i++) {
        await tester.pump(const Duration(milliseconds: 100));
        await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
      }
      if (!ready && controller != null) {
        final debug = await io(
          () => controller!.web.runJavaScriptReturningResult(
            'document.body.innerText',
          ),
        );
        fail(
          'Editor failed: $debug; ${tester.widgetList<SelectableText>(find.byType(SelectableText)).map((w) => w.data).join(' | ')}',
        );
      }
      expect(
        ready,
        isTrue,
        reason: tester
            .widgetList<SelectableText>(find.byType(SelectableText))
            .map((w) => w.data)
            .join(' | '),
      );
      expect(
        await io(
          () => controller!.web.runJavaScriptReturningResult(
            'Boolean(document.getElementById("toolbar") || document.getElementById("layouts") || document.getElementById("loading"))',
          ),
        ),
        anyOf(false, 'false'),
      );
      final marker = 'Resource UI ${DateTime.now().microsecondsSinceEpoch}';
      await io(
        () => controller!.web.runJavaScript('''(() => {
      const block=document.querySelector('.protyle-wysiwyg [contenteditable=true]');
      block.focus(); const r=document.createRange(); r.selectNodeContents(block);r.collapse(false);
      const s=getSelection();s.removeAllRanges();s.addRange(r);
      document.execCommand('insertText',false,${jsonEncode(marker)});
    })()'''),
      );
      await tester.pump(const Duration(milliseconds: 50));
      await io(() => controller!.flush());
      expect(dirty, isTrue);
      // Native fixtures exercise package dependencies, independently of UI input.
      final related =
          await io(
                () => api.call('/api/filetree/createDocWithMd', {
                  'notebook': lease.notebook,
                  'path': '/Related',
                  'markdown': 'Related resource body',
                }),
              )
              as String;
      await io(
        () => api.call('/api/block/appendBlock', {
          'parentID': lease.root,
          'dataType': 'markdown',
          'data': '(($related))',
        }),
      );
      final avId =
          '${lease.root.substring(0, 15)}db${lease.root.substring(17)}';
      await io(
        () => api.call('/api/block/appendBlock', {
          'parentID': lease.root,
          'dataType': 'dom',
          'data':
              '<div data-type="NodeAttributeView" data-av-id="$avId" data-av-type="table"></div>',
        }),
      );
      await io(
        () => api.call('/api/av/renderAttributeView', {
          'id': avId,
          'page': 1,
          'pageSize': 20,
        }),
      );
      await io(
        () => controller!.web.runJavaScript('''(async()=>{
      try {
        const form=new FormData();form.append('id',${jsonEncode(lease.root)});
        form.append('file[]',new File([new TextEncoder().encode(${jsonEncode('OM ASSET $marker')})],'resource-asset.txt',{type:'text/plain'}));
        const response=await (await fetch('/api/asset/upload',{method:'POST',body:form})).json();
        if(response.code!==0) throw new Error(response.msg);
        const path=response.data.succFiles[0].path;
        const append=await (await fetch('/api/block/appendBlock',{method:'POST',body:JSON.stringify({parentID:${jsonEncode(lease.root)},dataType:'markdown',data:'[asset]('+path+')'})})).json();
        if(append.code!==0) throw new Error(append.msg);
        document.documentElement.dataset.assetFixture='done';
      } catch(e){document.documentElement.dataset.assetFixture=String(e);}
    })()'''),
      );
      var uploaded = false;
      for (var i = 0; i < 100; i++) {
        await tester.pump(const Duration(milliseconds: 100));
        final v = await io(
          () => controller!.web.runJavaScriptReturningResult(
            'document.documentElement.dataset.assetFixture || "pending"',
          ),
        );
        if ('$v'.contains('done')) {
          uploaded = true;
          break;
        }
        await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
      }
      expect(uploaded, isTrue);
      final saved = await io(lease.snapshot);
      final files = ZipDecoder().decodeBytes(saved).files;
      expect(
        files.any(
          (f) => f.name.contains('/storage/av/') && f.name.endsWith('.json'),
        ),
        isTrue,
      );
      expect(
        files.any(
          (f) => f.name.contains('/assets/') && f.name.endsWith('.txt'),
        ),
        isTrue,
      );
      await io(
        () => controller!.web.runJavaScript(
          'window.openMuseResource.openBlock(${jsonEncode(related)})',
        ),
      );
      var navigated = false;
      for (var i = 0; i < 100; i++) {
        await tester.pump(const Duration(milliseconds: 100));
        final v = await io(
          () => controller!.web.runJavaScriptReturningResult(
            'document.querySelector(".protyle-wysiwyg")?.innerText || ""',
          ),
        );
        if ('$v'.contains('Related resource body')) {
          navigated = true;
          break;
        }
        await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
      }
      expect(navigated, isTrue);

      expect(
        SiYuanDocumentArchive.decode(saved).conversationText(),
        contains(marker),
      );
      final variant = Archive();
      for (final file in files.where((f) => f.isFile)) {
        final content = file.name.endsWith('.txt')
            ? utf8.encode('Variant asset')
            : file.content;
        variant.addFile(ArchiveFile(file.name, content.length, content));
      }
      final shadow = await io(() => api.open(ZipEncoder().encode(variant)));
      expect(shadow.root, isNot(lease.root));
      await io(
        () => api.call('/api/block/appendBlock', {
          'parentID': shadow.root,
          'dataType': 'markdown',
          'data': 'Second resource only',
        }),
      );
      final primaryBytes = await io(lease.snapshot),
          shadowBytes = await io(shadow.snapshot);
      expect(
        SiYuanDocumentArchive.decode(primaryBytes).conversationText(),
        isNot(contains('Second resource only')),
      );
      expect(
        SiYuanDocumentArchive.decode(shadowBytes).conversationText(),
        contains('Second resource only'),
      );
      final primaryAssets = ZipDecoder()
          .decodeBytes(primaryBytes)
          .files
          .where((f) => f.name.endsWith('.txt'))
          .map((f) => utf8.decode(f.content))
          .join();
      final shadowAssets = ZipDecoder()
          .decodeBytes(shadowBytes)
          .files
          .where((f) => f.name.endsWith('.txt'))
          .map((f) => utf8.decode(f.content))
          .join();
      expect(primaryAssets, contains('OM ASSET'));
      expect(shadowAssets, contains('Variant asset'));
      await io(shadow.close);
      await tester.pumpWidget(const SizedBox.shrink());
      await io(lease.close);
      final reopened = await io(() => api.open(saved));
      expect(reopened.root, isNot(lease.root));
      final exported = await io(reopened.snapshot);
      final reopenedPackage = SiYuanDocumentArchive.decode(exported);
      expect(reopenedPackage.conversationText(), contains(marker));
      expect(reopenedPackage.documents.length, 2);
      final refs = <String>[];
      void collect(Object? value) {
        if (value is Map) {
          if (value['TextMarkBlockRefID'] is String) {
            refs.add(value['TextMarkBlockRefID'] as String);
          }
          for (final child in value.values) {
            collect(child);
          }
        } else if (value is List) {
          for (final child in value) {
            collect(child);
          }
        }
      }

      collect(reopenedPackage.documents);
      expect(refs, isNotEmpty);
      expect(refs, isNot(contains(related)));
      final ids = reopenedPackage.documents.values
          .map((tree) => tree['ID'])
          .toSet();
      expect(refs.every(ids.contains), isTrue);
      final roundtripFiles = ZipDecoder().decodeBytes(exported).files;
      expect(
        roundtripFiles.any((f) => f.name.contains('/storage/av/')),
        isTrue,
      );
      expect(
        roundtripFiles.any(
          (f) => f.name.contains('/assets/') && f.name.endsWith('.txt'),
        ),
        isTrue,
      );
      await io(reopened.close);
      File(
        '${(await getApplicationSupportDirectory()).path}/siyuan-resource-native-result.json',
      ).writeAsStringSync(
        jsonEncode({
          'passed': true,
          'platform': Platform.operatingSystem,
          'checks': {
            'standaloneProtyle': true,
            'pendingInputFlush': true,
            'nativeDatabaseRoundTrip': true,
            'nativeAttachmentRoundTrip': true,
            'blockReferenceRemapping': true,
            'sameNameDifferentAssetIsolation': true,
            'twoResourceEditIsolation': true,
            'inPackageNavigation': true,
          },
          'marker': marker,
        }),
      );
    },
    timeout: const Timeout(Duration(minutes: 6)),
  );
}
