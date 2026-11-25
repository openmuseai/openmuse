library;

import 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart';

import 'package:openmuse_client_plugins/openmuse_client_plugins.dart';

import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:openmuse_cli_plugin/openmuse_cli_plugin.dart';
import 'package:openmuse_file_viewer/openmuse_file_viewer.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';
import 'package:openmuse_native_text_gate/openmuse_native_text_gate.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_plugin_market/openmuse_plugin_market.dart';
import 'package:flutter/widgets.dart';

export 'src/demo_workspace.dart';
export 'package:openmuse_smart_document_plugins/openmuse_smart_document_plugins.dart' show AppFlowyDocumentWorkspace;

List<OpenMusePlugin> createOpenMuseBuiltInPlugins({
  DshSidecarSupervisor? dshSupervisor,
  Future<bool> Function(PluginInteraction)? routeInteraction,
  Widget? Function(String pluginId)? nativeMcpEditorBuilder,
  AppFlowyDocumentWorkspace? cloudDocuments,
}) => [
  ...createOpenMuseClientPlugins(),
  ...createNativeSmartDocumentPlugins(cloudDocuments: cloudDocuments),
  OpenMuseHelixPlugin(),
  OpenMuseCliPlugin(),
  OpenMusePluginMarket(nativeEditorBuilder: nativeMcpEditorBuilder),
  OpenMuseFileViewerPlugin(),
  OpenMuseDshPlugin(
    supervisor: dshSupervisor,
    routeInteraction: routeInteraction,
  ),
  OpenMuseNativeTextGatePlugin(),
];
