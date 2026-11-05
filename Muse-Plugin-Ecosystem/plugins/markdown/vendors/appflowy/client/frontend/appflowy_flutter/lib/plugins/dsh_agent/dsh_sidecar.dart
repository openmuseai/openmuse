import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_desktop_error.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_web_auth.dart';
import 'package:flutter/foundation.dart';

/// Starts the local DSH web sidecar with the Muse AppFlowy Cordis patch.
class DshSidecar {
  DshSidecar(this.controller);

  final DshAgentController controller;
  Process? _process;
  bool _starting = false;
  bool _stopping = false;
  final List<String> _logTail = <String>[];
  String _logRemainder = '';
  String? _sessionUrl;
  String? _launchToken;

  static DshRuntimeLayout get layout => DshRuntimeLayout.resolve();

  static String get museRoot => layout.museRoot;

  static String get dshHome => layout.dshHome;

  Future<void> ensureStarted() async {
    if (_starting) {
      while (_starting) {
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (await _isReady()) {
        final sessionUrl = _sessionUrl;
        if (sessionUrl != null) controller.setUrl(sessionUrl);
        controller.setError(null);
        controller.setReady(true);
        controller.setLaunching(false);
      }
      return;
    }
    if (await _isReady()) {
      final sessionUrl = _sessionUrl;
      if (sessionUrl != null) controller.setUrl(sessionUrl);
      controller.setError(null);
      controller.setReady(true);
      controller.setLaunching(false);
      return;
    }
    _starting = true;
    controller.setLaunching(true);
    controller.setError(null);
    try {
      final key = await _apiKey();
      if (key == null || key.isEmpty) {
        throw StateError(
          'DEEPSEEK_API_KEY is missing. Enter it in the DeepSeek panel.',
        );
      }
      final secretsToRedact = <String>[
        key,
        ...[
          for (final name in gatewayApiKeyEnvNames)
            await _namedSecret(name) ?? '',
        ].where((value) => value.isNotEmpty),
      ];
      Directory(dshHome).createSync(recursive: true);
      seedGatewaySettings(layout);
      if (layout.closureEntry != null) {
        seedClosurePlugins(layout);
      } else {
        _seedMuseModules(layout);
        _seedDshMarket(layout);
      }
      clearLeakedHostPackages(layout);
      var attempt = 0;
      while (true) {
        attempt++;
        if (attempt > 1) {
          // The previous boot died on a package Node could not resolve, or on a
          // writer lock whose owner was gone: an entry that cannot resolve
          // shadows the installation fallback for the whole profile. Rebuild the
          // scopes and start again, so a leak that appeared after the pre-start
          // repair costs one restart instead of the session.
          clearLeakedHostPackages(layout);
        }
        _logTail.clear();
        _logRemainder = '';
        _sessionUrl = null;
        _launchToken = null;
        await _reclaimListenPort();
        // Reclaiming the port force-kills a sidecar that still holds it, and a
        // killed process cannot run the release path of its writer lock, so sweep
        // orphans after the kill rather than before it.
        clearOrphanedWriterLocks(layout);
        _stopping = false;
        _process = await _spawn(key);
        var exited = false;
        unawaited(
          _process!.exitCode.then((code) {
            exited = true;
            _launchToken = null;
            _sessionUrl = null;
            if (_stopping || !controller.ready) return;
            controller.setReady(false);
            controller.setError(
              DshWebAuth.summarizeExit(code, _logTail),
              code: DshDesktopError.sidecarExit,
            );
          }),
        );
        unawaited(
          _process!.stdout.transform(utf8.decoder).forEach((chunk) {
            _rememberLog(chunk, secretsToRedact);
            debugPrint('[dsh-sidecar] $chunk');
          }),
        );
        unawaited(
          _process!.stderr.transform(utf8.decoder).forEach((chunk) {
            _rememberLog(chunk, secretsToRedact);
            debugPrint('[dsh-sidecar:err] $chunk');
          }),
        );
        final deadline = DateTime.now().add(const Duration(seconds: 90));
        var retry = false;
        while (DateTime.now().isBefore(deadline)) {
          if (await _isReady() && await _stayedReady()) {
            final sessionUrl = _sessionUrl;
            if (sessionUrl != null) controller.setUrl(sessionUrl);
            controller.setError(null);
            controller.setReady(true);
            return;
          }
          if (exited) {
            await Future<void>.delayed(const Duration(milliseconds: 200));
            final code = await _process!.exitCode;
            if (attempt == 1 &&
                (_isModuleResolutionFailure() || _isWriterLockTimeout())) {
              retry = true;
              break;
            }
            throw StateError(DshWebAuth.summarizeExit(code, _logTail));
          }
          await Future<void>.delayed(const Duration(milliseconds: 500));
        }
        if (retry) continue;
        throw StateError('DSH sidecar did not become ready on ${controller.url}');
      }
    } catch (error) {
      controller.setReady(false);
      final message = error.toString();
      controller.setError(
        message,
        code: DshDesktopError.codeFromMessage(message),
      );
      rethrow;
    } finally {
      _starting = false;
      controller.setLaunching(false);
    }
  }

  Future<Process> _spawn(String apiKey) async {
    final resolved = layout;
    final environment = <String, String>{
      ...Platform.environment,
      'DEEPSEEK_API_KEY': apiKey,
      'MUSE_ROOT': resolved.museRoot,
      'DSH_HOME': resolved.dshHome,
      'DSH_WEB_HOST': DshWebAuth.listenHost,
      'DSH_WEB_PORT': '${DshWebAuth.listenPort}',
      // ESM package imports resolve from the file's realpath. Muse plugins
      // must therefore live under the harness node_modules tree, not the
      // sibling Resources/muse/packages copies.
      'NODE_PATH': '${resolved.harnessDir}/node_modules',
    };
    for (final name in gatewayApiKeyEnvNames) {
      final value = await _namedSecret(name);
      if (value != null && value.isNotEmpty) {
        environment[name] = value;
      }
    }
    environment['MUSE_PLUGIN_DIAGNOSTICS'] ??= '1';
    if (resolved.bundled) {
      environment['MUSE_BUNDLE_ROOT'] = resolved.museRoot;
      final node = resolved.nodeBin;
      if (node == null) {
        throw StateError('Packed Muse runtime is missing node/bin/node');
      }
      // Finder-launched .app PATH is /usr/bin:/bin. dshmarket shells out to
      // node/corepack/pnpm when installing community plugins.
      final nodeDir = File(node).parent.path;
      final shims = ensureDesktopBinShims(resolved);
      environment['PATH'] = _prependPath(environment['PATH'], shims);
      environment['PATH'] = _prependPath(environment['PATH'], nodeDir);
      // DSH Desktop measured these: forcing copy/clone made Windows installs
      // physically copy the profile's 150+ packages; hardlink is what makes
      // one-click installs seconds instead of minutes.
      environment['npm_config_side_effects_cache'] = 'false';
      environment['PNPM_CONFIG_SIDE_EFFECTS_CACHE'] = 'false';
      final closureEntry = resolved.closureEntry;
      if (closureEntry != null) {
        // Symlink-free closure layout: run the packaged lib/bin.js directly
        // (no tsx, no source tree). Profile plugins were seeded in
        // ensureStarted so @muse + dshmarket resolve on first boot.
        return _startClosureDshCli(node, closureEntry, resolved, environment);
      }
      return _startDshCli(node, resolved, environment);
    }
    return _spawnFromSource(resolved, environment);
  }

  /// Closure packages owned by the DSH install. Copying a subset into the
  /// profile creates `node_modules/@deepseek-ai` with only Muse peers; Node
  /// then fails ESM lookup for the rest of the web bundle
  /// (`ERR_MODULE_NOT_FOUND` / "Did you mean …/lib/index.js"). Those packages
  /// must come from `$DSH_HOME/profiles/node_modules` junctions that
  /// `healProfilesModuleFallback` writes.
  static bool isHostOwnedSeedPackage(String spec) {
    return spec.startsWith('@deepseek-ai/');
  }

  /// Identity of the bundled closure this profile seed was copied from.
  /// Changes when the install path or `@muse/dsh-appflowy` overlay changes, so
  /// an upgrade from portable → setup.exe re-copies Muse plugins.
  static String closureSeedGeneration(DshRuntimeLayout resolved) {
    final dsh = File(
      '${resolved.museRoot}/closure/node_modules/@deepseek-ai/dsh/package.json',
    );
    var version = '';
    if (dsh.existsSync()) {
      try {
        final data = jsonDecode(dsh.readAsStringSync());
        if (data is Map && data['version'] is String) {
          version = data['version'] as String;
        }
      } catch (_) {}
    }
    final connector = File(
      '${resolved.museRoot}/closure/node_modules/@muse/dsh-appflowy/dist/src/connector.js',
    );
    final stamp = connector.existsSync()
        ? connector.lastModifiedSync().millisecondsSinceEpoch
        : 0;
    return '3\t${resolved.museRoot}\t$version\t$stamp';
  }

  /// Repair every module-fallback scope dsh resolves `@deepseek-ai/*` through.
  ///
  /// Node resolves a bare `@deepseek-ai/*` specifier from the profile directory
  /// first and from `$DSH_HOME/profiles/node_modules` second — but it stops at
  /// the first `node_modules` directory that holds a directory with the
  /// package's name. A leftover there wins over a perfectly good installation
  /// fallback, so one entry that is a plain directory, a dangling link, a link
  /// into another installation, or a copy whose files were only half written
  /// makes that package unresolvable for the whole profile. The Loader then
  /// fails for every affected entry with `ERR_MODULE_NOT_FOUND` and
  /// "Did you mean …/lib/index.js?", the sidecar exits 1, and the UI only shows
  /// `Bad state: DSH sidecar exited with 1` plus a Node stack.
  ///
  /// dsh repairs only what is missing: `ensureProfileSymlink` returns as soon as
  /// the path exists, whatever it holds, and `moduleFallbackCurrent` trusts a
  /// link that points at the right directory. Deleting a bad entry is therefore
  /// what makes `healProfilesModuleFallback` write a healthy one again, which is
  /// why this runs before every start rather than only when re-seeding.
  static void clearLeakedHostPackages(DshRuntimeLayout resolved) {
    final installation = _normalizePath(resolved.museRoot);
    final owned = _normalizePath(
      '${resolved.dshHome}/profiles/web/.dsh-module-fallback',
    );
    for (final path in [
      '${resolved.dshHome}/profiles/node_modules/@deepseek-ai',
      '${resolved.dshHome}/profiles/web/node_modules/@deepseek-ai',
      '${resolved.dshHome}/profiles/web/.dsh-module-fallback/node_modules/@deepseek-ai',
    ]) {
      _pruneModuleScope(Directory(path), installation, owned);
    }
  }

  /// Delete writer locks whose owner is gone.
  ///
  /// `withFileLock` serializes writers by creating `<file>.lock` exclusively and
  /// writing its own pid into it. A contender never removes an existing lock
  /// ("file age cannot prove that its owner stopped; orphan recovery is an
  /// operator action") and gives up after two seconds. So one sidecar that dies
  /// mid-write - a crash, a `taskkill`, a power loss - leaves behind a lock that
  /// fails every later boot with `atomic-write: timed out waiting for the writer
  /// lock`, which the panel reports as SIDECAR_EXIT. That bricked an install on
  /// 2026-09-21: the lock held pid 24376, no such process existed, and
  /// `healProfilesModuleFallback` needs that lock on almost every start because
  /// [clearLeakedHostPackages] keeps handing it work to do.
  ///
  /// Only a lock that provably has no writer is removed: its pid is gone, the
  /// pid now names something that is not node, or the process started after the
  /// lock was written (Windows reuses pids). A lock that may still have a live
  /// writer stays, since two writers could then corrupt the file it protects.
  static List<String> clearOrphanedWriterLocks(DshRuntimeLayout resolved) =>
      clearOrphanedWriterLocksIn(resolved.dshHome);

  @visibleForTesting
  static List<String> clearOrphanedWriterLocksIn(String dshHome) {
    final home = Directory(dshHome);
    if (!home.existsSync()) return const <String>[];
    final removed = <String>[];
    for (final lock in _writerLocks(home)) {
      if (!_lockOwnerIsGone(lock)) continue;
      try {
        lock.deleteSync();
        removed.add(lock.path);
      } catch (_) {}
    }
    if (removed.isNotEmpty) {
      debugPrint('[dsh-sidecar] cleared orphaned writer locks: $removed');
    }
    return removed;
  }

  /// Every `*.lock` a dsh writer may have created under the DSH home. Package
  /// trees are not descended into: they hold thousands of entries, and these
  /// locks sit beside such a tree rather than inside it.
  static Iterable<File> _writerLocks(Directory dir, [int depth = 0]) sync* {
    if (depth > 4) return;
    List<FileSystemEntity> children;
    try {
      children = dir.listSync(followLinks: false);
    } catch (_) {
      return;
    }
    for (final child in children) {
      final name = child.uri.pathSegments
          .where((segment) => segment.isNotEmpty)
          .last;
      if (child is Directory) {
        if (name == 'node_modules' || name == '.git') continue;
        if (name.startsWith('.pnpm')) continue;
        yield* _writerLocks(child, depth + 1);
      } else if (child is File && name.endsWith('.lock')) {
        yield child;
      }
    }
  }

  /// Whether the pid recorded in [lock] cannot be the process holding it.
  static bool _lockOwnerIsGone(File lock) {
    final pid = _lockOwnerPid(lock);
    if (pid == null) return false;
    final owner = _processInfo(pid);
    if (owner == null) return true;
    final image = '${owner['image']}';
    if (!image.contains('node') && !image.contains('dsh')) return true;
    final started = owner['started'];
    if (started is! DateTime) return false;
    DateTime written;
    try {
      written = lock.lastModifiedSync();
    } catch (_) {
      return false;
    }
    // A process cannot have written a lock that predates it.
    return started.isAfter(written.add(const Duration(seconds: 2)));
  }

  /// The pid a lock file records, or null when it holds anything else.
  static int? _lockOwnerPid(File lock) {
    String text;
    try {
      if (lock.lengthSync() > 64) return null;
      text = lock.readAsStringSync().trim();
    } catch (_) {
      return null;
    }
    final pid = int.tryParse(text);
    if (pid == null || pid <= 0) return null;
    return pid;
  }

  /// Image name and start time of [pid], or null when no such process exists.
  static Map<String, Object?>? _processInfo(int pid) {
    try {
      if (Platform.isWindows) {
        final listed = Process.runSync('tasklist', [
          '/FI',
          'PID eq $pid',
          '/FO',
          'CSV',
          '/NH',
        ]);
        final out = '${listed.stdout}'.trim();
        if (listed.exitCode != 0 || out.isEmpty || out.startsWith('INFO:')) {
          return null;
        }
        final image = RegExp('^"([^"]+)"').firstMatch(out)?.group(1) ?? '';
        return <String, Object?>{
          'image': image.toLowerCase(),
          'started': _windowsStartTime(pid),
        };
      }
      final listed = Process.runSync('ps', ['-p', '$pid', '-o', 'comm=']);
      final image = '${listed.stdout}'.trim().toLowerCase();
      if (listed.exitCode != 0 || image.isEmpty) return null;
      return <String, Object?>{'image': image, 'started': null};
    } catch (_) {
      return null;
    }
  }

  /// Start time of a Windows process, or null when it cannot be read.
  static DateTime? _windowsStartTime(int pid) {
    try {
      final started = Process.runSync('powershell', [
        '-NoProfile',
        '-NonInteractive',
        '-Command',
        '(Get-Process -Id $pid -ErrorAction Stop).StartTime.ToString("o")',
      ]);
      if (started.exitCode != 0) return null;
      return DateTime.tryParse('${started.stdout}'.trim());
    } catch (_) {
      return null;
    }
  }

  /// Drop every entry of one `@deepseek-ai` scope that cannot resolve.
  static void _pruneModuleScope(
    Directory scope,
    String installation,
    String owned,
  ) {
    if (!scope.existsSync()) return;
    for (final entity in scope.listSync(followLinks: false)) {
      if (_usableModuleEntry(entity, installation, owned)) continue;
      try {
        if (FileSystemEntity.typeSync(entity.path, followLinks: false) ==
            FileSystemEntityType.link) {
          Link(entity.path).deleteSync();
        } else {
          entity.deleteSync(recursive: true);
        }
      } catch (_) {}
    }
  }

  /// Whether [entity] is a package Node can actually resolve: a manifest that
  /// parses, the entry that manifest declares, and a target belonging to this
  /// installation (or one of dsh's own packaged proxies).
  static bool _usableModuleEntry(
    FileSystemEntity entity,
    String installation,
    String owned,
  ) {
    String resolved;
    try {
      // Throws for a dangling link, which shadows the installation fallback
      // just as effectively as a bad copy does.
      resolved = entity.resolveSymbolicLinksSync();
    } catch (_) {
      return false;
    }
    if (FileSystemEntity.typeSync(resolved) != FileSystemEntityType.directory) {
      return false;
    }
    final manifest = File('$resolved/package.json');
    if (!manifest.existsSync()) return false;
    Map<dynamic, dynamic> data;
    try {
      final parsed = jsonDecode(manifest.readAsStringSync());
      if (parsed is! Map) return false;
      data = parsed;
    } catch (_) {
      return false;
    }
    final dsh = data['dsh'];
    if (dsh is Map && dsh['moduleFallback'] is Map) {
      // dsh's packaged-executable proxy: dsh resolves it through its own
      // targets and rewrites it whenever its records are stale.
      return true;
    }
    final home = _normalizePath(resolved);
    if (!_isInside(home, installation) && !_isInside(home, owned)) {
      // Another installation (possibly a removed one) owns this entry, so this
      // installation cannot vouch for it.
      return false;
    }
    if (data['exports'] != null) return true;
    final main = data['main'];
    if (main is String && main.trim().isNotEmpty) {
      return _declaredEntryExists(resolved, main);
    }
    return File('$resolved/index.js').existsSync();
  }

  /// Whether the `main` a manifest declares is present (a file, or a directory
  /// with the usual index files).
  static bool _declaredEntryExists(String packagePath, String main) {
    var relative = main.replaceAll('\\', '/');
    if (relative.startsWith('./')) relative = relative.substring(2);
    if (relative.isEmpty || relative.contains('..')) return false;
    final target = '$packagePath/$relative';
    if (File(target).existsSync()) return true;
    if (!Directory(target).existsSync()) return false;
    for (final candidate in const ['index.js', 'index.mjs', 'index.cjs']) {
      if (File('$target/$candidate').existsSync()) return true;
    }
    return false;
  }

  /// Canonical form for the path comparisons above: Windows is case
  /// insensitive and Dart returns `\\?\`-prefixed paths for some volumes.
  static String _normalizePath(String path) {
    var value = path.replaceAll('\\', '/');
    if (value.startsWith('//?/')) value = value.substring(4);
    while (value.length > 1 && value.endsWith('/')) {
      value = value.substring(0, value.length - 1);
    }
    return value.toLowerCase();
  }

  static bool _isInside(String path, String root) =>
      path == root || path.startsWith('$root/');

  /// Whether the captured sidecar output shows a package that could not be
  /// resolved, which is the one failure a fallback rebuild can fix.
  bool _isModuleResolutionFailure() {
    final tail = _logTail.join('\n');
    return tail.contains('ERR_MODULE_NOT_FOUND') ||
        tail.contains('Did you mean to import') ||
        tail.contains('ERR_INVALID_PACKAGE_CONFIG') ||
        tail.contains('No "exports" main defined');
  }

  /// Whether the sidecar died waiting for a writer lock, which clearing an
  /// orphaned lock can fix.
  bool _isWriterLockTimeout() => _logTail.any(
    (line) => line.contains('timed out waiting for the writer lock'),
  );

  /// Closure (npm) layout: seed real copies of the @muse + dshmarket plugin
  /// graph into `$DSH_HOME/profiles/web/node_modules`. The Loader anchors
  /// bare-name resolution at the profile dir, and on Windows Node ESM refuses
  /// symlinked node_modules entries, so plugins must be real directories.
  /// Host `@deepseek-ai/*` packages stay out of the profile: the install
  /// fallback supplies them. Re-seed when [closureSeedGeneration] changes.
  static void seedClosurePlugins(DshRuntimeLayout resolved) {
    final marker = File('${resolved.dshHome}/profiles/web/.muse-seeded');
    final generation = closureSeedGeneration(resolved);
    if (marker.existsSync() && marker.readAsStringSync() == generation) {
      return;
    }
    final closureNm = Directory('${resolved.museRoot}/closure/node_modules');
    if (!closureNm.existsSync()) return;
    final destRoots = [
      Directory('${resolved.dshHome}/profiles/web/node_modules'),
    ];
    for (final destRoot in destRoots) {
      final leakedHost = Directory('${destRoot.path}/@deepseek-ai');
      if (leakedHost.existsSync()) {
        leakedHost.deleteSync(recursive: true);
      }
    }
    // Bare names patch.yml mounts that are not part of the @muse scope. The
    // Loader resolves `name: dsh-model-capabilities` from the profile
    // node_modules, so this vendored plugin has to be seeded like dshmarket.
    final roots = <String>['dshmarket', 'dsh-model-capabilities'];
    final museScope = Directory('${closureNm.path}/@muse');
    if (museScope.existsSync()) {
      for (final entity in museScope.listSync()) {
        if (entity is! Directory) continue;
        final name = entity.uri.pathSegments
            .where((segment) => segment.isNotEmpty)
            .last;
        roots.add('@muse/$name');
      }
    }
    final seen = <String>{};
    final queue = List<String>.from(roots);
    while (queue.isNotEmpty) {
      final spec = queue.removeLast();
      if (!seen.add(spec)) continue;
      if (isHostOwnedSeedPackage(spec)) continue;
      final source = Directory('${closureNm.path}/$spec');
      final manifest = File('${source.path}/package.json');
      if (!manifest.existsSync()) continue;
      for (final destRoot in destRoots) {
        _copyPackageTree(source, Directory('${destRoot.path}/$spec'));
      }
      try {
        final data =
            jsonDecode(manifest.readAsStringSync()) as Map<String, dynamic>;
        for (final kind in [
          'dependencies',
          'peerDependencies',
          'optionalDependencies',
        ]) {
          final deps = data[kind];
          if (deps is Map) {
            for (final name in deps.keys) {
              final specName = name as String;
              if (isHostOwnedSeedPackage(specName)) continue;
              if (File('${closureNm.path}/$specName/package.json')
                  .existsSync()) {
                queue.add(specName);
              }
            }
          }
        }
      } catch (_) {
        // A manifest that fails to parse cannot contribute further roots.
      }
    }
    try {
      marker.parent.createSync(recursive: true);
      marker.writeAsStringSync(generation);
    } catch (_) {}
  }

  /// Real-directory copy. Dart 3.6 (Flutter 3.27) has no Directory.copySync,
  /// and Windows Node ESM rejects symlinked node_modules entries.
  static void _copyPackageTree(Directory source, Directory dest) {
    final existing = FileSystemEntity.typeSync(dest.path, followLinks: false);
    if (existing == FileSystemEntityType.link) {
      Link(dest.path).deleteSync();
    } else if (existing == FileSystemEntityType.directory) {
      dest.deleteSync(recursive: true);
    } else if (existing == FileSystemEntityType.file) {
      File(dest.path).deleteSync();
    }
    dest.createSync(recursive: true);
    for (final entity in source.listSync(followLinks: false)) {
      final name = entity.uri.pathSegments
          .where((segment) => segment.isNotEmpty)
          .last;
      final next = '${dest.path}/$name';
      if (entity is Directory) {
        _copyPackageTree(entity, Directory(next));
      } else if (entity is File) {
        entity.copySync(next);
      }
    }
  }

  Future<Process> _startClosureDshCli(
    String node,
    String closureEntry,
    DshRuntimeLayout resolved,
    Map<String, String> environment,
  ) {
    return Process.start(
      node,
      [
        closureEntry,
        'web',
        '--patch',
        resolved.patchFile,
        '--host',
        DshWebAuth.listenHost,
        '--port',
        '${DshWebAuth.listenPort}',
      ],
      workingDirectory: resolved.museRoot,
      environment: environment,
    );
  }

  /// DSH Desktop's `.desktop-bin` pattern: write `pnpm`/`node` shims that
  /// resolve every `pnpm` by-name invocation (`dsh plugin`, dshmarket) to the
  /// bundled node + pnpm runner, so one-click plugin installs need no
  /// user-side Node/pnpm. Returns the shim directory (empty when the bundled
  /// plugin-tools are missing, which only happens on hand-modified bundles).
  static String ensureDesktopBinShims(DshRuntimeLayout resolved) {
    final node = resolved.nodeBin;
    final tools = Directory('${resolved.museRoot}/plugin-tools');
    final runner = File('${tools.path}/pnpm-runner.mjs').absolute;
    final pnpmEntry = File('${tools.path}/pnpm/bin/pnpm.cjs').absolute;
    final dir = Directory('${resolved.dshHome}/.desktop-bin').absolute;
    dir.createSync(recursive: true);
    if (node == null || !runner.existsSync() || !pnpmEntry.existsSync()) {
      return dir.path;
    }
    // Absolute paths only: the shims run from arbitrary cwds (the profile
    // directory) where relative bundle paths would not resolve.
    final command = '"${runner.path}" "${pnpmEntry.path}"';
    final nodeQuoted = '"${File(node).absolute.path}"';
    if (Platform.isWindows) {
      File('${dir.path}/pnpm.cmd').writeAsStringSync(
        '@chcp 65001 >nul\r\n@echo off\r\n$nodeQuoted $command %*\r\n',
      );
      File('${dir.path}/node.cmd').writeAsStringSync(
        '@chcp 65001 >nul\r\n@echo off\r\n$nodeQuoted %*\r\n',
      );
    } else {
      File('${dir.path}/pnpm').writeAsStringSync(
        '#!/bin/sh\nexec $nodeQuoted $command "\$@"\n',
      );
      File('${dir.path}/node').writeAsStringSync(
        '#!/bin/sh\nexec $nodeQuoted "\$@"\n',
      );
    }
    return dir.path;
  }

  Future<Process> _spawnFromSource(
    DshRuntimeLayout resolved,
    Map<String, String> environment,
  ) async {
    final harness = Directory(resolved.harnessDir);
    if (!harness.existsSync()) {
      throw StateError(
        'Missing DSH harness at ${resolved.harnessDir}. '
        'Place vendors/deepseek-harness in the Muse checkout, or set MUSE_HARNESS_DIR.',
      );
    }
    if (!File(resolved.patchFile).existsSync()) {
      throw StateError('Missing DSH patch ${resolved.patchFile}');
    }
    if (!Directory('${resolved.harnessDir}/node_modules').existsSync()) {
      throw StateError(
        'DSH harness dependencies are missing at ${resolved.harnessDir}/node_modules. '
        'Run: cd vendors/deepseek-harness && pnpm install',
      );
    }

    if (!Platform.isWindows) {
      final script = File(resolved.runDshScript);
      if (script.existsSync()) {
        return Process.start(
          '/bin/bash',
          [script.path],
          workingDirectory: resolved.museRoot,
          environment: environment,
        );
      }
    }

    final nodeDir = _windowsNodeDir();
    if (nodeDir != null) {
      environment['PATH'] = _prependPath(environment['PATH'], nodeDir);
    }
    final pnpm = _findOnPath('pnpm');
    if (pnpm != null) {
      return Process.start(
        pnpm,
        [
          'dsh',
          '--profile',
          'web',
          '--patch',
          resolved.patchFile,
          '--host',
          DshWebAuth.listenHost,
          '--port',
          '${DshWebAuth.listenPort}',
        ],
        workingDirectory: resolved.harnessDir,
        environment: environment,
      );
    }
    final node = _findOnPath('node');
    if (node != null) {
      return _startDshCli(node, resolved, environment);
    }
    throw StateError(
      'node and pnpm are required to start the DSH sidecar. '
      'Install Node.js, or run middlewares/scripts/run-dsh-appflowy.sh from a shell.',
    );
  }

  Future<Process> _startDshCli(
    String node,
    DshRuntimeLayout resolved,
    Map<String, String> environment,
  ) {
    return Process.start(
      node,
      [
        '--import',
        'tsx/esm',
        'apps/cli/src/bin.ts',
        '--profile',
        'web',
        '--patch',
        resolved.patchFile,
        '--host',
        DshWebAuth.listenHost,
        '--port',
        '${DshWebAuth.listenPort}',
      ],
      workingDirectory: resolved.harnessDir,
      environment: environment,
    );
  }

  static String _prependPath(String? inherited, String prefix) {
    if (inherited == null || inherited.isEmpty) return prefix;
    final sep = Platform.isWindows ? ';' : ':';
    return '$prefix$sep$inherited';
  }

  static String? _windowsNodeDir() {
    if (!Platform.isWindows) return null;
    const candidate = r'C:\Program Files\nodejs';
    if (Directory(candidate).existsSync()) return candidate;
    return null;
  }

  static String? _findOnPath(String name) {
    final pathEnv = Platform.environment['PATH'] ?? '';
    final sep = Platform.isWindows ? ';' : ':';
    final exts = Platform.isWindows
        ? (Platform.environment['PATHEXT'] ?? '.EXE;.CMD;.BAT;.COM')
            .split(';')
            .where((ext) => ext.isNotEmpty)
            .toList()
        : const <String>[''];
    final names = Platform.isWindows
        ? <String>[
            for (final ext in exts) '$name$ext',
            for (final ext in exts) '$name${ext.toLowerCase()}',
            name,
          ]
        : <String>[name];
    final dirs = <String>[
      ...pathEnv.split(sep).where((dir) => dir.isNotEmpty),
      if (Platform.isWindows) r'C:\Program Files\nodejs',
    ];
    for (final dir in dirs) {
      for (final fileName in names) {
        final candidate = File('$dir${Platform.pathSeparator}$fileName');
        if (candidate.existsSync()) return candidate.path;
      }
    }
    return null;
  }

  void _seedMuseModules(DshRuntimeLayout resolved) {
    Directory? source;
    final harness = Directory('${resolved.harnessDir}/node_modules/@muse');
    final bundled = Directory('${resolved.museRoot}/packages');
    // Prefer the copy under dsh/node_modules/@muse so Node ESM can resolve
    // sibling @muse/* and @deepseek-ai/* from that tree. Linking at
    // Resources/muse/packages makes imports like @muse/plugin-kit fail, DSH
    // fail-loud exits, and the WebView is left on "Loading plugins…".
    if (harness.existsSync()) {
      source = harness;
    } else if (bundled.existsSync()) {
      source = bundled;
    }
    if (source == null) return;
    for (final destPath in [
      '${resolved.dshHome}/profiles/node_modules/@muse',
      '${resolved.dshHome}/profiles/web/node_modules/@muse',
    ]) {
      Directory(destPath).createSync(recursive: true);
      for (final entity in source.listSync()) {
        if (entity is! Directory) continue;
        final name =
            entity.uri.pathSegments.where((segment) => segment.isNotEmpty).last;
        final target = '$destPath/$name';
        _replaceWithLink(target, entity.path);
      }
    }
  }

  /// Loader baseUrl is the profile directory; ESM does not consult NODE_PATH.
  void _seedDshMarket(DshRuntimeLayout resolved) {
    for (final pkg in ['dshmarket', 'dsh-model-capabilities']) {
      final source = Directory('${resolved.harnessDir}/node_modules/$pkg');
      if (!source.existsSync()) continue;
      for (final destDir in [
        '${resolved.dshHome}/profiles/node_modules',
        '${resolved.dshHome}/profiles/web/node_modules',
      ]) {
        Directory(destDir).createSync(recursive: true);
        _replaceWithLink('$destDir/$pkg', source.path);
      }
    }
  }

  void _replaceWithLink(String target, String sourcePath) {
    try {
      // Always replace: a leftover directory from an older pack would
      // otherwise shadow the bundled package (WKWebView URL patch / dshmarket).
      final existing = FileSystemEntity.typeSync(target, followLinks: false);
      if (existing == FileSystemEntityType.link) {
        Link(target).deleteSync();
      } else if (existing == FileSystemEntityType.directory) {
        Directory(target).deleteSync(recursive: true);
      } else if (existing == FileSystemEntityType.file) {
        File(target).deleteSync();
      }
      Link(target).createSync(sourcePath);
    } catch (_) {}
  }

  void _rememberLog(String chunk, List<String> secrets) {
    final data = '$_logRemainder$chunk';
    final lines = const LineSplitter().convert(data);
    final complete = data.endsWith('\n') || data.endsWith('\r')
        ? lines
        : lines.take(lines.isEmpty ? 0 : lines.length - 1);
    _logRemainder = data.endsWith('\n') || data.endsWith('\r')
        ? ''
        : (lines.isEmpty ? data : lines.last);
    for (final rawLine in complete) {
      _captureLaunchUrl(rawLine);
      var line = rawLine;
      for (final secret in secrets) {
        if (secret.isNotEmpty) line = line.replaceAll(secret, '[REDACTED]');
      }
      line = line.trim();
      if (line.isEmpty) continue;
      _logTail.add(line);
      if (_logTail.length > 40) _logTail.removeAt(0);
      _appendLogFile(line);
    }
  }

  void _captureLaunchUrl(String line) {
    final url = DshWebAuth.extractLaunchUrl(line);
    if (url == null) return;
    _sessionUrl = url;
    _launchToken = DshWebAuth.extractLaunchToken(line);
  }

  void _appendLogFile(String line) {
    try {
      File('${DshRuntimeLayout.userMuseHome}/dsh-sidecar.log')
          .writeAsStringSync('$line\n', mode: FileMode.append);
    } catch (_) {}
  }

  /// First HTTP 200 is not "booted": Muse plugins still import after listen.
  /// A failed import fail-loud-exits a second later and leaves Loading plugins.
  Future<bool> _stayedReady() async {
    for (var i = 0; i < 8; i++) {
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (!await _isReady()) return false;
    }
    return true;
  }

  Future<bool> _isReady() async {
    try {
      final uri = DshWebAuth.probeUri(_sessionUrl);
      final client = HttpClient()
        ..connectionTimeout = const Duration(seconds: 2);
      final request = await client.getUrl(uri);
      final response = await request.close();
      await response.drain<void>();
      client.close(force: true);
      return DshWebAuth.isStartupProbeHealthy(
        response.statusCode,
        _launchToken,
      );
    } catch (_) {
      return false;
    }
  }

  /// Extra `apiKeyEnv` names from the gateway default `settings.yaml`.
  /// Finder-launched apps have no shell export; these must come from
  /// [credentialsFile] (or `.env.dsh.local` in a source checkout).
  static const gatewayApiKeyEnvNames = <String>[
    'OPENCODE_CUSTOM_API_KEY',
    'OPENCODE_GO_API_KEY',
  ];

  Future<String?> _apiKey() => _namedSecret('DEEPSEEK_API_KEY');

  Future<String?> _namedSecret(String name) async {
    final fromEnv = Platform.environment[name];
    if (fromEnv != null && fromEnv.trim().isNotEmpty) return fromEnv.trim();
    for (final path in [
      layout.credentialsFile,
      '${Directory.systemTemp.path}/appflowy-muse/credentials.env',
    ]) {
      final stored = File(path);
      if (!stored.existsSync()) continue;
      for (final raw in stored.readAsLinesSync()) {
        final parsed = parseEnvLine(raw, name);
        if (parsed != null) return parsed;
      }
    }
    final sourceEnv = File('${layout.museRoot}/.env.dsh.local');
    if (!layout.bundled && sourceEnv.existsSync()) {
      for (final raw in sourceEnv.readAsLinesSync()) {
        final parsed = parseEnvLine(raw, name);
        if (parsed != null) return parsed;
      }
    }
    return null;
  }

  static String? parseEnvLine(String raw, String name) {
    final line = raw.trim();
    if (line.isEmpty || line.startsWith('#')) return null;
    final index = line.indexOf('=');
    if (index <= 0) return null;
    if (line.substring(0, index).trim() != name) return null;
    var value = line.substring(index + 1).trim();
    if (value.length >= 2 &&
        ((value.startsWith('"') && value.endsWith('"')) ||
            (value.startsWith("'") && value.endsWith("'")))) {
      value = value.substring(1, value.length - 1);
    }
    return value.isEmpty ? null : value;
  }

  /// Insert or replace one `NAME=value` line without dropping other secrets.
  static String upsertEnvFile(String existing, String name, String value) {
    final lines = <String>[];
    var replaced = false;
    for (final raw in const LineSplitter().convert(existing)) {
      final line = raw.trimRight();
      if (line.isEmpty) continue;
      final index = line.indexOf('=');
      final key = index <= 0 ? '' : line.substring(0, index).trim();
      if (key == name) {
        lines.add('$name=$value');
        replaced = true;
      } else {
        lines.add(line);
      }
    }
    if (!replaced) lines.add('$name=$value');
    return '${lines.join('\n')}\n';
  }

  /// Copy gateway defaults into `$DSH_HOME/settings.yaml` once (never overwrite).
  static void seedGatewaySettings(DshRuntimeLayout resolved) {
    final dest = File('${resolved.dshHome}/settings.yaml');
    if (dest.existsSync()) return;
    for (final candidate in [
      '${resolved.museRoot}/defaults/settings.yaml',
      '${resolved.museRoot}/middlewares/dsh/deploy/defaults/settings.yaml',
    ]) {
      final src = File(candidate);
      if (!src.existsSync()) continue;
      dest.parent.createSync(recursive: true);
      src.copySync(dest.path);
      return;
    }
  }

  Future<void> saveApiKey(String key) async {
    final trimmed = key.trim();
    if (trimmed.isEmpty) {
      throw StateError('API key is empty');
    }
    final file = File(layout.credentialsFile);
    await file.parent.create(recursive: true);
    final existing = file.existsSync() ? file.readAsStringSync() : '';
    await file.writeAsString(
      upsertEnvFile(existing, 'DEEPSEEK_API_KEY', trimmed),
    );
    if (Platform.isMacOS || Platform.isLinux) {
      await Process.run('chmod', ['600', file.path]);
    }
  }

  bool get needsApiKey {
    final error = controller.lastError;
    return error != null && error.contains('DEEPSEEK_API_KEY');
  }

  Future<void> stop() async {
    _stopping = true;
    final process = _process;
    _process = null;
    _launchToken = null;
    _sessionUrl = null;
    if (process == null) return;
    await _killPidTree(process.pid);
    try {
      process.kill();
    } catch (_) {}
    try {
      await process.exitCode.timeout(const Duration(seconds: 4));
    } catch (_) {}
  }

  /// Windows does not kill Node children when the GUI exits, so a leftover
  /// `dsh web` from the previous launch still owns 3080. Reclaim it before
  /// spawn or Host fail-louds with EADDRINUSE.
  Future<void> _reclaimListenPort() async {
    await stop();
    final occupied = await _listeningPids(DshWebAuth.listenPort);
    final self = pid;
    var killed = false;
    for (final occupiedPid in occupied) {
      if (occupiedPid == self) continue;
      await _killPidTree(occupiedPid);
      killed = true;
    }
    if (killed) {
      await Future<void>.delayed(const Duration(milliseconds: 400));
    }
  }

  Future<Set<int>> _listeningPids(int port) async {
    try {
      if (Platform.isWindows) {
        final result = await Process.run('netstat', ['-ano', '-p', 'tcp']);
        return DshWebAuth.listeningPidsFromNetstat(
          '${result.stdout}',
          port: port,
        );
      }
      final result = await Process.run('lsof', [
        '-t',
        '-iTCP:$port',
        '-sTCP:LISTEN',
      ]);
      return '${result.stdout}'
          .split(RegExp(r'\s+'))
          .map(int.tryParse)
          .whereType<int>()
          .where((value) => value > 0)
          .toSet();
    } catch (_) {
      return {};
    }
  }

  Future<void> _killPidTree(int target) async {
    if (target <= 0) return;
    try {
      if (Platform.isWindows) {
        await Process.run('taskkill', ['/F', '/T', '/PID', '$target']);
        return;
      }
      Process.killPid(target, ProcessSignal.sigterm);
    } catch (_) {}
  }
}
