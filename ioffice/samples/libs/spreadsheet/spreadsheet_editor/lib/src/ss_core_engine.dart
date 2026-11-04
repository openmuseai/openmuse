import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'gridcraft_engine.dart';

typedef _CreateNative = Pointer<Void> Function();
typedef _CreateDart = Pointer<Void> Function();
typedef _DestroyNative = Void Function(Pointer<Void>);
typedef _DestroyDart = void Function(Pointer<Void>);
typedef _AbiNative = Uint32 Function();
typedef _AbiDart = int Function();
typedef _OpenNative =
    Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>, Pointer<Uint8>, IntPtr);
typedef _OpenDart =
    Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>, Pointer<Uint8>, int);
typedef _CallNative = Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>);
typedef _CallDart = Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>);
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

/// Loads `libss_core` and reads cells through `SpreadsheetSession`.
/// Edit, export, and new-workbook return an explicit unsupported error.
final class SsCoreEngine implements SpreadsheetEngine {
  SsCoreEngine({String? libraryPath}) {
    final dylib = DynamicLibrary.open(libraryPath ?? _defaultLibraryPath());
    final abi = dylib.lookupFunction<_AbiNative, _AbiDart>('ss_core_abi');
    if (abi() != 1) throw StateError('Incompatible ss-core engine ABI');
    _destroy = dylib.lookupFunction<_DestroyNative, _DestroyDart>('ss_core_destroy');
    _free = dylib.lookupFunction<_FreeNative, _FreeDart>('ss_core_string_free');
    _open = dylib.lookupFunction<_OpenNative, _OpenDart>('ss_core_open');
    _call = dylib.lookupFunction<_CallNative, _CallDart>('ss_core_call');
    _handle = dylib.lookupFunction<_CreateNative, _CreateDart>('ss_core_create')();
    if (_handle == nullptr) {
      throw StateError('Cannot create ss-core session');
    }
  }

  late Pointer<Void> _handle;
  late _DestroyDart _destroy;
  late _FreeDart _free;
  late _OpenDart _open;
  late _CallDart _call;

  static String _defaultLibraryPath() {
    final override = Platform.environment['SS_CORE_DYLIB'];
    if (override != null && override.isNotEmpty) return override;
    if (Platform.isAndroid) return 'libss_core.so';
    if (Platform.isMacOS) return 'libss_core.dylib';
    if (Platform.isWindows) return 'ss_core.dll';
    return 'libss_core.so';
  }

  Map<String, dynamic> _decode(Pointer<Utf8> ptr) {
    if (ptr == nullptr) throw StateError('ss-core returned no data');
    try {
      final answer = jsonDecode(ptr.toDartString()) as Map<String, dynamic>;
      if (answer['ok'] != true) throw StateError('${answer['error']}');
      return Map<String, dynamic>.from(answer['result'] as Map);
    } finally {
      _free(ptr);
    }
  }

  Object? _decodeAny(Pointer<Utf8> ptr) {
    if (ptr == nullptr) throw StateError('ss-core returned no data');
    try {
      final answer = jsonDecode(ptr.toDartString()) as Map<String, dynamic>;
      if (answer['ok'] != true) throw StateError('${answer['error']}');
      return answer['result'];
    } finally {
      _free(ptr);
    }
  }

  Object? _invoke(Map<String, dynamic> request) {
    final input = jsonEncode(request).toNativeUtf8();
    try {
      return _decodeAny(_call(_handle, input));
    } finally {
      malloc.free(input);
    }
  }

  @override
  Map<String, dynamic> newWorkbook() =>
      Map<String, dynamic>.from(_invoke({'op': 'new'}) as Map);

  @override
  Map<String, dynamic> open(String name, Uint8List bytes) {
    final namePtr = name.toNativeUtf8();
    final dataPtr = malloc<Uint8>(bytes.isEmpty ? 1 : bytes.length);
    try {
      if (bytes.isNotEmpty) {
        dataPtr.asTypedList(bytes.length).setAll(0, bytes);
      }
      return _decode(_open(_handle, namePtr, dataPtr, bytes.length));
    } finally {
      malloc.free(namePtr);
      malloc.free(dataPtr);
    }
  }

  @override
  Map<String, dynamic> snapshot({
    int row = 0,
    int col = 0,
    int rows = 80,
    int cols = 30,
  }) => Map<String, dynamic>.from(
    _invoke({
          'op': 'snapshot',
          'row': row,
          'col': col,
          'rows': rows,
          'cols': cols,
          'sheet': 0,
        })
        as Map,
  );

  @override
  List<Map<String, dynamic>> commands() => (_invoke({'op': 'commands'}) as List)
      .map((item) => Map<String, dynamic>.from(item as Map))
      .toList();

  @override
  Uint8List? imageBytes(int id) {
    final result = Map<String, dynamic>.from(
      _invoke({'op': 'imageBytes', 'id': id}) as Map,
    );
    return result['available'] == true ? Uint8List(0) : null;
  }

  @override
  Map<String, dynamic> execute(
    String id, [
    Map<String, dynamic> params = const {},
  ]) {
    final result = _invoke({'op': 'execute', 'id': id, 'params': params});
    return result is Map ? Map<String, dynamic>.from(result) : const {};
  }

  @override
  Uint8List export(String format) {
    final result = Map<String, dynamic>.from(
      _invoke({'op': 'export', 'format': format}) as Map,
    );
    return base64Decode(result['base64'] as String);
  }

  @override
  void dispose() {
    if (_handle != nullptr) {
      _destroy(_handle);
      _handle = nullptr;
    }
  }
}

SpreadsheetEngine engineFor(
  SpreadsheetEngineSelection selection, {
  String? libraryPath,
}) {
  switch (selection.kind) {
    case SpreadsheetEngineKind.gridcraft:
      return GridcraftEngine(libraryPath: libraryPath);
    case SpreadsheetEngineKind.ssCore:
      return SsCoreEngine(libraryPath: libraryPath);
  }
}
