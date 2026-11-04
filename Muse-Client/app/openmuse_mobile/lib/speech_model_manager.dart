import 'dart:async';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import 'package:openmuse_speech_input/openmuse_speech_input.dart';

/// One reviewed model, pinned to its upstream revision and file digests.
/// Downloads are staged and verified before replacing known model files.
final class SpeechModelManager extends ChangeNotifier {
  SpeechModelManager(this.model, {this.beforeRemove, this.downloadOrigin});

  final SherpaZipformerConfig model;
  final Future<void> Function()? beforeRemove;
  final Uri? downloadOrigin;
  static const revision = '204ad334e2e683fd295359930cc16fc0432a23ac';
  static const files = <String, (int, String)>{
    'encoder-epoch-99-avg-1.int8.onnx': (
      21621684,
      '1c556ea57cec304e55ec4b72e52c1cc098bb01476ed7d90f3de939fe126487b1',
    ),
    'decoder-epoch-99-avg-1.onnx': (
      7509745,
      '5ee0f03a2768ff1d5c83ef3a493243c7935d316cd41280037b14783a3467cc78',
    ),
    'joiner-epoch-99-avg-1.int8.onnx': (
      1795562,
      'a7cf9d82757bdcf786059454495a9ca95e4bd7347f72473fc08d794475c36169',
    ),
    'tokens.txt': (
      48697,
      '8b294db9045d6e5f94647f4c1eec1af4da143a75053c399611444b378ff966ac',
    ),
  };
  static int get totalBytes => files.values.fold(0, (sum, f) => sum + f.$1);
  bool busy = false;
  bool removing = false;
  int downloadedBytes = 0;
  String? failure;
  String? diagnostic;
  HttpClient? _http;
  bool _cancelled = false;
  bool _disposed = false;
  bool get installed => model.isInstalled;
  double get progress => (downloadedBytes / totalBytes).clamp(0, 1);
  void _changed() {
    if (!_disposed) notifyListeners();
  }

  Future<void> install() async {
    if (busy || installed) return;
    busy = true;
    _cancelled = false;
    downloadedBytes = 0;
    failure = null;
    diagnostic = null;
    _changed();
    Directory? staging;
    final http = HttpClient()..connectionTimeout = const Duration(seconds: 20);
    _http = http;
    try {
      final destination = File(model.encoder).parent;
      await destination.parent.create(recursive: true);
      staging = await destination.parent.createTemp('.speech-download-');
      for (final entry in files.entries) {
        if (_cancelled) throw const _DownloadCancelled();
        final uri =
            downloadOrigin?.resolve(
              '/resources/speech/${model.modelId}/$revision/${entry.key}',
            ) ??
            Uri.https(
              'huggingface.co',
              '/csukuangfj/${model.modelId}/resolve/$revision/${entry.key}',
            );
        final request = await http
            .getUrl(uri)
            .timeout(const Duration(seconds: 25));
        final response = await request.close().timeout(
          const Duration(seconds: 30),
        );
        if (response.statusCode != 200) {
          throw HttpException('HTTP ${response.statusCode}');
        }
        final output = File('${staging.path}/${entry.key}');
        final sink = output.openWrite();
        var size = 0;
        try {
          await for (final chunk in response.timeout(
            const Duration(seconds: 30),
          )) {
            if (_cancelled) throw const _DownloadCancelled();
            size += chunk.length;
            if (size > entry.value.$1) {
              throw const FormatException('Model size mismatch');
            }
            sink.add(chunk);
            downloadedBytes += chunk.length;
            _changed();
          }
          await sink.flush();
        } finally {
          await sink.close();
        }
        if (size != entry.value.$1 ||
            (await sha256.bind(output.openRead()).first).toString() !=
                entry.value.$2) {
          throw const FormatException('Model digest mismatch');
        }
      }
      if (_cancelled) throw const _DownloadCancelled();
      await destination.create(recursive: true);
      for (final name in files.keys) {
        await File('${staging.path}/$name').rename('${destination.path}/$name');
      }
    } on Object catch (error) {
      failure = _cancelled ? 'cancelled' : 'download';
      diagnostic = error is HttpException
          ? error.message
          : '${error.runtimeType}';
      debugPrint('OpenMuse speech model install: $failure $diagnostic');
    } finally {
      http.close(force: true);
      _http = null;
      if (staging != null && await staging.exists()) {
        await staging.delete(recursive: true);
      }
      busy = false;
      _changed();
    }
  }

  void cancel() {
    _cancelled = true;
    _http?.close(force: true);
  }

  Future<void> remove() async {
    if (busy) return;
    busy = true;
    removing = true;
    failure = null;
    _changed();
    try {
      await beforeRemove?.call();
      for (final path in model.files) {
        final file = File(path);
        if (await file.exists()) await file.delete();
      }
    } on Object {
      failure = 'busy';
    } finally {
      busy = false;
      removing = false;
      downloadedBytes = 0;
      _changed();
    }
  }

  @override
  void dispose() {
    _disposed = true;
    cancel();
    super.dispose();
  }
}

final class _DownloadCancelled implements Exception {
  const _DownloadCancelled();
}
