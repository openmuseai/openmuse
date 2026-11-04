import 'mobile_build_locale.dart';

import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_word_product/openmuse_word_product.dart';
import 'package:path_provider/path_provider.dart';
import 'package:word_editor/word_editor.dart';

/// A mobile Host surface for the same signed product snapshot and Flutter
/// editor used on Desktop. Document bytes remain in the Host's capability
/// boundary; the product package receives neither ResourceHandle nor bytes.
final class MobileWordProductScreen extends StatefulWidget {
  const MobileWordProductScreen({
    super.key,
    required this.title,
    this.handle,
    this.ranges,
    this.generation,
  });

  final String title;
  final ResourceHandle? handle;
  final ResourceRangePort? ranges;
  final int? generation;

  @override
  State<MobileWordProductScreen> createState() =>
      _MobileWordProductScreenState();
}

final class _MobileWordProductScreenState
    extends State<MobileWordProductScreen> {
  final WordEditorController _controller = WordEditorController();
  VerifiedWordProduct? _product;
  String? _failure;
  bool _busy = true;

  @override
  void initState() {
    super.initState();
    _open();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _open() async {
    try {
      final product = await _MobileWordProductStore.load();
      await _controller.init();
      if (!_controller.engine.ffiReady) {
        throw StateError(_controller.status);
      }
      if (widget.handle != null) {
        await _controller.openBytes(await _readDocument(), name: widget.title);
        if (_controller.envelope == null) throw StateError(_controller.status);
      }
      if (!mounted) return;
      setState(() {
        _product = product;
        _failure = null;
        _busy = false;
      });
      if (widget.handle == null) await _pickDocument();
    } on Object catch (error) {
      if (!mounted) return;
      setState(() {
        _failure = '$error';
        _busy = false;
      });
    }
  }

  Future<Uint8List> _readDocument() async {
    final handle = widget.handle!;
    if (handle.audience != 'openmuse-mobile-office' ||
        handle.generation != widget.generation ||
        DateTime.now().millisecondsSinceEpoch >= handle.expiresAtMs ||
        handle.size <= 0 ||
        handle.size > 64 * 1024 * 1024) {
      throw StateError('DOCX resource permission expired or size exceeded');
    }
    final buffer = BytesBuilder(copy: false);
    const chunk = 512 * 1024;
    for (var start = 0; start < handle.size; start += chunk) {
      final end = (start + chunk).clamp(0, handle.size);
      final part = await widget.ranges!.read(handle, start, end);
      if (part.length != end - start) throw StateError('Incomplete DOCX range');
      buffer.add(part);
    }
    return buffer.toBytes();
  }

  Future<void> _pickDocument() async {
    final localeStrings = stringsOf(context);

    try {
      final choice = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: const ['docx'],
        withData: true,
      );
      final file = choice?.files.firstOrNull;
      if (file == null) return;
      if (!file.name.toLowerCase().endsWith('.docx')) {
        throw FormatException(
          (localeStrings.mobileChooseADocxDocument623b72e7()),
        );
      }
      final bytes =
          file.bytes ??
          (file.path == null ? null : await File(file.path!).readAsBytes());
      if (bytes == null) {
        throw StateError(
          (localeStrings.mobileUnableToReadTheDocument4e6f6997()),
        );
      }
      await _controller.openBytes(bytes, name: file.name);
      if (_controller.envelope == null) throw StateError(_controller.status);
    } on Object catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            (localeStrings.mobileUnableToOpenDocxBa4be5f8(value1: error)),
          ),
        ),
      );
    }
  }

  Future<void> _installProduct() async {
    final localeStrings = stringsOf(context);

    final choice = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: const ['mcpb'],
      withData: true,
    );
    final file = choice?.files.firstOrNull;
    if (file == null) return;
    try {
      if (!file.name.toLowerCase().endsWith('.mcpb')) {
        throw FormatException(
          (localeStrings.mobileChooseAMcpbPackageUseOpenDocumentFo889c6ea4()),
        );
      }
      final bytes =
          file.bytes ??
          (file.path == null ? null : await File(file.path!).readAsBytes());
      if (bytes == null) {
        throw StateError(
          (localeStrings.mobileUnableToReadTheFeaturePackage7b18502c()),
        );
      }
      final product = await _MobileWordProductStore.install(bytes);
      if (mounted) setState(() => _product = product);
    } on Object catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            (localeStrings.mobileUnableToInstallTheWordPackage21262110(
              value1: error,
            )),
          ),
        ),
      );
    }
  }

  void _command(String id) {
    switch (id) {
      case 'word.file.open':
        _pickDocument();
      case 'word.view.zoomIn':
        _controller.setZoom(_controller.zoom + 0.1);
      case 'word.view.zoomOut':
        _controller.setZoom(_controller.zoom - 0.1);
    }
  }

  bool _available(String id) {
    const implemented = {
      'word.file.open',
      'word.view.zoomIn',
      'word.view.zoomOut',
      'word.view.zoomDialog',
      'word.view.zoom100',
      'word.view.fitWidth',
      'word.view.fitPage',
      'word.view.fitMultiplePages',
      'word.view.gridlines',
      'word.view.darkMode',
      'word.view.ruler',
      'word.tools.wordCount',
      'word.navigation.goTo',
      'word.view.printLayout',
      'word.view.readMode',
    };
    final command = _product?.commands[id];
    return implemented.contains(id) &&
        command != null &&
        _product!.enabledFeatures.contains(command['feature']);
  }

  @override
  Widget build(BuildContext context) {
    final product = _product;
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        actions: [
          IconButton(
            tooltip: (stringsOf(context).mobileOpenADocxDocument2a94c464()),
            icon: const Icon(Icons.folder_open),
            onPressed: _busy ? null : _pickDocument,
          ),
          IconButton(
            tooltip: (stringsOf(
              context,
            ).mobileInstallASignedWordPackageCc6154ed()),
            icon: const Icon(Icons.extension),
            onPressed: _installProduct,
          ),
        ],
      ),
      body: _busy
          ? const Center(child: CircularProgressIndicator())
          : product == null
          ? Center(
              child: Text(
                _failure ??
                    (stringsOf(
                      context,
                    ).mobileTheWordPackageIsUnavailableB3078b51()),
              ),
            )
          : WordProductShell(
              definition: product.definition,
              controller: _controller,
              isCommandAvailable: _available,
              onCommand: _command,
              document: ListenableBuilder(
                listenable: _controller,
                builder: (context, _) => _controller.envelope == null
                    ? Center(
                        child: FilledButton.icon(
                          onPressed: _pickDocument,
                          icon: const Icon(Icons.folder_open),
                          label: Text(
                            (stringsOf(
                              context,
                            ).mobileOpenADocxDocument2a94c464()),
                          ),
                        ),
                      )
                    : WordWorkspace(
                        controller: _controller,
                        showChrome: false,
                        readOnly: true,
                        canvasColor: product.definition.chrome.canvas,
                        pageTopInset: product.definition.chrome.pageTopInset,
                        pageBottomInset:
                            product.definition.chrome.pageBottomInset,
                        pageGap: product.definition.chrome.pageGap,
                      ),
              ),
            ),
    );
  }
}

final class _MobileWordProductStore {
  static const _asset = '../openmuse_mobile/assets/word_product/';

  static Future<VerifiedWordProduct> load() async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/word-product',
    );
    final installed = File('${directory.path}/active.mcpb');
    if (await installed.exists()) {
      try {
        return await VerifiedWordProduct.loadBundle(
          bundle: await installed.readAsBytes(),
          trustedPublicKeyPem: key,
          expectedKeyId: identity['keyId'] as String,
        );
      } on Object {
        // A damaged update cannot prevent the signed bootstrap package opening.
      }
    }
    final data = await rootBundle.load('${_asset}word.mcpb');
    return VerifiedWordProduct.loadBundle(
      bundle: data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes),
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
      expectedVersion: identity['version'] as String,
    );
  }

  static Future<VerifiedWordProduct> install(Uint8List bundle) async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final verified = await VerifiedWordProduct.loadBundle(
      bundle: bundle,
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
    );
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/word-product',
    );
    await directory.create(recursive: true);
    final staged = File('${directory.path}/active.mcpb.staged');
    await staged.writeAsBytes(bundle, flush: true);
    await staged.rename('${directory.path}/active.mcpb');
    return verified;
  }
}
