import 'dart:async';
import 'dart:convert';
import 'dart:js_interop';
import 'package:web/web.dart' as web;
import 'core_api.dart';

@JS('muse_appflowy_realtime_wasm')
external JSPromise<JSAny?> _initialize(JSObject options);
@JS('muse_appflowy_realtime_wasm.muse_appflowy_realtime_call_json')
external JSString _call(JSString request);

/// Uses the same bounded original AppFlowy protocol and Yrs Awareness as native.
final class WasmRealtimeWireCore implements RealtimeWireCore {
  WasmRealtimeWireCore._();
  static Future<void>? _initializing;
  static Future<void>? _scriptLoaded;
  static Uri? _boundAssets;
  static Future<RealtimeWireCore> load({Uri? assetBase}) async {
    final uri =
        assetBase ??
        Uri.parse(
          web.document.baseURI,
        ).resolve('assets/packages/muse_appflowy_realtime/assets/wasm/');
    if (_boundAssets != null && uri != _boundAssets) {
      throw const RealtimeFailure('ENGINE_ASSET_BINDING_CHANGED');
    }
    _boundAssets = uri;
    await (_initializing ??= _load(uri).catchError((Object error) {
      _initializing = null;
      throw error;
    }));
    return WasmRealtimeWireCore._();
  }

  static Future<void> _loadScript(Uri base) async {
    final loaded = Completer<void>();
    final script = web.HTMLScriptElement()
      ..src = base.resolve('muse_appflowy_realtime.js').toString();
    script.onload = ((web.Event _) => loaded.complete()).toJS;
    script.onerror = ((web.Event _) => loaded.completeError(
      const RealtimeFailure('WASM_LOADER_UNAVAILABLE'),
    )).toJS;
    web.document.head!.appendChild(script);
    try {
      await loaded.future.timeout(const Duration(seconds: 20));
    } on Object {
      script.remove();
      _scriptLoaded = null;
      rethrow;
    }
  }

  static Future<void> _load(Uri base) async {
    await (_scriptLoaded ??= _loadScript(base));
    await _initialize(
      <String, String>{
            'module_or_path': base
                .resolve('muse_appflowy_realtime_bg.wasm')
                .toString(),
          }.jsify()!
          as JSObject,
    ).toDart.timeout(const Duration(seconds: 20));
  }

  @override
  Map<String, dynamic> call(Map<String, dynamic> request) {
    final response = jsonDecode(_call(jsonEncode(request).toJS).toDart) as Map;
    if (response['ok'] != true) {
      throw RealtimeFailure(response['error'] as String? ?? 'ENGINE_FAILURE');
    }
    return (response['data'] as Map).cast<String, dynamic>();
  }
}

Future<RealtimeWireCore> loadRealtimeWireCore({Uri? wasmAssetBase}) =>
    WasmRealtimeWireCore.load(assetBase: wasmAssetBase);
