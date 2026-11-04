import 'dart:convert';
import 'dart:ffi';
import 'dart:io';

import 'pdfium_bindings.dart' as pdfium_bindings;

const _isFlutter = bool.fromEnvironment('dart.library.ui');
bool get _isFlutterTest => Platform.environment.containsKey('FLUTTER_TEST');

/// Helper function to get PDFium instance.
///
/// PDFium is downloaded and bundled at build time by `hook/build.dart`.
/// Use [modulePath] only for custom deployments or tests.
///
/// This function supports both Flutter and pure Dart environments, and it
/// tries multiple strategies to locate the PDFium library:
/// - If [modulePath] is provided, it attempts to load the library from that
///   path directly.
/// - On iOS with Flutter, it uses the PDFium framework already in the process.
/// - On macOS with Flutter, it opens the same-source rebuild reader PDFium.
/// - For Linux with Flutter, it looks for the library from the
///   `[EXECUTABLE_DIR]/../lib/libpdfium.so` path.
/// - For other platforms, it tries to load the library from system paths using the standard file name.
/// - If all above strategies fail, it falls back to loading the library from
///   the bundled native asset; `dart test`, `dart run`, and `dart compile`
///   ensure the asset is available through `.dart_tool/native_assets.yaml`.
pdfium_bindings.PDFium getPdfium({String? modulePath}) {
  try {
    return pdfium_bindings.PDFium(_getModule(modulePath: modulePath));
  } catch (e) {
    throw Exception('Failed to load PDFium module: $e');
  }
}

/// Internal helper to load the PDFium dynamic library.
/// Use [modulePath] for custom paths, otherwise it tries system paths first and
/// falls back to bundled assets.
DynamicLibrary _getModule({String? modulePath}) {
  if (modulePath != null) {
    try {
      return DynamicLibrary.open(modulePath);
    } catch (e) {
      throw Exception(
        'Failed to load explicit PDFium module path "$modulePath": $e',
      );
    }
  }

  String? moduleFileName;
  try {
    if (Platform.isAndroid) {
      try {
        return DynamicLibrary.open('libpdfium_viewer.so');
      } on ArgumentError {
        // Other Android architectures still use the reader's upstream asset.
      }
    }
    if (Platform.isMacOS) {
      if (_isFlutterTest || !_isFlutter) {
        return DynamicLibrary.open(_findRebuildMacPdfium().path);
      }
      final contents = File(Platform.resolvedExecutable).parent.parent;
      final assetRoot = '${contents.path}/Frameworks/App.framework/Resources/flutter_assets';
      final candidates = [
        '$assetRoot/packages/pdf_rebuild_flutter/assets/native/libpdfium_viewer.dylib',
        '$assetRoot/assets/native/libpdfium_viewer.dylib',
      ];
      final productPdfium = File(candidates.firstWhere(
        (path) => File(path).existsSync(),
        orElse: () => candidates.first,
      ));
      return DynamicLibrary.open(productPdfium.path);
    }
    if (Platform.isIOS) {
      final productPdfium = File(
        '${File(Platform.resolvedExecutable).parent.path}/Frameworks/libpdfium_viewer.dylib',
      );
      if (productPdfium.existsSync()) {
        return DynamicLibrary.open(productPdfium.path);
      }
      // The product binary is currently built for arm64 simulators only.
      if (_isFlutter && !_isFlutterTest) return DynamicLibrary.process();
    }
    moduleFileName = _getModuleFileName();
    return DynamicLibrary.open(moduleFileName);
  } catch (primaryError) {
    try {
      // Fallback to loading from bundled native asset if not found in system paths.
      final nativeAssetPath = _resolveNativeAssetPath(
        'package:pdfium_dart/libpdfium',
      );
      return DynamicLibrary.open(nativeAssetPath);
    } catch (fallbackError) {
      throw Exception(
        'Tried to load the default PDFium module'
        '${moduleFileName == null ? '' : ' "$moduleFileName"'} but it failed: $primaryError. '
        'Then tried to load the bundled native asset, but that also failed: $fallbackError',
      );
    }
  }
}

File _findRebuildMacPdfium() {
  var directory = Directory.current;
  const relativePath = 'pdf-rebuild/3rdparty/prebuilt/lib/libpdfium_viewer.dylib';
  while (true) {
    final candidate = File('${directory.path}/$relativePath');
    if (candidate.existsSync()) return candidate;
    if (directory.parent.path == directory.path) {
      throw StateError('Rebuild PDFium binary not found: $relativePath');
    }
    directory = directory.parent;
  }
}

/// Gets the default PDFium library file name based on the platform.
String _getModuleFileName() {
  if (Platform.isAndroid) return 'libpdfium.so';
  if (Platform.isWindows) return 'pdfium.dll';
  if (Platform.isLinux) {
    // For Flutter on Linux, the PDFium library is bundled in the app's shared library directory.
    if (_isFlutter) {
      return '${File(Platform.resolvedExecutable).parent.path}/lib/libpdfium.so';
    }
    return 'libpdfium.so';
  }
  throw UnsupportedError('Unsupported platform');
}

String _resolveNativeAssetPath(String assetId) {
  final nativeAssetsFile = _findNativeAssetsFile();
  final contents = nativeAssetsFile
      .readAsLinesSync()
      .where((line) => !line.trimLeft().startsWith('#'))
      .join('\n');
  final json = jsonDecode(contents) as Map<String, Object?>;
  final nativeAssets = json['native-assets'] as Map<String, Object?>?;
  final targetAssets = nativeAssets?[_targetKey()] as Map<String, Object?>?;
  final asset = targetAssets?[assetId] as List<Object?>?;
  if (asset != null && asset.length == 2 && asset[0] == 'absolute') {
    return asset[1]! as String;
  }
  throw Exception('Asset not found in native assets file: $assetId');
}

File _findNativeAssetsFile() {
  Directory directory = Directory.current;
  while (true) {
    final candidate = File('${directory.path}/.dart_tool/native_assets.yaml');
    if (candidate.existsSync()) return candidate;

    final parent = directory.parent;
    if (parent.path == directory.path) {
      throw Exception('Native assets file not found.');
    }
    directory = parent;
  }
}

String _targetKey() {
  final match = RegExp(r'"([^_]+)_([^_]+)"').firstMatch(Platform.version);
  if (match == null) {
    throw Exception('Failed to parse platform version: ${Platform.version}');
  }
  final os = switch (match[1]!) {
    'macos' => 'macos',
    final value => value,
  };
  return '${os}_${match[2]!}';
}
