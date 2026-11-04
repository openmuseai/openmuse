import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'product_definition.dart';

/// A validated choice returned to the shell, which owns command dispatch.
final class SpreadsheetZoomChoice {
  const SpreadsheetZoomChoice.percent(this.percent) : fitSelection = false;
  const SpreadsheetZoomChoice.fit() : percent = null, fitSelection = true;

  final int? percent;
  final bool fitSelection;
}

enum _ZoomMode { preset, fitSelection, custom }

/// Product-configured modal for the status-bar zoom percentage.
final class SpreadsheetZoomDialog extends StatefulWidget {
  const SpreadsheetZoomDialog({
    super.key,
    required this.definition,
    required this.currentPercent,
    required this.surface,
    required this.ink,
    required this.border,
    required this.accent,
    required this.primary,
    required this.primaryText,
  });

  final SpreadsheetZoomDialogDefinition definition;
  final int currentPercent;
  final Color surface;
  final Color ink;
  final Color border;
  final Color accent;
  final Color primary;
  final Color primaryText;

  @override
  State<SpreadsheetZoomDialog> createState() => _SpreadsheetZoomDialogState();
}

final class _SpreadsheetZoomDialogState extends State<SpreadsheetZoomDialog> {
  late final TextEditingController _custom;
  late _ZoomMode _mode;
  late int _preset;
  bool _invalid = false;

  @override
  void initState() {
    super.initState();
    _custom = TextEditingController(text: '${widget.currentPercent}');
    _preset = widget.currentPercent;
    _mode = widget.definition.presets.contains(widget.currentPercent)
        ? _ZoomMode.preset
        : _ZoomMode.custom;
  }

  @override
  void dispose() {
    _custom.dispose();
    super.dispose();
  }

  void _apply() {
    if (_mode == _ZoomMode.fitSelection) {
      Navigator.of(context).pop(const SpreadsheetZoomChoice.fit());
      return;
    }
    final value = _mode == _ZoomMode.preset
        ? _preset
        : int.tryParse(_custom.text.trim().replaceFirst(RegExp(r'%$'), ''));
    if (value == null ||
        value < widget.definition.min ||
        value > widget.definition.max) {
      setState(() => _invalid = true);
      return;
    }
    Navigator.of(context).pop(SpreadsheetZoomChoice.percent(value));
  }

  Widget _radio(bool selected) => Container(
    width: 16,
    height: 16,
    decoration: BoxDecoration(
      shape: BoxShape.circle,
      border: Border.all(
        color: selected ? widget.accent : widget.border,
        width: 1.5,
      ),
    ),
    alignment: Alignment.center,
    child: selected
        ? Container(
            width: 7,
            height: 7,
            decoration: BoxDecoration(
              color: widget.accent,
              shape: BoxShape.circle,
            ),
          )
        : null,
  );

  Widget _option({
    required Key key,
    required bool selected,
    required VoidCallback onTap,
    required Widget child,
  }) => InkWell(
    key: key,
    onTap: onTap,
    borderRadius: BorderRadius.circular(4),
    child: SizedBox(
      height: 28,
      child: Row(children: [_radio(selected), const SizedBox(width: 6), child]),
    ),
  );

  @override
  Widget build(BuildContext context) {
    final definition = widget.definition;
    return Dialog(
      key: const ValueKey('sheet-zoom-dialog'),
      backgroundColor: widget.surface,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(color: widget.border),
      ),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 420),
        child: Padding(
          padding: const EdgeInsets.all(14),
          child: SingleChildScrollView(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  definition.title,
                  style: TextStyle(
                    color: widget.ink,
                    fontSize: 14,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 10),
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.fromLTRB(12, 8, 12, 12),
                  decoration: BoxDecoration(
                    border: Border.all(color: widget.border),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        definition.magnification,
                        style: TextStyle(
                          color: widget.ink.withValues(alpha: .7),
                          fontSize: 12,
                        ),
                      ),
                      const SizedBox(height: 6),
                      for (final value in definition.presets)
                        _option(
                          key: ValueKey('sheet-zoom-preset-$value'),
                          selected:
                              _mode == _ZoomMode.preset && _preset == value,
                          onTap: () => setState(() {
                            _mode = _ZoomMode.preset;
                            _preset = value;
                            _invalid = false;
                          }),
                          child: Text(
                            '$value%',
                            style: TextStyle(color: widget.ink, fontSize: 12),
                          ),
                        ),
                      _option(
                        key: const ValueKey('sheet-zoom-fit-selection'),
                        selected: _mode == _ZoomMode.fitSelection,
                        onTap: () => setState(() {
                          _mode = _ZoomMode.fitSelection;
                          _invalid = false;
                        }),
                        child: Text(
                          definition.fitSelection,
                          style: TextStyle(color: widget.ink, fontSize: 12),
                        ),
                      ),
                      _option(
                        key: const ValueKey('sheet-zoom-custom-option'),
                        selected: _mode == _ZoomMode.custom,
                        onTap: () => setState(() => _mode = _ZoomMode.custom),
                        child: Row(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            Text(
                              definition.custom,
                              style: TextStyle(color: widget.ink, fontSize: 12),
                            ),
                            const SizedBox(width: 12),
                            SizedBox(
                              width: 64,
                              height: 26,
                              child: TextField(
                                key: const ValueKey('sheet-zoom-custom-input'),
                                controller: _custom,
                                keyboardType: TextInputType.number,
                                inputFormatters: [
                                  FilteringTextInputFormatter.allow(
                                    RegExp(r'[0-9%]'),
                                  ),
                                ],
                                onTap: () =>
                                    setState(() => _mode = _ZoomMode.custom),
                                onChanged: (_) => setState(() {
                                  _mode = _ZoomMode.custom;
                                  _invalid = false;
                                }),
                                onSubmitted: (_) => _apply(),
                                textAlign: TextAlign.center,
                                style: TextStyle(
                                  color: widget.ink,
                                  fontSize: 12,
                                ),
                                decoration: InputDecoration(
                                  isDense: true,
                                  contentPadding: const EdgeInsets.symmetric(
                                    vertical: 4,
                                  ),
                                  border: OutlineInputBorder(
                                    borderRadius: BorderRadius.circular(4),
                                  ),
                                ),
                              ),
                            ),
                            const SizedBox(width: 4),
                            Text(
                              '%',
                              style: TextStyle(color: widget.ink, fontSize: 12),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
                if (_invalid) ...[
                  const SizedBox(height: 8),
                  Text(
                    definition.rangeError,
                    key: const ValueKey('sheet-zoom-range-error'),
                    style: const TextStyle(
                      color: Color(0xffee7777),
                      fontSize: 12,
                    ),
                  ),
                ],
                const SizedBox(height: 12),
                Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    TextButton(
                      key: const ValueKey('sheet-zoom-cancel'),
                      onPressed: () => Navigator.of(context).pop(),
                      child: Text(
                        definition.cancel,
                        style: TextStyle(color: widget.ink, fontSize: 13),
                      ),
                    ),
                    const SizedBox(width: 8),
                    TextButton(
                      key: const ValueKey('sheet-zoom-confirm'),
                      onPressed: _apply,
                      style: TextButton.styleFrom(
                        backgroundColor: widget.primary,
                        foregroundColor: widget.primaryText,
                        minimumSize: const Size(68, 32),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
                      child: Text(definition.confirm),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
