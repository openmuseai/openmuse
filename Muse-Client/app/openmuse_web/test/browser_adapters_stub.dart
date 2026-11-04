import 'package:flutter_test/flutter_test.dart';

void runBrowserAdapterTests() {
  test(
    'browser adapters are covered by the Chrome test target',
    () {},
    skip: 'Run flutter test --platform chrome test/browser_adapters_test.dart',
  );
}
