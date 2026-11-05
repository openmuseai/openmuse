import 'dart:convert';

import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/core/config/kv_keys.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/foundation.dart';

/// Per-extension default local engine ("默认打开方式").
final class MuseResourceOpenDefaults extends ChangeNotifier {
  MuseResourceOpenDefaults({KeyValueStorage? storage}) : _storage = storage;

  final KeyValueStorage? _storage;
  final Map<String, String> _byExtension = {};
  var _loaded = false;

  KeyValueStorage get storage => _storage ?? getIt<KeyValueStorage>();

  Future<void> ensureLoaded() async {
    if (_loaded) return;
    final raw = await storage.get(KVKeys.resourceOpenDefaults);
    if (raw != null && raw.isNotEmpty) {
      try {
        final decoded = jsonDecode(raw);
        if (decoded is Map) {
          for (final entry in decoded.entries) {
            final key = entry.key.toString();
            final value = entry.value?.toString() ?? '';
            if (key.isNotEmpty && value.isNotEmpty) {
              _byExtension[key.toLowerCase()] = value;
            }
          }
        }
      } on Object {
        _byExtension.clear();
      }
    }
    _loaded = true;
  }

  MuseLocalEngine? engineFor(String extension) {
    final raw = _byExtension[extension.toLowerCase()];
    if (raw == null) return null;
    for (final engine in MuseLocalEngine.values) {
      if (engine.name == raw) return engine;
    }
    return null;
  }

  Future<void> setEngine(String extension, MuseLocalEngine engine) async {
    _byExtension[extension.toLowerCase()] = engine.name;
    notifyListeners();
    await storage.set(
      KVKeys.resourceOpenDefaults,
      jsonEncode(_byExtension),
    );
  }
}
