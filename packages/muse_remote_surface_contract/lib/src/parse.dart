const remoteSurfaceHelloProtocol = 'openmuse.remote-surface/hello/v1';
const remoteSurfaceDescriptorProtocol = 'openmuse.remote-surface/descriptor/v1';
const remoteSurfaceSnapshotProtocol = 'openmuse.remote-surface/snapshot/v1';
const remoteControlRequestProtocol = 'openmuse.remote-control/request/v1';
const remoteControlReceiptProtocol = 'openmuse.remote-control/receipt/v1';
const remoteControlEventProtocol = 'openmuse.remote-control/event/v1';

const remoteSurfaceComponents = <String>{
  'text',
  'markdown',
  'image',
  'gallery',
  'video-player',
  'document',
  'link',
  'list',
  'grid',
  'form',
  'stepper',
  'progress',
  'status',
  'diff',
  'compare',
  'timeline-basic',
  'button',
  'confirmation',
};

const remoteSurfaceModes = <String>{
  'declarative',
  'media',
  'web-snapshot',
  'web-interactive',
};

const remoteActionEffects = <String>{
  'read',
  'workspace-propose',
  'workspace-commit',
  'external-side-effect',
};

const remoteReceiptStatuses = <String>{
  'accepted',
  'denied',
  'conflict',
  'unsupported',
};

const remoteEventStates = <String>{
  'accepted',
  'running',
  'succeeded',
  'failed',
  'outcome_unknown',
  'cancelled',
};

const remoteMediaFormats = <String>{
  'image/jpeg',
  'image/png',
  'video/poster',
  'video/hls',
  'video/mp4',
};

abstract final class RemoteSurfaceLimits {
  static const maxDepth = 6;
  static const maxNodes = 64;
  static const maxTextLength = 2000;
  static const maxControlBytes = 65536;
  static const protocolMajor = 1;
  static const protocolMinor = 0;
}

final class RemoteSurfaceFormatException implements FormatException {
  const RemoteSurfaceFormatException(this.message, {this.code});

  @override
  final String message;
  final String? code;

  @override
  Object? get source => null;

  @override
  int? get offset => null;

  @override
  String toString() => 'RemoteSurfaceFormatException: $message';
}

final class RemoteSurfaceTreeRejection implements Exception {
  const RemoteSurfaceTreeRejection(this.code);

  final String code;

  @override
  String toString() => 'RemoteSurfaceTreeRejection: $code';
}

bool remoteComponentSupported(String type, Iterable<String> components) =>
    remoteSurfaceComponents.contains(type) && components.contains(type);

Map<String, Object?> objectMap(Object? value, String field) {
  if (value is! Map) {
    throw RemoteSurfaceFormatException('$field must be an object');
  }
  final result = <String, Object?>{};
  for (final entry in value.entries) {
    if (entry.key is! String) {
      throw RemoteSurfaceFormatException('$field must use string keys');
    }
    result[entry.key as String] = entry.value;
  }
  return result;
}

List<Object?> objectList(Object? value, String field) {
  if (value is! List) {
    throw RemoteSurfaceFormatException('$field must be an array');
  }
  return List<Object?>.of(value);
}

void requireProtocol(Map<String, Object?> map, String protocol) {
  if (map['protocol'] != protocol) {
    throw RemoteSurfaceFormatException('protocol must be $protocol');
  }
}

void requireKeys(
  Map<String, Object?> map, {
  required Set<String> required,
  Set<String> optional = const {},
}) {
  final missing = required.difference(map.keys.toSet());
  if (missing.isNotEmpty) {
    throw RemoteSurfaceFormatException('missing fields: ${missing.join(', ')}');
  }
  final unknown = map.keys.toSet().difference(required.union(optional));
  if (unknown.isNotEmpty) {
    throw RemoteSurfaceFormatException('unknown fields: ${unknown.join(', ')}');
  }
  for (final key in map.keys) {
    rejectForbiddenKey(key, key);
  }
}

void rejectForbiddenKey(String key, String field) {
  const forbidden = {
    'path',
    'filepath',
    'url',
    'uri',
    'cookie',
    'secret',
    'widget',
    'darttype',
    'method',
    'src',
    'actor',
    'device',
    'mobiledevice',
    'desktopdevice',
  };
  if (forbidden.contains(key.toLowerCase())) {
    throw RemoteSurfaceFormatException(
      '$field is not allowed on this contract',
    );
  }
}

String stringField(
  Map<String, Object?> map,
  String field, {
  int max = RemoteSurfaceLimits.maxTextLength,
}) {
  final value = map[field];
  if (value is! String || value.isEmpty) {
    throw RemoteSurfaceFormatException('$field must be a non-empty string');
  }
  rejectDangerousText(value, field, max: max);
  return value;
}

String? optionalString(
  Map<String, Object?> map,
  String field, {
  int max = RemoteSurfaceLimits.maxTextLength,
}) {
  if (!map.containsKey(field) || map[field] == null) return null;
  return stringField(map, field, max: max);
}

int intField(
  Map<String, Object?> map,
  String field, {
  required int min,
  int max = 9007199254740991,
}) {
  final value = map[field];
  if (value is! int || value < min || value > max) {
    throw RemoteSurfaceFormatException(
      '$field must be an integer from $min to $max',
    );
  }
  return value;
}

bool boolField(Map<String, Object?> map, String field) {
  final value = map[field];
  if (value is! bool) {
    throw RemoteSurfaceFormatException('$field must be a boolean');
  }
  return value;
}

void rejectDangerousText(
  String value,
  String field, {
  int max = RemoteSurfaceLimits.maxTextLength,
}) {
  if (value.length > max) {
    throw RemoteSurfaceFormatException('$field exceeds $max characters');
  }
  for (final unit in value.codeUnits) {
    if (unit < 0x20 && unit != 0x0a && unit != 0x09) {
      throw RemoteSurfaceFormatException('$field contains a control character');
    }
  }
  final lower = value.toLowerCase();
  const banned = ['://', 'localhost', '127.0.0.1'];
  for (final item in banned) {
    if (lower.contains(item)) {
      throw RemoteSurfaceFormatException('$field contains a forbidden locator');
    }
  }
}

final _pluginId = RegExp(r'^[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)+$');
final _surfaceId = RegExp(r'^[a-z][a-z0-9-]{0,63}$');
final _actionId = RegExp(r'^[a-z][a-z0-9.-]{0,80}$');
final _opaqueRef = RegExp(r'^[A-Za-z0-9][A-Za-z0-9._:-]{0,128}$');
final _nodeId = RegExp(r'^[A-Za-z][A-Za-z0-9._-]{0,63}$');
final _mediaHandle = RegExp(r'^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$');
final _permission = RegExp(r'^[a-z][a-z0-9.-]{0,80}$');
final _errorCode = RegExp(r'^[A-Z][A-Z0-9_]{1,63}$');
final _fieldId = RegExp(r'^[A-Za-z][A-Za-z0-9_-]{0,64}$');
final _componentType = RegExp(r'^[a-z][a-z0-9-]{0,32}$');

String pluginIdField(Map<String, Object?> map, String field) =>
    _matched(stringField(map, field, max: 160), _pluginId, field);

String surfaceIdField(Map<String, Object?> map, String field) =>
    _matched(stringField(map, field, max: 64), _surfaceId, field);

String actionIdValue(String value, String field) =>
    _matched(value, _actionId, field);

String opaqueField(Map<String, Object?> map, String field) =>
    _matched(stringField(map, field, max: 129), _opaqueRef, field);

String nodeIdValue(String value, String field) =>
    _matched(value, _nodeId, field);

String mediaHandleValue(String value, String field) =>
    _matched(value, _mediaHandle, field);

String permissionValue(String value, String field) =>
    _matched(value, _permission, field);

String errorCodeField(Map<String, Object?> map, String field) =>
    _matched(stringField(map, field, max: 64), _errorCode, field);

String fieldIdValue(String value, String field) =>
    _matched(value, _fieldId, field);

String componentTypeValue(String value, String field) =>
    _matched(value, _componentType, field);

String _matched(String value, RegExp pattern, String field) {
  if (!pattern.hasMatch(value)) {
    throw RemoteSurfaceFormatException('$field has an invalid identifier');
  }
  return value;
}

List<String> stringList(
  Map<String, Object?> map,
  String field, {
  required int max,
  bool unique = true,
  bool allowEmpty = false,
}) {
  final result = objectList(map[field], field)
      .map((value) {
        if (value is! String || value.isEmpty) {
          throw RemoteSurfaceFormatException(
            '$field must contain non-empty strings',
          );
        }
        rejectDangerousText(value, field);
        return value;
      })
      .toList(growable: false);
  if (result.isEmpty && !allowEmpty) {
    throw RemoteSurfaceFormatException('$field must not be empty');
  }
  if (result.length > max) {
    throw RemoteSurfaceFormatException('$field exceeds $max items');
  }
  if (unique && result.toSet().length != result.length) {
    throw RemoteSurfaceFormatException('$field must contain unique values');
  }
  return result;
}

List<String> permissionList(Map<String, Object?> map, String field) {
  final values = objectList(map[field], field);
  if (values.length > 16) {
    throw RemoteSurfaceFormatException('$field exceeds 16 items');
  }
  final result = <String>[];
  for (final value in values) {
    if (value is! String) {
      throw RemoteSurfaceFormatException('$field must contain strings');
    }
    rejectDangerousText(value, field, max: 81);
    result.add(permissionValue(value, field));
  }
  if (result.toSet().length != result.length) {
    throw RemoteSurfaceFormatException('$field must contain unique values');
  }
  return result;
}

Object? deepCopyJson(Object? value) {
  if (value == null || value is String || value is bool || value is int) {
    return value;
  }
  if (value is Map) {
    final result = <String, Object?>{};
    for (final entry in value.entries) {
      if (entry.key is! String) {
        throw const RemoteSurfaceFormatException('JSON keys must be strings');
      }
      result[entry.key as String] = deepCopyJson(entry.value);
    }
    return result;
  }
  if (value is List) {
    return [for (final item in value) deepCopyJson(item)];
  }
  throw const RemoteSurfaceFormatException('unsupported JSON value');
}

String canonicalJson(Object? value) {
  if (value is Map) {
    final entries = value.entries.toList()
      ..sort((a, b) => a.key.toString().compareTo(b.key.toString()));
    return '{${entries.map((entry) => '${jsonString(entry.key.toString())}:${canonicalJson(entry.value)}').join(',')}}';
  }
  if (value is List) {
    return '[${value.map(canonicalJson).join(',')}]';
  }
  if (value is String) return jsonString(value);
  if (value == null) return 'null';
  if (value is bool || value is int) return '$value';
  throw const RemoteSurfaceFormatException('unsupported JSON value');
}

String jsonString(String value) {
  final buffer = StringBuffer('"');
  for (final unit in value.codeUnits) {
    switch (unit) {
      case 0x22:
        buffer.write(r'\"');
      case 0x5c:
        buffer.write(r'\\');
      case 0x0a:
        buffer.write(r'\n');
      case 0x09:
        buffer.write(r'\t');
      default:
        buffer.writeCharCode(unit);
    }
  }
  buffer.write('"');
  return buffer.toString();
}
