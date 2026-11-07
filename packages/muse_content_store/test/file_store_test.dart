import 'dart:io';
import 'package:muse_content_store/io.dart';
import 'package:test/test.dart';

void main() {
  test(
    'concurrent adapters never collide staging files or produce torn snapshots',
    () async {
      final dir = await Directory.systemTemp.createTemp('muse-snapshots-');
      final file = File('${dir.path}/private/cache.json');
      final writers = [FileSnapshotStore(file), FileSnapshotStore(file)];
      expect(await writers[0].read(), isNull);
      await Future.wait([
        for (var i = 0; i < 40; i++)
          writers[i % 2].write({'writer': i, 'payload': '$i' * 10000}),
      ]);
      final record = (await FileSnapshotStore(file).read())!;
      expect(record['payload'], '${record['writer']}' * 10000);
      expect(
        (await file.parent.list().toList()).whereType<Directory>(),
        isEmpty,
      );
      await dir.delete(recursive: true);
    },
  );
}
