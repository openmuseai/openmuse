import 'dart:async';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:sherpa_onnx/sherpa_onnx.dart' as sherpa;

import 'sherpa_config.dart';

final class SherpaWorkerEvent {
  const SherpaWorkerEvent({
    required this.sessionId,
    required this.kind,
    this.text,
    this.code,
    this.safeMessage,
  });

  final String sessionId;
  final String kind;
  final String? text;
  final String? code;
  final String? safeMessage;
}

final class SherpaSpeechWorker {
  SherpaSpeechWorker(this.config);

  final SherpaZipformerConfig config;
  final StreamController<SherpaWorkerEvent> _events =
      StreamController<SherpaWorkerEvent>.broadcast(sync: true);
  ReceivePort? _receive;
  SendPort? _commands;
  Isolate? _isolate;
  Future<void>? _starting;
  Completer<void>? _disposing;

  Stream<SherpaWorkerEvent> get events => _events.stream;

  Future<void> start() => _starting ??= _start();

  Future<void> _start() async {
    final receive = ReceivePort('openmuse.speech.worker');
    _receive = receive;
    final ready = Completer<void>();
    receive.listen((Object? message) {
      final map = _messageMap(message);
      switch (map['kind']) {
        case 'worker-ready':
          _commands = map['commands']! as SendPort;
          if (!ready.isCompleted) ready.complete();
        case 'worker-init-error':
          if (!ready.isCompleted) {
            ready.completeError(StateError(map['safeMessage']! as String));
          }
        case 'worker-disposed':
          final disposing = _disposing;
          if (disposing != null && !disposing.isCompleted) {
            disposing.complete();
          }
        default:
          final sessionId = map['sessionId'];
          if (sessionId is String) {
            _events.add(
              SherpaWorkerEvent(
                sessionId: sessionId,
                kind: map['kind']! as String,
                text: map['text'] as String?,
                code: map['code'] as String?,
                safeMessage: map['safeMessage'] as String?,
              ),
            );
          }
      }
    });
    _isolate = await Isolate.spawn<List<Object?>>(_sherpaWorkerMain, [
      receive.sendPort,
      config.toMessage(),
    ], debugName: 'openmuse-speech-engine');
    await ready.future.timeout(const Duration(seconds: 45));
  }

  void open({required String sessionId, required String hotwords}) {
    _requireCommands().send({
      'kind': 'open',
      'sessionId': sessionId,
      'hotwords': hotwords,
    });
  }

  void recognizeFile({required String sessionId, required String path}) {
    _requireCommands().send({
      'kind': 'file',
      'sessionId': sessionId,
      'path': path,
    });
  }

  void acceptPcm16({required String sessionId, required Uint8List bytes}) {
    _requireCommands().send({
      'kind': 'audio',
      'sessionId': sessionId,
      'bytes': TransferableTypedData.fromList([bytes]),
    });
  }

  void finish(String sessionId) =>
      _requireCommands().send({'kind': 'finish', 'sessionId': sessionId});

  void cancel(String sessionId) =>
      _requireCommands().send({'kind': 'cancel', 'sessionId': sessionId});

  Future<void> dispose() async {
    final commands = _commands;
    if (commands != null) {
      final disposing = _disposing ??= Completer<void>();
      commands.send({'kind': 'dispose'});
      try {
        await disposing.future.timeout(const Duration(seconds: 2));
      } on TimeoutException {
        // The isolate kill below is the bounded fallback.
      }
    }
    _commands = null;
    _isolate?.kill(priority: Isolate.beforeNextEvent);
    _isolate = null;
    _receive?.close();
    _receive = null;
    await _events.close();
  }

  SendPort _requireCommands() {
    final commands = _commands;
    if (commands == null) throw StateError('speech worker is not ready');
    return commands;
  }
}

Map<Object?, Object?> _messageMap(Object? message) {
  if (message is! Map<Object?, Object?>) {
    throw StateError('invalid speech worker message');
  }
  return message;
}

Future<void> _sherpaWorkerMain(List<Object?> bootstrap) async {
  final host = bootstrap[0]! as SendPort;
  final rawConfig = _messageMap(bootstrap[1]);
  sherpa.OnlineRecognizer? recognizer;
  sherpa.OnlineStream? stream;
  String? activeSession;
  String lastPartial = '';
  final commands = ReceivePort('openmuse.speech.commands');

  void send(String kind, {String? text, String? code, String? safeMessage}) {
    final sessionId = activeSession;
    if (sessionId == null) return;
    host.send({
      'kind': kind,
      'sessionId': sessionId,
      if (text != null) 'text': text,
      if (code != null) 'code': code,
      if (safeMessage != null) 'safeMessage': safeMessage,
    });
  }

  void decodeReady() {
    final value = recognizer;
    final input = stream;
    if (value == null || input == null) return;
    while (value.isReady(input)) {
      value.decode(input);
    }
    final text = value.getResult(input).text.trim();
    if (text.isNotEmpty && text != lastPartial) {
      lastPartial = text;
      send('partial', text: text);
    }
  }

  void finishActive() {
    final value = recognizer;
    final input = stream;
    if (value == null || input == null) return;
    input.inputFinished();
    decodeReady();
    final text = value.getResult(input).text.trim();
    send('final', text: text);
    input.free();
    stream = null;
    activeSession = null;
    lastPartial = '';
  }

  void fail(Object error) {
    send('error', code: 'engine_failed', safeMessage: '语音识别引擎运行失败，请重试。');
    stream?.free();
    stream = null;
    activeSession = null;
    lastPartial = '';
  }

  try {
    sherpa.initBindings();
    recognizer = sherpa.OnlineRecognizer(
      sherpa.OnlineRecognizerConfig(
        model: sherpa.OnlineModelConfig(
          transducer: sherpa.OnlineTransducerModelConfig(
            encoder: rawConfig['encoder']! as String,
            decoder: rawConfig['decoder']! as String,
            joiner: rawConfig['joiner']! as String,
          ),
          tokens: rawConfig['tokens']! as String,
          numThreads: rawConfig['numThreads']! as int,
          debug: false,
          // The compact 2023 14M bundle exports the legacy Zipformer metadata
          // schema. Declaring zipformer2 makes the native runtime require
          // query_head_dims and abort the process before Dart can recover.
          modelType: 'zipformer',
        ),
        decodingMethod: rawConfig['decodingMethod']! as String,
        enableEndpoint: false,
      ),
    );
    host.send({'kind': 'worker-ready', 'commands': commands.sendPort});
  } on Object {
    host.send({'kind': 'worker-init-error', 'safeMessage': '无法加载本地语音模型。'});
    commands.close();
    Isolate.exit();
  }

  await for (final Object? message in commands) {
    final command = _messageMap(message);
    try {
      switch (command['kind']) {
        case 'open':
          stream?.free();
          activeSession = command['sessionId']! as String;
          lastPartial = '';
          stream = recognizer.createStream(
            hotwords: command['hotwords']! as String,
          );
          send('listening');
        case 'file':
          if (activeSession != command['sessionId']) break;
          final wave = sherpa.readWave(command['path']! as String);
          if (wave.samples.isEmpty || wave.sampleRate <= 0) {
            throw StateError('invalid WAV input');
          }
          const chunkSamples = 1600;
          for (var offset = 0; offset < wave.samples.length;) {
            final end = (offset + chunkSamples)
                .clamp(0, wave.samples.length)
                .toInt();
            stream!.acceptWaveform(
              samples: Float32List.sublistView(wave.samples, offset, end),
              sampleRate: wave.sampleRate,
            );
            decodeReady();
            offset = end;
          }
          finishActive();
        case 'audio':
          if (activeSession != command['sessionId']) break;
          final transferable = command['bytes']! as TransferableTypedData;
          final bytes = transferable.materialize().asUint8List();
          final sampleCount = bytes.length ~/ 2;
          final samples = Float32List(sampleCount);
          final data = ByteData.sublistView(bytes);
          for (var index = 0; index < sampleCount; index += 1) {
            samples[index] = data.getInt16(index * 2, Endian.little) / 32768.0;
          }
          stream!.acceptWaveform(samples: samples, sampleRate: 16000);
          decodeReady();
        case 'finish':
          if (activeSession == command['sessionId']) finishActive();
        case 'cancel':
          if (activeSession != command['sessionId']) break;
          stream?.free();
          send('cancelled');
          stream = null;
          activeSession = null;
          lastPartial = '';
        case 'dispose':
          stream?.free();
          recognizer.free();
          host.send({'kind': 'worker-disposed'});
          commands.close();
          Isolate.exit();
      }
    } on Object catch (error) {
      fail(error);
    }
  }
}
