import 'dart:async';
import 'dart:typed_data';

import 'package:pdfrx/pdfrx.dart';

import 'pdf_edit_backend.dart';
import 'pdf_product_backend.dart';

/// A pdfrx document view over the current product edit session.
/// The product backend owns that session; disposing this view never closes it.
class ProductPdfDocument extends PdfDocument {
  ProductPdfDocument._(this.backend, this._sessionGeneration, String sourceName)
    : super(sourceName: sourceName);

  final PdfProductBackend backend;
  final int _sessionGeneration;
  final _events = StreamController<PdfDocumentEvent>.broadcast();
  List<PdfPage> _pages = const [];
  bool _disposed = false;

  static Future<ProductPdfDocument> open(
    PdfProductBackend backend, {
    required String sourceName,
  }) async {
    if (!backend.edit.isOpen || backend.currentDocument == null) {
      throw StateError('产品文档会话尚未打开');
    }
    final document = ProductPdfDocument._(
      backend,
      backend.sessionGeneration,
      sourceName,
    );
    try {
      final count = backend.currentDocument!.pageCount;
      final pages = <PdfPage>[];
      for (var index = 0; index < count; index++) {
        final size = await backend.pageSize(index);
        pages.add(_ProductPdfPage(document, index + 1, size));
      }
      document._pages = List.unmodifiable(pages);
      return document;
    } catch (_) {
      await document.dispose();
      rethrow;
    }
  }

  void _ensureOpen() {
    if (_disposed ||
        !backend.edit.isOpen ||
        backend.sessionGeneration != _sessionGeneration) {
      throw StateError('产品文档视图已关闭');
    }
  }

  /// Notify pdfrx that a page changed after editing it through the backend.
  void notifyPageModified(int zeroBasedPageIndex) {
    _ensureOpen();
    if (zeroBasedPageIndex < 0 || zeroBasedPageIndex >= _pages.length) {
      throw RangeError.index(zeroBasedPageIndex, _pages);
    }
    _events.add(
      PdfDocumentPageStatusChangedEvent(
        this,
        changes: {
          zeroBasedPageIndex + 1: PdfPageStatusChange.modified(
            page: _pages[zeroBasedPageIndex],
          ),
        },
      ),
    );
  }

  @override
  PdfPermissions? get permissions => null;

  @override
  bool get isEncrypted => false;

  @override
  Stream<PdfDocumentEvent> get events => _events.stream;

  @override
  List<PdfPage> get pages => _pages;

  @override
  set pages(List<PdfPage> value) =>
      throw UnsupportedError('页面修改须通过 PdfProductBackend 命令');

  @override
  Future<void> dispose() async {
    if (_disposed) return;
    _disposed = true;
    await _events.close();
  }

  @override
  Future<void> loadPagesProgressively<T>({
    PdfPageLoadingCallback<T>? onPageLoadProgress,
    T? data,
    Duration loadUnitDuration = const Duration(milliseconds: 250),
  }) async {
    // pdfrx can start a delayed progressive load after its viewer is removed.
    // A disposed view must finish that callback quietly during navigation.
    if (_disposed) return;
    _ensureOpen();
    if (onPageLoadProgress != null) {
      for (var page = 1; page <= _pages.length; page++) {
        if (_disposed) break;
        if (!await onPageLoadProgress(page, _pages.length, data)) break;
      }
    }
    if (!_disposed) _events.add(PdfDocumentLoadCompleteEvent(this));
  }

  @override
  Future<List<PdfOutlineNode>> loadOutline() async =>
      throw UnsupportedError('产品会话尚未提供书签接口');

  @override
  bool isIdenticalDocumentHandle(Object? other) => identical(this, other);

  @override
  Future<bool> assemble() async =>
      throw UnsupportedError('页面组合须通过 PdfProductBackend 命令');

  @override
  Future<Uint8List> encodePdf({
    bool incremental = false,
    bool removeSecurity = false,
  }) async {
    _ensureOpen();
    if (incremental || removeSecurity) {
      throw UnsupportedError('当前产品快照不支持增量保存或移除加密');
    }
    return backend.snapshot();
  }

  @override
  Future<T> useNativeDocumentHandle<T>(
    FutureOr<T> Function(int nativeDocumentHandle) task,
  ) async => throw UnsupportedError('产品会话不暴露 PDFium 裸句柄');

  @override
  Future<void> reloadPages({List<int>? pageNumbersToReload}) async {
    _ensureOpen();
    for (final number
        in pageNumbersToReload ??
            List<int>.generate(_pages.length, (index) => index + 1)) {
      notifyPageModified(number - 1);
    }
  }
}

class _ProductPdfPage extends PdfPage {
  _ProductPdfPage(this.document, this.pageNumber, PdfPageSize size)
    : width = size.width,
      height = size.height;

  @override
  final ProductPdfDocument document;
  @override
  final int pageNumber;
  @override
  final double width;
  @override
  final double height;
  @override
  PdfPageRotation get rotation => PdfPageRotation.none;
  @override
  bool get isLoaded => true;

  @override
  PdfPageRenderCancellationToken createCancellationToken() =>
      _ProductRenderCancellationToken();

  @override
  Future<PdfImage?> render({
    int x = 0,
    int y = 0,
    int? width,
    int? height,
    double? fullWidth,
    double? fullHeight,
    int? backgroundColor,
    PdfPageRotation? rotationOverride,
    PdfAnnotationRenderingMode annotationRenderingMode =
        PdfAnnotationRenderingMode.annotationAndForms,
    int flags = PdfPageRenderFlags.none,
    PdfPageRenderCancellationToken? cancellationToken,
  }) async {
    document._ensureOpen();
    if (cancellationToken != null &&
        cancellationToken is! _ProductRenderCancellationToken) {
      throw ArgumentError.value(cancellationToken, 'cancellationToken');
    }
    if (cancellationToken?.isCanceled ?? false) return null;
    if (rotationOverride != null && rotationOverride != rotation ||
        backgroundColor != null && backgroundColor != 0xffffffff ||
        annotationRenderingMode !=
            PdfAnnotationRenderingMode.annotationAndForms ||
        flags &
                ~(PdfPageRenderFlags.limitedImageCache |
                    PdfPageRenderFlags.premultipliedAlpha) !=
            0) {
      throw UnsupportedError('当前产品分块渲染不支持该渲染选项');
    }
    final virtualWidth = (fullWidth ?? this.width).round();
    final virtualHeight = (fullHeight ?? this.height).round();
    final regionWidth = width ?? virtualWidth;
    final regionHeight = height ?? virtualHeight;
    final raster = await document.backend.renderPageRegion(
      pageNumber - 1,
      x,
      y,
      regionWidth,
      regionHeight,
      virtualWidth,
      virtualHeight,
    );
    if (cancellationToken?.isCanceled ?? false) return null;
    final bgra = Uint8List.fromList(raster.rgba);
    for (var index = 0; index < bgra.length; index += 4) {
      final red = bgra[index];
      bgra[index] = bgra[index + 2];
      bgra[index + 2] = red;
    }
    return PdfImage.createFromBgraData(
      bgra,
      width: raster.width,
      height: raster.height,
    );
  }

  @override
  Future<PdfPageRawText?> loadText() async {
    document._ensureOpen();
    final text = await document.backend.readPageText(pageNumber - 1);
    return PdfPageRawText(
      text.text,
      text.characters
          .map((char) {
            final left = char.left <= char.right ? char.left : char.right;
            final right = char.left <= char.right ? char.right : char.left;
            final bottom = char.bottom <= char.top ? char.bottom : char.top;
            final top = char.bottom <= char.top ? char.top : char.bottom;
            return PdfRect(left, top, right, bottom);
          })
          .toList(growable: false),
    );
  }

  @override
  Future<List<PdfLink>> loadLinks({
    bool compact = false,
    bool enableAutoLinkDetection = true,
  }) async {
    document._ensureOpen();
    final links = await document.backend.readPageLinks(pageNumber - 1);
    final result = <PdfLink>[];
    for (final link in links) {
      if (link.kind == PdfPageLinkKind.unsupported) {
        throw UnsupportedError('此 PDF 含产品会话尚未支持的链接动作');
      }
      final rect = PdfRect(link.left, link.top, link.right, link.bottom);
      final dest = link.pageIndex == null
          ? null
          : PdfDest(link.pageIndex! + 1, PdfDestCommand.fit, null);
      result.add(PdfLink([rect], url: link.uri, dest: dest));
    }
    return compact ? result.map((link) => link.compact()).toList() : result;
  }
}

class _ProductRenderCancellationToken extends PdfPageRenderCancellationToken {
  bool _canceled = false;
  @override
  void cancel() => _canceled = true;
  @override
  bool get isCanceled => _canceled;
}
