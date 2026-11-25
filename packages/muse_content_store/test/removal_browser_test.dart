@TestOn('browser')
library;

import 'package:muse_content_store/browser.dart';
import 'package:test/test.dart';

void main() {
  test(
    'committed removal survives reopening and preserves other keys and namespaces',
    () async {
      final namespace = 'removal23-${DateTime.now().microsecondsSinceEpoch}';
      final target = IndexedDbSnapshotStore(
        namespace: namespace,
        objectId: 'target',
      );
      final peer = IndexedDbSnapshotStore(
        namespace: namespace,
        objectId: 'peer',
      );
      final other = IndexedDbSnapshotStore(
        namespace: '$namespace-other',
        objectId: 'target',
      );
      addTearDown(() async {
        await target.remove();
        await peer.remove();
        await other.remove();
      });
      await target.remove();
      await target.write({'payload': 'original'});
      await peer.write({'payload': 'peer'});
      await other.write({'payload': 'other namespace'});
      expect(await target.read(), {'payload': 'original'});
      await target.remove();
      expect(
        await IndexedDbSnapshotStore(
          namespace: namespace,
          objectId: 'target',
        ).read(),
        isNull,
      );
      expect(await peer.read(), {'payload': 'peer'});
      expect(await other.read(), {'payload': 'other namespace'});
      await target.remove();
      await target.write({'payload': 'fresh original read'});
      expect(await target.read(), {'payload': 'fresh original read'});
    },
  );
}
