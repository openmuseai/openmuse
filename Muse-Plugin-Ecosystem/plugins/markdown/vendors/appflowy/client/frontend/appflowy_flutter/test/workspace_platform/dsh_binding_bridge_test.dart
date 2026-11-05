import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_workspace_bridge.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as p;

MuseWorkspaceMount _mount(
  String name,
  String root, {
  required int order,
  bool readOnly = false,
  String providerId = 'muse.workspace.local.v1',
}) =>
    MuseWorkspaceMount(
      mountRef: 'mount:$name',
      providerId: providerId,
      bindingKey: 'device-local:$name',
      displayName: name,
      rootLocator: root,
      readOnly: readOnly,
      order: order,
    );

void main() {
  late Directory sandbox;
  late String dshHome;
  late String stateRoot;
  late String frontend;
  late String helix;

  setUp(() async {
    sandbox = await Directory.systemTemp.createTemp('openmuse-dsh-binding-');
    dshHome = p.join(sandbox.path, 'dsh');
    stateRoot = p.join(sandbox.path, 'host-support');
    frontend = p.join(sandbox.path, 'frontend');
    helix = p.join(sandbox.path, 'helix');
    await Directory(p.join(dshHome, 'bindings')).create(recursive: true);
    await Directory(frontend).create(recursive: true);
    await Directory(helix).create(recursive: true);
    DshWorkspaceBridge.dshHomeOverride = dshHome;
    DshWorkspaceBridge.stateRootOverride = stateRoot;
  });

  tearDown(() async {
    DshWorkspaceBridge.dshHomeOverride = null;
    DshWorkspaceBridge.stateRootOverride = null;
    if (await sandbox.exists()) await sandbox.delete(recursive: true);
  });

  Future<Map<String, dynamic>> document() async =>
      jsonDecode(await DshWorkspaceBridge.bindingFile.readAsString())
          as Map<String, dynamic>;

  Future<Map<String, dynamic>> stateFile(String workspaceRef) async =>
      jsonDecode(
        await File(
          p.join(
            stateRoot,
            'dsh-bindings',
            '${DshWorkspaceBridge.safeSegment(workspaceRef)}.json',
          ),
        ).readAsString(),
      ) as Map<String, dynamic>;

  test('locator names are the vectors the DSH plugin derives', () {
    expect(DshWorkspaceBridge.safeSegment('mount:abc'), 'mount_abc');
    expect(DshWorkspaceBridge.safeSegment('mount/with/slash'), 'mount_with_slash');
    expect(DshWorkspaceBridge.safeSegment('中文 ref'), 'ref');
    expect(DshWorkspaceBridge.safeSegment(''), 'mount');
    expect(DshWorkspaceBridge.safeSegment('..dash-dot_ok'), '..dash-dot_ok');
    expect(DshWorkspaceBridge.safeSegment(List.filled(200, 'a').join()).length, 128);
  });

  test('provider families match the protocol vocabulary', () {
    expect(DshWorkspaceBridge.providerKindOf('muse.workspace.local.v1'), 'local');
    expect(DshWorkspaceBridge.providerKindOf('muse.workspace.cloud.v1'), 'cloud');
    expect(DshWorkspaceBridge.providerKindOf('muse.workspace.ssh.v2'), 'ssh-agent');
    expect(DshWorkspaceBridge.providerKindOf('muse.workspace.sftp.v1'), 'sftp');
    expect(DshWorkspaceBridge.providerKindOf('other'), 'unknown');
  });

  test('publishes a pathless document and writes one locator per local Mount',
      () async {
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0, readOnly: true),
        _mount('helix', helix, order: 1),
        _mount('docs', 'ignored', order: 2, providerId: 'muse.workspace.cloud.v1'),
      ],
      activeMountRef: 'mount:helix',
    );

    final raw = await DshWorkspaceBridge.bindingFile.readAsString();
    expect(raw, isNot(contains(frontend)));
    expect(raw, isNot(contains(helix)));

    final published = await document();
    expect(published['protocol'], 'muse.workspace/binding/v1');
    expect(published['bindingRevision'], 1);
    expect(published['workspaceRef'], 'ws-1');
    expect(published['title'], 'My Workspace');
    expect(published['activeMountRef'], 'mount:helix');
    final mounts = (published['mounts'] as List).cast<Map<String, dynamic>>();
    expect(
      mounts.map((mount) => mount['mountRef']).toList(),
      ['mount:frontend', 'mount:helix', 'mount:docs'],
    );
    expect(mounts[0]['requestedMode'], 'read-only');
    expect(mounts[0]['materialization'], {'mode': 'host-path'});
    expect(mounts[2]['providerKind'], 'cloud');
    expect(mounts[2]['materialization'], {'mode': 'read-only-projection'});
    expect(mounts.every((mount) => !mount.containsKey('path')), isTrue);

    expect(
      await DshWorkspaceBridge.locatorFile('mount:frontend').readAsString(),
      '$frontend\n',
    );
    expect(
      await DshWorkspaceBridge.locatorFile('mount:helix').readAsString(),
      '$helix\n',
    );
    expect(
      await DshWorkspaceBridge.locatorFile('mount:docs').exists(),
      isFalse,
      reason: 'a non-local Mount is projected, never pointed at a Host path',
    );
  });

  test('pushes the granted Mount directories so the core can search them',
      () async {
    final pushed = <Map<String, String>>[];
    final original = DshWorkspaceBridge.publishMountRoots;
    DshWorkspaceBridge.publishMountRoots = (roots) {
      pushed.add(Map.of(roots));
      return true;
    };
    addTearDown(() => DshWorkspaceBridge.publishMountRoots = original);

    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0, readOnly: true),
        _mount('helix', helix, order: 1),
        _mount('docs', 'ignored', order: 2, providerId: 'muse.workspace.cloud.v1'),
      ],
      activeMountRef: 'mount:helix',
    );

    expect(
      pushed.single,
      {'mount:frontend': frontend, 'mount:helix': helix},
      reason: 'the Rust core searches exactly the Mounts this Host grants',
    );

    // A workspace without Mounts publishes an empty catalog: the core must stop
    // searching directories this Host no longer grants.
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: const [],
    );
    expect(pushed.last, isEmpty);
  });

  test('bumps the revision, reuses DSH ids and sweeps a removed locator',
      () async {
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0),
        _mount('helix', helix, order: 1),
      ],
      activeMountRef: 'mount:frontend',
    );
    await DshWorkspaceBridge.receiptFile.writeAsString(
      jsonEncode({
        'protocol': 'muse.workspace/binding-receipt/v1',
        'bindingRevision': 1,
        'workspaceRef': 'ws-1',
        'mounts': [
          {
            'mountRef': 'mount:frontend',
            'dshWorkspaceId': 'ba80ad2f-9023-43ff-8326-a5364d88f22d',
            'state': 'bound',
          },
        ],
      }),
    );

    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [_mount('frontend', frontend, order: 0)],
      activeMountRef: 'mount:frontend',
    );

    final published = await document();
    expect(published['bindingRevision'], 2);
    final mounts = (published['mounts'] as List).cast<Map<String, dynamic>>();
    expect(mounts.single['dshWorkspaceId'], 'ba80ad2f-9023-43ff-8326-a5364d88f22d');
    expect(await DshWorkspaceBridge.locatorFile('mount:frontend').exists(), isTrue);
    expect(
      await DshWorkspaceBridge.locatorFile('mount:helix').exists(),
      isFalse,
      reason: 'a locator without a Mount must not survive a publish',
    );

    final state = await stateFile('ws-1');
    expect(state['protocol'], 'muse.workspace/dsh-binding-map/v1');
    expect(state['bindingRevision'], 2);
    expect(state['mountWorkspaceIds'], {
      'mount:frontend': 'ba80ad2f-9023-43ff-8326-a5364d88f22d',
    });
  });

  test('keeps the published Mounts when the AppFlowy workspace is republished',
      () async {
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [_mount('frontend', frontend, order: 0)],
      activeMountRef: 'mount:frontend',
    );
    await DshWorkspaceBridge.legacyHintFile.writeAsString('{"stale":true}');

    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [_mount('frontend', frontend, order: 0)],
      activeMountRef: 'mount:frontend',
    );

    final published = await document();
    expect(published['mounts'] as List, hasLength(1));
    expect(published['bindingRevision'], 2);
  });

  test('retires the legacy hint when no Mount is published', () async {
    await DshWorkspaceBridge.legacyHintFile.writeAsString('{"stale":true}');
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: const [],
    );

    final published = await document();
    expect(published['mounts'], isEmpty);
    expect(published.containsKey('activeMountRef'), isFalse);
    expect(await DshWorkspaceBridge.legacyHintFile.exists(), isFalse);
  });

  test('adopts the active Mount the DSH panel switched to', () async {
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0),
        _mount('helix', helix, order: 1),
      ],
      activeMountRef: 'mount:frontend',
    );
    final firstState = await stateFile('ws-1');
    final requestedAt = (firstState['updatedAt'] as int) + 1000;

    await DshWorkspaceBridge.activeIntentFile.writeAsString(
      jsonEncode({
        'protocol': DshWorkspaceBridge.activeIntentProtocol,
        'workspaceRef': 'ws-1',
        'mountRef': 'mount:helix',
        'requestedAt': requestedAt,
      }),
    );
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0),
        _mount('helix', helix, order: 1),
      ],
      activeMountRef: 'mount:frontend',
    );

    final adopted = await document();
    expect(adopted['activeMountRef'], 'mount:helix');
    expect(adopted['bindingRevision'], 2);
    final state = await stateFile('ws-1');
    expect(state['activeMountRef'], 'mount:helix');
    expect(state['updatedAt'], requestedAt);
  });

  test('ignores a stale, foreign or unresolvable active-Mount intent',
      () async {
    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [
        _mount('frontend', frontend, order: 0),
        _mount('helix', helix, order: 1),
      ],
      activeMountRef: 'mount:helix',
    );
    final state = await stateFile('ws-1');
    final stale = (state['updatedAt'] as int) - 1000;

    for (final intent in [
      {
        'protocol': DshWorkspaceBridge.activeIntentProtocol,
        'workspaceRef': 'ws-1',
        'mountRef': 'mount:frontend',
        'requestedAt': stale,
      },
      {
        'protocol': DshWorkspaceBridge.activeIntentProtocol,
        'workspaceRef': 'ws-other',
        'mountRef': 'mount:frontend',
        'requestedAt': stale + 2000,
      },
      {
        'protocol': DshWorkspaceBridge.activeIntentProtocol,
        'workspaceRef': 'ws-1',
        'mountRef': 'mount:missing',
        'requestedAt': stale + 2000,
      },
      {
        'protocol': 'muse.workspace/active-intent/v0',
        'workspaceRef': 'ws-1',
        'mountRef': 'mount:frontend',
        'requestedAt': stale + 2000,
      },
    ]) {
      await DshWorkspaceBridge.activeIntentFile
          .writeAsString(jsonEncode(intent));
      await DshWorkspaceBridge.publishProjectWorkspace(
        appflowyWorkspaceId: 'ws-1',
        title: 'My Workspace',
        mounts: [
          _mount('frontend', frontend, order: 0),
          _mount('helix', helix, order: 1),
        ],
        activeMountRef: 'mount:helix',
      );
      final published = await document();
      expect(
        published['activeMountRef'],
        'mount:helix',
        reason: 'intent $intent must not win over the Host',
      );
    }
  });

  test('sweeps stale temporary files left by an interrupted publish', () async {
    final stale = File(
      p.join(
        dshHome,
        'bindings',
        'workspace-binding.json.999.1.tmp',
      ),
    );
    await stale.writeAsString('partial');
    await stale.setLastModified(
      DateTime.now().subtract(const Duration(hours: 3)),
    );

    await DshWorkspaceBridge.publishProjectWorkspace(
      appflowyWorkspaceId: 'ws-1',
      title: 'My Workspace',
      mounts: [_mount('frontend', frontend, order: 0)],
    );

    expect(await stale.exists(), isFalse);
    final leftovers = (await Directory(p.join(dshHome, 'bindings'))
            .list()
            .toList())
        .whereType<File>()
        .where((file) => file.path.endsWith('.tmp'))
        .toList();
    expect(leftovers, isEmpty);
  });
}
