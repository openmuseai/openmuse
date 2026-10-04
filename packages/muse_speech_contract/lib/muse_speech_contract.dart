library;

import 'dart:async';

enum SpeechSessionPhase {
  preparing,
  listening,
  recognizing,
  finalizing,
  completed,
  cancelled,
  failed,
}

enum SpeechEnginePolicy { localOnly, remoteOnly, preferLocal }

enum SpeechPermissionState { unknown, denied, granted }

enum SpeechEngineAvailability { unavailable, needsModel, ready }

enum SpeechEventKind { state, partial, finalResult, error }

final class SpeechException implements Exception {
  const SpeechException(this.code, this.safeMessage);

  final String code;
  final String safeMessage;

  @override
  String toString() => 'SpeechException($code): $safeMessage';
}

sealed class SpeechAudioSource {
  const SpeechAudioSource();
}

final class SpeechMicrophoneSource extends SpeechAudioSource {
  const SpeechMicrophoneSource();
}

/// A trusted Host test seam. File paths are never accepted over the public
/// plugin wire contract.
final class SpeechFileSource extends SpeechAudioSource {
  const SpeechFileSource(this.path, {this.realtime = false});

  final String path;
  final bool realtime;
}

final class SpeechContext {
  const SpeechContext({
    this.localeHints = const ['zh-CN'],
    this.terms = const [],
    this.phraseHints = const [],
    this.revision = 0,
  });

  final List<String> localeHints;
  final List<String> terms;
  final List<String> phraseHints;
  final int revision;

  String get hotwords {
    final values = <String>[];
    var totalLength = 0;
    for (final raw in [...terms, ...phraseHints]) {
      var value = raw.replaceAll(RegExp(r'\s+'), ' ').trim();
      if (value.isEmpty) continue;
      if (value.length > 64) value = value.substring(0, 64);
      if (totalLength + value.length > 2048) break;
      values.add(value);
      totalLength += value.length;
      if (values.length == 64) break;
    }
    return values.join('\n');
  }
}

final class SpeechStartRequest {
  const SpeechStartRequest({
    required this.requestId,
    required this.source,
    this.enginePolicy = SpeechEnginePolicy.localOnly,
    this.context = const SpeechContext(),
  });

  final String requestId;
  final SpeechAudioSource source;
  final SpeechEnginePolicy enginePolicy;
  final SpeechContext context;
}

final class SpeechProbe {
  const SpeechProbe({
    required this.permission,
    required this.localEngine,
    this.supportsPartial = true,
    this.supportsContextBias = true,
    this.detail,
  });

  final SpeechPermissionState permission;
  final SpeechEngineAvailability localEngine;
  final bool supportsPartial;
  final bool supportsContextBias;
  final String? detail;

  bool get ready => localEngine == SpeechEngineAvailability.ready;
}

final class SpeechSessionRef {
  const SpeechSessionRef(this.value);

  final String value;

  @override
  bool operator ==(Object other) =>
      other is SpeechSessionRef && other.value == value;

  @override
  int get hashCode => value.hashCode;

  @override
  String toString() => value;
}

final class SpeechEvent {
  const SpeechEvent({
    required this.session,
    required this.sequence,
    required this.kind,
    required this.phase,
    this.text,
    this.code,
    this.safeMessage,
  });

  final SpeechSessionRef session;
  final int sequence;
  final SpeechEventKind kind;
  final SpeechSessionPhase phase;
  final String? text;
  final String? code;
  final String? safeMessage;

  bool get terminal =>
      phase == SpeechSessionPhase.completed ||
      phase == SpeechSessionPhase.cancelled ||
      phase == SpeechSessionPhase.failed;
}

abstract interface class SpeechSession {
  SpeechSessionRef get ref;
  Stream<SpeechEvent> get events;
}

abstract interface class SpeechRecognitionPort {
  Future<SpeechProbe> probe({bool requestPermission = false});

  Future<SpeechSession> open(SpeechStartRequest request);

  Future<void> updateContext(SpeechSessionRef session, SpeechContext context);

  Future<void> stop(SpeechSessionRef session);

  Future<void> cancel(SpeechSessionRef session);
}
