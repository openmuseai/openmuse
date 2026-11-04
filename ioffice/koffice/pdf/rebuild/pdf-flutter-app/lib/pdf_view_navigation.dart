/// Product-defined key policy. The PDF session owns viewport movement.
enum PdfNavigationAction {
  viewportNext,
  viewportPrevious,
  documentStart,
  documentEnd,
  lineNext,
  linePrevious,
  pageNext,
  pagePrevious,
}

final class PdfKeyboardNavigation {
  PdfKeyboardNavigation.fromJson(Map<String, dynamic>? json)
    : linePixels = (json?['linePixels'] as num?)?.toDouble() ?? 60,
      viewportInset = (json?['viewportInset'] as num?)?.toDouble() ?? 40,
      document = json == null
          ? const {
              'ArrowRight': PdfNavigationAction.pageNext,
              'ArrowLeft': PdfNavigationAction.pagePrevious,
            }
          : _bindings(json['document'] as Map<String, dynamic>?),
      thumbnails = _bindings(json?['thumbnails'] as Map<String, dynamic>?);

  final double linePixels;
  final double viewportInset;
  final Map<String, PdfNavigationAction> document;
  final Map<String, PdfNavigationAction> thumbnails;

  PdfNavigationAction? actionFor(String key, {bool inThumbnails = false}) =>
      (inThumbnails ? thumbnails[key] : null) ?? document[key];

  static Map<String, PdfNavigationAction> _bindings(
    Map<String, dynamic>? json,
  ) {
    final bindings = <String, PdfNavigationAction>{};
    for (final entry in json?.entries ?? <MapEntry<String, dynamic>>[]) {
      for (final action in PdfNavigationAction.values) {
        if (entry.value == action.name) {
          bindings[entry.key] = action;
          break;
        }
      }
    }
    return Map.unmodifiable(bindings);
  }
}
