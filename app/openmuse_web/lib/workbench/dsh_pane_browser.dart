import 'dart:async';
import 'dart:js_interop';
import 'dart:ui_web' as ui_web;

import 'package:flutter/material.dart';
import 'package:web/web.dart' as web;

@JS('openMuseDshMount')
external JSPromise<JSAny?> _mountDsh(web.HTMLElement container, JSString path);

@JS('openMuseDshUnmount')
external JSPromise<JSAny?> _unmountDsh(web.HTMLElement container);

/// A real DOM node inside the shared Flutter workbench; no iframe/WebView.
final class OpenMuseDshPane extends StatefulWidget {
  const OpenMuseDshPane({super.key, this.bootstrapPath = '/dsh/'});

  final String bootstrapPath;

  @override
  State<OpenMuseDshPane> createState() => _OpenMuseDshPaneState();
}

final class _OpenMuseDshPaneState extends State<OpenMuseDshPane> {
  late final String _viewType = 'openmuse-dsh-pane-${identityHashCode(this)}';
  late final web.HTMLElement _container = web.HTMLDivElement()
    ..id = 'root'
    ..style.width = '100%'
    ..style.height = '100%';
  String? _error;

  @override
  void initState() {
    super.initState();
    ui_web.platformViewRegistry.registerViewFactory(
      _viewType,
      (_) => _container,
    );
  }

  Future<void> _start() async {
    try {
      await _mountDsh(_container, widget.bootstrapPath.toJS).toDart;
    } on Object catch (error) {
      if (mounted) setState(() => _error = error.toString());
    }
  }

  @override
  void dispose() {
    unawaited(_unmountDsh(_container).toDart.catchError((Object _) => null));
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Stack(
    children: [
      Positioned.fill(
        child: HtmlElementView(
          viewType: _viewType,
          onPlatformViewCreated: (_) => unawaited(_start()),
        ),
      ),
      if (_error != null)
        Positioned.fill(
          child: ColoredBox(
            color: Theme.of(context).colorScheme.surface,
            child: Center(child: Text(_error!)),
          ),
        ),
    ],
  );
}
