import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('host-owned DSH packages stay out of the profile seed', () {
    expect(DshSidecar.isHostOwnedSeedPackage('@deepseek-ai/cordis'), isTrue);
    expect(
      DshSidecar.isHostOwnedSeedPackage('@deepseek-ai/cordis-plugin-timer'),
      isTrue,
    );
    expect(DshSidecar.isHostOwnedSeedPackage('@muse/dsh-appflowy'), isFalse);
    expect(DshSidecar.isHostOwnedSeedPackage('dshmarket'), isFalse);
    expect(DshSidecar.isHostOwnedSeedPackage('js-yaml'), isFalse);
  });

  test('seed generation changes with install path and overlay stamp', () {
    final root = Directory.systemTemp.createTempSync('muse-seed-gen-');
    addTearDown(() => root.deleteSync(recursive: true));
    final connector = File(
      '${root.path}/closure/node_modules/@muse/dsh-appflowy/dist/src/connector.js',
    )..createSync(recursive: true);
    connector.writeAsStringSync('export {}\n');
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(jsonEncode({'name': '@deepseek-ai/dsh', 'version': '0.1.2'}));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: '${root.path}/dsh-home',
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    final first = DshSidecar.closureSeedGeneration(layout);
    expect(first, contains('0.1.2'));
    expect(first, contains(root.path));
    final moved = DshRuntimeLayout(
      bundled: true,
      museRoot: '${root.path}/elsewhere',
      dshHome: layout.dshHome,
      harnessDir: layout.harnessDir,
      patchFile: layout.patchFile,
      nodeBin: layout.nodeBin,
      credentialsFile: layout.credentialsFile,
      closureEntry: layout.closureEntry,
    );
    expect(DshSidecar.closureSeedGeneration(moved), isNot(first));
  });

  test('re-seed removes a leaked @deepseek-ai profile scope', () {
    final root = Directory.systemTemp.createTempSync('muse-seed-host-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(jsonEncode({'name': '@deepseek-ai/dsh', 'version': '1.0.0'}));
    File(
      '${root.path}/closure/node_modules/@muse/dsh-appflowy/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(
        jsonEncode({
          'name': '@muse/dsh-appflowy',
          'version': '0.1.0',
          'peerDependencies': {'@deepseek-ai/cordis': '1.0.0'},
        }),
      );
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/cordis/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(jsonEncode({'name': '@deepseek-ai/cordis'}));
    File(
      '${root.path}/closure/node_modules/dshmarket/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(jsonEncode({'name': 'dshmarket'}));
    final dshHome = Directory('${root.path}/dsh-home')..createSync();
    final leaked = Directory(
      '${dshHome.path}/profiles/web/node_modules/@deepseek-ai/cordis',
    )..createSync(recursive: true);
    File('${leaked.path}/package.json').writeAsStringSync('{}');
    File('${dshHome.path}/profiles/web/.muse-seeded').writeAsStringSync('1');
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: dshHome.path,
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    DshSidecar.seedClosurePlugins(layout);
    expect(
      Directory(
        '${dshHome.path}/profiles/web/node_modules/@deepseek-ai',
      ).existsSync(),
      isFalse,
    );
    expect(
      File(
        '${dshHome.path}/profiles/web/node_modules/@muse/dsh-appflowy/package.json',
      ).existsSync(),
      isTrue,
    );
    expect(
      File('${dshHome.path}/profiles/web/.muse-seeded').readAsStringSync(),
      DshSidecar.closureSeedGeneration(layout),
    );
  });

  test('seeds the vendored bare-name plugin that patch.yml mounts', () {
    // `name: dsh-model-capabilities` is not under the @muse scope and is not a
    // dependency of dshmarket, so only an explicit seed root gets it into the
    // profile. Without it the packed row resolves to nothing at boot.
    final root = Directory.systemTemp.createTempSync('muse-seed-vendored-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync(
      '- insert:\n'
      '    - id: model-capabilities\n'
      '      name: dsh-model-capabilities\n',
    );
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(
        jsonEncode({'name': '@deepseek-ai/dsh', 'version': '1.0.0'}),
      );
    File(
      '${root.path}/closure/node_modules/dsh-model-capabilities/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(jsonEncode({'name': 'dsh-model-capabilities'}));
    final dshHome = Directory('${root.path}/dsh-home')..createSync();
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: dshHome.path,
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    DshSidecar.seedClosurePlugins(layout);
    expect(
      File(
        '${dshHome.path}/profiles/web/node_modules/dsh-model-capabilities/package.json',
      ).existsSync(),
      isTrue,
    );
  });

  test('a valid seed marker does not keep leaked host packages alive', () {
    // The generation marker stays valid across boots, so seedClosurePlugins
    // returns early and a scope that leaked in afterwards would shadow the
    // installation fallback forever: Node resolves @deepseek-ai/* from the
    // profile first and dies with ERR_MODULE_NOT_FOUND ("Did you mean
    // …/lib/index.js?") for exactly the leaked packages.
    final root = Directory.systemTemp.createTempSync('muse-seed-leak-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    File(
      '${root.path}/closure/node_modules/@deepseek-ai/dsh/package.json',
    )
      ..createSync(recursive: true)
      ..writeAsStringSync(
        jsonEncode({'name': '@deepseek-ai/dsh', 'version': '1.0.0'}),
      );
    final dshHome = Directory('${root.path}/dsh-home')..createSync();
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: dshHome.path,
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    // Marker matches the current generation, so the seed below is a no-op.
    Directory('${dshHome.path}/profiles/web').createSync(recursive: true);
    File('${dshHome.path}/profiles/web/.muse-seeded')
        .writeAsStringSync(DshSidecar.closureSeedGeneration(layout));

    final profileLeak = Directory(
      '${dshHome.path}/profiles/web/node_modules/@deepseek-ai/cordis',
    )..createSync(recursive: true);
    File('${profileLeak.path}/index.js').writeAsStringSync('export {}\n');
    final fallback = Directory(
      '${dshHome.path}/profiles/node_modules/@deepseek-ai',
    )..createSync(recursive: true);
    final fallbackLeak = Directory('${fallback.path}/cordis-plugin-timer')
      ..createSync(recursive: true);
    File('${fallbackLeak.path}/index.js').writeAsStringSync('export {}\n');
    // dsh's own proxy packages carry a dsh.moduleFallback record and must stay.
    final proxy = Directory('${fallback.path}/dsh-client-ui-chat')
      ..createSync(recursive: true);
    File('${proxy.path}/package.json').writeAsStringSync(
      jsonEncode({
        'name': '@deepseek-ai/dsh-client-ui-chat',
        'dsh': {
          'moduleFallback': {'targets': <String>[]},
        },
      }),
    );
    // A healthy fallback entry: a link into this installation whose package
    // really resolves, so the repair must keep it.
    final good = Directory(
      '${root.path}/closure/node_modules/@deepseek-ai/good',
    )..createSync(recursive: true);
    File('${good.path}/package.json').writeAsStringSync(
      jsonEncode({'name': '@deepseek-ai/good', 'main': 'lib/index.js'}),
    );
    File('${good.path}/lib/index.js')
      ..createSync(recursive: true)
      ..writeAsStringSync('export {}\n');
    var linked = false;
    try {
      Link('${fallback.path}/dsh-client-ui-workflow-run')
          .createSync(good.path);
      linked = true;
    } catch (_) {}

    DshSidecar.seedClosurePlugins(layout);
    expect(
      Directory('${dshHome.path}/profiles/web/node_modules/@deepseek-ai')
          .existsSync(),
      isTrue,
      reason: 'the seed is generation-gated, so it leaves the leak in place',
    );

    DshSidecar.clearLeakedHostPackages(layout);
    expect(
      Directory('${dshHome.path}/profiles/web/node_modules/@deepseek-ai/cordis')
          .existsSync(),
      isFalse,
      reason: 'a manifest-less copy must never shadow the installation',
    );
    expect(fallbackLeak.existsSync(), isFalse);
    expect(File('${proxy.path}/package.json').existsSync(), isTrue);
    if (linked) {
      expect(
        FileSystemEntity.typeSync(
          '${fallback.path}/dsh-client-ui-workflow-run',
          followLinks: false,
        ),
        FileSystemEntityType.link,
      );
    }
  });

  test('module fallback repair drops every entry that cannot resolve', () {
    // Node stops at the first node_modules holding a directory with the
    // package's name, so an entry that cannot resolve shadows the installation
    // fallback for the whole profile: a plain directory, a dangling link, a
    // half-written copy (manifest but no entry file), or a link into another
    // installation. All of them must go, and dsh re-heals what it needs.
    final root = Directory.systemTemp.createTempSync('muse-fallback-repair-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/patch.yml').writeAsStringSync('[]\n');
    final installation = '${root.path}/closure/node_modules/@deepseek-ai';

    void addPackage(String dir, {String? main, bool writeEntry = true}) {
      Directory(dir).createSync(recursive: true);
      File('$dir/package.json').writeAsStringSync(
        jsonEncode({
          'name': '@deepseek-ai/${dir.split('/').last}',
          if (main != null) 'main': main,
        }),
      );
      if (main == null || !writeEntry) return;
      final entry = File('$dir/$main');
      entry.parent.createSync(recursive: true);
      entry.writeAsStringSync('export {}\n');
    }

    addPackage('$installation/good', main: 'lib/index.js');
    addPackage('$installation/halfcopied', main: 'lib/index.js', writeEntry: false);
    // Another installation lives outside this one's museRoot.
    final foreignRoot = Directory.systemTemp.createTempSync('muse-foreign-');
    addTearDown(() => foreignRoot.deleteSync(recursive: true));
    addPackage('${foreignRoot.path}/elsewhere/foreign', main: 'lib/index.js');

    final dshHome = Directory('${root.path}/dsh-home')..createSync();
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: dshHome.path,
      harnessDir: '${root.path}/closure',
      patchFile: '${root.path}/patch.yml',
      nodeBin: '${root.path}/node/node.exe',
      credentialsFile: '${root.path}/credentials.env',
      closureEntry:
          '${root.path}/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    final profileScope = Directory(
      '${dshHome.path}/profiles/web/node_modules/@deepseek-ai',
    )..createSync(recursive: true);
    final fallbackScope = Directory(
      '${dshHome.path}/profiles/node_modules/@deepseek-ai',
    )..createSync(recursive: true);
    final ownedScope = Directory(
      '${dshHome.path}/profiles/web/.dsh-module-fallback/node_modules/@deepseek-ai',
    )..createSync(recursive: true);

    // A manifest-less copy and a half-written package: both are what the
    // sidecar dies on, because Node reports them as
    // `Cannot find package …/index.js` + "Did you mean …/lib/index.js?".
    Directory('${profileScope.path}/cordis').createSync(recursive: true);
    final halfCopied = Directory('${profileScope.path}/dsh-client-hmr')
      ..createSync(recursive: true);
    File('${halfCopied.path}/package.json').writeAsStringSync(
      jsonEncode({'name': '@deepseek-ai/dsh-client-hmr', 'main': 'lib/index.js'}),
    );
    Directory('${ownedScope.path}/dsh-llm').createSync(recursive: true);

    var linked = false;
    try {
      Link('${profileScope.path}/dsh-client-ui-chat').createSync('$installation/good');
      Link('${profileScope.path}/dsh-client-ui-jobs').createSync('$installation/halfcopied');
      Link('${profileScope.path}/dsh-client-ui-goal').createSync('${foreignRoot.path}/elsewhere/foreign');
      Link('${profileScope.path}/dsh-client-ui-plan').createSync('${root.path}/does-not-exist');
      Link('${fallbackScope.path}/dsh-client-ui-reference').createSync('$installation/halfcopied');
      Link('${ownedScope.path}/dsh-session').createSync('$installation/halfcopied');
      linked = true;
    } catch (_) {}

    DshSidecar.clearLeakedHostPackages(layout);

    expect(Directory('${profileScope.path}/cordis').existsSync(), isFalse);
    expect(halfCopied.existsSync(), isFalse);
    expect(
      Directory('${ownedScope.path}/dsh-llm').existsSync(),
      isFalse,
      reason: 'the profile-owned fallback is a resolution step too',
    );
    if (linked) {
      // The only entry that resolves into this installation survives.
      expect(
        FileSystemEntity.typeSync(
          '${profileScope.path}/dsh-client-ui-chat',
          followLinks: false,
        ),
        FileSystemEntityType.link,
      );
      for (final gone in const [
        'dsh-client-ui-jobs', // link to a package whose entry file is missing
        'dsh-client-ui-goal', // link into another installation
        'dsh-client-ui-plan', // dangling link
      ]) {
        expect(
          FileSystemEntity.typeSync(
            '${profileScope.path}/$gone',
            followLinks: false,
          ),
          FileSystemEntityType.notFound,
          reason: '$gone cannot resolve and must be rebuilt by dsh',
        );
      }
      expect(
        FileSystemEntity.typeSync(
          '${fallbackScope.path}/dsh-client-ui-reference',
          followLinks: false,
        ),
        FileSystemEntityType.notFound,
      );
      expect(
        FileSystemEntity.typeSync(
          '${ownedScope.path}/dsh-session',
          followLinks: false,
        ),
        FileSystemEntityType.notFound,
      );
    }
  });

  test('upsertEnvFile keeps sibling secrets', () {
    const existing = 'DEEPSEEK_API_KEY=sk-old\nOPENCODE_GO_API_KEY=go-key\n';
    final next = DshSidecar.upsertEnvFile(
      existing,
      'OPENCODE_CUSTOM_API_KEY',
      'sk-custom',
    );
    expect(next, contains('DEEPSEEK_API_KEY=sk-old'));
    expect(next, contains('OPENCODE_GO_API_KEY=go-key'));
    expect(next, contains('OPENCODE_CUSTOM_API_KEY=sk-custom'));
    final replaced = DshSidecar.upsertEnvFile(
      next,
      'DEEPSEEK_API_KEY',
      'sk-new',
    );
    expect(replaced, contains('DEEPSEEK_API_KEY=sk-new'));
    expect(replaced, contains('OPENCODE_CUSTOM_API_KEY=sk-custom'));
    expect(replaced, isNot(contains('sk-old')));
  });

  test('seedGatewaySettings copies defaults once', () {
    final root = Directory.systemTemp.createTempSync('muse-gw-seed-');
    addTearDown(() => root.deleteSync(recursive: true));
    File('${root.path}/defaults/settings.yaml')
      ..createSync(recursive: true)
      ..writeAsStringSync('agent-default-model:\n  provider: opencode-custom\n');
    final layout = DshRuntimeLayout(
      bundled: true,
      museRoot: root.path,
      dshHome: '${root.path}/dsh-home',
      harnessDir: '${root.path}/dsh',
      patchFile: '${root.path}/patch.yml',
      nodeBin: null,
      credentialsFile: '${root.path}/credentials.env',
    );
    DshSidecar.seedGatewaySettings(layout);
    final dest = File('${layout.dshHome}/settings.yaml');
    expect(dest.existsSync(), isTrue);
    expect(dest.readAsStringSync(), contains('opencode-custom'));
    dest.writeAsStringSync('keep-me:\n  yes: 1\n');
    DshSidecar.seedGatewaySettings(layout);
    expect(dest.readAsStringSync(), contains('keep-me'));
  });
}
