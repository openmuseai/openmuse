import 'mobile_build_locale.dart';

import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_spreadsheet_product/openmuse_spreadsheet_product.dart';
import 'package:path_provider/path_provider.dart';
import 'package:spreadsheet_editor/spreadsheet_editor.dart';

/// Mobile Host entry. Only the signed chrome is hot installed; the native
/// GridCraft library and Flutter editor surface are shipped with the Host.
final class MobileSpreadsheetProductScreen extends StatefulWidget {
  const MobileSpreadsheetProductScreen({
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
  State<MobileSpreadsheetProductScreen> createState() =>
      _MobileSpreadsheetProductScreenState();
}

final class _MobileSpreadsheetProductScreenState
    extends State<MobileSpreadsheetProductScreen> {
  VerifiedSpreadsheetProduct? _product;
  SpreadsheetEngine? _engine;
  Uint8List? _bytes;
  String? _documentName;
  Object? _failure;

  @override
  void initState() {
    super.initState();
    _open();
  }

  @override
  void dispose() {
    _engine?.dispose();
    super.dispose();
  }

  Future<void> _open() async {
    try {
      final product = await _SpreadsheetProductStore.load();
      final bytes = widget.handle == null ? null : await _readDocument();
      final engine = GridcraftEngine();
      if (!mounted) {
        engine.dispose();
        return;
      }
      setState(() {
        _product = product;
        _bytes = bytes;
        _engine = engine;
        _failure = null;
      });
    } on Object catch (error) {
      if (mounted) setState(() => _failure = error);
    }
  }

  Future<Uint8List> _readDocument() async {
    final handle = widget.handle!;
    if (handle.audience != 'openmuse-mobile-office' ||
        handle.generation != widget.generation ||
        DateTime.now().millisecondsSinceEpoch >= handle.expiresAtMs ||
        handle.size <= 0 ||
        handle.size > 128 * 1024 * 1024 ||
        widget.ranges == null) {
      throw StateError(
        'Spreadsheet resource permission expired or size exceeded',
      );
    }
    final buffer = BytesBuilder(copy: false);
    const chunk = 512 * 1024;
    for (var start = 0; start < handle.size; start += chunk) {
      final end = (start + chunk).clamp(0, handle.size);
      final part = await widget.ranges!.read(handle, start, end);
      if (part.length != end - start) {
        throw StateError('Incomplete spreadsheet range');
      }
      buffer.add(part);
    }
    return buffer.toBytes();
  }

  Future<void> _pickDocument() async {
    final picked = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: const ['xlsx', 'csv'],
      withData: true,
    );
    final file = picked?.files.firstOrNull;
    if (file == null) return;
    final bytes =
        file.bytes ??
        (file.path == null ? null : await File(file.path!).readAsBytes());
    if (bytes == null) return;
    _engine?.dispose();
    if (mounted) {
      setState(() {
        _engine = GridcraftEngine();
        _bytes = bytes;
        _documentName = file.name;
        _failure = null;
      });
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
      final bytes =
          file.bytes ??
          (file.path == null ? null : await File(file.path!).readAsBytes());
      if (bytes == null) {
        throw StateError(
          (localeStrings.mobileUnableToReadTheFeaturePackage7b18502c()),
        );
      }
      final product = await _SpreadsheetProductStore.install(bytes);
      if (mounted) setState(() => _product = product);
    } on Object catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              (localeStrings.mobileUnableToInstallTheSpreadsheetPackage91122c4c(
                value1: error,
              )),
            ),
          ),
        );
      }
    }
  }

  Future<void> _saveLocal(Uint8List bytes, String name) async {
    final localeStrings = stringsOf(context);

    final directory = Directory(
      '${(await getApplicationDocumentsDirectory()).path}/spreadsheets',
    );
    await directory.create(recursive: true);
    final safeName = name.replaceAll(RegExp(r'[^A-Za-z0-9._-]'), '_');
    final target = File(
      '${directory.path}/${safeName.endsWith('.xlsx') ? safeName : '$safeName.xlsx'}',
    );
    final staging = File('${target.path}.staged');
    await staging.writeAsBytes(bytes, flush: true);
    await staging.rename(target.path);
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            (localeStrings.mobileLocalCopySavedF1994442(value1: target.path)),
          ),
        ),
      );
    }
  }

  Future<void> _saveCurrent() async {
    final localeStrings = stringsOf(context);

    final engine = _engine;
    if (engine == null) return;
    try {
      await _saveLocal(engine.export('xlsx'), _documentName ?? widget.title);
    } on Object catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              (localeStrings.mobileUnableToSaveALocalCopyB8d12b7a(
                value1: error,
              )),
            ),
          ),
        );
      }
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: Text(widget.title),
      actions: [
        IconButton(
          tooltip: (stringsOf(context).mobileSaveALocalXlsxCopy0740ebca()),
          icon: const Icon(Icons.save_outlined),
          onPressed: _engine == null ? null : _saveCurrent,
        ),
        IconButton(
          tooltip: (stringsOf(context).mobileOpenXlsxOrCsv67db3684()),
          icon: const Icon(Icons.folder_open),
          onPressed: _pickDocument,
        ),
        IconButton(
          tooltip: (stringsOf(
            context,
          ).mobileInstallASignedSpreadsheetPackage8e3f20c8()),
          icon: const Icon(Icons.extension),
          onPressed: _installProduct,
        ),
      ],
    ),
    body: _product != null && _engine != null
        ? SpreadsheetProductShell(
            key: ValueKey(
              '${_product!.product['version']}.${identityHashCode(_engine)}',
            ),
            chrome: _product!.chrome,
            engine: _engine!,
            initialBytes: _bytes,
            initialName: _documentName ?? widget.title,
            onSave: _saveLocal,
          )
        : _failure == null
        ? const Center(child: CircularProgressIndicator())
        : Center(
            child: Text(
              (stringsOf(
                context,
              ).mobileUnableToLoadTheSpreadsheetPackageOr8d1d8c71(
                value1: _failure,
              )),
            ),
          ),
  );
}

final class _SpreadsheetProductStore {
  static const _asset = '../openmuse_mobile/assets/spreadsheet_product/';

  static Future<VerifiedSpreadsheetProduct> load() async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/spreadsheet-product',
    );
    final installed = File('${directory.path}/active.mcpb');
    if (await installed.exists()) {
      try {
        return await VerifiedSpreadsheetProduct.loadBundle(
          bundle: await installed.readAsBytes(),
          trustedPublicKeyPem: key,
          expectedKeyId: identity['keyId'] as String,
        );
      } on Object {
        /* Fall back to the verified bootstrap package. */
      }
    }
    final data = await rootBundle.load('${_asset}spreadsheet.mcpb');
    return VerifiedSpreadsheetProduct.loadBundle(
      bundle: data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes),
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
      expectedVersion: identity['version'] as String,
    );
  }

  static Future<VerifiedSpreadsheetProduct> install(Uint8List bundle) async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final product = await VerifiedSpreadsheetProduct.loadBundle(
      bundle: bundle,
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
    );
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/spreadsheet-product',
    );
    await directory.create(recursive: true);
    final staged = File('${directory.path}/active.mcpb.staged');
    await staged.writeAsBytes(bundle, flush: true);
    await staged.rename('${directory.path}/active.mcpb');
    return product;
  }
}
