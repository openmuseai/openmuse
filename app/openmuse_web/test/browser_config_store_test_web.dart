@TestOn('browser')
library;

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_web/auth/browser_config_store.dart';

void main() {
  test(
    'IndexedDB restores whole records without crossing deployment identities',
    () async {
      final run = DateTime.now().microsecondsSinceEpoch;
      final first = BrowserConfigStore('test.$run.first');
      final second = BrowserConfigStore('test.$run.second');
      expect(await first.read(), isNull);
      await first.commit('{"jws":"first","resources":{"a":"data"}}');
      expect(await second.read(), isNull);
      await second.commit('{"jws":"second","resources":{}}');
      await first.commit('{"jws":"new","resources":{"b":"new-data"}}');
      expect(
        await BrowserConfigStore('test.$run.first').read(),
        '{"jws":"new","resources":{"b":"new-data"}}',
      );
      expect(await second.read(), '{"jws":"second","resources":{}}');
    },
  );
}
