import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_host/src/host/layout/layout.dart';
import 'package:openmuse_host/src/host/layout/plugin_panel.dart';

class TestPanel implements OpenMusePlugin {
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'test.app',
    name: 'Test App',
    version: '1',
    runtime: OpenMusePluginRuntime.webView,
    panels: [
      OpenMusePanelContribution(
        id: 'test.main',
        region: OpenMuseSurfaceRegion.rightSidebar,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {}
  @override
  Future<void> deactivate() async {}
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox();
  @override
  Widget? buildPanel(BuildContext context, String panelId) => const SizedBox();
}

void main() {
  test(
    'App opens to the right of Agent, preserves bindings and is a singleton',
    () {
      final layout = WorkbenchLayoutController(createDefaultWorkbenchLayout());
      final anchor = layout.snapshot.paneIds.first;
      final original = layout.snapshot.bindings[anchor];
      final app = TestPanel();
      final candidate = OpenMusePanelCandidate(
        plugin: app,
        panel: app.descriptor.panels.single,
      );
      final before = layout.snapshot.paneIds.length;
      revealPluginPanel(layout, candidate, right: true, besidePaneId: anchor);
      expect(layout.snapshot.paneIds.length, before + 1);
      expect(layout.snapshot.bindings[anchor], original);
      expect(
        layout.snapshot.bindings[layout.snapshot.focusedPaneId]!.surfaceRef,
        'plugin.panel:test.main',
      );
      final encoded = layout.snapshot.encode();
      // Layout serialization order is visual left-to-right for a horizontal split.
      expect(
        encoded.indexOf('"paneId":"$anchor"'),
        lessThan(
          encoded.indexOf('"paneId":"${layout.snapshot.focusedPaneId}"'),
        ),
      );
      revealPluginPanel(layout, candidate, right: true, besidePaneId: anchor);
      expect(layout.snapshot.paneIds.length, before + 1);
    },
  );
}
