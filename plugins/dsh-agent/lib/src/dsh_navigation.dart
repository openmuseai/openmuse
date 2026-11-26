import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'dsh_sidecar.dart';
import 'dsh_web_view.dart';

/// The original DSH SidebarRoot and workspace plugin own every list/action.
/// This adapter owns consent, lifecycle and the Host's in-flow slot only.
final class DshNavigation extends StatefulWidget {
  const DshNavigation({
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
  State<DshNavigation> createState() => _DshNavigationState();
}

final class _DshNavigationState extends State<DshNavigation> {
  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() async {
    if (!widget.supervisor.aiSharingAllowed) return;
    try {
      await widget.supervisor.ensureStarted();
    } catch (_) {
      /* Show real state and retry below. */
    }
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: Listenable.merge([widget.supervisor, widget.activeMount]),
    builder: (context, _) {
      final zh = Localizations.localeOf(context).languageCode == 'zh';
      if (!widget.supervisor.aiSharingAllowed) {
        return Padding(
          padding: const EdgeInsets.all(16),
          child: Text(
            zh
                ? '请在 Agent 主区确认 AI 数据共享。'
                : 'Confirm AI data sharing in the Agent pane.',
          ),
        );
      }
      final endpoint = widget.supervisor.endpoint;
      if (widget.supervisor.state == DshSidecarState.ready &&
          endpoint != null) {
        return DshWebView(
          key: ValueKey('dsh.original-sidebar.$endpoint'),
          url: endpoint,
          presentation: 'navigation',
          activeMountPath: widget.activeMount.value,
          requestedSession: widget.supervisor.requestedSession,
          requestedPanel: widget.supervisor.requestedPanel,
          onNavigation: widget.supervisor.selectNavigation,
          onActivateWorkspace: widget.onActivateWorkspace,
          onOpenResource: widget.onOpenResource,
          reloadToken: 0,
        );
      }
      if (widget.supervisor.state == DshSidecarState.failed ||
          widget.supervisor.state == DshSidecarState.configurationRequired) {
        return Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(zh ? '无法连接 DSH 侧栏' : 'Unable to connect to DSH sidebar'),
              TextButton(onPressed: _start, child: Text(zh ? '重试' : 'Retry')),
            ],
          ),
        );
      }
      return const Center(
        child: SizedBox.square(
          dimension: 20,
          child: CircularProgressIndicator(strokeWidth: 2),
        ),
      );
    },
  );
}
