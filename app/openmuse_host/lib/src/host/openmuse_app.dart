import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';

import 'design_system.dart';
import 'layout/layout.dart';
import 'layout/surface_mutation_guard.dart';
import 'local_settings.dart';
import 'workbench_shell.dart';
import 'workspace_controller.dart';

final class OpenMuseHostApp extends StatelessWidget {
  const OpenMuseHostApp({
    super.key,
    required this.registry,
    required this.workspace,
    this.settings,
    this.layoutController,
    this.layoutStore,
    this.mutationGuards,
    this.authentication,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings? settings;
  final WorkbenchLayoutController? layoutController;
  final LayoutSnapshotWriter? layoutStore;
  final SurfaceMutationGuards? mutationGuards;
  final OpenMuseAuthenticationContributor? authentication;

  @override
  Widget build(BuildContext context) {
    final preferences = settings ?? OpenMuseLocalSettings();
    return ListenableBuilder(
      listenable: preferences,
      builder: (context, _) => MaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'OpenMuse',
        theme: buildOpenMuseTheme(),
        darkTheme: buildOpenMuseTheme(brightness: Brightness.dark),
        themeMode: preferences.themeMode,
        navigatorObservers: [DshPopupRouteObserver.instance],
        home: Builder(
          builder: (context) {
            final workbench = OpenMuseWorkbench(
              registry: registry,
              workspace: workspace,
              settings: preferences,
              layoutController: layoutController,
              layoutStore: layoutStore,
              mutationGuards: mutationGuards,
            );
            final auth = authentication;
            if (auth == null) return workbench;
            return auth.buildAuthenticationGate(
              context,
              authenticatedChild: workbench,
            );
          },
        ),
      ),
    );
  }
}
