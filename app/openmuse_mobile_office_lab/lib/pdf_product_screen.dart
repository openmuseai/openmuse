import 'mobile_build_locale.dart';

import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:openmuse_pdf_product/openmuse_pdf_product.dart';
import 'package:path_provider/path_provider.dart';
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;

/// Mobile Host route for a verified PDF product snapshot. PDF bytes are read
/// through the Host's short-lived resource handle before entering the SDK.
final class MobilePdfProductScreen extends StatefulWidget {
  const MobilePdfProductScreen({
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
  State<MobilePdfProductScreen> createState() => _MobilePdfProductScreenState();
}

final class _MobilePdfProductScreenState extends State<MobilePdfProductScreen> {
  VerifiedPdfProduct? _product;
  Uint8List? _bytes;
  Object? _failure;

  @override
  void initState() {
    super.initState();
    _open();
  }

  Future<void> _open() async {
    try {
      final product = await _MobilePdfProductStore.load();
      final bytes = widget.handle == null ? null : await _readDocument();
      if (!mounted) return;
      setState(() {
        _product = product;
        _bytes = bytes;
        _failure = null;
      });
    } on Object catch (error) {
      if (!mounted) return;
      setState(() => _failure = error);
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
      throw StateError('PDF resource permission expired or size exceeded');
    }
    final buffer = BytesBuilder(copy: false);
    const chunk = 512 * 1024;
    for (var start = 0; start < handle.size; start += chunk) {
      final end = (start + chunk).clamp(0, handle.size);
      final part = await widget.ranges!.read(handle, start, end);
      if (part.length != end - start) throw StateError('Incomplete PDF range');
      buffer.add(part);
    }
    return buffer.toBytes();
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
      final product = await _MobilePdfProductStore.install(bytes);
      if (mounted) setState(() => _product = product);
    } on Object catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            (localeStrings.mobileUnableToInstallThePdfPackage73981105(
              value1: error,
            )),
          ),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: Text(widget.title),
      actions: [
        IconButton(
          tooltip: (stringsOf(
            context,
          ).mobileInstallASignedPdfPackage76ca0551()),
          icon: const Icon(Icons.extension),
          onPressed: _installProduct,
        ),
      ],
    ),
    body: _product != null
        ? PdfWorkspace(
            key: ValueKey(_product!.product['version']),
            chrome: _product!.chrome,
            commands: _product!.commands,
            enabledFeatures: _product!.enabledFeatures,
            initialBytes: _bytes,
            initialName: widget.title,
          )
        : _failure == null
        ? const Center(child: CircularProgressIndicator())
        : Center(
            child: Text(
              (stringsOf(
                context,
              ).mobileUnableToLoadThePdfPackage9a77d994(value1: _failure)),
            ),
          ),
  );
}

final class _MobilePdfProductStore {
  static const _asset = '../openmuse_mobile/assets/pdf_product/';

  static Future<VerifiedPdfProduct> load() async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/pdf-product',
    );
    final installed = File('${directory.path}/active.mcpb');
    if (await installed.exists()) {
      try {
        return await VerifiedPdfProduct.loadBundle(
          bundle: await installed.readAsBytes(),
          trustedPublicKeyPem: key,
          expectedKeyId: identity['keyId'] as String,
        );
      } on Object {
        // Keep the signed bootstrap package available after a bad update.
      }
    }
    final data = await rootBundle.load('${_asset}pdf.mcpb');
    return VerifiedPdfProduct.loadBundle(
      bundle: data.buffer.asUint8List(data.offsetInBytes, data.lengthInBytes),
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
      expectedVersion: identity['version'] as String,
    );
  }

  static Future<VerifiedPdfProduct> install(Uint8List bundle) async {
    final identity =
        jsonDecode(await rootBundle.loadString('${_asset}identity.json'))
            as Map;
    final key = await rootBundle.loadString('${_asset}public.pem');
    final product = await VerifiedPdfProduct.loadBundle(
      bundle: bundle,
      trustedPublicKeyPem: key,
      expectedKeyId: identity['keyId'] as String,
    );
    final directory = Directory(
      '${(await getApplicationSupportDirectory()).path}/pdf-product',
    );
    await directory.create(recursive: true);
    final staged = File('${directory.path}/active.mcpb.staged');
    await staged.writeAsBytes(bundle, flush: true);
    await staged.rename('${directory.path}/active.mcpb');
    return product;
  }
}
