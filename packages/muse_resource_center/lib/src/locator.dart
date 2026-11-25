import 'dart:convert';
import 'model.dart';

/// An untrusted hint addressed to an installed resource owner, never authority.
/// Only that owner interprets target/context; the resource layer routes it.
final class ResourceLocator {
  ResourceLocator({
    required this.owner,
    required this.kind,
    this.target = '',
    this.context = const {},
  }) {
    checkedRef(owner);
    checkedRef(kind);
    if (utf8.encode(jsonEncode(toJson())).length > 16384)
      throw const ResourceFailure('INVALID_REFERENCE');
  }
  final String owner, kind, target;
  final Map<String, Object?> context;
  Map<String, Object?> toJson() => {
    'owner': owner,
    'kind': kind,
    'target': target,
    'context': context,
  };
  factory ResourceLocator.fromJson(Object? value) {
    final m = resourceObject(value, {'owner', 'kind', 'target', 'context'});
    return ResourceLocator(
      owner: checkedRef(m['owner']),
      kind: checkedRef(m['kind']),
      target: m['target'] as String? ?? '',
      context: Map<String, Object?>.from(m['context'] as Map? ?? {}),
    );
  }
}

final class ResourceLocatorMatch {
  const ResourceLocatorMatch(this.reference, this.label);
  final ResourceReference reference;

  /// Owner-provided display label, not an executable path.
  final String label;
  Map<String, Object?> toJson() => {
    'reference': reference.toJson(),
    'label': label,
  };
  factory ResourceLocatorMatch.fromJson(Object? value) {
    final m = resourceObject(value, {'reference', 'label'});
    return ResourceLocatorMatch(
      ResourceReference.fromJson(m['reference']),
      m['label'] as String,
    );
  }
}

final class ResourceLocatorResult {
  const ResourceLocatorResult(
    this.matches, {
    this.truncated = false,
    this.line,
    this.column,
  });
  final List<ResourceLocatorMatch> matches;
  final bool truncated;
  final int? line, column;
  Map<String, Object?> toJson() => {
    'matches': matches.map((m) => m.toJson()).toList(),
    'truncated': truncated,
    if (line != null) 'line': line,
    if (column != null) 'column': column,
  };
  factory ResourceLocatorResult.fromJson(Object? value) {
    final m = resourceObject(value, {'matches', 'truncated', 'line', 'column'});
    final values = m['matches'];
    if (values is! List || values.length > 50)
      throw const ResourceFailure('INVALID_ENVELOPE');
    return ResourceLocatorResult(
      values.map(ResourceLocatorMatch.fromJson).toList(),
      truncated: m['truncated'] == true,
      line: m['line'] as int?,
      column: m['column'] as int?,
    );
  }
}

/// Optional provider capability, equally valid for local, cloud and sandbox owners.
abstract interface class ResourceLocatorProvider {
  String get locatorOwner;
  Future<ResourceLocatorResult> locate(
    ResourceLocator locator,
    ResourceAccess access,
  );
}

/// A link is interpreted by the owner of its base resource. Clients never
/// reconstruct a filesystem path or infer a session from display metadata.
abstract interface class ResourceRelativeLocatorProvider {
  Future<ResourceLocatorResult> locateRelative(
    ResourceReference base,
    String target,
    ResourceAccess access,
  );
}
