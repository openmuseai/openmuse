import 'package:flutter/material.dart';

/// Optional Desktop capability. Business navigation and data stay with the
/// contributing plugin. Selecting a container does not grant any permission.
abstract interface class OpenMuseNavigationContributor {
  List<OpenMuseNavigationItem> get navigationItems;
}

@immutable
final class OpenMuseNavigationItem {
  const OpenMuseNavigationItem({
    required this.id,
    required this.title,
    required this.icon,
    required this.sidebar,
    this.onOpenWorkspace,
    this.order = 100,
  });
  final int order;
  final String id;
  final String title;

  /// A presentation glyph, never a plugin or business routing identifier.
  final String icon;
  final WidgetBuilder sidebar;
  final VoidCallback? onOpenWorkspace;
}

/// Account-scoped shared folders, for example, belong inside Explorer rather
/// than creating a second primary navigation container.
abstract interface class OpenMuseExplorerContributor {
  Widget buildExplorerSection(BuildContext context);
}

/// A plugin-owned, retained page inside the workspace, beside the fixed Agent.
@immutable
final class OpenMuseWorkspacePage {
  const OpenMuseWorkspacePage({
    required this.id,
    required this.title,
    required this.builder,
  });
  final String id, title;
  final WidgetBuilder builder;
}

/// The Host binds the owner while building a contribution. A presentation
/// request never grants data access or changes the application's Navigator.
final class OpenMuseWorkspaceRouting extends InheritedWidget {
  const OpenMuseWorkspaceRouting({
    super.key,
    required this.open,
    required super.child,
  });
  final void Function(OpenMuseWorkspacePage page) open;
  static OpenMuseWorkspaceRouting? maybeOf(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<OpenMuseWorkspaceRouting>();
  @override
  bool updateShouldNotify(OpenMuseWorkspaceRouting oldWidget) =>
      open != oldWidget.open;
}
