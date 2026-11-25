import 'dart:async';
import 'dart:io';
import 'dart:typed_data';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/speech_model_manager.dart';
import 'package:openmuse_speech_input/openmuse_speech_input.dart';

void main() {
  late Directory root;
  late SpeechModelManager manager;
  setUp(() async {
    root = await Directory.systemTemp.createTemp('speech-install-test-');
    manager = SpeechModelManager(
      SherpaZipformerConfig.zh14m('${root.path}/model'),
    );
  });
  tearDown(() async {
    manager.dispose();
    await root.delete(recursive: true);
  });
  test(
    'an exact-size corrupt model fails digest validation and leaves no partial install',
    () async {
      await HttpOverrides.runZoned(
        () => manager.install(),
        createHttpClient: (_) =>
            _Client(SpeechModelManager.files.values.first.$1),
      );
      expect(manager.failure, 'download');
      expect(manager.installed, isFalse);
      expect(manager.busy, isFalse);
      expect(root.listSync(), isEmpty);
    },
  );
  test(
    'cancel releases the operation and permits retry without an installed marker',
    () async {
      void cancelOnStart() {
        if (manager.busy) manager.cancel();
      }

      manager.addListener(cancelOnStart);
      await HttpOverrides.runZoned(
        () => manager.install(),
        createHttpClient: (_) => _Client(3),
      );
      expect(manager.failure, 'cancelled');
      expect(manager.busy, isFalse);
      expect(manager.installed, isFalse);
      manager.removeListener(cancelOnStart);
      await HttpOverrides.runZoned(
        () => manager.install(),
        createHttpClient: (_) => _Client(3),
      );
      expect(manager.failure, 'download');
      expect(manager.busy, isFalse);
      expect(root.listSync(), isEmpty);
    },
  );
  test('removal unloads the engine and preserves unrelated files', () async {
    var unloaded = false;
    manager.dispose();
    manager = SpeechModelManager(
      SherpaZipformerConfig.zh14m('${root.path}/model'),
      beforeRemove: () async => unloaded = true,
    );
    for (final path in manager.model.files) {
      await File(path).parent.create(recursive: true);
      await File(path).writeAsString('fixture');
    }
    final unrelated = File('${root.path}/model/user-note.txt');
    await unrelated.writeAsString('keep');
    await manager.remove();
    expect(unloaded, isTrue);
    expect(manager.installed, isFalse);
    expect(await unrelated.readAsString(), 'keep');
  });
  test('an active engine prevents deletion', () async {
    manager.dispose();
    manager = SpeechModelManager(
      SherpaZipformerConfig.zh14m('${root.path}/model'),
      beforeRemove: () async => throw StateError('active'),
    );
    for (final path in manager.model.files) {
      await File(path).parent.create(recursive: true);
      await File(path).writeAsString('fixture');
    }
    await manager.remove();
    expect(manager.failure, 'busy');
    expect(manager.installed, isTrue);
  });
}

final class _Client extends Fake implements HttpClient {
  _Client(this.size);
  final int size;
  @override
  set connectionTimeout(Duration? value) {}
  @override
  Future<HttpClientRequest> getUrl(Uri url) async => _Request(size);
  @override
  void close({bool force = false}) {}
}

final class _Request extends Fake implements HttpClientRequest {
  _Request(this.size);
  final int size;
  @override
  Future<HttpClientResponse> close() async => _Response(size);
}

final class _Response extends Stream<List<int>> implements HttpClientResponse {
  _Response(this.size);
  final int size;
  @override
  int get statusCode => 200;
  @override
  StreamSubscription<List<int>> listen(
    void Function(List<int>)? onData, {
    Function? onError,
    void Function()? onDone,
    bool? cancelOnError,
  }) {
    final zeros = Uint8List(512 * 1024);
    final chunks = Iterable.generate(
      (size / zeros.length).ceil(),
      (i) => zeros.sublist(0, (size - i * zeros.length).clamp(0, zeros.length)),
    );
    return Stream<List<int>>.fromIterable(chunks).listen(
      onData,
      onError: onError,
      onDone: onDone,
      cancelOnError: cancelOnError,
    );
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => super.noSuchMethod(invocation);
}
