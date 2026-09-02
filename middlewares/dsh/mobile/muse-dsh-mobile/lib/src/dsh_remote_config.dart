import 'package:flutter/foundation.dart';

@immutable
class DshRemoteConfig {
  const DshRemoteConfig._(this.publicUri);

  static const _configuredUrl = String.fromEnvironment('MUSE_DSH_PUBLIC_URL');

  final Uri publicUri;

  String get origin => publicUri.origin;

  static DshRemoteConfig? fromEnvironment() => tryParse(_configuredUrl);

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

  bool allows(Uri uri) =>
      uri.scheme == 'https' &&
      uri.host == publicUri.host &&
      uri.port == publicUri.port &&
      uri.userInfo.isEmpty;
}
