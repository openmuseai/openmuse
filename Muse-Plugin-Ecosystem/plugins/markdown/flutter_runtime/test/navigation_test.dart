import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/src/navigation.dart';

void main() {
  final origin = Uri.parse('http://127.0.0.1:54321');
  test('local editor navigation and blank frames stay inside the plugin', () {
    expect(
      classifyClientNavigation(origin, '$origin/stage/build/mobile/'),
      ClientNavigation.local,
    );
    expect(
      classifyClientNavigation(origin, 'about:blank'),
      ClientNavigation.local,
    );
  });
  test(
    'external links use the system application without exposing file URLs',
    () {
      expect(
        classifyClientNavigation(origin, 'https://example.org/'),
        ClientNavigation.external,
      );
      expect(
        classifyClientNavigation(origin, 'mailto:hello@example.org'),
        ClientNavigation.external,
      );
      expect(
        classifyClientNavigation(origin, 'file:///private/document'),
        ClientNavigation.blocked,
      );
      expect(
        classifyClientNavigation(origin, 'javascript:alert(1)'),
        ClientNavigation.blocked,
      );
    },
  );
}
