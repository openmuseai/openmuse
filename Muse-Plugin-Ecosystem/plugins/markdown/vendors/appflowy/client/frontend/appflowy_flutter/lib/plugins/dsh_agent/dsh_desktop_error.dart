/// Maps Desktop sidecar failures onto the shared attachment error codes.
class DshDesktopError {
  DshDesktopError._();

  static const needApiKey = 'NEED_API_KEY';
  static const sidecarExit = 'SIDECAR_EXIT';
  static const frameTimeout = 'FRAME_TIMEOUT';

  static String? codeFromMessage(String message) {
    if (message.contains('DEEPSEEK_API_KEY')) return needApiKey;
    if (message.contains('did not become ready')) return frameTimeout;
    if (message.contains('exited') || message.contains('sidecar')) {
      return sidecarExit;
    }
    return null;
  }
}
