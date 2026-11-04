import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'dart:typed_data';

@JS('kspProductBridgeReady')
external JSPromise<JSObject> get _bridgeReady;

final Future<JSObject> webProductBridge = _bridgeReady.toDart;

R webCall<R extends JSAny?>(
  JSObject bridge,
  String method, [
  List<JSAny?> arguments = const [],
]) => bridge.callMethodVarArgs<R>(method.toJS, arguments);

int webInt(JSObject value, String name) => (value[name] as JSNumber).toDartInt;

double webDouble(JSObject value, String name) =>
    (value[name] as JSNumber).toDartDouble;

String webString(JSObject value, String name) =>
    (value[name] as JSString).toDart;

Uint8List webBytes(JSUint8Array value) => Uint8List.fromList(value.toDart);
