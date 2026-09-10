import 'package:muse_dsh_mobile/src/dsh_session_api.dart';

/// Placement rules for the Android remote shell (E2).
///
/// Compile-time [MUSE_DSH_PUBLIC_URL] is an origin allowlist, not a page URL.
/// Production iframe/WebView src must be `session/open` `webUrl` under `/u/<hex>/`.
class DshPlacement {
  DshPlacement._();

  static final _tenantPath = RegExp(r'^/u/[0-9a-fA-F]{32}/?$');

  static bool isSharedDshPath(Uri uri) {
    final path = uri.path.replaceAll(RegExp(r'/+$'), '');
    return path == '/dsh';
  }

  static bool isTenantInstancePath(Uri uri) {
    return _tenantPath.hasMatch(uri.path);
  }

  static bool hostAllowed(Uri candidate, Uri allowlist) {
    if (candidate.scheme != 'https' || allowlist.scheme != 'https') {
      return false;
    }
    if (candidate.host != allowlist.host) return false;
    final candidatePort = candidate.hasPort ? candidate.port : 443;
    final allowlistPort = allowlist.hasPort ? allowlist.port : 443;
    return candidatePort == allowlistPort;
  }

  /// Reject a `session/open` webUrl. Null means the URL may be loaded.
  static String? rejectSessionWebUrl(String? raw, {Uri? allowlist}) {
    if (raw == null || raw.trim().isEmpty) return 'DSH_CONFIG_INVALID';
    final uri = Uri.tryParse(raw.trim());
    if (uri == null || uri.scheme != 'https' || uri.host.isEmpty) {
      return 'DSH_CONFIG_INVALID';
    }
    if (isSharedDshPath(uri)) return 'DSH_CONFIG_INVALID';
    if (!isTenantInstancePath(uri)) return 'DSH_CONFIG_INVALID';
    if (allowlist != null && !hostAllowed(uri, allowlist)) {
      return 'DSH_CONFIG_INVALID';
    }
    return null;
  }

  static DshPlacementDecision fromOpen({
    required String? accessToken,
    required DshSessionOpen? session,
    Uri? allowlist,
  }) {
    if (accessToken == null || accessToken.trim().isEmpty) {
      return const DshPlacementDecision.failed('NEED_AUTH');
    }
    if (session == null) {
      return const DshPlacementDecision.failed('DSH_CONFIG_INVALID');
    }
    if (session.isQueued) {
      return DshPlacementDecision.queued(
        sessionRef: session.sessionRef,
        queuePosition: session.queuePosition,
        retryAfterMs: session.retryAfterMs,
      );
    }
    final rejected = rejectSessionWebUrl(session.webUrl, allowlist: allowlist);
    if (rejected != null) {
      return DshPlacementDecision.failed(rejected);
    }
    return DshPlacementDecision.ready(
      sessionRef: session.sessionRef,
      webUrl: session.webUrl!,
    );
  }
}

class DshPlacementDecision {
  const DshPlacementDecision._({
    required this.kind,
    this.errorCode,
    this.sessionRef,
    this.webUrl,
    this.queuePosition,
    this.retryAfterMs,
  });

  const DshPlacementDecision.failed(String code)
      : this._(kind: DshPlacementKind.failed, errorCode: code);

  const DshPlacementDecision.queued({
    required String sessionRef,
    int? queuePosition,
    int? retryAfterMs,
  }) : this._(
          kind: DshPlacementKind.queued,
          sessionRef: sessionRef,
          queuePosition: queuePosition,
          retryAfterMs: retryAfterMs,
        );

  const DshPlacementDecision.ready({
    required String sessionRef,
    required String webUrl,
  }) : this._(
          kind: DshPlacementKind.ready,
          sessionRef: sessionRef,
          webUrl: webUrl,
        );

  final DshPlacementKind kind;
  final String? errorCode;
  final String? sessionRef;
  final String? webUrl;
  final int? queuePosition;
  final int? retryAfterMs;

  bool get isReady => kind == DshPlacementKind.ready;
  bool get isQueued => kind == DshPlacementKind.queued;
  bool get isFailed => kind == DshPlacementKind.failed;
}

enum DshPlacementKind { ready, queued, failed }
