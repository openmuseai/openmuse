import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'remote_app.dart';
import 'request_failure.dart';
import 'resource_link.dart';
import 'mcp_resource_source.dart';

class RemoteMcpAppView extends StatefulWidget {
  const RemoteMcpAppView({
    super.key,
    required this.app,
    this.presentation = 'workspace',
  });
  final RemoteMcpApp app;
  final String presentation;
  @override
  State<RemoteMcpAppView> createState() => _RemoteMcpAppViewState();
}

class _RemoteMcpAppViewState extends State<RemoteMcpAppView> {
  MethodChannel? _channel;
  WebViewController? _web;
  String? _document, _failure;
  late final String _scope = widget.app.scope();
  @override
  void initState() {
    super.initState();
    widget.app.addListener(_scopeChanged);
    _start();
  }

  Map<String, Object?> get _arguments => {
    '_openmuse': {
      'presentation': widget.presentation,
      'navigation': widget.app.navigationState,
    },
  };
  void _sendNavigation() {
    final message = {
      'jsonrpc': '2.0',
      'method': 'ui/notifications/tool-input',
      'params': {'arguments': _arguments},
    };
    if (_channel != null)
      unawaited(_channel!.invokeMethod<void>('message', message));
    if (_web != null)
      unawaited(
        _web!.runJavaScript('window.__mcpHostReceive(${jsonEncode(message)})'),
      );
  }

  void _scopeChanged() {
    if (mounted && widget.app.scope() == _scope) {
      _sendNavigation();
      return;
    }

    if (mounted && widget.app.scope() != _scope) {
      _channel?.setMethodCallHandler(null);
      _web = null;
      setState(() {
        _document = null;
        _failure = '账号已切换，请重新打开插件。';
      });
    }
  }

  Future<void> _start() async {
    try {
      await widget.app.request('tools/list', {});
      if (!mounted || widget.app.scope() != _scope) return;
      final document = buildMcpAppHostDocument(
        appHtml: widget.app.html,
        arguments: _arguments,
        platform: Platform.isMacOS ? 'desktop' : 'mobile',
        serverAvailable: true,
        resourceLinksAvailable: ResourceLinkRouting.maybeOf(context) != null,
        nativeSend: Platform.isMacOS
            ? 'window.webkit.messageHandlers.mcpApp.postMessage(message)'
            : 'window.McpHost.postMessage(JSON.stringify(message))',
      );
      if (!Platform.isMacOS) {
        final web = WebViewController()
          ..setJavaScriptMode(JavaScriptMode.unrestricted);
        await web.addJavaScriptChannel(
          'McpHost',
          onMessageReceived: (event) async {
            final response = await _dispatch(jsonDecode(event.message));
            if (mounted && response != null)
              await web.runJavaScript(
                'window.__mcpHostReceive(${jsonEncode(response)})',
              );
          },
        );
        await web.setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) =>
                request.url == 'about:blank' ||
                    request.url == 'about:srcdoc' ||
                    request.url.startsWith('data:text/html')
                ? NavigationDecision.navigate
                : NavigationDecision.prevent,
          ),
        );
        await web.loadHtmlString(document, baseUrl: 'https://mcp-app.invalid/');
        _web = web;
      }
      if (mounted) setState(() => _document = document);
    } catch (error) {
      // Only locally defined codes are safe to render. Transport exceptions can
      // contain credentials, endpoints or response bodies.
      final code =
          error is StateError &&
              RegExp(r'^APP_[A-Z_]+$').hasMatch('${error.message}')
          ? '${error.message}'
          : 'APP_CONNECTION_FAILED';
      if (mounted) {
        setState(() => _failure = '插件连接失败（$code），请检查后端、凭据和当前账号。');
      }
    }
  }

  Future<Map<String, Object?>?> _dispatch(dynamic raw) async {
    if (raw is! Map || raw['jsonrpc'] != '2.0' || raw['id'] == null)
      return null;
    final response = <String, Object?>{'jsonrpc': '2.0', 'id': raw['id']};
    try {
      if (!mounted || widget.app.scope() != _scope)
        throw StateError('VIEW_REVOKED');
      final Map<String, Object?> result;
      if (raw['method'] == 'ui/open-link') {
        final target = (raw['params'] as Map?)?['url'];
        final reference = target is String && target.startsWith('mcp-resource:')
            ? checkedMcpResourceLink(target, _scope, widget.app.id)
            : checkedAppResourceLink(target, _scope);
        final routing = ResourceLinkRouting.maybeOf(context);
        if (routing == null)
          throw StateError('APP_RESOURCE_ROUTING_UNAVAILABLE');
        // Opening the resource awaits its own native authorization/UI route.
        // Release the MCP reply before navigation's Future waits for page pop.
        unawaited(routing.open(reference.toUri().toString()));
        result = {'status': 'success'};
      } else {
        result = await widget.app.request(
          raw['method'] as String,
          Map<String, Object?>.from(raw['params'] as Map? ?? {}),
        );
      }
      if (!mounted || widget.app.scope() != _scope)
        throw StateError('VIEW_REVOKED');
      response['result'] = result;
    } catch (error) {
      response['error'] = {
        'code': -32602,
        'message': appRequestFailureCode(error),
      };
    }
    return response;
  }

  void _created(int id) {
    final channel = MethodChannel('com.openmuse.mcp/app-view/$id');
    _channel = channel;
    channel.setMethodCallHandler((call) async {
      if (call.method != 'message') return;
      final response = await _dispatch(call.arguments);
      if (mounted && response != null)
        await channel.invokeMethod<void>('message', response);
    });
  }

  @override
  void dispose() {
    widget.app.removeListener(_scopeChanged);
    _channel?.setMethodCallHandler(null);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_failure != null)
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(_failure!),
            TextButton(
              onPressed: () {
                setState(() => _failure = null);
                _start();
              },
              child: const Text('重试'),
            ),
          ],
        ),
      );
    if (_document == null)
      return const Center(child: CircularProgressIndicator());
    if (Platform.isMacOS)
      return AppKitView(
        viewType: 'com.openmuse.mcp/app-view',
        creationParams: {'html': _document},
        creationParamsCodec: const StandardMessageCodec(),
        onPlatformViewCreated: _created,
      );
    return WebViewWidget(controller: _web!);
  }
}
