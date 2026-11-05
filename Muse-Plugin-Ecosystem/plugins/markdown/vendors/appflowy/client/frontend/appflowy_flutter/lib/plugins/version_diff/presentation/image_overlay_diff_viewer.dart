import 'package:appflowy/plugins/version_diff/application/image_overlay_diff_service.dart';
import 'package:appflowy/plugins/version_diff/image/image_overlay_diff_provider.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

final class MuseImageOverlayDiffViewer extends StatefulWidget {
  const MuseImageOverlayDiffViewer({super.key, required this.document});

  final MuseImageComparisonDocument document;

  @override
  State<MuseImageOverlayDiffViewer> createState() =>
      _MuseImageOverlayDiffViewerState();
}

final class _MuseImageOverlayDiffViewerState
    extends State<MuseImageOverlayDiffViewer> {
  double _blend = 0.5;

  MuseImageOverlayDiffPayload get _payload => widget.document.diff.payload;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return ColoredBox(
      color: scheme.surface,
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(12, 10, 12, 8),
            child: Wrap(
              spacing: 10,
              runSpacing: 6,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: [
                const Icon(Icons.photo_size_select_actual_outlined, size: 18),
                Text(
                  p.basename(widget.document.file.path),
                  style: Theme.of(context).textTheme.titleSmall,
                ),
                Text(
                  _payload.changed ? '像素已变化' : '像素一致',
                  style: Theme.of(context).textTheme.labelMedium,
                ),
                Text(
                  '基线 ${_short(_payload.beforeDigest)}',
                  style: Theme.of(context).textTheme.labelSmall,
                ),
                Text(
                  '当前 ${_short(_payload.afterDigest)}',
                  style: Theme.of(context).textTheme.labelSmall,
                ),
              ],
            ),
          ),
          Divider(height: 1, color: scheme.outlineVariant),
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: DecoratedBox(
                decoration: BoxDecoration(
                  color: scheme.surfaceContainerHighest,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Stack(
                  fit: StackFit.expand,
                  children: [
                    Image.memory(
                      _payload.beforeBytes,
                      key: const ValueKey('diff-image-before'),
                      fit: BoxFit.contain,
                      gaplessPlayback: true,
                    ),
                    Opacity(
                      opacity: _blend,
                      child: Image.memory(
                        _payload.afterBytes,
                        key: const ValueKey('diff-image-after'),
                        fit: BoxFit.contain,
                        gaplessPlayback: true,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
            child: Row(
              children: [
                const Text('基线'),
                Expanded(
                  child: Slider(
                    key: const ValueKey('diff-image-blend'),
                    value: _blend,
                    onChanged: (value) => setState(() => _blend = value),
                  ),
                ),
                const Text('当前'),
              ],
            ),
          ),
        ],
      ),
    );
  }

  static String _short(String value) =>
      value.length <= 8 ? value : value.substring(0, 8);
}
