import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/services.dart' show rootBundle;

bool get isAndroidHost => Platform.isAndroid;

/// Locate `libword_core_ffi` built by cargo (debug first, then release).
String? findFfiLibrary() {
  final env = Platform.environment['WORD_CORE_FFI'];
  if (env != null && env.isNotEmpty && File(env).existsSync()) return env;

  final names = _libNames();

  final roots = <String>[];

  // Signed .app: Frameworks copy first. Cargo debug dylib is unsigned; dlopen
  // from a signed bundle is SIGKILL (Code Signature Invalid) on recent macOS.
  try {
    final exe = File(Platform.resolvedExecutable).absolute;
    final macosDir = exe.parent;
    roots.add('${macosDir.path}/../Frameworks');
    roots.add(macosDir.path);
  } catch (_) {}

  roots.addAll([
    '../native/target/debug',
    '../native/target/release',
    '../../native/target/debug',
    '../../native/target/release',
    'native/target/debug',
    'native/target/release',
    '../../../docx-mvp-render/native/target/debug',
    '../../../docx-mvp-render/native/target/release',
    '../../docx-mvp-render/native/target/debug',
    '../../docx-mvp-render/native/target/release',
  ]);

  for (final root in roots) {
    for (final name in names) {
      final f = File('$root/$name');
      if (f.existsSync()) return f.absolute.path;
    }
  }
  return null;
}

/// 开箱路径：先查宿主/环境，再查随包预编译产物（asset 提取）。
///
/// `findFfiLibrary` 保持同步兼容旧调用；新代码用本方法（异步，含 asset 回退）。
Future<String?> resolveFfiLibrary() async {
  // Android packages the Rust library in jniLibs; the platform loader resolves
  // its soname from the installed APK. There is no readable File path here.
  if (Platform.isAndroid) return 'libword_core_ffi.so';
  final quick = findFfiLibrary();
  if (quick != null) return quick;
  return extractBundledFfi();
}

/// 平台子目录名（与 `native/<platform>/` 一致）。
String? _platformKey() {
  if (Platform.isMacOS) return 'macos';
  if (Platform.isLinux) return 'linux';
  if (Platform.isWindows) return 'windows';
  if (Platform.isAndroid) return 'android';
  return null; // iOS/web 走其它集成路径（见 README）
}

List<String> _libNames() {
  if (Platform.isWindows) return <String>['word_core_ffi.dll'];
  if (Platform.isMacOS) return <String>['libword_core_ffi.dylib'];
  return <String>['libword_core_ffi.so'];
}

/// 提取目录版本号：native ABI 变更时手动 bump（避免旧缓存）。
const String _kExtractVersion = 'v1';

/// 从 flutter asset `packages/word_render/native/<platform>/<lib>` 提取到
/// 每平台可写缓存目录并返回绝对路径；macOS 上先做 ad-hoc 签名（unsigned
/// dylib 在签名 App 中 dlopen 会被 SIGKILL）。
Future<String?> extractBundledFfi() async {
  final platform = _platformKey();
  if (platform == null) return null;
  final libName = _libNames().first;

  final assetKey = 'packages/word_render/native/$platform/$libName';
  final ByteData bytes;
  try {
    bytes = await rootBundle.load(assetKey);
  } catch (_) {
    return null; // 该平台产物未随包分发
  }

  final dir = _cacheDir('word_render_native/$_kExtractVersion/$platform');
  final target = File('${dir.path}/$libName');
  try {
    if (!target.existsSync() || target.lengthSync() != bytes.lengthInBytes) {
      await dir.create(recursive: true);
      await target.writeAsBytes(
        bytes.buffer.asUint8List(bytes.offsetInBytes, bytes.lengthInBytes),
        flush: true,
      );
    }
    if (Platform.isMacOS) {
      // 失败不影响返回：dev 未装 Xcode 时让后续 dlopen 试错。
      try {
        await Process.run('codesign', ['--force', '-s', '-', target.path]);
      } catch (_) {}
    }
    return target.absolute.path;
  } catch (_) {
    return null;
  }
}

/// 无插件依赖的跨平台缓存目录（不用 path_provider，避免给消费者加插件）。
Directory _cacheDir(String sub) {
  final env = Platform.environment;
  String base;
  if (Platform.isMacOS) {
    base = '${env['HOME'] ?? '.'}/Library/Application Support';
  } else if (Platform.isWindows) {
    base = env['LOCALAPPDATA'] ?? (env['USERPROFILE'] ?? '.');
  } else if (Platform.isLinux) {
    base = env['XDG_CACHE_HOME'] ?? '${env['HOME'] ?? '.'}/.cache';
  } else {
    // Android：进程私有缓存目录（可通过 `getApplicationCacheDir` 取得，这里用环境近似）
    base = env['TMPDIR'] ?? Directory.systemTemp.path;
  }
  return Directory('$base/$sub');
}
