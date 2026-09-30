import 'package:flutter/material.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:webview_flutter/webview_flutter.dart';

Uri resolveRemoteDshUri(DshSessionDescriptor session) {
  final origin = Uri.tryParse(session.origin);
  if (origin == null ||
      origin.host.isEmpty ||
      origin.userInfo.isNotEmpty ||
      origin.hasQuery ||
      origin.hasFragment ||
      (origin.path.isNotEmpty && origin.path != '/')) {
    throw const FormatException('invalid Remote DSH origin');
  }
  final loopback =
      origin.host == 'localhost' ||
      origin.host == '127.0.0.1' ||
      origin.host == '::1' ||
      origin.host == '10.0.2.2';
  if (origin.scheme != 'https' &&
      !(session.allowInsecureLoopback && origin.scheme == 'http' && loopback)) {
    throw const FormatException('insecure Remote DSH origin');
  }
  if ((!session.path.startsWith('/session/') &&
          !session.path.startsWith('/u/')) ||
      session.path.contains('..') ||
      session.path.contains('#')) {
    throw const FormatException('invalid Remote DSH path');
  }
  final target = origin.resolve(session.path);
  if (!isAllowedRemoteDshNavigation(target, origin)) {
    throw const FormatException('cross-origin Remote DSH URL');
  }
  return target;
}

bool isAllowedRemoteDshNavigation(Uri target, Uri origin) =>
    target.scheme == origin.scheme &&
    target.host == origin.host &&
    target.port == origin.port &&
    target.userInfo.isEmpty;

final class RemoteDshPage extends StatefulWidget {
  const RemoteDshPage({
    super.key,
    required this.session,
    required this.workspaceTitle,
  });

  final DshSessionDescriptor session;
  final String workspaceTitle;

  @override
  State<RemoteDshPage> createState() => _RemoteDshPageState();
}

final class _RemoteDshPageState extends State<RemoteDshPage> {
  WebViewController? _controller;
  Uri? _target;
  bool _loading = true;
  String? _failure;

  @override
  void initState() {
    super.initState();
    try {
      final target = resolveRemoteDshUri(widget.session);
      final origin = Uri(
        scheme: target.scheme,
        host: target.host,
        port: target.hasPort ? target.port : null,
      );
      _target = target;
      _controller = WebViewController()
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) {
              final requested = Uri.tryParse(request.url);
              if (requested == null ||
                  !isAllowedRemoteDshNavigation(requested, origin)) {
                return NavigationDecision.prevent;
              }
              return NavigationDecision.navigate;
            },
            onPageFinished: (_) {
              if (!mounted) return;
              setState(() {
                _loading = false;
                _failure = null;
              });
            },
            onWebResourceError: (error) {
              if (error.isForMainFrame == false || !mounted) return;
              setState(() {
                _loading = false;
                _failure = 'Agent 页面加载失败，请检查连接后重试。';
              });
            },
          ),
        )
        ..loadRequest(target);
    } on Object {
      _loading = false;
      _failure = 'Remote DSH 地址无效，已拒绝加载。';
    }
  }

  Future<void> _back() async {
    final controller = _controller;
    if (controller != null && await controller.canGoBack()) {
      await controller.goBack();
      return;
    }
    if (mounted) Navigator.of(context).pop();
  }

  Future<void> _reload() async {
    final controller = _controller;
    final target = _target;
    if (controller == null || target == null) return;
    setState(() {
      _loading = true;
      _failure = null;
    });
    await controller.loadRequest(target);
  }

  @override
  Widget build(BuildContext context) => PopScope(
    canPop: false,
    onPopInvokedWithResult: (didPop, _) {
      if (!didPop) _back();
    },
    child: Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: MaterialLocalizations.of(context).backButtonTooltip,
          onPressed: _back,
          icon: const BackButtonIcon(),
        ),
        title: Text('${widget.workspaceTitle} · Agent'),
        actions: [
          IconButton(
            key: const ValueKey('remote-dsh.reload'),
            onPressed: _loading ? null : _reload,
            tooltip: '重新连接',
            icon: const Icon(Icons.refresh),
          ),
        ],
      ),
      body: Stack(
        children: [
          if (_controller case final controller?)
            Positioned.fill(
              child: WebViewWidget(
                key: const ValueKey('remote-dsh.webview'),
                controller: controller,
              ),
            ),
          if (_loading)
            const Positioned.fill(
              child: ColoredBox(
                color: Colors.white,
                child: Center(child: CircularProgressIndicator.adaptive()),
              ),
            ),
          if (_failure case final failure?)
            Positioned.fill(
              child: ColoredBox(
                color: Theme.of(context).colorScheme.surface,
                child: Center(
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(failure, textAlign: TextAlign.center),
                        const SizedBox(height: 12),
                        OutlinedButton(
                          onPressed: _controller == null ? null : _reload,
                          child: const Text('重试'),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
        ],
      ),
    ),
  );
}
