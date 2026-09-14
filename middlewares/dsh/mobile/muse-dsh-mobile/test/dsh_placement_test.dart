import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  const tenant =
      'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t';
  final allowlist = Uri.parse('https://openmuseai.com/dsh/');

  test('E2-T1 accepts a tenant webUrl on the allowlist host', () {
    final decision = DshPlacement.fromOpen(
      accessToken: 'jwt',
      session: DshSessionOpen.fromJson({
        'sessionRef': 'sess-1',
        'webUrl': tenant,
      }),
      allowlist: allowlist,
    );
    expect(decision.isReady, isTrue);
    expect(decision.webUrl, tenant);
    expect(DshPlacement.isTenantInstancePath(Uri.parse(tenant)), isTrue);
  });

  test('E2-T2 treats queued open as waiting-room, not a page URL', () {
    final decision = DshPlacement.fromOpen(
      accessToken: 'jwt',
      session: DshSessionOpen.fromJson({
        'sessionRef': 'sess-q',
        'queuePosition': 2,
        'retryAfterMs': 30,
      }),
      allowlist: allowlist,
    );
    expect(decision.isQueued, isTrue);
    expect(decision.queuePosition, 2);
    expect(decision.webUrl, isNull);
  });

  test('E2-T3 does not place without an access token', () {
    final decision = DshPlacement.fromOpen(
      accessToken: null,
      session: DshSessionOpen.fromJson({
        'sessionRef': 'sess-1',
        'webUrl': tenant,
      }),
      allowlist: allowlist,
    );
    expect(decision.errorCode, 'NEED_AUTH');
  });

  test('E2-T4 rejects a webUrl host outside the allowlist', () {
    final decision = DshPlacement.fromOpen(
      accessToken: 'jwt',
      session: DshSessionOpen.fromJson({
        'sessionRef': 'sess-1',
        'webUrl':
            'https://evil.example/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t',
      }),
      allowlist: allowlist,
    );
    expect(decision.errorCode, 'DSH_CONFIG_INVALID');
  });

  test('E2-T5 rejects production /dsh/ as a session page URL', () {
    expect(
      DshPlacement.rejectSessionWebUrl(
        'https://openmuseai.com/dsh/',
        allowlist: allowlist,
      ),
      'DSH_CONFIG_INVALID',
    );
    expect(
      DshPlacement.fromOpen(
        accessToken: 'jwt',
        session: DshSessionOpen.fromJson({
          'sessionRef': 'sess-1',
          'webUrl': 'https://openmuseai.com/dsh/?token=t',
        }),
        allowlist: allowlist,
      ).errorCode,
      'DSH_CONFIG_INVALID',
    );
  });

  test('E2-T13 retired dsh. dart-define does not block Cloud /u/ webUrl', () {
    final allow = DshPlacement.pageAllowlist(
      cloudOrigin: Uri.parse('https://openmuseai.com'),
      compiled: Uri.parse('https://dsh.openmuseai.com/'),
    );
    expect(allow.host, 'openmuseai.com');
    final decision = DshPlacement.fromOpen(
      accessToken: 'jwt',
      session: DshSessionOpen.fromJson({
        'sessionRef': 'sess-1',
        'webUrl': tenant,
      }),
      allowlist: allow,
    );
    expect(decision.isReady, isTrue);
  });
}
