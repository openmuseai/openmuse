import 'package:flutter/foundation.dart';

/// Read-only Host-owned language context. Does not expose service credentials,
/// change the data region, or let plugin code choose arbitrary download URLs.
abstract interface class OpenMusePluginLocalizations implements Listenable {
  String get locale;
  String get direction;
  String get formatLocale;
  int get revision;
  String message(String id, [Map<String, Object?> arguments = const {}]);
  Map<String, Object?> namespaceData(String namespace);
}

/// Public, read-only runtime parameters. Plugins should use keys prefixed by
/// their manifest ID. This interface confers no permissions or entitlements.
abstract interface class OpenMusePluginConfiguration implements Listenable {
  String get releaseId;
  bool feature(String key, {bool fallback = false});
  T parameter<T extends Object>(String key, {required T fallback});
}
