import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';

const documentWorkspace = '10000000-0000-0000-0000-000000000001';
const documentSpace = '20000000-0000-0000-0000-000000000001';
const documentParent = '20000000-0000-0000-0000-000000000002';
const otherDocumentSpace = '20000000-0000-0000-0000-000000000003';

class DocumentCreationFixture {
  late HttpServer server;
  late AppFlowyCloudWorkspaceService service;
  String subject = 'alice', role = 'Owner', parentName = 'Parent document';
  bool movedParent = false, publish = true;
  int folderReads = 0;
  final bodies = <Map<String, dynamic>>[];
  final published = <Map<String, dynamic>>[];
  Future<bool> Function(HttpRequest)? intercept;
  Future<void> Function(HttpRequest, Map<String, dynamic>)? post;
  static Future<DocumentCreationFixture> create({
    Duration timeout = const Duration(seconds: 2),
    Duration renameTimeout = const Duration(seconds: 2),
  }) async {
    final f = DocumentCreationFixture();
    f.server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    f.service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${f.server.port}'),
      accessToken: () async => 'test',
      accountSubject: () => f.subject,
      deviceId: 'original-document-creation',
      allowHttpForTesting: true,
      documentCreationTimeout: timeout,
      pageRenameTimeout: renameTimeout,
    );
    unawaited(
      f.server.forEach((r) async {
        if (await f.intercept?.call(r) ?? false) return;
        if (r.method == 'POST') {
          final value = (jsonDecode(await utf8.decoder.bind(r).join()) as Map)
              .cast<String, dynamic>();
          f.bodies.add(value);
          if (f.publish) f.published.add(value);
          if (f.post != null) {
            await f.post!(r, value);
          } else {
            await f.reply(r, {'view_id': value['view_id']});
          }
        } else if (r.uri.path == '/api/workspace') {
          await f.reply(r, [
            {'workspace_id': documentWorkspace, 'role': f.role},
          ]);
        } else if (r.uri.path.endsWith('/folder')) {
          f.folderReads++;
          await f.reply(r, f.folder());
        } else if (r.uri.path.endsWith('/database')) {
          await f.reply(r, []);
        } else {
          r.response.statusCode = 404;
          await r.response.close();
        }
      }),
    );
    return f;
  }

  Map<String, dynamic> folder() {
    final parent = <String, dynamic>{
      'view_id': documentParent,
      'name': parentName,
      'layout': 0,
      'children': [
        for (final row in published.where(
          (p) => p['parent_view_id'] == documentParent,
        ))
          {
            'view_id': row['view_id'],
            'name': row['name'],
            'layout': 0,
            'children': [],
          },
      ],
    };
    return {
      'view_id': documentWorkspace,
      'name': 'Workspace',
      'layout': 0,
      'children': [
        {
          'view_id': documentSpace,
          'name': 'General',
          'layout': 0,
          'is_space': true,
          'children': [
            if (!movedParent) parent,
            for (final row in published.where(
              (p) => p['parent_view_id'] == documentSpace,
            ))
              {
                'view_id': row['view_id'],
                'name': row['name'],
                'layout': 0,
                'children': [],
              },
          ],
        },
        if (movedParent)
          {
            'view_id': otherDocumentSpace,
            'name': 'Other space',
            'layout': 0,
            'is_space': true,
            'children': [parent],
          },
      ],
    };
  }

  Future<AppFlowyDocumentParent> target() async =>
      (await service.documentCreationParents(
        documentWorkspace,
      )).singleWhere((p) => p.viewId == documentParent);
  Future<void> reply(HttpRequest r, Object? value) async {
    r.response.headers.contentType = ContentType.json;
    r.response.write(jsonEncode({'code': 0, 'data': value}));
    try {
      await r.response.close();
    } on SocketException {
      /* timed out caller */
    }
  }

  Future<void> close() async {
    service.closeClient();
    await server.close(force: true);
  }
}
