import 'package:flutter/foundation.dart';

@immutable
class DshRemoteConfig {
  const DshRemoteConfig._(this.publicUri);

  static const _configuredUrl = String.fromEnvironment('MUSE_DSH_PUBLIC_URL');

  final Uri publicUri;

  String get origin => publicUri.origin;

  static DshRemoteConfig? fromEnvironment() => tryParse(_configuredUrl);

  /// Compile-time public origin. Query strings are rejected so launch tokens
  /// cannot be baked into the binary.
  ///
  /// This is an **origin allowlist**, not the WebView page URL. Production
  /// loads `DshRemoteConfig.fromWebUrl` from `session/open`. Do not call
  /// `loadRequest(fromEnvironment().publicUri)` when the path is `/dsh/`.
  @visibleForTesting
  static DshRemoteConfig? tryParse(String raw) {
    final uri = Uri.tryParse(raw.trim());
    if (uri == null ||
        uri.scheme != 'https' ||
        uri.host.isEmpty ||
        uri.hasFragment ||
        uri.hasQuery ||
        uri.userInfo.isNotEmpty) {
      return null;
    }
    return DshRemoteConfig._(
      uri.replace(path: uri.path.isEmpty ? '/' : uri.path),
    );
  }

  /// Runtime URL from `session/open` (`/dsh/` or `/u/<hash>/?token=`).
  static DshRemoteConfig? fromWebUrl(String raw) {
    final uri = Uri.tryParse(raw.trim());
    if (uri == null ||
        uri.scheme != 'https' ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty) {
      return null;
    }
    return DshRemoteConfig._(uri);
  }

  bool allows(Uri uri) =>
      uri.scheme == 'https' &&
      uri.host == publicUri.host &&
      uri.port == publicUri.port &&
      uri.userInfo.isEmpty;
}
