import 'dart:io';

/// Locate `libppt_core_ffi` built by cargo (debug first, then release).
String? findFfiLibrary() {
  final env = Platform.environment['PPT_CORE_FFI'];
  if (env != null && env.isNotEmpty && File(env).existsSync()) return env;

  final names = Platform.isWindows
      ? <String>['ppt_core_ffi.dll']
      : Platform.isMacOS
          ? <String>['libppt_core_ffi.dylib']
          : <String>['libppt_core_ffi.so'];

  final roots = <String>[];

  void add(String path) {
    if (path.isNotEmpty) roots.add(path);
  }

  try {
    final exe = File(Platform.resolvedExecutable).absolute;
    var dir = exe.parent;
    for (var i = 0; i < 10; i++) {
      add('${dir.path}/../Frameworks');
      add(dir.path);
      add('${dir.path}/native/target/debug');
      add('${dir.path}/native/target/release');
      dir = dir.parent;
    }
  } catch (_) {}

  try {
    var dir = Directory.current.absolute;
    for (var i = 0; i < 10; i++) {
      add('${dir.path}/native/target/debug');
      add('${dir.path}/native/target/release');
      add('${dir.path}/samples/ppt-render/native/target/debug');
      add('${dir.path}/samples/ppt-render/native/target/release');
      dir = dir.parent;
    }
  } catch (_) {}

  addAllKnownRelatives(roots);

  for (final root in roots) {
    for (final name in names) {
      final f = File('$root/$name');
      if (f.existsSync()) return f.absolute.path;
    }
  }
  return null;
}

void addAllKnownRelatives(List<String> roots) {
  roots.addAll([
    '../native/target/debug',
    '../native/target/release',
    '../../native/target/debug',
    '../../native/target/release',
    'native/target/debug',
    'native/target/release',
    '../../../ppt-render/native/target/debug',
    '../../../ppt-render/native/target/release',
    '../../ppt-render/native/target/debug',
    '../../ppt-render/native/target/release',
  ]);
}
