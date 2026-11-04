import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';

/// Local Host preference. The provider speaks the public MCP Registry API.
final class McpRegistryProviders extends ChangeNotifier {
  McpRegistryProviders({required this.file, this.defaultOrigin});

  final File file;
  final Uri? defaultOrigin;
  final List<Uri> _origins = [];
  Uri? _selected;

  List<Uri> get origins => List.unmodifiable(_origins);
  Uri? get selected => _selected;

  static Uri validate(String value) {
    final uri = Uri.parse(value.trim());
    if (uri.host.isEmpty ||
        uri.userInfo.isNotEmpty ||
        uri.hasQuery ||
        uri.hasFragment ||
        (uri.path.isNotEmpty && uri.path != '/') ||
        !(uri.scheme == 'https' ||
            (uri.scheme == 'http' &&
                {'localhost', '127.0.0.1'}.contains(uri.host)))) {
      throw const FormatException('市场地址须为 HTTPS，或本机联调地址');
    }
    return uri.replace(path: uri.path.replaceAll(RegExp(r'/+$'), ''));
  }

  Future<void> load() async {
    _origins.clear();
    _selected = null;
    if (await file.exists()) {
      try {
        final value = jsonDecode(await file.readAsString());
        if (value is Map && value['providers'] is List) {
          for (final raw in value['providers'] as List) {
            if (raw is! String) continue;
            try {
              final uri = validate(raw);
              if (!_origins.contains(uri)) _origins.add(uri);
            } on FormatException {
              // Reject invalid persisted providers.
            }
          }
          final rawSelected = value['selected'];
          if (rawSelected is String) {
            try {
              final uri = validate(rawSelected);
              if (_origins.contains(uri)) _selected = uri;
            } on FormatException {
              // A bad selection falls back to the first usable provider.
            }
          }
        }
      } on Object {
        // Corrupt user preference must not prevent market startup.
      }
    }
    if (defaultOrigin case final candidate?) {
      final uri = validate(candidate.toString());
      if (!_origins.contains(uri)) _origins.add(uri);
    }
    _selected ??= _origins.firstOrNull;
    notifyListeners();
  }

  Future<void> select(String value) async {
    final uri = validate(value);
    if (!_origins.contains(uri)) _origins.add(uri);
    _selected = uri;
    await _save();
  }

  Future<void> remove(String value) async {
    final uri = validate(value);
    _origins.remove(uri);
    if (_selected == uri) _selected = _origins.firstOrNull;
    await _save();
  }

  Future<void> _save() async {
    await file.parent.create(recursive: true);
    final staging = File('${file.path}.tmp');
    await staging.writeAsString(
      jsonEncode({
        'providers': [for (final uri in _origins) uri.toString()],
        'selected': _selected?.toString(),
      }),
      flush: true,
    );
    await staging.rename(file.path);
    notifyListeners();
  }
}
