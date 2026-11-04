import 'package:flutter/material.dart';

import 'go_to_reference.dart';
import 'product_definition.dart';

/// Product-configured reference entry. Command dispatch stays with the shell.
final class SpreadsheetGoToDialog extends StatefulWidget {
  const SpreadsheetGoToDialog({
    super.key,
    required this.definition,
    required this.surface,
    required this.ink,
    required this.border,
    required this.focus,
    required this.primary,
    required this.primaryText,
  });

  final SpreadsheetGoToDialogDefinition definition;
  final Color surface;
  final Color ink;
  final Color border;
  final Color focus;
  final Color primary;
  final Color primaryText;

  @override
  State<SpreadsheetGoToDialog> createState() => _SpreadsheetGoToDialogState();
}

final class _SpreadsheetGoToDialogState extends State<SpreadsheetGoToDialog> {
  final _controller = TextEditingController();
  bool _invalid = false;

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _submit() {
    final reference = GoToReference.parse(_controller.text);
    if (reference == null) {
      setState(() => _invalid = true);
      return;
    }
    Navigator.of(context).pop(reference);
  }

  @override
  Widget build(BuildContext context) {
    final definition = widget.definition;
    return Dialog(
      key: const ValueKey('sheet-go-to-dialog'),
      backgroundColor: widget.surface,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(color: widget.border),
      ),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 420),
        child: Padding(
          padding: const EdgeInsets.all(14),
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
              SizedBox(
                height: 150,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      definition.reference,
                      style: TextStyle(
                        color: widget.ink.withValues(alpha: .65),
                        fontSize: 11,
                      ),
                    ),
                    const SizedBox(height: 4),
                    TextField(
                      key: const ValueKey('sheet-go-to-input'),
                      controller: _controller,
                      autofocus: true,
                      style: TextStyle(color: widget.ink, fontSize: 13),
                      decoration: InputDecoration(
                        hintText: definition.placeholder,
                        hintStyle: TextStyle(
                          color: widget.ink.withValues(alpha: .55),
                        ),
                        isDense: true,
                        contentPadding: const EdgeInsets.symmetric(
                          horizontal: 8,
                          vertical: 9,
                        ),
                        enabledBorder: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(4),
                          borderSide: BorderSide(color: widget.border),
                        ),
                        focusedBorder: OutlineInputBorder(
                          borderRadius: BorderRadius.circular(4),
                          borderSide: BorderSide(color: widget.focus, width: 2),
                        ),
                      ),
                      onChanged: (_) => setState(() => _invalid = false),
                      onSubmitted: (_) => _submit(),
                    ),
                    const SizedBox(height: 5),
                    Text(
                      _invalid ? definition.invalidReference : definition.hint,
                      style: TextStyle(
                        color: _invalid
                            ? Colors.redAccent
                            : widget.ink.withValues(alpha: .6),
                        fontSize: 12,
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 10),
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  OutlinedButton(
                    style: OutlinedButton.styleFrom(
                      minimumSize: const Size(68, 32),
                      maximumSize: const Size(double.infinity, 32),
                      side: BorderSide(color: widget.border),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                    ),
                    onPressed: () => Navigator.of(context).pop(),
                    child: Text(
                      definition.cancel,
                      style: TextStyle(color: widget.ink, fontSize: 13),
                    ),
                  ),
                  const SizedBox(width: 8),
                  FilledButton(
                    key: const ValueKey('sheet-go-to-confirm'),
                    style: FilledButton.styleFrom(
                      minimumSize: const Size(68, 32),
                      maximumSize: const Size(double.infinity, 32),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12),
                      ),
                      backgroundColor: widget.primary,
                      foregroundColor: widget.primaryText,
                    ),
                    onPressed: _controller.text.trim().isEmpty ? null : _submit,
                    child: Text(
                      definition.confirm,
                      style: const TextStyle(fontSize: 13),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
