import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_engine.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:ffi/ffi.dart';

/// Host-facing text diff runtime.
///
/// Comparison always happens off the UI isolate. When `libdart_ffi` exports
/// `muse_diff_text_compare_json`, the native engine returns compact change-block
/// coordinates only. Missing symbols fall back to the Dart Myers engine so the
/// existing Host compare path keeps working.
final class MuseDiffTextRuntime {
  MuseDiffTextRuntime({
    MuseDiffTextFfiClient? ffi,
    this.forceDartFallback = false,
  }) : ffi = ffi ?? const MuseNativeDiffTextFfiClient();

  static final MuseDiffTextRuntime instance = MuseDiffTextRuntime();

  final MuseDiffTextFfiClient ffi;
  final bool forceDartFallback;

  Future<MuseTextDiffEngineResult> compare({
    required String before,
    required String after,
    Duration maxDuration = const Duration(seconds: 10),
    int maxTraceBytes = 256 * 1024 * 1024,
  }) async {
    if (!forceDartFallback && ffi.symbolsAvailable) {
      try {
        final json = identical(ffi, const MuseNativeDiffTextFfiClient()) ||
                ffi is MuseNativeDiffTextFfiClient
            ? await Isolate.run(
                () => const MuseNativeDiffTextFfiClient().compareJson(
                  before: before,
                  after: after,
                  maxMillis: maxDuration.inMilliseconds,
                  maxTraceBytes: maxTraceBytes,
                ),
              )
            : ffi.compareJson(
                before: before,
                after: after,
                maxMillis: maxDuration.inMilliseconds,
                maxTraceBytes: maxTraceBytes,
              );
        final parsed = MuseDiffTextFfiCodec.decode(json);
        if (parsed != null) return parsed;
      } catch (_) {
        // Native symbols can disappear between lookup and isolate load.
      }
    }
    try {
      return await Isolate.run(
        () => const MuseDartTextDiffEngine().compare(before, after),
      );
    } catch (_) {
      return const MuseDartTextDiffEngine().compare(before, after);
    }
  }
}

abstract interface class MuseDiffTextFfiClient {
  bool get symbolsAvailable;

  String compareJson({
    required String before,
    required String after,
    required int maxMillis,
    required int maxTraceBytes,
  });
}

final class MuseNativeDiffTextFfiClient implements MuseDiffTextFfiClient {
  const MuseNativeDiffTextFfiClient();

  static _NativeBindings? _bindings;
  static bool _lookupAttempted = false;

  @override
  bool get symbolsAvailable => _load() != null;

  @override
  String compareJson({
    required String before,
    required String after,
    required int maxMillis,
    required int maxTraceBytes,
  }) {
    final native = _load();
    if (native == null) {
      throw StateError('muse_diff_text FFI symbols are unavailable');
    }
    final requestId = DateTime.now().microsecondsSinceEpoch;
    final beforePtr = before.toNativeUtf8();
    final afterPtr = after.toNativeUtf8();
    Pointer<Utf8> resultPtr = nullptr;
    try {
      resultPtr = native.compare(
        requestId,
        beforePtr,
        afterPtr,
        maxMillis < 1 ? 1 : maxMillis,
        maxTraceBytes < 1 ? 1 : maxTraceBytes,
      );
      if (resultPtr == nullptr) {
        throw StateError('muse_diff_text returned a null JSON pointer');
      }
      return resultPtr.toDartString();
    } finally {
      malloc.free(beforePtr);
      malloc.free(afterPtr);
      if (resultPtr != nullptr) {
        native.freeJson(resultPtr);
      }
    }
  }

  static _NativeBindings? _load() {
    if (_lookupAttempted) return _bindings;
    _lookupAttempted = true;
    try {
      final library = _openLibrary();
      _bindings = _NativeBindings(
        compare: library.lookupFunction<_CompareNative, _CompareDart>(
          'muse_diff_text_compare_json',
        ),
        freeJson: library.lookupFunction<_FreeNative, _FreeDart>(
          'muse_diff_text_free_json',
        ),
      );
    } catch (_) {
      _bindings = null;
    }
    return _bindings;
  }

  static DynamicLibrary _openLibrary() {
    if (Platform.environment.containsKey('FLUTTER_TEST')) {
      final prefix = '${Directory.current.path}/.sandbox';
      if (Platform.isMacOS) {
        return DynamicLibrary.open('$prefix/libdart_ffi.dylib');
      }
      if (Platform.isLinux || Platform.isAndroid) {
        return DynamicLibrary.open('$prefix/libdart_ffi.so');
      }
      if (Platform.isWindows) {
        return DynamicLibrary.open('$prefix/dart_ffi.dll');
      }
    }
    if (Platform.isMacOS || Platform.isIOS) {
      return DynamicLibrary.executable();
    }
    if (Platform.isLinux || Platform.isAndroid) {
      return DynamicLibrary.open('libdart_ffi.so');
    }
    if (Platform.isWindows) {
      return DynamicLibrary.open('dart_ffi.dll');
    }
    throw UnsupportedError('Unsupported FFI platform');
  }
}

final class MuseDiffTextFfiCodec {
  static const allowedKeys = {
    'quality',
    'blocks',
    'similarBoundaries',
    'additions',
    'deletions',
    'beforeNewline',
    'afterNewline',
    'error',
  };

  static const contentLeakKeys = {
    'text',
    'before',
    'after',
    'content',
    'lines',
    'rows',
    'hunks',
    'preview',
  };

  static MuseTextDiffEngineResult? decode(String json) {
    final decoded = jsonDecode(json);
    if (decoded is! Map) return null;
    final map = Map<String, dynamic>.from(decoded);
    if (map['error'] != null) return null;
    for (final key in map.keys) {
      if (contentLeakKeys.contains(key)) {
        throw FormatException('FFI change-block payload leaked content: $key');
      }
    }
    final blocks = <MuseChangeBlock>[];
    final rawBlocks = map['blocks'];
    if (rawBlocks is List) {
      for (final entry in rawBlocks) {
        if (entry is Map) {
          blocks
              .add(MuseChangeBlock.fromJson(Map<String, dynamic>.from(entry)));
        }
      }
    }
    final boundaries = <MuseSimilarBoundary>[];
    final rawBoundaries = map['similarBoundaries'];
    if (rawBoundaries is List) {
      for (final entry in rawBoundaries) {
        if (entry is Map) {
          boundaries.add(
            MuseSimilarBoundary.fromJson(Map<String, dynamic>.from(entry)),
          );
        }
      }
    }
    return MuseTextDiffEngineResult(
      blocks: blocks,
      additions: map['additions'] as int? ?? 0,
      deletions: map['deletions'] as int? ?? 0,
      engine: 'muse-diff-text-ffi',
      quality: switch (map['quality'] as String?) {
        'budget-exceeded' => MuseDiffQualityKind.budgetExceeded,
        'cancelled' => MuseDiffQualityKind.failed,
        _ => MuseDiffQualityKind.exact,
      },
      similarBoundaries: boundaries,
    );
  }
}

final class _NativeBindings {
  const _NativeBindings({required this.compare, required this.freeJson});

  final _CompareDart compare;
  final _FreeDart freeJson;
}

typedef _CompareNative = Pointer<Utf8> Function(
  Uint64 requestId,
  Pointer<Utf8> before,
  Pointer<Utf8> after,
  Uint64 maxMillis,
  Uint64 maxTraceBytes,
);
typedef _CompareDart = Pointer<Utf8> Function(
  int requestId,
  Pointer<Utf8> before,
  Pointer<Utf8> after,
  int maxMillis,
  int maxTraceBytes,
);
typedef _FreeNative = Void Function(Pointer<Utf8> value);
typedef _FreeDart = void Function(Pointer<Utf8> value);
