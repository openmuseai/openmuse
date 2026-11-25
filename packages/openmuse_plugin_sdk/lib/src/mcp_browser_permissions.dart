/// Supported MCP Apps UI resource permissions. Tool metadata and arbitrary
/// truthy values never grant browser/device capabilities.
final class McpBrowserPermissions {
  const McpBrowserPermissions({
    this.microphone = false,
    this.clipboardWrite = false,
  });
  final bool microphone, clipboardWrite;
  bool get needsOrigin => microphone || clipboardWrite;
  Map<String, Object?> toJson() => {
    if (microphone) 'microphone': <String, Object?>{},
    if (clipboardWrite) 'clipboardWrite': <String, Object?>{},
  };
  String get allow => [
    if (microphone) 'microphone',
    if (clipboardWrite) 'clipboard-write',
  ].join('; ');
  factory McpBrowserPermissions.fromJson(Object? value) {
    if (value == null) return const McpBrowserPermissions();
    if (value is! Map) throw const FormatException('Invalid UI permissions');
    bool requested(String name) {
      if (!value.containsKey(name)) return false;
      final permission = value[name];
      if (permission is! Map || permission.isNotEmpty)
        throw const FormatException('Invalid UI permission');
      return true;
    }

    return McpBrowserPermissions(
      microphone: requested('microphone'),
      clipboardWrite: requested('clipboardWrite'),
    );
  }
}

/// An export is not an update of the opened resource unless both identities
/// match. This also routes Save As to a new destination.
bool isOriginalMcpDownload(
  Map<String, Object?> request,
  String? name,
  String? mime,
) {
  if (name == null || mime == null || request['method'] != 'ui/download-file')
    return false;
  try {
    final contents = ((request['params'] as Map)['contents'] as List);
    if (contents.length != 1) return false;
    final resource = (contents.single as Map)['resource'] as Map;
    final uri = Uri.parse(resource['uri'] as String);
    return uri.pathSegments.isNotEmpty &&
        uri.pathSegments.last == name &&
        (resource['mimeType'] as String).toLowerCase() == mime.toLowerCase();
  } on Object {
    return false;
  }
}
