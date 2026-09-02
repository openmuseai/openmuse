import 'dart:async';

import 'package:flutter/material.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_file_chooser_host.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_mobile_back_dispatcher.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_host.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_control_host.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_coordinator.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_surface_state.dart';
import 'package:muse_dsh_mobile/src/dsh_remote_config.dart';
import 'package:webview_flutter/webview_flutter.dart';

/// Host-agnostic DSH shell. The embedding app supplies [controlHost] and
/// optional native capability / file-chooser ports.
class DshMobileShellPage extends StatefulWidget {
  const DshMobileShellPage({
    super.key,
    required this.scope,
    this.controlHost,
    this.capabilityHost,
    this.fileChooserHost,
    this.endpoint,
    this.title = 'DeepSeek Agent',
  });

  final DshMobileScope scope;
  final DshMobileControlHost? controlHost;
  final DshNativeCapabilityHost? capabilityHost;
  final DshFileChooserHost? fileChooserHost;
  final DshRemoteConfig? endpoint;
  final String title;

  @override
  State<DshMobileShellPage> createState() => _DshMobileShellPageState();
}

class _DshMobileShellPageState extends State<DshMobileShellPage> {
  late final DshMobileCoordinator _coordinator;

  @override
  void initState() {
    super.initState();
    _coordinator = DshMobileCoordinator(
      scope: widget.scope,
      controlHost: widget.controlHost,
      capabilityHost: widget.capabilityHost,
      fileChooserHost: widget.fileChooserHost,
      endpoint: widget.endpoint,
      notify: () {
        if (mounted) setState(() {});
      },
    );
    _coordinator.start();
  }

  @override
  void dispose() {
    unawaited(_coordinator.dispose());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final surface = _coordinator.surface;
    final showingWebView = surface.showWebView;
    return PopScope(
      canPop: false,
      onPopInvokedWithResult: (didPop, _) async {
        if (didPop) return;
        await DshMobileBackDispatcher(_coordinator.capabilityBroker)
            .handle(context);
      },
      child: Scaffold(
        appBar: AppBar(
          title: Text(widget.title),
          actions: [
            IconButton(
              onPressed: surface.loading ? null : _coordinator.connect,
              tooltip: '重新连接',
              icon: const Icon(Icons.refresh),
            ),
          ],
        ),
        body: showingWebView
            ? Column(
                children: [
                  Padding(
                    padding: const EdgeInsets.all(8),
                    child: Text(
                      '${surface.statusBanner}\n${surface.documentBanner}',
                      style: const TextStyle(fontSize: 12),
                    ),
                  ),
                  Expanded(
                    child: WebViewWidget(controller: surface.controller!),
                  ),
                ],
              )
            : SafeArea(
                child: surface.loading
                    ? const Center(child: CircularProgressIndicator.adaptive())
                    : Center(
                        child: Padding(
                          padding: const EdgeInsets.all(24),
                          child: Text(
                            surface.fatalMessage ?? '',
                            textAlign: TextAlign.center,
                          ),
                        ),
                      ),
              ),
      ),
    );
  }
}
