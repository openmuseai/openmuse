import 'model.dart';

/// Handler identifiers are a client protocol. A remote snapshot supplies paths
/// only for handlers installed in this binary, never executable route code.
enum RuntimeRouteHandler { session, login }

final class RuntimeRoute {
  RuntimeRoute._(this.id, this.path, this.handler);
  final String id;
  final String path;
  final RuntimeRouteHandler handler;
}

final class RuntimeNavigation {
  RuntimeNavigation._(this.entryRouteId, this.routes);
  factory RuntimeNavigation.fromJson(Object? value) {
    final data = configObject(value, 'navigation');
    configKeys(data, {'entryRouteId', 'routes'}, 'navigation');
    final entry = configText(data['entryRouteId'], 'entryRouteId');
    final raw = configObject(data['routes'], 'routes');
    if (raw.isEmpty || raw.length > 32) {
      throw const FormatException('Invalid route count');
    }
    final routes = <String, RuntimeRoute>{};
    final paths = <String>{};
    for (final item in raw.entries) {
      if (!RegExp(r'^[a-z][a-zA-Z0-9.]{0,63}$').hasMatch(item.key)) {
        throw const FormatException('Invalid route ID');
      }
      final route = configObject(item.value, 'route');
      configKeys(route, {'path', 'handler'}, 'route');
      final path = configText(route['path'], 'route path');
      if (!path.startsWith('/') ||
          path.startsWith('//') ||
          path.contains(RegExp(r'[\\?#\x00-\x20\s]'))) {
        throw const FormatException('Expected local route path');
      }
      for (final segment in path.split('/')) {
        final decoded = Uri.decodeComponent(segment);
        if (decoded == '.' ||
            decoded == '..' ||
            decoded.contains(RegExp(r'[/\\%?#\x00-\x20\s]'))) {
          throw const FormatException('Unsafe route path');
        }
      }
      if (!paths.add(path)) throw const FormatException('Duplicate route path');
      final handlers = RuntimeRouteHandler.values.where(
        (h) => h.name == route['handler'],
      );
      if (handlers.isEmpty) {
        throw const FormatException('Route handler is not installed');
      }
      routes[item.key] = RuntimeRoute._(item.key, path, handlers.single);
    }
    if (!routes.containsKey(entry)) {
      throw const FormatException('Missing entry route');
    }
    return RuntimeNavigation._(entry, Map.unmodifiable(routes));
  }
  final String entryRouteId;
  final Map<String, RuntimeRoute> routes;
  RuntimeRoute get entry => routes[entryRouteId]!;
  String path(String id) =>
      routes[id]?.path ?? (throw ArgumentError.value(id, 'route ID'));
  RuntimeRoute? resolve(String location) {
    final uri = Uri.tryParse(location);
    if (uri == null || uri.hasAuthority || uri.hasScheme) return null;
    for (final route in routes.values) {
      if (uri.path == route.path) return route;
    }
    return null;
  }
}
