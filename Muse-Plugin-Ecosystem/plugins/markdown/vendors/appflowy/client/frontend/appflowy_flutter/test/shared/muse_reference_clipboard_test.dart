import 'dart:convert';

import 'package:appflowy/plugins/document/application/muse_document_selection_reference.dart';
import 'package:appflowy/shared/muse_reference_clipboard.dart';
import 'package:flutter_test/flutter_test.dart';

/// The Host half of the Muse resource-reference clipboard format (RCX-01):
/// the payload the Host writes on copy, its bounds, and the HTML marker the DSH
/// composer reads. This is the producing half; the DSH client half decodes the
/// same bytes (`middlewares/dsh/plugins/dsh-client-ui-resource-reference`).
void main() {
  const viewId = '8f14e45f-ea0a-4f2b-9d4a-3f2b1c0d9e77';

  MuseResourceReference reference({
    String text = 'the plan',
    String? displayName,
    int? startBlock,
    int? endBlock,
    int? capturedAt,
    int maxExcerptBytes = museReferenceExcerptMaxBytes,
  }) =>
      buildMuseResourceReference(
        resourceRef: museDocumentResourceRef(viewId),
        viewRef: museDocumentViewRef(viewId),
        mountRef: 'mount:1d7c0de5b12f4a8e9c3b6d5a0f2e8b41c7d9a6f3e5b8c2d4a1f0e9b7c6d5a4f3',
        displayName: displayName ?? 'Design doc',
        selectedText: text,
        startBlock: startBlock ?? 3,
        endBlock: endBlock ?? 8,
        startBlockRef: 'block-3',
        endBlockRef: 'block-8',
        capturedAt: capturedAt ?? 1765000000000,
        maxExcerptBytes: maxExcerptBytes,
      );

  group('opaque references', () {
    test('a document mints namespaced refs that carry no path and no colon', () {
      final resourceRef = museDocumentResourceRef(viewId);
      final ref = museDocumentViewRef(viewId);
      expect(resourceRef, 'resource.appflowy.document.$viewId');
      expect(ref, 'view.appflowy.document.$viewId');
      expect(museReferenceOpaqueRefPattern.hasMatch(resourceRef), isTrue);
      expect(museReferenceOpaqueRefPattern.hasMatch(ref), isTrue);
      for (final value in [resourceRef, ref]) {
        expect(value, isNot(contains('/')));
        expect(value, isNot(contains(r'\')));
        expect(value, isNot(contains(':')));
      }
    });

    test('a view id that is not an AppFlowy id never becomes a ref', () {
      expect(() => museDocumentResourceRef('../../etc/passwd'), throwsArgumentError);
      expect(() => museDocumentViewRef(''), throwsArgumentError);
      expect(() => museDocumentViewRef('a' * 65), throwsArgumentError);
    });
  });

  group('payload round-trip', () {
    test('every field survives encode → decode', () {
      final source = reference();
      final decoded = MuseResourceReference.tryDecode(source.encode());
      expect(decoded, isNotNull);
      expect(decoded!.toJson(), source.toJson());
      expect(decoded.encode(), source.encode());
      expect(decoded.protocol, museReferenceProtocol);
      expect(decoded.anchor.kind, museReferenceAnchorKindRange);
      expect(decoded.anchor.label, 'blocks 4–9');
      expect(decoded.anchor.startBlockRef, 'block-3');
      expect(decoded.anchor.endBlockRef, 'block-8');
      expect(decoded.viewRef, museDocumentViewRef(viewId));
      expect(decoded.mountRef, startsWith('mount:'));
    });

    test('the encoded payload is the documented clipboard JSON, key order included', () {
      expect(jsonDecode(reference().encode()), <String, Object?>{
        'protocol': 'muse.clipboard/resource-reference/v1',
        'source': 'host.selection',
        'resourceRef': 'resource.appflowy.document.$viewId',
        'viewRef': 'view.appflowy.document.$viewId',
        'mountRef': 'mount:1d7c0de5b12f4a8e9c3b6d5a0f2e8b41c7d9a6f3e5b8c2d4a1f0e9b7c6d5a4f3',
        'displayName': 'Design doc',
        'anchor': <String, Object?>{
          'kind': 'range',
          'startBlock': 3,
          'endBlock': 8,
          'startBlockRef': 'block-3',
          'endBlockRef': 'block-8',
          'label': 'blocks 4–9',
        },
        'excerpt': 'the plan',
        'excerptBytes': 8,
        'truncated': false,
        'capturedAt': 1765000000000,
      });
      // The exact JSON text the DSH client half reads out of the HTML marker.
      expect(
        reference().encode(),
        '{"protocol":"muse.clipboard/resource-reference/v1","source":"host.selection",'
        '"resourceRef":"resource.appflowy.document.$viewId",'
        '"viewRef":"view.appflowy.document.$viewId",'
        '"mountRef":"mount:1d7c0de5b12f4a8e9c3b6d5a0f2e8b41c7d9a6f3e5b8c2d4a1f0e9b7c6d5a4f3",'
        '"displayName":"Design doc","anchor":{"kind":"range","startBlock":3,"endBlock":8,'
        '"startBlockRef":"block-3","endBlockRef":"block-8","label":"blocks 4–9"},'
        '"excerpt":"the plan","excerptBytes":8,"truncated":false,"capturedAt":1765000000000}',
      );
    });

    test('one block anchor is labelled as a single block', () {
      final single = reference(startBlock: 0, endBlock: 0);
      expect(single.anchor.label, 'block 1');
    });

    test('a payload without view or mount still encodes and decodes', () {
      final bare = buildMuseResourceReference(
        resourceRef: 'resource.appflowy.document.$viewId',
        displayName: 'Note',
        selectedText: 'x',
        startBlock: 0,
        endBlock: 0,
        capturedAt: 1,
      );
      final json = jsonDecode(bare.encode()) as Map<String, Object?>;
      expect(json.containsKey('viewRef'), isFalse);
      expect(json.containsKey('mountRef'), isFalse);
      expect(MuseResourceReference.tryDecode(bare.encode())!.excerptBytes, 1);
    });
  });

  group('bounds', () {
    test('an excerpt under the bound is whole and not truncated', () {
      final bounded = boundMuseExcerpt('short');
      expect(bounded.text, 'short');
      expect(bounded.bytes, 5);
      expect(bounded.truncated, isFalse);
    });

    test('an excerpt exactly at the bound is whole and not truncated', () {
      final text = 'a' * museReferenceExcerptMaxBytes;
      final bounded = boundMuseExcerpt(text);
      expect(bounded.text.length, museReferenceExcerptMaxBytes);
      expect(bounded.bytes, museReferenceExcerptMaxBytes);
      expect(bounded.truncated, isFalse);
      expect(utf8.encode(bounded.text).length, museReferenceExcerptMaxBytes);
    });

    test('an excerpt one byte over the bound is cut at the bound and truncated', () {
      final text = 'a' * (museReferenceExcerptMaxBytes + 1);
      final bounded = boundMuseExcerpt(text);
      expect(bounded.bytes, museReferenceExcerptMaxBytes);
      expect(bounded.text.length, museReferenceExcerptMaxBytes);
      expect(bounded.truncated, isTrue);
    });

    test('a two-byte character straddling the bound is dropped whole', () {
      // 4000 bytes of 'é', then 'a's up to byte 8191, then a 3-byte character
      // that cannot fit: the cut lands before it and the 'a' run survives.
      final text = 'é' * 2000 + 'a' * (museReferenceExcerptMaxBytes - 4001) + '✓尾';
      final bounded = boundMuseExcerpt(text);
      expect(bounded.truncated, isTrue);
      expect(bounded.bytes, museReferenceExcerptMaxBytes - 1);
      expect(bounded.text.endsWith('a'), isTrue);
      expect(bounded.text.contains('✓'), isFalse);
      expect(bounded.text.contains('尾'), isFalse);
      expect(utf8.encode(bounded.text).length, bounded.bytes);
    });

    test('a four-byte character straddling the bound is dropped whole', () {
      final text = '😀' * 2047 + 'ab😀';
      expect(utf8.encode('😀' * 2047).length, 8188);
      final bounded = boundMuseExcerpt(text);
      expect(bounded.truncated, isTrue);
      expect(bounded.text, '😀' * 2047 + 'ab');
      expect(bounded.bytes, 8190);
      expect(bounded.text.contains('\uFFFD'), isFalse);
      expect(utf8.encode(bounded.text).length, 8190);
    });

    test('the payload records the byte count and the truncation of a long selection', () {
      final source = reference(text: 'x' * (museReferenceExcerptMaxBytes + 10));
      final decoded = MuseResourceReference.tryDecode(source.encode())!;
      expect(decoded.truncated, isTrue);
      expect(decoded.excerptBytes, museReferenceExcerptMaxBytes);
      expect(decoded.excerpt.length, museReferenceExcerptMaxBytes);
    });

    test('a multibyte excerpt at the bound is counted in bytes, not runes', () {
      // 4096 × 'é' = 8192 bytes: fits exactly even though it is 4096 characters.
      final bounded = boundMuseExcerpt('é' * 4096);
      expect(bounded.bytes, museReferenceExcerptMaxBytes);
      expect(bounded.truncated, isFalse);
      final over = boundMuseExcerpt('é' * 4096 + 'é');
      expect(over.bytes, museReferenceExcerptMaxBytes);
      expect(over.truncated, isTrue);
    });
  });

  group('decode is a clipboard boundary', () {
    test('a foreign protocol, a path-shaped ref, or a broken anchor is refused', () {
      final valid = jsonDecode(reference().encode()) as Map<String, Object?>;
      Map<String, Object?> withField(String key, Object? value) =>
          Map<String, Object?>.from(valid)..[key] = value;

      expect(MuseResourceReference.tryDecode('not json'), isNull);
      expect(MuseResourceReference.tryDecode('{}'), isNull);
      expect(MuseResourceReference.tryDecode(jsonEncode(withField('protocol', 'other'))), isNull);
      expect(
        MuseResourceReference.tryDecode(
          jsonEncode(withField('resourceRef', r'resource:C:\Users\me\notes.md')),
        ),
        isNull,
      );
      expect(MuseResourceReference.tryDecode(jsonEncode(withField('resourceRef', 'a/b'))), isNull);
      expect(MuseResourceReference.tryDecode(jsonEncode(withField('displayName', '  '))), isNull);
      expect(MuseResourceReference.tryDecode(jsonEncode(withField('excerpt', 42))), isNull);
      expect(
        MuseResourceReference.tryDecode(
          jsonEncode(withField('anchor', {'startBlock': 8, 'endBlock': 3})),
        ),
        isNull,
      );
      expect(MuseResourceReference.tryDecode(jsonEncode(withField('anchor', null))), isNull);
    });

    test('an over-long excerpt pasted back in is re-bounded, with the flag set', () {
      final valid = jsonDecode(reference().encode()) as Map<String, Object?>;
      valid['excerpt'] = 'z' * (museReferenceExcerptMaxBytes * 3);
      valid['excerptBytes'] = museReferenceExcerptMaxBytes * 3;
      valid['truncated'] = false;
      final decoded = MuseResourceReference.tryDecode(jsonEncode(valid))!;
      expect(decoded.excerptBytes, museReferenceExcerptMaxBytes);
      expect(decoded.truncated, isTrue);
    });

    test('a Host truncation flag is preserved through the round-trip', () {
      final valid = jsonDecode(reference().encode()) as Map<String, Object?>;
      valid['truncated'] = true;
      expect(MuseResourceReference.tryDecode(jsonEncode(valid))!.truncated, isTrue);
    });
  });

  group('HTML marker carrier', () {
    test('the marker prefixes the copied HTML and survives extraction', () {
      final payload = reference().encode();
      final html = museReferenceHtmlCarrier('<p>the plan</p>', payload);
      expect(html, startsWith('<div $museReferenceHtmlAttribute="'));
      expect(html, contains(' $museReferenceHtmlVersionAttribute="1"></div>'));
      expect(html, endsWith('<p>the plan</p>'));
      expect(museReferencePayloadFromHtml(html), payload);
    });

    test('an empty fragment still carries the marker', () {
      final html = museReferenceHtmlCarrier(null, reference().encode());
      expect(html, startsWith('<div $museReferenceHtmlAttribute="'));
      expect(museReferencePayloadFromHtml(html), reference().encode());
    });

    test('a fragment without a marker, or with a broken one, yields nothing', () {
      expect(museReferencePayloadFromHtml('<p>the plan</p>'), isNull);
      expect(
        museReferencePayloadFromHtml('<div $museReferenceHtmlAttribute="!!!"></div>'),
        isNull,
      );
      expect(
        museReferencePayloadFromHtml('<div $museReferenceHtmlAttribute=""></div>'),
        isNull,
      );
    });

    test('the base64url payload is extracted from a foreign base64 alphabet too', () {
      final payload = reference().encode();
      final encoded = base64Url.encode(utf8.encode(payload));
      final html =
          '<div $museReferenceHtmlAttribute="$encoded"></div><p>kept</p>';
      expect(museReferencePayloadFromHtml(html), payload);
    });
  });

  group('selection source registry', () {
    setUp(MuseSelectionReferenceRegistry.instance.reset);
    tearDown(MuseSelectionReferenceRegistry.instance.reset);

    test('with no live surface a copy describes nothing (plain-text fallback)', () {
      expect(MuseSelectionReferenceRegistry.instance.source, isNull);
      expect(
        MuseSelectionReferenceRegistry.instance.describe(
          selectedText: 'the plan',
          startBlock: 0,
          endBlock: 1,
        ),
        isNull,
      );
    });

    test('an attached surface describes its selection, and detaching clears it', () {
      final source = _FakeSource();
      final registry = MuseSelectionReferenceRegistry.instance;
      registry.attach(source);
      expect(registry.source, same(source));

      final described = registry.describe(
        selectedText: 'the plan',
        startBlock: 2,
        endBlock: 4,
        startBlockRef: 'b2',
        endBlockRef: 'b4',
        capturedAt: 7,
      );
      expect(described, isNotNull);
      expect(described!.resourceRef, _FakeSource.resourceRefValue);
      expect(described.anchor.startBlock, 2);
      expect(described.anchor.endBlock, 4);
      expect(described.anchor.startBlockRef, 'b2');
      expect(described.capturedAt, 7);
      expect(described.excerpt, 'the plan');

      registry.detach(source);
      expect(registry.source, isNull);
      expect(
        registry.describe(selectedText: 'the plan', startBlock: 0, endBlock: 0),
        isNull,
      );
    });

    test('an empty selection and a source that describes nothing are refused', () {
      final registry = MuseSelectionReferenceRegistry.instance;
      registry.attach(_FakeSource());
      expect(
        registry.describe(selectedText: '   ', startBlock: 0, endBlock: 0),
        isNull,
      );
      registry.attach(_InvalidSource());
      expect(
        registry.describe(selectedText: 'text', startBlock: 0, endBlock: 0),
        isNull,
      );
    });

    test('attaching a second surface replaces the first, and the stale detach is a no-op', () {
      final first = _FakeSource();
      final second = _FakeSource();
      final registry = MuseSelectionReferenceRegistry.instance;
      registry.attach(first);
      registry.attach(second);
      registry.detach(first);
      expect(registry.source, same(second));
      registry.detach(second);
      expect(registry.source, isNull);
    });
  });

  group('a document as the reference source', () {
    test('it describes a selection with the namespaced refs and the live Mount', () {
      var mount = 'mount:first';
      final source = MuseDocumentSelectionReference(
        viewId: viewId,
        displayName: 'Design doc',
        activeMountRef: () => mount,
      );
      expect(source.resourceRef, museDocumentResourceRef(viewId));
      expect(source.viewRef, museDocumentViewRef(viewId));
      expect(source.mountRef, 'mount:first');

      final described = source.describeSelection(
        selectedText: 'the plan',
        startBlock: 3,
        endBlock: 8,
        startBlockRef: 'block-3',
        endBlockRef: 'block-8',
      );
      expect(described.displayName, 'Design doc');
      expect(described.excerpt, 'the plan');
      expect(described.mountRef, 'mount:first');

      // A Mount switch after the document opened reaches the next copy.
      mount = 'mount:second';
      expect(source.mountRef, 'mount:second');
      expect(
        source.describeSelection(selectedText: 'x', startBlock: 0, endBlock: 0).mountRef,
        'mount:second',
      );
    });

    test('a rename refreshes the display name, and a blank one is ignored', () {
      final source = MuseDocumentSelectionReference(
        viewId: viewId,
        displayName: 'Design doc',
        activeMountRef: () => null,
      );
      source.displayName = '  Roadmap  ';
      expect(source.displayName, 'Roadmap');
      source.displayName = '   ';
      expect(source.displayName, 'Roadmap');
    });
  });
}

class _FakeSource implements MuseSelectionReferenceSource {
  static const String resourceRefValue = 'resource.appflowy.document.fake';

  @override
  String get resourceRef => _FakeSource.resourceRefValue;

  @override
  String? get viewRef => 'view.appflowy.document.fake';

  @override
  String? get mountRef => 'mount:fake';

  @override
  String get displayName => 'Fake doc';

  @override
  MuseResourceReference describeSelection({
    required String selectedText,
    required int startBlock,
    required int endBlock,
    String? startBlockRef,
    String? endBlockRef,
    int? capturedAt,
  }) =>
      buildMuseResourceReference(
        resourceRef: resourceRef,
        viewRef: viewRef,
        mountRef: mountRef,
        displayName: displayName,
        selectedText: selectedText,
        startBlock: startBlock,
        endBlock: endBlock,
        startBlockRef: startBlockRef,
        endBlockRef: endBlockRef,
        capturedAt: capturedAt,
      );
}

class _InvalidSource extends _FakeSource {
  @override
  String get resourceRef => r'resource:C:\Users\me\notes.md';
}
