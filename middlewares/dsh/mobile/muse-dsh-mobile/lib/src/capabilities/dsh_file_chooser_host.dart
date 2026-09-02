/// Host-owned WebView file selector. Returns URI strings, never bytes or paths
/// for JavaScript. The embedding app maps Photo Picker / Camera / SAF.
abstract class DshFileChooserHost {
  Future<List<String>> chooseFiles(DshFileChooserRequest request);
}

enum DshFileChooserMode { open, openMultiple, save }

class DshFileChooserRequest {
  const DshFileChooserRequest({
    required this.isCaptureEnabled,
    required this.acceptTypes,
    required this.mode,
    this.filenameHint,
  });

  final bool isCaptureEnabled;
  final List<String> acceptTypes;
  final DshFileChooserMode mode;
  final String? filenameHint;
}
