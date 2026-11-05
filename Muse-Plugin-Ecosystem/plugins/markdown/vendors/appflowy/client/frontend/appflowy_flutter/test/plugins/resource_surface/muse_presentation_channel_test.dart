import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:appflowy/plugins/resource_surface/muse_presentation_channel.dart';
import 'package:appflowy/plugins/resource_surface/muse_presentation_dispatch.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface_dialog.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late _FakeTransport transport;
  late List<String> refusals;
  late RawReceivePort port;

  setUp(() {
    transport = _FakeTransport();
    refusals = [];
    port = RawReceivePort();
  });

  tearDown(() {
    try {
      port.close();
    } on Object {
      // A refused install already closed its port.
    }
  });

  MusePresentationChannelHost hostWith({
    MuseResourcePresenter? presenter,
  }) =>
      MusePresentationChannelHost(
        transport: transport,
        portFactory: () => port,
        onRefused: refusals.add,
        dispatcher: MuseResourceSurfaceDispatcher(
          mountRoot: (_) => Directory.systemTemp.path,
          presenter: presenter ??
              (request) async => MuseResourceSurfaceOpenResult.opened(
                    resource: MuseResolvedResource(
                      file: File(request.path),
                      engine: MuseLocalEngine.helix,
                      origin: request.origin,
                    ),
                    surfaceKey: 'resource:${request.path}',
                  ),
        ),
      );

  test('installs the surface dispatcher exactly once', () {
    final host = hostWith();

    expect(host.install(), isTrue);
    expect(host.installed, isTrue);
    // A second install is a no-op: one port, one Host dispatcher, for the
    // lifetime of the app.
    expect(host.install(), isFalse);
    expect(transport.installCalls, 1);
    expect(transport.installedPort, port.sendPort.nativePort);
    expect(refusals, isEmpty);

    host.dispose();
  });

  test('a refused install leaves nothing installed', () {
    transport.acceptInstall = false;
    final host = hostWith();

    expect(host.install(), isFalse);
    expect(host.installed, isFalse);
    expect(host.port, isNull);
    expect(refusals.single, contains('no surface dispatcher install'));

    host.dispose();
  });

  test('answers a posted dispatch with an outcome the Host can project',
      () async {
    final host = hostWith();
    expect(host.install(), isTrue);

    port.sendPort.send(
      utf8.encode(
        jsonEncode(_wire(requestRef: 'request.42')),
      ),
    );
    final answered = await _until(() => transport.answers.isNotEmpty);

    expect(answered, isTrue);
    final answer = transport.answers.single;
    expect(answer.requestRef, 'request.42');
    final wire = jsonDecode(answer.outcomeJson!) as Map<String, dynamic>;
    expect(wire['result'], 'opened');
    expect(wire['effectiveMode'], 'view');
    expect(wire['revision'], 'revision.7');
    expect(wire['layout'], 'resource');
    expect(wire['title'], 'spec.md');
    // Only opaque refs cross back to the Host.
    expect(answer.outcomeJson, isNot(contains('\\')));
    expect(answer.outcomeJson, isNot(contains(':/')));

    host.dispose();
  });

  test('answers a refusal with no outcome so the Host fails closed', () async {
    final host = hostWith(
      presenter: (_) async => const MuseResourceSurfaceOpenResult.unavailable(),
    );
    expect(host.install(), isTrue);

    port.sendPort.send(
      utf8.encode(jsonEncode(_wire(requestRef: 'request.43'))),
    );
    final answered = await _until(() => transport.answers.isNotEmpty);

    expect(answered, isTrue);
    expect(transport.answers.single.requestRef, 'request.43');
    expect(transport.answers.single.outcomeJson, isNull);
    expect(refusals.single, contains(MusePresentationReason.surfaceUnavailable));

    host.dispose();
  });

  test('publishes the granted Mount catalog the Rust core searches', () {
    final host = hostWith();

    expect(
      host.publishMountRoots({
        'mount:0d71': r'D:\agentic\src\openmuse-io\vendors\helix',
        'mount:914e': r'D:\agentic\src\openmuse-io\openmuse',
      }),
      isTrue,
    );
    expect(
      jsonDecode(transport.publishedMountRoots.single),
      {
        'mount:0d71': r'D:\agentic\src\openmuse-io\vendors\helix',
        'mount:914e': r'D:\agentic\src\openmuse-io\openmuse',
      },
    );

    // An empty catalog is meaningful: it retires every Mount this Host no longer
    // grants, and a blank directory is never one.
    expect(host.publishMountRoots({'mount:0d71': '   '}), isTrue);
    expect(jsonDecode(transport.publishedMountRoots.last), <String, Object?>{});
    expect(refusals, isEmpty);

    host.dispose();
  });

  test('a refused Mount catalog is reported, never assumed', () {
    transport.acceptMountRoots = false;
    final host = hostWith();

    expect(host.publishMountRoots({'mount:0d71': r'D:\helix'}), isFalse);
    expect(refusals.single, contains('granted Mount catalog'));

    host.dispose();
  });

  test('a build without the export publishes nothing and warns about nothing',
      () {
    final host = MusePresentationChannelHost(
      transport: _UnavailableTransport(),
      dispatcher: MuseResourceSurfaceDispatcher(),
      onRefused: refusals.add,
      portFactory: () => port,
    );

    expect(host.publishMountRoots({'mount:0d71': r'D:\helix'}), isFalse);
    expect(
      refusals,
      isEmpty,
      reason: 'a dart_ffi.dll from before this seam is not a failure',
    );

    host.dispose();
  });

  test('a dispatch with no requestRef is never answered blindly', () async {
    final host = hostWith();
    expect(host.install(), isTrue);

    port.sendPort.send(utf8.encode(jsonEncode(_wire(requestRef: null))));
    final refused = await _until(
      () => refusals.any((message) => message.contains('without a requestRef')),
    );

    expect(refused, isTrue);
    expect(transport.answers, isEmpty);
    expect(
      refusals.any((message) => message.contains('without a requestRef')),
      isTrue,
    );

    host.dispose();
  });
}

/// One `muse.presentation/dispatch/v1` payload.
Map<String, Object?> _wire({
  String? requestRef = 'request.1',
  String? mountRef = 'mount:test',
  String? relativePath = 'notes/spec.md',
}) =>
    <String, Object?>{
      'protocol': musePresentationDispatchProtocol,
      'requestRef': requestRef,
      'resourceRef': 'resource.abc',
      'workspaceId': 'workspace.1',
      'disposition': 'open',
      'requestedMode': 'view',
      'placementHint': 'current-window',
      'adapterRef': museHostResolvedAdapterRef,
      'effectiveMode': 'view',
      'sessionRef': 'session.1',
      'revision': 'revision.7',
      'causeKind': 'deliverable',
      'resolved': true,
      if (mountRef != null) 'mountRef': mountRef,
      if (relativePath != null) 'relativePath': relativePath,
    };

final class _Answer {
  const _Answer(this.requestRef, this.outcomeJson);

  final String requestRef;
  final String? outcomeJson;
}

final class _FakeTransport implements MusePresentationTransport {
  bool acceptInstall = true;
  bool acceptMountRoots = true;
  int installCalls = 0;
  int? installedPort;
  final List<_Answer> answers = [];
  final List<String> publishedMountRoots = [];

  @override
  bool get available => true;

  @override
  bool install(int port) {
    installCalls++;
    if (!acceptInstall) return false;
    installedPort = port;
    return true;
  }

  @override
  bool complete(String requestRef, String? outcomeJson) {
    answers.add(_Answer(requestRef, outcomeJson));
    return true;
  }

  @override
  bool publishMountRoots(String rootsJson) {
    if (!acceptMountRoots) return false;
    publishedMountRoots.add(rootsJson);
    return true;
  }
}

/// A build whose `dart_ffi.dll` predates the presentation exports.
final class _UnavailableTransport implements MusePresentationTransport {
  @override
  bool get available => false;

  @override
  bool install(int port) => false;

  @override
  bool complete(String requestRef, String? outcomeJson) => false;

  @override
  bool publishMountRoots(String rootsJson) => false;
}

/// Pumps the event loop until [ready] holds or the budget runs out.
Future<bool> _until(bool Function() ready) async {
  for (var attempt = 0; attempt < 200; attempt++) {
    if (ready()) return true;
    await Future<void>.delayed(const Duration(milliseconds: 5));
  }
  return ready();
}
