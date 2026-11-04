import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'pdf_edit_backend.dart';
import 'pdf_edit_backend_factory_stub.dart';
import 'pdf_native_bytes.dart';

PdfEditBackend createPdfEditBackend() =>
    Platform.isMacOS ||
        Platform.isAndroid ||
        (Platform.isIOS && _iosProductLibrary().existsSync())
    ? NativePdfEditBackend()
    : const ReadOnlyPdfEditBackend();
typedef _Session = Pointer<Void>;

final class _NativeTextChar extends Struct {
  @Uint32()
  external int utf16;

  @Float()
  external double left;

  @Float()
  external double bottom;

  @Float()
  external double right;

  @Float()
  external double top;
}

final class _NativePageText extends Struct {
  external Pointer<_NativeTextChar> chars;

  @Int32()
  external int count;
}

final class _NativePageLink extends Struct {
  @Float()
  external double left;
  @Float()
  external double bottom;
  @Float()
  external double right;
  @Float()
  external double top;
  @Int32()
  external int kind;
  @Int32()
  external int pageIndex;
  @Uint32()
  external int uriOffset;
  @Uint32()
  external int uriLength;
}

final class _NativePageLinks extends Struct {
  external Pointer<_NativePageLink> links;
  @Int32()
  external int count;
  external Pointer<Uint8> uriData;
  @Uint32()
  external int uriSize;
}

File _iosProductLibrary() => File(
  '${File(Platform.resolvedExecutable).parent.path}/Frameworks/libksp_product.dylib',
);

class NativePdfEditBackend implements PdfEditBackend {
  DynamicLibrary? _library;
  _Session _session = nullptr;
  late Pointer<Void> Function(Pointer<Utf8>, Pointer<Int32>) _open;
  late Pointer<Void> Function(Pointer<Uint8>, int, Pointer<Int32>) _openBytes;
  late void Function(_Session) _close;
  late int Function(_Session, int) _loadPage;
  late int Function(_Session) _domainCount;
  late int Function(
    _Session,
    int,
    Pointer<Float>,
    Pointer<Float>,
    Pointer<Float>,
    Pointer<Float>,
  )
  _domainRect;
  late int Function(_Session, int, Pointer<Uint16>, int) _domainText;
  late int Function(_Session, int, Pointer<Uint16>, int) _replaceDomain;
  late int Function(_Session, double, double) _focusAtNative;
  late int Function(_Session, Pointer<Uint16>, int) _focusedText;
  late int Function(_Session, Pointer<Int32>, Pointer<Int32>) _selection;
  late int Function(_Session, int, int) _setSelection;
  late int Function(
    _Session,
    Pointer<Float>,
    Pointer<Float>,
    Pointer<Float>,
    Pointer<Float>,
  )
  _caret;
  late int Function(_Session, Pointer<Uint16>, int) _replaceSelection;
  late int Function(_Session, int, int, Pointer<Uint8>, int) _renderPage;
  int Function(_Session, int, Pointer<Float>, Pointer<Float>)? _pageSize;
  int Function(_Session, int, int, int, Pointer<Uint8>, int)? _renderPageAt;
  int Function(
    _Session,
    int,
    int,
    int,
    int,
    int,
    int,
    int,
    Pointer<Uint8>,
    int,
  )?
  _renderPageRegion;
  int Function(_Session, int, Pointer<_NativePageText>)? _readPageText;
  void Function(Pointer<_NativePageText>)? _freePageText;
  int Function(_Session, int, Pointer<_NativePageLinks>)? _readPageLinks;
  void Function(Pointer<_NativePageLinks>)? _freePageLinks;
  late int Function(_Session) _undoNative,
      _redoNative,
      _canUndoNative,
      _canRedoNative;
  late int Function(_Session, Pointer<Utf8>) _save;
  int Function(_Session, Pointer<PdfNativeBytes>)? _exportBytes;
  void Function(Pointer<PdfNativeBytes>)? _freeBytes;

  @override
  bool get supportsMutation => true;
  @override
  String get unavailableReason => '';
  @override
  bool get isOpen => _session != nullptr;
  @override
  bool get canUndo => isOpen && _canUndoNative(_session) != 0;
  @override
  bool get canRedo => isOpen && _canRedoNative(_session) != 0;

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
      if (!dylib.existsSync()) {
        throw StateError('native 编辑库不存在：${dylib.path}');
      }
      library = DynamicLibrary.open(dylib.path);
    }
    final editAbiVersion = library
        .lookupFunction<Uint32 Function(), int Function()>(
          'ksp_product_edit_abi_version',
        )();
    if (editAbiVersion < 1 || editAbiVersion > 6) {
      throw StateError('不兼容的产品编辑 ABI 版本：$editAbiVersion');
    }
    _open = library
        .lookupFunction<
          Pointer<Void> Function(Pointer<Utf8>, Pointer<Int32>),
          Pointer<Void> Function(Pointer<Utf8>, Pointer<Int32>)
        >('ksp_product_edit_open');
    _openBytes = library
        .lookupFunction<
          Pointer<Void> Function(Pointer<Uint8>, Uint64, Pointer<Int32>),
          Pointer<Void> Function(Pointer<Uint8>, int, Pointer<Int32>)
        >('ksp_product_edit_open_bytes');
    _close = library
        .lookupFunction<
          Void Function(Pointer<Void>),
          void Function(Pointer<Void>)
        >('ksp_product_edit_close');
    _loadPage = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Int32),
          int Function(Pointer<Void>, int)
        >('ksp_product_edit_load_page');
    _domainCount = library
        .lookupFunction<
          Int32 Function(Pointer<Void>),
          int Function(Pointer<Void>)
        >('ksp_product_edit_domain_count');
    _domainRect = library
        .lookupFunction<
          Int32 Function(
            Pointer<Void>,
            Int32,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
          ),
          int Function(
            Pointer<Void>,
            int,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
          )
        >('ksp_product_edit_domain_rect');
    _domainText = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Int32, Pointer<Uint16>, Int32),
          int Function(Pointer<Void>, int, Pointer<Uint16>, int)
        >('ksp_product_edit_domain_text');
    _replaceDomain = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Int32, Pointer<Uint16>, Int32),
          int Function(Pointer<Void>, int, Pointer<Uint16>, int)
        >('ksp_product_edit_replace_domain');
    _focusAtNative = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Float, Float),
          int Function(Pointer<Void>, double, double)
        >('ksp_product_edit_focus_at');
    _focusedText = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Uint16>, Int32),
          int Function(Pointer<Void>, Pointer<Uint16>, int)
        >('ksp_product_edit_focused_text');
    _selection = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Int32>, Pointer<Int32>),
          int Function(Pointer<Void>, Pointer<Int32>, Pointer<Int32>)
        >('ksp_product_edit_selection');
    _setSelection = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Int32, Int32),
          int Function(Pointer<Void>, int, int)
        >('ksp_product_edit_set_selection');
    _caret = library
        .lookupFunction<
          Int32 Function(
            Pointer<Void>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
          ),
          int Function(
            Pointer<Void>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
            Pointer<Float>,
          )
        >('ksp_product_edit_caret');
    _replaceSelection = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Uint16>, Int32),
          int Function(Pointer<Void>, Pointer<Uint16>, int)
        >('ksp_product_edit_replace_selection');
    _renderPage = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Int32, Int32, Pointer<Uint8>, Int32),
          int Function(Pointer<Void>, int, int, Pointer<Uint8>, int)
        >('ksp_product_edit_render_page');
    if (editAbiVersion >= 3) {
      _pageSize = library
          .lookupFunction<
            Int32 Function(
              Pointer<Void>,
              Int32,
              Pointer<Float>,
              Pointer<Float>,
            ),
            int Function(_Session, int, Pointer<Float>, Pointer<Float>)
          >('ksp_product_edit_page_size');
      _renderPageAt = library
          .lookupFunction<
            Int32 Function(
              Pointer<Void>,
              Int32,
              Int32,
              Int32,
              Pointer<Uint8>,
              Int32,
            ),
            int Function(_Session, int, int, int, Pointer<Uint8>, int)
          >('ksp_product_edit_render_page_at');
    }
    if (editAbiVersion >= 4) {
      _renderPageRegion = library
          .lookupFunction<
            Int32 Function(
              Pointer<Void>,
              Int32,
              Int32,
              Int32,
              Int32,
              Int32,
              Int32,
              Int32,
              Pointer<Uint8>,
              Int32,
            ),
            int Function(
              _Session,
              int,
              int,
              int,
              int,
              int,
              int,
              int,
              Pointer<Uint8>,
              int,
            )
          >('ksp_product_edit_render_page_region');
    }
    if (editAbiVersion >= 5) {
      _readPageText = library
          .lookupFunction<
            Int32 Function(Pointer<Void>, Int32, Pointer<_NativePageText>),
            int Function(_Session, int, Pointer<_NativePageText>)
          >('ksp_product_edit_read_page_text');
      _freePageText = library
          .lookupFunction<
            Void Function(Pointer<_NativePageText>),
            void Function(Pointer<_NativePageText>)
          >('ksp_product_edit_free_page_text');
    }
    if (editAbiVersion >= 6) {
      _readPageLinks = library
          .lookupFunction<
            Int32 Function(Pointer<Void>, Int32, Pointer<_NativePageLinks>),
            int Function(_Session, int, Pointer<_NativePageLinks>)
          >('ksp_product_edit_read_page_links');
      _freePageLinks = library
          .lookupFunction<
            Void Function(Pointer<_NativePageLinks>),
            void Function(Pointer<_NativePageLinks>)
          >('ksp_product_edit_free_page_links');
    }
    _undoNative = library
        .lookupFunction<
          Int32 Function(Pointer<Void>),
          int Function(Pointer<Void>)
        >('ksp_product_edit_undo');
    _redoNative = library
        .lookupFunction<
          Int32 Function(Pointer<Void>),
          int Function(Pointer<Void>)
        >('ksp_product_edit_redo');
    _canUndoNative = library
        .lookupFunction<
          Int32 Function(Pointer<Void>),
          int Function(Pointer<Void>)
        >('ksp_product_edit_can_undo');
    _canRedoNative = library
        .lookupFunction<
          Int32 Function(Pointer<Void>),
          int Function(Pointer<Void>)
        >('ksp_product_edit_can_redo');
    _save = library
        .lookupFunction<
          Int32 Function(Pointer<Void>, Pointer<Utf8>),
          int Function(Pointer<Void>, Pointer<Utf8>)
        >('ksp_product_edit_save');
    if (editAbiVersion >= 2) {
      _exportBytes = library
          .lookupFunction<
            Int32 Function(Pointer<Void>, Pointer<PdfNativeBytes>),
            int Function(Pointer<Void>, Pointer<PdfNativeBytes>)
          >('ksp_product_edit_export_bytes');
      _freeBytes = library
          .lookupFunction<
            Void Function(Pointer<PdfNativeBytes>),
            void Function(Pointer<PdfNativeBytes>)
          >('ksp_product_free_bytes');
    }
    _library = library;
  }

  void _check(int result, String operation) {
    if (result < 0) throw StateError('$operation 失败，native error=$result');
  }

  @override
  Future<void> openPath(String path) async {
    _bind();
    final nativePath = path.toNativeUtf8();
    final error = calloc<Int32>();
    try {
      final next = _open(nativePath, error);
      if (next == nullptr) {
        _check(error.value, '打开编辑文档');
        throw StateError('打开编辑文档失败，native 未返回会话');
      }
      final previous = _session;
      _session = next;
      if (previous != nullptr) _close(previous);
    } finally {
      calloc.free(nativePath);
      calloc.free(error);
    }
  }

  @override
  Future<void> openBytes(Uint8List bytes) async {
    _bind();
    if (bytes.isEmpty) throw ArgumentError('PDF 字节为空');
    final input = calloc<Uint8>(bytes.length);
    final error = calloc<Int32>();
    try {
      input.asTypedList(bytes.length).setAll(0, bytes);
      final next = _openBytes(input, bytes.length, error);
      if (next == nullptr) {
        _check(error.value, '打开编辑文档');
        throw StateError('打开编辑文档失败，native 未返回会话');
      }
      final previous = _session;
      _session = next;
      if (previous != nullptr) _close(previous);
    } finally {
      calloc.free(input);
      calloc.free(error);
    }
  }

  @override
  Future<List<PdfEditDomain>> loadPage(int zeroBasedPageIndex) async {
    _check(_loadPage(_session, zeroBasedPageIndex), '识别编辑域');
    return _readDomains();
  }

  @override
  Future<List<PdfEditDomain>> refreshDomains() async => _readDomains();

  List<PdfEditDomain> _readDomains() {
    final count = _domainCount(_session);
    _check(count, '读取编辑域数量');
    final values = <PdfEditDomain>[];
    final left = calloc<Float>(),
        bottom = calloc<Float>(),
        right = calloc<Float>(),
        top = calloc<Float>();
    try {
      for (var index = 0; index < count; index++) {
        _check(_domainRect(_session, index, left, bottom, right, top), '读取编辑域');
        values.add(
          PdfEditDomain(
            index,
            left.value,
            bottom.value,
            right.value,
            top.value,
          ),
        );
      }
    } finally {
      calloc.free(left);
      calloc.free(bottom);
      calloc.free(right);
      calloc.free(top);
    }
    return values;
  }

  @override
  Future<String> domainText(int domainIndex) async {
    final length = _domainText(_session, domainIndex, nullptr, 0);
    _check(length, '读取编辑域文字');
    if (length == 0) return '';
    final output = calloc<Uint16>(length);
    try {
      final copied = _domainText(_session, domainIndex, output, length);
      _check(copied, '读取编辑域文字');
      return String.fromCharCodes(output.asTypedList(copied));
    } finally {
      calloc.free(output);
    }
  }

  @override
  Future<void> replaceDomain(int domainIndex, String text) async {
    final units = Uint16List.fromList(text.codeUnits);
    final input = units.isEmpty ? nullptr : calloc<Uint16>(units.length);
    try {
      if (input != nullptr) input.asTypedList(units.length).setAll(0, units);
      _check(
        _replaceDomain(_session, domainIndex, input, units.length),
        '替换编辑域文字',
      );
    } finally {
      if (input != nullptr) calloc.free(input);
    }
  }

  String _readFocusedText() {
    final length = _focusedText(_session, nullptr, 0);
    _check(length, '读取焦点文字');
    if (length == 0) return '';
    final output = calloc<Uint16>(length);
    try {
      final copied = _focusedText(_session, output, length);
      _check(copied, '读取焦点文字');
      return String.fromCharCodes(output.asTypedList(copied));
    } finally {
      calloc.free(output);
    }
  }

  PdfEditFocus _readFocus() {
    final start = calloc<Int32>(), end = calloc<Int32>();
    final bottomX = calloc<Float>(),
        bottomY = calloc<Float>(),
        topX = calloc<Float>(),
        topY = calloc<Float>();
    try {
      final text = _readFocusedText();
      _check(_selection(_session, start, end), '读取光标选区');
      // Android treats GetCaretPoints as decoration after OnChars. Newline and
      // empty-run boundaries can legitimately have no glyph geometry while the
      // selection and mutation are already valid, so keep the edit successful.
      final caretResult = _caret(_session, bottomX, bottomY, topX, topY);
      return PdfEditFocus(
        text,
        start.value,
        end.value,
        caretResult >= 0
            ? PdfEditCaret(bottomX.value, bottomY.value, topX.value, topY.value)
            : null,
      );
    } finally {
      calloc.free(start);
      calloc.free(end);
      calloc.free(bottomX);
      calloc.free(bottomY);
      calloc.free(topX);
      calloc.free(topY);
    }
  }

  @override
  Future<PdfEditFocus> focusAt(double pageX, double pageY) async {
    final focused = _focusAtNative(_session, pageX, pageY);
    _check(focused, '定位编辑光标');
    if (focused == 0) throw StateError('点击位置没有可编辑文字');
    return _readFocus();
  }

  @override
  Future<PdfEditFocus> setSelection(int start, int end) async {
    _check(_setSelection(_session, start, end), '移动编辑光标');
    return _readFocus();
  }

  @override
  Future<PdfEditFocus> replaceRange(int start, int end, String text) async {
    _check(_setSelection(_session, start, end), '设置编辑选区');
    final units = Uint16List.fromList(text.codeUnits);
    final input = units.isEmpty ? nullptr : calloc<Uint16>(units.length);
    try {
      if (input != nullptr) input.asTypedList(units.length).setAll(0, units);
      _check(_replaceSelection(_session, input, units.length), '原地替换选区文字');
    } finally {
      if (input != nullptr) calloc.free(input);
    }
    return _readFocus();
  }

  @override
  Future<PdfEditRaster> renderPage(int width, int height) async {
    final capacity = _rasterCapacity(width, height);
    final output = calloc<Uint8>(capacity);
    try {
      final written = _renderPage(_session, width, height, output, capacity);
      _check(written, '渲染编辑页');
      if (written != capacity) {
        throw StateError('渲染编辑页长度异常：$written/$capacity');
      }
      return PdfEditRaster(
        width,
        height,
        Uint8List.fromList(output.asTypedList(capacity)),
      );
    } finally {
      calloc.free(output);
    }
  }

  int _rasterCapacity(int width, int height) {
    if (width <= 0 || height <= 0 || width * height > 16 * 1024 * 1024) {
      throw ArgumentError('渲染尺寸无效或超出限制');
    }
    return width * height * 4;
  }

  @override
  Future<PdfPageSize> pageSize(int zeroBasedPageIndex) async {
    _bind();
    final readSize = _pageSize;
    if (readSize == null) throw UnsupportedError('产品编辑 ABI 不支持页面尺寸读取');
    final width = calloc<Float>();
    final height = calloc<Float>();
    try {
      _check(readSize(_session, zeroBasedPageIndex, width, height), '读取页面尺寸');
      return PdfPageSize(width.value, height.value);
    } finally {
      calloc.free(width);
      calloc.free(height);
    }
  }

  @override
  Future<PdfPageTextData> readPageText(int zeroBasedPageIndex) async {
    _bind();
    final read = _readPageText;
    final release = _freePageText;
    if (read == null || release == null) {
      throw UnsupportedError('产品编辑 ABI 不支持页面文字读取');
    }
    final output = calloc<_NativePageText>();
    try {
      _check(read(_session, zeroBasedPageIndex, output), '读取页面文字');
      final count = output.ref.count;
      if (count < 0 ||
          count > 1000000 ||
          (count > 0 && output.ref.chars == nullptr)) {
        throw StateError('页面文字结果无效');
      }
      final chars = List<PdfPageTextCharacter>.generate(count, (index) {
        final native = output.ref.chars[index];
        return PdfPageTextCharacter(
          native.utf16,
          native.left,
          native.bottom,
          native.right,
          native.top,
        );
      });
      return PdfPageTextData(
        String.fromCharCodes(chars.map((char) => char.codeUnit)),
        chars,
      );
    } finally {
      release(output);
      calloc.free(output);
    }
  }

  @override
  Future<List<PdfPageLinkData>> readPageLinks(int zeroBasedPageIndex) async {
    _bind();
    final read = _readPageLinks;
    final release = _freePageLinks;
    if (read == null || release == null) {
      throw UnsupportedError('产品编辑 ABI 不支持页面链接读取');
    }
    final output = calloc<_NativePageLinks>();
    try {
      _check(read(_session, zeroBasedPageIndex, output), '读取页面链接');
      final count = output.ref.count;
      final uriSize = output.ref.uriSize;
      if (count < 0 ||
          count > 4096 ||
          uriSize > 1024 * 1024 ||
          (count > 0 && output.ref.links == nullptr) ||
          (uriSize > 0 && output.ref.uriData == nullptr)) {
        throw StateError('页面链接结果无效');
      }
      return List<PdfPageLinkData>.generate(count, (index) {
        final link = output.ref.links[index];
        Uri? uri;
        if (link.kind == 1) {
          if (link.uriOffset > uriSize ||
              link.uriLength > uriSize - link.uriOffset) {
            throw StateError('页面链接 URI 范围无效');
          }
          try {
            final bytes = (output.ref.uriData + link.uriOffset).asTypedList(
              link.uriLength,
            );
            uri = Uri.tryParse(utf8.decode(bytes));
          } on FormatException {
            uri = null;
          }
        }
        final kind = switch (link.kind) {
          1 when uri != null => PdfPageLinkKind.uri,
          2 when link.pageIndex >= 0 => PdfPageLinkKind.page,
          _ => PdfPageLinkKind.unsupported,
        };
        return PdfPageLinkData(
          kind: kind,
          left: link.left,
          bottom: link.bottom,
          right: link.right,
          top: link.top,
          uri: uri,
          pageIndex: kind == PdfPageLinkKind.page ? link.pageIndex : null,
        );
      });
    } finally {
      release(output);
      calloc.free(output);
    }
  }

  @override
  Future<PdfEditRaster> renderPageAt(
    int zeroBasedPageIndex,
    int width,
    int height,
  ) async {
    _bind();
    final render = _renderPageAt;
    if (render == null) throw UnsupportedError('产品编辑 ABI 不支持页面渲染');
    final capacity = _rasterCapacity(width, height);
    final output = calloc<Uint8>(capacity);
    try {
      final written = render(
        _session,
        zeroBasedPageIndex,
        width,
        height,
        output,
        capacity,
      );
      _check(written, '渲染文档页面');
      if (written != capacity) {
        throw StateError('渲染文档页面长度异常：$written/$capacity');
      }
      return PdfEditRaster(
        width,
        height,
        Uint8List.fromList(output.asTypedList(capacity)),
      );
    } finally {
      calloc.free(output);
    }
  }

  @override
  Future<PdfEditRaster> renderPageRegion(
    int zeroBasedPageIndex,
    int x,
    int y,
    int width,
    int height,
    int fullWidth,
    int fullHeight,
  ) async {
    _bind();
    final render = _renderPageRegion;
    if (render == null) {
      throw UnsupportedError('产品编辑 ABI 不支持分块渲染');
    }
    if (x < 0 ||
        y < 0 ||
        fullWidth <= 0 ||
        fullHeight <= 0 ||
        x > fullWidth ||
        y > fullHeight ||
        width > fullWidth - x ||
        height > fullHeight - y) {
      throw ArgumentError('分块渲染区域无效');
    }
    final capacity = _rasterCapacity(width, height);
    final output = calloc<Uint8>(capacity);
    try {
      final written = render(
        _session,
        zeroBasedPageIndex,
        x,
        y,
        width,
        height,
        fullWidth,
        fullHeight,
        output,
        capacity,
      );
      _check(written, '渲染文档区域');
      if (written != capacity) {
        throw StateError('渲染文档区域长度异常：$written/$capacity');
      }
      return PdfEditRaster(
        width,
        height,
        Uint8List.fromList(output.asTypedList(capacity)),
      );
    } finally {
      calloc.free(output);
    }
  }

  @override
  Future<void> undo() async => _check(_undoNative(_session), '撤销');
  @override
  Future<void> redo() async => _check(_redoNative(_session), '重做');

  @override
  Future<Uint8List> previewBytes() async {
    final export = _exportBytes;
    final freeBytes = _freeBytes;
    if (export != null && freeBytes != null) {
      final output = calloc<PdfNativeBytes>();
      try {
        _check(export(_session, output), '导出编辑文档');
        if (output.ref.data == nullptr || output.ref.size == 0) {
          throw StateError('编辑内核返回了空 PDF');
        }
        return Uint8List.fromList(output.ref.data.asTypedList(output.ref.size));
      } finally {
        freeBytes(output);
        calloc.free(output);
      }
    }
    final file = File(
      '${Directory.systemTemp.path}/pdf-rebuild-preview-${DateTime.now().microsecondsSinceEpoch}.pdf',
    );
    await saveTo(file.path);
    final bytes = await file.readAsBytes();
    await file.delete();
    return bytes;
  }

  @override
  Future<void> saveTo(String path) async {
    final nativePath = path.toNativeUtf8();
    try {
      _check(_save(_session, nativePath), '保存 PDF');
    } finally {
      calloc.free(nativePath);
    }
  }

  @override
  void close() {
    if (_session != nullptr) _close(_session);
    _session = nullptr;
  }
}
