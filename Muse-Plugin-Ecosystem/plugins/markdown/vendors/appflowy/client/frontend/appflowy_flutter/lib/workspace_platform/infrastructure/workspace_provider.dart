import 'dart:async';
import 'dart:io';

import 'package:appflowy/workspace_platform/domain/workspace_models.dart';

abstract interface class MuseWorkspaceProvider {
  String get id;

  Set<MuseWorkspaceCapability> get capabilities;

  Future<MuseWorkspaceEntry> bind(MuseWorkspaceMount mount);

  Future<List<MuseWorkspaceEntry>> listChildren({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
  });

  Future<MuseWorkspaceEntry> createFile({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required String name,
  });

  Future<MuseWorkspaceEntry> createDirectory({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required String name,
  });

  Future<MuseWorkspaceEntry> rename({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry entry,
    required String newName,
  });

  Future<void> delete({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry entry,
  });

  Future<List<MuseWorkspaceEntry>> importFiles({
    required MuseWorkspaceMount mount,
    required MuseWorkspaceEntry parent,
    required List<File> files,
  });

  Stream<void> watch(MuseWorkspaceMount mount);
}

final class MuseWorkspaceProviderRegistry {
  final Map<String, MuseWorkspaceProvider> _providers = {};

  void register(MuseWorkspaceProvider provider) {
    _providers[provider.id] = provider;
  }

  void unregister(String id) {
    _providers.remove(id);
  }

  MuseWorkspaceProvider require(String id) {
    final provider = _providers[id];
    if (provider == null) {
      throw StateError('Workspace provider not installed: $id');
    }
    return provider;
  }

  List<String> get providerIds => _providers.keys.toList(growable: false);
}
