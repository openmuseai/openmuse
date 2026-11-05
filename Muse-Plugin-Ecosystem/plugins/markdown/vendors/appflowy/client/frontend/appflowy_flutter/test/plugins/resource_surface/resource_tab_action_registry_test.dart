import 'dart:io';

import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('external contributions are ordered, replaceable and executable', () {
    final registry = MuseResourceTabActionRegistry()
      ..register(
        MuseResourceTabAction(
          id: 'engine.viewer',
          label: 'Viewer',
          group: 10,
          order: 20,
          handler: (_, target) =>
              target.selectEngine(MuseLocalEngine.openFileViewer),
        ),
      )
      ..register(
        MuseResourceTabAction(
          id: 'engine.helix',
          label: 'Helix old',
          group: 10,
          order: 10,
          handler: (_, target) => target.selectEngine(MuseLocalEngine.helix),
        ),
      )
      ..register(
        MuseResourceTabAction(
          id: 'engine.helix',
          label: 'Helix',
          group: 10,
          order: 10,
          handler: (_, target) => target.selectEngine(MuseLocalEngine.helix),
        ),
      );
    final target = _Target();

    final actions = registry.actionsFor(target);
    expect(actions.map((action) => action.label), ['Helix', 'Viewer']);
    actions.first.invoke(_FakeBuildContext());
    expect(target.selectedEngine, MuseLocalEngine.helix);
  });

  test('open-with is one cascading action, not one row per engine', () {
    final registry = MuseResourceTabActionRegistry()
      ..register(
        MuseResourceTabAction(
          id: 'muse.open-with',
          label: '打开方式',
          handler: (_, __) {},
          submenuBuilder: (context, target, close) => const SizedBox.shrink(),
        ),
      );
    final actions = registry.actionsFor(_Target());
    expect(actions.single.id, 'muse.open-with');
    expect(actions.single.label, '打开方式');
    expect(actions.single.submenuBuilder, isNotNull);
  });
}

final class _Target implements MuseResourceTabTarget {
  MuseLocalEngine _engine = MuseLocalEngine.openFileViewer;

  @override
  MuseResolvedResource get resource => MuseResolvedResource(
        file: File('/tmp/example.rs'),
        engine: MuseLocalEngine.helix,
        origin: MuseResourceOpenOrigin.hostPicker,
      );

  @override
  MuseLocalEngine get selectedEngine => _engine;

  @override
  void selectEngine(MuseLocalEngine engine) => _engine = engine;
}

final class _FakeBuildContext implements BuildContext {
  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}
