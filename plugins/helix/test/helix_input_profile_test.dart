import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  test('legacy shortcut preference never becomes nonmodal', () {
    final legacy = HelixPreferences.fromJson({'vscodeKeymap': true});
    expect(legacy.inputProfile, HelixInputProfile.helixModal);
    expect(legacy.configToml, isNot(contains('input-profile')));
    expect(
      legacy
          .copyWith(inputProfile: HelixInputProfile.standardNonmodal)
          .configToml,
      contains('input-profile = "standard-nonmodal"'),
    );
    expect(
      HelixPreferences.fromJson(
        legacy
            .copyWith(inputProfile: HelixInputProfile.standardNonmodal)
            .toJson(),
      ).inputProfile,
      HelixInputProfile.standardNonmodal,
    );
  });

  test(
    'unsupported bundled engine refuses nonmodal instead of falling back',
    () async {
      final runtime = HelixRuntimePool(
        executable: '/openmuse/missing-helix-binary',
      );
      addTearDown(runtime.dispose);
      expect(await runtime.probeCapabilities(), isFalse);
      await expectLater(
        runtime.configure(
          const HelixPreferences(
            inputProfile: HelixInputProfile.standardNonmodal,
          ),
        ),
        throwsStateError,
      );
      expect(runtime.preferences.inputProfile, HelixInputProfile.helixModal);
    },
  );

  test(
    'repository-local macOS engine advertises its input capability',
    () async {
      if (!Platform.isMacOS) return;
      final executable = File('assets/engines/helix/hx').absolute.path;
      final runtime = HelixRuntimePool(executable: executable);
      addTearDown(runtime.dispose);
      expect(await runtime.probeCapabilities(), isTrue);
      expect(runtime.supportsNonmodal, isTrue);
    },
  );
}
