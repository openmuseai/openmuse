import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'workbench_page.dart';

AcceptanceDesktop? debugAcceptanceDesktop() {
  if (kReleaseMode) {
    const enabled = bool.fromEnvironment('OPENMUSE_REMOTE_SURFACE_ACCEPTANCE');
    if (!enabled) return null;
  }
  return AcceptanceDesktop();
}

final class OpenMuseRemoteWorkbenchPlugin implements OpenMusePlugin {
  OpenMuseRemoteWorkbenchPlugin({
    RemoteSurfaceTransport? transport,
    RemoteConnectionContext context = const RemoteConnectionContext.unbound(),
    RemoteClientHello? hello,
  }) : _transport = transport, // ignore: prefer_initializing_formals
       _context = context, // ignore: prefer_initializing_formals
       _hello = hello; // ignore: prefer_initializing_formals

  final RemoteSurfaceTransport? _transport;
  final RemoteConnectionContext _context;
  final RemoteClientHello? _hello;
  RemoteWorkbenchController? _controller;

  RemoteWorkbenchController? get controller => _controller;

  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'com.openmuse.remote-workbench',
    name: 'OpenMuse Remote Workbench',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'remote-surface.discover', 'remote-surface.render'},
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    final transport = _transport;
    if (transport == null) return;
    _controller = RemoteWorkbenchController(
      transport: transport,
      context: _context,
      hello: _hello,
    );
  }

  @override
  Future<void> deactivate() async {
    _controller?.dispose();
    _controller = null;
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    return const Center(child: Text('远程工作台不编辑本地文件'));
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) {
    if (panelId != 'remote-workbench') return null;
    final controller = _controller;
    if (controller == null) return const RemoteWorkbenchUnavailable();
    return RemoteWorkbenchPage(controller: controller);
  }
}
