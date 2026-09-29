import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

void main() {
  late Directory sandbox;

  setUp(() {
    sandbox = Directory.systemTemp.createTempSync('openmuse-rust-projects-');
  });

  tearDown(() {
    if (sandbox.existsSync()) sandbox.deleteSync(recursive: true);
  });

  File writeFile(String relative, [String content = '']) {
    final file = File(p.join(sandbox.path, relative));
    file.parent.createSync(recursive: true);
    file.writeAsStringSync(content);
    return file;
  }

  String path(String relative) => p.normalize(p.join(sandbox.path, relative));

  group('rustLanguageServerRoot', () {
    test('walks up to the crate of a nested source file', () {
      writeFile('crate/Cargo.toml', '[package]\nname = "crate"\n');
      expect(
        rustLanguageServerRoot(path('crate/src/deep/lib.rs')),
        path('crate'),
      );
    });

    test('accepts the lock file as a root marker', () {
      writeFile('locked/Cargo.lock');
      expect(rustLanguageServerRoot(path('locked/src/main.rs')), path('locked'));
    });

    test('reports no root outside every crate', () {
      writeFile('scratch/notes.rs', 'fn main() {}\n');
      expect(rustLanguageServerRoot(path('scratch/notes.rs')), isNull);
    });
  });

  group('discoverRustProjectManifests', () {
    test('finds projects without descending into build output', () {
      writeFile('ws/alpha/Cargo.toml', '[package]\nname = "alpha"\n');
      writeFile('ws/alpha/target/debug/Cargo.toml');
      writeFile('ws/beta/Cargo.toml', '[package]\nname = "beta"\n');
      expect(discoverRustProjectManifests([path('ws')]), [
        path('ws/alpha/Cargo.toml'),
        path('ws/beta/Cargo.toml'),
      ]);
    });

    test('stops at the depth and project limits', () {
      writeFile('ws/top/mid/Cargo.toml');
      writeFile('ws/top/mid/deeper/Cargo.toml');
      expect(discoverRustProjectManifests([path('ws')], maxDepth: 2), [
        path('ws/top/mid/Cargo.toml'),
      ]);
      expect(discoverRustProjectManifests([path('ws')], maxProjects: 1), [
        path('ws/top/mid/Cargo.toml'),
      ]);
    });

    test('ignores missing and irrelevant roots', () {
      writeFile('ws/readme.md');
      expect(discoverRustProjectManifests([path('ws')]), isEmpty);
      expect(discoverRustProjectManifests([path('absent')]), isEmpty);
      expect(discoverRustProjectManifests(const []), isEmpty);
    });
  });

  group('generated rust-analyzer configuration', () {
    test('links the discovered projects', () {
      final toml = const HelixPreferences().languagesTomlFor(
        rustLinkedProjects: [r'D:\workspaces\helix\Cargo.toml'],
      );
      expect(toml, contains('[language-server.rust-analyzer.config]'));
      expect(
        toml,
        contains(r'linkedProjects = ["D:\\workspaces\\helix\\Cargo.toml"]'),
      );
      expect(
        '[language-server.rust-analyzer.config]'.allMatches(toml).length,
        1,
      );
    });

    test('omits the block when nothing was discovered', () {
      expect(
        const HelixPreferences().languagesToml,
        isNot(contains('linkedProjects')),
      );
    });

    test('defers to a user supplied configuration file', () {
      final config = writeFile('rust-analyzer.toml', 'checkOnSave = true\n');
      final toml = HelixPreferences(
        languageServerConfigPaths: {'rust-analyzer': config.path},
      ).languagesTomlFor(
        rustLinkedProjects: [r'D:\workspaces\helix\Cargo.toml'],
      );
      expect(toml, contains('checkOnSave = true'));
      expect(toml, isNot(contains('linkedProjects')));
      expect(
        '[language-server.rust-analyzer.config]'.allMatches(toml).length,
        1,
      );
    });
  });

  group('HelixRuntimePool.linkedRustProjectsFor', () {
    HelixRuntimePool runtimeFor() {
      final runtime = HelixRuntimePool(executable: path('hx.exe'));
      runtime.rustWorkspaceRoots = [path('ws')];
      return runtime;
    }

    test('links mounted projects for a Rust buffer outside any crate', () {
      writeFile('ws/helix/Cargo.toml', '[workspace]\nmembers = []\n');
      writeFile('scratch/notes.rs', 'fn main() {}\n');
      final runtime = runtimeFor();
      addTearDown(runtime.dispose);
      expect(runtime.linkedRustProjectsFor(path('scratch/notes.rs')), [
        path('ws/helix/Cargo.toml'),
      ]);
    });

    test('stays out of the way inside a crate', () {
      writeFile('ws/helix/Cargo.toml', '[package]\nname = "helix"\n');
      writeFile('ws/helix/src/lib.rs', 'pub fn answer() -> u32 { 42 }\n');
      final runtime = runtimeFor();
      addTearDown(runtime.dispose);
      expect(runtime.linkedRustProjectsFor(path('ws/helix/src/lib.rs')), isEmpty);
    });

    test('ignores non-Rust buffers and disabled LSP', () {
      writeFile('ws/helix/Cargo.toml', '[package]\nname = "helix"\n');
      writeFile('scratch/notes.md', '# notes\n');
      final runtime = runtimeFor();
      addTearDown(runtime.dispose);
      expect(runtime.linkedRustProjectsFor(path('scratch/notes.md')), isEmpty);

      final disabled = runtimeFor()
        ..preferences = const HelixPreferences(enableLsp: false);
      addTearDown(disabled.dispose);
      expect(disabled.linkedRustProjectsFor(path('scratch/notes.rs')), isEmpty);
    });

    test('respects a user supplied rust-analyzer configuration', () {
      writeFile('ws/helix/Cargo.toml', '[package]\nname = "helix"\n');
      writeFile('scratch/notes.rs', 'fn main() {}\n');
      final config = writeFile('rust-analyzer.toml', 'checkOnSave = true\n');
      final runtime = runtimeFor()
        ..preferences = HelixPreferences(
          languageServerConfigPaths: {'rust-analyzer': config.path},
        );
      addTearDown(runtime.dispose);
      expect(runtime.linkedRustProjectsFor(path('scratch/notes.rs')), isEmpty);
    });
  });

  test('activation hands the mounted workspaces to the runtime', () async {
    final hx = Platform.environment['OPENMUSE_HELIX_BIN'];
    if (hx == null || !File(hx).existsSync()) {
      markTestSkipped('OPENMUSE_HELIX_BIN is not an hx executable');
      return;
    }
    final runtime = HelixRuntimePool(executable: hx);
    final plugin = OpenMuseHelixPlugin(runtime: runtime);
    addTearDown(plugin.deactivate);
    final commands = <String>[];
    final context = OpenMusePluginContext(
      executeHostCommand: (command, arguments) async {
        commands.add(command);
        return switch (command) {
          'workspace.snapshot' => {
            'mounts': [
              {'path': path('ws'), 'name': 'ws'},
              {'path': 42, 'name': 'invalid'},
            ],
          },
          _ => null,
        };
      },
    );
    await plugin.activate(context);
    expect(commands, contains('workspace.snapshot'));
    expect(runtime.rustWorkspaceRoots, [path('ws')]);
  });
}
