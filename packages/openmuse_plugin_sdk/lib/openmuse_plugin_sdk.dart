library;

import 'dart:async';

import 'package:flutter/material.dart';

enum OpenMusePluginRuntime { builtIn, nativeProcess, webView }

enum OpenMusePluginState { installed, activating, active, deactivating, failed }

enum OpenMuseSurfaceRegion { editor, rightSidebar }

@immutable
final class OpenMuseResource {
  const OpenMuseResource({
    required this.uri,
    required this.displayName,
    this.mediaType,
  });

  final Uri uri;
  final String displayName;
  final String? mediaType;

  String get extension {
    final segment = uri.pathSegments.isEmpty ? '' : uri.pathSegments.last;
    final separator = segment.lastIndexOf('.');
    return separator < 0 ? '' : segment.substring(separator + 1).toLowerCase();
  }
}

@immutable
final class OpenMuseEditorCandidate {
  const OpenMuseEditorCandidate({required this.plugin, required this.editor});

  final OpenMusePlugin plugin;
  final OpenMuseEditorContribution editor;
}

@immutable
final class OpenMusePanelCandidate {
  const OpenMusePanelCandidate({required this.plugin, required this.panel});

  final OpenMusePlugin plugin;
  final OpenMusePanelContribution panel;
}

@immutable
final class OpenMuseEditorContribution {
  const OpenMuseEditorContribution({
    required this.id,
    required this.extensions,
    required this.priority,
    this.mediaTypes = const {},
    this.catchAll = false,
  });

  final String id;
  final Set<String> extensions;

  /// Content types take precedence when the Host has inspected local bytes.
  /// A trailing `/*` accepts a media type family such as `text/*`.
  final Set<String> mediaTypes;
  final int priority;
  final bool catchAll;

  bool accepts(OpenMuseResource resource) {
    if (catchAll) return true;
    final type = resource.mediaType;
    if (type != null && mediaTypes.isNotEmpty) {
      return mediaTypes.any(
        (accepted) =>
            accepted == type ||
            (accepted.endsWith('/*') &&
                type.startsWith(accepted.substring(0, accepted.length - 1))),
      );
    }
    return extensions.contains(resource.extension);
  }
}

@immutable
final class OpenMusePanelContribution {
  const OpenMusePanelContribution({required this.id, required this.region});

  final String id;
  final OpenMuseSurfaceRegion region;
}

@immutable
final class OpenMusePluginDescriptor {
  const OpenMusePluginDescriptor({
    required this.id,
    required this.name,
    required this.version,
    required this.runtime,
    this.activationEvents = const [],
    this.permissions = const {},
    this.editors = const [],
    this.panels = const [],
  });

  final String id;
  final String name;
  final String version;
  final OpenMusePluginRuntime runtime;
  final List<String> activationEvents;
  final Set<String> permissions;
  final List<OpenMuseEditorContribution> editors;
  final List<OpenMusePanelContribution> panels;
}

@immutable
final class OpenMusePluginContext {
  const OpenMusePluginContext({
    required this.executeHostCommand,
    this.hostChanges,
  });

  final Future<Object?> Function(String command, Object? arguments)
  executeHostCommand;

  /// Invalidates read-only Host snapshots; plugins re-query through commands.
  final Listenable? hostChanges;
}

/// Host chrome that a panel can place on its own toolbar instead of a
/// dedicated title row. `trailing` is typically the pane overflow menu.
final class OpenMuseSurfaceChrome extends InheritedWidget {
  const OpenMuseSurfaceChrome({super.key, this.trailing, required super.child});

  final Widget? trailing;

  static Widget? trailingOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<OpenMuseSurfaceChrome>()
      ?.trailing;

  @override
  bool updateShouldNotify(OpenMuseSurfaceChrome oldWidget) =>
      trailing != oldWidget.trailing;
}

abstract interface class OpenMusePlugin {
  OpenMusePluginDescriptor get descriptor;

  Future<void> activate(OpenMusePluginContext context);

  Future<void> deactivate();

  Widget buildEditor(BuildContext context, OpenMuseResource resource);

  Widget? buildPanel(BuildContext context, String panelId);
}

/// Optional plugin-owned editor chrome. The Host only places the widget above
/// the surface and routes an explicit editor switch within the active tab.
abstract interface class OpenMuseEditorBannerContributor {
  Widget? buildEditorBanner(
    BuildContext context,
    OpenMuseResource resource, {
    required bool Function(String editorId) canOpenWith,
    required void Function(String editorId) openWith,
  });
}

final class OpenMuseEditorBanner extends StatelessWidget {
  const OpenMuseEditorBanner({
    super.key,
    required this.message,
    this.actionLabel,
    this.onAction,
  });

  final String message;
  final String? actionLabel;
  final VoidCallback? onAction;

  @override
  Widget build(BuildContext context) => Container(
    height: 28,
    padding: const EdgeInsets.symmetric(horizontal: 10),
    color: const Color(0xff202329),
    child: Row(
      children: [
        Expanded(
          child: Text(
            message,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(color: Color(0xffc9cbd1), fontSize: 11),
          ),
        ),
        if (actionLabel != null && onAction != null)
          TextButton(
            onPressed: onAction,
            style: TextButton.styleFrom(
              foregroundColor: const Color(0xffaabaff),
              padding: const EdgeInsets.symmetric(horizontal: 10),
              minimumSize: const Size(0, 26),
              tapTargetSize: MaterialTapTargetSize.shrinkWrap,
            ),
            child: Text(actionLabel!, style: const TextStyle(fontSize: 11)),
          ),
      ],
    ),
  );
}

/// Optional editor capability: settle an in-memory buffer before the Host
/// reads that resource from disk for snapshots or comparisons.
abstract interface class OpenMuseBufferFlushContributor {
  Future<void> flushResource(OpenMuseResource resource);
}

/// Optional plugin-owned settings surface. Host provides the containing page
/// and namespaced persistence; it does not know editor or agent preferences.
abstract interface class OpenMuseSettingsContributor {
  Widget buildSettings(BuildContext context);
}

/// Optional runtime diagnostics shown from the Host's plugin list.
abstract interface class OpenMusePluginLogContributor {
  Future<String> readLog();
}

final class OpenMusePluginRegistry extends ChangeNotifier {
  OpenMusePluginRegistry({required OpenMusePluginContext context})
    : _context = context;

  final OpenMusePluginContext _context;
  final Map<String, OpenMusePlugin> _plugins = {};
  final Map<String, OpenMusePluginState> _states = {};
  final Map<String, Future<void>> _transitions = {};
  final Map<String, List<String>> _logs = {};

  Iterable<OpenMusePluginDescriptor> get descriptors =>
      _plugins.values.map((plugin) => plugin.descriptor);

  OpenMusePluginState? stateOf(String pluginId) => _states[pluginId];

  OpenMusePlugin? plugin(String pluginId) => _plugins[pluginId];

  String logOf(String pluginId) => (_logs[pluginId] ?? const []).join('\n');

  void _log(String pluginId, String message) {
    final lines = _logs.putIfAbsent(pluginId, () => []);
    lines.add('${DateTime.now().toIso8601String()} $message');
    if (lines.length > 100) lines.removeAt(0);
    notifyListeners();
  }

  void install(OpenMusePlugin plugin) {
    final id = plugin.descriptor.id;
    if (_plugins.containsKey(id)) {
      throw StateError('Plugin already installed: $id');
    }
    _plugins[id] = plugin;
    _states[id] = OpenMusePluginState.installed;
    _log(id, '已安装 ${plugin.descriptor.version}');
    notifyListeners();
  }

  Future<void> uninstall(String pluginId) async {
    final plugin = _plugins[pluginId];
    if (plugin == null) return;
    await deactivate(pluginId);
    _plugins.remove(pluginId);
    _states.remove(pluginId);
    _logs.remove(pluginId);
    notifyListeners();
  }

  Future<void> activate(String pluginId) {
    return _serialize(pluginId, () async {
      final plugin = _requirePlugin(pluginId);
      if (_states[pluginId] == OpenMusePluginState.active) return;
      _states[pluginId] = OpenMusePluginState.activating;
      _log(pluginId, '正在启动');
      notifyListeners();
      try {
        await plugin.activate(_context);
        _states[pluginId] = OpenMusePluginState.active;
        _log(pluginId, '启动成功');
      } catch (error, stack) {
        _states[pluginId] = OpenMusePluginState.failed;
        _log(pluginId, '启动失败: $error\n$stack');
        rethrow;
      } finally {
        notifyListeners();
      }
    });
  }

  Future<void> deactivate(String pluginId) {
    return _serialize(pluginId, () async {
      final plugin = _plugins[pluginId];
      if (plugin == null ||
          _states[pluginId] == OpenMusePluginState.installed) {
        return;
      }
      _states[pluginId] = OpenMusePluginState.deactivating;
      _log(pluginId, '正在停止');
      notifyListeners();
      try {
        await plugin.deactivate();
        _states[pluginId] = OpenMusePluginState.installed;
        _log(pluginId, '已停止');
      } catch (error, stack) {
        _states[pluginId] = OpenMusePluginState.failed;
        _log(pluginId, '停止失败: $error\n$stack');
        rethrow;
      } finally {
        notifyListeners();
      }
    });
  }

  List<OpenMuseEditorCandidate> editorCandidates(OpenMuseResource resource) {
    final result = <OpenMuseEditorCandidate>[
      for (final plugin in _plugins.values)
        for (final editor in plugin.descriptor.editors)
          if (editor.accepts(resource))
            OpenMuseEditorCandidate(plugin: plugin, editor: editor),
    ]..sort((a, b) => b.editor.priority.compareTo(a.editor.priority));
    return result;
  }

  OpenMusePlugin? editorFor(OpenMuseResource resource, {String? editorId}) {
    final candidates = editorCandidates(resource);
    if (editorId == null) return candidates.firstOrNull?.plugin;
    return candidates
            .where((candidate) => candidate.editor.id == editorId)
            .firstOrNull
            ?.plugin ??
        candidates.firstOrNull?.plugin;
  }

  OpenMusePlugin? panelProvider(
    OpenMuseSurfaceRegion region, {
    String? panelId,
  }) {
    return _plugins.values.cast<OpenMusePlugin?>().firstWhere(
      (plugin) => plugin!.descriptor.panels.any(
        (panel) =>
            panel.region == region && (panelId == null || panel.id == panelId),
      ),
      orElse: () => null,
    );
  }

  List<OpenMusePanelCandidate> panelCandidates({
    OpenMuseSurfaceRegion? defaultRegion,
  }) => [
    for (final plugin in _plugins.values)
      for (final panel in plugin.descriptor.panels)
        if (defaultRegion == null || panel.region == defaultRegion)
          OpenMusePanelCandidate(plugin: plugin, panel: panel),
  ];

  OpenMusePlugin? panelProviderById(String panelId) {
    for (final plugin in _plugins.values) {
      if (plugin.descriptor.panels.any((panel) => panel.id == panelId)) {
        return plugin;
      }
    }
    return null;
  }

  Future<void> ensureActive(OpenMusePlugin plugin) =>
      activate(plugin.descriptor.id);

  OpenMusePlugin _requirePlugin(String pluginId) {
    final plugin = _plugins[pluginId];
    if (plugin == null) throw StateError('Plugin not installed: $pluginId');
    return plugin;
  }

  Future<void> _serialize(String pluginId, Future<void> Function() operation) {
    final previous = _transitions[pluginId] ?? Future<void>.value();
    final next = previous.then((_) => operation());
    late final Future<void> tracked;
    tracked = next.whenComplete(() {
      if (identical(_transitions[pluginId], tracked)) {
        _transitions.remove(pluginId);
      }
    });
    _transitions[pluginId] = tracked;
    return tracked;
  }
}
