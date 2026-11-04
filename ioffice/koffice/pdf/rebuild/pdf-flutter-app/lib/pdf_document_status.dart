/// Document facts supplied by the PDF product session. Chrome only presents
/// them; it does not infer persistence from a button press.
enum PdfCopySavePhase { idle, saving, saved, failed }

final class PdfDocumentStatus {
  const PdfDocumentStatus({
    required this.hasDocument,
    required this.hasPendingChanges,
    this.isReadOnly = false,
    this.copySavePhase = PdfCopySavePhase.idle,
    this.copySaveError,
  });

  final bool hasDocument;
  final bool hasPendingChanges;
  final bool isReadOnly;
  final PdfCopySavePhase copySavePhase;
  final String? copySaveError;
}
