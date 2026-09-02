import 'package:flutter/foundation.dart';
import 'package:webview_flutter/webview_flutter.dart';

enum DshMobileSurfaceKind { boot, webView, offline, fatal }

enum DshReadinessFace { document, client, session, facet }

@immutable
class DshMobileSurfaceState {
  const DshMobileSurfaceState({
    required this.generation,
    required this.loading,
    required this.documentReady,
    required this.facetReady,
    this.clientReady = false,
    this.sessionReady = false,
    this.fatalMessage,
    this.bridgeWarning,
    this.controller,
  });

  final int generation;
  final bool loading;
  final bool documentReady;
  final bool clientReady;
  final bool sessionReady;
  final bool facetReady;
  final String? fatalMessage;
  final String? bridgeWarning;
  final WebViewController? controller;

  DshMobileSurfaceKind get kind {
    if (fatalMessage != null) return DshMobileSurfaceKind.fatal;
    if (loading) return DshMobileSurfaceKind.boot;
    if (controller != null) return DshMobileSurfaceKind.webView;
    return DshMobileSurfaceKind.boot;
  }

  bool get showWebView =>
      controller != null && fatalMessage == null && !loading;

  String get statusBanner {
    if (facetReady) {
      return '独占内测 · 工作区联动已连接';
    }
    if (bridgeWarning != null) {
      return '工作区联动未连接 · $bridgeWarning';
    }
    return '正在尝试工作区联动（不影响 DSH 页面加载）';
  }

  String get documentBanner =>
      documentReady ? '页面已加载（非聊天握手确认）' : '页面加载中';

  Map<DshReadinessFace, bool> get readiness => {
        DshReadinessFace.document: documentReady,
        DshReadinessFace.client: clientReady,
        DshReadinessFace.session: sessionReady,
        DshReadinessFace.facet: facetReady,
      };
}

/// Embedding-app session identity. Names are protocol refs, not AppFlowy types.
@immutable
class DshMobileScope {
  const DshMobileScope({
    required this.workspaceRef,
    required this.workspaceTitle,
    required this.accountRef,
    required this.isCloudAccount,
    required this.isCurrentScope,
  });

  final String workspaceRef;
  final String workspaceTitle;
  final String accountRef;
  final bool isCloudAccount;
  final bool Function() isCurrentScope;
}
