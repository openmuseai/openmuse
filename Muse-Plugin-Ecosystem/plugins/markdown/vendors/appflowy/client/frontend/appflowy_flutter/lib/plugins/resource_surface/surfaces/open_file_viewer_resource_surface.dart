import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/core/performance/muse_performance_trace.dart';
import 'package:flutter/material.dart';
import 'package:mime/mime.dart';
import 'package:path/path.dart' as p;
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_windows/webview_windows.dart';

/// Virtual host used to serve the bundled viewer page on Windows. WebView2
/// cannot load Flutter assets from the asset bundle, and `file://` URLs break
/// the viewer's relative resources (pdf.js worker, cmaps, standard fonts) and
/// its module/worker loading rules, so the on-disk asset directory is mapped to
/// a host name instead.
const _windowsViewerHost = 'openmuse.viewer';

/// The viewer page plus its runtime live next to the executable in every build
/// (portable dir, installer, dev tree all use `data/flutter_assets/...`).
Directory? _bundledViewerRoot() {
  final root = Directory(
    p.join(
      File(Platform.resolvedExecutable).parent.path,
      'data',
      'flutter_assets',
      'assets',
      'engines',
      'open-file-viewer',
    ),
  );
  return root.existsSync() ? root : null;
}

class OpenFileViewerResourceSurface extends StatefulWidget {
  const OpenFileViewerResourceSurface({
    super.key,
    required this.file,
    this.initialLine,
    this.performanceTrace,
  });

  final File file;
  final int? initialLine;
  final MusePerformanceTrace? performanceTrace;

  @override
  State<OpenFileViewerResourceSurface> createState() =>
      _OpenFileViewerResourceSurfaceState();
}

class _OpenFileViewerResourceSurfaceState
    extends State<OpenFileViewerResourceSurface> {
  WebViewController? _controller;
  String? _error;
  var _loaded = false;

  // Windows (webview_windows / Edge WebView2) state.
  WebviewController? _windowsController;
  StreamSubscription<dynamic>? _windowsMessages;
  StreamSubscription<LoadingState>? _windowsLoading;
  var _windowsSent = false;
  var _disposed = false;
  var _firstFrameScheduled = false;

  @override
  void initState() {
    super.initState();
    if (Platform.isWindows) {
      unawaited(_bootWindows());
      return;
    }
    unawaited(_boot());
  }

  Future<void> _boot() async {
    try {
      final controller = WebViewController();
      await controller.setJavaScriptMode(JavaScriptMode.unrestricted);
      await controller.setNavigationDelegate(
        NavigationDelegate(
          onPageFinished: (_) => _sendFile(),
          onNavigationRequest: (request) =>
              request.isMainFrame && !request.url.startsWith('file:')
                  ? NavigationDecision.prevent
                  : NavigationDecision.navigate,
        ),
      );
      await controller.addJavaScriptChannel(
        'MuseViewerBridge',
        onMessageReceived: (message) => _onBridgeMessage(message.message),
      );
      if (!mounted) return;
      setState(() => _controller = controller);
      await controller.loadFlutterAsset(
        'assets/engines/open-file-viewer/index.html',
      );
    } on Object catch (error) {
      widget.performanceTrace?.fail(error);
      if (mounted) {
        setState(() => _error = 'Viewer runtime unavailable: $error');
      }
    }
  }

  Future<void> _bootWindows() async {
    try {
      final controller = WebviewController();
      await controller.initialize();
      if (_disposed) {
        await controller.dispose();
        return;
      }
      final viewerRoot = _bundledViewerRoot();
      if (viewerRoot == null) {
        throw StateError('assets/engines/open-file-viewer is missing');
      }
      await controller.addVirtualHostNameMapping(
        _windowsViewerHost,
        viewerRoot.path,
        WebviewHostResourceAccessKind.allow,
      );
      await controller.setBackgroundColor(const Color(0xFF20252E));
      _windowsLoading = controller.loadingState.listen((state) {
        if (state == LoadingState.navigationCompleted) {
          unawaited(_sendFileWindows());
        }
      });
      _windowsMessages = controller.webMessage.listen((message) {
        if (message is String) {
          _onBridgeMessage(message);
        } else if (message is Map) {
          _onBridgeMessage(jsonEncode(message));
        }
      });
      if (!mounted || _disposed) return;
      setState(() => _windowsController = controller);
      await controller.loadUrl('https://$_windowsViewerHost/index.html');
    } on Object catch (error) {
      widget.performanceTrace?.fail(error);
      if (mounted) {
        setState(() => _error = 'Viewer runtime unavailable: $error');
      }
    }
  }

  void _onBridgeMessage(String raw) {
    final decoded = jsonDecode(raw);
    if (decoded is! Map) return;
    if (decoded['type'] == 'viewer.ready' && mounted) {
      setState(() => _loaded = true);
      _finishFirstInteractiveFrame();
    }
    if (decoded['type'] == 'viewer.error' && mounted) {
      widget.performanceTrace?.fail('${decoded['message']}');
      setState(() => _error = '${decoded['message']}');
    }
  }

  void _finishFirstInteractiveFrame() {
    final trace = widget.performanceTrace;
    if (trace == null || trace.isFinished || _firstFrameScheduled) return;
    _firstFrameScheduled = true;
    final span = trace.startSpan(
      'engine.first-interactive-frame',
      category: 'engine',
    );
    WidgetsBinding.instance.addPostFrameCallback((_) {
      span.end();
      trace.finish();
    });
  }

  Future<String> _payload() async {
    return jsonEncode({
      'name': p.basename(widget.file.path),
      'mime': lookupMimeType(widget.file.path) ?? 'application/octet-stream',
      'base64': base64Encode(await widget.file.readAsBytes()),
      if (widget.initialLine != null) 'line': widget.initialLine,
      if (widget.performanceTrace != null)
        'requestId': widget.performanceTrace!.traceId,
    });
  }

  Future<void> _sendFile() async {
    final controller = _controller;
    if (controller == null) return;
    try {
      final length = await widget.file.length();
      if (length > 64 * 1024 * 1024) {
        throw StateError('RESOURCE_TOO_LARGE (64 MiB desktop bridge limit)');
      }
      final payload = await _payload();
      // WKWebView cannot bridge a JavaScript Promise back through
      // evaluateJavaScript. Explicitly discard the async open() result; viewer
      // readiness and errors are reported through MuseViewerBridge instead.
      await controller.runJavaScript('void window.MuseViewer.open($payload)');
    } on Object catch (error) {
      widget.performanceTrace?.fail(error);
      if (mounted) setState(() => _error = 'Unable to render resource: $error');
    }
  }

  Future<void> _sendFileWindows() async {
    final controller = _windowsController;
    if (controller == null || _windowsSent || _disposed) return;
    _windowsSent = true;
    try {
      final length = await widget.file.length();
      if (length > 64 * 1024 * 1024) {
        throw StateError('RESOURCE_TOO_LARGE (64 MiB desktop bridge limit)');
      }
      await controller.postWebMessage(await _payload());
    } on Object catch (error) {
      widget.performanceTrace?.fail(error);
      if (mounted) setState(() => _error = 'Unable to render resource: $error');
    }
  }

  @override
  void dispose() {
    _disposed = true;
    unawaited(_windowsLoading?.cancel());
    unawaited(_windowsMessages?.cancel());
    final windowsController = _windowsController;
    if (windowsController != null) {
      unawaited(windowsController.dispose());
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) return Center(child: Text(_error!));
    if (Platform.isWindows) {
      final windowsController = _windowsController;
      if (windowsController == null) {
        return const Center(child: CircularProgressIndicator());
      }
      return SizedBox.expand(
        child: Stack(
          fit: StackFit.expand,
          children: [
            Webview(windowsController),
            if (!_loaded) const Center(child: CircularProgressIndicator()),
          ],
        ),
      );
    }
    final controller = _controller;
    if (controller == null) {
      return const Center(child: CircularProgressIndicator());
    }
    return SizedBox.expand(
      child: Stack(
        fit: StackFit.expand,
        children: [
          WebViewWidget(controller: controller),
          if (!_loaded) const Center(child: CircularProgressIndicator()),
        ],
      ),
    );
  }
}
