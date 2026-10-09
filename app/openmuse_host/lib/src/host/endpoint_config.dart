import 'dart:io';

import 'package:flutter/foundation.dart';

/// Compile-time endpoint overrides. `String.fromEnvironment` only reads values
/// passed through `--dart-define`, so these stay empty unless a build sets them.
const _gotrueDefine = String.fromEnvironment('OPENMUSE_GOTRUE_ORIGIN');
const _cloudDefine = String.fromEnvironment('OPENMUSE_CLOUD_ORIGIN');
const _relayDefine = String.fromEnvironment('OPENMUSE_RELAY_PUBLIC_ORIGIN');
const _allowInsecureLoopbackDefine = String.fromEnvironment(
  'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
);
const _authKeychainAccountDefine = String.fromEnvironment(
  'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT',
);
const _pairedDesktopPortDefine = String.fromEnvironment(
  'OPENMUSE_PAIRED_DESKTOP_PORT',
);
const _pairedDesktopPairingCodeDefine = String.fromEnvironment(
  'OPENMUSE_PAIRED_DESKTOP_PAIRING_CODE',
);

/// Single source of truth for every host-related endpoint on the desktop host:
/// the GoTrue origin, the Cloud origin, the public outbound-relay origin, the
/// paired-Desktop gateway port and the auth keychain account.
///
/// Every value resolves the same way — compile-time dart-define, then the
/// process environment, then the one default declared here. Nothing depends on
/// debug or release mode: a build that must reach another deployment passes
/// configuration instead of inheriting a build-mode default. Callers read this
/// config rather than embedding hosts, ports or loopback addresses.
@immutable
final class HostEndpointConfig {
  /// Host of the public deployment, used to tell whether Cloud points at it.
  static const productionHost = 'openmuseai.com';

  /// Public GoTrue origin.
  static const defaultGoTrueOrigin = 'https://openmuseai.com/gotrue';

  /// Public Cloud origin.
  static const defaultCloudOrigin = 'https://openmuseai.com';

  /// Public outbound-relay origin. Inherited only while Cloud points at
  /// [productionHost]; every other deployment opts in explicitly.
  static const defaultRelayOrigin = 'https://openmuseai.com:8443';

  /// IPv4 loopback host advertised by the local CLI broker.
  static const loopbackHostIpv4 = '127.0.0.1';

  /// Hosts that count as loopback. Only genuine loopback names belong here: the
  /// plugin-package allow-list uses this set as a security predicate.
  static const loopbackHosts = <String>{loopbackHostIpv4, 'localhost'};

  /// Port the paired-Desktop gateway listens on.
  static const defaultPairedDesktopPort = 13180;

  /// Secure-storage account that holds the auth session.
  static const defaultAuthKeychainAccount = 'flutter_secure_storage_service';

  const HostEndpointConfig({
    required this.gotrueOrigin,
    required this.cloudOrigin,
    required this.allowInsecureLoopback,
    required this.relayPublicOrigin,
    required this.pairedDesktopPort,
    required this.fixedPairedDesktopPairingCode,
    required this.authKeychainAccount,
  });

  /// Resolves the configuration for the running host.
  factory HostEndpointConfig.fromEnvironment() => HostEndpointConfig.resolve(
    environment: Platform.environment,
    gotrueDefine: _gotrueDefine,
    cloudDefine: _cloudDefine,
    relayDefine: _relayDefine,
    allowInsecureLoopbackDefine: _allowInsecureLoopbackDefine,
    authKeychainAccountDefine: _authKeychainAccountDefine,
    pairedDesktopPortDefine: _pairedDesktopPortDefine,
    pairedDesktopPairingCodeDefine: _pairedDesktopPairingCodeDefine,
  );

  /// Resolution seam with explicit inputs, so precedence stays testable without
  /// mutating the real process environment.
  @visibleForTesting
  factory HostEndpointConfig.resolve({
    Map<String, String> environment = const <String, String>{},
    String gotrueDefine = '',
    String cloudDefine = '',
    String relayDefine = '',
    String allowInsecureLoopbackDefine = '',
    String authKeychainAccountDefine = '',
    String pairedDesktopPortDefine = '',
    String pairedDesktopPairingCodeDefine = '',
  }) {
    final cloudOrigin = Uri.parse(
      _endpoint(
        key: 'OPENMUSE_CLOUD_ORIGIN',
        dartDefine: cloudDefine,
        environment: environment,
        fallback: defaultCloudOrigin,
      ),
    );
    return HostEndpointConfig(
      gotrueOrigin: Uri.parse(
        _endpoint(
          key: 'OPENMUSE_GOTRUE_ORIGIN',
          dartDefine: gotrueDefine,
          environment: environment,
          fallback: defaultGoTrueOrigin,
        ),
      ),
      cloudOrigin: cloudOrigin,
      allowInsecureLoopback: _boolean(
        key: 'OPENMUSE_ALLOW_INSECURE_LOOPBACK',
        dartDefine: allowInsecureLoopbackDefine,
        environment: environment,
      ),
      relayPublicOrigin: _endpoint(
        key: 'OPENMUSE_RELAY_PUBLIC_ORIGIN',
        dartDefine: relayDefine,
        environment: environment,
        fallback: cloudOrigin.host == productionHost ? defaultRelayOrigin : '',
      ),
      pairedDesktopPort: _port(
        _configured(
          key: 'OPENMUSE_PAIRED_DESKTOP_PORT',
          dartDefine: pairedDesktopPortDefine,
          environment: environment,
        ),
      ),
      fixedPairedDesktopPairingCode: _orNull(
        _configured(
          key: 'OPENMUSE_PAIRED_DESKTOP_PAIRING_CODE',
          dartDefine: pairedDesktopPairingCodeDefine,
          environment: environment,
        ),
      ),
      authKeychainAccount: _endpoint(
        key: 'OPENMUSE_AUTH_KEYCHAIN_ACCOUNT',
        dartDefine: authKeychainAccountDefine,
        environment: environment,
        fallback: defaultAuthKeychainAccount,
      ),
    );
  }

  /// GoTrue authentication origin.
  final Uri gotrueOrigin;

  /// Cloud API origin, also used for the account device directory.
  final Uri cloudOrigin;

  /// Whether plain-`http` loopback endpoints are accepted.
  final bool allowInsecureLoopback;

  /// Explicit public relay origin, or empty when the host stays local-only.
  final String relayPublicOrigin;

  /// Port the paired-Desktop gateway listens on.
  final int pairedDesktopPort;

  /// Fixed pairing code for acceptance runs, or null when the gateway should
  /// generate one.
  final String? fixedPairedDesktopPairingCode;

  /// Secure-storage account that holds the auth session.
  final String authKeychainAccount;
}

/// Returns the trimmed dart-define when set, otherwise the trimmed environment
/// value, otherwise an empty string.
String _configured({
  required String key,
  required String dartDefine,
  required Map<String, String> environment,
}) {
  final compiled = dartDefine.trim();
  if (compiled.isNotEmpty) return compiled;
  return environment[key]?.trim() ?? '';
}

/// Returns the configured value, otherwise [fallback].
String _endpoint({
  required String key,
  required String dartDefine,
  required Map<String, String> environment,
  required String fallback,
}) {
  final configured = _configured(
    key: key,
    dartDefine: dartDefine,
    environment: environment,
  );
  return configured.isEmpty ? fallback : configured;
}

/// Reads a boolean override. Only `true` and `1` enable it; an unset or
/// unparsable value keeps the safe default of `false`.
bool _boolean({
  required String key,
  required String dartDefine,
  required Map<String, String> environment,
}) {
  final raw = _configured(
    key: key,
    dartDefine: dartDefine,
    environment: environment,
  ).toLowerCase();
  return raw == 'true' || raw == '1';
}

/// Parses a TCP port, falling back to [HostEndpointConfig.defaultPairedDesktopPort]
/// for an empty or out-of-range value.
int _port(String configured) {
  final parsed = int.tryParse(configured);
  if (parsed == null || parsed < 1 || parsed > 65535) {
    return HostEndpointConfig.defaultPairedDesktopPort;
  }
  return parsed;
}

String? _orNull(String value) => value.isEmpty ? null : value;
