import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/endpoint_config.dart';

void main() {
  test('the shipped endpoint contract matches the deployment document', () {
    expect(
      HostEndpointConfig.defaultGoTrueOrigin,
      'https://openmuseai.com/gotrue',
    );
    expect(HostEndpointConfig.defaultCloudOrigin, 'https://openmuseai.com');
    expect(
      HostEndpointConfig.defaultRelayOrigin,
      'https://openmuseai.com:8443',
    );
    // No shipped default may point at loopback: a local backend is opt-in.
    for (final origin in const [
      HostEndpointConfig.defaultGoTrueOrigin,
      HostEndpointConfig.defaultCloudOrigin,
      HostEndpointConfig.defaultRelayOrigin,
    ]) {
      expect(
        HostEndpointConfig.loopbackHosts,
        isNot(contains(Uri.parse(origin).host)),
        reason: origin,
      );
    }
  });

  test(
    'an unconfigured host points every endpoint at the public deployment',
    () {
      final endpoints = HostEndpointConfig.resolve();

      expect(
        endpoints.gotrueOrigin.toString(),
        HostEndpointConfig.defaultGoTrueOrigin,
      );
      expect(
        endpoints.cloudOrigin.toString(),
        HostEndpointConfig.defaultCloudOrigin,
      );
      expect(endpoints.gotrueOrigin.isScheme('https'), isTrue);
      expect(endpoints.cloudOrigin.host, HostEndpointConfig.productionHost);
      // Loopback permission and the relay are both opt-in, never implied by a
      // build mode.
      expect(endpoints.allowInsecureLoopback, isFalse);
      expect(
        endpoints.relayPublicOrigin,
        HostEndpointConfig.defaultRelayOrigin,
      );
      expect(endpoints.pairedDesktopPort, 13180);
      expect(endpoints.fixedPairedDesktopPairingCode, isNull);
      expect(endpoints.authKeychainAccount, 'flutter_secure_storage_service');
    },
  );

  test('a dart-define wins over the process environment', () {
    final endpoints = HostEndpointConfig.resolve(
      environment: const {
        'OPENMUSE_GOTRUE_ORIGIN': 'https://env.example/gotrue',
        'OPENMUSE_CLOUD_ORIGIN': 'https://env.example',
      },
      gotrueDefine: 'https://define.example/gotrue',
      cloudDefine: 'https://define.example',
    );

    expect(endpoints.gotrueOrigin.toString(), 'https://define.example/gotrue');
    expect(endpoints.cloudOrigin.toString(), 'https://define.example');
  });

  test('the process environment overrides the shipped default', () {
    final endpoints = HostEndpointConfig.resolve(
      environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://env.example/'},
    );

    expect(endpoints.cloudOrigin.host, 'env.example');
    // GoTrue keeps its own default when only Cloud is configured.
    expect(
      endpoints.gotrueOrigin.toString(),
      HostEndpointConfig.defaultGoTrueOrigin,
    );
  });

  test('the relay is inherited only from the production Cloud host', () {
    // An explicitly configured production Cloud still inherits the public relay.
    expect(
      HostEndpointConfig.resolve(
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://openmuseai.com'},
      ).relayPublicOrigin,
      HostEndpointConfig.defaultRelayOrigin,
    );
    // Any other Cloud host must not silently advertise the public relay.
    expect(
      HostEndpointConfig.resolve(
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://env.example/'},
      ).relayPublicOrigin,
      isEmpty,
    );
    // An explicit relay origin always wins.
    expect(
      HostEndpointConfig.resolve(
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'http://127.0.0.1:8000'},
        relayDefine: 'https://relay.example:9443',
      ).relayPublicOrigin,
      'https://relay.example:9443',
    );
  });

  test('loopback permission is off unless configured truthy', () {
    expect(HostEndpointConfig.resolve().allowInsecureLoopback, isFalse);
    for (final raw in const ['true', 'TRUE', '1', ' true ']) {
      expect(
        HostEndpointConfig.resolve(
          allowInsecureLoopbackDefine: raw,
        ).allowInsecureLoopback,
        isTrue,
        reason: raw,
      );
    }
    // Falsy and unparsable values alike keep the safe default.
    for (final raw in const ['false', 'FALSE', '0', 'maybe', '']) {
      expect(
        HostEndpointConfig.resolve(
          allowInsecureLoopbackDefine: raw,
        ).allowInsecureLoopback,
        isFalse,
        reason: 'value "$raw"',
      );
    }
  });

  test('the paired-Desktop port falls back when the value is unusable', () {
    expect(
      HostEndpointConfig.resolve(
        pairedDesktopPortDefine: '13200',
      ).pairedDesktopPort,
      13200,
    );
    expect(
      HostEndpointConfig.resolve(
        environment: const {'OPENMUSE_PAIRED_DESKTOP_PORT': '13181'},
      ).pairedDesktopPort,
      13181,
    );
    for (final raw in const ['', 'abc', '0', '65536', '-1']) {
      expect(
        HostEndpointConfig.resolve(
          pairedDesktopPortDefine: raw,
        ).pairedDesktopPort,
        13180,
        reason: 'port "$raw"',
      );
    }
  });

  test('the fixed pairing code is only set when configured', () {
    expect(HostEndpointConfig.resolve().fixedPairedDesktopPairingCode, isNull);
    expect(
      HostEndpointConfig.resolve(
        pairedDesktopPairingCodeDefine: '2468',
      ).fixedPairedDesktopPairingCode,
      '2468',
    );
  });

  test('the keychain account falls back to the reviewed default', () {
    expect(
      HostEndpointConfig.resolve().authKeychainAccount,
      'flutter_secure_storage_service',
    );
    expect(
      HostEndpointConfig.resolve(
        environment: const {'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT': 'openmuse.e2e'},
      ).authKeychainAccount,
      'openmuse.e2e',
    );
    expect(
      HostEndpointConfig.resolve(
        authKeychainAccountDefine: 'openmuse.define',
        environment: const {'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT': 'openmuse.env'},
      ).authKeychainAccount,
      'openmuse.define',
    );
  });

  test('the loopback allow-list never gains a routable host', () {
    // `_allowed` in plugin_package.dart uses this set as a security predicate,
    // and plugin_cli_broker.dart advertises loopbackHostIpv4, so the shipped
    // set must stay exactly the two genuine loopback names.
    expect(HostEndpointConfig.loopbackHosts, const {'127.0.0.1', 'localhost'});
    expect(HostEndpointConfig.loopbackHostIpv4, '127.0.0.1');
  });
}
