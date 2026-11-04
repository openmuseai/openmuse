import 'client_export.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';

import 'kernel.dart';
import 'navigation.dart';

/// The original SiYuan UI and local kernel are owned by the plugin. The Host
/// mounts this widget through an ordinary contribution and interprets no APIs.
final class SiYuanClientView extends StatefulWidget {
  const SiYuanClientView({
    super.key,
    required this.kernel,
    this.onReady,
    this.exportWriter,
  });
  final SiYuanKernel kernel;
  final ValueChanged<WebViewController>? onReady;
  final Future<bool> Function(String name, Uint8List bytes)? exportWriter;
  @override
  State<SiYuanClientView> createState() => _SiYuanClientViewState();
}

final class _SiYuanClientViewState extends State<SiYuanClientView>
    with WidgetsBindingObserver {
  WebViewController? _controller;
  Object? _error;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  bool _started = false;
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (!_started) {
      _started = true;
      unawaited(
        _open(Localizations.maybeLocaleOf(context)?.toLanguageTag() ?? 'en'),
      );
    }
  }

  Future<void> _open(String language) async {
    try {
      final origin = await widget.kernel.start(language: language);
      if (!mounted) return;
      final controller = WebViewController()
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..addJavaScriptChannel(
          'SiYuanClient',
          onMessageReceived: (message) => unawaited(_export(message.message)),
        )
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) async {
              if (!request.isMainFrame) return NavigationDecision.navigate;
              switch (classifyClientNavigation(origin, request.url)) {
                case ClientNavigation.local:
                  return NavigationDecision.navigate;
                case ClientNavigation.external:
                  await launchUrl(
                    Uri.parse(request.url),
                    mode: LaunchMode.externalApplication,
                  );
                  return NavigationDecision.prevent;
                case ClientNavigation.blocked:
                  return NavigationDecision.prevent;
              }
            },
            onWebResourceError: (error) {
              if (mounted && error.isForMainFrame == true)
                setState(() => _error = error.description);
            },
            onPageFinished: (_) => widget.onReady?.call(_controller!),
          ),
        );
      if (controller.platform case final AndroidWebViewController android) {
        await android.setOnShowFileSelector((params) async {
          final selection = await FilePicker.platform.pickFiles(
            allowMultiple: params.mode == FileSelectorMode.openMultiple,
          );
          return [
            for (final path in selection?.paths ?? <String?>[])
              if (path != null) Uri.file(path).toString(),
          ];
        });
      }
      setState(() => _controller = controller);
      await controller.loadRequest(
        origin.resolve(
          Platform.isMacOS ? '/stage/build/desktop/' : '/stage/build/mobile/',
        ),
        headers: widget.kernel.localSessionHeaders,
      );
    } catch (error) {
      if (mounted) setState(() => _error = error);
    }
  }

  Future<void> _export(String raw) async {
    final response = await saveClientExport(
      raw,
      exportWriter: widget.exportWriter,
    );
    if (mounted && response != null) {
      await _controller?.runJavaScript(
        'window.dispatchEvent(new CustomEvent("muse-client-response",{detail:${jsonEncode(response)}}))',
      );
    }
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.inactive ||
        state == AppLifecycleState.paused) {
      unawaited(
        widget.kernel.flush().catchError((Object error) {
          if (mounted) setState(() => _error = error);
        }),
      );
    }
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    unawaited(widget.kernel.flush());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_error != null) return Center(child: SelectableText('OpenMuse Notes: $_error'));
    if (_controller case final controller?)
      return WebViewWidget(controller: controller);
    return const Center(child: CircularProgressIndicator());
  }
}
