import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';

import 'envelope.dart';
import 'ffi_loader_stub.dart' if (dart.library.io) 'ffi_loader_io.dart';

typedef _HelloNative = Pointer<Utf8> Function();
typedef _HelloDart = Pointer<Utf8> Function();
typedef _PptxNative = Pointer<Utf8> Function(Pointer<Uint8>, Uint64);
typedef _PptxDart = Pointer<Utf8> Function(Pointer<Uint8>, int);
typedef _PageNative = Pointer<Utf8> Function(Pointer<Uint8>, Uint64, Uint64);
typedef _PageDart = Pointer<Utf8> Function(Pointer<Uint8>, int, int);
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

/// One-shot ppt-core pipeline: hello fixture or `.pptx` bytes → JSON envelope.
class PptCoreEngine {
  DynamicLibrary? _lib;
  _HelloDart? _hello;
  _PptxDart? _pptx;
  _PageDart? _page;
  _FreeDart? _free;

  bool ffiReady = false;
  String source = 'not loaded';

  Future<void> initFfi() async {
    final path = findFfiLibrary();
    if (path == null) {
      source = 'libppt_core_ffi not found (build samples/ppt-render/native)';
      ffiReady = false;
      return;
    }
    try {
      final lib = DynamicLibrary.open(path);
      _hello = lib.lookup<NativeFunction<_HelloNative>>('ppt_render_hello').asFunction();
      _pptx = lib.lookup<NativeFunction<_PptxNative>>('ppt_render_pptx').asFunction();
      _page = lib.lookup<NativeFunction<_PageNative>>('ppt_render_page').asFunction();
      _free = lib.lookup<NativeFunction<_FreeNative>>('ppt_string_free').asFunction();
      _lib = lib;
      ffiReady = _lib != null;
      source = path;
    } catch (e) {
      ffiReady = false;
      source = 'dlopen failed: $e';
    }
  }

  String _takeJson(Pointer<Utf8> ptr) {
    if (ptr == nullptr) {
      throw StateError('ppt-core FFI returned null');
    }
    final s = ptr.toDartString();
    _free!(ptr);
    return s;
  }

  RenderEnvelope renderHello() {
    if (!ffiReady) {
      throw StateError('FFI not ready: $source');
    }
    return RenderEnvelope.fromJsonString(_takeJson(_hello!()));
  }

  RenderEnvelope renderPptx(Uint8List bytes) {
    if (!ffiReady) {
      throw StateError('FFI not ready: $source');
    }
    final ptr = malloc<Uint8>(bytes.length);
    try {
      ptr.asTypedList(bytes.length).setAll(0, bytes);
      return RenderEnvelope.fromJsonString(_takeJson(_pptx!(ptr, bytes.length)));
    } finally {
      malloc.free(ptr);
    }
  }

  /// Indexed page JSON promoted to a one-page envelope (`pages[0]` = selected page).
  RenderEnvelope renderPptxPage(Uint8List bytes, int index) {
    if (!ffiReady) {
      throw StateError('FFI not ready: $source');
    }
    final ptr = malloc<Uint8>(bytes.length);
    try {
      ptr.asTypedList(bytes.length).setAll(0, bytes);
      return RenderEnvelope.fromJsonString(
        _takeJson(_page!(ptr, bytes.length, index)),
      );
    } finally {
      malloc.free(ptr);
    }
  }
}

/// Isolate-friendly wrapper for tests that only need JSON parse.
@visibleForTesting
RenderEnvelope envelopeFromJson(String raw) => RenderEnvelope.fromJsonString(raw);
