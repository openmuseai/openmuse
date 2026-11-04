import 'package:flutter/foundation.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'controller.dart';
import 'client_runtime.dart';

/// Native plugin adapter; HTML consumers receive the same validated AST via
/// namespaceData and the existing host-owned HTML locale bridge.
final class OpenMusePluginLocalizationAdapter
    implements OpenMusePluginLocalizations {
  const OpenMusePluginLocalizationAdapter(this.controller);
  final OpenMuseLocaleController controller;
  @override
  String get locale => controller.tag;
  @override
  String get direction => controller.catalog.option(locale).direction;
  @override
  String get formatLocale => controller.catalog.option(locale).formatLocale;
  @override
  int get revision => controller.catalog.revision;
  @override
  String message(String id, [Map<String, Object?> arguments = const {}]) =>
      controller.catalog.format(locale, id, arguments);
  @override
  Map<String, Object?> namespaceData(String namespace) {
    if (!RegExp(r'^[a-z][a-z0-9_.]*$').hasMatch(namespace)) {
      throw const FormatException('Invalid plugin namespace');
    }
    return controller.catalog.namespaceData(locale, namespace);
  }

  @override
  void addListener(VoidCallback listener) => controller.addListener(listener);
  @override
  void removeListener(VoidCallback listener) =>
      controller.removeListener(listener);
}

final class OpenMusePluginConfigurationAdapter
    implements OpenMusePluginConfiguration {
  const OpenMusePluginConfigurationAdapter(this.runtime);
  final OpenMuseClientRuntime runtime;
  @override
  String get releaseId => runtime.presentation.releaseId;
  @override
  bool feature(String key, {bool fallback = false}) =>
      runtime.presentation.settings.features[key] ?? fallback;
  @override
  T parameter<T extends Object>(String key, {required T fallback}) =>
      runtime.presentation.settings.parameter<T>(key, fallback: fallback);
  @override
  void addListener(VoidCallback listener) => runtime.addListener(listener);
  @override
  void removeListener(VoidCallback listener) =>
      runtime.removeListener(listener);
}
