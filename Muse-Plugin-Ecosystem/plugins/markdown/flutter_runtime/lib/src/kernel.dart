import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';
import 'dart:math';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter_timezone/flutter_timezone.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

typedef _NativeStart = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _DartStart = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _NativeQuery = Pointer<Utf8> Function();
typedef _DartQuery = Pointer<Utf8> Function();
typedef _NativeFree = Void Function(Pointer<Utf8>);
typedef _DartFree = void Function(Pointer<Utf8>);

/// This package owns both the engine and its data. No Host command, workspace
/// database, cloud endpoint, MCP server, or downloaded executable is required.
final class SiYuanKernel {
  SiYuanKernel({this.supportDirectory});
  final Directory? supportDirectory;
  Future<Uri>? _starting;
  Uri? _origin;
  Uri? get origin => _origin;
  static final String _session = List<int>.generate(
    32,
    (_) => Random.secure().nextInt(256),
  ).map((byte) => byte.toRadixString(16).padLeft(2, '0')).join();
  Map<String, String> get localSessionHeaders => {
    'X-OpenMuse-Local-Session': _session,
  };

  Future<Uri> start({String language = 'en'}) =>
      _starting ??= _start(language).catchError((Object error) {
        _starting = null;
        throw error;
      });

  Future<Uri> _start(String language) async {
    final support = supportDirectory ?? await getApplicationSupportDirectory();
    final base = Directory(
      p.join(support.path, 'plugins', 'io.openmuse.siyuan'),
    );
    final bundle = await rootBundle.load(
      'packages/openmuse_siyuan_client/assets/runtime.zip',
    );
    final bytes = bundle.buffer.asUint8List(
      bundle.offsetInBytes,
      bundle.lengthInBytes,
    );
    final digest = sha256.convert(bytes).toString();
    final resources = await Isolate.run(
      () => installSiYuanResources(bytes, base.path, digest),
    );
    final data = Directory(p.join(base.path, 'userdata'));
    await data.create(recursive: true);
    var languageTag = siYuanLanguageTag(language);
    if (!await File(
      p.join(resources, 'app', 'appearance', 'langs', '$languageTag.json'),
    ).exists()) {
      languageTag = 'en';
    }
    final arguments = jsonEncode({
      'resources': resources,
      'data': data.path,
      'platform': Platform.operatingSystem,
      'language': languageTag,
      'timezone': (await FlutterTimezone.getLocalTimezone()).identifier,
      'session': _session,
      if (Platform.isAndroid) 'ocrLibrary': 'libsiyuan_ort.so',
      if (Platform.isMacOS)
        'ocrLibrary': p.join(
          File(Platform.resolvedExecutable).parent.parent.path,
          'Resources',
          'libonnxruntime.dylib',
        ),
    });
    var state = await Isolate.run(() => _invoke('start', arguments));
    final deadline = DateTime.now().add(const Duration(seconds: 90));
    while (state['state'] != 'ready') {
      if (state['error'] case final String error when error.isNotEmpty) {
        throw StateError(error);
      }
      if (DateTime.now().isAfter(deadline))
        throw TimeoutException('SiYuan local startup timed out');
      await Future<void>.delayed(const Duration(milliseconds: 100));
      state = await Isolate.run(() => _invoke('status'));
    }
    final origin = Uri.parse(state['origin'] as String);
    if (origin.scheme != 'http' ||
        origin.host != '127.0.0.1' ||
        !origin.hasPort ||
        origin.path.isNotEmpty) {
      throw StateError('Kernel returned an invalid local origin');
    }
    return _origin = origin;
  }

  Future<void> flush() async {
    if (_origin == null) return;
    final result = await Isolate.run(() => _invoke('flush'));
    if (result['saved'] != true) throw StateError('${result['error']}');
  }

  Future<void> suspend() async {
    if (_origin == null) return;
    final result = await Isolate.run(() => _invoke('suspend'));
    if (result['saved'] != true) throw StateError('${result['error']}');
  }

  Future<void> resume() async {
    if (_origin == null) return;
    await Isolate.run(() => _invoke('resume'));
  }
}

/// Map Flutter locale forms to the original client's bundled language names.
String siYuanLanguageTag(String language) {
  final tag = language.replaceAll('_', '-').toLowerCase();
  if (tag == 'zh' || tag.startsWith('zh-')) {
    return RegExp(r'(^|-)(hant|tw|hk|mo)($|-)').hasMatch(tag)
        ? 'zh-TW'
        : 'zh-CN';
  }
  if (tag == 'pt' || tag.startsWith('pt-')) return 'pt-BR';
  return tag.split('-').first;
}

Map<String, Object?> _invoke(String method, [String? arguments]) {
  final library = Platform.isIOS
      ? DynamicLibrary.process()
      : DynamicLibrary.open(
          Platform.isMacOS ? 'libmuse_siyuan.dylib' : 'libmuse_siyuan.so',
        );
  final release = library.lookupFunction<_NativeFree, _DartFree>(
    'muse_siyuan_free',
  );
  Pointer<Utf8> output;
  if (method == 'start') {
    final input = arguments!.toNativeUtf8();
    try {
      output = library.lookupFunction<_NativeStart, _DartStart>(
        'muse_siyuan_start',
      )(input);
    } finally {
      malloc.free(input);
    }
  } else {
    output = library.lookupFunction<_NativeQuery, _DartQuery>(
      'muse_siyuan_$method',
    )();
  }
  try {
    return Map<String, Object?>.from(jsonDecode(output.toDartString()) as Map);
  } finally {
    release(output);
  }
}

/// Digest-addressed immutable resources are separate from mutable notebooks.
/// Extraction rejects links/traversal and verifies every bundled file.
String installSiYuanResources(List<int> bytes, String base, String digest) {
  if (!RegExp(r'^[a-f0-9]{64}$').hasMatch(digest) ||
      sha256.convert(bytes).toString() != digest) {
    throw const FormatException('Invalid runtime digest');
  }
  final destination = Directory(p.join(base, 'resources', digest));
  if (File(p.join(destination.path, '.complete')).existsSync())
    return destination.path;
  final staging = Directory('${destination.path}.staging');
  if (staging.existsSync()) staging.deleteSync(recursive: true);
  staging.createSync(recursive: true);
  try {
    final archive = ZipDecoder().decodeBytes(bytes);
    final manifest = archive.findFile('catalog.json');
    if (manifest == null)
      throw const FormatException('Missing resource catalog');
    final catalog = Map<String, dynamic>.from(
      jsonDecode(utf8.decode(manifest.content)) as Map,
    );
    final names = <String>{};
    var total = 0;
    for (final entry in archive.files) {
      final name = entry.name;
      if (entry.isSymbolicLink ||
          name.contains('\\') ||
          name.startsWith('/') ||
          name.split('/').any((part) => part == '..' || part == '.') ||
          !names.add(name)) {
        throw const FormatException('Unsafe resource archive');
      }
      if (!entry.isFile || name == 'catalog.json') continue;
      total += entry.size;
      if (total > 512 * 1024 * 1024 || names.length > 20000)
        throw const FormatException('Resource budget exceeded');
      if (catalog.remove(name) != sha256.convert(entry.content).toString()) {
        throw FormatException('Resource digest mismatch: $name');
      }
      final file = File(p.join(staging.path, name));
      file.parent.createSync(recursive: true);
      file.writeAsBytesSync(entry.content, flush: true);
    }
    if (catalog.isNotEmpty)
      throw const FormatException('Incomplete resource archive');
    File(
      p.join(staging.path, '.complete'),
    ).writeAsStringSync(digest, flush: true);
    if (destination.existsSync()) destination.deleteSync(recursive: true);
    staging.renameSync(destination.path);
    return destination.path;
  } catch (_) {
    if (staging.existsSync()) staging.deleteSync(recursive: true);
    rethrow;
  }
}
