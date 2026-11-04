library;

import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_siyuan_plugin/openmuse_siyuan_plugin.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';

List<OpenMusePlugin> createOpenMuseClientPlugins() => [OpenMuseSiYuanPlugin(),OpenMuseResourceNotesPlugin()];
