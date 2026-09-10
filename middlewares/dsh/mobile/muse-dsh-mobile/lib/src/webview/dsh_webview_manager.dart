import 'dart:async';

import 'package:muse_dsh_mobile/src/capabilities/dsh_file_chooser_host.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_broker.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_codec.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_error_codes.dart';
import 'package:muse_dsh_mobile/src/dsh_remote_config.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_navigation_policy.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_webview_session.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_webview_storage.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_mobile_surface_inject.dart';

class DshWebViewManager {
  DshWebViewManager({
    required this.config,
    required this.session,
    required this.isLive,
    required this.onDocumentReadyChanged,
    required this.onFatal,
    this.storage = const DshWebViewStorage(),
    this.capabilityBroker,
    this.fileChooserHost,
  }) : policy = DshNavigationPolicy(config);

  static const nativeCapabilitiesEnabled = bool.fromEnvironment(
    'MUSE_DSH_NATIVE_CAPABILITIES_ENABLED',
    defaultValue: true,
  );

  static const mobileSurfaceEnabled = bool.fromEnvironment(
    'MUSE_DSH_MOBILE_SURFACE_ENABLED',
    defaultValue: true,
  );

  final DshRemoteConfig config;
  final DshWebViewSession session;
  final bool Function() isLive;
  final void Function(bool ready) onDocumentReadyChanged;
  final void Function(String message) onFatal;
  final DshWebViewStorage storage;
  final DshNativeCapabilityBroker? capabilityBroker;
  final DshFileChooserHost? fileChooserHost;
  final DshNavigationPolicy policy;

  WebViewController? controller;
  DateTime? createdAt;
  DateTime? documentReadyAt;

  Future<WebViewController> createAndLoad() async {
    session.enter(DshWebViewPhase.creating);
    createdAt = DateTime.now();
    final webView = WebViewController();
    await webView.setJavaScriptMode(JavaScriptMode.unrestricted);
    await webView.setNavigationDelegate(
      NavigationDelegate(
        onNavigationRequest: (request) => policy.allows(Uri.parse(request.url))
            ? NavigationDecision.navigate
            : NavigationDecision.prevent,
        onPageStarted: (_) {
          if (isLive()) {
            unawaited(capabilityBroker?.cancelSpeech());
            onDocumentReadyChanged(false);
            unawaited(_markMobileSurface(webView));
          }
        },
        onPageFinished: (url) {
          if (isLive() && policy.allows(Uri.parse(url))) {
            session.enter(DshWebViewPhase.documentReady);
            documentReadyAt = DateTime.now();
            onDocumentReadyChanged(true);
            unawaited(_markMobileSurface(webView));
          }
        },
        onSslAuthError: (error) {
          unawaited(error.cancel());
          if (isLive()) {
            session.enter(DshWebViewPhase.fatalError);
            onFatal(DshMobileErrorCode.tlsUntrusted.message);
          }
        },
        onWebResourceError: (error) {
          if (error.isForMainFrame != false && isLive()) {
            session.enter(DshWebViewPhase.fatalError);
            onFatal(DshMobileErrorCode.documentLoadFailed.message);
          }
        },
      ),
    );
    if (nativeCapabilitiesEnabled && capabilityBroker != null) {
      await capabilityBroker!.installChannel(webView);
    }
    await _installAndroidFileChooser(webView);
    controller = webView;
    session.enter(DshWebViewPhase.loadingDocument);
    await webView.loadRequest(config.publicUri);
    return webView;
  }

  Future<void> dispose() async {
    session.enter(DshWebViewPhase.disposing);
    await capabilityBroker?.dispose();
    controller = null;
    await storage.clearCookies();
  }

  Future<void> _markMobileSurface(WebViewController webView) async {
    if (!mobileSurfaceEnabled) return;
    await webView.runJavaScript(
      'document.documentElement.setAttribute("data-muse-surface","mobile");',
    );
    await webView.runJavaScript(dshMobileSurfaceRepairScript);
  }

  Future<void> _installAndroidFileChooser(WebViewController webView) async {
    final host = fileChooserHost;
    final platform = webView.platform;
    if (host == null || platform is! AndroidWebViewController) return;
    await platform.setOnShowFileSelector((params) async {
      if (!isLive()) return <String>[];
      final mode = switch (params.mode) {
        FileSelectorMode.open => DshFileChooserMode.open,
        FileSelectorMode.openMultiple => DshFileChooserMode.openMultiple,
        FileSelectorMode.save => DshFileChooserMode.save,
      };
      final uris = await host.chooseFiles(
        DshFileChooserRequest(
          isCaptureEnabled: params.isCaptureEnabled,
          acceptTypes: params.acceptTypes,
          mode: mode,
          filenameHint: params.filenameHint,
        ),
      );
      return uris
          .where(_isSafeChooserUri)
          .toList(growable: false);
    });
  }

  static bool _isSafeChooserUri(String value) {
    final uri = Uri.tryParse(value);
    if (uri == null) return false;
    return uri.scheme == 'content' || uri.scheme == 'file';
  }
}

/// Channel name exported for tests and host docs.
const dshNativeJavascriptChannel = DshNativeCapabilityCodec.javascriptChannelName;
