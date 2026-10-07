import 'dart:convert';
import 'dart:io';

import 'audit.dart';
import 'client.dart';
import 'fake_publish.dart';
import 'fake_video.dart';
import 'fake_web.dart';
import 'host.dart';
import 'media.dart';
import 'provider.dart';
import 'transport.dart';

const acceptanceLabNotice = '验收桌面：不会访问社媒网站';

const acceptanceContext = RemoteConnectionContext(
  actorRef: 'actor.acceptance',
  mobileDeviceRef: 'mobile.acceptance',
  desktopDeviceRef: 'desktop.1',
  workspaceRef: 'ws.opaque.1',
  permissions: {
    'workspace.resource.read',
    'workspace.resource.write',
    'social.content.publish',
  },
);

final acceptancePng = base64Decode(
  'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==',
);

final class AcceptanceDesktop {
  AcceptanceDesktop({
    Directory? directory,
    DateTime? now,
    String workspaceRef = 'ws.opaque.1',
    String desktopDeviceRef = 'desktop.1',
  }) : publish = FakePublishSurfaceProvider(workspaceRef: workspaceRef),
       video = FakeVideoEditProvider(workspaceRef: workspaceRef),
       snapshot = FakeWebSnapshotProvider(workspaceRef: workspaceRef),
       interactive = FakeWebInteractiveProvider(workspaceRef: workspaceRef),
       media = RemoteMediaAuthority(),
       audit = RemoteAuditLog(directory: directory),
       jobs = RemoteJobLedger(directory: directory) {
    final started = now ?? DateTime.now();
    DateTime current() => now ?? DateTime.now();
    connection = RemoteConnectionContext(
      actorRef: acceptanceContext.actorRef,
      mobileDeviceRef: acceptanceContext.mobileDeviceRef,
      desktopDeviceRef: desktopDeviceRef,
      workspaceRef: workspaceRef,
      permissions: acceptanceContext.permissions,
    );
    host = RemoteSurfaceHost(audit: audit, jobs: jobs, clock: current);
    for (final handle in [
      'media.cover.1',
      'media.poster.1',
      'media.video.1',
      'media.snapshot.1',
    ]) {
      media.issue(
        handle: handle,
        bytes: acceptancePng,
        workspaceRef: connection.workspaceRef,
        deviceRef: connection.desktopDeviceRef,
        expiresAt: started.add(const Duration(minutes: 30)),
      );
    }
    host
      ..register(publish)
      ..register(video)
      ..register(snapshot)
      ..register(interactive);
    controller = RemoteWorkbenchController(
      transport: InMemoryRemoteSurfaceTransport(host),
      context: connection,
      notice: acceptanceLabNotice,
      loadMedia: (handle) async => media.read(
        handle: handle,
        workspaceRef: connection.workspaceRef,
        deviceRef: connection.desktopDeviceRef,
        now: current(),
      ),
    );
  }

  final FakePublishSurfaceProvider publish;
  final FakeVideoEditProvider video;
  final FakeWebSnapshotProvider snapshot;
  final FakeWebInteractiveProvider interactive;
  final RemoteMediaAuthority media;
  final RemoteAuditLog audit;
  final RemoteJobLedger jobs;
  late final RemoteConnectionContext connection;
  late final RemoteSurfaceHost host;
  late final RemoteWorkbenchController controller;
}
