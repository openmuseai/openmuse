import 'package:flutter/widgets.dart';

import 'model.dart';

/// The host owns configuration; descendant surfaces resolve only what they need.
final class RuntimeConfigScope extends InheritedWidget {
  const RuntimeConfigScope({
    super.key,
    required this.config,
    required super.child,
  });

  final RuntimeConfig config;

  static RuntimeConfig? maybeOf(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<RuntimeConfigScope>()?.config;

  @override
  bool updateShouldNotify(RuntimeConfigScope oldWidget) =>
      config != oldWidget.config;
}
