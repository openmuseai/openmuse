import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:webview_flutter/webview_flutter.dart';

/// One isolated platform WebView for a plugin's full UI. The workbench itself
/// remains Flutter/A2UI and never embeds platform views in scrolling cards.
final class MobilePluginDetailPage extends StatefulWidget {
  const MobilePluginDetailPage({
    super.key,
    required this.plugin,
    required this.contribution,
    this.onControllerReady,
  });

  final OpenMusePlugin plugin;
  final OpenMuseMobileWorkbenchContributor contribution;
  final ValueChanged<WebViewController>? onControllerReady;

  @override
  State<MobilePluginDetailPage> createState() => _MobilePluginDetailPageState();
}

final class _MobilePluginDetailPageState extends State<MobilePluginDetailPage> {
  WebViewController? _controller;
  String? _failure;
  bool _loading = true;
  Brightness? _brightness;

  @override
  void initState() {
    super.initState();
    if (widget.contribution is OpenMuseMobileNativeDetailContributor) return;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) unawaited(_load());
    });
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final brightness = Theme.of(context).brightness;
    if (_brightness == brightness) return;
    _brightness = brightness;
    if (_controller != null) unawaited(_applyTheme(brightness));
  }

  Future<void> _applyTheme(Brightness brightness) async {
    final controller = _controller;
    if (controller == null) return;
    try {
      await controller.setBackgroundColor(
        Theme.of(context).colorScheme.surface,
      );
      await controller.runJavaScript(
        'document.documentElement.dataset.openmuseTheme='
        '${jsonEncode(brightness == Brightness.dark ? 'dark' : 'light')}',
      );
    } on Object {
      // A disposed WebView may finish after a system theme transition.
    }
  }

  Future<void> _load() async {
    try {
      final document = widget.contribution.buildMobileDetailDocument(
        Theme.of(context).brightness,
      );
      if (document.length > 512 * 1024 ||
          !document.toLowerCase().contains('<head>')) {
        throw const FormatException('invalid plugin detail document');
      }
      const policy =
          "default-src 'none'; script-src 'unsafe-inline'; "
          "style-src 'unsafe-inline'; img-src data:; font-src data:; "
          "connect-src 'none'; frame-src 'none'; object-src 'none'; "
          "form-action 'none'; base-uri 'none'";
      final html = document.replaceFirst(
        RegExp('<head>', caseSensitive: false),
        '<head><meta http-equiv="Content-Security-Policy" '
        'content="$policy">'
        '<script>document.documentElement.dataset.openmuseTheme='
        '${jsonEncode(Theme.of(context).brightness == Brightness.dark ? 'dark' : 'light')};</script>',
      );
      final controller = WebViewController()
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..setBackgroundColor(Theme.of(context).colorScheme.surface)
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) {
              final uri = Uri.tryParse(request.url);
              return uri?.toString() == 'about:blank'
                  ? NavigationDecision.navigate
                  : NavigationDecision.prevent;
            },
            onPageFinished: (_) {
              if (mounted) setState(() => _loading = false);
            },
            onWebResourceError: (error) {
              if (error.isForMainFrame == false || !mounted) return;
              setState(() {
                _loading = false;
                _failure = '插件页面加载失败';
              });
            },
          ),
        )
        ..addJavaScriptChannel(
          'OpenMusePlugin',
          onMessageReceived: (message) => unawaited(_handle(message.message)),
        );
      _controller = controller;
      await controller.loadHtmlString(html);
      widget.onControllerReady?.call(controller);
      if (mounted) setState(() {});
    } on Object catch (error) {
      if (mounted) {
        setState(() {
          _loading = false;
          _failure = '插件页面不可用：$error';
        });
      }
    }
  }

  Future<void> _handle(String raw) async {
    if (raw.length > 16 * 1024) return;
    Object? id;
    Object? result;
    String? error;
    try {
      final value = jsonDecode(raw);
      if (value is! Map ||
          value['id'] is! String ||
          (value['id'] as String).length > 80 ||
          value['method'] is! String ||
          (value['method'] as String).length > 100) {
        throw const FormatException('invalid plugin request');
      }
      id = value['id'];
      result = await widget.contribution.handleMobileDetailRequest(
        value['method'] as String,
        value['arguments'],
      );
    } on Object catch (failure) {
      error = switch (failure) {
        StateError() => failure.message.toString(),
        FormatException() => failure.message,
        _ => '操作失败，请检查 Desktop 连接后重试',
      };
    }
    if (!mounted || id == null) return;
    final response = jsonEncode({
      'id': id,
      if (error == null) 'result': result else 'error': error,
    });
    // JSON encoding prevents plugin-provided data from becoming script code.
    await _controller?.runJavaScript(
      'window.openMuseHostResponse(${jsonEncode(response)})',
    );
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text(widget.plugin.descriptor.name)),
    body: widget.contribution is OpenMuseMobileNativeDetailContributor
        ? (widget.contribution as OpenMuseMobileNativeDetailContributor)
              .buildMobileDetail(context)
        : Stack(
            children: [
              if (_controller case final controller?)
                WebViewWidget(controller: controller),
              if (_loading) const Center(child: CircularProgressIndicator()),
              if (_failure case final failure?)
                Center(child: Text(failure, textAlign: TextAlign.center)),
            ],
          ),
  );
}
