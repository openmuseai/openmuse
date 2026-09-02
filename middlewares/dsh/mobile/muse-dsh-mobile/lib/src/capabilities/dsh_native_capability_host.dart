/// Host-owned native capabilities. Implementations live in the embedding app.
/// Must not send tokens, filesystem paths, content URIs, or file bytes to JS.
abstract class DshNativeCapabilityHost {
  Future<DshNativeCapabilities> capabilities();

  Future<void> startSpeech(DshSpeechListenRequest request);

  Future<void> cancelSpeech();

  Future<void> shareOpen(String textOrHttpsUrl);
}

class DshNativeCapabilities {
  const DshNativeCapabilities({
    required this.file,
    required this.camera,
    required this.speech,
    required this.share,
  });

  static const none = DshNativeCapabilities(
    file: false,
    camera: false,
    speech: false,
    share: false,
  );

  final bool file;
  final bool camera;
  final bool speech;
  final bool share;

  Map<String, bool> toJson() => {
        'file': file,
        'camera': camera,
        'speech': speech,
        'share': share,
      };
}

class DshSpeechListenRequest {
  const DshSpeechListenRequest({
    required this.requestId,
    required this.generation,
    required this.sessionId,
    this.locale,
    required this.onPartial,
    required this.onFinal,
    required this.onError,
  });

  final String requestId;
  final int generation;
  final String sessionId;
  final String? locale;
  final void Function(String text) onPartial;
  final void Function(String text) onFinal;
  final void Function(String code) onError;
}
