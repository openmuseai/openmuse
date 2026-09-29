import 'dart:convert';
import 'dart:io';

import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  late HttpServer server;
  late HttpCloudWorkspaceService service;
  final requests = <String>[];

  setUp(() async {
    requests.clear();
    server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    service = HttpCloudWorkspaceService(
      baseUri: Uri.parse('http://${server.address.address}:${server.port}'),
      accessToken: () async => 'account-token',
      allowHttpForTesting: true,
    );
    server.listen((request) async {
      requests.add('${request.method} ${request.uri.path}');
      expect(
        request.headers.value(HttpHeaders.authorizationHeader),
        'Bearer account-token',
      );
      if (request.uri.path == '/v1/resources/commit') {
        expect(
          request.headers.contentType?.mimeType,
          'application/octet-stream',
        );
        expect(request.headers.value('OpenMuse-Resource-Ref'), 'resource:docx');
        expect(request.headers.value('OpenMuse-Expected-Revision'), 'r1');
        expect(request.headers.value('Idempotency-Key'), 'save:docx:1');
        expect(request.headers.value('OpenMuse-Generation'), '7');
        expect(
          await request.fold<List<int>>([], (all, part) => all..addAll(part)),
          [9, 8, 7],
        );
        request.response
          ..headers.contentType = ContentType.json
          ..write(
            jsonEncode({
              'commitRef': 'commit:c1',
              'resourceRef': 'resource:docx',
              'previousRevision': 'r1',
              'newRevision': 'r2',
              'generation': 7,
            }),
          )
          ..close();
        return;
      }
      final body = request.method == 'POST'
          ? jsonDecode(await utf8.decoder.bind(request).join())
                as Map<String, Object?>
          : <String, Object?>{};
      void json(Object value, {int status = 200}) {
        request.response
          ..statusCode = status
          ..headers.contentType = ContentType.json
          ..write(jsonEncode(value))
          ..close();
      }

      switch (request.uri.path) {
        case '/v1/workspaces':
          json({
            'items': [
              {
                'workspaceRef': 'cloud:w1',
                'title': 'Cloud Project',
                'revision': 'r1',
                'writable': true,
                'storageState': 'available',
              },
            ],
          });
        case '/v1/dsh/sessions':
          json({
            'sessionRef': 'dsh:s1',
            'origin': 'https://dsh.example.test',
            'path': '/session/s1',
            'generation': body['generation'],
          });
        case '/v1/resources/handles':
          json({
            'resourceRef': body['resourceRef'],
            'revision': body['revision'],
            'audience': body['audience'],
            'generation': body['generation'],
            'expiresAtMs': DateTime.now().millisecondsSinceEpoch + 60000,
            'size': 5,
            'mediaType': 'text/plain',
          });
        case '/v1/resources/range':
          request.response
            ..statusCode = 200
            ..add(utf8.encode('hello'))
            ..close();
        case '/v1/proposals':
          json({
            'proposalRef': 'proposal:p1',
            'workspaceRef': body['workspaceRef'],
            'expectedRevision': body['expectedRevision'],
            'summary': 'Update note.txt',
            'generation': body['generation'],
          });
        case '/v1/proposals/apply':
          json({
            'receiptRef': 'receipt:a1',
            'workspaceRef': body['workspaceRef'],
            'previousRevision': body['expectedRevision'],
            'newRevision': 'r2',
            'generation': body['generation'],
          });
        default:
          json({'error': 'not found'}, status: 404);
      }
    });
  });

  tearDown(() async {
    service.closeClient();
    await server.close(force: true);
  });

  test('service-backed Mobile vertical slice reaches a receipt', () async {
    final coordinator = CloudWorkspaceCoordinator(
      service: service,
      connector: service,
      resources: service,
    );
    final catalog = await coordinator.loginAndLoadCatalog();
    expect(catalog.single.revision, 'r1');
    await coordinator.select(catalog.single);
    expect(coordinator.flow.state, CloudFlowState.binding);

    final generation = coordinator.flow.generation;
    coordinator.pageLoaded(generation);
    coordinator.bridgeBound(generation);
    coordinator.workspaceAttached(generation, catalog.single.workspaceRef);
    expect(coordinator.presentation.state, DshPresentationState.ready);
    expect(coordinator.flow.state, CloudFlowState.ready);

    expect(
      await coordinator.readTextResource(
        'resource:note',
        nowMs: DateTime.now().millisecondsSinceEpoch,
      ),
      'hello',
    );
    final proposal = await coordinator.propose('Update the note');
    expect(proposal.expectedRevision, 'r1');
    final receipt = await coordinator.approve();
    expect(receipt.receiptRef, 'receipt:a1');
    expect(coordinator.flow.revision, 'r2');
    expect(coordinator.flow.state, CloudFlowState.applied);
    expect(requests, [
      'GET /v1/workspaces',
      'POST /v1/dsh/sessions',
      'POST /v1/resources/handles',
      'POST /v1/resources/range',
      'POST /v1/proposals',
      'POST /v1/proposals/apply',
    ]);
  });

  test(
    'adapter rejects insecure production origins and missing login',
    () async {
      expect(
        () => HttpCloudWorkspaceService(
          baseUri: Uri.parse('http://cloud.example.test'),
          accessToken: () async => 'token',
        ),
        throwsArgumentError,
      );
      final unsigned = HttpCloudWorkspaceService(
        baseUri: Uri.parse('http://${server.address.address}:${server.port}'),
        accessToken: () async => null,
        allowHttpForTesting: true,
      );
      addTearDown(unsigned.closeClient);
      await expectLater(
        unsigned.listWorkspaces(),
        throwsA(
          isA<CloudServiceException>().having(
            (error) => error.code,
            'code',
            CloudServiceErrorCode.unauthorized,
          ),
        ),
      );
    },
  );

  test('Office commit streams bytes and returns a CAS receipt', () async {
    final receipt = await service.commit(
      resourceRef: 'resource:docx',
      expectedRevision: 'r1',
      bytes: const [9, 8, 7],
      idempotencyKey: 'save:docx:1',
      generation: 7,
    );
    expect(receipt.commitRef, 'commit:c1');
    expect(receipt.previousRevision, 'r1');
    expect(receipt.newRevision, 'r2');
    expect(requests, ['POST /v1/resources/commit']);
  });
}
