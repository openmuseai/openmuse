import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:file_picker/file_picker.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';
import 'resource_document.dart';
import 'navigation.dart';
import 'kernel.dart';
import 'client_export.dart';
import 'resource_links.dart';
import 'package:open_filex/open_filex.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

/// A single native document surface. It never starts SiYuan's application shell.
final class SiYuanResourceView extends StatefulWidget {
  const SiYuanResourceView({
    super.key,
    required this.lease,
    required this.editing,
    required this.onDirty,
    required this.onReady,
    required this.onSave,
    this.onController,
    this.canEdit = true,
    this.onEditingChanged,
    this.onOpenLinkedContent,
    this.onOpenResourceUri,
  });
  final SiYuanDocumentLease lease;
  final bool editing;
  final bool canEdit;
  final ValueChanged<bool>? onEditingChanged;
  final Future<bool> Function(String name, Uint8List bytes)?
  onOpenLinkedContent;
  final Future<void> Function(String uri)? onOpenResourceUri;
  final VoidCallback onDirty, onReady, onSave;
  final ValueChanged<SiYuanResourceController>? onController;
  @override
  State<SiYuanResourceView> createState() => _SiYuanResourceViewState();
}

final class SiYuanResourceController {
  SiYuanResourceController._(this.web);
  final WebViewController web;
  final _flushes = <String, Completer<void>>{};
  Future<void> flush({bool preserveEditing = false}) async {
    final id = DateTime.now().microsecondsSinceEpoch.toString();
    final completer = Completer<void>();
    _flushes[id] = completer;
    try {
      await web.runJavaScript(
        'window.openMuseResource.flush(${jsonEncode(id)}, $preserveEditing)',
      );
      await completer.future.timeout(const Duration(seconds: 30));
    } finally {
      _flushes.remove(id);
    }
  }
}

final class _SiYuanResourceViewState extends State<SiYuanResourceView> {
  SiYuanResourceController? controller;
  bool ready = false;
  String? failure;
  bool openingLink = false;
  @override
  void initState() {
    super.initState();
    unawaited(open());
  }

  Future<void> open() async {
    try {
      final kernel = widget.lease.api.kernel;
      final origin = await kernel.start();
      if (!mounted) return;
      final web = WebViewController()
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..addJavaScriptChannel('OpenMuseResource', onMessageReceived: message)
        ..addJavaScriptChannel(
          'SiYuanClient',
          onMessageReceived: (value) => unawaited(export(value.message)),
        )
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) async {
              if (!request.isMainFrame) return NavigationDecision.navigate;
              final navigation = classifyClientNavigation(origin, request.url);
              if (navigation == ClientNavigation.external) {
                await launchUrl(
                  Uri.parse(request.url),
                  mode: LaunchMode.externalApplication,
                );
              }
              return navigation == ClientNavigation.local
                  ? NavigationDecision.navigate
                  : NavigationDecision.prevent;
            },
            onWebResourceError: (error) {
              if (mounted && error.isForMainFrame == true)
                setState(() => failure = error.description);
            },
          ),
        );
      if (web.platform case final AndroidWebViewController android) {
        await android.setOnShowFileSelector((params) async {
          final result = await FilePicker.platform.pickFiles(
            allowMultiple: params.mode == FileSelectorMode.openMultiple,
          );
          return [
            for (final path in result?.paths ?? <String?>[])
              if (path != null) Uri.file(path).toString(),
          ];
        });
      }
      final instance = SiYuanResourceController._(web);
      setState(() => controller = instance);
      widget.onController?.call(instance);
      final mobile =
          defaultTargetPlatform == TargetPlatform.android ||
          defaultTargetPlatform == TargetPlatform.iOS;
      final uri = origin
          .resolve(
            mobile ? '/stage/build/resource-mobile/' : '/stage/build/resource/',
          )
          .replace(
            queryParameters: {
              'id': widget.lease.root,
              'notebook': widget.lease.notebook,
              'editable': '${widget.canEdit}',
              'theme': Theme.of(context).brightness.name,
              'language': siYuanLanguageTag(
                Localizations.localeOf(context).toLanguageTag(),
              ),
            },
          );
      await web.loadRequest(uri, headers: kernel.localSessionHeaders);
    } catch (error) {
      if (mounted) setState(() => failure = '$error');
    }
  }

  Future<void> export(String raw) async {
    final response = await saveClientExport(raw);
    if (mounted && response != null) {
      await controller?.web.runJavaScript(
        'window.dispatchEvent(new CustomEvent("muse-client-response",{detail:${jsonEncode(response)}}))',
      );
    }
  }

  Future<void> openLink(String raw) async {
    if (openingLink) return;
    openingLink = true;
    try {
      final kernel = widget.lease.api.kernel;
      final origin = await kernel.start();
      final target = resolveResourceLink(origin, raw);
      switch (target.kind) {
        case ResourceLinkKind.resource:
          if (widget.onOpenResourceUri == null)
            throw StateError('Resource routing unavailable');
          await widget.onOpenResourceUri!(target.uri.toString());
        case ResourceLinkKind.asset:
          final bytes = await readResourceLinkAsset(
            origin,
            target.uri,
            kernel.localSessionHeaders,
          );
          final name = p.basename(target.uri.path);
          if (await widget.onOpenLinkedContent?.call(name, bytes) == true)
            return;
          final cache = Directory(
            p.join(
              (await getTemporaryDirectory()).path,
              'siyuan-linked',
              DateTime.now().microsecondsSinceEpoch.toString(),
            ),
          );
          await cache.create(recursive: true);
          final file = File(p.join(cache.path, name));
          await file.writeAsBytes(bytes, flush: true);
          final result = await OpenFilex.open(file.path);
          if (result.type != ResultType.done) throw StateError(result.message);
        case ResourceLinkKind.localFile:
          final result = await OpenFilex.open(target.uri.toFilePath());
          if (result.type != ResultType.done) throw StateError(result.message);
        case ResourceLinkKind.external:
          if (!await launchUrl(
            target.uri,
            mode: LaunchMode.externalApplication,
          ))
            throw StateError('Link could not be opened');
        case ResourceLinkKind.blocked:
          throw StateError('Unsupported document link');
      }
    } on Object {
      // A failed link must never replace the originating document surface.
      if (mounted)
        ScaffoldMessenger.maybeOf(context)?.showSnackBar(
          SnackBar(
            content: Text(
              Localizations.localeOf(context).languageCode == 'zh'
                  ? '无法打开此链接。请确认文件仍在手机上，且已安装可打开它的应用。'
                  : 'Unable to open this link. Check that the file is available and a compatible application is installed.',
            ),
          ),
        );
    } finally {
      openingLink = false;
    }
  }

  void message(JavaScriptMessage message) {
    try {
      if (message.message.length > 8192) return;
      final value = jsonDecode(message.message) as Map;
      final data = value['data'] as Map? ?? const {};
      switch (value['method']) {
        case 'ready':
          unawaited(
            controller!.web.runJavaScript(
              'window.openMuseResource?.setEditing(${widget.editing})',
            ),
          );
          setState(() => ready = true);
          widget.onReady();
        case 'dirty':
          if (widget.canEdit) widget.onDirty();
        case 'editing':
          widget.onEditingChanged?.call(data['editing'] == true);
        case 'save':
          if (widget.editing) widget.onSave();
        case 'openLink':
          if (data['uri'] is String) unawaited(openLink(data['uri'] as String));
        case 'flushed':
          controller?._flushes[data['requestId']]?.complete();
        case 'flushError':
          controller?._flushes[data['requestId']]?.completeError(
            StateError('${data['message']}'),
          );
        case 'referenceUnavailable':
          ScaffoldMessenger.maybeOf(context)?.showSnackBar(
            SnackBar(
              content: Text(
                Localizations.localeOf(context).languageCode == 'zh'
                    ? '此引用不在当前笔记包内。请打开其来源资源。'
                    : 'This reference is outside the note package. Open its source resource.',
              ),
            ),
          );
        case 'error':
          setState(() => failure = '${data['message']}');
      }
    } catch (error) {
      if (mounted) setState(() => failure = '$error');
    }
  }

  @override
  void didUpdateWidget(SiYuanResourceView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.editing != widget.editing && ready) {
      unawaited(
        controller!.web.runJavaScript(
          'window.openMuseResource?.setEditing(${widget.editing})',
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    if (failure != null) return Center(child: SelectableText(failure!));
    return Stack(
      children: [
        if (controller != null)
          Positioned.fill(
            child: Opacity(
              opacity: ready ? 1 : 0,
              child: WebViewWidget(controller: controller!.web),
            ),
          ),
        if (!ready) const Center(child: CircularProgressIndicator()),
      ],
    );
  }
}
