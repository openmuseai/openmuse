import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

/// Hosts choose the engine before the shell opens a workbook.
/// The product default stays Gridcraft. `ssCore` must not be served by
/// [GridcraftEngine]; the shell refuses that pair.
enum SpreadsheetEngineKind { gridcraft, ssCore }

class SpreadsheetEngineSelection {
  const SpreadsheetEngineSelection.gridcraft()
    : kind = SpreadsheetEngineKind.gridcraft;

  const SpreadsheetEngineSelection.ssCore() : kind = SpreadsheetEngineKind.ssCore;

  final SpreadsheetEngineKind kind;

  String get engineId =>
      kind == SpreadsheetEngineKind.ssCore ? 'ss-core' : 'gridcraft';
}

/// The Flutter surface only knows this versioned session interface. A later
/// ss-core adapter can implement it without changing product resources.
abstract class SpreadsheetEngine {
  Map<String, dynamic> newWorkbook();
  Map<String, dynamic> open(String name, Uint8List bytes);
  Map<String, dynamic> snapshot({
    int row = 0,
    int col = 0,
    int rows = 80,
    int cols = 30,
  });
  List<Map<String, dynamic>> commands();
  Uint8List? imageBytes(int id);
  Map<String, dynamic> execute(
    String id, [
    Map<String, dynamic> params = const {},
  ]);
  Uint8List export(String format);
  void dispose();
}

typedef _CreateNative = Pointer<Void> Function();
typedef _CreateDart = Pointer<Void> Function();
typedef _DestroyNative = Void Function(Pointer<Void>);
typedef _DestroyDart = void Function(Pointer<Void>);
typedef _AbiNative = Uint32 Function();
typedef _AbiDart = int Function();
typedef _OpenNative =
    Pointer<Utf8> Function(
      Pointer<Void>,
      Pointer<Utf8>,
      Pointer<Uint8>,
      IntPtr,
    );
typedef _OpenDart =
    Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>, Pointer<Uint8>, int);
typedef _CallNative = Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>);
typedef _CallDart = Pointer<Utf8> Function(Pointer<Void>, Pointer<Utf8>);
typedef _FreeNative = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

final class GridcraftEngine implements SpreadsheetEngine {
  GridcraftEngine({String? libraryPath}) {
    final dylib = DynamicLibrary.open(libraryPath ?? _defaultLibraryPath());
    final abi = dylib.lookupFunction<_AbiNative, _AbiDart>('om_sheet_abi');
    if (abi() != 1) throw StateError('Incompatible spreadsheet engine ABI');
    _destroy = dylib.lookupFunction<_DestroyNative, _DestroyDart>(
      'om_sheet_destroy',
    );
    _free = dylib.lookupFunction<_FreeNative, _FreeDart>(
      'om_sheet_string_free',
    );
    _open = dylib.lookupFunction<_OpenNative, _OpenDart>('om_sheet_open');
    _call = dylib.lookupFunction<_CallNative, _CallDart>('om_sheet_call');
    _handle = dylib.lookupFunction<_CreateNative, _CreateDart>(
      'om_sheet_create',
    )();
    if (_handle == nullptr)
      throw StateError('Cannot create spreadsheet session');
  }

  late Pointer<Void> _handle;
  late _DestroyDart _destroy;
  late _FreeDart _free;
  late _OpenDart _open;
  late _CallDart _call;

  static String _defaultLibraryPath() {
    final override = Platform.environment['OPENMUSE_GRIDCRAFT_DYLIB'];
    if (override != null && override.isNotEmpty) return override;
    if (Platform.isAndroid) return 'libopenmuse_gridcraft_bridge.so';
    if (Platform.isMacOS) {
      final executable = File(Platform.resolvedExecutable);
      return '${executable.parent.parent.path}/Frameworks/libopenmuse_gridcraft_bridge.dylib';
    }
    if (Platform.isWindows) return 'openmuse_gridcraft_bridge.dll';
    return 'libopenmuse_gridcraft_bridge.so';
  }

  Map<String, dynamic> _decode(Pointer<Utf8> ptr) {
    if (ptr == nullptr) throw StateError('Spreadsheet engine returned no data');
    try {
      final answer = jsonDecode(ptr.toDartString()) as Map<String, dynamic>;
      if (answer['ok'] != true) throw StateError('${answer['error']}');
      return Map<String, dynamic>.from(answer['result'] as Map);
    } finally {
      _free(ptr);
    }
  }

  Object? _decodeAny(Pointer<Utf8> ptr) {
    if (ptr == nullptr) throw StateError('Spreadsheet engine returned no data');
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
      dataPtr.asTypedList(bytes.length).setAll(0, bytes);
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
        })
        as Map,
  );

  @override
  List<Map<String, dynamic>> commands() => (_invoke({'op': 'commands'}) as List)
      .map((x) => Map<String, dynamic>.from(x as Map))
      .toList();

  @override
  Uint8List? imageBytes(int id) {
    final result = Map<String, dynamic>.from(
      _invoke({'op': 'imageBytes', 'id': id}) as Map,
    );
    final raw = result['base64'];
    return result['available'] == true && raw is String
        ? base64Decode(raw)
        : null;
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
