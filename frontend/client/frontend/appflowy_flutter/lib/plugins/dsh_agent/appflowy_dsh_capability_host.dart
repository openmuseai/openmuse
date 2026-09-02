import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:share_plus/share_plus.dart';
import 'package:speech_to_text/speech_to_text.dart';

/// AppFlowy speech / share adapter. Audio never leaves this class as bytes.
class AppFlowyDshCapabilityHost implements DshNativeCapabilityHost {
  AppFlowyDshCapabilityHost({SpeechToText? speech})
      : _speech = speech ?? SpeechToText();

  final SpeechToText _speech;
  var _listening = false;

  @override
  Future<DshNativeCapabilities> capabilities() async {
    final speechReady = await _speech.initialize();
    return DshNativeCapabilities(
      file: true,
      camera: true,
      speech: speechReady,
      share: true,
    );
  }

  @override
  Future<void> startSpeech(DshSpeechListenRequest request) async {
    final status = await Permission.microphone.request();
    if (!status.isGranted) {
      request.onError('PERMISSION_DENIED');
      return;
    }
    final available = await _speech.initialize(
      onError: (_) => request.onError('SPEECH_ERROR'),
      onStatus: (status) {
        if (status == 'notListening' || status == 'done') {
          _listening = false;
        }
      },
    );
    if (!available) {
      request.onError('CAPABILITY_UNAVAILABLE');
      return;
    }
    _listening = true;
    await _speech.listen(
      listenOptions: SpeechListenOptions(
        localeId: request.locale,
        listenFor: const Duration(seconds: 60),
        pauseFor: const Duration(seconds: 3),
        listenMode: ListenMode.dictation,
      ),
      onResult: (result) {
        final text = result.recognizedWords.trim();
        if (text.isEmpty) return;
        if (result.finalResult) {
          _listening = false;
          request.onFinal(text);
        } else {
          request.onPartial(text);
        }
      },
    );
  }

  @override
  Future<void> cancelSpeech() async {
    if (!_listening && !_speech.isListening) {
      await _speech.cancel();
      return;
    }
    _listening = false;
    await _speech.cancel();
  }

  @override
  Future<void> shareOpen(String textOrHttpsUrl) async {
    final uri = Uri.tryParse(textOrHttpsUrl);
    if (uri != null && uri.scheme == 'https' && uri.host.isNotEmpty) {
      await Share.shareUri(uri);
      return;
    }
    await Share.share(textOrHttpsUrl);
  }
}
