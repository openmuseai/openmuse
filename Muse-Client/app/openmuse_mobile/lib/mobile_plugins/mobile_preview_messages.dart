import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

List<Map<String, Object?>> mobilePreviewMessages(List<String> lines) {
  final safe = lines
      .take(3)
      .map((line) => line.length > 100 ? '${line.substring(0, 100)}…' : line)
      .toList(growable: false);
  return [
    {
      'version': 'v0.9',
      'createSurface': {
        'surfaceId': 'main',
        'catalogId': MobileWidgetCatalog.catalogId,
      },
    },
    {
      'version': 'v0.9',
      'updateComponents': {
        'surfaceId': 'main',
        'components': [
          {
            'id': 'root',
            'component': 'Column',
            'children': [for (var i = 0; i < safe.length; i++) 'line$i'],
          },
          for (var i = 0; i < safe.length; i++)
            {'id': 'line$i', 'component': 'Text', 'text': safe[i]},
        ],
      },
    },
  ];
}
