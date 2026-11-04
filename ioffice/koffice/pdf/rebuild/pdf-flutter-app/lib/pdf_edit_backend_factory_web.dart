import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'dart:typed_data';

import 'pdf_edit_backend.dart';
import 'pdf_web_bridge.dart';

PdfEditBackend createPdfEditBackend() => WebPdfEditBackend();

class WebPdfEditBackend implements PdfEditBackend {
  JSObject? _bridge;
  int _session = 0;
  int _activePage = -1;

  @override
  bool get supportsMutation => true;
  @override
  String get unavailableReason => '';
  @override
  bool get isOpen => _session != 0;
  @override
  bool get canUndo =>
      isOpen && webCall<JSBoolean>(_bridge!, 'canUndo', [_session.toJS]).toDart;
  @override
  bool get canRedo =>
      isOpen && webCall<JSBoolean>(_bridge!, 'canRedo', [_session.toJS]).toDart;

  JSObject get _ready {
    if (!isOpen || _bridge == null) throw StateError('编辑会话尚未打开');
    return _bridge!;
  }

  @override
  Future<void> openPath(String path) =>
      throw UnsupportedError('Web 请使用文件选择器打开 PDF 字节');

  @override
  Future<void> openBytes(Uint8List bytes) async {
    if (bytes.isEmpty) throw ArgumentError('PDF 字节为空');
    final bridge = await webProductBridge;
    final next = webCall<JSNumber>(bridge, 'open', [bytes.toJS]).toDartInt;
    if (_session != 0) webCall<JSAny?>(bridge, 'close', [_session.toJS]);
    _bridge = bridge;
    _session = next;
    _activePage = -1;
  }

  List<PdfEditDomain> _domains() {
    final values = webCall<JSArray<JSObject>>(_ready, 'domains', [
      _session.toJS,
    ]);
    return values.toDart
        .map(
          (value) => PdfEditDomain(
            webInt(value, 'index'),
            webDouble(value, 'left'),
            webDouble(value, 'bottom'),
            webDouble(value, 'right'),
            webDouble(value, 'top'),
          ),
        )
        .toList();
  }

  @override
  Future<List<PdfEditDomain>> loadPage(int zeroBasedPageIndex) async {
    webCall<JSAny?>(_ready, 'loadPage', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
    ]);
    _activePage = zeroBasedPageIndex;
    return _domains();
  }

  @override
  Future<List<PdfEditDomain>> refreshDomains() async => _domains();

  @override
  Future<String> domainText(int domainIndex) async => webCall<JSString>(
    _ready,
    'domainText',
    [_session.toJS, domainIndex.toJS],
  ).toDart;

  @override
  Future<void> replaceDomain(int domainIndex, String text) async {
    webCall<JSAny?>(_ready, 'replaceDomain', [
      _session.toJS,
      domainIndex.toJS,
      text.toJS,
    ]);
  }

  PdfEditFocus _focus(JSObject value) {
    final rawCaret = value['caret'];
    final caret = rawCaret == null ? null : rawCaret as JSObject;
    return PdfEditFocus(
      webString(value, 'text'),
      webInt(value, 'start'),
      webInt(value, 'end'),
      caret == null
          ? null
          : PdfEditCaret(
              webDouble(caret, 'bottomX'),
              webDouble(caret, 'bottomY'),
              webDouble(caret, 'topX'),
              webDouble(caret, 'topY'),
            ),
    );
  }

  @override
  Future<PdfEditFocus> focusAt(double pageX, double pageY) async => _focus(
    webCall<JSObject>(_ready, 'focusAt', [
      _session.toJS,
      pageX.toJS,
      pageY.toJS,
    ]),
  );

  @override
  Future<PdfEditFocus> setSelection(int start, int end) async => _focus(
    webCall<JSObject>(_ready, 'setSelection', [
      _session.toJS,
      start.toJS,
      end.toJS,
    ]),
  );

  @override
  Future<PdfEditFocus> replaceRange(int start, int end, String text) async =>
      _focus(
        webCall<JSObject>(_ready, 'replaceRange', [
          _session.toJS,
          start.toJS,
          end.toJS,
          text.toJS,
        ]),
      );

  PdfEditRaster _raster(int width, int height, JSUint8Array bytes) =>
      PdfEditRaster(width, height, webBytes(bytes));

  @override
  Future<PdfEditRaster> renderPage(int width, int height) async => _raster(
    width,
    height,
    webCall<JSUint8Array>(_ready, 'render', [
      _session.toJS,
      _activePage.toJS,
      0.toJS,
      0.toJS,
      width.toJS,
      height.toJS,
      width.toJS,
      height.toJS,
      true.toJS,
    ]),
  );

  @override
  Future<PdfPageSize> pageSize(int zeroBasedPageIndex) async {
    final value = webCall<JSObject>(_ready, 'pageSize', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
    ]);
    return PdfPageSize(webDouble(value, 'width'), webDouble(value, 'height'));
  }

  @override
  Future<PdfPageTextData> readPageText(int zeroBasedPageIndex) async {
    final value = webCall<JSObject>(_ready, 'pageText', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
    ]);
    final units = (value['units'] as JSUint16Array).toDart;
    final boxes = (value['boxes'] as JSFloat32Array).toDart;
    final chars = List<PdfPageTextCharacter>.generate(units.length, (index) {
      final offset = index * 4;
      return PdfPageTextCharacter(
        units[index],
        boxes[offset],
        boxes[offset + 1],
        boxes[offset + 2],
        boxes[offset + 3],
      );
    });
    return PdfPageTextData(String.fromCharCodes(units), chars);
  }

  @override
  Future<List<PdfPageLinkData>> readPageLinks(int zeroBasedPageIndex) async {
    final values = webCall<JSArray<JSObject>>(_ready, 'pageLinks', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
    ]);
    return values.toDart.map((value) {
      final kind = webInt(value, 'kind');
      final pageIndex = webInt(value, 'pageIndex');
      final uriText = value['uri'];
      final uri = uriText == null
          ? null
          : Uri.tryParse((uriText as JSString).toDart);
      final linkKind = kind == 1 && uri != null
          ? PdfPageLinkKind.uri
          : kind == 2 && pageIndex >= 0
          ? PdfPageLinkKind.page
          : PdfPageLinkKind.unsupported;
      return PdfPageLinkData(
        kind: linkKind,
        left: webDouble(value, 'left'),
        bottom: webDouble(value, 'bottom'),
        right: webDouble(value, 'right'),
        top: webDouble(value, 'top'),
        uri: uri,
        pageIndex: linkKind == PdfPageLinkKind.page ? pageIndex : null,
      );
    }).toList();
  }

  @override
  Future<PdfEditRaster> renderPageAt(
    int zeroBasedPageIndex,
    int width,
    int height,
  ) async => _raster(
    width,
    height,
    webCall<JSUint8Array>(_ready, 'render', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
      0.toJS,
      0.toJS,
      width.toJS,
      height.toJS,
      width.toJS,
      height.toJS,
    ]),
  );

  @override
  Future<PdfEditRaster> renderPageRegion(
    int zeroBasedPageIndex,
    int x,
    int y,
    int width,
    int height,
    int fullWidth,
    int fullHeight,
  ) async => _raster(
    width,
    height,
    webCall<JSUint8Array>(_ready, 'render', [
      _session.toJS,
      zeroBasedPageIndex.toJS,
      x.toJS,
      y.toJS,
      width.toJS,
      height.toJS,
      fullWidth.toJS,
      fullHeight.toJS,
    ]),
  );

  @override
  Future<void> undo() async => webCall<JSAny?>(_ready, 'undo', [_session.toJS]);
  @override
  Future<void> redo() async => webCall<JSAny?>(_ready, 'redo', [_session.toJS]);
  @override
  Future<Uint8List> previewBytes() async =>
      webBytes(webCall<JSUint8Array>(_ready, 'snapshot', [_session.toJS]));
  @override
  Future<void> saveTo(String path) =>
      throw UnsupportedError('Web 请使用另存副本功能下载 PDF');

  @override
  void close() {
    if (_session != 0) webCall<JSAny?>(_ready, 'close', [_session.toJS]);
    _session = 0;
    _activePage = -1;
  }
}
