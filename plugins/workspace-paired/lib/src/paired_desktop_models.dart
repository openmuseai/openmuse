import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

final class PairedDesktopConnection {
  const PairedDesktopConnection({
    required this.accountRef,
    required this.deviceRef,
    required this.workspaceRef,
    required this.workspaceTitle,
    required this.grantRef,
    required this.expiresAtMs,
    required this.session,
  });

  final String accountRef;
  final String deviceRef;
  final String workspaceRef;
  final String workspaceTitle;
  final String grantRef;
  final int expiresAtMs;
  final DshSessionDescriptor session;

  factory PairedDesktopConnection.fromJson(
    Map<String, Object?> json, {
    bool allowInsecurePrivateNetworkForTesting = false,
  }) {
    final accountRef = json['accountRef'];
    final deviceRef = json['deviceRef'];
    final workspaceRef = json['workspaceRef'];
    final workspaceTitle = json['workspaceTitle'];
    final grantRef = json['grantRef'];
    final expiresAtMs = json['expiresAtMs'];
    final session = json['session'];
    if (accountRef is! String ||
        accountRef.isEmpty ||
        deviceRef is! String ||
        deviceRef.isEmpty ||
        workspaceRef is! String ||
        workspaceRef.isEmpty ||
        workspaceTitle is! String ||
        workspaceTitle.isEmpty ||
        grantRef is! String ||
        grantRef.isEmpty ||
        expiresAtMs is! int ||
        session is! Map) {
      throw const FormatException('invalid paired Desktop response');
    }
    final sessionMap = session.cast<String, Object?>();
    final sessionRef = sessionMap['sessionRef'];
    final origin = sessionMap['origin'];
    final path = sessionMap['path'];
    final generation = sessionMap['generation'];
    if (sessionRef is! String ||
        sessionRef.isEmpty ||
        origin is! String ||
        path is! String ||
        generation is! int) {
      throw const FormatException('invalid paired DSH descriptor');
    }
    return PairedDesktopConnection(
      accountRef: accountRef,
      deviceRef: deviceRef,
      workspaceRef: workspaceRef,
      workspaceTitle: workspaceTitle,
      grantRef: grantRef,
      expiresAtMs: expiresAtMs,
      session: DshSessionDescriptor(
        sessionRef: sessionRef,
        origin: origin,
        path: path,
        generation: generation,
        allowInsecureLoopback: sessionMap['allowInsecureLoopback'] == true,
        allowInsecurePrivateNetworkForTesting:
            allowInsecurePrivateNetworkForTesting,
      ),
    );
  }
}

final class PairedDesktopFailure implements Exception {
  const PairedDesktopFailure(this.code, this.safeMessage);
  final String code;
  final String safeMessage;

  @override
  String toString() => safeMessage;
}
