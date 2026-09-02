/// Optional host enhancement. Implementations live in the embedding app.
/// Must not inject tokens, file paths, or Cloud cookies into the WebView.
abstract class DshMobileControlHost {
  Future<DshControlSession> connect(DshControlSessionRequest request);
}

abstract class DshControlSession {
  Future<void> close();
}

class DshControlSessionRequest {
  const DshControlSessionRequest({
    required this.endpoint,
    required this.workspaceRef,
    required this.workspaceTitle,
    required this.accountRef,
    required this.isCloudAccount,
    required this.isLive,
    required this.onDisconnected,
    required this.onDegraded,
  });

  final Uri endpoint;
  final String workspaceRef;
  final String workspaceTitle;
  final String accountRef;
  final bool isCloudAccount;
  final bool Function() isLive;
  final void Function() onDisconnected;
  final void Function(String message) onDegraded;
}

class DshControlConnectException implements Exception {
  const DshControlConnectException(this.code, [this.status]);
  final String code;
  final int? status;
}
