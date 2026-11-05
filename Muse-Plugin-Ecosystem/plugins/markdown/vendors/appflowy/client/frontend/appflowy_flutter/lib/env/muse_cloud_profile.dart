/// Public deployment endpoints, independent of Flutter, credentials and .env.
/// Shared by startup and the Android build-time validator.
final class MuseCloudProfile {
  MuseCloudProfile.fromMap(Map<String, String> values)
      : cloud = _origin(values['MUSE_CLOUD_URL'] ?? ''),
        web = _origin(values['MUSE_CLOUD_WEB_URL'] ?? '') {
    gotrue = _endpoint(values['MUSE_CLOUD_GOTRUE_URL'] ?? '', 'https');
    websocket = _endpoint(values['MUSE_CLOUD_WS_URL'] ?? '', 'wss');
    if (gotrue.origin != cloud.origin ||
        gotrue.path != '/gotrue' ||
        websocket.host != cloud.host ||
        (websocket.hasPort ? websocket.port : 443) != cloud.port ||
        websocket.path != '/ws/v1') {
      throw const FormatException('MOBILE_CLOUD_ENDPOINT_MISMATCH');
    }
  }

  final Uri cloud;
  final Uri web;
  late final Uri gotrue;
  late final Uri websocket;

  static const _values = {
    'MUSE_CLOUD_URL': String.fromEnvironment('MUSE_CLOUD_URL'),
    'MUSE_CLOUD_GOTRUE_URL': String.fromEnvironment('MUSE_CLOUD_GOTRUE_URL'),
    'MUSE_CLOUD_WS_URL': String.fromEnvironment('MUSE_CLOUD_WS_URL'),
    'MUSE_CLOUD_WEB_URL': String.fromEnvironment('MUSE_CLOUD_WEB_URL'),
  };

  static bool get isConfigured =>
      _values.values.any((value) => value.isNotEmpty);

  static MuseCloudProfile? fromEnvironment() =>
      isConfigured ? MuseCloudProfile.fromMap(_values) : null;

  static Uri _origin(String value) {
    final uri = _endpoint(value, 'https');
    if (uri.path.isNotEmpty && uri.path != '/') {
      throw const FormatException('MOBILE_CLOUD_ORIGIN_REQUIRED');
    }
    return Uri.parse(uri.origin);
  }

  static Uri _endpoint(String value, String scheme) {
    final uri = Uri.tryParse(value);
    if (uri == null ||
        value != value.trim() ||
        uri.scheme != scheme ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty ||
        uri.hasQuery ||
        uri.hasFragment ||
        uri.host.toLowerCase() == 'localhost' ||
        uri.host.toLowerCase().endsWith('.localhost') ||
        uri.host.startsWith('127.') ||
        uri.host == '0.0.0.0' ||
        uri.host == '::1' ||
        uri.host == '[::1]') {
      throw const FormatException('MOBILE_CLOUD_HTTPS_ENDPOINT_REQUIRED');
    }
    return uri;
  }
}
