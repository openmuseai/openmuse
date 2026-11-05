import 'dart:async';

import 'package:appflowy/core/helpers/url_launcher.dart';
import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_embedded_view.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:appflowy/plugins/resource_surface/resource_surface.dart';
import 'package:appflowy/plugins/version_diff/presentation/diff_deep_link_opener.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flowy_infra_ui/style_widget/icon_button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class DshAgentPanel extends StatefulWidget {
  const DshAgentPanel({super.key});

  @override
  State<DshAgentPanel> createState() => _DshAgentPanelState();
}

class _DshAgentPanelState extends State<DshAgentPanel> {
  String? _webViewError;
  DshAgentController? _controller;
  final _apiKeyController = TextEditingController();
  var _savingKey = false;
  var _embedEpoch = 0;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final controller = context.read<DshAgentController>();
    if (!identical(_controller, controller)) {
      _controller?.removeListener(_onControllerChanged);
      _controller = controller;
      _controller!.addListener(_onControllerChanged);
    }
    _onControllerChanged();
  }

  @override
  void dispose() {
    _apiKeyController.dispose();
    _controller?.removeListener(_onControllerChanged);
    super.dispose();
  }

  void _onControllerChanged() {
    final controller = _controller;
    if (!mounted || controller == null) return;
    if (controller.lastError != null && _webViewError != null) {
      setState(() => _webViewError = null);
    }
  }

  @override
  Widget build(BuildContext context) {
    final controller = context.watch<DshAgentController>();
    return ColoredBox(
      color: Theme.of(context).colorScheme.surface,
      child: Column(
        children: [
          SizedBox(
            height: 48,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: Row(
                children: [
                  const FlowySvg(
                    FlowySvgs.m_home_ai_chat_icon_m,
                    size: Size.square(16),
                  ),
                  const SizedBox(width: 8),
                  const Expanded(
                    child: FlowyText(
                      'DeepSeek Agent',
                      fontSize: 13,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  if (controller.launching)
                    const SizedBox(
                      width: 14,
                      height: 14,
                      child: CircularProgressIndicator(strokeWidth: 2),
                    ),
                  FlowyIconButton(
                    tooltipText: 'Open file in OpenMuse',
                    width: 24,
                    icon: const Icon(Icons.folder_open_outlined, size: 16),
                    onPressed: () =>
                        MuseResourceSurfaceOpener.pickAndOpen(context),
                  ),
                  FlowyIconButton(
                    tooltipText: 'Reload',
                    width: 24,
                    icon: const Icon(Icons.refresh, size: 16),
                    onPressed: () => _reload(controller),
                  ),
                  FlowyIconButton(
                    tooltipText: 'Open in browser',
                    width: 24,
                    icon: const Icon(Icons.open_in_browser, size: 16),
                    onPressed: () => _openInBrowser(controller),
                  ),
                  FlowyIconButton(
                    tooltipText: 'Close',
                    width: 24,
                    icon: const FlowySvg(
                      FlowySvgs.show_menu_s,
                      size: Size.square(16),
                    ),
                    onPressed: () => controller.setOpen(false),
                  ),
                ],
              ),
            ),
          ),
          const Divider(height: 1),
          Expanded(child: _body(controller)),
        ],
      ),
    );
  }

  Widget _body(DshAgentController controller) {
    if (controller.launching) {
      return const Center(
        child: Padding(
          padding: EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              CircularProgressIndicator.adaptive(),
              SizedBox(height: 12),
              FlowyText(
                'Starting DeepSeek Agent…',
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      );
    }
    if (controller.lastError != null) {
      if (controller.errorCode == 'NEED_API_KEY' ||
          controller.lastError!.contains('DEEPSEEK_API_KEY')) {
        return _apiKeyForm(controller);
      }
      return _message(
        controller.lastError!,
        code: controller.errorCode,
        actionLabel: 'Retry',
        onAction: () => _retry(controller),
      );
    }
    if (_webViewError != null) {
      return _message(
        _webViewError!,
        actionLabel: 'Reload',
        onAction: () => _reload(controller),
      );
    }
    if (!controller.ready) {
      return const Center(child: CircularProgressIndicator.adaptive());
    }
    return ClipRect(
      child: DshEmbeddedView(
        key: ValueKey('$_embedEpoch:${controller.url}'),
        url: controller.url,
        onError: (message) {
          if (!mounted) return;
          setState(() => _webViewError = message);
        },
        onOpenResource: (message) {
          unawaited(
            message.comparisonId == null
                ? MuseResourceSurfaceOpener.open(
                    context,
                    MuseResourceOpenRequest(
                      path: message.path,
                      sessionCwd: message.cwd,
                      line: message.line,
                      origin: MuseResourceOpenOrigin.dshConversation,
                    ),
                  )
                : MuseDiffDeepLinkOpener.open(
                    context: context,
                    path: message.path,
                    sessionCwd: message.cwd,
                    comparisonId: message.comparisonId!,
                    changeId: message.changeId,
                  ),
          );
        },
      ),
    );
  }

  Future<void> _retry(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      if (mounted) _rebuildEmbeddedView();
    } catch (_) {}
  }

  Widget _apiKeyForm(DshAgentController controller) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 360),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const FlowyText(
                'Enter a DeepSeek API key to start the agent. It is stored only on this device.',
                maxLines: 6,
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _apiKeyController,
                obscureText: true,
                enabled: !_savingKey,
                decoration: const InputDecoration(
                  labelText: 'DEEPSEEK_API_KEY',
                  border: OutlineInputBorder(),
                ),
                onSubmitted: (_) => _saveApiKey(controller),
              ),
              const SizedBox(height: 12),
              TextButton(
                onPressed: _savingKey ? null : () => _saveApiKey(controller),
                child: Text(_savingKey ? 'Saving…' : 'Save and start'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _saveApiKey(DshAgentController controller) async {
    final key = _apiKeyController.text.trim();
    if (key.isEmpty) return;
    setState(() => _savingKey = true);
    try {
      await getIt<DshSidecar>().saveApiKey(key);
      await _retry(controller);
    } catch (_) {
    } finally {
      if (mounted) setState(() => _savingKey = false);
    }
  }

  Widget _message(
    String text, {
    String? code,
    required String actionLabel,
    required VoidCallback onAction,
  }) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (code != null && code.isNotEmpty)
              Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: FlowyText(
                  code,
                  fontSize: 12,
                  textAlign: TextAlign.center,
                ),
              ),
            FlowyText(
              text,
              maxLines: 8,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 12),
            TextButton(
              onPressed: onAction,
              child: Text(actionLabel),
            ),
          ],
        ),
      ),
    );
  }

  void _rebuildEmbeddedView() {
    setState(() {
      _webViewError = null;
      _embedEpoch++;
    });
  }

  Future<void> _reload(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      if (mounted) _rebuildEmbeddedView();
    } catch (_) {
      // The controller carries the redacted sidecar error into the panel.
    }
  }

  Future<void> _openInBrowser(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      await afLaunchUrlString(controller.url);
    } catch (_) {}
  }
}
