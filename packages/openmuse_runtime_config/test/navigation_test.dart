import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  Map<String, Object?> config(String path, {String handler = 'session'}) => {
    'entryRouteId': 'home',
    'routes': {
      'home': {'path': path, 'handler': handler},
    },
  };
  test('semantic routes use configured local paths and installed handlers', () {
    final navigation = RuntimeNavigation.fromJson(config('/workspace'));
    expect(navigation.path('home'), '/workspace');
    expect(
      navigation.resolve('/workspace?source=link')?.handler,
      RuntimeRouteHandler.session,
    );
    expect(navigation.resolve('https://other.example/workspace'), isNull);
    expect(navigation.resolve('/unknown'), isNull);
    for (final path in [
      '//other.example',
      '/%2e%2e/x',
      '/%00',
      '/x%2fy',
      '/x%252fy',
      '/x#hash',
    ]) {
      expect(
        () => RuntimeNavigation.fromJson(config(path)),
        throwsFormatException,
      );
    }
    expect(
      () => RuntimeNavigation.fromJson(
        config('/custom', handler: 'downloaded-code'),
      ),
      throwsFormatException,
    );
  });
}
