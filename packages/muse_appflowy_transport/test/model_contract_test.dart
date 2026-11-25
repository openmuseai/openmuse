import 'dart:async';
import 'package:http/http.dart' as http;
import 'package:test/test.dart';
import 'package:muse_appflowy_transport/muse_appflowy_transport.dart';
import 'write_capabilities_test.dart'
    show object, workspace, transport, response, failure;

Map<String, Object?> contract({bool persisted = false}) => {
  'protocol': 'openmuse.collab-model-contract/1',
  'workspaceId': workspace,
  'objectId': object,
  'collabType': 1,
  'modelProfile': persisted
      ? 'future.database/1'
      : 'appflowy.original-collab/1',
  'epoch': persisted ? '9223372036854775807' : null,
  'binding': persisted ? 'persisted' : 'implicit-original',
  'originalPipelineAvailable': !persisted,
  'writerFencing': false,
  'semanticExtensions': <String>[],
};

void main() {
  for (final persisted in [false, true]) {
    test(
      'model descriptor keeps ${persisted ? 'opaque persisted epoch' : 'implicit original'} without a write grant',
      () async {
        final t = transport((request) async {
          expect(request.method, 'GET');
          expect(request.url.path, endsWith('/model-contract'));
          expect(request.url.queryParameters, {'collab_type': '1'});
          return response(contract(persisted: persisted));
        });
        addTearDown(t.close);
        final c = (await t.readModelContract(object))!;
        expect(
          c.modelProfile,
          persisted ? 'future.database/1' : 'appflowy.original-collab/1',
        );
        expect(c.epoch, persisted ? '9223372036854775807' : null);
        expect(c.originalPipelineAvailable, !persisted);
      },
    );
  }
  test('missing old route differs from a typed missing object', () async {
    final old = transport((_) async => http.Response('', 404));
    addTearDown(old.close);
    expect(await old.readModelContract(object), isNull);
    final missing = transport((_) async => http.Response('{"code":-2}', 404));
    addTearDown(missing.close);
    await expectLater(missing.readModelContract(object), failure('CLOUD_-2'));
  });
  for (final key in ['workspaceId', 'objectId', 'collabType']) {
    test('model $key must match binding', () async {
      final t = transport(
        (_) async =>
            response(contract()..[key] = key == 'collabType' ? 4 : 'other'),
      );
      addTearDown(t.close);
      await expectLater(
        t.readModelContract(object),
        failure('COLLABORATION_OBJECT_MISMATCH'),
      );
    });
  }
  for (final epoch in <Object?>[
    null,
    0,
    1,
    '0',
    '01',
    '-1',
    '1.0',
    '1e3',
    ' 1',
    '9223372036854775808',
    '9' * 20,
  ]) {
    test('persisted epoch $epoch is canonical signed64 decimal text', () async {
      final t = transport(
        (_) async => response(contract(persisted: true)..['epoch'] = epoch),
      );
      addTearDown(t.close);
      await expectLater(
        t.readModelContract(object),
        failure('INVALID_MODEL_CONTRACT'),
      );
    });
  }
  final invalid = <String, Object?>{
    'modelProfile': 'Future Bad',
    'binding': 'upgraded',
    'originalPipelineAvailable': true,
    'writerFencing': true,
    'semanticExtensions': ['new-fields'],
  };
  for (final entry in invalid.entries) {
    test('persisted model rejects inconsistent ${entry.key}', () async {
      final t = transport(
        (_) async =>
            response(contract(persisted: true)..[entry.key] = entry.value),
      );
      addTearDown(t.close);
      await expectLater(
        t.readModelContract(object),
        failure('INVALID_MODEL_CONTRACT'),
      );
    });
  }
  test(
    'implicit metadata cannot silently carry a persisted epoch or unknown model',
    () async {
      for (final change in [
        {'epoch': '1'},
        {'modelProfile': 'future/1'},
      ]) {
        final t = transport((_) async => response(contract()..addAll(change)));
        addTearDown(t.close);
        await expectLater(
          t.readModelContract(object),
          failure('INVALID_MODEL_CONTRACT'),
        );
      }
    },
  );
  test('unknown contract protocol cannot activate interpretation', () async {
    final t = transport(
      (_) async => response(contract()..['protocol'] = 'future-contract/2'),
    );
    addTearDown(t.close);
    await expectLater(
      t.readModelContract(object),
      failure('MODEL_CONTRACT_UNSUPPORTED'),
    );
  });
  for (final close in [false, true]) {
    test(
      'late model response respects ${close ? 'close' : 'revocation'}',
      () async {
        final entered = Completer<void>(), release = Completer<void>();
        var active = true;
        final t = transport(
          (_) async {
            entered.complete();
            await release.future;
            return response(contract());
          },
          checkAuthority: () {
            if (!active) throw const AppFlowyTransportFailure('FORBIDDEN');
          },
        );
        addTearDown(t.close);
        final pending = expectLater(
          t.readModelContract(object),
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
