import 'package:flutter_test/flutter_test.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';
import 'package:muse_word_surface/muse_word_surface.dart';

void main() {
  test('P2-T4/T5/T6 word.surface and word.selection then close', () async {
    final sink = _Sink();
    final runtime = MuseUiSurfaceRuntime(
      sink: sink,
      stateDebounce: Duration.zero,
    );
    registerMuseWordSurface(runtime);
    final binding = MuseWordSurfaceBinding.open(
      viewId: 'view.word.1',
      title: 'Spec',
      runtime: runtime,
    );
    await binding.publishSurface(
      title: 'Spec',
      mode: 'edit',
      ffiReady: true,
    );
    await binding.publishSelection(caretCp: 3, pageIndex: 1);
    await Future<void>.delayed(const Duration(milliseconds: 5));
    expect(
      sink.items.map((value) => value.contextType),
      containsAll(['word.surface', 'word.selection']),
    );
    expect(sink.items.first.payload, containsPair('viewId', 'view.word.1'));
    expect(wordSurfaceDigest, isNot(contains('780a1eed')));
    await binding.close();
    expect(runtime.surfaceCount, 0);
    expect(sink.closed, ['view.word.1']);
    await runtime.dispose();
  });
}

final class _Sink implements MuseSurfaceContextSink {
  final items = <MuseContextContributionV1>[];
  final closed = <String>[];
  @override
  Future<void> publish(MuseContextContributionV1 context) async =>
      items.add(context);
  @override
  Future<void> closeSurface(String surfaceInstanceRef, String scopeRef) async =>
      closed.add(scopeRef);
}
