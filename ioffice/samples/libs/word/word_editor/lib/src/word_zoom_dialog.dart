import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'editor_controller.dart';
import 'word_product_definition.dart';

/// View state only. The signed Word product defines the range and presets;
/// selecting a value never sends a plugin command or changes document data.
class WordZoomDialog extends StatefulWidget {
  const WordZoomDialog({
    super.key,
    required this.chrome,
    required this.initialZoom,
    required this.initialFitMode,
    required this.onApply,
    required this.onFit,
  });

  final WordChromeDefinition chrome;
  final double initialZoom;
  final WordZoomFitMode? initialFitMode;
  final ValueChanged<double> onApply;
  final ValueChanged<WordZoomFitMode> onFit;

  @override
  State<WordZoomDialog> createState() => _WordZoomDialogState();
}

class _WordZoomDialogState extends State<WordZoomDialog> {
  late final TextEditingController _percent;
  int? _preset;
  WordZoomFitMode? _fitMode;

  @override
  void initState() {
    super.initState();
    final current = (widget.initialZoom * 100).round();
    _percent = TextEditingController(text: '$current');
    _fitMode = widget.initialFitMode;
    if (_fitMode == null && widget.chrome.zoomPresets.contains(current)) {
      _preset = current;
    }
  }

  @override
  void dispose() {
    _percent.dispose();
    super.dispose();
  }

  void _submit() {
    if (_fitMode case final mode?) {
      widget.onFit(mode);
      Navigator.of(context).pop();
      return;
    }
    final value = _preset ??
        int.tryParse(_percent.text.trim()) ??
        (widget.initialZoom * 100).round();
    final percent =
        value.clamp(widget.chrome.zoomMinPercent, widget.chrome.zoomMaxPercent);
    widget.onApply(percent / 100);
    Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) {
    final c = widget.chrome;
    final choices = <(String, int?, WordZoomFitMode?)>[
      ('${c.zoomPresets[0]}%', c.zoomPresets[0], null),
      ('页宽', null, WordZoomFitMode.pageWidth),
      ('${c.zoomPresets[1]}%', c.zoomPresets[1], null),
      ('文字宽度', null, null),
      ('${c.zoomPresets[2]}%', c.zoomPresets[2], null),
      ('整页', null, WordZoomFitMode.wholePage),
      ('多页', null, WordZoomFitMode.multiplePages),
    ];
    return Dialog(
      key: const Key('word-zoom-dialog'),
      backgroundColor: c.chrome,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 380),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(24, 20, 24, 18),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('缩放',
                  style: TextStyle(
                      fontSize: 19,
                      fontWeight: FontWeight.w600,
                      color: c.text)),
              const SizedBox(height: 16),
              Stack(clipBehavior: Clip.none, children: [
                Container(
                  padding: const EdgeInsets.fromLTRB(10, 14, 10, 10),
                  decoration: BoxDecoration(
                      border: Border.all(color: c.border),
                      borderRadius: BorderRadius.circular(8)),
                  child: GridView.builder(
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    gridDelegate:
                        const SliverGridDelegateWithFixedCrossAxisCount(
                            crossAxisCount: 2,
                            mainAxisExtent: 34,
                            crossAxisSpacing: 8),
                    itemCount: choices.length,
                    itemBuilder: (context, index) {
                      final (label, percent, fitMode) = choices[index];
                      final available = percent != null || fitMode != null;
                      final selected = fitMode != null
                          ? _fitMode == fitMode
                          : _fitMode == null && _preset == percent;
                      return Tooltip(
                        message: available ? label : '$label（暂不可用）',
                        child: Semantics(
                          inMutuallyExclusiveGroup: true,
                          checked: available && selected,
                          enabled: available,
                          child: InkWell(
                            key: Key('word-zoom-choice-$index'),
                            onTap: available
                                ? () => setState(() {
                                      _preset = percent;
                                      _fitMode = fitMode;
                                      if (percent != null) {
                                        _percent.text = '$percent';
                                      }
                                    })
                                : null,
                            child: Row(children: [
                              Icon(
                                  available && selected
                                      ? Icons.radio_button_checked
                                      : Icons.radio_button_unchecked,
                                  size: 16,
                                  color: available
                                      ? c.accent
                                      : c.muted.withValues(alpha: 0.45)),
                              const SizedBox(width: 6),
                              Flexible(
                                  child: Text(label,
                                      maxLines: 1,
                                      overflow: TextOverflow.ellipsis,
                                      style: TextStyle(
                                          fontSize: 13,
                                          color: available
                                              ? c.text
                                              : c.muted
                                                  .withValues(alpha: 0.6)))),
                            ]),
                          ),
                        ),
                      );
                    },
                  ),
                ),
                Positioned(
                    left: 12,
                    top: -8,
                    child: ColoredBox(
                      color: c.chrome,
                      child: Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 4),
                          child: Text('缩放到',
                              style: TextStyle(fontSize: 12, color: c.muted))),
                    )),
              ]),
              const SizedBox(height: 16),
              Row(children: [
                Text('百分比', style: TextStyle(fontSize: 13, color: c.text)),
                const SizedBox(width: 10),
                SizedBox(
                    width: 84,
                    height: 34,
                    child: TextField(
                      key: const Key('word-zoom-percent'),
                      controller: _percent,
                      keyboardType: TextInputType.number,
                      inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                      onChanged: (_) => setState(() {
                        _preset = null;
                        _fitMode = null;
                      }),
                      onSubmitted: (_) => _submit(),
                      textAlign: TextAlign.center,
                      style: TextStyle(fontSize: 13, color: c.text),
                      decoration: const InputDecoration(
                          contentPadding: EdgeInsets.symmetric(horizontal: 4),
                          border: OutlineInputBorder()),
                    )),
                const SizedBox(width: 5),
                Text('%', style: TextStyle(fontSize: 13, color: c.text)),
              ]),
              const SizedBox(height: 18),
              Row(mainAxisAlignment: MainAxisAlignment.end, children: [
                TextButton(
                    onPressed: () => Navigator.of(context).pop(),
                    style: TextButton.styleFrom(foregroundColor: c.text),
                    child: const Text('取消')),
                const SizedBox(width: 8),
                FilledButton(
                    key: const Key('word-zoom-apply'),
                    onPressed: _submit,
                    style: FilledButton.styleFrom(
                        backgroundColor: c.accent,
                        foregroundColor: Colors.white),
                    child: const Text('确定')),
              ]),
            ],
          ),
        ),
      ),
    );
  }
}
