/// In-process projection of the focused Word page's extracted text.
///
/// Used by Host catalog/snapshot (P4). Not a write authority.
class WordSnapshotCache {
  WordSnapshotCache._();
  static final WordSnapshotCache instance = WordSnapshotCache._();

  final _text = <String, String>{};

  void put(String viewId, String text) {
    final id = viewId.trim();
    if (id.isEmpty) return;
    _text[id] = text;
  }

  String? textFor(String viewId) => _text[viewId.trim()];

  void remove(String viewId) => _text.remove(viewId.trim());

  void clear() => _text.clear();
}
