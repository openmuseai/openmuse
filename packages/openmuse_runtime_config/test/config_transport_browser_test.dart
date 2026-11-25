@TestOn('browser')
library;

import 'dart:js_interop';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/src/transport_browser.dart';
import 'package:web/web.dart' as web;

@JS('window.fetch')
external JSFunction get originalFetch;
@JS('window.fetch')
external set replacementFetch(JSFunction value);

void main() {
  test(
    'browser fetch omits credentials, rejects redirects and bounds streamed data',
    () async {
      final original = originalFetch;
      addTearDown(() => replacementFetch = original);
      replacementFetch = ((JSAny input, web.RequestInit options) {
        expect(options.credentials, 'omit');
        expect(options.redirect, 'error');
        return Future.value(web.Response('verified'.toJS)).toJS;
      }).toJS;
      final transport = BrowserConfigTransport();
      expect(
        await transport.get(
          Uri.parse('https://config.example.test/active.jws'),
          maxBytes: 20,
        ),
        'verified'.codeUnits,
      );
      await expectLater(
        transport.get(
          Uri.parse('https://config.example.test/large.json'),
          maxBytes: 2,
        ),
        throwsFormatException,
      );
    },
  );
}
