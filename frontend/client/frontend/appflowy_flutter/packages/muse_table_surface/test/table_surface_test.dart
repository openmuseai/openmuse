import 'package:flutter_test/flutter_test.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_table_surface/muse_table_surface.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';

void main() {
  test('active cell/range/filter contexts remain plugin-owned and close cleanly', () async {
    final sink = _Sink(); final runtime = MuseUiSurfaceRuntime(sink: sink, stateDebounce: Duration.zero);
    registerMuseTableSurface(runtime);
    final revisions = <String>[];
    final facet = MuseTableSurfaceFacet(onExternalReconcile: (value) async => revisions.add(value));
    final binding = MuseTableSurfaceBinding.open(runtime: runtime, databaseId: 'database.1', windowRef: 'window.1', facet: facet);
    await binding.publishActiveCell(rowRef: 'row.1', columnRef: 'done', value: false);
    await binding.publishVisibleRange(rowRefs: ['row.1'], columnRefs: ['name', 'done']);
    await binding.publishFilter(filterRef: 'filter.1', summary: 'done is false');
    await Future<void>.delayed(const Duration(milliseconds: 5));
    expect(sink.items.map((value) => value.contextType), containsAll(['table.active-cell', 'table.visible-range', 'table.filter']));
    await runtime.routeDomainChange(MuseDomainChangeV1(pluginId: museDatabasePluginId, providerInstanceRef: 'provider.1', scopeRef: 'scope.1', resourceRef: 'database.1', eventType: 'table.changed', eventSchemaDigest: 'sha256:${List.filled(64, '4').join()}', domainRevision: '2', epochRef: 'epoch.1', origin: MuseMutationOrigin.externalCommand, occurredAt: DateTime.now().millisecondsSinceEpoch, payload: const {'rowRef': 'row.1'}));
    expect(revisions, ['2']);
    await binding.close(); expect(runtime.surfaceCount, 0); expect(sink.closed, ['database.1']); await runtime.dispose();
  });
}

final class _Sink implements MuseSurfaceContextSink {
  final items = <MuseContextContributionV1>[]; final closed = <String>[];
  @override Future<void> publish(MuseContextContributionV1 context) async => items.add(context);
  @override Future<void> closeSurface(String surfaceInstanceRef, String scopeRef) async => closed.add(scopeRef);
}
