import 'package:flutter_test/flutter_test.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

void main() {
  const digest =
      'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';

  test('focus is exclusive per window and leases close deterministically',
      () async {
    final sink = _Sink();
    final runtime = MuseUiSurfaceRuntime(sink: sink);
    runtime.registerFacet(_registration);
    final firstFacet = _Facet();
    final secondFacet = _Facet();
    final first = runtime.openSurface(_open('one'), firstFacet);
    final second = runtime.openSurface(_open('two'), secondFacet);

    runtime.setFocused(first, focused: true);
    runtime.setFocused(second, focused: true);
    expect(
        runtime.inventory[first.surfaceInstanceRef], MuseSurfaceState.active);
    expect(
        runtime.inventory[second.surfaceInstanceRef], MuseSurfaceState.focused);

    await runtime.closeSurface(second);
    expect(sink.closed, [second.surfaceInstanceRef]);
    expect(secondFacet.disposed, isTrue);
    expect(
      () => runtime.setActive(second, active: true),
      throwsA(isA<MuseSurfaceRuntimeException>()),
    );
    await runtime.dispose();
  });

  test('state lane coalesces and control lane bypasses debounce', () async {
    var now = 1000;
    final sink = _Sink();
    final runtime = MuseUiSurfaceRuntime(
      sink: sink,
      stateDebounce: const Duration(milliseconds: 20),
      clock: () => now,
    );
    runtime.registerFacet(_registration);
    final lease = runtime.openSurface(_open('one'), _Facet());
    runtime.setActive(lease, active: true);

    await runtime.publishContext(
        lease, _context(lease, 'selection', '1', MuseContextLane.state, now));
    await runtime.publishContext(
        lease, _context(lease, 'selection', '2', MuseContextLane.state, now));
    await runtime.publishContext(
        lease, _context(lease, 'focus', '1', MuseContextLane.control, now));
    expect(sink.items.map((item) => item.contextType), ['focus']);
    await Future<void>.delayed(const Duration(milliseconds: 35));
    expect(sink.items.map((item) => item.contextRevision), ['1', '2']);
    await runtime.dispose();
  });

  test('background state is suppressed and stale revisions fail closed',
      () async {
    const now = 1000;
    final sink = _Sink();
    final runtime = MuseUiSurfaceRuntime(sink: sink, clock: () => now);
    runtime.registerFacet(_registration);
    final lease = runtime.openSurface(_open('one'), _Facet());
    await runtime.publishContext(
        lease, _context(lease, 'selection', '1', MuseContextLane.state, now));
    expect(sink.items, isEmpty);
    expect(
      () => runtime.publishContext(
          lease, _context(lease, 'selection', '1', MuseContextLane.state, now)),
      throwsA(isA<MuseSurfaceRuntimeException>()),
    );
    await runtime.dispose();
  });

  test('domain change routes by plugin and resource, not Host scope', () async {
    final sink = _Sink();
    final runtime = MuseUiSurfaceRuntime(sink: sink);
    runtime.registerFacet(_registration);
    final matching = _Facet(shouldThrow: true);
    final other = _Facet();
    runtime.openSurface(_open('one'), matching);
    runtime.openSurface(_open('two'), other);
    await runtime.routeDomainChange(MuseDomainChangeV1(
      pluginId: 'muse.test',
      providerInstanceRef: 'provider.1',
      scopeRef: 'scope.host.opaque',
      resourceRef: 'one',
      eventType: 'test.changed',
      eventSchemaDigest: digest,
      domainRevision: '1',
      epochRef: 'epoch.1',
      origin: MuseMutationOrigin.externalCommand,
      occurredAt: 1000,
      payload: const {'changed': true},
    ));
    expect(matching.changes, 1);
    expect(other.changes, 0);
    await runtime.dispose();
  });

  test('presentation intent targets focused surface and fails closed',
      () async {
    const now = 1000;
    final runtime = MuseUiSurfaceRuntime(sink: _Sink(), clock: () => now);
    runtime.registerFacet(_registration);
    final facet = _IntentFacet();
    final lease = runtime.openSurface(_open('one'), facet);
    runtime.setFocused(lease, focused: true);
    final applied = await runtime.routePresentationIntent(_intent(now));
    expect(applied.status, MusePresentationIntentStatus.applied);
    expect(applied.appliedSurfaceInstanceRef, 'surface.applied');
    final stale =
        await runtime.routePresentationIntent(_intent(now, expiresAt: now));
    expect(stale.status, MusePresentationIntentStatus.stale);
    await runtime.dispose();
  });
}

const _registration = MuseFacetRegistration(
  pluginId: 'muse.test',
  pluginVersion: '1.0.0',
  facetInstanceRef: 'facet.1',
  surfaceKinds: {'test.editor'},
);

MuseOpenSurface _open(String resource) => MuseOpenSurface(
      facetInstanceRef: 'facet.1',
      surfaceKind: 'test.editor',
      scopeRef: 'workspace.1',
      resourceRef: resource,
      windowRef: 'window.1',
    );

MuseContextContributionV1 _context(
  MuseSurfaceLease lease,
  String type,
  String revision,
  MuseContextLane lane,
  int now,
) =>
    MuseContextContributionV1(
      pluginId: 'muse.test',
      pluginVersion: '1.0.0',
      facetInstanceRef: 'facet.1',
      surfaceInstanceRef: lease.surfaceInstanceRef,
      surfaceKind: 'test.editor',
      scopeRef: 'workspace.1',
      contextType: type,
      contextSchemaDigest:
          'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
      contextRevision: revision,
      epochRef: 'epoch.1',
      lane: lane,
      capturedAt: now,
      expiresAt: now + 1000,
      payload: const {'value': true},
    );

MusePresentationIntentV1 _intent(int now, {int? expiresAt}) =>
    MusePresentationIntentV1(
      pluginId: 'muse.test',
      scopeRef: 'workspace.1',
      intentType: 'test.reveal',
      intentSchemaDigest:
          'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
      intentRef: 'intent.1',
      requestedAt: now,
      expiresAt: expiresAt ?? now + 1000,
      payload: const {'resourceRef': 'one'},
    );

final class _Sink implements MuseSurfaceContextSink {
  final items = <MuseContextContributionV1>[];
  final closed = <String>[];

  @override
  Future<void> closeSurface(String surfaceInstanceRef, String scopeRef) async {
    closed.add(surfaceInstanceRef);
  }

  @override
  Future<void> publish(MuseContextContributionV1 context) async {
    items.add(context);
  }
}

class _Facet implements MuseUiFacet {
  _Facet({this.shouldThrow = false});
  final bool shouldThrow;
  var disposed = false;
  var changes = 0;

  @override
  Future<void> dispose() async => disposed = true;

  @override
  Future<void> onDomainChange(MuseDomainChangeV1 change) async {
    changes++;
    if (shouldThrow) throw StateError('isolated');
  }
}

final class _IntentFacet extends _Facet
    implements MusePresentationIntentHandler {
  @override
  Future<MusePresentationIntentResultV1> onPresentationIntent(
    MusePresentationIntentV1 intent,
  ) async =>
      MusePresentationIntentResultV1(
        intentRef: intent.intentRef,
        status: MusePresentationIntentStatus.applied,
        appliedSurfaceInstanceRef: 'surface.applied',
        completedAt: 1000,
      );
}
