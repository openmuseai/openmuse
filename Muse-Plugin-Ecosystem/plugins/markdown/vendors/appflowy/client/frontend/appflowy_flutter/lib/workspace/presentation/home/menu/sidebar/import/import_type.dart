import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

enum ImportType {
  historyDocument,
  historyDatabase,
  markdownOrText,
  csv,
  afDatabase,
  wordDocx,
  excelXlsx,
  slidesPptx,
  pdfFile;

  @override
  String toString() {
    switch (this) {
      case ImportType.historyDocument:
        return LocaleKeys.importPanel_documentFromV010.tr();
      case ImportType.historyDatabase:
        return LocaleKeys.importPanel_databaseFromV010.tr();
      case ImportType.markdownOrText:
        return LocaleKeys.importPanel_textAndMarkdown.tr();
      case ImportType.csv:
        return LocaleKeys.importPanel_csv.tr();
      case ImportType.afDatabase:
        return LocaleKeys.importPanel_database.tr();
      case ImportType.wordDocx:
        return 'Word (.docx)';
      case ImportType.excelXlsx:
        return 'Excel (.xlsx)';
      case ImportType.slidesPptx:
        return 'Slides (.pptx)';
      case ImportType.pdfFile:
        return 'PDF (.pdf)';
    }
  }

  /// Office plugin catalog id, or null for Document/Database imports.
  String? get officePluginId => switch (this) {
        ImportType.wordDocx => 'word',
        ImportType.excelXlsx => 'excel',
        ImportType.slidesPptx => 'slides',
        ImportType.pdfFile => 'pdf',
        _ => null,
      };

  WidgetBuilder get icon => (context) {
        final FlowySvgData svg;
        switch (this) {
          case ImportType.historyDatabase:
            svg = FlowySvgs.document_s;
          case ImportType.historyDocument:
          case ImportType.csv:
          case ImportType.afDatabase:
            svg = FlowySvgs.board_s;
          case ImportType.markdownOrText:
            svg = FlowySvgs.text_s;
          case ImportType.wordDocx:
          case ImportType.excelXlsx:
          case ImportType.slidesPptx:
          case ImportType.pdfFile:
            svg = FlowySvgs.icon_document_s;
        }

        return FlowySvg(
          svg,
          color: Theme.of(context).colorScheme.tertiary,
        );
      };

  bool get enableOnRelease {
    switch (this) {
      case ImportType.historyDatabase:
      case ImportType.historyDocument:
      case ImportType.afDatabase:
        return kDebugMode;
      default:
        return true;
    }
  }

  List<String> get allowedExtensions {
    switch (this) {
      case ImportType.historyDocument:
        return ['afdoc'];
      case ImportType.historyDatabase:
      case ImportType.afDatabase:
        return ['afdb'];
      case ImportType.markdownOrText:
        return ['md', 'txt'];
      case ImportType.csv:
        return ['csv'];
      case ImportType.wordDocx:
        return ['docx'];
      case ImportType.excelXlsx:
        return ['xlsx'];
      case ImportType.slidesPptx:
        return ['pptx'];
      case ImportType.pdfFile:
        return ['pdf'];
    }
  }

  bool get allowMultiSelect {
    switch (this) {
      case ImportType.historyDocument:
      case ImportType.historyDatabase:
      case ImportType.csv:
      case ImportType.afDatabase:
      case ImportType.markdownOrText:
      case ImportType.wordDocx:
      case ImportType.excelXlsx:
      case ImportType.slidesPptx:
      case ImportType.pdfFile:
        return true;
    }
  }
}

