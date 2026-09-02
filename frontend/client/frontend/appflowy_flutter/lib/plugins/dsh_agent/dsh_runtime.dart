import 'dart:io';

/// Resolves the DSH sidecar layout for both source-tree development and a
/// packed macOS .app (Contents/Resources/muse).
class DshRuntimeLayout {
  const DshRuntimeLayout({
    required this.bundled,
    required this.museRoot,
    required this.dshHome,
    required this.harnessDir,
    required this.patchFile,
    required this.nodeBin,
    required this.credentialsFile,
  });

  final bool bundled;
  final String museRoot;
  final String dshHome;
  final String harnessDir;
  final String patchFile;
  final String? nodeBin;
  final String credentialsFile;

  static String get userMuseHome {
    final home = Platform.environment['HOME'];
    if (home == null || home.isEmpty) {
      return '${Directory.systemTemp.path}/appflowy-muse';
    }
    return '$home/Library/Application Support/AppFlowy/Muse';
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
      if (directory.existsSync()) return directory;
    }
    if (!Platform.isMacOS) return null;
    final exe = File(Platform.resolvedExecutable);
    final resources = Directory('${exe.parent.parent.path}/Resources/muse');
    if (_looksLikeBundle(resources.path)) return resources;
    return null;
  }

  static bool _looksLikeBundle(String root) {
    return File('$root/patch.yml').existsSync() &&
        Directory('$root/dsh').existsSync();
  }

  static DshRuntimeLayout resolve() {
    final bundle = bundleRoot();
    final dshHomePath = defaultDshHome;
    final credentials = credentialsPath;
    if (bundle != null) {
      final node = File('${bundle.path}/node/bin/node');
      return DshRuntimeLayout(
        bundled: true,
        museRoot: bundle.path,
        dshHome: dshHomePath,
        harnessDir: '${bundle.path}/dsh',
        patchFile: '${bundle.path}/patch.yml',
        nodeBin: node.existsSync() ? node.path : null,
        credentialsFile: credentials,
      );
    }
    final source = Platform.environment['MUSE_ROOT'] ?? '/Users/mac/src/muse';
    return DshRuntimeLayout(
      bundled: false,
      museRoot: source,
      dshHome: dshHomePath,
      harnessDir: '$source/vendors/deepseek-harness',
      patchFile: '$source/middlewares/dsh/plugins/dsh-appflowy/cordis.patch.yml',
      nodeBin: null,
      credentialsFile: credentials,
    );
  }
}
