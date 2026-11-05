import 'dart:async';

import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:flutter/material.dart';

abstract interface class MuseResourceTabTarget {
  MuseResolvedResource get resource;
  MuseLocalEngine get selectedEngine;
  void selectEngine(MuseLocalEngine engine);
}

typedef MuseResourceActionPredicate = bool Function(
  MuseResourceTabTarget target,
);
typedef MuseResourceActionHandler = FutureOr<void> Function(
  BuildContext context,
  MuseResourceTabTarget target,
);
typedef MuseResourceSubmenuBuilder = Widget Function(
  BuildContext context,
  MuseResourceTabTarget target,
  VoidCallback closeMenu,
);

/// Public Host extension point for resource-tab menu contributions.
///
/// Engine adapters and later third-party plugins register stable action IDs.
/// Re-registering an ID replaces the old contribution, which makes plugin
/// reload deterministic and avoids duplicate menu rows.
final class MuseResourceTabActionRegistry extends ChangeNotifier {
  final Map<String, MuseResourceTabAction> _actions = {};

  void register(MuseResourceTabAction action) {
    _actions[action.id] = action;
    notifyListeners();
  }

  void unregister(String id) {
    if (_actions.remove(id) != null) notifyListeners();
  }

  List<PluginTabMenuAction> actionsFor(MuseResourceTabTarget target) {
    final actions = _actions.values.toList()
      ..sort((a, b) {
        final group = a.group.compareTo(b.group);
        return group != 0 ? group : a.order.compareTo(b.order);
      });
    return actions
        .map((action) => _BoundResourceTabAction(action, target))
        .toList(growable: false);
  }
}

final class MuseResourceTabAction {
  const MuseResourceTabAction({
    required this.id,
    required this.label,
    required this.handler,
    this.icon,
    this.group = 0,
    this.order = 0,
    this.enabled,
    this.submenuBuilder,
  });

  final String id;
  final String label;
  final IconData? icon;
  final int group;
  final int order;
  final MuseResourceActionPredicate? enabled;
  final MuseResourceActionHandler handler;
  final MuseResourceSubmenuBuilder? submenuBuilder;
}

final class _BoundResourceTabAction implements PluginTabMenuAction {
  const _BoundResourceTabAction(this.action, this.target);

  final MuseResourceTabAction action;
  final MuseResourceTabTarget target;

  @override
  String get id => action.id;

  @override
  String get label => action.label;

  @override
  IconData? get icon => action.icon;

  @override
  int get group => action.group;

  @override
  bool get enabled => action.enabled?.call(target) ?? true;

  @override
  PluginTabMenuSubmenuBuilder? get submenuBuilder {
    final builder = action.submenuBuilder;
    if (builder == null) return null;
    return (context, closeMenu) => builder(context, target, closeMenu);
  }

  @override
  FutureOr<void> invoke(BuildContext context) =>
      action.handler(context, target);
}
