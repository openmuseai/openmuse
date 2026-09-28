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
                if (widget.supervisor.state != DshSidecarState.ready)
                  _Composer(onStart: _start),
              ],
            ),
            Positioned(
              top: 8,
              right: 6,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  IconButton(
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
                      minWidth: 26,
                      minHeight: 26,
                    ),
                  ),
                  ?paneMenu,
                ],
              ),
            ),
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
        url: supervisor.endpoint!,
        activeMountPath: activeMountPath,
        onActivateWorkspace: onActivateWorkspace,
        onOpenResource: onOpenResource,
        reloadToken: reloadToken,
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
                    '仅在首次使用时启动 DSH sidecar；它不会阻塞本地 Workspace。',
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
                child: const Text('启动插件'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _Composer extends StatelessWidget {
  const _Composer({required this.onStart});
  final VoidCallback onStart;

  @override
  Widget build(BuildContext context) => Container(
    margin: const EdgeInsets.fromLTRB(12, 0, 12, 14),
    padding: const EdgeInsets.fromLTRB(12, 11, 10, 9),
    height: 82,
    decoration: BoxDecoration(
      color: Theme.of(context).colorScheme.surface,
      borderRadius: BorderRadius.circular(14),
      border: Border.all(color: Theme.of(context).dividerColor),
    ),
    child: Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          '给智能体发消息',
          style: TextStyle(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
            fontSize: 12,
          ),
        ),
        const Spacer(),
        Row(
          children: [
            const Icon(Icons.add, color: Color(0xff777b84), size: 18),
            const Spacer(),
            IconButton(
              onPressed: onStart,
              padding: EdgeInsets.zero,
              constraints: const BoxConstraints.tightFor(width: 28, height: 28),
              icon: const Icon(Icons.arrow_upward, size: 16),
              color: const Color(0xffdfe5ff),
              style: IconButton.styleFrom(
                backgroundColor: const Color(0xff496bb8),
              ),
            ),
          ],
        ),
      ],
    ),
  );
}

String? _friendlyError(Object? error) {
  if (error == null) return null;
  return error.toString().replaceFirst(
    RegExp(r'^(Bad state|StateError):\s*'),
    '',
  );
}
