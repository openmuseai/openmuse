const bool supportsPlatformDocumentSave = false;

Future<String?> saveGeneratedPdfDesktop(
  String name,
  List<int> bytes, {
  String? configuredDirectory,
}) async => null;

Future<int?> savePdfBatchWithPlatformPicker(
  Map<String, List<int>> files,
) async => null;

Future<bool> savePdfWithPlatformPicker(
  String name,
  Future<void> Function(String path) writeCopy,
) => throw UnsupportedError('Platform document save is unavailable');
