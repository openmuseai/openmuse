import 'package:webview_flutter/webview_flutter.dart';

class DshWebViewStorage {
  const DshWebViewStorage();

  Future<void> clearCookies() async {
    try {
      await WebViewCookieManager().clearCookies();
    } catch (_) {}
  }
}
