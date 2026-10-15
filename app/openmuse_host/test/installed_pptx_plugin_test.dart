import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/installed_pptx_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test('installed PPTX worker contributes a Desktop editor', () async {
    final directory = Directory.systemTemp.createTempSync('pptx-adapter-test-');
    addTearDown(() => directory.deleteSync(recursive: true));
    final worker = File('${directory.path}/worker.mjs')
      ..writeAsStringSync('''
process.stdout.write(JSON.stringify({
  schema: 'openmuse.pptx.inspect/v1',
  slides: 1,
  pages: [{index: 0, elements: 2}]
}));
''');
    final file = File('${directory.path}/deck.pptx')
      ..writeAsBytesSync([1, 2, 3]);
    final node = (await Process.run('which', ['node'])).stdout.toString().trim();
    final plugin = OpenMuseInstalledPptxPlugin(
      workerPath: worker.path,
      nodeExecutable: node,
    );
    final resource = OpenMuseResource(uri: file.uri, displayName: 'deck.pptx');
    expect(plugin.descriptor.editors.single.accepts(resource), isTrue);
    final result = await inspectInstalledPptx(
      workerPath: worker.path,
      nodeExecutable: node,
      filePath: file.path,
    );
    expect(result['slides'], 1);
    expect((result['pages'] as List).single['elements'], 2);
  });
}
