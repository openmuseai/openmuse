import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config_core.dart';

void main() {
  InstalledRelease load(String market) => InstalledRelease.parse(
    File('assets/releases/$market.json').readAsStringSync(),
  );
  test(
    'domestic fixed China and overseas explicit US selection keep separate trust and credentials',
    () async {
      final domestic = load('china'), overseas = load('overseas');
      final cn = await RegionSelection(domestic, MemoryConfigStore()).restore();
      expect(cn!.id, 'cn');
      expect(domestic.regions.length, 1);
      final store = MemoryConfigStore(),
          selection = RegionSelection(overseas, store);
      expect(await selection.restore(), isNull);
      expect(overseas.regions.where((v) => v.available).map((v) => v.id), [
        'us',
      ]);
      await expectLater(selection.choose('eu'), throwsStateError);
      final us = await selection.choose('us');
      expect((await RegionSelection(overseas, store).restore())!.id, 'us');
      expect(
        us.bundle!.snapshot.authSessionStorageKey,
        isNot(cn.bundle!.snapshot.authSessionStorageKey),
      );
      expect(await selection.hasAcceptedLegal(us), isFalse);
      await selection.acceptLegal(us);
      expect(
        await RegionSelection(overseas, store).hasAcceptedLegal(us),
        isTrue,
      );
      expect((await RegionSelection(overseas, store).restore())!.id, 'us');
      await store.commit('corrupted');
      expect(await selection.hasAcceptedLegal(us), isFalse);
      expect(await selection.restore(), isNull);
    },
  );
  test(
    'unavailable regions cannot smuggle endpoints and overseas cannot include a China binding',
    () {
      final document = jsonDecode(
        File('assets/releases/overseas.json').readAsStringSync(),
      );
      document['regions'][1]['bundle'] = document['regions'][0]['bundle'];
      expect(
        () => InstalledRelease.parse(jsonEncode(document)),
        throwsFormatException,
      );
      document['regions'].removeAt(1);
      document['regions'][0] = jsonDecode(
        File('assets/releases/china.json').readAsStringSync(),
      )['regions'][0];
      expect(
        () => InstalledRelease.parse(jsonEncode(document)),
        throwsFormatException,
      );
    },
  );
}
