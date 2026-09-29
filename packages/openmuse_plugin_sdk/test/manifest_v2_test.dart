import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

const fixtureNames = [
  'helix.json',
  'dsh-agent.json',
  'native-text-gate.json',
  'open-file-viewer.json',
];

Future<Map<String, Object?>> fixture(String name) async =>
    (jsonDecode(
              await File(
                '../../schemas/fixtures/plugin/v2/$name',
              ).readAsString(),
            )
            as Map)
        .cast<String, Object?>();

const android = OpenMuseTarget(
  os: OpenMuseTargetOs.android,
  arch: OpenMuseTargetArch.aarch64,
  libc: OpenMuseTargetLibc.bionic,
);

const ios = OpenMuseTarget(
  os: OpenMuseTargetOs.ios,
  arch: OpenMuseTargetArch.aarch64,
  libc: OpenMuseTargetLibc.darwin,
);

void main() {
  for (final name in fixtureNames) {
    test('$name round trips with explicit mobile decisions', () async {
      final input = await fixture(name);
      final manifest = OpenMusePluginManifestV2.fromJson(input);
      expect(manifest.toJson(), equals(input));
      expect(manifest.compatibility, hasLength(6));
      for (final target in [android, ios]) {
        expect(
          manifest.compatibility
              .singleWhere((decision) => decision.target == target)
              .status,
          OpenMuseTargetStatus.unsupported,
        );
        expect(
          () => manifest.resolveArtifacts(target),
          throwsA(isA<OpenMuseManifestFormatException>()),
        );
      }
    });
  }

  test('unknown fields and targets fail closed', () async {
    final withUnknownField = await fixture('helix.json');
    withUnknownField['future_grant'] = true;
    expect(
      () => OpenMusePluginManifestV2.fromJson(withUnknownField),
      throwsA(isA<OpenMuseManifestFormatException>()),
    );

    final withUnknownContribution = await fixture('helix.json');
    final contributions = withUnknownContribution['contributes']! as Map;
    final commands = contributions['commands']! as List;
    (commands.first as Map)['ambient_authority'] = true;
    expect(
      () => OpenMusePluginManifestV2.fromJson(withUnknownContribution),
      throwsA(isA<OpenMuseManifestFormatException>()),
    );

    final withUnknownTarget = await fixture('helix.json');
    final compatibility = withUnknownTarget['compatibility']! as Map;
    final targets = compatibility['targets']! as List;
    final decision = targets.first as Map;
    final target = decision['target']! as Map;
    target['os'] = 'plan9';
    expect(
      () => OpenMusePluginManifestV2.fromJson(withUnknownTarget),
      throwsA(isA<OpenMuseManifestFormatException>()),
    );
  });

  test(
    'Agent CLI schema is typed independently from UI contributions',
    () async {
      final manifest = OpenMusePluginManifestV2.fromJson(
        await fixture('dsh-agent.json'),
      );
      final command = manifest.contributes.agentCli.single;
      expect(command.identity, 'agent/dsh/prompt');
      expect(command.requiredPermissions, contains('credentials.model.use'));
      expect(command.effects, contains('workspace.read'));
    },
  );
}
