import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

/// One local editor/viewer and the formats it knows how to open.
///
/// Editors own their [extensions] list and register a spec here. The router
/// and the "打开方式" menu read this table instead of a hardcoded matrix.
final class MuseResourceEngineSpec {
  const MuseResourceEngineSpec({
    required this.engine,
    required this.id,
    required this.label,
    required this.icon,
    required this.extensions,
    this.catchAll = false,
    this.priority = 0,
  });

  final MuseLocalEngine engine;
  final String id;
  final String label;
  final IconData icon;

  /// Lower-case extensions without a leading dot.
  final Set<String> extensions;

  /// When true, this engine can open any file the others do not claim.
  final bool catchAll;

  /// Higher wins when several engines accept the same extension.
  final int priority;

  bool accepts(String extension) =>
      catchAll || extensions.contains(extension.toLowerCase());
}

final class MuseResourceEngineRegistry {
  MuseResourceEngineRegistry();

  final Map<MuseLocalEngine, MuseResourceEngineSpec> _byEngine = {};

  void register(MuseResourceEngineSpec spec) {
    _byEngine[spec.engine] = spec;
  }

  List<MuseResourceEngineSpec> get all {
    final specs = _byEngine.values.toList()
      ..sort((a, b) => b.priority.compareTo(a.priority));
    return specs;
  }

  MuseResourceEngineSpec? operator [](MuseLocalEngine engine) =>
      _byEngine[engine];

  List<MuseResourceEngineSpec> accepting(String extension) =>
      all.where((spec) => spec.accepts(extension)).toList(growable: false);

  static String extensionOf(String path) =>
      p.extension(path).toLowerCase().replaceFirst('.', '');

  MuseLocalEngine resolve(
    String path, {
    MuseLocalEngine? preferred,
  }) {
    final extension = extensionOf(path);
    final matches = accepting(extension);
    if (preferred != null && matches.any((spec) => spec.engine == preferred)) {
      return preferred;
    }
    if (matches.isEmpty) return MuseLocalEngine.openFileViewer;
    return matches.first.engine;
  }
}
