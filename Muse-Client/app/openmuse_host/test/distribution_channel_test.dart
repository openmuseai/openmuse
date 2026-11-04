import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/distribution_channel.dart';

void main() {
  test('Store refuses terminal and external installation command routes', () {
    for (final command in [
      'terminal.session.create',
      'terminal.session.release',
      'plugin.installFromCatalog',
      'plugin.market.install',
      'plugin.market.selectProvider',
    ]) {
      expect(
        hostCommandAllowed(command, appStore: true),
        isFalse,
        reason: command,
      );
      expect(
        hostCommandAllowed(command, appStore: false),
        isTrue,
        reason: command,
      );
    }
    expect(
      hostCommandAllowed('workspace.openResource', appStore: true),
      isTrue,
    );
    expect(hostCommandAllowed('settings.plugin.read', appStore: true), isTrue);
  });
}
