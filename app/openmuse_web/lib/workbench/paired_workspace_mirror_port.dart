import 'dart:convert';
import 'dart:typed_data';

import 'package:http/browser_client.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

/// Same-origin browser adapter. The paired Edge must transport this request
/// through the authenticated Desktop grant, including on reconnect.
final class PairedWorkspaceMirrorPort
    implements WorkspaceMirrorPort, WorkspaceResourcePort {
  PairedWorkspaceMirrorPort({Uri? endpoint})
    : endpoint = endpoint ?? Uri.base.resolve('/openmuse/workspace-mirror/v1'),
      _client = BrowserClient()..withCredentials = true;

  final Uri endpoint;
  final BrowserClient _client;

  Future<Map<String, Object?>> _get(Map<String, String> query) async {
    final response = await _client.get(
      endpoint.replace(queryParameters: query),
    );
    if (response.statusCode != 200) {
      throw StateError('Workspace 镜像请求失败：HTTP ${response.statusCode}');
    }
    final data = jsonDecode(response.body);
    if (data is! Map<String, dynamic>) {
      throw const FormatException('Workspace 镜像响应无效');
    }
    return data;
  }

  @override
  Future<List<WorkspaceMirrorMount>> listMounts(String workspaceRef) async {
    final body = await _get({
      'operation': 'mounts',
      'workspaceRef': workspaceRef,
    });
    final raw = body['mounts'];
    if (raw is! List) throw const FormatException('Workspace 挂载列表无效');
    return [
      for (final item in raw)
        if (item is Map<String, dynamic>)
          WorkspaceMirrorMount(
            mountRef: item['mountRef'] as String,
            rootRef: item['rootRef'] as String,
            title: item['title'] as String,
          )
        else
          throw const FormatException('Workspace 挂载项无效'),
    ];
  }

  @override
  Future<WorkspaceMirrorPage> listChildren({
    required String workspaceRef,
    required String mountRef,
    required String parentRef,
    required int limit,
    String? cursor,
  }) async {
    final body = await _get({
      'operation': 'children',
      'workspaceRef': workspaceRef,
      'mountRef': mountRef,
      'parentRef': parentRef,
      'limit': '$limit',
      'cursor': ?cursor,
    });
    final raw = body['entries'];
    if (raw is! List) throw const FormatException('Workspace 目录页无效');
    return WorkspaceMirrorPage(
      entries: [
        for (final item in raw)
          if (item is Map<String, dynamic>)
            WorkspaceMirrorEntry(
              nodeRef: item['nodeRef'] as String,
              name: item['name'] as String,
              isDirectory: item['isDirectory'] as bool,
              resourceRef: item['resourceRef'] as String?,
              mediaType: item['mediaType'] as String?,
            )
          else
            throw const FormatException('Workspace 目录项无效'),
      ],
      nextCursor: body['nextCursor'] as String?,
    );
  }

  void close() => _client.close();

  @override
  Future<Uint8List> readResource({
    required String workspaceRef,
    required String resourceRef,
  }) async {
    final response = await _client.get(
      endpoint.replace(
        path: '/openmuse/workspace-mirror/resource/v1',
        queryParameters: {
          'workspaceRef': workspaceRef,
          'resourceRef': resourceRef,
        },
      ),
    );
    if (response.statusCode != 200) {
      throw StateError('文件预览请求失败：HTTP ${response.statusCode}');
    }
    return response.bodyBytes;
  }
}
