import 'package:flutter/material.dart';
import 'package:word_editor/word_editor.dart';

/// Muse-themed formatting strip. Does not use vendor Word-blue constants.
class WordMuseToolbar extends StatelessWidget {
  const WordMuseToolbar({
    super.key,
    required this.controller,
    this.onImport,
    this.onSave,
    this.canSave = false,
  });

  final WordEditorController controller;
  final VoidCallback? onImport;
  final VoidCallback? onSave;
  final bool canSave;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        return Material(
          color: theme.colorScheme.surfaceContainerHighest,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
            child: Row(
              children: [
                _ToolButton(
                  label: 'Open .docx',
                  onPressed: onImport,
                ),
                _ToolButton(
                  label: 'Save',
                  onPressed: canSave ? onSave : null,
                ),
                const SizedBox(width: 12),
                _ToolButton(
                  label: 'Left',
                  onPressed: () => controller.setAlign(0),
                ),
                _ToolButton(
                  label: 'Center',
                  onPressed: () => controller.setAlign(1),
                ),
                _ToolButton(
                  label: 'Right',
                  onPressed: () => controller.setAlign(2),
                ),
                const SizedBox(width: 12),
                _ToolButton(
                  label: 'A−',
                  onPressed: () => controller.setFontSizePt(
                    (controller.view.fontSizePt - 1).clamp(8, 72),
                  ),
                ),
                _ToolButton(
                  label: 'A+',
                  onPressed: () => controller.setFontSizePt(
                    (controller.view.fontSizePt + 1).clamp(8, 72),
                  ),
                ),
                _ToolButton(
                  label: 'U',
                  onPressed: () =>
                      controller.setUnderline(!controller.view.underline),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class _ToolButton extends StatelessWidget {
  const _ToolButton({required this.label, this.onPressed});

  final String label;
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 2),
      child: TextButton(
        onPressed: onPressed,
        child: Text(label),
      ),
    );
  }
}
