import 'dart:async';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';

/// Transport-neutral observation of native UI facets. Local FFI remains the
/// primary sink; an attached remote carrier may subscribe without owning UI.
final class MuseSurfaceContextFeed {
  static final instance = MuseSurfaceContextFeed();
  final _events =
      StreamController<MuseContextContributionV1>.broadcast(sync: true);
  final _closed = StreamController<String>.broadcast(sync: true);
  final _latest = <String, MuseContextContributionV1>{};
  Stream<MuseContextContributionV1> get contributions => _events.stream;
  Stream<String> get closedSurfaces => _closed.stream;
  void publish(MuseContextContributionV1 context) {
    _latest.removeWhere(
      (_, value) => value.expiresAt <= DateTime.now().millisecondsSinceEpoch,
    );
    if (_latest.length >= 128) _latest.remove(_latest.keys.first);
    _latest['${context.surfaceInstanceRef}/${context.contextType}'] = context;
    _events.add(context);
  }

  void closeSurface(String ref) {
    _latest.removeWhere((_, value) => value.surfaceInstanceRef == ref);
    _closed.add(ref);
  }

  List<MuseContextContributionV1> snapshot(String resourceRef) => _latest.values
      .where(
        (value) =>
            value.scopeRef == resourceRef &&
            value.expiresAt > DateTime.now().millisecondsSinceEpoch,
      )
      .toList(growable: false);
  void clear() {
    for (final ref in _latest.values.map((e) => e.surfaceInstanceRef).toSet()) {
      closeSurface(ref);
    }
  }
}
