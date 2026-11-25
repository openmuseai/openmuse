import 'dart:io';
import 'package:muse_content_store/io.dart';
import 'package:test/test.dart';

void main() {
  test(
    'exact file retirement is idempotent and preserves peer snapshots',
    () async {
      final dir = await Directory.systemTemp.createTemp('snapshot-removal23-');
      addTearDown(() => dir.delete(recursive: true));
      final target = FileSnapshotStore(File('${dir.path}/target.json'));
      final peer = FileSnapshotStore(File('${dir.path}/peer.json'));
      await target.remove();
      await target.write({'original': 1});
      await peer.write({'peer': 2});
      await target.remove();
      await target.remove();
      expect(
        await FileSnapshotStore(File('${dir.path}/target.json')).read(),
        isNull,
      );
      expect(await peer.read(), {'peer': 2});
      await target.write({'fresh': 3});
      expect(await target.read(), {'fresh': 3});
    },
  );
  test('retirement does not swallow storage errors', () async {
    final dir = await Directory.systemTemp.createTemp(
      'snapshot-removal-error23-',
    );
    addTearDown(() => dir.delete(recursive: true));
    final target = Directory('${dir.path}/target.json');
    await target.create();
    await File('${target.path}/preserved').writeAsString('keep');
    await expectLater(
      FileSnapshotStore(File(target.path)).remove(),
      throwsA(isA<FileSystemException>()),
    );
    expect(await File('${target.path}/preserved').readAsString(), 'keep');
  });
}
