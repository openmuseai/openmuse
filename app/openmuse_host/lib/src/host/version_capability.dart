import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:crypto/crypto.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;
import 'workspace_controller.dart';

/// Public, opt-in file version capability, shared by any installed MCP App.
/// Grants are installation-owned relative paths; web content only sees opaque
/// resource IDs. The active account's existing LocalVersionStore is authoritative.
final class HostVersionCapability {
  HostVersionCapability(
    this.workspace, {
    required this.resources,
    required this.writable,
    this.authorized,
  });
  final LocalWorkspaceController workspace;
  final List<String> resources;
  final bool writable;
  final bool Function()? authorized;
  void _guard() {
    if (authorized != null && !authorized!())
      throw StateError('VERSION_ACCOUNT_REVOKED');
  }

  Future<void> _tail = Future.value();
  static const _limit = 900000;
  List<Map<String, Object?>> get tools => [
    _tool(
      'host_versions_list',
      'List explicitly granted files and immutable history.',
      {
        'resourceId': {'type': 'string'},
      },
    ),
    _tool(
      'host_versions_read',
      'Read exact saved or current bytes; validates snapshot integrity.',
      {
        'resourceId': {'type': 'string'},
        'versionId': {'type': 'string'},
      },
      required: ['resourceId'],
    ),
    if (writable)
      _tool(
        'host_versions_capture',
        'Capture current, flushed file in the Host version authority.',
        {
          'resourceId': {'type': 'string'},
        },
        required: ['resourceId'],
        write: true,
      ),
    if (writable)
      _tool(
        'host_versions_restore',
        'Restore saved bytes with expected current digest and automatic backup; close native editor first.',
        {
          'resourceId': {'type': 'string'},
          'versionId': {'type': 'string'},
          'expectedSha256': {'type': 'string'},
        },
        required: ['resourceId', 'versionId', 'expectedSha256'],
        write: true,
      ),
  ];
  Map<String, Object?> _tool(
    String name,
    String description,
    Map<String, Object?> properties, {
    List<String> required = const [],
    bool write = false,
  }) => {
    'name': name,
    'description': description,
    'inputSchema': {
      'type': 'object',
      'properties': properties,
      'required': required,
      'additionalProperties': false,
    },
    'annotations': {
      'readOnlyHint': !write,
      'destructiveHint': write,
      'openWorldHint': false,
    },
  };
  String _id(String relative) =>
      sha256.convert(utf8.encode(relative)).toString();
  Future<OpenMuseResource> _resource(String id) async {
    final relative = resources.where((r) => _id(r) == id).firstOrNull;
    if (relative == null ||
        p.isAbsolute(relative) ||
        p.split(relative).any((v) => v == '..' || v == '.git') ||
        relative.contains('\\'))
      throw StateError('VERSION_RESOURCE_NOT_GRANTED');
    final root = await Directory(workspace.rootPath).resolveSymbolicLinks();
    var current = root;
    for (final part in p.split(relative)) {
      current = p.join(current, part);
      if (await FileSystemEntity.type(current, followLinks: false) ==
          FileSystemEntityType.link)
        throw StateError('VERSION_SYMLINK_DENIED');
    }
    if (!p.isWithin(root, await File(current).resolveSymbolicLinks()))
      throw StateError('VERSION_RESOURCE_NOT_GRANTED');
    if (await File(current).length() > _limit)
      throw StateError('VERSION_FILE_BUDGET_EXCEEDED');
    return OpenMuseResource(
      uri: Uri.file(current),
      displayName: p.basename(current),
    );
  }

  Future<Map<String, Object?>> call(
    String name,
    Map<String, Object?> arguments,
  ) async {
    final allowed = tools.where((t) => t['name'] == name).firstOrNull;
    if (allowed == null) throw StateError('VERSION_CAPABILITY_NOT_GRANTED');
    final schema = allowed['inputSchema'] as Map;
    final properties = schema['properties'] as Map;
    if (arguments.keys.any((k) => !properties.containsKey(k)) ||
        (schema['required'] as List).any((k) => arguments[k] is! String) ||
        arguments.values.any((v) => v is! String || (v as String).length > 256))
      throw StateError('VERSION_ARGUMENT_INVALID');
    final next = _tail.then((_) {
      // Queued calls and metadata enumeration share the same account gate.
      _guard();
      return _call(name, arguments);
    });
    _tail = next.then<void>((_) {}, onError: (Object e, StackTrace st) {});
    try {
      final value = await next;
      // A flush, history read or disk I/O may outlive its authorization.
      _guard();
      return {
        'structuredContent': value,
        'content': [
          {'type': 'text', 'text': jsonEncode(value)},
        ],
      };
    } catch (e) {
      final code = e is StateError
          ? e.message.toString()
          : 'VERSION_SOURCE_UNAVAILABLE';
      return {
        'isError': true,
        'content': [
          {'type': 'text', 'text': code},
        ],
      };
    }
  }

  Future<Map<String, Object?>> _call(
    String name,
    Map<String, Object?> args,
  ) async {
    if (name == 'host_versions_list' && args['resourceId'] == null) {
      return {
        'contract': 'openmuse.host-versions/1',
        'writable': writable,
        'resources': [
          for (final relative in resources)
            {'id': _id(relative), 'path': relative},
        ],
      };
    }
    _guard();
    final resource = await _resource(args['resourceId'] as String);
    _guard();
    if (name == 'host_versions_list')
      return {
        'versions': [
          for (final version in await workspace.versions(resource))
            {
              'id': version.versionId,
              'sha256': version.id,
              'createdAt': version.createdAt.toUtc().toIso8601String(),
              'byteLength': version.byteLength,
            },
        ],
      };
    if (name == 'host_versions_capture') {
      final s = await workspace.captureVersion(resource);
      return {
        'versionId': s.versionId,
        'sha256': s.id,
        'byteLength': s.byteLength,
      };
    }
    if (name == 'host_versions_restore') {
      // Never let an unsaved native buffer overwrite a restored file later.
      if (workspace.editorGroups.any(
        (g) => g.tabs.any(
          (t) =>
              t.kind == WorkspaceTabKind.resource &&
              t.resource.uri == resource.uri,
        ),
      ))
        throw StateError('VERSION_CLOSE_EDITOR_BEFORE_RESTORE');
      final expected = args['expectedSha256'];
      final file = File.fromUri(resource.uri);
      if (sha256.convert(await file.readAsBytes()).toString() != expected)
        throw StateError('VERSION_CURRENT_CHANGED');
      final restored = await workspace.readVersionBytes(
        resource,
        args['versionId'] as String,
      );
      if (restored.length > _limit)
        throw StateError('VERSION_FILE_BUDGET_EXCEEDED');
      _guard();
      final backup = await workspace.captureVersion(resource);
      _guard();
      if (sha256.convert(await file.readAsBytes()).toString() != expected)
        throw StateError('VERSION_CURRENT_CHANGED');
      final temporary = File(
        '${file.path}.openmuse-restore-${DateTime.now().microsecondsSinceEpoch}',
      );
      try {
        await temporary.writeAsBytes(restored, flush: true);
        _guard();
        await temporary.rename(file.path);
      } finally {
        if (await temporary.exists()) await temporary.delete();
      }
      final result = await workspace.captureVersion(resource);
      return {
        'restoredVersionId': result.versionId,
        'backupVersionId': backup.versionId,
        'sha256': result.id,
        'byteLength': result.byteLength,
      };
    }
    final versionId = args['versionId'] as String?;
    if (versionId == null) {
      await workspace.flushBeforeDiskRead?.call(resource);
      _guard();
    }
    final bytes = versionId == null
        ? await File.fromUri(resource.uri).readAsBytes()
        : await workspace.readVersionBytes(resource, versionId);
    _guard();
    if (bytes.length > _limit) throw StateError('VERSION_FILE_BUDGET_EXCEEDED');
    String? text;
    try {
      if (!bytes.contains(0)) text = utf8.decode(bytes);
    } on FormatException {
      /* binary */
    }
    return {
      'resourceId': args['resourceId'],
      'versionId': versionId,
      'path': resources.singleWhere((r) => _id(r) == args['resourceId']),
      'sha256': sha256.convert(bytes).toString(),
      'byteLength': bytes.length,
      'binary': text == null,
      if (text != null) 'text': text,
      if (text == null) 'base64': base64Encode(bytes),
    };
  }
}
