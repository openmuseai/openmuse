import 'dart:io';

import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flowy_infra_ui/style_widget/icon_button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:webview_flutter/webview_flutter.dart';

class DshAgentPanel extends StatefulWidget {
  const DshAgentPanel({super.key});

  @override
  State<DshAgentPanel> createState() => _DshAgentPanelState();
}

class _DshAgentPanelState extends State<DshAgentPanel> {
  WebViewController? _webView;
  String? _loadedUrl;
  String? _webViewError;
  DshAgentController? _controller;
  bool _waitingForSidecar = true;
  final _apiKeyController = TextEditingController();
  var _savingKey = false;

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
    if (controller.launching) {
      _waitingForSidecar = true;
      return;
    }
    if (controller.lastError != null) {
      if (_webView != null || _webViewError != null) {
        setState(() {
          _webView = null;
          _loadedUrl = null;
          _webViewError = null;
        });
      }
      return;
    }
    if (!controller.ready) return;
    if (_webView != null &&
        _loadedUrl == controller.url &&
        _webViewError == null) {
      _waitingForSidecar = false;
      return;
    }
    _waitingForSidecar = false;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      _ensureWebView(controller);
    });
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
      if (controller.lastError!.contains('DEEPSEEK_API_KEY')) {
        return _apiKeyForm(controller);
      }
      return _message(
        controller.lastError!,
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
    final webView = _webView;
    if (webView == null) {
      return const Center(child: CircularProgressIndicator.adaptive());
    }
    return ClipRect(child: WebViewWidget(controller: webView));
  }

  Future<void> _retry(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      if (mounted) {
        _ensureWebView(controller, force: true);
      }
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
                'Enter a DeepSeek API key to start the agent. It is stored only on this Mac.',
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
    required String actionLabel,
    required VoidCallback onAction,
  }) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
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

  void _ensureWebView(DshAgentController controller, {bool force = false}) {
    if (controller.launching ||
        !controller.ready ||
        controller.lastError != null) {
      return;
    }
    if (!force &&
        _webView != null &&
        _loadedUrl == controller.url &&
        _webViewError == null) {
      return;
    }
    try {
      // Do not call setBackgroundColor: WKWebView on macOS throws
      // UnimplementedError ("opaque is not implemented on macOS"), which
      // leaves a native platform view covering the Flutter surface.
      final webView = WebViewController()
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..setNavigationDelegate(
          NavigationDelegate(
            onWebResourceError: (error) {
              if (!mounted) return;
              // Plugin bundles 404ing must not replace the whole panel with
              // "1004". Only the main document failing is a load error.
              if (error.isForMainFrame == false) return;
              final code = error.errorCode.abs();
              if (code == 1004 ||
                  error.errorType == WebResourceErrorType.connect) {
                setState(() {
                  _webView = null;
                  _loadedUrl = null;
                  _webViewError = null;
                });
                return;
              }
              setState(() {
                _webViewError =
                    'Could not load DSH (${error.errorCode}). Use Reload after the sidecar is ready, or Open in browser.';
              });
            },
            onPageFinished: (_) {
              if (!mounted || _webViewError == null) return;
              setState(() => _webViewError = null);
            },
          ),
        )
        ..loadRequest(Uri.parse(controller.url));
      setState(() {
        _webView = webView;
        _loadedUrl = controller.url;
        _webViewError = null;
      });
    } catch (error) {
      setState(() {
        _webViewError =
            'Embedded DSH view is unavailable on this macOS build.\n$error';
      });
    }
  }

  Future<void> _reload(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      if (mounted) _ensureWebView(controller, force: true);
    } catch (_) {
      // The controller carries the redacted sidecar error into the panel.
    }
  }

  Future<void> _openInBrowser(DshAgentController controller) async {
    try {
      await getIt<DshSidecar>().ensureStarted();
      await Process.run('open', [controller.url]);
    } catch (_) {}
  }
}
