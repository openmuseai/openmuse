import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/engines/helix.dart';
import 'package:appflowy/plugins/resource_surface/engines/register.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_defaults.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('builtin editors register their own format lists', () {
    final registry = builtinResourceEngineRegistry();
    expect(
      registry[MuseLocalEngine.helix]!.extensions,
      helixSupportedExtensions,
    );
    expect(
      registry[MuseLocalEngine.ioffice]!.extensions,
      contains('docx'),
    );
    expect(registry[MuseLocalEngine.openFileViewer]!.catchAll, isTrue);
  });

  test('priority and catch-all pick iOffice, Helix, then the viewer', () {
    final registry = builtinResourceEngineRegistry();
    expect(registry.resolve('brief.docx'), MuseLocalEngine.ioffice);
    expect(registry.resolve('main.dart'), MuseLocalEngine.helix);
    expect(registry.resolve('diagram.png'), MuseLocalEngine.openFileViewer);
    expect(registry.resolve('unknown.foo'), MuseLocalEngine.openFileViewer);
  });

  test('a stored default wins over the registered priority', () async {
    final defaults = MuseResourceOpenDefaults(storage: _MemoryKV());
    await defaults.setEngine('md', MuseLocalEngine.openFileViewer);
    final registry = builtinResourceEngineRegistry();
    expect(
      registry.resolve('notes.md', preferred: defaults.engineFor('md')),
      MuseLocalEngine.openFileViewer,
    );
    expect(
      registry.resolve('notes.md'),
      MuseLocalEngine.helix,
    );
  });

  test('an engine that does not accept the file cannot be the default', () {
    final registry = builtinResourceEngineRegistry();
    expect(
      registry.resolve('notes.md', preferred: MuseLocalEngine.ioffice),
      MuseLocalEngine.helix,
    );
  });
}

class _MemoryKV implements KeyValueStorage {
  final Map<String, String> values = {};

  @override
  Future<void> clear() async => values.clear();

  @override
  Future<String?> get(String key) async => values[key];

  @override
  Future<T?> getWithFormat<T>(
    String key,
    T Function(String value) formatter,
  ) async {
    final value = await get(key);
    return value == null ? null : formatter(value);
  }

  @override
  Future<void> remove(String key) async => values.remove(key);

  @override
  Future<void> set(String key, String value) async => values[key] = value;
}
