import 'package:flutter/foundation.dart';
import 'dart:convert';

/// Read-only style metadata supplied by the document engine. Product resources
/// may provide the same shape as a preview until the engine exposes a snapshot.
@immutable
class WordStyleInfo {
  const WordStyleInfo({
    required this.styleId,
    required this.name,
    required this.type,
    this.isDefault = false,
    this.qFormat = false,
    this.semiHidden = false,
    this.unhideWhenUsed = false,
    this.linkedCharShell = false,
    this.uiPriority = 0,
    this.fontFamily,
    this.sizeHalfPoints,
    this.color,
    this.bold = false,
    this.italic = false,
    this.underline = false,
    this.strike = false,
    this.caps,
  });

  final String styleId;
  final String name;
  final String type;
  final bool isDefault;
  final bool qFormat;
  final bool semiHidden;
  final bool unhideWhenUsed;
  final bool linkedCharShell;
  final int uiPriority;
  final String? fontFamily;
  final int? sizeHalfPoints;
  final String? color;
  final bool bold;
  final bool italic;
  final bool underline;
  final bool strike;
  final String? caps;

  factory WordStyleInfo.fromJson(Map<String, dynamic> json) => WordStyleInfo(
        styleId: json['styleId'] as String,
        name: json['name'] as String,
        type: json['type'] as String,
        isDefault: json['isDefault'] == true,
        qFormat: json['qFormat'] == true,
        semiHidden: json['semiHidden'] == true,
        unhideWhenUsed: json['unhideWhenUsed'] == true,
        linkedCharShell: json['linkedCharShell'] == true,
        uiPriority: json['uiPriority'] as int? ?? 0,
        fontFamily: json['fontFamily'] as String?,
        sizeHalfPoints: json['sizeHalfPoints'] as int?,
        color: json['color'] as String?,
        bold: json['bold'] == true,
        italic: json['italic'] == true,
        underline: json['underline'] == true,
        strike: json['strike'] == true,
        caps: json['caps'] as String?,
      );
}

/// This snapshot carries neither document ownership nor mutation methods.
/// Its version changes when the engine's document/selection state changes.
@immutable
class WordStyleSnapshot {
  const WordStyleSnapshot({
    required this.styles,
    this.usedStyleIds = const {},
    this.activeStyleKey,
    this.defaultSizeHalfPoints = 22,
    this.defaultColor,
    this.defaultBold = false,
    this.defaultItalic = false,
  });

  final List<WordStyleInfo> styles;
  final Set<String> usedStyleIds;
  final String? activeStyleKey;
  final int defaultSizeHalfPoints;
  final String? defaultColor;
  final bool defaultBold;
  final bool defaultItalic;

  factory WordStyleSnapshot.fromEngineJson(String raw) {
    final json = jsonDecode(raw) as Map<String, dynamic>;
    if (json['schemaVersion'] != 1) {
      throw const FormatException('Unsupported Word style snapshot schema');
    }
    return WordStyleSnapshot(
      styles: (json['styles'] as List)
          .map((style) =>
              WordStyleInfo.fromJson(Map<String, dynamic>.from(style as Map)))
          .toList(growable: false),
      usedStyleIds: Set<String>.from(json['usedStyleIds'] as List? ?? const []),
      activeStyleKey: json['activeStyleKey'] as String?,
      defaultSizeHalfPoints: json['defaultSizeHalfPoints'] as int? ?? 22,
      defaultColor: json['defaultColor'] as String?,
      defaultBold: json['defaultBold'] == true,
      defaultItalic: json['defaultItalic'] == true,
    );
  }

  List<WordStyleInfo> get quickStyles {
    final result = <WordStyleInfo>[];
    bool listable(WordStyleInfo s) =>
        (s.type == 'paragraph' || s.type == 'character') && !s.linkedCharShell;
    for (final style in styles) {
      if (!listable(style)) continue;
      final isDefault = style.type == 'paragraph' && style.isDefault;
      if (!style.qFormat && !isDefault) continue;
      if (style.semiHidden &&
          !isDefault &&
          !(style.unhideWhenUsed && usedStyleIds.contains(style.styleId))) {
        continue;
      }
      result.add(style);
    }
    if (result.every((style) => !style.qFormat)) {
      for (final style in styles) {
        if (listable(style) && !style.semiHidden && !result.contains(style)) {
          result.add(style);
        }
      }
    }
    result.sort((a, b) {
      final priority = a.uiPriority.compareTo(b.uiPriority);
      if (priority != 0) return priority;
      return a.name.toLowerCase().compareTo(b.name.toLowerCase());
    });
    return List.unmodifiable(result);
  }
}
