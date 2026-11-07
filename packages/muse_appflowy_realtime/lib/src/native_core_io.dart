import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'core_api.dart';

@Native<Pointer<Utf8> Function(Pointer<Utf8>)>(
  symbol: 'muse_appflowy_realtime_call',
)
external Pointer<Utf8> _bundledCall(Pointer<Utf8> input);
@Native<Void Function(Pointer<Utf8>)>(symbol: 'muse_appflowy_realtime_free')
external void _bundledFree(Pointer<Utf8> output);

typedef _CallC = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _CallDart = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _FreeC = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

Future<RealtimeWireCore> loadRealtimeWireCore({Uri? wasmAssetBase}) async =>
    NativeRealtimeWireCore();

final class NativeRealtimeWireCore implements RealtimeWireCore {
  NativeRealtimeWireCore({String? libraryPath}) {
    if (libraryPath == null) {
      _call = _bundledCall;
      _free = _bundledFree;
      return;
    }
    final library = DynamicLibrary.open(libraryPath);
    _call = library.lookupFunction<_CallC, _CallDart>(
      'muse_appflowy_realtime_call',
    );
    _free = library.lookupFunction<_FreeC, _FreeDart>(
      'muse_appflowy_realtime_free',
    );
  }
  late final _CallDart _call;
  late final _FreeDart _free;
  @override
  Map<String, dynamic> call(Map<String, dynamic> request) {
    final input = jsonEncode(request).toNativeUtf8();
    Pointer<Utf8>? output;
    try {
      output = _call(input);
      final response =
          jsonDecode(output.toDartString()) as Map<String, dynamic>;
      if (response['ok'] != true) {
        throw RealtimeFailure(response['error'] as String? ?? 'ENGINE_FAILURE');
      }
      return (response['data'] as Map).cast<String, dynamic>();
    } finally {
      calloc.free(input);
      if (output != null) _free(output);
    }
  }
}
