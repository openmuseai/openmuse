import 'package:flutter/widgets.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'layout.dart';

/// Reveal a singleton contribution without replacing document/editor bindings.
void revealPluginPanel(
  WorkbenchLayoutController layout,
  OpenMusePanelCandidate candidate,
) {
  final ref = 'plugin.panel:${candidate.panel.id}';
  final existing = layout.snapshot.bindings.entries
      .where((e) => e.value.surfaceRef == ref)
      .firstOrNull;
  if (existing != null) {
    layout.focus(existing.key);
    return;
  }
  final target =
      layout.snapshot.bindings.entries
          .where((e) => e.value.surfaceRef.startsWith('host.editorGroup:'))
          .firstOrNull
          ?.key ??
      layout.snapshot.focusedPaneId;
  if (target == null) return;
  final pane = layout.splitPane(
    target,
    axis: Axis.horizontal,
    ratio: 0.4,
    newPaneFirst: true,
  );
  final id = '${candidate.plugin.descriptor.id}.${candidate.panel.id}';
  layout.bind(
    pane,
    SurfaceBinding(
      bindingId: 'binding.panel.$id',
      surfaceRef: ref,
      instanceRef: 'surface.panel.$id',
      mobility: SurfaceMobility.snapshotRestore,
    ),
  );
}
