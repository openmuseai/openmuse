import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'pdf_page_operations.dart';
import 'pdf_page_operations_factory_stub.dart';
import 'pdf_native_bytes.dart';

PdfPageOperations createPdfPageOperations() =>
    Platform.isMacOS ||
        Platform.isAndroid ||
        (Platform.isIOS && _iosProductLibrary().existsSync())
    ? NativePdfPageOperations()
    : const UnavailablePdfPageOperations();

File _iosProductLibrary() => File(
  '${File(Platform.resolvedExecutable).parent.path}/Frameworks/libksp_product.dylib',
);

final class _NativePageSelection extends Struct {
  external Pointer<Uint8> pdf;
  @Uint64()
  external int pdfSize;
  external Pointer<Int32> pageIndices;
  @Uint32()
  external int pageCount;
}

class NativePdfPageOperations implements PdfPageOperations {
  DynamicLibrary? _library;
  late int Function() _version;
  late int Function(Pointer<Uint8>, int) _pageCount;
  late int Function(
    Pointer<_NativePageSelection>,
    int,
    Pointer<PdfNativeBytes>,
    Pointer<Utf8>,
    int,
  )
  _compose;
  late void Function(Pointer<PdfNativeBytes>) _freeBytes;

  @override
  bool get isAvailable => true;

  @override
  String get unavailableReason => '';

  void _bind() {
    if (_library != null) return;
    final DynamicLibrary library;
    if (Platform.isAndroid) {
      library = DynamicLibrary.open('libkspfreetext.so');
    } else if (Platform.isIOS) {
      library = DynamicLibrary.open(_iosProductLibrary().path);
    } else {
      final overridePath = Platform.environment['KSP_MAC_EDIT_DYLIB'];
      final contents = File(Platform.resolvedExecutable).parent.parent;
      final assetRoot = '${contents.path}/Frameworks/App.framework/Resources/flutter_assets';
      final candidates = [
        ?overridePath,
        '$assetRoot/packages/pdf_rebuild_flutter/assets/native/libkspfreetext.dylib',
        '$assetRoot/assets/native/libkspfreetext.dylib',
      ];
      final dylib = File(candidates.firstWhere(
        (path) => File(path).existsSync(),
        orElse: () => candidates.first,
      ));
      if (!dylib.existsSync()) throw StateError('产品内核不存在：${dylib.path}');
      library = DynamicLibrary.open(dylib.path);
    }
    _version = library.lookupFunction<Uint32 Function(), int Function()>(
      'ksp_product_abi_version',
    );
    _pageCount = library
        .lookupFunction<
          Int32 Function(Pointer<Uint8>, Uint64),
          int Function(Pointer<Uint8>, int)
        >('ksp_product_page_count');
    _compose = library
        .lookupFunction<
          Int32 Function(
            Pointer<_NativePageSelection>,
            Uint32,
            Pointer<PdfNativeBytes>,
            Pointer<Utf8>,
            Uint32,
          ),
          int Function(
            Pointer<_NativePageSelection>,
            int,
            Pointer<PdfNativeBytes>,
            Pointer<Utf8>,
            int,
          )
        >('ksp_product_compose_pages');
    _freeBytes = library
        .lookupFunction<
          Void Function(Pointer<PdfNativeBytes>),
          void Function(Pointer<PdfNativeBytes>)
        >('ksp_product_free_bytes');
    if (_version() < 2) throw StateError('产品内核 ABI 版本过旧');
    _library = library;
  }

  @override
  Future<int> pageCount(Uint8List bytes) async {
    _bind();
    if (bytes.isEmpty) throw ArgumentError('PDF 字节为空');
    final input = calloc<Uint8>(bytes.length);
    try {
      input.asTypedList(bytes.length).setAll(0, bytes);
      final count = _pageCount(input, bytes.length);
      if (count < 0) throw StateError('无法读取 PDF 页数，native error=$count');
      return count;
    } finally {
      calloc.free(input);
    }
  }

  @override
  Future<Uint8List> compose(List<PdfPageSelection> sources) async {
    _bind();
    if (sources.isEmpty) throw ArgumentError('没有待组合的 PDF');
    final nativeSources = calloc<_NativePageSelection>(sources.length);
    final bytes = <Pointer<Uint8>>[];
    final indices = <Pointer<Int32>>[];
    final output = calloc<PdfNativeBytes>();
    final error = calloc<Uint8>(256);
    try {
      for (var index = 0; index < sources.length; index++) {
        final source = sources[index];
        if (source.bytes.isEmpty || source.pageIndices.isEmpty) {
          throw ArgumentError('PDF 字节或页序为空');
        }
        final input = calloc<Uint8>(source.bytes.length);
        bytes.add(input);
        input.asTypedList(source.bytes.length).setAll(0, source.bytes);
        final pages = calloc<Int32>(source.pageIndices.length);
        indices.add(pages);
        pages
            .asTypedList(source.pageIndices.length)
            .setAll(0, source.pageIndices);
        nativeSources[index]
          ..pdf = input
          ..pdfSize = source.bytes.length
          ..pageIndices = pages
          ..pageCount = source.pageIndices.length;
      }
      final status = _compose(
        nativeSources,
        sources.length,
        output,
        error.cast<Utf8>(),
        256,
      );
      if (status != 0) {
        final detail = error.cast<Utf8>().toDartString();
        throw StateError('页面组合失败：$detail (native error=$status)');
      }
      return Uint8List.fromList(output.ref.data.asTypedList(output.ref.size));
    } finally {
      _freeBytes(output);
      calloc.free(output);
      calloc.free(error);
      for (final input in bytes) {
        calloc.free(input);
      }
      for (final pages in indices) {
        calloc.free(pages);
      }
      calloc.free(nativeSources);
    }
  }
}
