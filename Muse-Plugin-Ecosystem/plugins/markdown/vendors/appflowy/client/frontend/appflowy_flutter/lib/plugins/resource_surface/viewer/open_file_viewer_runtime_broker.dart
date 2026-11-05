import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:ui';

import 'package:path/path.dart' as p;
import 'package:webview_windows/webview_windows.dart';

const windowsViewerHost = 'openmuse.viewer';
const windowsResourceHost = 'resource.openmuse';

Directory? bundledViewerRoot() {
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

final class OpenFileViewerRuntime {
  OpenFileViewerRuntime._({
    required this.controller,
    required this.messages,
    required StreamSubscription<dynamic> nativeMessages,
    required StreamController<dynamic> messageBus,
    required this.prewarmed,
  })  : _nativeMessages = nativeMessages,
        _messageBus = messageBus;

  final WebviewController controller;
  final Stream<dynamic> messages;
  final bool prewarmed;
  final StreamSubscription<dynamic> _nativeMessages;
  final StreamController<dynamic> _messageBus;

  Future<void> dispose() async {
    await _nativeMessages.cancel();
    await _messageBus.close();
    await controller.dispose();
  }
}

/// Keeps one fully initialized, shell-ready WebView2 runtime per process.
///
/// The broker has no dependency on Resource routing, Tabs, Workspace or DSH.
/// A surface claims ownership of a controller and disposes it normally; after
/// its first interactive frame it asks the broker to prepare the next one.
final class OpenFileViewerRuntimeBroker {
  OpenFileViewerRuntimeBroker._();

  static final instance = OpenFileViewerRuntimeBroker._();
  static const idleTtl = Duration(minutes: 10);

  Future<OpenFileViewerRuntime>? _next;
  Timer? _idleExpiry;

  bool get supported => Platform.isWindows;

  Future<void> warmUp() async {
    if (!supported) return;
    final future = _next ??= _createShell();
    try {
      await future;
      if (identical(_next, future)) _scheduleExpiry(future);
    } on Object {
      if (identical(_next, future)) _next = null;
      // Warm-up is an optional optimization. The real acquisition path will
      // retry and report a user-visible error if WebView2 is unavailable.
    }
  }

  Future<OpenFileViewerRuntime> acquire() async {
    if (!supported) {
      throw StateError('WEBVIEW2_RUNTIME_UNAVAILABLE');
    }
    _idleExpiry?.cancel();
    _idleExpiry = null;
    final prewarmed = _next != null;
    final future = _next ??= _createShell();
    try {
      final runtime = await future;
      _idleExpiry?.cancel();
      _idleExpiry = null;
      if (identical(_next, future)) _next = null;
      return OpenFileViewerRuntime._(
        controller: runtime.controller,
        messages: runtime.messages,
        nativeMessages: runtime._nativeMessages,
        messageBus: runtime._messageBus,
        prewarmed: prewarmed,
      );
    } on Object {
      if (identical(_next, future)) _next = null;
      rethrow;
    }
  }

  void _scheduleExpiry(Future<OpenFileViewerRuntime> future) {
    _idleExpiry?.cancel();
    _idleExpiry = Timer(idleTtl, () {
      if (!identical(_next, future)) return;
      _next = null;
      _idleExpiry = null;
      unawaited(future.then((runtime) => runtime.dispose(), onError: (_) {}));
    });
  }

  Future<OpenFileViewerRuntime> _createShell() async {
    final root = bundledViewerRoot();
    if (root == null) {
      throw StateError('assets/engines/open-file-viewer is missing');
    }
    final controller = WebviewController();
    final messageBus = StreamController<dynamic>.broadcast();
    StreamSubscription<dynamic>? nativeMessages;
    StreamSubscription<dynamic>? shellMessages;
    try {
      await controller.initialize();
      await controller.addVirtualHostNameMapping(
        windowsViewerHost,
        root.path,
        WebviewHostResourceAccessKind.allow,
      );
      await controller.setBackgroundColor(const Color(0xFF20252E));
      nativeMessages = controller.webMessage.listen(
        messageBus.add,
        onError: messageBus.addError,
      );
      final shellReady = Completer<void>();
      shellMessages = messageBus.stream.listen((message) {
        if (shellReady.isCompleted || message is! String) return;
        try {
          final decoded = jsonDecode(message);
          if (decoded is Map && decoded['type'] == 'viewer.shell-ready') {
            shellReady.complete();
          }
        } on FormatException {
          // Ignore unrelated messages while the shell starts.
        }
      });
      await controller.loadUrl('https://$windowsViewerHost/index.html');
      await shellReady.future.timeout(const Duration(seconds: 15));
      await shellMessages.cancel();
      shellMessages = null;
      return OpenFileViewerRuntime._(
        controller: controller,
        messages: messageBus.stream,
        nativeMessages: nativeMessages,
        messageBus: messageBus,
        prewarmed: false,
      );
    } on Object {
      await shellMessages?.cancel();
      await nativeMessages?.cancel();
      await messageBus.close();
      await controller.dispose();
      rethrow;
    }
  }

  void dispose() {
    _idleExpiry?.cancel();
    _idleExpiry = null;
    final pending = _next;
    _next = null;
    if (pending != null) {
      unawaited(
        pending.then((runtime) => runtime.dispose(), onError: (_) {}),
      );
    }
  }
}
