import 'model.dart';

enum AppFeature {
  pluginMarket('plugins.market.enabled'),
  updatePrompts('updates.prompts.enabled');

  const AppFeature(this.key);
  final String key;
}

/// Public configuration values, never secrets or client-side entitlements.
/// Unknown names are safe data for forward compatibility; code must explicitly
/// consume a key to introduce behavior. Known operational knobs are bounded.
final class ClientSettings {
  ClientSettings._(this.features, this.parameters);
  factory ClientSettings.fromJson(Object? value) {
    final data = value == null
        ? <String, dynamic>{}
        : configObject(value, 'settings');
    configKeys(data, {'features', 'parameters'}, 'settings');
    final features = configObject(
      data['features'] ?? <String, dynamic>{},
      'features',
    );
    final parameters = configObject(
      data['parameters'] ?? <String, dynamic>{},
      'parameters',
    );
    if (features.length > 128 ||
        parameters.length > 128 ||
        [...features.keys, ...parameters.keys].any(
          (key) => !RegExp(r'^[a-z][a-zA-Z0-9_.-]{0,127}$').hasMatch(key),
        ) ||
        features.values.any((v) => v is! bool) ||
        parameters.values.any((v) => v is! bool && v is! num && v is! String) ||
        parameters.values.any(
          (v) => (v is num && !v.isFinite) || (v is String && v.length > 4096),
        )) {
      throw const FormatException('Invalid runtime settings');
    }
    final interval = parameters['runtime.refreshIntervalSeconds'];
    if (interval != null &&
        (interval is! int || interval < 60 || interval > 1800)) {
      throw const FormatException('Invalid configuration refresh interval');
    }
    return ClientSettings._(
      Map<String, bool>.unmodifiable(features.cast<String, bool>()),
      Map<String, Object>.unmodifiable(parameters.cast<String, Object>()),
    );
  }
  final Map<String, bool> features;
  final Map<String, Object> parameters;
  bool enabled(AppFeature feature) => features[feature.key] ?? true;
  int get refreshIntervalSeconds =>
      parameters['runtime.refreshIntervalSeconds'] as int? ?? 300;
  T parameter<T extends Object>(String key, {required T fallback}) {
    final value = parameters[key];
    return value is T ? value : fallback;
  }
}
