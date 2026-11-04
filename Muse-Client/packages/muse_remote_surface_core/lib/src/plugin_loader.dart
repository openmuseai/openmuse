import 'dart:convert';
import 'dart:io';

import 'package:crypto/crypto.dart';

import 'host.dart';
import 'provider.dart';

final class SurfacePackageCatalog {
  const SurfacePackageCatalog(this.factories);

  final Map<String, RemoteSurfaceProvider Function()> factories;

  void sync(RemoteSurfaceHost host, File manifest, {File? digest}) {
    final text = manifest.readAsStringSync();
    if (digest != null) {
      final expected = digest.readAsStringSync().trim();
      final actual = 'sha256:${sha256.convert(utf8.encode(text))}';
      if (expected != actual) {
        throw StateError('plugin manifest digest mismatch');
      }
    }
    final decoded = jsonDecode(text);
    if (decoded is! Map || decoded['enabled'] is! List) {
      throw StateError('plugin manifest is invalid');
    }
    final enabled = (decoded['enabled'] as List).whereType<String>().toSet();
    for (final id in enabled) {
      if (!factories.containsKey(id)) {
        throw StateError('unknown plugin package $id');
      }
    }
    for (final id in host.registeredPluginIds.toList()) {
      if (factories.containsKey(id) && !enabled.contains(id)) {
        host.unregister(id);
      }
    }
    for (final id in enabled) {
      final factory = factories[id];
      if (factory == null) {
        throw StateError('unknown plugin package $id');
      }
      if (!host.registeredPluginIds.contains(id)) {
        host.register(factory());
      }
    }
  }
}
