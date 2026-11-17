import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'catalog_test_support.dart';

/// Exercises the plugin against the existing Cloud routes, without owning a
/// second document model or importing the service's private test fixtures.
class DocumentUiFixture {
  DocumentUiFixture(this.catalog);
  final CatalogFixture catalog;
  final bodies = <Map<String, dynamic>>[];
  final published = <Map<String, dynamic>>[];
  String role = 'Owner';
  bool publish = true, breakAck = false;
  Future<bool> Function(HttpRequest)? intercept;
  static Future<DocumentUiFixture> create() async {
    final f = DocumentUiFixture(await CatalogFixture.create());
    f.catalog.intercept = (r) async {
      if (await f.intercept?.call(r) ?? false) return true;
      if (r.uri.path == '/api/workspace') {
        await f.catalog.reply(r, [
          for (final wid in [catalogWorkspace, catalogOtherWorkspace])
            {'workspace_id': wid, 'role': f.role},
        ]);
        return true;
      }
      if (r.method == 'POST') {
        final body = (jsonDecode(await utf8.decoder.bind(r).join()) as Map)
            .cast<String, dynamic>();
        f.bodies.add(body);
        if (f.publish) f.published.add(body);
        if (f.breakAck) {
          final socket = await r.response.detachSocket(writeHeaders: false);
          socket.destroy();
        } else {
          await f.catalog.reply(r, {'view_id': body['view_id']});
        }
        return true;
      }
      if (r.uri.path.endsWith('/folder') ||
          r.uri.path.startsWith('/api/sharing/workspace/')) {
        final folder = f.catalog.folder(
          r.uri.path.startsWith('/api/sharing/')
              ? r.uri.pathSegments[3]
              : r.uri.pathSegments[2],
        );
        final children = folder['children']! as List;
        for (final parent in children.cast<Map>()) {
          parent['children'] = [
            ...parent['children'] as List,
            for (final row in f.published.where(
              (p) => p['parent_view_id'] == parent['view_id'],
            ))
              {
                'view_id': row['view_id'],
                'name': row['name'],
                'layout': 0,
                'children': [],
              },
          ];
        }
        await f.catalog.reply(r, folder);
        return true;
      }
      return false;
    };
    return f;
  }

  DocumentCreationController controller() => DocumentCreationController(
    authentication: catalog.auth,
    service: catalog.service,
    workspaceId: catalogWorkspace,
  );
  Future<void> close() => catalog.close();
}
