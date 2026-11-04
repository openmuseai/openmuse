import '../mobile_build_locale.dart';

import 'dart:async';
import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Generic application-wide plugin surfaces. No PPTX, narration, DSH or PDF
/// operations are interpreted here. Minimize preserves the isolated View.
final class MobileResourceExtensions extends ChangeNotifier {
  MobileResourceExtensions({
    required this.market,
    required this.contributors,
    required this.approve,
  });
  final Future<RemoteMcpMarket> Function() market;
  final List<({String id, OpenMuseToolContributor plugin})> Function()
  contributors;
  final Future<bool> Function(String, String) approve;
  final broker = ResourceAgentBroker();
  final windows = <MobileExtensionWindow>[];
  ResourceAppSession? _resource;
  RemoteMcpViewConnection? _connection;
  String? _type;
  int _generation = 0;
  bool _disposed = false;
  Future<void> open(
    RemoteMcpViewConnection connection,
    String title,
    String type,
    String digest,
  ) async {
    final previous = _connection;
    if (previous != null) closeResource(previous);
    final previousWindows = windows.toList();
    windows.clear();
    for (final window in previousWindows) {
      unawaited(window.session.close());
    }
    final generation = ++_generation;
    _connection = connection;
    _type = type;
    final resource = ResourceAppSession(
      uri: 'urn:openmuse:mobile-resource:$generation',
      title: title,
      request: connection.request,
    );
    _resource = resource;
    broker.register(resource);
    broker.activeUri = resource.uri;
    try {
      final source = await market();
      final specs = await source.resourceExtensions(type);
      if (_disposed || generation != _generation) return;
      for (final spec in specs) {
        final entry = MobileExtensionWindow(spec, resource.uri);
        final session = ResourceExtensionSession(
          caller: 'mobile-extension:$generation:${spec['id']}',
          resourceUri: resource.uri,
          broker: broker,
          contributors: contributors,
          approve: approve,
          resourceTools: (spec['resourceTools'] as List).cast<String>().toSet(),
          isCurrent: () async =>
              !_disposed &&
              generation == _generation &&
              (await source.installedEntries()).any(
                (e) =>
                    e.name == spec['name'] &&
                    e.version == spec['version'] &&
                    e.packageSha256 == spec['digest'],
              ),
        );
        entry.session = session;
        windows.add(entry);
      }
      notifyListeners();
    } catch (_) {
      /* Optional UI discovery cannot prevent a document opening. */
    }
  }

  void closeResource(RemoteMcpViewConnection connection) {
    if (!identical(connection, _connection)) return;
    ++_generation;
    _connection = null;
    if (_resource case final resource?) broker.unregister(resource);
    _resource = null;
    broker.activeUri = null;
    for (final window in windows) {
      window.active = false;
      unawaited(
        window.view?.receive({
              'jsonrpc': '2.0',
              'method': 'ui/notifications/tool-input',
              'params': {
                'arguments': {
                  'resourceUri': window.resourceUri,
                  'mediaType': _type,
                  'active': false,
                },
              },
            }) ??
            Future<void>.value(),
      );
      unawaited(window.session.close());
    }
    notifyListeners();
  }

  Future<void> closeWindow(MobileExtensionWindow window) async {
    if (window.closing) return;
    window.closing = true;
    // Revoke before disposing the View; task cancellation belongs to provider.
    final close = window.session.close();
    window.active = false;
    await window.view?.receive({
      'jsonrpc': '2.0',
      'method': 'ui/notifications/tool-input',
      'params': {
        'arguments': {
          'resourceUri': window.resourceUri,
          'mediaType': _type,
          'active': false,
        },
      },
    });
    if (!_disposed) {
      windows.remove(window);
      notifyListeners();
    }
    await close;
  }

  Future<void> refresh() async {
    final connection = _connection, resource = _resource, type = _type;
    if (connection != null && resource != null && type != null) {
      await open(connection, resource.title, type, '');
    }
  }

  Future<void> clear() async {
    ++_generation;
    _connection = null;
    _resource = null;
    broker.clear();
    final old = windows.toList();
    windows.clear();
    if (!_disposed) notifyListeners();
    await Future.wait(old.map((w) => w.session.close()));
  }

  void toggle(MobileExtensionWindow window) {
    window.minimized = !window.minimized;
    notifyListeners();
  }

  Map<String, Object?> arguments(MobileExtensionWindow window) => {
    'resourceUri': window.resourceUri,
    'mediaType': _type,
    'active': window.active,
  };
  @override
  void dispose() {
    _disposed = true;
    unawaited(clear());
    broker.dispose();
    super.dispose();
  }
}

final class MobileExtensionWindow {
  MobileExtensionWindow(this.spec, this.resourceUri);
  final Map<String, Object?> spec;
  final String resourceUri;
  late final ResourceExtensionSession session;
  RemoteMcpViewConnection? view;
  bool minimized = false, active = true, closing = false;
}

final class MobileResourceExtensionLayer extends StatelessWidget {
  const MobileResourceExtensionLayer({
    super.key,
    required this.controller,
    required this.child,
  });
  final MobileResourceExtensions controller;
  final Widget child;
  @override
  Widget build(BuildContext context) => RemoteMcpResourceHostScope(
    open: controller.open,
    close: controller.closeResource,
    refresh: controller.refresh,
    child: Overlay.wrap(
      child: ListenableBuilder(
        listenable: controller,
        builder: (context, _) => Stack(
          children: [
            Positioned.fill(child: child),
            for (final (index, window) in controller.windows.indexed)
              Positioned(
                key: ObjectKey(window),
                right: 12,
                bottom: MediaQuery.of(context).padding.bottom + 80 + index * 60,
                child: _FloatingWindow(controller: controller, window: window),
              ),
          ],
        ),
      ),
    ),
  );
}

final class _FloatingWindow extends StatelessWidget {
  const _FloatingWindow({required this.controller, required this.window});
  final MobileResourceExtensions controller;
  final MobileExtensionWindow window;
  @override
  Widget build(BuildContext context) {
    final spec = window.spec;
    final width = math.min(
      (spec['width'] as int).toDouble(),
      MediaQuery.sizeOf(context).width - 24,
    );
    final height = (spec['height'] as int).toDouble();
    return Material(
      elevation: 4,
      borderRadius: BorderRadius.circular(12),
      clipBehavior: Clip.antiAlias,
      color: Theme.of(context).colorScheme.surface,
      child: SizedBox(
        width: window.minimized ? 176 : width,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(
              height: 48,
              child: Row(
                children: [
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      spec['title'] as String,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                  IconButton(
                    key: ValueKey('extension-toggle:${spec['id']}'),
                    tooltip: window.minimized
                        ? (stringsOf(context).mobileExpand71acc101())
                        : (stringsOf(context).mobileMinimize4f64bec2()),
                    onPressed: () => controller.toggle(window),
                    icon: Icon(
                      window.minimized ? Icons.open_in_full : Icons.minimize,
                      size: 20,
                    ),
                  ),
                  IconButton(
                    key: ValueKey('extension-close:${spec['id']}'),
                    tooltip: (stringsOf(context).mobileClose94058aa6()),
                    onPressed: () => unawaited(controller.closeWindow(window)),
                    icon: const Icon(Icons.close, size: 20),
                  ),
                ],
              ),
            ),
            // Keep the same native View mounted while collapsed; no reload or re-init.
            Offstage(
              offstage: window.minimized,
              child: SizedBox(
                width: width,
                height: height,
                child: RemoteMcpOfflineView(
                  key: ObjectKey(window),
                  digest: spec['digest'] as String,
                  hostDocument: buildMcpAppHostDocument(
                    appHtml: spec['html'] as String,
                    arguments: controller.arguments(window),
                    platform: 'mobile',
                    theme: Theme.of(context).brightness.name,
                    serverAvailable: true,
                    nativeSend:
                        'window.McpHost.postMessage(JSON.stringify(message))',
                  ),
                  onCreated: (view) {
                    window.view = view;
                    view.onRequest = window.session.dispatch;
                  },
                  onClosed: (_) {
                    window.view = null;
                  },
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
