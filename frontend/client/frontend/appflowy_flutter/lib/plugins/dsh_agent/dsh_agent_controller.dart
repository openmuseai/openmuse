import 'dart:async';

import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/core/config/kv_keys.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/foundation.dart';

class DshAgentController extends ChangeNotifier {
  DshAgentController({
    this.url = 'http://127.0.0.1:3080',
    this.width = 420,
  });

  final String url;
  double width;
  // Closed until restore() so the first home frame has no WKWebView. A native
  // WebView created during that frame covers the Flutter surface (black window).
  bool open = false;
  bool launching = false;
  bool ready = false;
  String? lastError;
  var _restored = false;

  Future<void> restore() async {
    if (_restored) return;
    _restored = true;
    try {
      final kv = getIt<KeyValueStorage>();
      final storedOpen = await kv.get(KVKeys.dshPanelOpen);
      open = storedOpen == null ? true : storedOpen == 'true';
      if (!open) {
        launching = false;
      } else {
        launching = true;
      }
      final storedWidth = await kv.get(KVKeys.dshPanelWidth);
      final parsed = storedWidth == null ? null : double.tryParse(storedWidth);
      if (parsed != null) {
        width = parsed.clamp(320.0, 900.0);
      }
      notifyListeners();
    } catch (_) {}
  }

  void setOpen(bool value) {
    if (open == value) return;
    open = value;
    notifyListeners();
    unawaited(_persist(KVKeys.dshPanelOpen, value.toString()));
  }

  void toggle() => setOpen(!open);

  void setWidth(double value) {
    final next = value.clamp(320.0, 900.0);
    if (width == next) return;
    width = next;
    notifyListeners();
  }

  void persistWidth() {
    unawaited(_persist(KVKeys.dshPanelWidth, width.round().toString()));
  }

  void setLaunching(bool value) {
    if (launching == value) return;
    launching = value;
    notifyListeners();
  }

  void setError(String? value) {
    lastError = value;
    if (value != null) {
      ready = false;
    }
    notifyListeners();
  }

  void setReady(bool value) {
    if (ready == value) return;
    ready = value;
    notifyListeners();
  }

  Future<void> _persist(String key, String value) async {
    try {
      await getIt<KeyValueStorage>().set(key, value);
    } catch (_) {}
  }
}
