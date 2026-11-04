import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_word_product/openmuse_word_product.dart';
import 'package:path/path.dart' as p;
import 'package:word_editor/word_editor.dart';

import 'mcp_market.dart';

const wordProductId = 'io.openmuse/word';
const wordDocxMediaType =
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document';

/// Native Host adapter. Product data is loaded from an installed, signed MCPB;
/// the document model and page drawing remain in ioffice.
final class OpenMuseWordProductPlugin implements OpenMusePlugin {
  OpenMuseWordProductPlugin._(
    this.entry,
    this.product,
    this.definition,
    this.commands,
    this.enabledFeatures,
  ) : descriptor = OpenMusePluginDescriptor(
        id: entry.name,
        name: product['displayName'] as String,
        version: entry.version,
        runtime: OpenMusePluginRuntime.builtIn,
        activationEvents: const ['onFileType:docx'],
        permissions: const {'filesystem.workspace.read'},
        editors: const [
          OpenMuseEditorContribution(
            id: 'io.openmuse/word.native',
            extensions: {'docx'},
            mediaTypes: {wordDocxMediaType},
            priority: 100,
          ),
        ],
      );

  final McpMarketEntry entry;
  final Map<String, dynamic> product;
  final WordProductDefinition definition;
  final Map<String, Map<String, dynamic>> commands;
  final Set<String> enabledFeatures;

  @override
  final OpenMusePluginDescriptor descriptor;

  static Future<OpenMuseWordProductPlugin> load({
    required McpMarketEntry entry,
    required Directory installedDirectory,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
  }) async {
    if (entry.name != wordProductId) {
      throw const FormatException('Not a Word product');
    }
    final snapshot = await VerifiedWordProduct.load(
      read: (relativePath) =>
          File(p.join(installedDirectory.path, relativePath)).readAsBytes(),
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: entry.version,
    );
    return OpenMuseWordProductPlugin._(
      entry,
      snapshot.product,
      snapshot.definition,
      snapshot.commands,
      snapshot.enabledFeatures,
    );
  }

  static Future<OpenMuseWordProductPlugin> loadBundled({
    required File bundleFile,
    required String trustedPublicKeyPem,
    required String expectedKeyId,
    required String expectedVersion,
  }) async {
    final bytes = await bundleFile.readAsBytes();
    final snapshot = await VerifiedWordProduct.loadBundle(
      bundle: bytes,
      trustedPublicKeyPem: trustedPublicKeyPem,
      expectedKeyId: expectedKeyId,
      expectedVersion: expectedVersion,
    );
    final entry = McpMarketEntry(
      name: wordProductId,
      title: snapshot.product['displayName'] as String,
      description: 'Bundled signed Word product resources',
      version: expectedVersion,
      packageUri: bundleFile.uri,
      sha256: sha256.convert(bytes).toString(),
    );
    return OpenMuseWordProductPlugin._(
      entry,
      snapshot.product,
      snapshot.definition,
      snapshot.commands,
      snapshot.enabledFeatures,
    );
  }

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') || resource.extension != 'docx') {
      return const Center(child: Text('请选择本地 DOCX 文件'));
    }
    return WordProductSurface(
      key: ValueKey(resource.uri.toString()),
      resource: resource,
      definition: definition,
      commands: commands,
      enabledFeatures: enabledFeatures,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

class WordProductSurface extends StatefulWidget {
  const WordProductSurface({
    super.key,
    required this.resource,
    required this.definition,
    required this.commands,
    required this.enabledFeatures,
  });

  final OpenMuseResource resource;
  final WordProductDefinition definition;
  final Map<String, Map<String, dynamic>> commands;
  final Set<String> enabledFeatures;

  @override
  State<WordProductSurface> createState() => _WordProductSurfaceState();
}

class _WordProductSurfaceState extends State<WordProductSurface> {
  final WordEditorController _controller = WordEditorController();

  @override
  void initState() {
    super.initState();
    _openFile(File(widget.resource.uri.toFilePath()));
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _openFile(File file) async {
    await _controller.init();
    if (!_controller.engine.ffiReady) return;
    try {
      await _controller.openBytes(
        await file.readAsBytes(),
        name: p.basename(file.path),
      );
      if (Platform.environment.containsKey('OPENMUSE_E2E_OPEN_DOCX') &&
          _controller.envelope != null) {
        debugPrint(
          'OPENMUSE_WORD_RENDER_READY:${_controller.pageCount}:${_controller.envelope!.commands.length}',
        );
      }
    } catch (error) {
      _controller.report('打开失败：$error');
    }
  }

  Future<void> _pickFile() async {
    final choice = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: const ['docx'],
    );
    final path = choice?.files.firstOrNull?.path;
    if (path != null) await _openFile(File(path));
  }

  void _run(String commandId) {
    switch (commandId) {
      case 'word.file.open':
        _pickFile();
      case 'word.view.zoomIn':
        _controller.setZoom(_controller.zoom + 0.1);
      case 'word.view.zoomOut':
        _controller.setZoom(_controller.zoom - 0.1);
    }
  }

  bool _isCommandAvailable(String commandId) {
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
    final command = widget.commands[commandId];
    return command != null &&
        implemented.contains(commandId) &&
        widget.enabledFeatures.contains(command['feature']);
  }

  @override
  Widget build(BuildContext context) => WordProductShell(
    definition: widget.definition,
    controller: _controller,
    isCommandAvailable: _isCommandAvailable,
    onCommand: _run,
    document: WordWorkspace(
      controller: _controller,
      showChrome: false,
      readOnly: true,
      canvasColor: widget.definition.chrome.canvas,
      pageTopInset: widget.definition.chrome.pageTopInset,
      pageBottomInset: widget.definition.chrome.pageBottomInset,
      pageGap: widget.definition.chrome.pageGap,
    ),
  );
}
