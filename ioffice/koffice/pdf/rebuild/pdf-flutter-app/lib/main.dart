import 'dart:async';
import 'dart:convert';
import 'dart:math' as math;
import 'dart:ui' as ui;

import 'package:file_selector/file_selector.dart';
import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:pdfrx/pdfrx.dart';

import 'pdf_edit_backend.dart';
import 'pdf_page_operations.dart';
import 'pdf_product_backend.dart';
import 'product_pdf_document.dart';
import 'pdf_document_save.dart';
import 'pdf_document_status.dart';
import 'pdf_product_chrome.dart';
import 'pdf_page_arrangement.dart';
import 'pdf_view_navigation.dart';

void main() => runApp(const PdfRebuildApp());

enum WorkspaceMode { read, edit, annotate }

class PdfRebuildApp extends StatefulWidget {
  const PdfRebuildApp({
    super.key,
    this.initialBytes,
    this.initialName,
    this.editBackend,
    this.pageOperations,
    this.productBackend,
    this.chrome,
    this.commands,
    this.enabledFeatures,
    this.generatedOutputDirectory,
    this.openGeneratedDocument,
  });
  final Uint8List? initialBytes;
  final String? initialName;
  final PdfEditBackend? editBackend;
  final PdfPageOperations? pageOperations;
  final PdfProductBackend? productBackend;
  final PdfChromeDefinition? chrome;
  final Map<String, Map<String, dynamic>>? commands;
  final Set<String>? enabledFeatures;
  final String? Function()? generatedOutputDirectory;
  final Future<void> Function(String path)? openGeneratedDocument;

  @override
  State<PdfRebuildApp> createState() => _PdfRebuildAppState();
}

class _PdfRebuildAppState extends State<PdfRebuildApp> {
  late final Future<PdfChromeDefinition> _chrome = widget.chrome == null
      ? Future.wait([
          rootBundle.loadString('assets/product/chrome.json'),
          rootBundle.loadString('assets/product/icons.json'),
        ]).then(
          (texts) => PdfChromeDefinition.fromJson(
            jsonDecode(texts[0]) as Map<String, dynamic>,
            iconData: jsonDecode(texts[1]) as Map<String, dynamic>,
          ),
        )
      : Future.value(widget.chrome!);

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'PdfRebuild',
    theme: ThemeData(
      colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xff2563eb)),
      useMaterial3: true,
    ),
    home: FutureBuilder<PdfChromeDefinition>(
      future: _chrome,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Scaffold(
            body: Center(child: Text('PDF 产品资源加载失败：${snapshot.error}')),
          );
        }
        if (!snapshot.hasData) {
          return const Scaffold(
            body: Center(child: CircularProgressIndicator()),
          );
        }
        return PdfWorkspace(
          chrome: snapshot.data!,
          initialBytes: widget.initialBytes,
          initialName: widget.initialName,
          editBackend: widget.editBackend,
          pageOperations: widget.pageOperations,
          productBackend: widget.productBackend,
          commands: widget.commands,
          enabledFeatures: widget.enabledFeatures,
          generatedOutputDirectory: widget.generatedOutputDirectory,
          openGeneratedDocument: widget.openGeneratedDocument,
        );
      },
    ),
  );
}

class PdfWorkspace extends StatefulWidget {
  const PdfWorkspace({
    super.key,
    required this.chrome,
    this.initialBytes,
    this.initialName,
    this.editBackend,
    this.pageOperations,
    this.productBackend,
    this.commands,
    this.enabledFeatures,
    this.generatedOutputDirectory,
    this.openGeneratedDocument,
  });
  final Uint8List? initialBytes;
  final String? initialName;
  final PdfEditBackend? editBackend;
  final PdfPageOperations? pageOperations;
  final PdfProductBackend? productBackend;
  final PdfChromeDefinition chrome;
  final Map<String, Map<String, dynamic>>? commands;
  final Set<String>? enabledFeatures;
  final String? Function()? generatedOutputDirectory;

  /// Host navigation port. The product session retains the source document.
  final Future<void> Function(String path)? openGeneratedDocument;

  @override
  State<PdfWorkspace> createState() => _PdfWorkspaceState();
}

enum _PdfFitMode { width, page }

class _PdfWorkspaceState extends State<PdfWorkspace> {
  // Web ships only the product WASM. Native platforms retain a diagnostic
  // reader fallback until the remaining pdfrx capabilities are migrated.
  static const bool _useProductReader =
      kIsWeb || bool.fromEnvironment('KSP_PRODUCT_READER', defaultValue: true);
  static const XTypeGroup _pdfType = XTypeGroup(
    label: 'PDF',
    extensions: <String>['pdf'],
    mimeTypes: <String>['application/pdf'],
    uniformTypeIdentifiers: <String>['com.adobe.pdf'],
    webWildCards: <String>['application/pdf'],
  );

  final PdfViewerController _viewerController = PdfViewerController();
  late final PdfViewerSizeDelegateProviderLegacy _viewerSizePolicy =
      PdfViewerSizeDelegateProviderLegacy(
        minScale: (widget.chrome.statusBar['minZoom'] as num).toDouble(),
        maxScale: (widget.chrome.statusBar['maxZoom'] as num).toDouble(),
        useAlternativeFitScaleAsMinScale: false,
      );
  PdfTextSearcher? _textSearcher;
  late final PdfEditBackend _editBackend;
  late final PdfPageOperations _pageOperations;
  late final PdfProductBackend _productBackend;
  ProductPdfDocument? _productReader;
  final FocusNode _inlineFocusNode = FocusNode(debugLabel: 'pdf-edit-caret');
  final TextEditingController _inlineController = TextEditingController();
  Uint8List? _bytes;
  String _sourceName = '未打开文档';
  int _sourceRevision = 0;
  int _sessionGeneration = 0;
  int _pageNumber = 1;
  int _pageCount = 0;
  double _zoom = 1;
  _PdfFitMode? _fitMode = _PdfFitMode.width;
  PdfPageArrangement _pageArrangement = PdfPageArrangement.single;
  double? _lastViewportWidth;
  int _viewportFitGeneration = 0;
  WorkspaceMode _mode = WorkspaceMode.read;
  bool _busy = false;
  PdfCopySavePhase _copySavePhase = PdfCopySavePhase.idle;
  String? _copySaveError;
  Timer? _copySaveFeedbackTimer;
  List<PdfEditDomain> _editDomains = const <PdfEditDomain>[];
  int? _activeDomainIndex;
  PdfEditFocus? _textFocus;
  Future<void> _editQueue = Future<void>.value();
  bool _settingInlineController = false;
  ui.Image? _managedEditImage;
  int _managedRasterWidth = 0;
  int _managedRasterHeight = 0;
  bool _managedRasterBusy = false;
  bool _managedRasterRefreshPending = false;

  @override
  void initState() {
    super.initState();
    _viewerController.addListener(_onViewerTransform);
    _productBackend =
        widget.productBackend ??
        PdfProductBackend(
          edit: widget.editBackend,
          pages: widget.pageOperations,
        );
    _editBackend = _productBackend.edit;
    _pageOperations = _productBackend.pages;
    _bytes = widget.initialBytes;
    if (_bytes != null) {
      _sourceName = widget.initialName ?? 'sample.pdf';
      _busy = true;
      WidgetsBinding.instance.addPostFrameCallback((_) async {
        try {
          await _openDocumentBytes(widget.initialBytes!, _sourceName);
        } catch (error, stackTrace) {
          debugPrint('打开初始 PDF 失败：$error');
          debugPrintStack(stackTrace: stackTrace);
          if (!mounted) return;
          setState(() => _busy = false);
          _showMessage('打开失败：$error');
        }
      });
    }
    _inlineController.addListener(_onInlineSelectionChanged);
  }

  @override
  void dispose() {
    _copySaveFeedbackTimer?.cancel();
    _textSearcher?.dispose();
    _viewerController.removeListener(_onViewerTransform);
    _inlineFocusNode.dispose();
    _inlineController.removeListener(_onInlineSelectionChanged);
    _inlineController.dispose();
    _managedEditImage?.dispose();
    final reader = _productReader;
    if (reader != null) unawaited(reader.dispose());
    _productBackend.close();
    super.dispose();
  }

  void _onViewerTransform() {
    if (!mounted || !_viewerController.isReady) return;
    final zoom = _viewerController.currentZoom;
    if ((zoom - _zoom).abs() > 0.01) setState(() => _zoom = zoom);
  }

  void _onViewportWidthChanged(double width) {
    if (!width.isFinite || width <= 0) return;
    final previous = _lastViewportWidth;
    if (previous != null && (width - previous).abs() < 1) return;
    _lastViewportWidth = width;
    if (previous == null || _fitMode == null) return;
    final generation = ++_viewportFitGeneration;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted ||
          generation != _viewportFitGeneration ||
          !_viewerController.isReady) {
        return;
      }
      _applyCurrentFit();
    });
  }

  void _applyCurrentFit() {
    if (!_viewerController.isReady || _fitMode == null) return;
    final controller = _viewerController;
    if (_pageArrangement == PdfPageArrangement.single) {
      final matrix = switch (_fitMode) {
        _PdfFitMode.width => controller.calcMatrixFitWidthForPage(
          pageNumber: _pageNumber,
        ),
        _PdfFitMode.page => controller.calcMatrixForFit(
          pageNumber: _pageNumber,
        ),
        null => null,
      };
      if (matrix != null) controller.value = matrix;
      return;
    }
    final viewport = widget.chrome.pageViewport;
    final geometry = PdfArrangementGeometry(
      controller.layout.pageLayouts,
      controller.layout.documentSize,
    );
    final row = geometry.rowBounds(_pageNumber, _pageArrangement);
    final fitWidth =
        (controller.viewSize.width - viewport.horizontalPadding * 2) /
        row.width;
    final fitHeight =
        (controller.viewSize.height - viewport.verticalPadding * 2) /
        row.height;
    final zoom = _fitMode == _PdfFitMode.width
        ? fitWidth
        : math.min(fitWidth, fitHeight);
    if (!zoom.isFinite || zoom <= 0) return;
    final center = _fitMode == _PdfFitMode.width
        ? Offset(
            row.center.dx,
            row.top +
                (controller.viewSize.height / 2 - viewport.verticalPadding) /
                    zoom,
          )
        : row.center;
    controller.value = controller.calcMatrixFor(center, zoom: zoom);
  }

  Future<void> _openPdf() async {
    try {
      final file = await openFile(
        acceptedTypeGroups: const <XTypeGroup>[_pdfType],
      );
      if (file == null) return;
      setState(() => _busy = true);
      final data = await file.readAsBytes();
      await _openDocumentBytes(data, file.name);
    } catch (error) {
      if (!mounted) return;
      setState(() => _busy = false);
      _showMessage('打开失败：$error');
    }
  }

  Future<void> _openDocumentBytes(
    Uint8List data,
    String name, {
    int initialPageNumber = 1,
  }) async {
    await _editQueue;
    final document = await _productBackend.open(data, name);
    if (!mounted) return;
    ProductPdfDocument? reader;
    Object? readerError;
    if (_useProductReader && _editBackend.isOpen) {
      try {
        reader = await ProductPdfDocument.open(
          _productBackend,
          sourceName: '$name#${_sourceRevision + 1}',
        );
      } catch (error, stackTrace) {
        readerError = error;
        debugPrint('共用阅读视图打开失败：$error');
        debugPrintStack(stackTrace: stackTrace);
      }
    }
    if (kIsWeb && reader == null) {
      throw StateError(
        'Web 产品阅读会话不可用：${readerError ?? _editBackend.unavailableReason}',
      );
    }
    final previousReader = _productReader;
    _textSearcher?.dispose();
    _textSearcher = null;
    _inlineFocusNode.unfocus();
    _copySaveFeedbackTimer?.cancel();
    setState(() {
      _sessionGeneration++;
      _copySavePhase = PdfCopySavePhase.idle;
      _copySaveError = null;
      _productReader = reader;
      _bytes = document.bytes;
      _sourceName = document.name;
      _sourceRevision++;
      _pageNumber = document.pageCount > 0
          ? initialPageNumber.clamp(1, document.pageCount)
          : 1;
      _pageCount = document.pageCount;
      _fitMode = _PdfFitMode.width;
      _mode = WorkspaceMode.read;
      _editDomains = const <PdfEditDomain>[];
      _activeDomainIndex = null;
      _textFocus = null;
      _replaceManagedEditImage(null);
      _busy = false;
    });
    if (previousReader != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        unawaited(previousReader.dispose());
      });
    }
    if (readerError != null) {
      _showMessage('共用阅读视图不可用，已切换兼容阅读器：$readerError');
    }
  }

  Future<Uint8List> _currentDocumentBytes() async {
    await _editQueue;
    return _productBackend.snapshot();
  }

  Future<void> _extractPage(int pageNumber, [List<int>? pageRange]) async {
    if (!_pageOperations.isAvailable) {
      _showMessage(_pageOperations.unavailableReason);
      return;
    }
    if (pageNumber < 1 || pageNumber > _pageCount) return;
    final selected = pageRange ?? [pageNumber];
    try {
      setState(() => _busy = true);
      await _editQueue;
      final extracted = await _productBackend.extractCurrentRange(selected);
      if (!mounted) return;
      setState(() => _busy = false);
      final baseName = _sourceName.replaceFirst(
        RegExp(r'\.pdf$', caseSensitive: false),
        '',
      );
      final spec = Map<String, dynamic>.from(
        widget.chrome.dialogs['extractRange'] as Map? ?? {},
      );
      final safeBase = baseName.replaceAll(RegExp(r'[/\\:*?"<>|]'), '_');
      final first = selected.first;
      final last = selected.last;
      final pattern = selected.length == 1
          ? spec['singleNamePattern'] as String? ?? '{base}-p{first}.pdf'
          : spec['rangeNamePattern'] as String? ?? '{base}-p{first}-{last}.pdf';
      final name = pattern
          .replaceAll('{base}', safeBase.isEmpty ? 'pages' : safeBase)
          .replaceAll('{first}', '$first')
          .replaceAll('{last}', '$last');
      final generatedPath = spec['desktopDestination'] == 'generatedDirectory'
          ? await saveGeneratedPdfDesktop(
              name,
              extracted,
              configuredDirectory: widget.generatedOutputDirectory?.call(),
            )
          : null;
      if (generatedPath == null &&
          !await _savePdfBytes(Uint8List.fromList(extracted), name)) {
        return;
      }
      if (!mounted) return;
      if (generatedPath != null &&
          spec['desktopOpenResult'] == 'newDocumentTab' &&
          widget.openGeneratedDocument != null) {
        // The file is durable before the Host receives a navigation request.
        // A tab-routing error must not report the extraction as a failed write.
        try {
          await widget.openGeneratedDocument!(generatedPath);
        } catch (error) {
          _showMessage('PDF 已保存，打开新标签失败：$error');
          return;
        }
      } else {
        await _openDocumentBytes(extracted, name);
      }
      _showMessage('已保存抽取 PDF：$name');
    } catch (error) {
      if (mounted) setState(() => _busy = false);
      _showMessage('抽取页面失败：$error');
    }
  }

  Future<void> _deletePage(int pageNumber) async {
    if (!_pageOperations.isAvailable) {
      _showMessage(_pageOperations.unavailableReason);
      return;
    }
    if (_pageCount <= 1 || pageNumber < 1 || pageNumber > _pageCount) return;
    try {
      setState(() => _busy = true);
      await _editQueue;
      final deletedPage = pageNumber;
      final result = await _productBackend.deleteCurrentPage(deletedPage - 1);
      final baseName = _sourceName.replaceFirst(
        RegExp(r'\.pdf$', caseSensitive: false),
        '',
      );
      await _openDocumentBytes(
        result,
        '${baseName}_删除第$deletedPage页.pdf',
        initialPageNumber: deletedPage.clamp(1, _pageCount - 1),
      );
    } catch (error) {
      if (mounted) setState(() => _busy = false);
      _showMessage('删除页面失败：$error');
    }
  }

  Future<void> _mergePdf() async {
    if (!_pageOperations.isAvailable) {
      _showMessage(_pageOperations.unavailableReason);
      return;
    }
    try {
      final file = await openFile(acceptedTypeGroups: const [_pdfType]);
      if (file == null) return;
      setState(() => _busy = true);
      final incoming = await file.readAsBytes();
      await _editQueue;
      final merged = await _productBackend.mergeCurrent(incoming);
      final baseName = _sourceName.replaceFirst(
        RegExp(r'\.pdf$', caseSensitive: false),
        '',
      );
      await _openDocumentBytes(merged, '${baseName}_合并.pdf');
    } catch (error) {
      if (mounted) setState(() => _busy = false);
      _showMessage('合并 PDF 失败：$error');
    }
  }

  Future<void> _reversePages() async {
    if (!_pageOperations.isAvailable || _pageCount < 2) return;
    try {
      setState(() => _busy = true);
      await _editQueue;
      final reversed = await _productBackend.reverseCurrentPages();
      final baseName = _sourceName.replaceFirst(
        RegExp(r'\.pdf$', caseSensitive: false),
        '',
      );
      await _openDocumentBytes(
        reversed,
        '${baseName}_倒序.pdf',
        initialPageNumber: _pageCount - _pageNumber + 1,
      );
    } catch (error) {
      if (mounted) setState(() => _busy = false);
      _showMessage('页面倒序失败：$error');
    }
  }

  Future<void> _splitPdf(int pagesPerFile) async {
    if (!_pageOperations.isAvailable || _pageCount < 2) return;
    try {
      setState(() => _busy = true);
      await _editQueue;
      final parts = await _productBackend.splitCurrent(pagesPerFile);
      if (!mounted) return;
      setState(() => _busy = false);
      final baseName = _sourceName.replaceFirst(
        RegExp(r'\.pdf$', caseSensitive: false),
        '',
      );
      final splitSpec = Map<String, dynamic>.from(
        widget.chrome.dialogs['splitPdf'] as Map? ?? {},
      );
      final safeBase = baseName.replaceAll(RegExp(r'[/\\:*?"<>|]'), '_');
      final fileNamePattern =
          splitSpec['fileNamePattern'] as String? ?? '{base}-{index}.pdf';
      final outputs = <String, List<int>>{};
      for (var index = 0; index < parts.length; index++) {
        final name = fileNamePattern
            .replaceAll('{base}', safeBase.isEmpty ? 'split' : safeBase)
            .replaceAll('{index}', '${index + 1}');
        outputs[name] = parts[index];
      }
      var saved = splitSpec['destinationMode'] == 'directory'
          ? await savePdfBatchWithPlatformPicker(outputs)
          : null;
      if (saved == null) {
        var fallbackSaved = 0;
        for (final entry in outputs.entries) {
          if (!await _savePdfBytes(
            Uint8List.fromList(entry.value),
            entry.key,
          )) {
            break;
          }
          fallbackSaved++;
        }
        saved = fallbackSaved;
      }
      _showMessage('已保存 $saved/${parts.length} 个拆分文件');
    } catch (error) {
      if (mounted) setState(() => _busy = false);
      _showMessage('拆分 PDF 失败：$error');
    }
  }

  Future<bool> _savePdfBytes(Uint8List data, String name) async {
    Future<void> writeCopy(String path) => XFile.fromData(
      data,
      name: name,
      mimeType: 'application/pdf',
    ).saveTo(path);
    if (supportsPlatformDocumentSave) {
      return savePdfWithPlatformPicker(name, writeCopy);
    }
    final location = await getSaveLocation(
      suggestedName: name,
      acceptedTypeGroups: const <XTypeGroup>[_pdfType],
    );
    if (location == null) return false;
    await writeCopy(location.path);
    return true;
  }

  Future<void> _saveCopy() async {
    if (_bytes == null) {
      _showMessage('请先打开 PDF');
      return;
    }
    _copySaveFeedbackTimer?.cancel();
    final generation = _sessionGeneration;
    final sourceName = _sourceName;
    setState(() {
      _copySavePhase = PdfCopySavePhase.saving;
      _copySaveError = null;
    });
    try {
      final snapshot = await _currentDocumentBytes();
      if (await _savePdfBytes(snapshot, sourceName)) {
        if (!mounted || generation != _sessionGeneration) return;
        setState(() => _copySavePhase = PdfCopySavePhase.saved);
        _clearCopySaveFeedbackLater();
        _showMessage('副本已保存');
      } else if (mounted && generation == _sessionGeneration) {
        setState(() => _copySavePhase = PdfCopySavePhase.idle);
      }
    } catch (error) {
      if (!mounted || generation != _sessionGeneration) return;
      setState(() {
        _copySavePhase = PdfCopySavePhase.failed;
        _copySaveError = '$error';
      });
      _clearCopySaveFeedbackLater();
      _showMessage('另存 PDF 失败：$error');
    }
  }

  void _clearCopySaveFeedbackLater() {
    final duration =
        (widget.chrome.ribbonState['feedbackMs'] as num?)?.toInt() ?? 3000;
    _copySaveFeedbackTimer = Timer(Duration(milliseconds: duration), () {
      if (!mounted) return;
      setState(() {
        _copySavePhase = PdfCopySavePhase.idle;
        _copySaveError = null;
      });
    });
  }

  void _showMessage(String message) {
    if (!mounted) return;
    ScaffoldMessenger.of(context)
      ..hideCurrentSnackBar()
      ..showSnackBar(SnackBar(content: Text(message)));
  }

  Future<void> _goToPage(int page) async {
    if (!_viewerController.isReady || _pageCount == 0) return;
    if (_pageArrangement == PdfPageArrangement.spread) {
      final geometry = PdfArrangementGeometry(
        _viewerController.layout.pageLayouts,
        _viewerController.layout.documentSize,
      );
      final row = geometry.rowBounds(
        page.clamp(1, _pageCount),
        _pageArrangement,
      );
      await _viewerController.goToPosition(
        documentOffset: Offset(0, row.top),
        duration: Duration.zero,
      );
      return;
    }
    await _viewerController.goToPage(
      pageNumber: page.clamp(1, _pageCount),
      anchor: PdfPageAnchor.topLeft,
    );
  }

  Future<void> _navigateDocument(PdfNavigationAction action) async {
    if (!_viewerController.isReady) return;
    final controller = _viewerController;
    final visible = controller.visibleRect;
    final zoom = controller.currentZoom;
    if (zoom <= 0) return;
    final policy = widget.chrome.keyboardNavigation;
    final viewportStep =
        (controller.viewSize.height - policy.viewportInset).clamp(
          1.0,
          double.infinity,
        ) /
        zoom;
    final lineStep = policy.linePixels / zoom;
    final target = switch (action) {
      PdfNavigationAction.viewportNext => visible.top + viewportStep,
      PdfNavigationAction.viewportPrevious => visible.top - viewportStep,
      PdfNavigationAction.documentStart => 0.0,
      PdfNavigationAction.documentEnd => controller.documentSize.height,
      PdfNavigationAction.lineNext => visible.top + lineStep,
      PdfNavigationAction.linePrevious => visible.top - lineStep,
      PdfNavigationAction.pageNext ||
      PdfNavigationAction.pagePrevious => visible.top,
    };
    await controller.goToPosition(
      documentOffset: Offset(visible.left, target),
      duration: Duration.zero,
    );
  }

  Future<void> _selectMode(WorkspaceMode mode) async {
    final leavingEdit =
        _mode == WorkspaceMode.edit &&
        mode != WorkspaceMode.edit &&
        _productBackend.isDirty &&
        _editBackend.isOpen;
    setState(() => _mode = mode);
    if (mode != WorkspaceMode.edit) {
      _inlineFocusNode.unfocus();
      _activeDomainIndex = null;
      _textFocus = null;
    }
    if (mode != WorkspaceMode.read && !_editBackend.supportsMutation) {
      _showMessage(_editBackend.unavailableReason);
      return;
    }
    if (mode == WorkspaceMode.edit && _editBackend.isOpen) {
      await _loadEditPage();
    } else if (leavingEdit) {
      await _commitViewerPreview();
    }
  }

  Future<void> _loadEditPage() async {
    if (!_editBackend.isOpen) return;
    try {
      final domains = await _editBackend.loadPage(_pageNumber - 1);
      if (mounted) {
        setState(() {
          _editDomains = domains;
          _replaceManagedEditImage(null);
        });
      }
    } catch (error) {
      _showMessage('编辑域识别失败：$error');
    }
  }

  Future<void> _beginInlineEdit(
    PdfEditDomain domain,
    Offset localPosition,
    Size overlaySize,
  ) async {
    try {
      final pageX =
          domain.left +
          localPosition.dx / overlaySize.width * (domain.right - domain.left);
      final pageY =
          domain.top -
          localPosition.dy / overlaySize.height * (domain.top - domain.bottom);
      final focus = await _editBackend.focusAt(pageX, pageY);
      if (!mounted) return;
      _setInlineController(focus);
      setState(() {
        _activeDomainIndex = domain.index;
        _textFocus = focus;
      });
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _inlineFocusNode.requestFocus();
      });
    } catch (error) {
      _showMessage('光标定位失败：$error');
    }
  }

  Future<void> _toggleInlineKeyboard() async {
    if (_textFocus == null) {
      if (_editDomains.isEmpty) {
        _showMessage('当前页面没有可编辑文字');
        return;
      }
      await _beginInlineEdit(
        _editDomains.first,
        const Offset(0.5, 0.5),
        const Size(1, 1),
      );
    } else if (_inlineFocusNode.hasFocus) {
      _inlineFocusNode.unfocus();
    } else {
      _inlineFocusNode.requestFocus();
    }
  }

  void _setInlineController(PdfEditFocus focus) {
    _settingInlineController = true;
    try {
      _inlineController.value = TextEditingValue(
        text: focus.text,
        selection: TextSelection(
          baseOffset: focus.start.clamp(0, focus.text.length),
          extentOffset: focus.end.clamp(0, focus.text.length),
        ),
      );
    } finally {
      _settingInlineController = false;
    }
  }

  void _onInlineSelectionChanged() {
    if (_settingInlineController) return;
    final focus = _textFocus;
    final value = _inlineController.value;
    if (focus == null ||
        value.text != focus.text ||
        !value.selection.isValid ||
        (value.selection.start == focus.start &&
            value.selection.end == focus.end)) {
      return;
    }
    _editQueue = _editQueue
        .then((_) => _syncInlineSelection(value.selection))
        .catchError((Object error) {
          _showMessage('移动光标失败：$error');
        });
  }

  Future<void> _syncInlineSelection(TextSelection selection) async {
    final updated = await _editBackend.setSelection(
      selection.start,
      selection.end,
    );
    if (!mounted) return;
    setState(() => _textFocus = updated);
  }

  void _onInlineTextChanged(String _) {
    final target = _inlineController.value;
    _editQueue = _editQueue.then((_) => _applyInlineTarget(target)).catchError((
      Object error,
    ) {
      _showMessage('原地编辑失败：$error');
    });
  }

  Future<void> _applyInlineTarget(TextEditingValue target) async {
    final current = _textFocus;
    final targetText = target.text;
    if (current == null || targetText == current.text) return;
    final replacement = derivePdfTextReplacement(
      currentText: current.text,
      selectionStart: current.start,
      selectionEnd: current.end,
      targetText: targetText,
      targetCaret: target.selection.isValid
          ? target.selection.extentOffset
          : current.start,
    );
    final updated = await _productBackend.replaceRange(
      replacement.start,
      replacement.end,
      replacement.text,
    );
    if (!mounted) return;
    _setInlineController(updated);
    setState(() => _textFocus = updated);
    await _refreshManagedEditPage(focus: updated);
  }

  void _clearInlineFocus() {
    _inlineFocusNode.unfocus();
    _activeDomainIndex = null;
    _textFocus = null;
  }

  Future<void> _refreshManagedEditPage({PdfEditFocus? focus}) async {
    // Match Android's scheduleEditRefresh: retain the loaded native edit page,
    // refresh its current domain geometry and render that same managed page.
    final domains = await _editBackend.refreshDomains();
    if (!mounted) return;
    setState(() {
      _editDomains = domains;
      if (focus != null) _textFocus = focus;
    });
    if (_managedRasterWidth > 0 && _managedRasterHeight > 0) {
      await _renderManagedEditPage(
        _managedRasterWidth,
        _managedRasterHeight,
        force: true,
      );
    }
    if (focus != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _inlineFocusNode.requestFocus();
      });
    }
  }

  Future<void> _undo() async {
    if (!_editBackend.canUndo) return;
    try {
      _clearInlineFocus();
      await _productBackend.undo();
      await _refreshManagedEditPage();
    } catch (error) {
      _showMessage('撤销失败：$error');
    }
  }

  Future<void> _redo() async {
    if (!_editBackend.canRedo) return;
    try {
      _clearInlineFocus();
      await _productBackend.redo();
      await _refreshManagedEditPage();
    } catch (error) {
      _showMessage('重做失败：$error');
    }
  }

  Future<void> _commitViewerPreview() async {
    try {
      if (_productReader != null) {
        _productReader!.notifyPageModified(_pageNumber - 1);
        if (mounted) setState(() => _replaceManagedEditImage(null));
        return;
      }
      final preview = await _currentDocumentBytes();
      if (!mounted) return;
      setState(() {
        _bytes = preview;
        _sourceRevision++;
        _replaceManagedEditImage(null);
      });
    } catch (error) {
      _showMessage('提交编辑预览失败：$error');
    }
  }

  void _replaceManagedEditImage(ui.Image? image) {
    if (!identical(_managedEditImage, image)) _managedEditImage?.dispose();
    _managedEditImage = image;
    if (image == null) {
      _managedRasterWidth = 0;
      _managedRasterHeight = 0;
    }
  }

  void _ensureManagedEditPage(Size pageSize) {
    if (_mode != WorkspaceMode.edit || !_editBackend.isOpen) return;
    final width = (pageSize.width * 2).round().clamp(600, 1800);
    final height = (width * pageSize.height / pageSize.width).round();
    if (_managedEditImage != null &&
        _managedRasterWidth == width &&
        _managedRasterHeight == height) {
      return;
    }
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _renderManagedEditPage(width, height);
    });
  }

  Future<void> _renderManagedEditPage(
    int width,
    int height, {
    bool force = false,
  }) async {
    if (_managedRasterBusy) {
      _managedRasterRefreshPending = _managedRasterRefreshPending || force;
      return;
    }
    if (!force &&
        _managedEditImage != null &&
        _managedRasterWidth == width &&
        _managedRasterHeight == height) {
      return;
    }
    _managedRasterBusy = true;
    try {
      final raster = await _editBackend.renderPage(width, height);
      final completer = Completer<ui.Image>();
      ui.decodeImageFromPixels(
        raster.rgba,
        raster.width,
        raster.height,
        ui.PixelFormat.rgba8888,
        completer.complete,
      );
      final image = await completer.future;
      if (!mounted || _mode != WorkspaceMode.edit) {
        image.dispose();
        return;
      }
      setState(() {
        _replaceManagedEditImage(image);
        _managedRasterWidth = width;
        _managedRasterHeight = height;
      });
    } catch (error) {
      _showMessage('渲染编辑页失败：$error');
    } finally {
      _managedRasterBusy = false;
      if (_managedRasterRefreshPending) {
        _managedRasterRefreshPending = false;
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted) _renderManagedEditPage(width, height, force: true);
        });
      }
    }
  }

  Future<void> _showLongPressMenu() async {
    if (_bytes == null) return;
    final selected = await showMenu<String>(
      context: context,
      position: const RelativeRect.fromLTRB(96, 180, 96, 0),
      items: const <PopupMenuEntry<String>>[
        PopupMenuItem(value: 'edit', child: Text('编辑')),
        PopupMenuItem(value: 'copy', child: Text('复制')),
        PopupMenuItem(value: 'annotate', child: Text('批注')),
      ],
    );
    if (selected == 'edit') _selectMode(WorkspaceMode.edit);
    if (selected == 'annotate') _selectMode(WorkspaceMode.annotate);
    if (selected == 'copy') _showMessage('拖动文本选区后使用系统复制菜单');
  }

  bool _canRun(String id) {
    if (widget.commands case final commands?) {
      final command = commands[id];
      if (command == null ||
          widget.enabledFeatures?.contains(command['feature']) != true) {
        return false;
      }
    }
    if (id == 'pdf.file.open') return true;
    if (_bytes == null || _busy) return false;
    return switch (id) {
      'pdf.file.saveCopy' ||
      'pdf.view.thumbs' ||
      'pdf.view.zoomIn' ||
      'pdf.view.zoomOut' ||
      'pdf.view.fitWidth' ||
      'pdf.view.fitPage' ||
      'pdf.view.goPage' ||
      'pdf.view.toggleSpread' ||
      'pdf.file.properties' ||
      'pdf.search.open' => true,
      'pdf.edit.text' => _editBackend.supportsMutation,
      'pdf.edit.undo' => _editBackend.canUndo,
      'pdf.edit.redo' => _editBackend.canRedo,
      'pdf.page.extract' || 'pdf.page.merge' => _pageOperations.isAvailable,
      'pdf.page.split' ||
      'pdf.page.reverse' => _pageOperations.isAvailable && _pageCount > 1,
      'pdf.page.delete' => _pageOperations.isAvailable && _pageCount > 1,
      _ => false,
    };
  }

  void _runProductCommand(String id, Map<String, dynamic> args) {
    if (!_canRun(id)) return;
    switch (id) {
      case 'pdf.file.open':
        unawaited(_openPdf());
      case 'pdf.file.saveCopy':
        unawaited(_saveCopy());
      case 'pdf.edit.undo':
        unawaited(_undo());
      case 'pdf.edit.redo':
        unawaited(_redo());
      case 'pdf.edit.text':
        if (_mode == WorkspaceMode.edit) {
          unawaited(_toggleInlineKeyboard());
        } else {
          unawaited(_selectMode(WorkspaceMode.edit));
        }
      case 'pdf.view.zoomIn':
        _fitMode = null;
        unawaited(_viewerController.zoomUp());
      case 'pdf.view.zoomOut':
        _fitMode = null;
        unawaited(_viewerController.zoomDown());
      case 'pdf.view.fitWidth':
        _fitMode = _PdfFitMode.width;
        _applyCurrentFit();
      case 'pdf.view.fitPage':
        _fitMode = _PdfFitMode.page;
        _applyCurrentFit();
      case 'pdf.view.toggleSpread':
        setState(() {
          _pageArrangement = _pageArrangement == PdfPageArrangement.single
              ? PdfPageArrangement.spread
              : PdfPageArrangement.single;
        });
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted) _applyCurrentFit();
        });
      case 'pdf.view.goPage':
        _showGoToPage();
      case 'pdf.view.thumbs':
        _showMessage('缩略图已显示在左侧；手机上可从状态栏打开');
      case 'pdf.page.extract':
        final selected = (args['pageRange'] as List?)?.cast<int>();
        unawaited(
          _extractPage(
            args['pageNumber'] as int? ?? selected?.first ?? _pageNumber,
            selected,
          ),
        );
      case 'pdf.page.delete':
        unawaited(_deletePage(args['pageNumber'] as int? ?? _pageNumber));
      case 'pdf.page.merge':
        unawaited(_mergePdf());
      case 'pdf.page.split':
        unawaited(_splitPdf(args['pagesPerFile'] as int? ?? 1));
      case 'pdf.page.reverse':
        unawaited(_reversePages());
      case 'pdf.file.properties':
        _showProperties();
      case 'pdf.search.open':
        break;
    }
  }

  void _showGoToPage() {
    final input = TextEditingController(text: '$_pageNumber');
    showDialog<void>(
      context: context,
      builder: (dialogContext) => AlertDialog(
        title: const Text('跳转页面'),
        content: TextField(
          controller: input,
          autofocus: true,
          keyboardType: TextInputType.number,
          decoration: InputDecoration(labelText: '页码（1–$_pageCount）'),
          onSubmitted: (_) {
            Navigator.pop(dialogContext);
            unawaited(_goToPage(int.tryParse(input.text) ?? _pageNumber));
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialogContext),
            child: const Text('取消'),
          ),
          TextButton(
            onPressed: () {
              Navigator.pop(dialogContext);
              unawaited(_goToPage(int.tryParse(input.text) ?? _pageNumber));
            },
            child: const Text('跳转'),
          ),
        ],
      ),
    ).whenComplete(input.dispose);
  }

  void _showProperties() => showDialog<void>(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('文档属性'),
      content: Text(
        '文件：$_sourceName\n页数：$_pageCount\n编辑状态：${_productBackend.isDirty ? '未保存' : '未修改'}',
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('关闭'),
        ),
      ],
    ),
  );

  void _onSearchQuery(String query) {
    _textSearcher?.startTextSearch(query, searchImmediately: false);
  }

  void _onSearchStep(int direction) {
    final searcher = _textSearcher;
    if (searcher == null || searcher.matches.isEmpty) return;
    final current = searcher.currentIndex ?? 0;
    final next =
        (current + direction + searcher.matches.length) %
        searcher.matches.length;
    unawaited(searcher.goToMatchOfIndex(next));
    if (mounted) setState(() {});
  }

  Future<ui.Image?> _renderThumbnail(int index, int width) async {
    if (!_editBackend.isOpen) return null;
    final size = await _productBackend.pageSize(index);
    final height = (width * size.height / size.width).round().clamp(1, 320);
    final raster = await _productBackend.renderPageAt(index, width, height);
    final completer = Completer<ui.Image>();
    ui.decodeImageFromPixels(
      raster.rgba,
      raster.width,
      raster.height,
      ui.PixelFormat.rgba8888,
      completer.complete,
    );
    return completer.future;
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    backgroundColor: widget.chrome.color('canvas'),
    body: SafeArea(
      child: PdfProductShell(
        definition: widget.chrome,
        document: _buildDocument(),
        pageNumber: _pageNumber,
        pageCount: _pageCount,
        fileName: _sourceName,
        documentStatus: PdfDocumentStatus(
          hasDocument: _bytes != null,
          hasPendingChanges: _productBackend.isDirty,
          copySavePhase: _copySavePhase,
          copySaveError: _copySaveError,
        ),
        zoom: _zoom,
        pageArrangement: _pageArrangement,
        isCommandAvailable: _canRun,
        onCommand: _runProductCommand,
        onGoToPage: (page) => unawaited(_goToPage(page)),
        onNavigateDocument: (action) => unawaited(_navigateDocument(action)),
        onZoom: (zoom) {
          _fitMode = null;
          if (_viewerController.isReady) {
            unawaited(_viewerController.setZoom(Offset.zero, zoom));
          }
        },
        onViewportWidthChanged: _onViewportWidthChanged,
        renderThumbnail: _renderThumbnail,
        searchCurrent: (_textSearcher?.currentIndex ?? -1) + 1,
        searchTotal: _textSearcher?.matches.length ?? 0,
        searchBusy: _textSearcher?.isSearching ?? false,
        onSearchQuery: _onSearchQuery,
        onSearchStep: _onSearchStep,
        onSearchClose: () => _textSearcher?.resetTextSearch(),
      ),
    ),
  );

  PdfPageLayout _layoutProductPages(
    List<PdfPage> pages,
    PdfViewerParams params,
  ) {
    final viewport = widget.chrome.pageViewport;
    final geometry = PdfArrangementGeometry.layout(
      [for (final page in pages) Size(page.width, page.height)],
      arrangement: _pageArrangement,
      horizontalPadding: viewport.horizontalPadding,
      verticalPadding: viewport.verticalPadding,
      pageGap: viewport.pageGap,
    );
    return PdfPageLayout(
      pageLayouts: geometry.pageRects,
      documentSize: geometry.documentSize,
    );
  }

  Widget _buildDocument() {
    if (_busy) return const Center(child: CircularProgressIndicator());
    final data = _bytes;
    if (data == null) {
      return Center(
        child: Card(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                const Icon(Icons.picture_as_pdf_outlined, size: 64),
                const SizedBox(height: 16),
                const Text(
                  '打开一个 PDF 开始阅读',
                  style: TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
                ),
                const SizedBox(height: 8),
                const Text('支持触控板、鼠标滚轮、双指缩放、文本选择和分页导航。'),
                const SizedBox(height: 20),
                FilledButton.icon(
                  onPressed: _openPdf,
                  icon: const Icon(Icons.folder_open),
                  label: const Text('打开 PDF'),
                ),
              ],
            ),
          ),
        ),
      );
    }
    final reader = _productReader;
    final viewport = widget.chrome.pageViewport;
    return PdfViewer(
      reader == null
          ? PdfDocumentRefData(
              data,
              sourceName: '$_sourceName#$_sourceRevision',
            )
          : PdfDocumentRefDirect(
              reader,
              autoDispose: false,
              requiresBackendInitialization: false,
            ),
      key: ValueKey<int>(_sourceRevision),
      controller: _viewerController,
      initialPageNumber: _pageNumber,
      params: PdfViewerParams(
        sizeDelegateProvider: _pageArrangement == PdfPageArrangement.spread
            ? _viewerSizePolicy
            : null,
        pagePaintCallbacks: _textSearcher == null
            ? null
            : [_textSearcher!.pageTextMatchPaintCallback],
        backgroundColor: widget.chrome.color('canvas'),
        margin: viewport.horizontalPadding,
        layoutPages: _layoutProductPages,
        pageDropShadow: viewport.pageShadow,
        scaleEnabled: true,
        // Enter belongs to the multiline editor while it owns focus. Leaving
        // pdfrx navigation enabled here also interprets Enter as page advance.
        enableKeyboardNavigation: _mode != WorkspaceMode.edit,
        onLongPressStart: (_) => _showLongPressMenu(),
        onPageChanged: (page) {
          if (page != null && mounted && page != _pageNumber) {
            setState(() {
              _pageNumber = page;
              _editDomains = const <PdfEditDomain>[];
              _activeDomainIndex = null;
              _textFocus = null;
            });
            _inlineFocusNode.unfocus();
            if (_mode == WorkspaceMode.edit) {
              _loadEditPage();
            }
          }
        },
        onDocumentLoadFinished: (_, loadSucceeded) {
          if (loadSucceeded && _viewerController.isReady && mounted) {
            _textSearcher?.dispose();
            _textSearcher = PdfTextSearcher(_viewerController)
              ..addListener(() {
                if (mounted) setState(() {});
              });
            setState(() {
              _pageCount = _viewerController.pageCount;
              _pageNumber = _viewerController.pageNumber ?? 1;
            });
            if (_mode == WorkspaceMode.edit) {
              _loadEditPage();
            }
          }
        },
        pageOverlaysBuilder: (context, pageRect, page) {
          if (_mode != WorkspaceMode.edit || page.pageNumber != _pageNumber) {
            return const <Widget>[];
          }
          _ensureManagedEditPage(pageRect.size);
          final overlays = <Widget>[];
          final managedImage = _managedEditImage;
          if (managedImage != null) {
            overlays.add(
              Positioned.fill(
                child: IgnorePointer(
                  child: RawImage(image: managedImage, fit: BoxFit.fill),
                ),
              ),
            );
          }
          overlays.addAll(
            _editDomains.map((domain) {
              final left = domain.left / page.width * pageRect.width;
              final top =
                  (page.height - domain.top) / page.height * pageRect.height;
              final width =
                  (domain.right - domain.left) / page.width * pageRect.width;
              final height =
                  (domain.top - domain.bottom) / page.height * pageRect.height;
              final content = Stack(
                fit: StackFit.expand,
                children: <Widget>[
                  DecoratedBox(
                    decoration: BoxDecoration(
                      color: const Color(0x142563eb),
                      border: Border.all(
                        color: _activeDomainIndex == domain.index
                            ? const Color(0xff1d4ed8)
                            : const Color(0xff3b82f6),
                        width: _activeDomainIndex == domain.index ? 2 : 1,
                      ),
                    ),
                  ),
                  if (_activeDomainIndex == domain.index)
                    IgnorePointer(
                      child: Opacity(
                        // Flutter Web skips its DOM text input when this is
                        // fully transparent, even after requestFocus().
                        opacity: kIsWeb ? 0.01 : 0,
                        child: TextField(
                          controller: _inlineController,
                          focusNode: _inlineFocusNode,
                          maxLines: null,
                          expands: true,
                          onChanged: _onInlineTextChanged,
                          decoration: const InputDecoration(
                            border: InputBorder.none,
                          ),
                        ),
                      ),
                    ),
                ],
              );
              return Positioned(
                left: left,
                top: top,
                width: width.clamp(2.0, pageRect.width),
                height: height.clamp(2.0, pageRect.height),
                child: PdfOverlayInteractionRegion(
                  onTap: (details) {
                    _beginInlineEdit(
                      domain,
                      details.localPosition,
                      Size(width, height),
                    );
                    return true;
                  },
                  child: content,
                ),
              );
            }),
          );
          final focus = _textFocus;
          final caret = focus?.caret;
          if (caret != null) {
            overlays.add(
              Positioned.fill(
                child: IgnorePointer(
                  child: CustomPaint(
                    painter: _PdfCaretPainter(
                      Offset(
                        caret.bottomX / page.width * pageRect.width,
                        (page.height - caret.bottomY) /
                            page.height *
                            pageRect.height,
                      ),
                      Offset(
                        caret.topX / page.width * pageRect.width,
                        (page.height - caret.topY) /
                            page.height *
                            pageRect.height,
                      ),
                    ),
                  ),
                ),
              ),
            );
          }
          return overlays;
        },
      ),
    );
  }
}

class _PdfCaretPainter extends CustomPainter {
  const _PdfCaretPainter(this.bottom, this.top);
  final Offset bottom, top;

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawLine(
      bottom,
      top,
      Paint()
        ..color = const Color(0xff2563eb)
        ..strokeWidth = 2,
    );
  }

  @override
  bool shouldRepaint(_PdfCaretPainter oldDelegate) =>
      oldDelegate.bottom != bottom || oldDelegate.top != top;
}
