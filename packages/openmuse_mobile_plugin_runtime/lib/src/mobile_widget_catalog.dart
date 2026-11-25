import 'dart:convert';

import 'package:a2ui_core/a2ui_core.dart' as a2ui;
import 'package:flutter/material.dart';
import 'package:genui/genui.dart';

/// The one Mobile workbench component profile shared by built-in and market
/// plugins. A plugin composes these primitives; it cannot ship Flutter code.
abstract final class MobileWidgetCatalog {
  static const catalogId = basicCatalogId;
  static const allowedComponents = {'Column', 'Row', 'Text', 'Divider', 'Card'};
  static const maxEnvelopes = 8;
  static const maxComponents = 24;
  static const maxEncodedBytes = 16 * 1024;

  static SurfaceController surface() =>
      SurfaceController(catalogs: [BasicCatalogItems.asNoAssetCatalog()]);

  static void apply(
    SurfaceController target,
    List<Map<String, Object?>> messages,
  ) {
    if (messages.isEmpty || messages.length > maxEnvelopes) {
      throw const FormatException('invalid workbench envelope count');
    }
    if (utf8.encode(jsonEncode(messages)).length > maxEncodedBytes) {
      throw const FormatException('workbench preview too large');
    }
    var componentCount = 0;
    var surfaceCount = 0;
    var updateCount = 0;
    for (final message in messages) {
      if (message['version'] != 'v0.9') {
        throw const FormatException('unsupported workbench A2UI version');
      }
      final create = message['createSurface'];
      final update = message['updateComponents'];
      if ((create == null) == (update == null) ||
          message.keys.any(
            (key) => !const {
              'version',
              'createSurface',
              'updateComponents',
            }.contains(key),
          )) {
        throw const FormatException('unsupported workbench operation');
      }
      if (create is Map &&
          (create['catalogId'] != catalogId || create['surfaceId'] != 'main')) {
        throw const FormatException('invalid workbench surface');
      }
      if (create != null && (create is! Map || ++surfaceCount > 1)) {
        throw const FormatException('invalid workbench surface');
      }
      if (update is Map) {
        updateCount++;
        if (update['surfaceId'] != 'main' || update['components'] is! List) {
          throw const FormatException('invalid workbench components');
        }
        for (final item in update['components'] as List) {
          if (item is! Map ||
              !allowedComponents.contains(item['component']) ||
              item['id'] is! String) {
            throw const FormatException('component outside workbench profile');
          }
          componentCount++;
          if (componentCount > maxComponents) {
            throw const FormatException('workbench preview too complex');
          }
        }
      }
      if (update != null && update is! Map) {
        throw const FormatException('invalid workbench components');
      }
      target.handleMessage(a2ui.A2uiMessage.fromJson(message));
    }
    if (surfaceCount != 1 || updateCount == 0) {
      throw const FormatException('incomplete workbench preview');
    }
  }
}

/// Lifecycle is tied to one card. A snapshot update replaces its surface.
final class MobileWidgetPreview extends StatefulWidget {
  const MobileWidgetPreview({
    super.key,
    required this.messages,
    this.changes,
    this.reload,
  });

  final List<Map<String, Object?>> messages;
  final Listenable? changes;
  final List<Map<String, Object?>> Function()? reload;

  @override
  State<MobileWidgetPreview> createState() => _MobileWidgetPreviewState();
}

final class _MobileWidgetPreviewState extends State<MobileWidgetPreview> {
  SurfaceController? _surface;
  Object? _error;

  @override
  void initState() {
    super.initState();
    widget.changes?.addListener(_refresh);
    _load(widget.messages);
  }

  @override
  void didUpdateWidget(covariant MobileWidgetPreview oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.changes != widget.changes) {
      oldWidget.changes?.removeListener(_refresh);
      widget.changes?.addListener(_refresh);
      _load(widget.messages);
    } else if (widget.changes == null &&
        !identical(oldWidget.messages, widget.messages)) {
      _load(widget.messages);
    }
  }

  void _refresh() {
    if (!mounted) return;
    setState(() => _load(widget.reload?.call() ?? widget.messages));
  }

  void _load(List<Map<String, Object?>> messages) {
    final previous = _surface;
    _surface = null;
    _error = null;
    try {
      final next = MobileWidgetCatalog.surface();
      MobileWidgetCatalog.apply(next, messages);
      _surface = next;
    } on Object catch (error) {
      _error = error;
    }
    previous?.dispose();
  }

  @override
  void dispose() {
    widget.changes?.removeListener(_refresh);
    _surface?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final surface = _surface;
    if (surface == null) {
      return Text(
        '预览不可用',
        style: TextStyle(color: Theme.of(context).colorScheme.error),
        semanticsLabel: '插件预览不可用：$_error',
      );
    }
    return SingleChildScrollView(
      child: Surface(surfaceContext: surface.contextFor('main')),
    );
  }
}
