import 'dart:convert';

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_engine.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_runtime.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const resource = MuseResourceRef(
    id: 'resource',
    repository: MuseRepositoryRef(id: 'repo', providerId: 'test'),
    locator: '/workspace/AICombConfig.kt',
    mediaType: 'text/plain',
    displayName: 'AICombConfig.kt',
  );

  test('FFI codec rejects content leaks and keeps compact coordinates', () {
    expect(
      () => MuseDiffTextFfiCodec.decode(
        jsonEncode({
          'quality': 'exact',
          'blocks': [],
          'text': 'secret-document',
        }),
      ),
      throwsFormatException,
    );

    final result = MuseDiffTextFfiCodec.decode(
      jsonEncode({
        'quality': 'exact',
        'blocks': [
          {
            'kind': 'replace',
            'oldStart': 1,
            'oldCount': 1,
            'newStart': 1,
            'newCount': 1,
          },
        ],
        'similarBoundaries': [
          {'oldLine': 0, 'newLine': 0},
          {'oldLine': 2, 'newLine': 2},
        ],
        'additions': 1,
        'deletions': 1,
        'beforeNewline': 'lf',
        'afterNewline': 'lf',
      }),
    );

    expect(MuseDiffTextFfiCodec.allowedKeys, contains('blocks'));
    expect(result, isNotNull);
    expect(result!.engine, 'muse-diff-text-ffi');
    expect(result.blocks, hasLength(1));
    expect(result.blocks.single.oldStart, 1);
    expect(result.similarBoundaries, hasLength(2));
  });

  test('Host compare uses compact FFI blocks when symbols are available',
      () async {
    final provider = MuseTextDiffProvider(
      runtime: MuseDiffTextRuntime(ffi: _FakeFfi()),
    );
    const before = 'fun isSupportRTC(): Boolean {\n  return true\n}\n';
    const after = 'fun isSupportRTC(): Boolean {\n  return false\n}\n';
    final payload = await provider.compareTextAsync(
      resource: resource,
      baseText: before,
      targetText: after,
    );

    expect(payload.engine, 'muse-diff-text-ffi');
    expect(payload.hunks, isNotEmpty);
    expect(payload.changeCount, 1);
    expect(payload.baseText, before);
    expect(
      payload.hunks.expand((hunk) => hunk.changes).single.kind,
      MuseTextChangeKind.replace,
    );
  });

  test('Host compare falls back to Dart Myers when FFI is unavailable',
      () async {
    final provider = MuseTextDiffProvider(
      runtime: MuseDiffTextRuntime(forceDartFallback: true),
    );
    final payload = await provider.compareTextAsync(
      resource: resource,
      baseText: 'one\ntwo\n',
      targetText: 'one\nTWO\nthree\n',
    );
    expect(payload.engine, MuseDartTextDiffEngine.engineId);
    expect(payload.changeCount, 1);
    expect(payload.additions, 2);
    expect(payload.deletions, 1);
  });

  test('search and cross-line copy stay on local snapshots', () {
    const document = 'alpha\nbeta\ngamma\n';
    expect(
      MuseDartTextDiffEngine.copyLines(document, 2, 3),
      'beta\ngamma',
    );
    final hits = MuseDartTextDiffEngine.search(document, 'ga');
    expect(hits, hasLength(1));
    expect(hits.single.line, 3);
  });
}

final class _FakeFfi implements MuseDiffTextFfiClient {
  @override
  bool get symbolsAvailable => true;

  @override
  String compareJson({
    required String before,
    required String after,
    required int maxMillis,
    required int maxTraceBytes,
  }) {
    expect(before, contains('isSupportRTC'));
    expect(after, contains('return false'));
    return jsonEncode({
      'quality': 'exact',
      'blocks': [
        {
          'kind': 'replace',
          'oldStart': 1,
          'oldCount': 1,
          'newStart': 1,
          'newCount': 1,
        },
      ],
      'similarBoundaries': [
        {'oldLine': 0, 'newLine': 0},
        {'oldLine': 1, 'newLine': 1},
        {'oldLine': 2, 'newLine': 2},
        {'oldLine': 3, 'newLine': 3},
      ],
      'additions': 1,
      'deletions': 1,
      'beforeNewline': 'lf',
      'afterNewline': 'lf',
    });
  }
}
