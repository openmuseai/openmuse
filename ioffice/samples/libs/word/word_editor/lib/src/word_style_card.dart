import 'package:flutter/material.dart';

import 'word_product_definition.dart';
import 'word_style_snapshot.dart';

/// A GenOffice-sized preview card; it only represents style metadata.
class WordStyleCard extends StatelessWidget {
  const WordStyleCard({
    super.key,
    required this.style,
    required this.label,
    required this.chrome,
    this.snapshot,
    this.active = false,
    this.width = 74,
    this.height = 62,
    this.onTap,
  });

  final WordStyleInfo style;
  final String label;
  final WordChromeDefinition chrome;
  final WordStyleSnapshot? snapshot;
  final bool active;
  final double width;
  final double height;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final fontSize = style.type == 'character'
        ? 15.0
        : (((style.sizeHalfPoints ?? snapshot?.defaultSizeHalfPoints ?? 22) /
                    2 *
                    15 /
                    11)
                .round())
            .clamp(11, 20)
            .toDouble();
    final color = style.color ?? snapshot?.defaultColor;
    final previewColor = color != null && color != 'auto'
        ? WordChromeDefinition.colorFromHex(
            color.startsWith('#') ? color : '#$color')
        : chrome.text;
    final decoration = <TextDecoration>[
      if (style.underline) TextDecoration.underline,
      if (style.strike) TextDecoration.lineThrough,
    ];
    final preview = style.type == 'character' ? 'Aa' : 'Aa文';
    return Tooltip(
      message: onTap == null ? '$label（暂不可用）' : label,
      child: Semantics(
        button: true,
        enabled: onTap != null,
        selected: active,
        label: label,
        child: Material(
          color: active ? chrome.accent.withValues(alpha: 0.09) : chrome.chrome,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(4),
            side: BorderSide(color: active ? chrome.accent : chrome.border),
          ),
          child: InkWell(
            onTap: onTap,
            borderRadius: BorderRadius.circular(4),
            child: SizedBox(
              width: width,
              height: height,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Text(
                    style.caps == 'all' ? preview.toUpperCase() : preview,
                    maxLines: 1,
                    overflow: TextOverflow.clip,
                    style: TextStyle(
                      // A DOCX family name is not necessarily registered with
                      // Flutter on this device. Until FontHost resolves that
                      // family, use the loaded surface font for legible cards.
                      fontFamily:
                          Theme.of(context).textTheme.bodyMedium?.fontFamily,
                      fontSize: fontSize,
                      height: 1.15,
                      fontWeight: style.bold || snapshot?.defaultBold == true
                          ? FontWeight.w700
                          : FontWeight.normal,
                      fontStyle: style.italic || snapshot?.defaultItalic == true
                          ? FontStyle.italic
                          : FontStyle.normal,
                      decoration: decoration.isEmpty
                          ? null
                          : TextDecoration.combine(decoration),
                      color: previewColor,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 3),
                    child: Text(label,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(fontSize: 10.5, color: chrome.muted)),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
