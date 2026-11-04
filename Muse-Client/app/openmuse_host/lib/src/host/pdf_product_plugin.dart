import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_pdf_product/openmuse_pdf_product.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;
import 'package:pdf_rebuild_flutter/main.dart' show PdfWorkspace;

import 'mcp_market.dart';
import 'local_settings.dart';

const pdfProductId = 'io.openmuse/pdf';

/// Desktop Host adapter. The installed MCPB supplies signed product chrome;
/// PDF document state and rendering stay in the ioffice product SDK.
final class OpenMusePdfProductPlugin implements OpenMusePlugin {
  OpenMusePdfProductPlugin._(
    this.entry,
    this.snapshot,
    this.settings,
    this.openGeneratedDocument,
  ) : descriptor = OpenMusePluginDescriptor(
        id: entry.name,
        name: snapshot.product['displayName'] as String,
        version: entry.version,
        runtime: OpenMusePluginRuntime.builtIn,
        activationEvents: const ['onFileType:pdf'],
        permissions: const {'filesystem.workspace.read'},
        editors: const [
          OpenMuseEditorContribution(
            id: 'io.openmuse/pdf.native',
            extensions: {'pdf'},
            mediaTypes: {'application/pdf'},
            priority: 110,
          ),
        ],
      );

  final McpMarketEntry entry;
  final VerifiedPdfProduct snapshot;
  final OpenMuseLocalSettings? settings;
  final Future<void> Function(String path)? openGeneratedDocument;

  @override
  final OpenMusePluginDescriptor descriptor;

  static Future<OpenMusePdfProductPlugin> load({
    required McpMarketEntry entry,
    required Directory installedDirectory,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    OpenMuseLocalSettings? settings,
    Future<void> Function(String path)? openGeneratedDocument,
  }) async {
    if (entry.name != pdfProductId) {
      throw const FormatException('Not a PDF product');
    }
    final snapshot = await VerifiedPdfProduct.load(
      read: (path) => File(p.join(installedDirectory.path, path)).readAsBytes(),
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: entry.version,
    );
    return OpenMusePdfProductPlugin._(
      entry,
      snapshot,
      settings,
      openGeneratedDocument,
    );
  }

  static Future<OpenMusePdfProductPlugin> loadBundled({
    required File bundleFile,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    required String expectedVersion,
    OpenMuseLocalSettings? settings,
    Future<void> Function(String path)? openGeneratedDocument,
  }) async {
    final bytes = await bundleFile.readAsBytes();
    final snapshot = await VerifiedPdfProduct.loadBundle(
      bundle: bytes,
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: expectedVersion,
    );
    final entry = McpMarketEntry(
      name: pdfProductId,
      title: snapshot.product['displayName'] as String,
      description: 'Bundled signed PDF product resources',
      version: expectedVersion,
      packageUri: bundleFile.uri,
      sha256: sha256.convert(bytes).toString(),
    );
    return OpenMusePdfProductPlugin._(
      entry,
      snapshot,
      settings,
      openGeneratedDocument,
    );
  }

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') || resource.extension != 'pdf') {
      return const Center(child: Text('请选择本地 PDF 文件'));
    }
    return _PdfProductSurface(
      key: ValueKey(resource.uri.toString()),
      file: File(resource.uri.toFilePath()),
      snapshot: snapshot,
      settings: settings,
      openGeneratedDocument: openGeneratedDocument,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

class _PdfProductSurface extends StatefulWidget {
  const _PdfProductSurface({
    super.key,
    required this.file,
    required this.snapshot,
    this.settings,
    this.openGeneratedDocument,
  });

  final File file;
  final VerifiedPdfProduct snapshot;
  final OpenMuseLocalSettings? settings;
  final Future<void> Function(String path)? openGeneratedDocument;

  @override
  State<_PdfProductSurface> createState() => _PdfProductSurfaceState();
}

class _PdfProductSurfaceState extends State<_PdfProductSurface> {
  late final Future<Uint8List> _bytes = widget.file.readAsBytes();

  @override
  Widget build(BuildContext context) => FutureBuilder<Uint8List>(
    future: _bytes,
    builder: (context, result) {
      if (result.hasError) {
        return Center(child: Text('打开 PDF 失败：${result.error}'));
      }
      if (!result.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      return PdfWorkspace(
        chrome: widget.snapshot.chrome,
        commands: widget.snapshot.commands,
        enabledFeatures: widget.snapshot.enabledFeatures,
        initialBytes: result.data!,
        initialName: p.basename(widget.file.path),
        generatedOutputDirectory: () =>
            widget.settings?.generatedOutputDirectory,
        openGeneratedDocument: widget.openGeneratedDocument,
      );
    },
  );
}
