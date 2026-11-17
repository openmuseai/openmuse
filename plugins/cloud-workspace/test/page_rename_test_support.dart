import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'catalog_test_support.dart';

class RenameUiFixture {
  RenameUiFixture(this.catalog);
  final CatalogFixture catalog;
  final writes = <Map<String, dynamic>>[];
  final names = <String, String>{};
  String role = 'Owner';
  bool breakAck = false, publish = true;
  Future<bool> Function(HttpRequest)? guard;
  static Future<RenameUiFixture> create() async {
    final f = RenameUiFixture(await CatalogFixture.create());
    f.catalog.intercept = (r) async {
      if (await f.guard?.call(r) ?? false) return true;
      if (r.uri.path.endsWith('/update-name')) {
        final body = (jsonDecode(await utf8.decoder.bind(r).join()) as Map)
            .cast<String, dynamic>();
        f.writes.add(body);
        if (f.publish) f.names[r.uri.pathSegments[4]] = body['name'] as String;
        if (f.breakAck) {
          final socket = await r.response.detachSocket(writeHeaders: false);
          socket.destroy();
        } else {
          await f.catalog.reply(r, null);
        }
        return true;
      }
      if (r.uri.path == '/api/workspace') {
        await f.catalog.reply(r, [
          for (final wid in [catalogWorkspace, catalogOtherWorkspace])
            {'workspace_id': wid, 'role': f.role},
        ]);
        return true;
      }
      if (r.uri.path.endsWith('/folder') ||
          r.uri.path.startsWith('/api/sharing/workspace/')) {
        final folder = f.catalog.folder(
          r.uri.path.startsWith('/api/sharing/')
              ? r.uri.pathSegments[3]
              : r.uri.pathSegments[2],
        );
        void labels(Map node) {
          if (f.names.containsKey(node['view_id']))
            node['name'] = f.names[node['view_id']];
          for (final child in node['children'] as List) {
            labels(child as Map);
          }
        }

        labels(folder);
        await f.catalog.reply(r, folder);
        return true;
      }
      return false;
    };
    return f;
  }

  PageRenameController controller() => PageRenameController(
    authentication: catalog.auth,
    service: catalog.service,
    page: const AppFlowyDocumentPage(
      workspaceId: catalogWorkspace,
      viewId: catalogParent,
      name: 'Parent document',
      ancestors: [catalogWorkspace],
    ),
  );
  Future<void> close() => catalog.close();
}
