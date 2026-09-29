import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class OfficeViewerException implements Exception {
  const OfficeViewerException(this.message);
  final String message;

  @override
  String toString() => 'OfficeViewerException: $message';
}

final class _NativeViewerBuffer extends Struct {
  external Pointer<Uint8> pointer;

  @Size()
  external int length;

  @Size()
  external int capacity;

  @Int32()
  external int status;
}

typedef _AbiNative = Uint32 Function();
typedef _AbiDart = int Function();
typedef _InspectNative = _NativeViewerBuffer Function(Pointer<Uint8>, Size);
typedef _InspectDart = _NativeViewerBuffer Function(Pointer<Uint8>, int);
typedef _FreeNative = Void Function(_NativeViewerBuffer);
typedef _FreeDart = void Function(_NativeViewerBuffer);

final class OfficeViewersFfiEngine implements OfficeEnginePort {
  OfficeViewersFfiEngine._(DynamicLibrary library)
    : _abiVersion = library.lookupFunction<_AbiNative, _AbiDart>(
        'openmuse_office_viewers_abi_version',
      ),
      _inspectXlsx = library.lookupFunction<_InspectNative, _InspectDart>(
        'openmuse_xlsx_inspect',
      ),
      _inspectPptx = library.lookupFunction<_InspectNative, _InspectDart>(
        'openmuse_pptx_inspect',
      ),
      _free = library.lookupFunction<_FreeNative, _FreeDart>(
        'openmuse_office_viewer_buffer_free',
      ) {
    if (_abiVersion() != 1) {
      throw const OfficeViewerException('unsupported Office viewers ABI');
    }
  }

  factory OfficeViewersFfiEngine.open({String? libraryPath}) {
    final library = libraryPath == null
        ? (Platform.isIOS
              ? DynamicLibrary.process()
              : DynamicLibrary.open(_platformLibraryName()))
        : DynamicLibrary.open(libraryPath);
    return OfficeViewersFfiEngine._(library);
  }

  final _AbiDart _abiVersion;
  final _InspectDart _inspectXlsx;
  final _InspectDart _inspectPptx;
  final _FreeDart _free;

  @override
  String get abi => 'openmuse-office-viewers-ffi@${_abiVersion()}';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async {
    final (operation, schema) = switch (format) {
      OfficeFormat.sheet => (_inspectXlsx, 'openmuse.office.xlsx-inspection@1'),
      OfficeFormat.slides => (
        _inspectPptx,
        'openmuse.office.pptx-inspection@1',
      ),
      _ => throw const OfficeViewerException(
        'viewer format is not implemented',
      ),
    };
    final input = _allocate(bytes);
    try {
      final payload = _copyAndFree(operation(input.pointer, input.length));
      final decoded = jsonDecode(utf8.decode(payload));
      if (decoded is! Map<String, dynamic> ||
          decoded['schema'] != schema ||
          decoded['profile'] != 'view-only' ||
          decoded['paragraphs'] is! List ||
          decoded['capabilities'] is! List) {
        throw const OfficeViewerException('invalid XLSX inspection envelope');
      }
      final paragraphs = decoded['paragraphs'] as List;
      final capabilities = decoded['capabilities'] as List;
      if (paragraphs.any((value) => value is! String) ||
          capabilities.length != 1 ||
          capabilities.single != 'view') {
        throw const OfficeViewerException('invalid XLSX capability payload');
      }
      return OfficeEngineInspection(
        format: format,
        profile: 'view-only',
        paragraphs: paragraphs.cast<String>().toList(growable: false),
        capabilities: const {OfficeCapability.view},
      );
    } finally {
      malloc.free(input.pointer);
    }
  }

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw const OfficeViewerException(
    'original-format export is unavailable for view engines',
  );

  Uint8List _copyAndFree(_NativeViewerBuffer result) {
    try {
      final bytes = Uint8List.fromList(
        result.pointer.asTypedList(result.length),
      );
      if (result.status != 0) {
        throw OfficeViewerException(utf8.decode(bytes, allowMalformed: true));
      }
      return bytes;
    } finally {
      _free(result);
    }
  }

  static _AllocatedBytes _allocate(List<int> bytes) {
    if (bytes.isEmpty || bytes.any((value) => value < 0 || value > 255)) {
      throw const OfficeViewerException('invalid Office input');
    }
    final pointer = malloc<Uint8>(bytes.length);
    pointer.asTypedList(bytes.length).setAll(0, bytes);
    return _AllocatedBytes(pointer, bytes.length);
  }

  static String _platformLibraryName() {
    if (Platform.isAndroid) return 'libopenmuse_office_viewers.so';
    if (Platform.isMacOS) return 'libopenmuse_office_viewers.dylib';
    if (Platform.isLinux) return 'libopenmuse_office_viewers.so';
    if (Platform.isWindows) return 'openmuse_office_viewers.dll';
    throw UnsupportedError('Office viewers FFI is unavailable');
  }
}

final class _AllocatedBytes {
  const _AllocatedBytes(this.pointer, this.length);
  final Pointer<Uint8> pointer;
  final int length;
}
