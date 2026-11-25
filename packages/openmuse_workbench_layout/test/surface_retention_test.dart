import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

SurfaceBinding binding(String id) => SurfaceBinding(
  bindingId: 'binding.$id',
  surfaceRef: id,
  instanceRef: 'instance.$id',
);
WorkbenchLayoutSnapshot pair() => WorkbenchLayoutSnapshot(
  root: SplitNode(
    axis: Axis.horizontal,
    ratio: .5,
    first: PaneNode(paneId: 'a'),
    second: PaneNode(paneId: 'b'),
  ),
  bindings: {'a': binding('a'), 'b': binding('b')},
  focusedPaneId: 'a',
);
void main() {
  test(
    'additional surfaces preserve existing region and avoid reserved pane names',
    () {
      final before = pair();
      final added = binding('missing');
      final target = WorkbenchLayoutSnapshot(
        root: SplitNode(
          axis: Axis.horizontal,
          ratio: .5,
          first: PaneNode(paneId: 'editor'),
          second: PaneNode(paneId: 'recovered-1'),
        ),
        bindings: {'recovered-1': binding('sidebar')},
      );
      final after = reframeWorkbenchSurfaces(
        current: before,
        target: target,
        targetPaneId: 'editor',
        retainedInstances: {'instance.a', 'instance.b'},
        additionalSurfaces: [added],
      );
      expect(after.bindings['a'], same(before.bindings['a']));
      expect(after.bindings['b'], same(before.bindings['b']));
      expect(after.bindings['recovered-2'], same(added));
      expect(after.bindings['recovered-1']!.surfaceRef, 'sidebar');
      expect(after.focusedPaneId, 'a');
      expect(WorkbenchLayoutSnapshot.tryDecode(after.encode()), after);
      expect(before.paneIds, {'a', 'b'});
    },
  );
  test(
    'missing surfaces can form the editor region without a currently retained pane',
    () {
      final before = pair();
      final after = reframeWorkbenchSurfaces(
        current: before,
        target: createDefaultWorkbenchLayout(),
        targetPaneId: 'editor',
        retainedInstances: {},
        additionalSurfaces: [
          binding('first'),
          binding('second'),
          binding('third'),
        ],
      );
      expect(
        after.bindings.values
            .where((b) => b.instanceRef.startsWith('instance.'))
            .map((b) => b.surfaceRef),
        ['first', 'second', 'third'],
      );
      expect(after.focusedPaneId, isNotNull);
      expect(after.paneIds.contains(after.focusedPaneId), isTrue);
      expect(WorkbenchLayoutSnapshot.tryDecode(after.encode()), after);
    },
  );
  test(
    'additional identity conflicts and layout budgets refuse without mutating either input',
    () {
      final before = pair(), target = createDefaultWorkbenchLayout();
      final original = before.encode(), originalTarget = target.encode();
      for (final additions in [
        [binding('a')],
        [binding('same'), binding('same')],
        List.generate(20, (i) => binding('many-$i')),
      ]) {
        expect(
          () => reframeWorkbenchSurfaces(
            current: before,
            target: target,
            targetPaneId: 'editor',
            retainedInstances: {'instance.a', 'instance.b'},
            additionalSurfaces: additions,
          ),
          throwsA(anyOf(isA<ArgumentError>(), isA<StateError>())),
        );
        expect(before.encode(), original);
        expect(target.encode(), originalTarget);
      }
    },
  );
  test(
    'reframing preserves retained bindings, nested splits and focus even when target pane names collide',
    () {
      final controller = WorkbenchLayoutController(
        createDefaultWorkbenchLayout(),
      );
      controller.splitPane('editor', axis: Axis.vertical, newPaneId: 'second');
      controller.bind('second', binding('secondary'));
      controller.swap('editor', 'workspace');
      controller.focus('second');
      final before = controller.snapshot;
      final retained = {
        before.bindingFor('workspace')!.instanceRef,
        before.bindingFor('second')!.instanceRef,
      };
      final after = reframeWorkbenchSurfaces(
        current: before,
        target: createDefaultWorkbenchLayout(),
        targetPaneId: 'editor',
        retainedInstances: retained,
      );
      expect(
        after.bindingFor('workspace'),
        same(before.bindingFor('workspace')),
      );
      expect(after.bindingFor('second'), same(before.bindingFor('second')));
      expect(after.focusedPaneId, 'second');
      expect(
        after.bindings.values.where(
          (b) => b.surfaceRef == 'host.workspaceExplorer',
        ),
        hasLength(1),
      );
      expect(after.paneIds, hasLength(4));
      expect(WorkbenchLayoutSnapshot.tryDecode(after.encode()), after);
      controller.dispose();
    },
  );
  test(
    'missing retained surfaces and missing target slots reject without changing either input',
    () {
      final before = pair(), target = createDefaultWorkbenchLayout();
      final encoded = before.encode();
      expect(
        () => reframeWorkbenchSurfaces(
          current: before,
          target: target,
          targetPaneId: 'editor',
          retainedInstances: {'missing'},
        ),
        throwsArgumentError,
      );
      expect(
        () => reframeWorkbenchSurfaces(
          current: before,
          target: target,
          targetPaneId: 'missing',
          retainedInstances: {'instance.a'},
        ),
        throwsArgumentError,
      );
      expect(before.encode(), encoded);
    },
  );
  test(
    'oversized reframe respects the existing model depth budget without mutating its source',
    () {
      LayoutNode region = PaneNode(paneId: 'a');
      final bindings = {'a': binding('a')};
      for (var i = 0; i < 7; i++) {
        region = SplitNode(
          axis: Axis.vertical,
          ratio: .5,
          first: region,
          second: PaneNode(paneId: 'p$i'),
        );
        bindings['p$i'] = binding('p$i');
      }
      final before = WorkbenchLayoutSnapshot(root: region, bindings: bindings);
      expect(
        () => reframeWorkbenchSurfaces(
          current: before,
          target: createDefaultWorkbenchLayout(),
          targetPaneId: 'editor',
          retainedInstances: bindings.values.map((b) => b.instanceRef).toSet(),
        ),
        throwsStateError,
      );
      expect(before.paneIds, hasLength(8));
    },
  );
  testWidgets(
    'hidden retained surfaces keep State, selection and viewport while losing focus; all-hidden frames can resume',
    (t) async {
      final controller = WorkbenchLayoutController(pair());
      var hidden = <String>{};
      var opened = 0, disposed = 0;
      Future<void> render() async {
        await t.pumpWidget(
          MaterialApp(
            home: SizedBox(
              width: 600,
              height: 400,
              child: WorkbenchCanvas(
                controller: controller,
                hiddenPaneIds: hidden,
                retainedInstanceRefs: const {'instance.a', 'instance.b'},
                paneBuilder: (_, id, _) => Probe(
                  key: ValueKey('probe:$id'),
                  opened: () => opened++,
                  disposed: () => disposed++,
                ),
              ),
            ),
          ),
        );
        await t.pump();
      }

      await render();
      final finder = find.byKey(const ValueKey('probe:a'), skipOffstage: false);
      final state = t.state<ProbeState>(finder);
      state.text.selection = const TextSelection(
        baseOffset: 2,
        extentOffset: 6,
      );
      state.scroll.jumpTo(320);
      state.focus.requestFocus();
      await t.pump();
      hidden = {'a'};
      await render();
      expect(t.state(finder), same(state));
      expect(find.byKey(const ValueKey('probe:a')), findsNothing);
      expect(state.focus.hasFocus, isFalse);
      expect(
        state.text.selection,
        const TextSelection(baseOffset: 2, extentOffset: 6),
      );
      expect(state.scroll.offset, 320);
      hidden = {'a', 'b'};
      await render();
      expect(opened, 2);
      expect(disposed, 0);
      hidden = {};
      await render();
      expect(t.state(finder), same(state));
      expect(state.scroll.offset, 320);
      expect(
        state.text.selection,
        const TextSelection(baseOffset: 2, extentOffset: 6),
      );
      await t.pumpWidget(const SizedBox.shrink());
      expect(disposed, 2);
      controller.dispose();
    },
  );
  testWidgets(
    'moving retained surfaces to another surrounding layout preserves the same live Widget states',
    (t) async {
      final controller = WorkbenchLayoutController(pair());
      var opened = 0, disposed = 0;
      await t.pumpWidget(
        MaterialApp(
          home: WorkbenchCanvas(
            controller: controller,
            paneBuilder: (_, id, b) => Probe(
              key: ValueKey('probe.${b!.instanceRef}'),
              opened: () => opened++,
              disposed: () => disposed++,
            ),
          ),
        ),
      );
      final a = t.state<ProbeState>(
        find.byKey(const ValueKey('probe.instance.a')),
      );
      a.text.text = '布局调整前的本机输入😀';
      controller.restore(
        reframeWorkbenchSurfaces(
          current: controller.snapshot,
          target: createDefaultWorkbenchLayout(),
          targetPaneId: 'editor',
          retainedInstances: {'instance.a', 'instance.b'},
        ),
      );
      await t.pump();
      expect(t.state(find.byKey(const ValueKey('probe.instance.a'))), same(a));
      expect(a.text.text, '布局调整前的本机输入😀');
      expect(opened, 4);
      expect(disposed, 0);
      await t.pumpWidget(const SizedBox.shrink());
      controller.dispose();
    },
  );
}

class Probe extends StatefulWidget {
  const Probe({super.key, required this.opened, required this.disposed});
  final VoidCallback opened, disposed;
  @override
  State<Probe> createState() => ProbeState();
}

class ProbeState extends State<Probe> {
  final text = TextEditingController(text: '保留独立选区与输入状态');
  final focus = FocusNode();
  final scroll = ScrollController();
  @override
  void initState() {
    super.initState();
    widget.opened();
  }

  @override
  Widget build(BuildContext context) => Material(
    child: Column(
      children: [
        TextField(controller: text, focusNode: focus),
        Expanded(
          child: ListView(
            controller: scroll,
            children: [
              for (var i = 0; i < 30; i++)
                SizedBox(height: 50, child: Text('行 $i')),
            ],
          ),
        ),
      ],
    ),
  );
  @override
  void dispose() {
    widget.disposed();
    text.dispose();
    focus.dispose();
    scroll.dispose();
    super.dispose();
  }
}
