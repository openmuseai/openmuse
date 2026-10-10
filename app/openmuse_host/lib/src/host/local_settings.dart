import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/material.dart';

/// Local, non-secret Host preferences. Plugin values are namespaced and never
/// interpreted by the Host; credentials remain the responsibility of plugins.
final class OpenMuseLocalSettings extends ChangeNotifier {
  OpenMuseLocalSettings({this.file});

  final File? file;

  /// Serial that keeps two concurrent saves in this isolate from sharing a
  /// temporary name. `DateTime.now()` only advances by about a millisecond on
  /// Windows, so the timestamp alone is not unique enough there and the second
  /// save would consume the first one's file.
  static int _tempSerial = 0;

  ThemeMode themeMode = ThemeMode.system;
  double? sidebarWidth;
  double? assistantWidth;
  bool assistantVisible = true;
  final Map<String, String> _defaultEditors = {};
  final Map<String, Map<String, Object?>> _pluginValues = {};
  String? _accountSubject;

  void setAccountSubject(String? subject) {
    if (_accountSubject == subject) return;
    _accountSubject = subject;
    notifyListeners();
  }

  String _pluginKey(String pluginId) {
    final subject = _accountSubject;
    if (subject == null) return pluginId;
    final digest = sha256.convert(utf8.encode(subject));
    return 'account.$digest/$pluginId';
  }

  Future<void> load() async {
    final source = file;
    if (source == null || !await source.exists()) return;
    try {
      final data = jsonDecode(await source.readAsString());
      if (data is! Map<String, dynamic> || data['version'] != 1) return;
      themeMode = ThemeMode.values.firstWhere(
        (mode) => mode.name == data['themeMode'],
        orElse: () => ThemeMode.system,
      );
      sidebarWidth = _validWidth(data['sidebarWidth'], 180, 520);
      assistantWidth = _validWidth(data['assistantWidth'], 280, 960);
      assistantVisible = data['assistantVisible'] is bool
          ? data['assistantVisible'] as bool
          : true;
      if (data['defaultEditors'] case final Map defaults) {
        for (final entry in defaults.entries) {
          if (entry.key is String && entry.value is String) {
            _defaultEditors[entry.key as String] = entry.value as String;
          }
        }
      }
      if (data['plugins'] case final Map plugins) {
        for (final entry in plugins.entries) {
          if (entry.key is String && entry.value is Map) {
            _pluginValues[entry.key as String] = Map<String, Object?>.from(
              entry.value as Map,
            );
          }
        }
      }
      notifyListeners();
    } on FormatException {
      // A corrupt preferences file must not block local workspace startup.
    } on FileSystemException {
      // Read-only profiles can still launch with defaults.
    }
  }

  static double? _validWidth(Object? value, double min, double max) =>
      value is num && value.isFinite && value >= min && value <= max
      ? value.toDouble()
      : null;

  Map<String, Object?> pluginValues(String pluginId) =>
      Map.unmodifiable(_pluginValues[_pluginKey(pluginId)] ?? const {});

  String? defaultEditorFor(String extension) =>
      _defaultEditors[extension.toLowerCase()];

  Future<void> setDefaultEditor(String extension, String editorId) async {
    _defaultEditors[extension.toLowerCase()] = editorId;
    notifyListeners();
    await save();
  }

  Future<void> updatePluginValues(
    String pluginId,
    Map<String, Object?> values,
  ) async {
    if (!pluginId.startsWith('com.openmuse.')) {
      throw ArgumentError.value(pluginId, 'pluginId');
    }
    _pluginValues[_pluginKey(pluginId)] = Map.of(values);
    notifyListeners();
    await save();
  }

  Future<void> setThemeMode(ThemeMode value) async {
    themeMode = value;
    notifyListeners();
    await save();
  }

  Future<void> setAssistantVisible(bool value) async {
    assistantVisible = value;
    notifyListeners();
    await save();
  }

  void setPaneWidth({double? sidebar, double? assistant}) {
    if (sidebar != null) sidebarWidth = sidebar.clamp(180, 520);
    if (assistant != null) assistantWidth = assistant.clamp(280, 960);
    notifyListeners();
  }

  Future<void> resetPaneWidth({required bool sidebar}) async {
    if (sidebar) {
      sidebarWidth = null;
    } else {
      assistantWidth = null;
    }
    notifyListeners();
    await save();
  }

  Future<void> save() async {
    final target = file;
    if (target == null) return;
    await target.parent.create(recursive: true);
    // Distinct Host instances (for example during an app update handoff) can
    // briefly save the same profile, and so can two concurrent saves inside one
    // Host. A shared `.tmp` name lets one writer rename the other's file and
    // makes the loser fail at startup, so the name carries the pid, the clock
    // and a serial that is unique inside this isolate.
    final temporary = File(
      '${target.path}.tmp.$pid.${DateTime.now().microsecondsSinceEpoch}'
      '.${_tempSerial++}',
    );
    await temporary.writeAsString(
      jsonEncode({
        'version': 1,
        'themeMode': themeMode.name,
        'sidebarWidth': sidebarWidth,
        'assistantWidth': assistantWidth,
        'assistantVisible': assistantVisible,
        'defaultEditors': _defaultEditors,
        'plugins': _pluginValues,
      }),
      flush: true,
    );
    // A concurrent save can hold the destination open long enough for the whole
    // replacement to fail with a sharing violation, so retry it briefly instead
    // of failing the save.
    for (var attempt = 0; ; attempt++) {
      try {
        await _renameOver(temporary, target);
        return;
      } on FileSystemException {
        if (!Platform.isWindows || attempt >= 4) rethrow;
        await Future<void>.delayed(Duration(milliseconds: 20 * (attempt + 1)));
      }
    }
  }

  /// Renames [temporary] onto [target], dropping the destination first on
  /// Windows, which does not replace an existing file atomically.
  Future<void> _renameOver(File temporary, File target) async {
    try {
      await temporary.rename(target.path);
    } on FileSystemException {
      if (!Platform.isWindows || !await target.exists()) rethrow;
      await target.delete();
      await temporary.rename(target.path);
    }
  }
}
