import 'model.dart';

enum ResourceCodeOperation {
  status,
  enable,
  disable,
  definition,
  references,
  implementation,
  hover,
}

final class ResourceCodeRequest {
  ResourceCodeRequest(
    this.operation, {
    this.version,
    this.line = 0,
    this.character = 0,
  }) {
    if (line < 0 ||
        character < 0 ||
        line > 1000000 ||
        character > 1000000 ||
        operation.index >= ResourceCodeOperation.definition.index &&
            version == null) {
      throw const ResourceFailure('INVALID_QUERY');
    }
  }
  final ResourceCodeOperation operation;
  final ResourceVersion? version;
  final int line, character;
  Map<String, Object?> toJson() => {
    'operation': operation.name,
    if (version != null) 'version': version!.toJson(),
    'line': line,
    'character': character,
  };
  factory ResourceCodeRequest.fromJson(Object? value) {
    final m = resourceObject(value, {
      'operation',
      'version',
      'line',
      'character',
    });
    final op = ResourceCodeOperation.values
        .where((x) => x.name == m['operation'])
        .firstOrNull;
    if (op == null) throw const ResourceFailure('INVALID_QUERY');
    return ResourceCodeRequest(
      op,
      version: m['version'] == null
          ? null
          : ResourceVersion.fromJson(m['version']),
      line: m['line'] as int? ?? 0,
      character: m['character'] as int? ?? 0,
    );
  }
}

final class ResourceCodeTarget {
  ResourceCodeTarget({
    required this.reference,
    required this.label,
    required this.line,
    required this.character,
    required this.endLine,
    required this.endCharacter,
  }) {
    if ([line, character, endLine, endCharacter].any((x) => x < 0) ||
        endLine < line ||
        endLine == line && endCharacter < character)
      throw const ResourceFailure('INVALID_ENVELOPE');
    if (label.length > 4096 || reference.version == null)
      throw const ResourceFailure('INVALID_ENVELOPE');
  }
  final ResourceReference reference;
  final String label;
  final int line, character, endLine, endCharacter;
  Map<String, Object?> toJson() => {
    'reference': reference.toJson(),
    'label': label,
    'line': line,
    'character': character,
    'endLine': endLine,
    'endCharacter': endCharacter,
  };
  factory ResourceCodeTarget.fromJson(Object? value) {
    final m = resourceObject(value, {
      'reference',
      'label',
      'line',
      'character',
      'endLine',
      'endCharacter',
    });
    return ResourceCodeTarget(
      reference: ResourceReference.fromJson(m['reference']),
      label: m['label'] as String,
      line: m['line'] as int,
      character: m['character'] as int,
      endLine: m['endLine'] as int,
      endCharacter: m['endCharacter'] as int,
    );
  }
}

final class ResourceCodeResult {
  const ResourceCodeResult({
    required this.enabled,
    this.configured = true,
    this.instances = 0,
    this.matches = const [],
    this.truncated = false,
    this.hover,
  });
  final bool enabled, configured, truncated;
  final int instances;
  final List<ResourceCodeTarget> matches;
  final String? hover;
  Map<String, Object?> toJson() => {
    'enabled': enabled,
    'configured': configured,
    'instances': instances,
    'matches': matches.map((x) => x.toJson()).toList(),
    'truncated': truncated,
    if (hover != null) 'hover': hover,
  };
  factory ResourceCodeResult.fromJson(Object? value) {
    final m = resourceObject(value, {
      'enabled',
      'configured',
      'instances',
      'matches',
      'truncated',
      'hover',
    });
    final values = m['matches'] as List? ?? [];
    if (values.length > 100 || (m['hover'] as String? ?? '').length > 65536)
      throw const ResourceFailure('INVALID_ENVELOPE');
    return ResourceCodeResult(
      enabled: m['enabled'] == true,
      configured: m['configured'] == true,
      instances: m['instances'] as int? ?? 0,
      matches: values.map(ResourceCodeTarget.fromJson).toList(),
      truncated: m['truncated'] == true,
      hover: m['hover'] as String?,
    );
  }
}

/// Optional resource-owner capability. No host paths, executable commands or
/// language-server protocol escape hatches are accepted from a client.
abstract interface class ResourceCodeNavigationProvider {
  Future<ResourceCodeResult> codeNavigation(
    ResourceReference base,
    ResourceCodeRequest request,
    ResourceAccess access,
  );
}
