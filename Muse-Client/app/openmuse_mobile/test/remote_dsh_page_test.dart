import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/remote_dsh_page.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

void main() {
  test('resolves an HTTPS session URL without exposing another origin', () {
    const session = DshSessionDescriptor(
      sessionRef: 'session-1',
      origin: 'https://dsh.example.com',
      path: '/u/session-1/?token=opaque',
      generation: 1,
    );

    expect(
      resolveRemoteDshUri(session),
      Uri.parse('https://dsh.example.com/u/session-1/?token=opaque'),
    );
  });

  test('permits loopback HTTP only when the descriptor opts in', () {
    const allowed = DshSessionDescriptor(
      sessionRef: 'session-1',
      origin: 'http://127.0.0.1:13080',
      path: '/u/session-1/',
      generation: 1,
      allowInsecureLoopback: true,
    );
    const denied = DshSessionDescriptor(
      sessionRef: 'session-1',
      origin: 'http://127.0.0.1:13080',
      path: '/u/session-1/',
      generation: 1,
    );

    expect(resolveRemoteDshUri(allowed).port, 13080);
    expect(() => resolveRemoteDshUri(denied), throwsFormatException);
  });

  test('blocks cross-origin navigation and path traversal', () {
    final origin = Uri.parse('https://dsh.example.com');
    expect(
      isAllowedRemoteDshNavigation(
        Uri.parse('https://evil.example/u/session-1/'),
        origin,
      ),
      isFalse,
    );
    expect(
      () => resolveRemoteDshUri(
        const DshSessionDescriptor(
          sessionRef: 'session-1',
          origin: 'https://dsh.example.com',
          path: '/u/../admin',
          generation: 1,
        ),
      ),
      throwsFormatException,
    );
  });
}
