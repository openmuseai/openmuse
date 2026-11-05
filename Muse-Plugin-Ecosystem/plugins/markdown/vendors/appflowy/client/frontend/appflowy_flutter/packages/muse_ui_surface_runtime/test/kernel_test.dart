import 'package:flutter_test/flutter_test.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

void main() {
  test('U03-01 contribution order and conflicts are deterministic', () {
    final registry = MuseContributionRegistry();
    registry.register(_contribution('z.owner', 1));
    registry.register(_contribution('a.owner', 10));
    expect(registry.selected.single.ownerRef, 'a.owner');
    expect(registry.diagnostics.single.code, 'CONTRIBUTION_CONFLICT');
    expect(registry.diagnostics.single.selectedOwner, 'a.owner');
  });

  test('U03-02 multi-window focus honors visible pin and recent input', () {
    final focus = MuseFocusArbiter();
    focus.update(const MuseFocusCandidate(
        surfaceRef: 'one',
        windowRef: 'w1',
        visible: true,
        lastUserInputAt: 10,
        focused: true));
    focus.update(const MuseFocusCandidate(
        surfaceRef: 'two',
        windowRef: 'w2',
        visible: true,
        lastUserInputAt: 20,
        focused: true));
    expect(focus.primary, 'two');
    focus.pin('one');
    expect(focus.primary, 'one');
    focus.remove('one');
    expect(focus.primary, 'two');
  });

  test('U03-03 high frequency context coalesces, expires and obeys budget', () {
    var now = 1000;
    final focus = MuseFocusArbiter()
      ..update(const MuseFocusCandidate(
          surfaceRef: 'surface.1',
          windowRef: 'w1',
          visible: true,
          lastUserInputAt: 1,
          focused: true));
    final coordinator = MuseContextCoordinator(
        focus: focus, clock: () => now, maxSnapshotBytes: 1200);
    for (var revision = 1; revision <= 10000; revision++) {
      coordinator.ingest(_context('surface.1', 'selection', '$revision', now));
      coordinator.ingest(_context('surface.1', 'viewport', '$revision', now));
    }
    expect(coordinator.itemCount, 2);
    final snapshot = coordinator.snapshot();
    expect(snapshot.items.map((value) => value.contextRevision).toSet(),
        {'10000'});
    expect(snapshot.bytes, lessThanOrEqualTo(1200));
    now = 3000;
    expect(coordinator.snapshot().items, isEmpty);
  });

  test('U03-04 switching surfaces revokes old context before next snapshot',
      () {
    final focus = MuseFocusArbiter()
      ..update(const MuseFocusCandidate(
          surfaceRef: 'markdown',
          windowRef: 'w1',
          visible: true,
          lastUserInputAt: 1,
          focused: true));
    final coordinator = MuseContextCoordinator(focus: focus, clock: () => 1000);
    coordinator.ingest(_context('markdown', 'selection', '1', 1000));
    expect(coordinator.snapshot().primarySurfaceRef, 'markdown');
    coordinator.revokeSurface('markdown');
    focus.remove('markdown');
    focus.update(const MuseFocusCandidate(
        surfaceRef: 'mock',
        windowRef: 'w1',
        visible: true,
        lastUserInputAt: 2,
        focused: true));
    coordinator.ingest(_context('mock', 'active-cell', '1', 1000));
    final snapshot = coordinator.snapshot();
    expect(snapshot.primarySurfaceRef, 'mock');
    expect(snapshot.items.map((value) => value.contextType), ['active-cell']);
  });

  test(
      'U03-05 renderer routes without exposing Flutter callbacks to intent payload',
      () async {
    final registry = MuseRendererRegistry();
    final lease = MuseFacetLease(facetRef: 'mock/ui', generation: 1);
    registry.register(
        MuseRendererRegistration(
          rendererId: 'typed.mock',
          kind: MuseRendererKind.typedFlutter,
          surfaceKinds: const {'mock.editor'},
          build: (_, payload) => 'render:$payload',
        ),
        lease);
    expect(registry.render('mock.editor', 7), 'render:7');
    await lease.close();
    expect(registry.count, 0);
    expect(() => registry.render('mock.editor', 7), throwsStateError);
  });

  test('U03-06 generation disable reverse-disposes every owned effect',
      () async {
    final kernel = MusePresentationKernel();
    final events = <String>[];
    kernel.contributions.register(MuseContribution(
      ownerRef: 'mock/ui',
      slot: 'workspace.editor',
      contributionId: 'mock',
      generation: 3,
      priority: 1,
      activate: (lease) {
        lease.add(() => events.add('first'));
        lease.add(() => events.add('second'));
      },
    ));
    await kernel.activateGeneration(3);
    expect(kernel.effectsForGeneration(3), 2);
    await kernel.disposeGeneration(3);
    expect(events, ['second', 'first']);
    expect(kernel.effectsForGeneration(3), 0);
  });

  test('U03-07 grants are verifiable only by the trusted approval host', () {
    const request = MuseApprovalRequest(
        requestRef: 'approval.1', operation: 'document.write', expiresAt: 2000);
    final trusted = MuseTrustedApprovalHost(clock: () => 1000);
    final attacker = MuseTrustedApprovalHost(clock: () => 1000);
    final grant = trusted.approve(request);
    expect(trusted.verify(grant), isTrue);
    expect(attacker.verify(grant), isFalse);
    expect(() => MuseTrustedApprovalHost(clock: () => 3000).approve(request),
        throwsStateError);
  });

  test(
      'U03-08 web compatibility surface requires isolated loopback/CSP/storage',
      () {
    final surface = MuseWebCompatibilitySurface(
      origin: 'http://127.0.0.1:43121',
      csp: "default-src 'none'; frame-ancestors 'none'",
      storageNamespace: 'muse-g1',
    );
    expect(surface.storageNamespace, 'muse-g1');
    expect(
        () => MuseWebCompatibilitySurface(
              origin: 'https://evil.example',
              csp: "default-src 'none'; frame-ancestors 'none'",
              storageNamespace: 'x',
            ),
        throwsArgumentError);
  });

  test(
      'U03-09 default experiences are removable contributions, not bootstrap imports',
      () {
    final values = museDefaultExperienceContributions(
      generation: 1,
      workspace: (_) {},
      assistant: (_) {},
      markdown: (_) {},
    );
    expect(values.map((value) => value.ownerRef), [
      'muse.experience.workspace',
      'muse.experience.assistant',
      'muse.appflowy.markdown/ui',
    ]);
    expect(values.every((value) => value.generation == 1), isTrue);
  });
}

MuseContribution _contribution(String owner, int priority) => MuseContribution(
      ownerRef: owner,
      slot: 'workspace.editor',
      contributionId: 'mock',
      generation: 1,
      priority: priority,
      activate: (_) {},
    );

MuseContextContributionV1 _context(
  String surface,
  String type,
  String revision,
  int now,
) =>
    MuseContextContributionV1(
      pluginId: 'muse.test',
      pluginVersion: '2.0.0',
      facetInstanceRef: 'facet.1',
      surfaceInstanceRef: surface,
      surfaceKind: 'test.editor',
      scopeRef: 'workspace.1',
      contextType: type,
      contextSchemaDigest:
          'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
      contextRevision: revision,
      epochRef: 'epoch.1',
      lane: MuseContextLane.control,
      capturedAt: now,
      expiresAt: now + 1000,
      payload: const {'value': true},
    );
