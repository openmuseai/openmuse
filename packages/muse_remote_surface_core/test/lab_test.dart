import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';

void main() {
  test('RS-MEDIA denies expired, cross-scope, and path-like handles', () {
    final authority = RemoteMediaAuthority();
    final now = DateTime.utc(2026, 10, 5, 12);
    final bytes = Uint8List.fromList([1, 2, 3, 4]);
    final handle = authority.issue(
      handle: 'media.cover.1',
      bytes: bytes,
      workspaceRef: 'ws.opaque.1',
      deviceRef: 'desktop.1',
      expiresAt: now.add(const Duration(minutes: 5)),
    );

    expect(
      authority.read(
        handle: handle,
        workspaceRef: 'ws.opaque.1',
        deviceRef: 'desktop.1',
        now: now,
      ),
      bytes,
    );
    expect(
      authority.read(
        handle: handle,
        workspaceRef: 'ws.other',
        deviceRef: 'desktop.1',
        now: now,
      ),
      isNull,
    );
    expect(
      authority.read(
        handle: handle,
        workspaceRef: 'ws.opaque.1',
        deviceRef: 'desktop.2',
        now: now,
      ),
      isNull,
    );
    expect(
      authority.read(
        handle: handle,
        workspaceRef: 'ws.opaque.1',
        deviceRef: 'desktop.1',
        now: now.add(const Duration(minutes: 5)),
      ),
      isNull,
    );
    expect(
      () => authority.issue(
        handle: 'media/../cover',
        bytes: bytes,
        workspaceRef: 'ws.opaque.1',
        deviceRef: 'desktop.1',
        expiresAt: now,
      ),
      throwsFormatException,
    );
  });

  test('RS-WEB only allows the virtual snapshot origin', () {
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('https://surface.openmuse.invalid/surface/publish'),
      ),
      isTrue,
    );
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('https://127.0.0.1/surface/publish'),
      ),
      isFalse,
    );
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('https://localhost/surface/publish'),
      ),
      isFalse,
    );
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('https://[::1]/surface/publish'),
      ),
      isFalse,
    );
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('http://surface.openmuse.invalid/surface/publish'),
      ),
      isFalse,
    );
    expect(
      RemoteWebGuard.allowsNavigation(
        Uri.parse('https://example.com/surface/publish'),
      ),
      isFalse,
    );
    expect(RemoteWebGuard.allowsCookie('session=secret'), isFalse);
  });

  test(
    'RS-EASEL pins the package manifest and unregisters a removed plugin',
    () {
      final directory = Directory.systemTemp.createTempSync('remote-packages');
      addTearDown(() => directory.deleteSync(recursive: true));
      final manifest = File('${directory.path}/enabled.json');
      final digest = File('${directory.path}/enabled.sha256');
      void write(List<String> enabled) {
        final text = jsonEncode({'enabled': enabled});
        manifest.writeAsStringSync(text);
        digest.writeAsStringSync('sha256:${sha256.convert(utf8.encode(text))}');
      }

      final host = RemoteSurfaceHost();
      final catalog = SurfacePackageCatalog({
        'com.openmuse.fake-publish': FakePublishSurfaceProvider.new,
        'com.openmuse.fake-video': FakeVideoEditProvider.new,
      });
      write(['com.openmuse.fake-publish']);
      catalog.sync(host, manifest, digest: digest);
      expect(host.registeredPluginIds, ['com.openmuse.fake-publish']);

      digest.writeAsStringSync('sha256:${'0' * 64}');
      expect(
        () => catalog.sync(host, manifest, digest: digest),
        throwsStateError,
      );
      expect(host.registeredPluginIds, ['com.openmuse.fake-publish']);

      write(<String>[]);
      catalog.sync(host, manifest, digest: digest);
      expect(host.registeredPluginIds, isEmpty);
    },
  );

  test('RS-LEDGER a restarted host returns the stored receipt', () {
    final directory = Directory.systemTemp.createTempSync('remote-jobs');
    addTearDown(() => directory.deleteSync(recursive: true));
    final firstProvider = FakePublishSurfaceProvider();
    final first = RemoteSurfaceHost(
      jobs: RemoteJobLedger(directory: directory),
      audit: RemoteAuditLog(directory: directory),
    )..register(firstProvider);
    final opened = _open(first);
    first.submit(
      _request(opened, 'social.preview', {'title': '周末散步'}, key: 'preview-1'),
      context: _context,
    );
    final current = first.readSnapshot(
      surfaceSessionRef: opened.surfaceSessionRef,
      generation: opened.generation,
    );
    final commit = _request(current, 'social.publish.commit', {
      'previewRef': 'preview-1',
      'accountRef': 'account-1',
    }, key: 'commit-1');
    final accepted = first.submit(commit, context: _context);
    expect(firstProvider.externalExecutions, 1);
    expect(
      File('${directory.path}/audit.jsonl').readAsStringSync(),
      isNot(contains('周末散步')),
    );

    final secondProvider = FakePublishSurfaceProvider();
    final restarted = RemoteSurfaceHost(
      jobs: RemoteJobLedger(directory: directory),
    )..register(secondProvider);
    final fresh = _open(restarted);
    final replay = restarted.submit(
      RemoteControlRequest(
        requestId: 'req-replay',
        surfaceSessionRef: fresh.surfaceSessionRef,
        generation: fresh.generation,
        actionId: 'social.publish.commit',
        input: commit.input,
        expectedStateRevision: 'state-does-not-match',
        idempotencyKey: 'commit-1',
        deadlineMs: 10000,
      ),
      context: _context,
    );
    final conflict = restarted.submit(
      RemoteControlRequest(
        requestId: 'req-conflict',
        surfaceSessionRef: fresh.surfaceSessionRef,
        generation: fresh.generation,
        actionId: 'social.publish.commit',
        input: {'previewRef': 'preview-9', 'accountRef': 'account-1'},
        expectedStateRevision: fresh.stateRevision,
        idempotencyKey: 'commit-1',
        deadlineMs: 10000,
      ),
      context: _context,
    );

    expect(replay.status, 'accepted');
    expect(replay.decisionRef, accepted.decisionRef);
    expect(conflict.errorCode, 'IDEMPOTENCY_CONFLICT');
    expect(secondProvider.externalExecutions, 0);
  });
}

const _context = RemoteConnectionContext(
  actorRef: 'actor.1',
  mobileDeviceRef: 'mobile.1',
  desktopDeviceRef: 'desktop.1',
  workspaceRef: 'ws.opaque.1',
  permissions: {
    'workspace.resource.read',
    'workspace.resource.write',
    'social.content.publish',
  },
);

RemoteSurfaceSnapshot _open(RemoteSurfaceHost host) {
  final result = host.open(
    pluginId: 'com.openmuse.fake-publish',
    surfaceId: 'publish-workflow',
    hello: RemoteClientHello.mobileV1,
    context: _context,
  );
  expect(result, isA<RemoteOpenAccepted>());
  return (result as RemoteOpenAccepted).snapshot;
}

RemoteControlRequest _request(
  RemoteSurfaceSnapshot snapshot,
  String actionId,
  Map<String, Object?> input, {
  required String key,
}) {
  return RemoteControlRequest(
    requestId: 'req-$key',
    surfaceSessionRef: snapshot.surfaceSessionRef,
    generation: snapshot.generation,
    actionId: actionId,
    input: input,
    expectedStateRevision: snapshot.stateRevision,
    idempotencyKey: key,
    deadlineMs: 10000,
  );
}
