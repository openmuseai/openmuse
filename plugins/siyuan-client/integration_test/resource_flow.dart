import 'dart:convert';
import 'dart:io';
import 'package:archive/archive.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:path_provider/path_provider.dart';

Future<Map<String, Object?>> runSiYuanResourceFlow(
  WidgetTester tester, {
  required bool desktop,
  bool manualVisualCheckpoints = true,
  bool externalDocumentChecks = true,
  Future<void> Function(String stage)? onVisualCheckpoint,
}) async {
  Future<T> io<T>(Future<T> Function() action) async {
    T? value;
    Object? error;
    StackTrace? stack;
    await tester.runAsync(() async {
      try {
        value = await action();
      } catch (e, s) {
        error = e;
        stack = s;
      }
    });
    if (error != null) Error.throwWithStackTrace(error!, stack!);
    return value as T;
  }

  Future<void> wait(Finder finder) async {
    for (var i = 0; i < 240; i++) {
      await tester.pump(const Duration(milliseconds: 100));
      if (finder.evaluate().isNotEmpty) return;
      await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    }
    fail(
      'Widget timed out: $finder. Text: ${tester.widgetList<Text>(find.byType(Text)).map((w) => w.data).join(' | ')}',
    );
  }

  Future<void> tap(String key) async {
    final finder = find.byKey(ValueKey(key));
    await wait(finder);
    await tester.ensureVisible(finder);
    await tester.pumpAndSettle();
    await tester.tap(finder);
    await tester.pump(const Duration(milliseconds: 300));
  }

  Future<void> visualCheckpoint(String stage) async {
    await tester.pump(const Duration(milliseconds: 300));
    await io(() => Future<void>.delayed(const Duration(milliseconds: 300)));
    await tester.pump();
    final support = await io(getApplicationSupportDirectory);
    await io(
      () => File(
        '${support.path}/resource-visual-ready.json',
      ).writeAsString(stage),
    );
    await onVisualCheckpoint?.call(stage);
    if (!manualVisualCheckpoints ||
        !await io(
          () => File('${support.path}/resource-visual-checkpoints').exists(),
        ))
      return;
    final release = File('${support.path}/resource-visual-release-$stage');
    final deadline = DateTime.now().add(const Duration(seconds: 90));
    while (!await io(release.exists) && DateTime.now().isBefore(deadline)) {
      await tester.pump(const Duration(milliseconds: 100));
      await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    }
  }

  if (desktop) {
    await tap('desktop-resource-center');
  } else {
    await tap('wb-tab-Home');
  }
  await tap('resource-section-local');
  await tap('resource-section-recent');
  await tap('resource-section-local');
  expect(tester.takeException(), isNull);
  await tap('resource-section-recent');
  await tap('resource-new');
  await tap('resource-creator-io.openmuse.siyuan/siyuan.document');
  final stamp = DateTime.now().microsecondsSinceEpoch;
  final name = 'Blocks-$stamp.sydoc';
  await wait(find.byKey(const ValueKey('resource-create-name')));
  await tester.enterText(
    find.byKey(const ValueKey('resource-create-name')),
    name,
  );
  await tester.pump();
  expect(
    tester
        .widget<TextField>(find.byKey(const ValueKey('resource-create-name')))
        .controller!
        .text,
    name,
  );
  await tap('resource-create-confirm');
  await wait(find.byType(WebViewWidget));
  WebViewController currentWeb = WebViewController.fromPlatform(
    tester
        .widget<WebViewWidget>(find.byType(WebViewWidget).last)
        .platform
        .params
        .controller,
  );
  WebViewController web() => currentWeb;
  Future<dynamic> js(String source) =>
      io(() => web().runJavaScriptReturningResult(source));
  Future<void> until(String expression) async {
    for (var i = 0; i < 240; i++) {
      await tester.pump(const Duration(milliseconds: 100));
      final result = await js('Boolean($expression)');
      if (result == true || result == 'true') return;
      await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    }
    fail(
      'Editor condition timed out: $expression; ${await js('document.body.innerText')}; ${await js('JSON.stringify({range:getSelection().rangeCount?getSelection().getRangeAt(0).getBoundingClientRect().toJSON():null,keyboard:document.getElementById("keyboardToolbar")?.getBoundingClientRect().toJSON(),keyboardClass:document.getElementById("keyboardToolbar")?.className,viewport:{height:innerHeight,visualHeight:visualViewport?.height,offset:visualViewport?.offsetTop},content:document.querySelector(".protyle-content")?.getBoundingClientRect().toJSON(),active:document.activeElement?.outerHTML.slice(0,200)})')}; ${tester.widgetList<SelectableText>(find.byType(SelectableText)).map((w) => w.data).join(' | ')}',
    );
  }

  await until('document.documentElement.dataset.resourceReady === "true"');
  if (desktop) {
    expect(
      find.text('文件预览 · 只读'),
      findsNothing,
      reason:
          'A resource session must not inherit the legacy file viewer banner',
    );
  }
  expect(
    await js(
      'Boolean(document.querySelector("#toolbar,#layouts,#dockLeft,#sidebar,#loading"))',
    ),
    anyOf(false, 'false'),
  );
  final checks = <String, bool>{
    'resourceCenterCreator': true,
    'deviceSourceNavigation': true,
    'standaloneEditorWithoutApplicationShell': true,
  };
  await until(
    'getComputedStyle(document.documentElement).getPropertyValue("--b3-font-family").trim().length > 0 && document.querySelector("svg symbol")',
  );
  checks['editorThemeAndIcons'] = true;
  expect(find.byKey(const ValueKey('resource-edit')), findsNothing);
  expect(find.byKey(const ValueKey('resource-save')), findsNothing);
  await js(
    'document.querySelector(".protyle-breadcrumb [data-type=readonly]").click(); true',
  );
  if (!desktop) {
    await until(
      'document.documentElement.dataset.openmuseResourcePlatform === "mobile"',
    );
    await until(
      'document.querySelector("#toolbarName") && document.querySelector(".protyle-breadcrumb [data-type=readonly]") && document.querySelector(".protyle-background__action [data-type=tag]") && document.querySelector("#keyboardToolbar .keyboard__bar")',
    );
    checks['nativeMobileDocumentStructure'] = true;
    await js(
      'document.querySelector(".protyle-background__action [data-type=icon]").click(); true',
    );
    await until(
      'document.querySelector(".emojis .emojis__item[data-unicode]")',
    );
    await visualCheckpoint('icon-picker');
    await js(
      'document.querySelector(".emojis .emojis__item[data-unicode]").click(); true',
    );
    await until(
      'document.querySelector(".protyle-background__icon").innerHTML.length > 0 && !document.querySelector(".emojis")',
    );
    checks['nativeIconPicker'] = true;
  }
  await until(
    'document.querySelector(".protyle-wysiwyg [contenteditable=true]")',
  );
  final marker = 'OpenMuse resource UI ${Platform.operatingSystem} $stamp';
  final body = desktop
      ? marker
      : '$marker\n${List.generate(18, (i) => 'Caret visibility line $i').join('\n')}';
  await js('''(() => {
    const el=document.querySelector('.protyle-wysiwyg [contenteditable=true]'); el.focus();
    const r=document.createRange();r.selectNodeContents(el);r.collapse(false);
    const s=getSelection();s.removeAllRanges();s.addRange(r);
    el.dispatchEvent(new MouseEvent('mouseup', {bubbles:true,button:0}));
    el.dispatchEvent(new MouseEvent('dblclick', {bubbles:true,button:0}));
    el.dispatchEvent(new KeyboardEvent('keyup', {bubbles:true,key:'ArrowRight',code:'ArrowRight'}));
    return document.execCommand('insertText',false,${jsonEncode(body)});
  })()''');
  await until(
    'document.querySelector(".protyle-wysiwyg").innerText.includes(${jsonEncode(marker)})',
  );
  await js('''(async () => {
    try {
      const form=new FormData();form.append('id',window.openMuseResourceEditor.protyle.block.rootID);
      form.append('file[]',new File(['<svg xmlns="http://www.w3.org/2000/svg" width="240" height="80"><rect width="240" height="80" fill="#1764ee"/><text x="18" y="48" font-size="22" fill="white">OpenMuse image</text></svg>'],'parity-image.svg',{type:'image/svg+xml'}));
      const upload=await (await fetch('/api/asset/upload',{method:'POST',body:form})).json();
      if(upload.code!==0)throw Error(upload.msg);
      const append=await (await fetch('/api/block/appendBlock',{method:'POST',body:JSON.stringify({parentID:window.openMuseResourceEditor.protyle.block.rootID,dataType:'markdown',data:'![Image rendering]('+upload.data.succFiles[0].path+')'})})).json();
      if(append.code!==0)throw Error(append.msg);
      document.documentElement.dataset.imageFixture='done';
    }catch(e){document.documentElement.dataset.imageFixture=String(e);}
  })();true''');
  await until('document.documentElement.dataset.imageFixture === "done"');
  if (!desktop) {
    // A real kernel attachment and original link click exercise the boundary
    // which previously dereferenced the absent application-shell websocket.
    await js('''(async () => {
      try {
        const form=new FormData();form.append('id',window.openMuseResourceEditor.protyle.block.rootID);
        form.append('file[]',new File([new TextEncoder().encode('Linked resource $stamp')],'linked-$stamp.txt',{type:'text/plain'}));
        const upload=await (await fetch('/api/asset/upload',{method:'POST',body:form})).json();
        if(upload.code!==0)throw Error(upload.msg);
        const path=upload.data.succFiles[0].path;
        const append=await (await fetch('/api/block/appendBlock',{method:'POST',body:JSON.stringify({parentID:window.openMuseResourceEditor.protyle.block.rootID,dataType:'markdown',data:'[Linked resource]('+path+')'})})).json();
        if(append.code!==0)throw Error(append.msg);
        document.documentElement.dataset.linkFixture='done';
      }catch(e){document.documentElement.dataset.linkFixture=String(e);}
    })();true''');
    await until('document.documentElement.dataset.linkFixture === "done"');
    await js(
      'document.getElementById("toolbarName").value="";document.getElementById("toolbarName").dispatchEvent(new Event("blur"));true',
    );
    await until(
      'document.getElementById("mobileTopBar").classList.contains("fn__none")',
    );
  }
  checks['nativeEditControlsAndAutosave'] = true;
  checks['noInjectedEditorToolbar'] = true;
  if (desktop) {
    // Close the single actual workspace tab through its existing close control.
    final tooltip = find.descendant(
      of: find
          .ancestor(
            of: find.textContaining(name).first,
            matching: find.byType(Row),
          )
          .first,
      matching: find.byTooltip('关闭'),
    );
    if (tooltip.evaluate().isNotEmpty) {
      await tester.tap(tooltip.first);
    } else {
      final close = find.byIcon(Icons.close).hitTestable();
      expect(close, findsWidgets);
      await tester.tap(close.last);
    }
    for (
      var i = 0;
      i < 120 && find.byType(WebViewWidget).evaluate().isNotEmpty;
      i++
    ) {
      await tester.pump(const Duration(milliseconds: 100));
      await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    }
    expect(
      find.byType(WebViewWidget),
      findsNothing,
      reason: 'Workspace tab must actually close before reopening',
    );
    await tap('desktop-resource-center');
  } else {
    await tester.tap(find.byType(BackButton).first);
    await wait(find.byKey(const ValueKey('resource-new')));
  }
  expect(find.byKey(const ValueKey('resource-keep-editing')), findsNothing);
  checks['automaticSaveAndCloseBarrier'] = true;
  final recent = find.text(name);
  await wait(recent);
  await tester.ensureVisible(recent.first);
  await tester.pumpAndSettle();
  await tester.tap(recent.first);
  await wait(find.byType(WebViewWidget));
  currentWeb = WebViewController.fromPlatform(
    tester
        .widget<WebViewWidget>(find.byType(WebViewWidget).last)
        .platform
        .params
        .controller,
  );
  await until('document.documentElement.dataset.resourceReady === "true"');
  await until(
    'document.querySelector(".protyle-wysiwyg").innerText.includes(${jsonEncode(marker)})',
  );
  expect(
    await js(
      'Boolean(document.querySelector(".protyle-wysiwyg [contenteditable=true]"))',
    ),
    anyOf(false, 'false'),
  );
  checks['recentReopenAndReadOnlyView'] = true;
  if (!desktop) {
    await until(
      'document.getElementById("mobileTopBar").classList.contains("fn__none") && document.getElementById("toolbarName").value === "" && document.querySelector("#editor > .protyle-breadcrumb").getBoundingClientRect().top <= 2',
    );
    checks['emptyDocumentTitleSuppressed'] = true;
  }
  await until(
    '[...document.querySelectorAll(".protyle-wysiwyg img")].some(e=>e.complete && e.naturalWidth===240 && e.naturalHeight===80)',
  );
  checks['packageImageRendering'] = true;
  await js(
    '[...document.querySelectorAll(".protyle-wysiwyg img")].find(e=>e.naturalWidth===240).scrollIntoView({block:"center"});true',
  );
  await visualCheckpoint('image-preview');
  checks['hostNavigationAndDocumentTitle'] = desktop
      ? find.text(name).evaluate().isNotEmpty
      : find
            .byKey(const ValueKey('resource-editor-title'))
            .evaluate()
            .isNotEmpty;
  Map<String, Object?>? keyboardMetrics;
  if (!desktop) {
    await until(
      '[...document.querySelectorAll(".protyle-wysiwyg [data-href]")].some(e=>e.textContent==="Linked resource")',
    );
    await js('''(() => {
      const link=[...document.querySelectorAll('.protyle-wysiwyg [data-type~="a"]')].find(e=>e.textContent==='Linked resource');
      link.scrollIntoView();const r=document.createRange();r.selectNodeContents(link);r.collapse(true);
      const s=getSelection();s.removeAllRanges();s.addRange(r);link.click();return true;
    })()''');
    await wait(find.byKey(const ValueKey('resource-notes-content')));
    expect(
      tester
          .widget<TextField>(
            find.byKey(const ValueKey('resource-notes-content')),
          )
          .controller!
          .text,
      contains('Linked resource $stamp'),
    );
    await visualCheckpoint('linked-document');
    await tester.tap(find.byType(BackButton).last);
    await tester.pumpAndSettle();
    await until('document.documentElement.dataset.resourceReady === "true"');
    checks['linkedDocumentHandoff'] = true;
    await js(
      'document.querySelector(".protyle-breadcrumb [data-type=readonly]").click(); true',
    );
    await until(
      'document.querySelector(".protyle-wysiwyg [contenteditable=true]")',
    );
    await js(
      '[...document.querySelectorAll(".protyle-wysiwyg [contenteditable=true]")].findLast(e=>e.textContent.includes("Caret visibility line")).scrollIntoView({block:"end"}); true',
    );
    await tester.pump(const Duration(milliseconds: 300));
    await io(() => Future<void>.delayed(const Duration(milliseconds: 300)));
    final position = await js('''(() => {
      const el=[...document.querySelectorAll('.protyle-wysiwyg [contenteditable=true]')].findLast(e=>e.textContent.includes('Caret visibility line'));
      const walker=document.createTreeWalker(el,NodeFilter.SHOW_TEXT);let n,last;
      while(n=walker.nextNode()){if(n.textContent.trim())last=n;}
      const caret=document.createRange();caret.setStart(last,Math.max(0,last.length-1));caret.setEnd(last,last.length);
      const r=caret.getBoundingClientRect();return JSON.stringify([r.right-1,r.top+r.height/2]);
    })()''');
    dynamic decodedPosition = position;
    while (decodedPosition is String) {
      decodedPosition = jsonDecode(decodedPosition);
    }
    final point = decodedPosition as List;
    await tester.tapAt(
      tester.getTopLeft(find.byType(WebViewWidget).last) +
          Offset((point[0] as num).toDouble(), (point[1] as num).toDouble()),
    );
    if (Platform.isIOS) {
      // UiKitView does not receive a real UITouch from Flutter's synthetic
      // pointer dispatch. Focus through the embedding WKWebView's supported
      // evaluateJavaScript API instead, then assert the real keyboard inset.
      await js('''(() => {
        const el=[...document.querySelectorAll('.protyle-wysiwyg [contenteditable=true]')].findLast(e=>e.textContent.includes('Caret visibility line'));
        el.focus({preventScroll:true});
        const walker=document.createTreeWalker(el,NodeFilter.SHOW_TEXT);let n,last;
        while(n=walker.nextNode()){if(n.textContent.trim())last=n;}
        const r=document.createRange();r.setStart(last,last.length);r.collapse(true);
        const s=getSelection();s.removeAllRanges();s.addRange(r);return true;
      })()''');
      await visualCheckpoint('focus-native');
      debugPrint(
        'SIYUAN_IOS_NATIVE_FOCUS_READY: tap the Caret visibility line paragraph on the unlocked iPhone',
      );
    }
    for (
      var i = 0;
      i < (Platform.isIOS ? 1200 : 120) &&
          tester.view.viewInsets.bottom <= 100 * tester.view.devicePixelRatio;
      i++
    ) {
      await tester.pump(const Duration(milliseconds: 100));
      await io(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    }
    expect(
      tester.view.viewInsets.bottom,
      greaterThan(100 * tester.view.devicePixelRatio),
      reason:
          'A full platform software IME must be visible, not just an accessory bar',
    );
    await until('''(() => {
      const s=getSelection();if(!s.rangeCount)return false;
      const r=s.getRangeAt(0).getBoundingClientRect(),t=document.getElementById('keyboardToolbar').getBoundingClientRect();
      const c=document.querySelector('.protyle-content').getBoundingClientRect();
      return t.height>0 && !document.getElementById('keyboardToolbar').classList.contains('fn__none') && r.height>0 && r.top>=c.top-1 && r.bottom<=Math.min(t.top,innerHeight)+1;
    })()''');
    final metrics = await js('''JSON.stringify((() => {
      const r=getSelection().getRangeAt(0).getBoundingClientRect(),t=document.getElementById('keyboardToolbar').getBoundingClientRect();
      return {caretBottom:r.bottom,toolbarTop:t.top,viewportHeight:innerHeight};
    })())''');
    dynamic decodedMetrics = metrics;
    while (decodedMetrics is String) {
      decodedMetrics = jsonDecode(decodedMetrics);
    }
    keyboardMetrics = Map<String, Object?>.from(decodedMetrics as Map)
      ..['physicalImeInset'] = tester.view.viewInsets.bottom;
    checks['keyboardViewportAndCaret'] = true;
    await visualCheckpoint('keyboard');
  }
  if (externalDocumentChecks &&
      Platform.isAndroid &&
      await io(() async {
        final support = await getApplicationSupportDirectory();
        return File('${support.path}/resource-word-attachment').exists();
      })) {
    final docx = Archive()
      ..addFile(
        ArchiveFile.string(
          '[Content_Types].xml',
          '<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>',
        ),
      )
      ..addFile(
        ArchiveFile.string(
          '_rels/.rels',
          '<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>',
        ),
      )
      ..addFile(
        ArchiveFile.string(
          'word/document.xml',
          '<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>OpenMuse Word link regression</w:t></w:r></w:p><w:sectPr/></w:body></w:document>',
        ),
      );
    final encoded = base64Encode(ZipEncoder().encode(docx));
    await js('''(async () => {
      try {
        const root=window.openMuseResourceEditor.protyle.block.rootID;
        const form=new FormData();form.append('id',root);
        form.append('file[]',new File([Uint8Array.from(atob('$encoded'),c=>c.charCodeAt(0))],'OpenMuse-Word-link.docx',{type:'application/vnd.openxmlformats-officedocument.wordprocessingml.document'}));
        const upload=await (await fetch('/api/asset/upload',{method:'POST',body:form})).json();
        if(upload.code!==0)throw Error(upload.msg);
        const append=await (await fetch('/api/block/appendBlock',{method:'POST',body:JSON.stringify({parentID:root,dataType:'markdown',data:'[OpenMuse Word link]('+upload.data.succFiles[0].path+')'})})).json();
        if(append.code!==0)throw Error(append.msg);
        await window.openMuseResource.openBlock(root);
        document.documentElement.dataset.wordFixture='done';
      }catch(e){document.documentElement.dataset.wordFixture=String(e);}
    })();true''');
    await until(
      'document.documentElement.dataset.wordFixture === "done" && [...document.querySelectorAll(".protyle-wysiwyg [data-href]")].some(e=>e.textContent==="OpenMuse Word link")',
    );
    await js('''(() => {
      const link=[...document.querySelectorAll('.protyle-wysiwyg [data-type~="a"]')].find(e=>e.textContent==='OpenMuse Word link');
      const r=document.createRange();r.selectNodeContents(link);r.collapse(true);
      const s=getSelection();s.removeAllRanges();s.addRange(r);link.click();return true;
    })()''');
    // The originating Activity may now be paused by the actual system viewer.
    // Keep this checkpoint on I/O rather than waiting for a Flutter frame.
    final support = await io(getApplicationSupportDirectory);
    await io(
      () => File(
        '${support.path}/resource-visual-ready.json',
      ).writeAsString('word-attachment'),
    );
    final release = File(
      '${support.path}/resource-visual-release-word-attachment',
    );
    final deadline = DateTime.now().add(const Duration(minutes: 3));
    while (!await io(release.exists) && DateTime.now().isBefore(deadline)) {
      await io(() => Future<void>.delayed(const Duration(milliseconds: 200)));
    }
    expect(
      await io(release.exists),
      isTrue,
      reason:
          'Native UI driver must verify the system Word viewer before releasing this checkpoint',
    );
    checks['systemWordDocumentOpen'] = true;
  }
  final result = <String, Object?>{
    'passed': checks.values.every((v) => v),
    'platform': Platform.operatingSystem,
    'checks': checks,
    'resource': name,
    'marker': marker,
    if (keyboardMetrics != null) 'keyboard': keyboardMetrics,
    'method':
        'Native Host UI and original Protyle DOM input; resource-center create, CAS save and recent reopen',
  };
  await io(() async {
    final support = await getApplicationSupportDirectory();
    await File(
      '${support.path}/siyuan-resource-ui.json',
    ).writeAsString(jsonEncode(result), flush: true);
  });
  return result;
}
