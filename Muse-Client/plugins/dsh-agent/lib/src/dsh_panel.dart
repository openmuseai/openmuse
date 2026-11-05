import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'dsh_sidecar.dart';
import 'dsh_web_view.dart';
import 'plugin_interaction.dart';

final class DshPanel extends StatefulWidget {
  const DshPanel({
    super.key,
    required this.supervisor,
    required this.desktopInteraction,
    required this.activeMount,
    required this.onActivateWorkspace,
    required this.onOpenResource,
  });
  final DshSidecarSupervisor supervisor;
  final ValueNotifier<PluginInteraction?> desktopInteraction;
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
    widget.desktopInteraction.addListener(_presentDesktopInteraction);
    WidgetsBinding.instance.addPostFrameCallback(
      (_) => _presentDesktopInteraction(),
    );
    Future<void>.microtask(_start);
  }

  @override
  void didUpdateWidget(covariant DshPanel oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.desktopInteraction != widget.desktopInteraction) {
      oldWidget.desktopInteraction.removeListener(_presentDesktopInteraction);
      widget.desktopInteraction.addListener(_presentDesktopInteraction);
      WidgetsBinding.instance.addPostFrameCallback(
        (_) => _presentDesktopInteraction(),
      );
    }
  }

  void _presentDesktopInteraction() {
    if (!mounted) return;
    final interaction = widget.desktopInteraction.value;
    if (interaction == null) return;
    widget.desktopInteraction.value = null;
    unawaited(showPluginInteraction(context, interaction));
  }

  @override
  void dispose() {
    widget.desktopInteraction.removeListener(_presentDesktopInteraction);
    super.dispose();
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
                  child: !widget.supervisor.aiSharingAllowed
                      ? Center(
                          child: Padding(
                            padding: const EdgeInsets.all(24),
                            child: Column(
                              mainAxisSize: MainAxisSize.min,
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  Localizations.localeOf(
                                            context,
                                          ).languageCode ==
                                          'zh'
                                      ? 'AI 数据共享'
                                      : 'AI data sharing',
                                  style: Theme.of(context).textTheme.titleLarge,
                                ),
                                const SizedBox(height: 16),
                                Text(
                                  Localizations.localeOf(
                                            context,
                                          ).languageCode ==
                                          'zh'
                                      ? '你发送的消息、附件，以及要求助手使用的文件内容，会传给 DeepSeek 或你配置的其他模型供应商，并受该供应商的条款和保留政策约束。请仅使用你愿意分享的内容。无需启用 AI 也可以使用本地文档功能。'
                                      : 'Your messages, attachments and file content you ask the assistant to use are sent to DeepSeek or the other model provider you configure. The provider’s terms and retention policies apply. Only share content you intend to send. Local document features remain available without AI.',
                                ),
                                const SizedBox(height: 20),
                                FilledButton(
                                  key: const ValueKey('allow-ai-sharing'),
                                  onPressed: () {
                                    widget.supervisor.allowAiSharing();
                                    unawaited(_start());
                                  },
                                  child: Text(
                                    Localizations.localeOf(
                                              context,
                                            ).languageCode ==
                                            'zh'
                                        ? '允许 AI 共享'
                                        : 'Allow AI sharing',
                                  ),
                                ),
                              ],
                            ),
                          ),
                        )
                      : _PanelBody(
                          supervisor: widget.supervisor,
                          activeMountPath: widget.activeMount.value,
                          onActivateWorkspace: widget.onActivateWorkspace,
                          onOpenResource: widget.onOpenResource,
                          reloadToken: _reloadToken,
                        ),
                ),
              ],
            ),
            if (paneMenu != null) Positioned(top: 4, right: 4, child: paneMenu),
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
        requestedSession: supervisor.requestedSession,
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
                _friendlyError(supervisor.lastError) ?? 'DSH sidecar 未能自动启动。',
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
