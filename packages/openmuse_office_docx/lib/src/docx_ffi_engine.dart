import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class DocxEngineException implements Exception {
  const DocxEngineException(this.message);
  final String message;

  @override
  String toString() => 'DocxEngineException: $message';
}

final class _NativeDocxBuffer extends Struct {
  external Pointer<Uint8> pointer;

  @Size()
  external int length;

  @Size()
  external int capacity;

  @Int32()
  external int status;
}

typedef _AbiVersionNative = Uint32 Function();
typedef _AbiVersionDart = int Function();
typedef _InspectNative = _NativeDocxBuffer Function(Pointer<Uint8>, Size);
typedef _InspectDart = _NativeDocxBuffer Function(Pointer<Uint8>, int);
typedef _ExportNative =
    _NativeDocxBuffer Function(Pointer<Uint8>, Size, Pointer<Uint8>, Size);
typedef _ExportDart =
    _NativeDocxBuffer Function(Pointer<Uint8>, int, Pointer<Uint8>, int);
typedef _FreeNative = Void Function(_NativeDocxBuffer);
typedef _FreeDart = void Function(_NativeDocxBuffer);

/// Loads only the versioned DOCX ABI. It has no Workspace, network, account,
/// DSH, filesystem, or credential handle and can operate only on caller-owned
/// byte buffers.
final class DocxFfiEngine implements OfficeEnginePort {
  DocxFfiEngine._(DynamicLibrary library)
    : _abiVersion = library.lookupFunction<_AbiVersionNative, _AbiVersionDart>(
        'openmuse_docx_abi_version',
      ),
      _inspect = library.lookupFunction<_InspectNative, _InspectDart>(
        'openmuse_docx_inspect',
      ),
      _export = library.lookupFunction<_ExportNative, _ExportDart>(
        'openmuse_docx_export_simple',
      ),
      _free = library.lookupFunction<_FreeNative, _FreeDart>(
        'openmuse_docx_buffer_free',
      ) {
    if (_abiVersion() != 1) {
      throw const DocxEngineException('unsupported DOCX native ABI');
    }
  }

  factory DocxFfiEngine.open({String? libraryPath}) {
    final library = libraryPath == null
        ? (Platform.isIOS
              ? DynamicLibrary.process()
              : DynamicLibrary.open(_platformLibraryName()))
        : DynamicLibrary.open(libraryPath);
    return DocxFfiEngine._(library);
  }

  static String _platformLibraryName() {
    if (Platform.isAndroid) return 'libopenmuse_office_docx.so';
    if (Platform.isMacOS) return 'libopenmuse_office_docx.dylib';
    if (Platform.isLinux) return 'libopenmuse_office_docx.so';
    if (Platform.isWindows) return 'openmuse_office_docx.dll';
    throw UnsupportedError('DOCX FFI is not packaged for this platform');
  }

  final _AbiVersionDart _abiVersion;
  final _InspectDart _inspect;
  final _ExportDart _export;
  final _FreeDart _free;

  @override
  String get abi => 'openmuse-docx-ffi@${_abiVersion()}';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat format,
    List<int> bytes,
  ) async {
    _requireWord(format);
    final payload = _callUnary(_inspect, bytes);
    final decoded = jsonDecode(utf8.decode(payload));
    if (decoded is! Map<String, dynamic> ||
        decoded['schema'] != 'openmuse.office.docx-inspection@1') {
      throw const DocxEngineException('invalid inspection envelope');
    }
    final profile = decoded['profile'];
    final paragraphs = decoded['paragraphs'];
    final capabilities = decoded['capabilities'];
    if (profile is! String ||
        paragraphs is! List ||
        capabilities is! List ||
        paragraphs.any((value) => value is! String) ||
        capabilities.any((value) => value is! String)) {
      throw const DocxEngineException('invalid inspection payload');
    }
    return OfficeEngineInspection(
      format: OfficeFormat.word,
      profile: profile,
      paragraphs: paragraphs.cast<String>().toList(growable: false),
      capabilities: capabilities.cast<String>().map(_capability).toSet(),
    );
  }

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) async {
    _requireWord(format);
    final document = _allocate(originalBytes);
    final replacement = _allocate(utf8.encode(jsonEncode(paragraphs)));
    try {
      return _copyAndFree(
        _export(
          document.pointer,
          document.length,
          replacement.pointer,
          replacement.length,
        ),
      );
    } finally {
      malloc.free(document.pointer);
      malloc.free(replacement.pointer);
    }
  }

  List<int> _callUnary(_InspectDart operation, List<int> bytes) {
    final input = _allocate(bytes);
    try {
      return _copyAndFree(operation(input.pointer, input.length));
    } finally {
      malloc.free(input.pointer);
    }
  }

  List<int> _copyAndFree(_NativeDocxBuffer result) {
    try {
      final bytes = Uint8List.fromList(
        result.pointer.asTypedList(result.length),
      );
      if (result.status != 0) {
        throw DocxEngineException(utf8.decode(bytes, allowMalformed: true));
      }
      return bytes;
    } finally {
      _free(result);
    }
  }

  static _AllocatedBytes _allocate(List<int> bytes) {
    if (bytes.isEmpty) throw const DocxEngineException('empty input');
    final pointer = malloc<Uint8>(bytes.length);
    pointer.asTypedList(bytes.length).setAll(0, bytes);
    return _AllocatedBytes(pointer, bytes.length);
  }

  static void _requireWord(OfficeFormat format) {
    if (format != OfficeFormat.word) {
      throw const DocxEngineException('DOCX engine only accepts Word');
    }
  }

  static OfficeCapability _capability(String value) => switch (value) {
    'view' => OfficeCapability.view,
    'edit' => OfficeCapability.edit,
    'export' => OfficeCapability.export,
    _ => throw DocxEngineException('unknown capability: $value'),
  };
}

final class _AllocatedBytes {
  const _AllocatedBytes(this.pointer, this.length);
  final Pointer<Uint8> pointer;
  final int length;
}
