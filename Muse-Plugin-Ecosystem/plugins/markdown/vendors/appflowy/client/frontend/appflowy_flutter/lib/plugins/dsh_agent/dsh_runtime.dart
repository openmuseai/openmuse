import 'dart:io';

import 'package:appflowy/brand/brand.dart';

/// Resolves the DSH sidecar layout for source-tree development and packed
/// apps: macOS `Contents/Resources/muse`, Windows `{exeDir}/muse`.
class DshRuntimeLayout {
  const DshRuntimeLayout({
    required this.bundled,
    required this.museRoot,
    required this.dshHome,
    required this.harnessDir,
    required this.patchFile,
    required this.nodeBin,
    required this.credentialsFile,
    this.closureEntry,
  });

  final bool bundled;
  final String museRoot;
  final String dshHome;
  final String harnessDir;
  final String patchFile;
  final String? nodeBin;
  final String credentialsFile;

  /// Symlink-free closure entry (`muse/closure/node_modules/@deepseek-ai/dsh/lib/bin.js`).
  /// Non-null in the closure layout; the legacy whole-tree bundle runs tsx
  /// source instead.
  final String? closureEntry;

  String get runDshScript =>
      '$museRoot/middlewares/scripts/run-dsh-appflowy.sh';

  static bool looksLikeMuseRoot(String path) {
    return Directory('$path/middlewares/dsh').existsSync() &&
        Directory('$path/frontend/client').existsSync();
  }

  /// Walks [searchFrom] (default: cwd + executable) looking for the split
  /// Muse repo. Honors `MUSE_ROOT` when that directory exists.
  static String? discoverMuseRoot({Iterable<String>? searchFrom}) {
    final env = Platform.environment['MUSE_ROOT']?.trim();
    if (env != null && env.isNotEmpty && Directory(env).existsSync()) {
      return env;
    }
    final starts = searchFrom ??
        <String>[
          Directory.current.path,
          File(Platform.resolvedExecutable).parent.path,
        ];
    for (final start in starts) {
      var dir = Directory(start);
      for (var i = 0; i < 16; i++) {
        if (looksLikeMuseRoot(dir.path)) return dir.path;
        final parent = dir.parent;
        if (parent.path == dir.path) break;
        dir = parent;
      }
    }
    return null;
  }

  static String get userMuseHome {
    final preferred = _preferredUserMuseHome();
    _migrateLegacyUserMuseHome(preferred);
    return preferred;
  }

  static String _preferredUserMuseHome() {
    if (Platform.isWindows) {
      final appData = Platform.environment['APPDATA']?.trim();
      if (appData != null && appData.isNotEmpty) {
        return '$appData/${Brand.dataDirName}';
      }
      final profile = Platform.environment['USERPROFILE']?.trim();
      if (profile != null && profile.isNotEmpty) {
        return '$profile/AppData/Roaming/${Brand.dataDirName}';
      }
    } else if (Platform.isLinux) {
      final home = Platform.environment['HOME']?.trim();
      if (home != null && home.isNotEmpty) {
        return '$home/.local/share/${Brand.dataDirLinux}';
      }
    } else {
      final home = Platform.environment['HOME']?.trim();
      if (home != null && home.isNotEmpty) {
        return '$home/Library/Application Support/${Brand.dataDirName}';
      }
    }
    return '${Directory.systemTemp.path}/${Brand.dataDirLinux}';
  }

  static List<String> _legacyUserMuseHomes() {
    final homes = <String>[];
    if (Platform.isWindows) {
      final appData = Platform.environment['APPDATA']?.trim();
      final profile = Platform.environment['USERPROFILE']?.trim();
      if (appData != null && appData.isNotEmpty) {
        homes.addAll([
          '$appData/Muse',
          '$appData/DSH Office/Muse',
          '$appData/OpenMuse AI/Muse',
        ]);
      }
      if (profile != null && profile.isNotEmpty) {
        homes.add('$profile/AppData/Roaming/DSH Office/Muse');
      }
    } else if (Platform.isLinux) {
      final home = Platform.environment['HOME']?.trim();
      if (home != null && home.isNotEmpty) {
        homes.addAll([
          '$home/.local/share/muse',
          '$home/.local/share/dsh-office/muse',
        ]);
      }
    } else {
      final home = Platform.environment['HOME']?.trim();
      if (home != null && home.isNotEmpty) {
        homes.addAll([
          '$home/Library/Application Support/Muse',
          '$home/Library/Application Support/AppFlowy/Muse',
          '$home/Library/Application Support/DSH Office/Muse',
          '$home/Library/Application Support/OpenMuse AI/Muse',
        ]);
      }
    }
    return homes;
  }

  static bool _dirHasEntries(Directory dir) {
    try {
      return dir.existsSync() && dir.listSync(followLinks: false).isNotEmpty;
    } catch (_) {
      return false;
    }
  }

  static void _copyDirectory(Directory source, Directory dest) {
    dest.createSync(recursive: true);
    for (final entity in source.listSync(recursive: true, followLinks: false)) {
      final relative = entity.path.substring(source.path.length);
      final targetPath = '${dest.path}$relative';
      if (entity is Directory) {
        Directory(targetPath).createSync(recursive: true);
      } else if (entity is File) {
        File(targetPath).parent.createSync(recursive: true);
        entity.copySync(targetPath);
      }
    }
  }

  /// Move (or copy) the first non-empty legacy DSH data dir into [preferred].
  static void _migrateLegacyUserMuseHome(String preferred) {
    final dest = Directory(preferred);
    if (_dirHasEntries(dest)) return;
    for (final legacy in _legacyUserMuseHomes()) {
      if (legacy == preferred) continue;
      final src = Directory(legacy);
      if (!_dirHasEntries(src)) continue;
      try {
        dest.parent.createSync(recursive: true);
        src.renameSync(preferred);
        return;
      } catch (_) {
        try {
          _copyDirectory(src, dest);
          return;
        } catch (_) {}
      }
    }
  }

  static String get defaultDshHome {
    final override = Platform.environment['MUSE_DSH_HOME'];
    if (override != null && override.trim().isNotEmpty) {
      return override.trim();
    }
    return '$userMuseHome/dsh';
  }

  static String get credentialsPath => '$userMuseHome/credentials.env';

  static Directory? bundleRoot() {
    final fromEnv = Platform.environment['MUSE_BUNDLE_ROOT'];
    if (fromEnv != null && fromEnv.trim().isNotEmpty) {
      final directory = Directory(fromEnv.trim());
      if (looksLikeBundleRoot(directory.path)) return directory;
    }
    final exe = File(Platform.resolvedExecutable);
    if (Platform.isMacOS) {
      final resources = Directory('${exe.parent.parent.path}/Resources/muse');
      if (looksLikeBundleRoot(resources.path)) return resources;
    }
    if (Platform.isWindows || Platform.isLinux) {
      final beside = Directory('${exe.parent.path}/muse');
      if (looksLikeBundleRoot(beside.path)) return beside;
    }
    return null;
  }

  static bool looksLikeBundleRoot(String root) {
    return File('$root/patch.yml').existsSync() &&
        (Directory('$root/dsh').existsSync() ||
            Directory('$root/closure').existsSync());
  }

  /// Official Node tarball uses `bin/node`; the Windows zip uses `node.exe`.
  static String? bundledNodePath(String root) {
    for (final rel in [
      'node/bin/node',
      'node/bin/node.exe',
      'node/node.exe',
    ]) {
      final file = File('$root/$rel');
      if (file.existsSync()) return file.path;
    }
    return null;
  }

  /// The symlink-free closure CLI entry, when the bundle ships one.
  static String? closureEntryPath(String root) {
    final candidate = File(
      '$root/closure/node_modules/@deepseek-ai/dsh/lib/bin.js',
    );
    return candidate.existsSync() ? candidate.path : null;
  }

  static DshRuntimeLayout resolve() {
    final bundle = bundleRoot();
    final dshHomePath = defaultDshHome;
    final credentials = credentialsPath;
    if (bundle != null) {
      final entry = closureEntryPath(bundle.path);
      return DshRuntimeLayout(
        bundled: true,
        museRoot: bundle.path,
        dshHome: dshHomePath,
        harnessDir:
            entry != null ? '${bundle.path}/closure' : '${bundle.path}/dsh',
        patchFile: '${bundle.path}/patch.yml',
        nodeBin: bundledNodePath(bundle.path),
        credentialsFile: credentials,
        closureEntry: entry,
      );
    }
    final source = discoverMuseRoot();
    if (source == null) {
      final exeDir = File(Platform.resolvedExecutable).parent.path;
      final beside = Directory('$exeDir/muse');
      if (beside.existsSync()) {
        throw StateError(
          'Packed Muse runtime at ${beside.path} is incomplete '
          '(need patch.yml and closure/ or dsh/). Reinstall ${Brand.productName}.',
        );
      }
      throw StateError(
        'Cannot find the Muse repo (middlewares/dsh + frontend/client). '
        'Set MUSE_ROOT to the openmuse checkout, or start the app from that tree.',
      );
    }
    final harnessOverride = Platform.environment['MUSE_HARNESS_DIR']?.trim();
    final harnessDir =
        (harnessOverride != null && harnessOverride.isNotEmpty)
            ? harnessOverride
            : '$source/vendors/deepseek-harness';
    return DshRuntimeLayout(
      bundled: false,
      museRoot: source,
      dshHome: dshHomePath,
      harnessDir: harnessDir,
      patchFile:
          '$source/middlewares/dsh/plugins/dsh-appflowy/cordis.patch.yml',
      nodeBin: null,
      credentialsFile: credentials,
    );
  }
}
