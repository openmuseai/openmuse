import 'package:flutter/foundation.dart';

/// Host-owned view selection. This carries no plugin data or access grants.
final class DesktopNavigationController extends ChangeNotifier {
  DesktopNavigationController({String? selected = 'explorer', String? last})
    : _selected = selected,
      _last = last ?? selected ?? 'explorer';
  String? _selected;
  String _last;
  String? get selected => _selected;
  String get last => _last;
  void restore({required String? selected, required String last}) {
    if (_selected == selected && _last == last) return;
    _selected = selected;
    _last = last;
    notifyListeners();
  }

  void select(String id) {
    _selected = _selected == id ? null : id;
    _last = id;
    notifyListeners();
  }

  void toggle() {
    _selected = _selected == null ? _last : null;
    notifyListeners();
  }

  void reconcile(Set<String> available) {
    if (!available.contains(_last)) _last = 'explorer';
    if (_selected != null && !available.contains(_selected)) {
      _selected = 'explorer';
      _last = 'explorer';
    }
  }
}
