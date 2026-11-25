import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_cloud_workspace_plugin/openmuse_cloud_workspace_plugin.dart';
import 'invitations_test_support.dart';

const catalogWorkspace = '10000000-0000-0000-0000-000000000001';
const catalogOtherWorkspace = '10000000-0000-0000-0000-000000000002';
const catalogParent = '20000000-0000-0000-0000-000000000001';
const catalogChild = '20000000-0000-0000-0000-000000000002';
const catalogSibling = '20000000-0000-0000-0000-000000000003';
const catalogRoot = '30000000-0000-0000-0000-000000000001';

class CatalogFixture {
  late HttpServer server;
  final auth = InvitationTestAuth();
  late AppFlowyCloudWorkspaceService service;
  final seen = <String>[];
  Future<bool> Function(HttpRequest)? intercept;
  String childName = 'Nested original database';

  Map<String, Object?> folder(String workspace) => {
    'view_id': workspace,
    'is_space': false,
    'layout': 0,
    'children': [
      {
        'view_id': catalogParent,
        'name': 'Parent document',
        'layout': 0,
        'children': [
          {
            'view_id': catalogChild,
            'name': childName,
            'layout': 1,
            'children': [],
          },
        ],
      },
      {
        'view_id': catalogSibling,
        'name': 'Parent document',
        'layout': 0,
        'children': [],
      },
    ],
  };

  static Future<CatalogFixture> create() async {
    final f = CatalogFixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: f.auth.accessToken,
      accountSubject: () => f.auth.snapshot.identity?.subject,
      deviceId: 'original-catalog-test',
      allowHttpForTesting: true,
    );
    unawaited(
      f.server.forEach((request) async {
        f.seen.add('${request.method} ${request.uri.path}');
        if (await f.intercept?.call(request) ?? false) return;
        if (request.uri.path == '/api/workspace') {
          await f.reply(request, [
            for (final wid in [catalogWorkspace, catalogOtherWorkspace])
              {'workspace_id': wid, 'role': 'Owner'},
          ]);
        } else if (request.uri.path.endsWith('/folder')) {
          await f.reply(request, f.folder(request.uri.pathSegments[2]));
        } else if (request.uri.path.endsWith('/database')) {
          await f.reply(request, [
            {
              'id': catalogRoot,
              'views': [
                {'view_id': catalogChild},
              ],
            },
          ]);
        } else {
          request.response.statusCode = 404;
          await request.response.close();
        }
      }),
    );
    return f;
  }

  WorkspaceCatalogController controller() => WorkspaceCatalogController(
    authentication: auth,
    service: service,
    workspaceId: catalogWorkspace,
  );

  Future<void> reply(HttpRequest request, Object? value, {int code = 0}) async {
    request.response.headers.contentType = ContentType.json;
    request.response.write(jsonEncode({'code': code, 'data': value}));
    await request.response.close();
  }

  Future<void> close() async {
    service.closeClient();
    auth.dispose();
    await server.close(force: true);
  }
}
