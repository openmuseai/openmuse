import 'package:webview_flutter/webview_flutter.dart';

class DshWebViewStorage {
  const DshWebViewStorage();

  Future<void> seedCloudSession({
    required String host,
    String? accessToken,
  }) async {
    final token = accessToken?.trim() ?? '';
    if (token.isEmpty || host.isEmpty) return;
    try {
      await WebViewCookieManager().setCookie(
        WebViewCookie(name: 'access_token', value: token, domain: host, path: '/'),
      );
    } catch (_) {}
  }

  Future<void> clearCookies() async {
    try {
      await WebViewCookieManager().clearCookies();
    } catch (_) {}
  }
}
