import 'package:flutter/material.dart';

import 'word_product_definition.dart';
import 'word_statistics_snapshot.dart';

/// Low-frequency, read-only document summary. All labels and row order come
/// from the signed product; values come from word-core and its current layout.
class WordStatisticsDialog extends StatelessWidget {
  const WordStatisticsDialog({
    super.key,
    required this.chrome,
    required this.statistics,
  });

  final WordChromeDefinition chrome;
  final WordStatisticsSnapshot statistics;

  @override
  Widget build(BuildContext context) => Dialog(
        key: const Key('word-statistics-dialog'),
        backgroundColor: chrome.chrome,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 440),
          child: SingleChildScrollView(
            child: Padding(
              padding: const EdgeInsets.fromLTRB(24, 22, 24, 22),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(chrome.statisticsTitle,
                      style: TextStyle(
                          color: chrome.text,
                          fontSize: 17,
                          fontWeight: FontWeight.w600)),
                  const SizedBox(height: 16),
                  for (final row in chrome.statisticsRows)
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 4, vertical: 7),
                      decoration: BoxDecoration(
                          border:
                              Border(bottom: BorderSide(color: chrome.border))),
                      child: Row(children: [
                        Expanded(
                            child: Text(row.label,
                                style: TextStyle(
                                    color: chrome.text, fontSize: 13))),
                        Text('${statistics.value(row.id)}',
                            key: Key('word-statistics-${row.id}'),
                            style: TextStyle(
                                color: chrome.text,
                                fontSize: 13,
                                fontWeight: FontWeight.w600)),
                      ]),
                    ),
                  const SizedBox(height: 18),
                  Align(
                    alignment: Alignment.centerRight,
                    child: FilledButton(
                      key: const Key('word-statistics-close'),
                      onPressed: () => Navigator.of(context).pop(),
                      style: FilledButton.styleFrom(
                          backgroundColor: chrome.accent,
                          foregroundColor: Colors.white),
                      child: Text(chrome.statisticsCloseLabel),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );
}
