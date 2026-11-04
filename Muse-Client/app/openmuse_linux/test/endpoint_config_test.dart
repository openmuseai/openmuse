import 'dart:io';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_linux/src/host/endpoint_config.dart';

void main() {
  test('both US installation inputs enable the public desktop relay', () async {
    const root = '../../packages/openmuse_runtime_config/assets';
    final standalone = DeploymentBundle.parse(
      await File('$root/deployments/global-us.json').readAsString(),
    );
    final overseas = InstalledRelease.parse(
      await File('$root/releases/overseas.json').readAsString(),
    );
    final bundled = overseas.regions
        .singleWhere((region) => region.id == 'us')
        .bundle!;
    for (final bundle in [standalone, bundled]) {
      final endpoints = HostEndpointConfig.resolve(config: bundle.snapshot);
      expect(endpoints.relayPublicOrigin, 'https://us.openmuseai.com');
      expect(endpoints.cloudOrigin.host, 'us.openmuseai.com');
      expect(endpoints.allowInsecureLoopback, isFalse);
    }
  });

  late RuntimeConfig config;
  setUpAll(() async {
    config = DeploymentBundle.parse(
      await File(
        '../../packages/openmuse_runtime_config/assets/deployments/default.json',
      ).readAsString(),
    ).snapshot;
  });
  test('the shipped endpoint contract matches the deployment document', () {
    expect(
      config.serviceUri(ServiceId.auth).toString(),
      'https://openmuseai.com/gotrue',
    );
    expect(
      config.serviceUri(ServiceId.cloud).toString(),
      'https://openmuseai.com',
    );
    expect(
      config.serviceUri(ServiceId.relay).toString(),
      'https://openmuseai.com:8443',
    );
    // No shipped default may point at loopback: a local backend is opt-in.
    for (final origin in [
      config.serviceUri(ServiceId.auth).toString(),
      config.serviceUri(ServiceId.cloud).toString(),
      config.serviceUri(ServiceId.relay).toString(),
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
      final endpoints = HostEndpointConfig.resolve(config: config);

      expect(
        endpoints.gotrueOrigin.toString(),
        config.serviceUri(ServiceId.auth).toString(),
      );
      expect(
        endpoints.cloudOrigin.toString(),
        config.serviceUri(ServiceId.cloud).toString(),
      );
      expect(endpoints.gotrueOrigin.isScheme('https'), isTrue);
      expect(
        endpoints.cloudOrigin.host,
        config.serviceUri(ServiceId.cloud).host,
      );
      // Loopback permission and the relay are both opt-in, never implied by a
      // build mode.
      expect(endpoints.allowInsecureLoopback, isFalse);
      expect(
        endpoints.relayPublicOrigin,
        config.serviceUri(ServiceId.relay).toString(),
      );
      expect(endpoints.pairedDesktopPort, 13180);
      expect(endpoints.fixedPairedDesktopPairingCode, isNull);
      expect(endpoints.authKeychainAccount, 'flutter_secure_storage_service');
    },
  );

  test(
    'another installed deployment does not inherit the public endpoints',
    () async {
      final alternate = DeploymentBundle.parse(
        await File(
          '../../packages/openmuse_runtime_config/assets/deployments/global-us.json',
        ).readAsString(),
      ).snapshot;
      final endpoints = HostEndpointConfig.resolve(config: alternate);
      expect(endpoints.cloudOrigin, alternate.serviceUri(ServiceId.cloud));
      expect(endpoints.gotrueOrigin, alternate.serviceUri(ServiceId.auth));
      expect(endpoints.cloudOrigin, isNot(config.serviceUri(ServiceId.cloud)));
      expect(
        endpoints.relayPublicOrigin,
        alternate.serviceUri(ServiceId.relay).toString(),
      );
      expect(
        alternate.authSessionStorageKey,
        isNot(config.authSessionStorageKey),
      );
    },
  );

  test('a dart-define wins over the process environment', () {
    final endpoints = HostEndpointConfig.resolve(
      config: config,
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
      config: config,
      environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://env.example/'},
    );

    expect(endpoints.cloudOrigin.host, 'env.example');
    // GoTrue keeps its own default when only Cloud is configured.
    expect(
      endpoints.gotrueOrigin.toString(),
      config.serviceUri(ServiceId.auth).toString(),
    );
  });

  test('the relay is inherited only from the production Cloud host', () {
    // An explicitly configured production Cloud still inherits the public relay.
    expect(
      HostEndpointConfig.resolve(
        config: config,
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://openmuseai.com'},
      ).relayPublicOrigin,
      config.serviceUri(ServiceId.relay).toString(),
    );
    // Any other Cloud host must not silently advertise the public relay.
    expect(
      HostEndpointConfig.resolve(
        config: config,
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'https://env.example/'},
      ).relayPublicOrigin,
      isEmpty,
    );
    // An explicit relay origin always wins.
    expect(
      HostEndpointConfig.resolve(
        config: config,
        environment: const {'OPENMUSE_CLOUD_ORIGIN': 'http://127.0.0.1:8000'},
        relayDefine: 'https://relay.example:9443',
      ).relayPublicOrigin,
      'https://relay.example:9443',
    );
  });

  test('loopback permission is off unless configured truthy', () {
    expect(
      HostEndpointConfig.resolve(config: config).allowInsecureLoopback,
      isFalse,
    );
    for (final raw in const ['true', 'TRUE', '1', ' true ']) {
      expect(
        HostEndpointConfig.resolve(
          config: config,
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
          config: config,
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
        config: config,
        pairedDesktopPortDefine: '13200',
      ).pairedDesktopPort,
      13200,
    );
    expect(
      HostEndpointConfig.resolve(
        config: config,
        environment: const {'OPENMUSE_PAIRED_DESKTOP_PORT': '13181'},
      ).pairedDesktopPort,
      13181,
    );
    for (final raw in const ['', 'abc', '0', '65536', '-1']) {
      expect(
        HostEndpointConfig.resolve(
          config: config,
          pairedDesktopPortDefine: raw,
        ).pairedDesktopPort,
        13180,
        reason: 'port "$raw"',
      );
    }
  });

  test('the fixed pairing code is only set when configured', () {
    expect(
      HostEndpointConfig.resolve(config: config).fixedPairedDesktopPairingCode,
      isNull,
    );
    expect(
      HostEndpointConfig.resolve(
        config: config,
        pairedDesktopPairingCodeDefine: '2468',
      ).fixedPairedDesktopPairingCode,
      '2468',
    );
  });

  test('the keychain account falls back to the reviewed default', () {
    expect(
      HostEndpointConfig.resolve(config: config).authKeychainAccount,
      'flutter_secure_storage_service',
    );
    expect(
      HostEndpointConfig.resolve(
        config: config,
        environment: const {'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT': 'openmuse.e2e'},
      ).authKeychainAccount,
      'openmuse.e2e',
    );
    expect(
      HostEndpointConfig.resolve(
        config: config,
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
