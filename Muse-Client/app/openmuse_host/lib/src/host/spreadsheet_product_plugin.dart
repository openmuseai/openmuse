import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_spreadsheet_product/openmuse_spreadsheet_product.dart';
import 'package:path/path.dart' as p;
import 'package:spreadsheet_editor/spreadsheet_editor.dart';

import 'mcp_market.dart';

const spreadsheetProductId = 'io.openmuse/spreadsheet';

/// Host adapter for the signed product chrome. Workbook bytes and the GridCraft
/// Rust session never enter the installable resource package.
final class OpenMuseSpreadsheetProductPlugin implements OpenMusePlugin {
  OpenMuseSpreadsheetProductPlugin._(this.entry, this.snapshot)
    : descriptor = OpenMusePluginDescriptor(
        id: entry.name,
        name: snapshot.product['displayName'] as String,
        version: entry.version,
        runtime: OpenMusePluginRuntime.builtIn,
        activationEvents: const ['onFileType:xlsx', 'onFileType:csv'],
        permissions: const {'filesystem.workspace.read'},
        editors: const [
          OpenMuseEditorContribution(
            id: 'io.openmuse/spreadsheet.native',
            extensions: {'xlsx', 'csv'},
            mediaTypes: {
              'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
              'text/csv',
            },
            priority: 120,
          ),
        ],
      );

  final McpMarketEntry entry;
  final VerifiedSpreadsheetProduct snapshot;

  @override
  final OpenMusePluginDescriptor descriptor;

  static Future<OpenMuseSpreadsheetProductPlugin> load({
    required McpMarketEntry entry,
    required Directory installedDirectory,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
  }) async {
    if (entry.name != spreadsheetProductId) {
      throw const FormatException('Not a Spreadsheet product');
    }
    final snapshot = await VerifiedSpreadsheetProduct.load(
      read: (path) => File(p.join(installedDirectory.path, path)).readAsBytes(),
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: entry.version,
    );
    return OpenMuseSpreadsheetProductPlugin._(entry, snapshot);
  }

  static Future<OpenMuseSpreadsheetProductPlugin> loadBundled({
    required File bundleFile,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    required String expectedVersion,
  }) async {
    final bytes = await bundleFile.readAsBytes();
    final snapshot = await VerifiedSpreadsheetProduct.loadBundle(
      bundle: bytes,
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: expectedVersion,
    );
    final entry = McpMarketEntry(
      name: spreadsheetProductId,
      title: snapshot.product['displayName'] as String,
      description: 'Bundled signed Spreadsheet product resources',
      version: expectedVersion,
      packageUri: bundleFile.uri,
      sha256: sha256.convert(bytes).toString(),
    );
    return OpenMuseSpreadsheetProductPlugin._(entry, snapshot);
  }

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') ||
        !{'xlsx', 'csv'}.contains(resource.extension)) {
      return const Center(child: Text('请选择本地 XLSX 或 CSV 文件'));
    }
    return _SpreadsheetProductSurface(
      key: ValueKey(resource.uri.toString()),
      file: File(resource.uri.toFilePath()),
      snapshot: snapshot,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _SpreadsheetProductSurface extends StatefulWidget {
  const _SpreadsheetProductSurface({
    super.key,
    required this.file,
    required this.snapshot,
  });
  final File file;
  final VerifiedSpreadsheetProduct snapshot;

  @override
  State<_SpreadsheetProductSurface> createState() =>
      _SpreadsheetProductSurfaceState();
}

final class _SpreadsheetProductSurfaceState
    extends State<_SpreadsheetProductSurface> {
  late final Future<Uint8List> _bytes = widget.file.readAsBytes();
  SpreadsheetEngine? _engine;

  @override
  void dispose() {
    _engine?.dispose();
    super.dispose();
  }

  Future<void> _save(Uint8List bytes, String name) async {
    final target = widget.file;
    final temporary = File('${target.path}.openmuse-tmp');
    await temporary.writeAsBytes(bytes, flush: true);
    await temporary.rename(target.path);
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<Uint8List>(
    future: _bytes,
    builder: (context, result) {
      if (result.hasError) {
        return Center(child: Text('打开工作簿失败：${result.error}'));
      }
      if (!result.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      try {
        _engine ??= GridcraftEngine();
      } on Object catch (error) {
        return Center(child: Text('表格内核加载失败：$error'));
      }
      return SpreadsheetProductShell(
        chrome: widget.snapshot.chrome,
        engine: _engine!,
        initialBytes: result.data!,
        initialName: p.basename(widget.file.path),
        onSave: _save,
      );
    },
  );
}
