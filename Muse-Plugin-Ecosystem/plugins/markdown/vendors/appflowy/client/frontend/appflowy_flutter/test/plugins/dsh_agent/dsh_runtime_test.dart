import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('looksLikeMuseRoot requires middlewares/dsh and frontend/client', () {
    final root = Directory.systemTemp.createTempSync('muse-root-');
    addTearDown(() => root.deleteSync(recursive: true));
    expect(DshRuntimeLayout.looksLikeMuseRoot(root.path), isFalse);
    Directory('${root.path}/middlewares/dsh').createSync(recursive: true);
    expect(DshRuntimeLayout.looksLikeMuseRoot(root.path), isFalse);
    Directory('${root.path}/frontend/client').createSync(recursive: true);
    expect(DshRuntimeLayout.looksLikeMuseRoot(root.path), isTrue);
  });

  test('discoverMuseRoot walks up from a nested windows build dir', () {
    final root = Directory.systemTemp.createTempSync('muse-walk-');
    addTearDown(() => root.deleteSync(recursive: true));
    Directory('${root.path}/middlewares/dsh').createSync(recursive: true);
    Directory('${root.path}/frontend/client').createSync(recursive: true);
    final nested = Directory(
      '${root.path}/frontend/client/frontend/appflowy_flutter/build/windows/x64/runner/Debug',
    )..createSync(recursive: true);
    expect(
      DshRuntimeLayout.discoverMuseRoot(searchFrom: [nested.path]),
      root.path,
    );
  });

  test('source layout points at middlewares run-dsh-appflowy.sh', () {
    final root = Directory.systemTemp.createTempSync('muse-script-');
    addTearDown(() => root.deleteSync(recursive: true));
    Directory('${root.path}/middlewares/dsh').createSync(recursive: true);
    Directory('${root.path}/frontend/client').createSync(recursive: true);
    final layout = DshRuntimeLayout(
      bundled: false,
      museRoot: root.path,
      dshHome: '${root.path}/dsh',
      harnessDir: '${root.path}/vendors/deepseek-harness',
      patchFile:
          '${root.path}/middlewares/dsh/plugins/dsh-appflowy/cordis.patch.yml',
      nodeBin: null,
      credentialsFile: '${root.path}/credentials.env',
    );
    expect(
      layout.runDshScript.replaceAll('\\', '/'),
      endsWith('middlewares/scripts/run-dsh-appflowy.sh'),
    );
    expect(layout.runDshScript.contains('/Users/mac/src/muse'), isFalse);
  });

  test('looksLikeBundleRoot and bundledNodePath accept Windows zip layout', () {
    final root = Directory.systemTemp.createTempSync('muse-bundle-');
    addTearDown(() => root.deleteSync(recursive: true));
    expect(DshRuntimeLayout.looksLikeBundleRoot(root.path), isFalse);
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    Directory('${root.path}/dsh').createSync();
    expect(DshRuntimeLayout.looksLikeBundleRoot(root.path), isTrue);
    expect(DshRuntimeLayout.bundledNodePath(root.path), isNull);
    File('${root.path}/node/node.exe').createSync(recursive: true);
    expect(
      DshRuntimeLayout.bundledNodePath(root.path)!
          .replaceAll('\\', '/')
          .endsWith('node/node.exe'),
      isTrue,
    );
  });

  test('looksLikeBundleRoot and closureEntryPath accept closure layout', () {
    final root = Directory.systemTemp.createTempSync('muse-closure-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    Directory('${root.path}/closure').createSync();
    expect(DshRuntimeLayout.looksLikeBundleRoot(root.path), isTrue);
    expect(DshRuntimeLayout.closureEntryPath(root.path), isNull);
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    ).createSync(recursive: true);
    expect(
      DshRuntimeLayout.closureEntryPath(root.path)!
          .replaceAll('\\', '/')
          .endsWith('closure/node_modules/@deepseek-ai/dsh/lib/bin.js'),
      isTrue,
    );
  });

  test('bundled closure layout points harnessDir at closure not dsh', () {
    final root = Directory.systemTemp.createTempSync('muse-closure-layout-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    ).createSync(recursive: true);
    File('${root.path}/node/node.exe').createSync(recursive: true);
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: '${root.path}/dsh-home',
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    expect(layout.bundled, isTrue);
    expect(layout.closureEntry, isNotNull);
    expect(
      layout.harnessDir.replaceAll('\\', '/'),
      endsWith('/closure'),
    );
  });
}
