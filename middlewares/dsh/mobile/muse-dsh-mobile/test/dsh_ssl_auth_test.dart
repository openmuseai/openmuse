import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:webview_flutter_platform_interface/webview_flutter_platform_interface.dart';

void main() {
  test('parent-bridge URI stays under /u/<hash>/ like Web inject', () {
    expect(
      dshParentBridgeUri(
        Uri.parse(
          'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=secret',
        ),
      ).toString(),
      'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/muse/v1/parent-bridge',
    );
  });

  test('SSL proceed only when host is allowed and system TLS handshake succeeds',
      () async {
    final error = _SslError();
    await resolveDshSslAuthError(
      error: error,
      requestUrl: Uri.parse('https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/'),
      allows: (uri) => uri.host == 'openmuseai.com',
      handshake: (_) async => true,
      onUntrusted: () => fail('should not fail'),
    );
    expect(error.proceeded, isTrue);
    expect(error.cancelled, isFalse);
  });

  test('SSL cancel when handshake fails or host is not the session origin',
      () async {
    final untrusted = _SslError();
    var fatal = 0;
    await resolveDshSslAuthError(
      error: untrusted,
      requestUrl: Uri.parse('https://openmuseai.com/'),
      allows: (uri) => uri.host == 'openmuseai.com',
      handshake: (_) async => false,
      onUntrusted: () => fatal += 1,
    );
    expect(untrusted.cancelled, isTrue);
    expect(untrusted.proceeded, isFalse);

    final foreign = _SslError();
    await resolveDshSslAuthError(
      error: foreign,
      requestUrl: Uri.parse('https://dsh.openmuseai.com/'),
      allows: (uri) => uri.host == 'openmuseai.com',
      handshake: (_) async => true,
      onUntrusted: () => fatal += 1,
    );
    expect(foreign.cancelled, isTrue);
    expect(foreign.proceeded, isFalse);
    expect(fatal, 2);
  });
}

class _SslError extends PlatformSslAuthError {
  _SslError() : super(certificate: null, description: 'Untrusted certificate');
  var proceeded = false;
  var cancelled = false;
  @override
  Future<void> proceed() async => proceeded = true;
  @override
  Future<void> cancel() async => cancelled = true;
}
