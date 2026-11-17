import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_builtin_plugins/openmuse_builtin_plugins.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

const wid = '00000000-0000-4000-8000-000000000001';
const docId = '00000000-0000-4000-8000-000000000002';
const missingId = '00000000-0000-4000-8000-000000000003';

final class Auth extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot snapshot =
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: 'alice',
          email: 'alice@example.test',
        ),
      );
  void switchTo(String subject) {
    snapshot = OpenMuseAuthenticationSnapshot(
      phase: OpenMuseAuthenticationPhase.authenticated,
      identity: OpenMuseAuthenticatedIdentity(
        subject: subject,
        email: '$subject@example.test',
      ),
    );
    notifyListeners();
  }

  @override
  Future<String?> accessToken({bool forceRefresh = false}) async =>
      'synthetic-test';
  @override
  Future<void> restore() async {}
  @override
  Future<void> signInWithPassword(String email, String password) async {}
  @override
  Future<void> signOut() async {}
}

void main() {
  late HttpServer server;
  late Auth auth;
  late AppFlowyCloudWorkspaceService service;
  late AppFlowyDocumentWorkspace workspace;
  late ResourceCenterController center;
  late OpenMusePluginRegistry registry;
  late ResourceCenterHub hub;
  var requests = 0;
  var revoked = false;
  setUp(() async {
    requests = 0;
    revoked = false;
    auth = Auth();
    server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    server.listen((request) async {
      requests++;
      final Object data = request.uri.path == '/api/workspace'
          ? [
              {'workspace_id': wid, 'workspace_name': '原工作区名称', 'role': 2},
            ]
          : request.uri.path.endsWith('/folder')
          ? {
              'view_id': wid,
              'children': [
                {
                  'view_id': docId,
                  'layout': 0,
                  'name': 'Original Cloud document',
                  'children': [],
                },
              ],
            }
          : <Object>[];
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'code': revoked || request.uri.path.contains('/page-view/')
              ? 1012
              : 0,
          'data': data,
        }),
      );
      await request.response.close();
    });
    service = AppFlowyCloudWorkspaceService(
      baseUri: Uri.parse('http://127.0.0.1:${server.port}'),
      accessToken: auth.accessToken,
      accountSubject: () => auth.snapshot.identity?.subject,
      deviceId: 'scope-test',
      allowHttpForTesting: true,
    );
    workspace = AppFlowyDocumentWorkspace(
      authentication: auth,
      service: service,
    );
    registry = OpenMusePluginRegistry(
      context: OpenMusePluginContext(executeHostCommand: (_, _) async => null),
    );
    hub = ResourceCenterHub(scope: 'alice');
    center = ResourceCenterController(hub: hub, registry: registry);
    workspace.attach(center);
  });
  tearDown(() async {
    workspace.dispose();
    center.dispose();
    registry.dispose();
    auth.dispose();
    service.closeClient();
    await hub.dispose();
    await server.close(force: true);
  });
  ResourceReference ref({
    String mount = 'pages',
    String resource = missingId,
  }) => ResourceReference(
    identity: ResourceIdentity(
      scope: 'sender',
      source: workspace.sourceId,
      workspace: wid,
      mount: mount,
      resource: resource,
    ),
  );
  Matcher denied = throwsA(
    isA<ResourceFailure>().having((e) => e.code, 'code', 'DENIED'),
  );
  test(
    'Cloud source discovers deferred metadata with current account and rejects device pins',
    () async {
      await expectLater(
        center.resolve(ref()),
        denied,
      ); // No matching page: never opens a native content core.
      final id = ResourceIdentity(
        scope: 'alice',
        source: workspace.sourceId,
        workspace: wid,
        mount: 'pages',
        resource: docId,
      );
      final provider = hub.provider(id);
      final page = await provider.list(
        'root',
        hub.access('browser', operations: {ResourceOperation.describe}),
      );
      expect(page.entries.single.deferredIdentity, id);
      expect(page.entries.single.resource, isNull);
      expect(provider.source.exactVersions, isFalse);
      expect(provider.source.workspaceLabel, '原工作区名称');
      final count = requests;
      await expectLater(
        provider.read(
          docId,
          ResourceVersion(revision: 'device', digest: resourceDigest([])),
          hub.access('ui'),
        ),
        throwsA(
          isA<ResourceFailure>().having(
            (e) => e.code,
            'code',
            'VERSION_UNAVAILABLE',
          ),
        ),
      );
      expect(requests, count);
    },
  );
  test(
    'unrecognized mounts and non-UUID references cannot issue remote requests',
    () async {
      for (final reference in [
        ref(mount: 'arbitrary-endpoint'),
        ref(resource: 'not-a-cloud-object'),
      ]) {
        await expectLater(
          center.resolve(reference),
          throwsA(isA<ResourceFailure>()),
        );
      }
      expect(requests, 0);
    },
  );
  test(
    'account ABA permanently retires its old bridge and providers',
    () async {
      await expectLater(center.resolve(ref()), denied);
      final provider = hub.providers.single;
      final count = requests;
      auth.switchTo('bob');
      auth.switchTo('alice');
      expect(center.editorLifetime.active, isFalse);
      expect(provider.available, isFalse);
      await expectLater(center.resolve(ref()), denied);
      expect(requests, count);
    },
  );
  test(
    'attaching a new account center detaches the old reference source',
    () async {
      final nextHub = ResourceCenterHub(scope: 'alice');
      final next = ResourceCenterController(hub: nextHub, registry: registry);
      workspace.attach(next);
      expect(center.editorLifetime.active, isFalse);
      await expectLater(center.resolve(ref()), denied);
      expect(requests, 0);
      await expectLater(next.resolve(ref()), denied);
      expect(requests, greaterThan(0));
      workspace.dispose();
      next.dispose();
      await nextHub.dispose();
    },
  );
  test(
    'original Cloud authorization rejection becomes a resource denial after revocation',
    () async {
      await expectLater(center.resolve(ref()), denied);
      revoked = true;
      await expectLater(center.resolve(ref()), denied);
    },
  );
}
