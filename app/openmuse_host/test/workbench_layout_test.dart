import 'dart:io';

import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/layout/surface_mutation_guard.dart';

SurfaceBinding binding(String id) => SurfaceBinding(
  bindingId: id,
  surfaceRef: 'surface.$id',
  instanceRef: 'instance.$id',
);

void main() {
  group('layout model', () {
    test('JSON v1 round-trips an immutable snapshot', () {
      final original = WorkbenchLayoutSnapshot(
        root: SplitNode(
          axis: Axis.horizontal,
          ratio: 0.35,
          first: PaneNode(paneId: 'left'),
          second: PaneNode(paneId: 'right'),
        ),
        bindings: {'right': binding('document')},
        focusedPaneId: 'right',
      );

      final restored = WorkbenchLayoutSnapshot.tryDecode(original.encode());

      expect(restored, original);
      expect(restored!.toJson(), original.toJson());
      expect(
        () => restored.bindings['left'] = binding('other'),
        throwsUnsupportedError,
      );
    });

    test('safe parser rejects invalid versions and invariant violations', () {
      expect(WorkbenchLayoutSnapshot.tryDecode('{"version":1,"root":'), isNull);
      expect(
        WorkbenchLayoutSnapshot.tryFromJson({
          'version': 2,
          'root': {'type': 'pane', 'paneId': 'editor'},
          'bindings': <String, Object?>{},
        }),
        isNull,
      );
      expect(
        WorkbenchLayoutSnapshot.tryFromJson({
          'version': 1,
          'root': {
            'type': 'split',
            'axis': 'horizontal',
            'ratio': 0.5,
            'first': {'type': 'pane', 'paneId': 'same'},
            'second': {'type': 'pane', 'paneId': 'same'},
          },
          'bindings': <String, Object?>{},
        }),
        isNull,
      );
    });

    test('default factory creates workspace, editor, and dsh panes', () {
      final snapshot = WorkbenchLayoutDefaults.threePane();

      expect(snapshot.paneIds, {'workspace', 'editor', 'dsh'});
      expect(snapshot.focusedPaneId, 'editor');
      expect(
        snapshot.bindingFor('editor')?.surfaceRef,
        'host.editorGroup:editor.primary',
      );
    });
  });

  group('controller', () {
    test('split and close empty pane merge its sibling', () {
      final controller = WorkbenchLayoutController(
        WorkbenchLayoutSnapshot(
          root: PaneNode(paneId: 'editor'),
          focusedPaneId: 'editor',
        ),
      );

      controller.splitPane(
        'editor',
        axis: Axis.vertical,
        ratio: 0.6,
        newPaneId: 'preview',
      );
      expect(controller.snapshot.paneIds, {'editor', 'preview'});
      expect(controller.snapshot.root, isA<SplitNode>());

      expect(controller.closeEmptyPane('preview'), isTrue);
      expect(controller.snapshot.root, PaneNode(paneId: 'editor'));
      expect(controller.closeEmptyPane('editor'), isFalse);
    });

    test('move and swap transfer surface bindings', () {
      final controller = WorkbenchLayoutController(
        WorkbenchLayoutSnapshot(
          root: SplitNode(
            axis: Axis.horizontal,
            ratio: 0.5,
            first: PaneNode(paneId: 'a'),
            second: SplitNode(
              axis: Axis.vertical,
              ratio: 0.5,
              first: PaneNode(paneId: 'b'),
              second: PaneNode(paneId: 'c'),
            ),
          ),
          bindings: {'a': binding('one'), 'b': binding('two')},
        ),
      );

      controller.swap('a', 'b');
      expect(controller.snapshot.bindingFor('a')?.bindingId, 'two');
      expect(controller.snapshot.bindingFor('b')?.bindingId, 'one');

      controller.move('b', 'c');
      expect(controller.snapshot.bindingFor('b'), isNull);
      expect(controller.snapshot.bindingFor('c')?.bindingId, 'one');
      expect(controller.snapshot.focusedPaneId, 'c');
    });

    test('focus and resize preserve valid state', () {
      final controller = WorkbenchLayoutController(
        WorkbenchLayoutSnapshot(
          root: SplitNode(
            axis: Axis.horizontal,
            ratio: 0.5,
            first: PaneNode(paneId: 'a'),
            second: PaneNode(paneId: 'b'),
          ),
        ),
      );

      controller.focus('b');
      controller.resize('a', 0.7);

      expect(controller.snapshot.focusedPaneId, 'b');
      expect((controller.snapshot.root as SplitNode).ratio, 0.7);
      expect(() => controller.resize('a', 1), throwsArgumentError);
    });

    test('repeated split, swap, and close remains valid', () {
      final controller = WorkbenchLayoutController(
        WorkbenchLayoutSnapshot(
          root: PaneNode(paneId: 'root'),
          bindings: {'root': binding('root')},
        ),
      );

      for (var index = 0; index < 200; index++) {
        final pane = controller.splitPane(
          'root',
          axis: index.isEven ? Axis.horizontal : Axis.vertical,
          newPaneId: 'temporary-$index',
        );
        controller.swap('root', pane);
        controller.swap('root', pane);
        expect(controller.unbind(pane), isNull);
        expect(controller.closeEmptyPane(pane), isTrue);
      }

      expect(controller.snapshot.paneIds, {'root'});
      expect(controller.snapshot.bindingFor('root')?.bindingId, 'root');
    });
  });

  group('geometry and neighbors', () {
    final root = SplitNode(
      axis: Axis.horizontal,
      ratio: 0.5,
      first: PaneNode(paneId: 'left'),
      second: SplitNode(
        axis: Axis.vertical,
        ratio: 0.5,
        first: PaneNode(paneId: 'topRight'),
        second: PaneNode(paneId: 'bottomRight'),
      ),
    );

    test('solver emits pane and divider rectangles including gutters', () {
      final geometry = const WorkbenchLayoutSolver().solve(
        root,
        const Size(1000, 600),
        gutter: 10,
      );

      expect(geometry.paneRects['left'], const Rect.fromLTWH(0, 0, 495, 600));
      expect(
        geometry.paneRects['topRight'],
        const Rect.fromLTWH(505, 0, 495, 295),
      );
      expect(
        geometry.paneRects['bottomRight'],
        const Rect.fromLTWH(505, 305, 495, 295),
      );
      expect(geometry.dividers, hasLength(2));
      expect(
        geometry.dividers.first.rect,
        const Rect.fromLTWH(495, 0, 10, 600),
      );
    });

    test('divider endpoints resize the matching ancestor split', () {
      final nested = SplitNode(
        axis: Axis.horizontal,
        ratio: 0.3,
        first: SplitNode(
          axis: Axis.vertical,
          ratio: 0.4,
          first: PaneNode(paneId: 'a'),
          second: PaneNode(paneId: 'b'),
        ),
        second: PaneNode(paneId: 'c'),
      );
      final controller = WorkbenchLayoutController(
        WorkbenchLayoutSnapshot(root: nested),
      );
      final divider = const WorkbenchLayoutSolver()
          .solve(nested, const Size(1000, 600))
          .dividers
          .firstWhere((divider) => divider.path.isEmpty);

      expect(divider.leadingPaneId, 'a');
      expect(divider.trailingPaneId, 'c');
      expect(
        controller.ratioBetween(divider.leadingPaneId, divider.trailingPaneId),
        0.3,
      );
      controller.resizeBetween(
        divider.leadingPaneId,
        divider.trailingPaneId,
        0.6,
      );

      final updated = controller.snapshot.root as SplitNode;
      expect(updated.ratio, 0.6);
      expect((updated.first as SplitNode).ratio, 0.4);
    });

    test('neighbors are selected from visual rectangles', () {
      final geometry = const WorkbenchLayoutSolver().solve(
        root,
        const Size(1000, 600),
        gutter: 10,
      );

      expect(geometry.findNeighbor('left', PaneDirection.right), 'topRight');
      expect(
        geometry.findNeighbor('topRight', PaneDirection.down),
        'bottomRight',
      );
      expect(geometry.findNeighbor('bottomRight', PaneDirection.left), 'left');
      expect(geometry.findNeighbor('left', PaneDirection.left), isNull);
    });
  });

  test(
    'LayoutStore atomically saves and falls back from corrupt JSON',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-layout-test-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final file = File('${directory.path}/layout.json');
      final fallback = createDefaultWorkbenchLayout();
      final store = LayoutStore(file, fallback: fallback);
      final saved = WorkbenchLayoutSnapshot(
        root: PaneNode(paneId: 'only'),
        focusedPaneId: 'only',
      );

      await store.save(saved);
      expect(await store.load(), saved);
      expect(await File('${file.path}.tmp').exists(), isFalse);

      await file.writeAsString('{broken');
      expect(await store.load(), fallback);
    },
  );

  test('surface mutation guards aggregate defer and deny decisions', () async {
    final guards = SurfaceMutationGuards();
    guards.register(
      'surface.native',
      _Guard(
        const SurfaceMutationResult(
          SurfaceMutationDecision.defer,
          reason: 'IME composition',
        ),
      ),
    );

    expect(
      (await guards.prepare([
        'surface.native',
      ], SurfaceMutationKind.swap)).decision,
      SurfaceMutationDecision.defer,
    );
    guards.register(
      'surface.unspecified',
      const _Guard(SurfaceMutationResult(SurfaceMutationDecision.defer)),
    );
    expect(
      (await guards.prepare([
        'surface.unspecified',
      ], SurfaceMutationKind.move)).decision,
      SurfaceMutationDecision.defer,
    );

    guards.register(
      'surface.web',
      _Guard(
        const SurfaceMutationResult(
          SurfaceMutationDecision.deny,
          reason: 'modal interaction',
        ),
      ),
    );
    expect(
      (await guards.prepare([
        'surface.native',
        'surface.web',
      ], SurfaceMutationKind.close)).decision,
      SurfaceMutationDecision.deny,
    );
  });
}

final class _Guard implements SurfaceMutationGuard {
  const _Guard(this.result);
  final SurfaceMutationResult result;

  @override
  Future<SurfaceMutationResult> prepareMutation(
    SurfaceMutationKind kind,
  ) async => result;
}
