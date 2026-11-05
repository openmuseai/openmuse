import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/engines/helix.dart';
import 'package:appflowy/plugins/resource_surface/engines/ioffice.dart';
import 'package:appflowy/plugins/resource_surface/engines/open_file_viewer.dart';

/// Installs every built-in editor's format list. Call once at Host start.
void registerBuiltinResourceEngines(MuseResourceEngineRegistry registry) {
  registry
    ..register(iofficeEngineSpec)
    ..register(helixEngineSpec)
    ..register(openFileViewerEngineSpec);
}

MuseResourceEngineRegistry builtinResourceEngineRegistry() {
  final registry = MuseResourceEngineRegistry();
  registerBuiltinResourceEngines(registry);
  return registry;
}
