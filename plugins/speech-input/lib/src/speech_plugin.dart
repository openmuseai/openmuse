import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:record/record.dart';

import 'sherpa_config.dart';
import 'sherpa_worker.dart';

/// Built-in Mobile Host plugin that owns audio capture and exposes the typed
/// speech-recognition capability to Host UI consumers.
final class OpenMuseSpeechInputPlugin
    implements OpenMusePlugin, SpeechRecognitionPort {
  OpenMuseSpeechInputPlugin({required this.localModel, AudioRecorder? recorder})
    : _recorder = recorder;

  final SherpaZipformerConfig localModel;
  AudioRecorder? _recorder;

  SherpaSpeechWorker? _worker;
  StreamSubscription<SherpaWorkerEvent>? _workerEvents;
  StreamSubscription<List<int>>? _microphoneAudio;
  _SpeechSession? _active;
  SpeechSessionRef? _lastTerminal;
  bool _activated = false;

  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'com.openmuse.speech.input',
    name: 'OpenMuse Speech Input',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup', 'onCapability:speech.recognition'],
    permissions: {'device.microphone.capture', 'speech.recognition.provide'},
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _recorder ??= AudioRecorder();
    _activated = true;
  }

  @override
  Future<void> deactivate() async {
    if (!_activated) return;
    final active = _active;
    if (active != null) {
      await cancel(active.ref);
      if (_active == active) _finishCancelled(active);
    }
    await _workerEvents?.cancel();
    _workerEvents = null;
    await _worker?.dispose();
    _worker = null;
    await _recorder?.dispose();
    _recorder = null;
    _activated = false;
  }

  @override
  Future<SpeechProbe> probe({bool requestPermission = false}) async {
    final recorder = _recorder;
    if (!_activated || recorder == null) {
      throw const SpeechException('not_active', '语音输入插件尚未就绪。');
    }
    final permission = await recorder.hasPermission(request: requestPermission)
        ? SpeechPermissionState.granted
        : SpeechPermissionState.denied;
    final engine = localModel.isInstalled
        ? SpeechEngineAvailability.ready
        : SpeechEngineAvailability.needsModel;
    return SpeechProbe(
      permission: permission,
      localEngine: engine,
      supportsContextBias: localModel.decodingMethod == 'modified_beam_search',
      detail: localModel.isInstalled
          ? localModel.modelId
          : '缺少本地模型文件：${localModel.missingFiles.join(', ')}',
    );
  }

  @override
  Future<SpeechSession> open(SpeechStartRequest request) async {
    if (!_activated) {
      throw const SpeechException('not_active', '语音输入插件尚未就绪。');
    }
    if (_active != null) {
      throw const SpeechException('busy', '已有语音识别正在进行。');
    }
    if (request.enginePolicy == SpeechEnginePolicy.remoteOnly) {
      throw const SpeechException('remote_unavailable', '当前版本尚未配置服务端语音识别引擎。');
    }
    if (!localModel.isInstalled) {
      throw const SpeechException('model_missing', '本地语音模型尚未安装。');
    }
    if (request.source case SpeechFileSource(:final path)) {
      if (!kDebugMode) {
        throw const SpeechException('source_not_allowed', '当前构建不允许从文件启动语音识别。');
      }
      if (!File(path).existsSync()) {
        throw const SpeechException('audio_missing', '测试音频文件不存在。');
      }
    }

    final session = _SpeechSession(SpeechSessionRef(request.requestId));
    _lastTerminal = null;
    _active = session;
    session.emit(
      kind: SpeechEventKind.state,
      phase: SpeechSessionPhase.preparing,
    );
    Future<void>.microtask(() => _begin(session, request));
    return session;
  }

  Future<void> _begin(
    _SpeechSession session,
    SpeechStartRequest request,
  ) async {
    try {
      // Start capture before loading the model. The first utterance must not
      // lose its opening words while the worker performs a cold start.
      if (request.source is SpeechMicrophoneSource) {
        await _startMicrophoneCapture(session);
        if (_active != session) return;
      }

      final worker = _worker ??= SherpaSpeechWorker(localModel);
      _workerEvents ??= worker.events.listen(
        _onWorkerEvent,
        onError: (Object error, StackTrace stackTrace) {
          _failActive('worker_stream_failed', '语音识别服务异常，请重试。');
        },
      );
      await worker.start();
      if (_active != session) return;
      if (request.source is SpeechMicrophoneSource && session.stopRequested) {
        await session.captureStopping;
        if (_active != session) return;
      }
      worker.open(
        sessionId: session.ref.value,
        hotwords: localModel.decodingMethod == 'modified_beam_search'
            ? request.context.hotwords
            : '',
      );
      session.engineOpened = true;

      switch (request.source) {
        case SpeechFileSource(:final path):
          worker.recognizeFile(sessionId: session.ref.value, path: path);
        case SpeechMicrophoneSource():
          for (final bytes in session.pendingAudio) {
            worker.acceptPcm16(sessionId: session.ref.value, bytes: bytes);
          }
          session.pendingAudio.clear();
          session.pendingAudioBytes = 0;
          session.audioForwarding = true;
          if (session.stopRequested) worker.finish(session.ref.value);
      }
    } on SpeechException catch (error) {
      _fail(session, error.code, error.safeMessage);
    } on Object {
      _fail(session, 'engine_start_failed', '无法启动本地语音识别，请检查模型文件。');
    }
  }

  Future<void> _startMicrophoneCapture(_SpeechSession session) async {
    final recorder = _recorder;
    if (recorder == null) {
      throw const SpeechException('not_active', '语音输入插件尚未就绪。');
    }
    if (!await recorder.hasPermission()) {
      throw const SpeechException('permission_denied', '未获得麦克风权限。');
    }
    if (_active != session) return;
    final stream = await recorder.startStream(
      const RecordConfig(
        encoder: AudioEncoder.pcm16bits,
        sampleRate: 16000,
        numChannels: 1,
        autoGain: true,
        echoCancel: true,
        noiseSuppress: true,
        streamBufferSize: 3200,
      ),
    );
    if (_active != session) {
      await recorder.stop();
      return;
    }
    session.captureStarted = true;
    _microphoneAudio = stream.listen(
      (bytes) {
        if (_active != session) return;
        if (session.audioForwarding) {
          _worker?.acceptPcm16(sessionId: session.ref.value, bytes: bytes);
          return;
        }
        // 30 seconds of mono PCM16 is bounded to 960 KiB. An unusually slow
        // model load fails explicitly instead of silently dropping speech.
        const maxPendingAudioBytes = 16000 * 2 * 30;
        if (session.pendingAudioBytes + bytes.length > maxPendingAudioBytes) {
          _fail(session, 'model_start_timeout', '语音模型启动超时，请重试。');
          return;
        }
        session.pendingAudio.add(Uint8List.fromList(bytes));
        session.pendingAudioBytes += bytes.length;
      },
      onError: (Object error, StackTrace stackTrace) {
        _failActive('capture_failed', '麦克风采集失败，请重试。');
      },
    );
    if (session.stopRequested) {
      await _stopCapture(session, cancel: false);
    }
  }

  Future<void> _stopCapture(
    _SpeechSession session, {
    required bool cancel,
  }) async {
    final ongoing = session.captureStopping;
    if (ongoing != null) {
      await ongoing;
      return;
    }
    if (!session.captureStarted) return;
    final stopping = _endMicrophoneCapture(cancel: cancel);
    session.captureStopping = stopping;
    try {
      await stopping;
    } finally {
      session.captureStarted = false;
      session.captureStopping = null;
    }
  }

  void _onWorkerEvent(SherpaWorkerEvent event) {
    final session = _active;
    if (session == null || session.ref.value != event.sessionId) return;
    switch (event.kind) {
      case 'listening':
        session.emit(
          kind: SpeechEventKind.state,
          phase: SpeechSessionPhase.listening,
        );
      case 'partial':
        session.emit(
          kind: SpeechEventKind.partial,
          phase: SpeechSessionPhase.recognizing,
          text: event.text,
        );
      case 'final':
        session.emit(
          kind: SpeechEventKind.finalResult,
          phase: SpeechSessionPhase.completed,
          text: event.text ?? '',
        );
        _complete(session);
      case 'cancelled':
        _finishCancelled(session);
      case 'error':
        _fail(
          session,
          event.code ?? 'engine_failed',
          event.safeMessage ?? '语音识别失败，请重试。',
        );
    }
  }

  @override
  Future<void> updateContext(
    SpeechSessionRef session,
    SpeechContext context,
  ) async {
    _requireActive(session);
    // sherpa-onnx binds hotwords when a stream is created. The updated context
    // is intentionally applied by the Host when it opens the next segment.
  }

  @override
  Future<void> stop(SpeechSessionRef session) async {
    if (_lastTerminal == session) return;
    final active = _requireActive(session);
    if (active.stopRequested) return;
    active.stopRequested = true;
    await _stopCapture(active, cancel: false);
    if (_active == active && active.engineOpened && _worker != null) {
      _worker!.finish(session.value);
    }
  }

  @override
  Future<void> cancel(SpeechSessionRef session) async {
    if (_lastTerminal == session) return;
    final active = _requireActive(session)..stopRequested = true;
    await _stopCapture(active, cancel: true);
    if (_active == active && active.engineOpened && _worker != null) {
      _worker!.cancel(session.value);
    }
    _finishCancelled(active);
  }

  _SpeechSession _requireActive(SpeechSessionRef ref) {
    final active = _active;
    if (active == null || active.ref != ref) {
      throw const SpeechException('session_not_found', '语音识别会话已结束。');
    }
    return active;
  }

  Future<void> _endMicrophoneCapture({required bool cancel}) async {
    final subscription = _microphoneAudio;
    _microphoneAudio = null;
    final recorder = _recorder;
    try {
      if (cancel) {
        await recorder?.cancel();
      } else {
        await recorder?.stop();
      }
    } on Object {
      // A file-backed session or an already stopped recorder has no capture.
    }
    await subscription?.cancel();
  }

  void _failActive(String code, String message) {
    final active = _active;
    if (active != null) _fail(active, code, message);
  }

  void _fail(_SpeechSession session, String code, String message) {
    if (_active != session) return;
    session.emit(
      kind: SpeechEventKind.error,
      phase: SpeechSessionPhase.failed,
      code: code,
      safeMessage: message,
    );
    _complete(session);
    unawaited(_abortResources(session));
  }

  Future<void> _abortResources(_SpeechSession session) async {
    await _endMicrophoneCapture(cancel: true);
    if (session.engineOpened) _worker?.cancel(session.ref.value);
  }

  void _finishCancelled(_SpeechSession session) {
    if (_active != session) return;
    session.emit(
      kind: SpeechEventKind.state,
      phase: SpeechSessionPhase.cancelled,
    );
    _complete(session);
  }

  void _complete(_SpeechSession session) {
    session.pendingAudio.clear();
    session.pendingAudioBytes = 0;
    if (_active == session) {
      _active = null;
      _lastTerminal = session.ref;
    }
    unawaited(session.close());
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _SpeechSession implements SpeechSession {
  _SpeechSession(this.ref);

  @override
  final SpeechSessionRef ref;
  final StreamController<SpeechEvent> _events = StreamController<SpeechEvent>();
  int _sequence = 0;
  bool stopRequested = false;
  bool engineOpened = false;
  bool captureStarted = false;
  Future<void>? captureStopping;
  bool audioForwarding = false;
  final List<Uint8List> pendingAudio = [];
  int pendingAudioBytes = 0;

  @override
  Stream<SpeechEvent> get events => _events.stream;

  void emit({
    required SpeechEventKind kind,
    required SpeechSessionPhase phase,
    String? text,
    String? code,
    String? safeMessage,
  }) {
    if (_events.isClosed) return;
    _events.add(
      SpeechEvent(
        session: ref,
        sequence: _sequence++,
        kind: kind,
        phase: phase,
        text: text,
        code: code,
        safeMessage: safeMessage,
      ),
    );
  }

  Future<void> close() => _events.close();
}
