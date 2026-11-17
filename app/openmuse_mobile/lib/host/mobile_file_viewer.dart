import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Grants the compiled viewer one private local copy, never a Desktop path.
/// The copy lives until the viewer route has finished its closing transition.
Future<bool> openMobileFileViewer({
  required NavigatorState navigator,
  required OpenMusePlugin plugin,
  required OpenMuseResource resource,
  required Uint8List bytes,
  required Directory cacheRoot,
}) async {
  final directory = await cacheRoot.createTemp('openmuse-viewer-');
  try {
    var name = resource.displayName.split(RegExp(r'[/\\]')).last;
    if (name.isEmpty || name == '.' || name == '..') name = 'resource';
    final file = File.fromUri(directory.uri.resolveUri(Uri(path: name)));
    await file.writeAsBytes(bytes, flush: true);
    if (!navigator.mounted) return false;
    final localResource = OpenMuseResource(
      uri: file.uri,
      displayName: resource.displayName,
      mediaType: resource.mediaType,
    );
    final route = MaterialPageRoute<void>(
      settings: const RouteSettings(name: openMuseResourceRouteName),
      builder: (context) => Scaffold(
        appBar: AppBar(title: Text(resource.displayName)),
        body: Column(
          children: [
            if (plugin is OpenMuseEditorBannerContributor)
              (plugin as OpenMuseEditorBannerContributor).buildEditorBanner(
                    context,
                    localResource,
                    canOpenWith: (_) => false,
                    openWith: (_) {},
                  ) ??
                  const SizedBox.shrink(),
            Expanded(child: plugin.buildEditor(context, localResource)),
          ],
        ),
      ),
    );
    debugPrint('WorkspaceOpen plugin=${plugin.descriptor.id}');
    await navigator.push<void>(route);
    await route.completed;
    return true;
  } finally {
    await directory.delete(recursive: true);
  }
}
