import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:http/testing.dart';
import 'package:muse_appflowy_transport/muse_appflowy_transport.dart';
import 'package:test/test.dart';

const object = '041c01d0-831a-408a-b8fa-190d40dd9e94';
const workspace = '041c01d0-831a-408a-b8fa-190d40dd9e93';
Map<String, Object?> capability() => {
  'profile': 'appflowy.original-collab/1',
  'workspaceId': workspace,
  'objectId': object,
  'collabType': 1,
  'canWrite': true,
  'qualifiedEditing': true,
  'documentActorBits': 32,
  'epoch': null,
  'writerFencing': false,
  'semanticExtensions': <String>[],
  'updateEncodings': ['yrs.update-v1', 'yrs.update-v2'],
  'maxFullSyncBytes': 50 * 1024 * 1024,
  'maxLegacyRealtimeMessageBytes': 10 * 1024 * 1024,
  'maxBatchBytes': 50 * 1024 * 1024,
  'maxBatchObjects': 4096,
};
http.Response response(Map<String, Object?> value) =>
    http.Response(jsonEncode({'code': 0, 'data': value}), 200);
AppFlowyObjectTransport transport(
  Future<http.Response> Function(http.Request) handle, {
  void Function()? checkAuthority,
}) => AppFlowyObjectTransport(
  origin: Uri.parse('https://cloud.example.test'),
  workspaceId: workspace,
  collabType: 1,
  boundObjectId: object,
  deviceId: 'capability-test',
  accessToken: () async => 'synthetic',
  checkAuthority: checkAuthority,
  binding: const CollaborationBinding(
    protocol: 'appflowy.full-sync/1',
    engine: 'test',
    encoding: 'yrs.update-v1',
    schema: 'root',
    epoch: 0,
  ),
  client: MockClient(handle),
);
Matcher failure(String code) => throwsA(
  isA<AppFlowyTransportFailure>().having((error) => error.code, 'code', code),
);

void main() {
  test(
    'snapshot validation discovery is opaque, optional and immutable',
    () async {
      final legacy = transport((_) async => response(capability()));
      addTearDown(legacy.close);
      expect(
        (await legacy.readWriteCapabilities(
          object,
        ))!.snapshotValidationProfiles,
        isEmpty,
      );
      final t = transport(
        (_) async => response(
          capability()
            ..['canWrite'] = false
            ..['snapshotValidationProfiles'] = [
              'appflowy.document-tree/1',
              'future.domain/2',
            ],
        ),
      );
      addTearDown(t.close);
      final c = (await t.readWriteCapabilities(object))!;
      expect(c.canWrite, isFalse);
      expect(c.snapshotValidationProfiles, [
        'appflowy.document-tree/1',
        'future.domain/2',
      ]);
      expect(
        () => c.snapshotValidationProfiles.add('other/1'),
        throwsUnsupportedError,
      );
    },
  );
  for (final value in [
    null,
    'profile',
    [1],
    [''],
    ['invalid profile'],
    ['same/1', 'same/1'],
    List.filled(33, 'profile/1'),
    ['x' * 129],
  ]) {
    test('malformed snapshot profiles are rejected: $value', () async {
      final t = transport(
        (_) async =>
            response(capability()..['snapshotValidationProfiles'] = value),
      );
      addTearDown(t.close);
      await expectLater(
        t.readWriteCapabilities(object),
        failure('INVALID_CAPABILITIES'),
      );
    });
  }
  test(
    'discovery is bound to original object and reflects read-only authority',
    () async {
      final t = transport((request) async {
        expect(request.method, 'GET');
        expect(
          request.url.path,
          '/api/workspace/v1/$workspace/collab/$object/write-capabilities',
        );
        expect(request.url.queryParameters, {'collab_type': '1'});
        expect(request.headers['Authorization'], 'Bearer synthetic');
        expect(request.followRedirects, isFalse);
        return response(capability()..['canWrite'] = false);
      });
      addTearDown(t.close);
      final c = (await t.readWriteCapabilities(object))!;
      expect(c.canWrite, isFalse);
      expect(c.qualifiedEditing, isTrue);
      expect(c.objectId, object);
      expect(c.maxFullSyncBytes, 50 * 1024 * 1024);
      expect(c.maxLegacyRealtimeMessageBytes, 10 * 1024 * 1024);
    },
  );
  test(
    'server qualification is independent of content kind and write authority',
    () async {
      final t = transport(
        (_) async => response(capability()..['qualifiedEditing'] = false),
      );
      addTearDown(t.close);
      final c = (await t.readWriteCapabilities(object))!;
      expect(c.canWrite, isTrue);
      expect(c.qualifiedEditing, isFalse);
    },
  );
  test('cross-object request never reaches credentials or network', () async {
    final t = transport((_) async => throw StateError('must not request'));
    addTearDown(t.close);
    await expectLater(
      t.readWriteCapabilities(workspace),
      failure('COLLABORATION_OBJECT_MISMATCH'),
    );
  });
  test(
    'older Cloud route absence leaves original state editing usable',
    () async {
      final t = transport(
        (r) async => r.url.path.endsWith('/write-capabilities')
            ? http.Response('<html>Not found</html>', 404)
            : http.Response('{"code":0,"data":{"doc_state":[1,2]}}', 200),
      );
      addTearDown(t.close);
      expect(await t.readWriteCapabilities(object), isNull);
      expect(await t.readState(object), [1, 2]);
    },
  );
  for (final entry in {
    'deleted': -4,
    'revoked': 1012,
    'missing-object': -2,
  }.entries) {
    test('${entry.key} never becomes an unsupported server', () async {
      final t = transport(
        (_) async => http.Response(jsonEncode({'code': entry.value}), 404),
      );
      addTearDown(t.close);
      await expectLater(
        t.readWriteCapabilities(object),
        failure(
          entry.value == -4
              ? 'OBJECT_DELETED'
              : entry.value == 1012
              ? 'FORBIDDEN'
              : 'CLOUD_-2',
        ),
      );
    });
  }
  for (final status in [401, 403, 500]) {
    test('HTTP$status remains failure', () async {
      final t = transport((_) async => http.Response('', status));
      addTearDown(t.close);
      await expectLater(
        t.readWriteCapabilities(object),
        failure(status == 500 ? 'CLOUD_HTTP_500' : 'FORBIDDEN'),
      );
    });
  }
  test('unknown profile cannot enable new behavior', () async {
    final t = transport(
      (_) async => response(capability()..['profile'] = 'future/2'),
    );
    addTearDown(t.close);
    await expectLater(
      t.readWriteCapabilities(object),
      failure('CAPABILITIES_UNSUPPORTED'),
    );
  });
  for (final key in ['workspaceId', 'objectId', 'collabType']) {
    test('response $key must match the authenticated binding', () async {
      final t = transport(
        (_) async =>
            response(capability()..[key] = key == 'collabType' ? 4 : 'other'),
      );
      addTearDown(t.close);
      await expectLater(
        t.readWriteCapabilities(object),
        failure('COLLABORATION_OBJECT_MISMATCH'),
      );
    });
  }
  final malformed = <String, Object?>{
    'canWrite': 'true',
    'qualifiedEditing': 'false',
    'documentActorBits': 53,
    'epoch': 1,
    'writerFencing': true,
    'semanticExtensions': ['email'],
    'updateEncodings': ['yrs.update-v1'],
    'maxFullSyncBytes': 0,
    'maxLegacyRealtimeMessageBytes': -1,
    'maxBatchBytes': 1.5,
    'maxBatchObjects': 0x80000000,
  };
  for (final entry in malformed.entries) {
    test('malformed ${entry.key} is rejected without caching', () async {
      var calls = 0;
      final t = transport(
        (_) async => response(
          calls++ == 0
              ? (capability()..[entry.key] = entry.value)
              : capability(),
        ),
      );
      addTearDown(t.close);
      await expectLater(
        t.readWriteCapabilities(object),
        failure('INVALID_CAPABILITIES'),
      );
      expect((await t.readWriteCapabilities(object))!.canWrite, isTrue);
    });
  }
  test('missing epoch is not silently interpreted as a fenced epoch', () async {
    final t = transport((_) async => response(capability()..remove('epoch')));
    addTearDown(t.close);
    await expectLater(
      t.readWriteCapabilities(object),
      failure('INVALID_CAPABILITIES'),
    );
  });
  for (final close in [true, false]) {
    test(
      'late response respects ${close ? "closed session" : "account revocation"}',
      () async {
        final entered = Completer<void>(), release = Completer<void>();
        var active = true;
        final t = transport(
          (_) async {
            entered.complete();
            await release.future;
            return response(capability());
          },
          checkAuthority: () {
            if (!active) throw const AppFlowyTransportFailure('FORBIDDEN');
          },
        );
        addTearDown(t.close);
        final pending = expectLater(
          t.readWriteCapabilities(object),
          failure(close ? 'SESSION_CLOSED' : 'FORBIDDEN'),
        );
        await entered.future;
        if (close) {
          t.close();
        } else {
          active = false;
        }
        release.complete();
        await pending;
      },
    );
  }
}
