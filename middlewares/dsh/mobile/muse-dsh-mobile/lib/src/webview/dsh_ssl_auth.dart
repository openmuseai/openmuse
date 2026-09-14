import 'dart:io';

import 'package:webview_flutter_platform_interface/webview_flutter_platform_interface.dart';

/// Probe TLS with dart:io (Android system CAs), not Chromium WebView.
/// OEM WebViews (ColorOS etc.) often fire [onSslAuthError] for DigiCert/ISRG
/// chains that the platform HttpClient already accepts. Proceed only then.
Future<bool> dshSystemTrustHandshake(Uri uri) async {
  if (uri.scheme != 'https' || uri.host.isEmpty || uri.userInfo.isNotEmpty) {
    return false;
  }
  final probe = Uri(
    scheme: 'https',
    host: uri.host,
    port: uri.hasPort ? uri.port : 443,
    path: '/',
  );
  final client = HttpClient();
  client.badCertificateCallback = (cert, host, port) => false;
  client.connectionTimeout = const Duration(seconds: 8);
  try {
    final request = await client.getUrl(probe);
    request.followRedirects = false;
    final response = await request.close().timeout(const Duration(seconds: 8));
    await response.drain<void>();
    return true;
  } on HandshakeException {
    return false;
  } on TlsException {
    return false;
  } on CertificateException {
    return false;
  } on SocketException {
    return false;
  } catch (_) {
    return false;
  } finally {
    client.close(force: true);
  }
}

Future<void> resolveDshSslAuthError({
  required PlatformSslAuthError error,
  required Uri requestUrl,
  required bool Function(Uri uri) allows,
  required Future<bool> Function(Uri uri) handshake,
  required void Function() onUntrusted,
}) async {
  if (!allows(requestUrl) || !await handshake(requestUrl)) {
    await error.cancel();
    onUntrusted();
    return;
  }
  await error.proceed();
}

/// Parent-bridge lives under the tenant prefix, same as the Web inject script.
Uri dshParentBridgeUri(Uri page) {
  final stripped = page.replace(query: '', fragment: '');
  final path = stripped.path.endsWith('/') ? stripped.path : '${stripped.path}/';
  return stripped.replace(path: path).resolve('muse/v1/parent-bridge');
}
