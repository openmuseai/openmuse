library;

import 'dart:convert';
import 'package:flutter/widgets.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
export 'package:muse_resource_center/muse_resource_center.dart'
    show ResourceReference, ResourceIdentity, ResourceFailure;

/// A portable locator, never an authority grant. The installed surface plugin
/// interprets its selector; neither the document nor collaboration knows it.
final class ResourceEmbedTarget {
  ResourceEmbedTarget({
    required this.reference,
    required String surface,
    required Map<String, String> selector,
  }) : surface = checkedRef(surface),
       selector = Map.unmodifiable(selector) {
    if (selector.isEmpty ||
        selector.length > 16 ||
        selector.entries.any(
          (e) =>
              e.key.isEmpty ||
              e.key.length > 64 ||
              e.value.isEmpty ||
              e.value.length > 512 ||
              e.key.codeUnits.any((c) => c < 32) ||
              e.value.codeUnits.any((c) => c < 32),
        ) ||
        utf8.encode(jsonEncode(toJson())).length > 8192) {
      throw const ResourceFailure('INVALID_EMBED_TARGET');
    }
  }
  final ResourceReference reference;
  final String surface;
  final Map<String, String> selector;
  String get key => jsonEncode(toJson());
  Map<String, Object?> toJson() => {
    'protocol': 'openmuse.resource-embed/v1',
    'reference': reference.toJson(),
    'surface': surface,
    'selector': selector,
  };
  factory ResourceEmbedTarget.fromJson(Object? value) {
    final m = resourceObject(value, {
      'protocol',
      'reference',
      'surface',
      'selector',
    });
    if (m['protocol'] != 'openmuse.resource-embed/v1' ||
        m['surface'] is! String ||
        m['selector'] is! Map ||
        (m['selector'] as Map).entries.any(
          (e) => e.key is! String || e.value is! String,
        )) {
      throw const ResourceFailure('INVALID_EMBED_TARGET');
    }
    return ResourceEmbedTarget(
      reference: ResourceReference.fromJson(m['reference']),
      surface: m['surface'] as String,
      selector: Map<String, String>.from(m['selector'] as Map),
    );
  }
}

/// The parent may narrow interaction, but cannot grant the child permission.
/// Historical parent snapshots must never resolve mutable live children.
final class ResourceEmbedPolicy {
  const ResourceEmbedPolicy({
    required this.allowEditing,
    this.allowLive = true,
  });
  final bool allowEditing, allowLive;
}

/// Bound by the Host to the current resource/account lifetime. The picker must
/// return only visible resources, and build must independently authorize reads
/// and writes. No provider, path, credentials or global Host command is exposed.
final class ResourceEmbedHost {
  const ResourceEmbedHost({required this.build, this.pick});
  final Widget Function(BuildContext, ResourceEmbedTarget, ResourceEmbedPolicy)
  build;
  final Future<ResourceEmbedTarget?> Function(BuildContext, String surface)?
  pick;
}

/// Resolution changes (including removal/revocation by the Host) invalidate
/// existing contributed blocks even when an editor caches its block builders.
final class ResourceEmbedScope extends InheritedWidget {
  const ResourceEmbedScope({
    super.key,
    required this.host,
    required this.allowEditing,
    required super.child,
  });
  final ResourceEmbedHost? host;
  final bool allowEditing;
  static bool editingAllowed(BuildContext context) =>
      context
          .dependOnInheritedWidgetOfExactType<ResourceEmbedScope>()
          ?.allowEditing ??
      false;
  static ResourceEmbedHost? of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<ResourceEmbedScope>()?.host;
  @override
  bool updateShouldNotify(ResourceEmbedScope oldWidget) =>
      host != oldWidget.host || allowEditing != oldWidget.allowEditing;
}
