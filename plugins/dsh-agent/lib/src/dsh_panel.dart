import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'dsh_sidecar.dart';
import 'dsh_web_view.dart';

final class DshPanel extends StatefulWidget {
  const DshPanel({
    super.key,
    required this.supervisor,
    required this.activeMount,
    required this.onActivateWorkspace,
    required this.onOpenResource,
  });
  final DshSidecarSupervisor supervisor;
  final ValueListenable<String?> activeMount;
  final Future<void> Function(String path) onActivateWorkspace;
  final Future<void> Function(DshResourceOpenMessage request) onOpenResource;

  @override
  State<DshPanel> createState() => _DshPanelState();
}

final class _DshPanelState extends State<DshPanel> {
  int _reloadToken = 0;

  @override
  void initState() {
    super.initState();
    Future<void>.microtask(_start);
  }

  @override
  Widget build(BuildContext context) {
    final paneMenu = OpenMuseSurfaceChrome.trailingOf(context);
    return ListenableBuilder(
      listenable: Listenable.merge([widget.supervisor, widget.activeMount]),
      builder: (context, _) => ColoredBox(
        color: Theme.of(context).brightness == Brightness.dark
            ? const Color(0xff202228)
            : const Color(0xfffbfbfc),
        child: Stack(
          children: [
            Column(
              children: [
                Expanded(
                  child: _PanelBody(
                    supervisor: widget.supervisor,
                    activeMountPath: widget.activeMount.value,
                    onActivateWorkspace: widget.onActivateWorkspace,
                    onOpenResource: widget.onOpenResource,
                    reloadToken: _reloadToken,
                  ),
                ),
              ],
            ),
            if (!Platform.isWindows)
              Positioned(
                top: 8,
                left: 210,
                child: IconButton(
                  key: const Key('dsh-reload'),
                  tooltip: '刷新 DSH 面板',
                  onPressed: () => setState(() => _reloadToken++),
                  icon: Icon(
                    Icons.refresh,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                    size: 16,
                  ),
                  visualDensity: VisualDensity.compact,
                  padding: EdgeInsets.zero,
                  constraints: const BoxConstraints(
                    minWidth: 36,
                    minHeight: 36,
                  ),
                ),
              ),
            if (paneMenu != null)
              Positioned(top: 8, right: 6, child: paneMenu),
          ],
        ),
      ),
    );
  }

  Future<void> _start() async {
    try {
      await widget.supervisor.ensureStarted();
    } catch (_) {
      // State and redacted diagnostics remain inside the plugin panel.
    }
  }
}

final class _PanelBody extends StatelessWidget {
  const _PanelBody({
    required this.supervisor,
    required this.activeMountPath,
    required this.onActivateWorkspace,
    required this.onOpenResource,
    required this.reloadToken,
  });
  final DshSidecarSupervisor supervisor;
  final String? activeMountPath;
  final Future<void> Function(String path) onActivateWorkspace;
  final Future<void> Function(DshResourceOpenMessage request) onOpenResource;
  final int reloadToken;

  @override
  Widget build(BuildContext context) {
    if (supervisor.state == DshSidecarState.ready) {
      return DshWebView(
        key: ValueKey<String>(supervisor.endpoint.toString()),
        url: supervisor.endpoint!,
        activeMountPath: activeMountPath,
        onActivateWorkspace: onActivateWorkspace,
        onOpenResource: onOpenResource,
        reloadToken: reloadToken,
      );
    }
    if (supervisor.state == DshSidecarState.starting ||
        supervisor.state == DshSidecarState.stopped) {
      return const Center(
        child: SizedBox(
          width: 22,
          height: 22,
          child: CircularProgressIndicator(strokeWidth: 2),
        ),
      );
    }
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 18, 16, 12),
      child: Align(
        alignment: Alignment.topLeft,
        child: Container(
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surface,
            borderRadius: BorderRadius.circular(10),
            border: Border.all(color: Theme.of(context).dividerColor),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Agent 可选组件',
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 7),
              Text(
                _friendlyError(supervisor.lastError) ??
                    'DSH sidecar 未能自动启动。',
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                  fontSize: 12,
                  height: 1.45,
                ),
              ),
              const SizedBox(height: 12),
              TextButton(
                onPressed: () =>
                    context.findAncestorStateOfType<_DshPanelState>()?._start(),
                style: TextButton.styleFrom(
                  foregroundColor: const Color(0xff2f6de1),
                  padding: EdgeInsets.zero,
                ),
                child: const Text('重试启动'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

String? _friendlyError(Object? error) {
  if (error == null) return null;
  return error.toString().replaceFirst(
    RegExp(r'^(Bad state|StateError):\s*'),
    '',
  );
}
