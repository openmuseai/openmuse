import 'dart:async';

import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_codec.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_host.dart';
import 'package:muse_dsh_mobile/src/dsh_remote_config.dart';
import 'package:webview_flutter/webview_flutter.dart';

/// Binds native capability requests to the current WebView generation.
/// One inflight speech request. Navigation or dispose cancels immediately.
class DshNativeCapabilityBroker {
  DshNativeCapabilityBroker({
    required this.config,
    required this.host,
    required this.generationOf,
    required this.isLive,
  });

  final DshRemoteConfig config;
  final DshNativeCapabilityHost host;
  final int Function() generationOf;
  final bool Function() isLive;

  WebViewController? _controller;
  String? _speechRequestId;
  Completer<bool>? _back;

  void attach(WebViewController controller) {
    _controller = controller;
  }

  Future<void> installChannel(WebViewController controller) async {
    _controller = controller;
    await controller.addJavaScriptChannel(
      DshNativeCapabilityCodec.javascriptChannelName,
      onMessageReceived: (message) {
        unawaited(handleWebMessage(message.message));
      },
    );
  }

  Future<void> handleWebMessage(String raw) async {
    final inbound = DshNativeCapabilityCodec.decode(raw);
    if (inbound == null || !isLive()) return;
    if (!await _originAllowed()) return;
    final generation = generationOf();
    if (inbound.generation != null && inbound.generation != generation) {
      if (inbound.type.startsWith('speech.')) {
        await _dispatch({
          'type': 'speech.error',
          'requestId': inbound.requestId,
          'code': 'STALE_GENERATION',
        });
      }
      return;
    }
    switch (inbound.type) {
      case 'capabilities.get':
        final caps = await host.capabilities();
        await _dispatch({
          'type': 'capabilities',
          'requestId': inbound.requestId,
          'generation': generation,
          ...caps.toJson().map((key, value) => MapEntry(key, value)),
        });
      case 'speech.start':
        await _startSpeech(inbound, generation);
      case 'speech.cancel':
        await cancelSpeech();
      case 'share.open':
        await _share(inbound);
      case 'back.result':
        _back?.complete(inbound.consumed == true);
        _back = null;
    }
  }

  Future<bool> requestBack() async {
    final previous = _back;
    if (previous != null && !previous.isCompleted) {
      previous.complete(false);
    }
    final pending = Completer<bool>();
    _back = pending;
    await _dispatch({
      'type': 'back.request',
      'requestId': 'back.${DateTime.now().microsecondsSinceEpoch}',
      'generation': generationOf(),
    });
    return pending.future.timeout(
      const Duration(milliseconds: 400),
      onTimeout: () {
        if (_back == pending) _back = null;
        return false;
      },
    );
  }

  Future<void> emitLifecycle(String state) async {
    if (state != 'foreground' && state != 'background') return;
    if (state == 'background') await cancelSpeech();
    await _dispatch({'type': 'lifecycle.changed', 'state': state});
  }

  Future<void> cancelSpeech() async {
    final requestId = _speechRequestId;
    _speechRequestId = null;
    await host.cancelSpeech();
    if (requestId != null) {
      await _dispatch({
        'type': 'speech.error',
        'requestId': requestId,
        'code': 'CANCELLED',
      });
    }
  }

  Future<void> dispose() async {
    await cancelSpeech();
    _controller = null;
    if (_back != null && !_back!.isCompleted) {
      _back!.complete(false);
    }
    _back = null;
  }

  Future<void> _startSpeech(DshNativeInbound inbound, int generation) async {
    if (_speechRequestId != null) {
      await _dispatch({
        'type': 'speech.error',
        'requestId': inbound.requestId,
        'code': 'CAPABILITY_UNAVAILABLE',
      });
      return;
    }
    final caps = await host.capabilities();
    if (!caps.speech) {
      await _dispatch({
        'type': 'speech.error',
        'requestId': inbound.requestId,
        'code': 'CAPABILITY_UNAVAILABLE',
      });
      return;
    }
    _speechRequestId = inbound.requestId;
    await host.startSpeech(
      DshSpeechListenRequest(
        requestId: inbound.requestId,
        generation: generation,
        sessionId: inbound.sessionId ?? '',
        locale: inbound.locale,
        onPartial: (text) {
          if (_speechRequestId != inbound.requestId || !isLive()) return;
          unawaited(
            _dispatch({
              'type': 'speech.partial',
              'requestId': inbound.requestId,
              'text': text,
            }),
          );
        },
        onFinal: (text) {
          if (_speechRequestId != inbound.requestId || !isLive()) return;
          _speechRequestId = null;
          unawaited(
            _dispatch({
              'type': 'speech.final',
              'requestId': inbound.requestId,
              'text': text,
            }),
          );
        },
        onError: (code) {
          if (_speechRequestId != inbound.requestId) return;
          _speechRequestId = null;
          unawaited(
            _dispatch({
              'type': 'speech.error',
              'requestId': inbound.requestId,
              'code': code,
            }),
          );
        },
      ),
    );
  }

  Future<void> _share(DshNativeInbound inbound) async {
    final target = inbound.textOrHttpsUrl?.trim() ?? '';
    if (target.isEmpty || target.length > 2048) return;
    final uri = Uri.tryParse(target);
    final isHttps = uri != null && uri.scheme == 'https' && uri.host.isNotEmpty;
    if (!isHttps && target.contains('://')) return;
    await host.shareOpen(target);
  }

  Future<bool> _originAllowed() async {
    final controller = _controller;
    if (controller == null) return false;
    final current = await controller.currentUrl();
    if (current == null) return false;
    final uri = Uri.tryParse(current);
    return uri != null && config.allows(uri);
  }

  Future<void> _dispatch(Map<String, Object?> message) async {
    final controller = _controller;
    if (controller == null || !isLive()) return;
    await controller.runJavaScript(
      DshNativeCapabilityCodec.dispatchScript(message),
    );
  }
}
