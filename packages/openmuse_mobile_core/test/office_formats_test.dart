import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  test('unimplemented and unverified capabilities are not advertised', () {
    const registry = OfficeFormatRegistry([
      OfficeArtifact(
        format: OfficeFormat.word,
        platform: 'android-arm64',
        abi: 'openmuse-docx-ffi@1',
        digest:
            'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
        capabilities: {
          OfficeCapability.view,
          OfficeCapability.edit,
          OfficeCapability.export,
        },
        originalFormatExportVerified: false,
      ),
    ]);
    expect(
      registry.capabilities(
        OfficeFormat.word,
        'android-arm64',
        abi: 'openmuse-docx-ffi@1',
      ),
      {OfficeCapability.view},
    );
    expect(registry.capabilities(OfficeFormat.sheet, 'android-arm64'), isEmpty);
  });
  test('export validation defers revision creation to Resource Authority', () {
    const registry = OfficeFormatRegistry([]);
    registry.validateExportCommit(
      expectedRevision: 'r1',
      currentRevision: 'r1',
      bytes: [1, 2],
    );
    expect(
      () => registry.validateExportCommit(
        expectedRevision: 'r1',
        currentRevision: 'r2',
        bytes: [1],
      ),
      throwsStateError,
    );
  });

  test(
    'document profile capabilities are intersected with admitted artifact',
    () {
      const registry = OfficeFormatRegistry([
        OfficeArtifact(
          format: OfficeFormat.word,
          platform: 'android-arm64',
          abi: 'openmuse-docx-ffi@1',
          digest:
              'sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
          capabilities: {
            OfficeCapability.view,
            OfficeCapability.edit,
            OfficeCapability.export,
          },
          originalFormatExportVerified: true,
        ),
      ]);
      expect(
        registry.admittedDocumentCapabilities(
          inspection: const OfficeEngineInspection(
            format: OfficeFormat.word,
            profile: 'view-only',
            paragraphs: ['cell'],
            capabilities: {OfficeCapability.view},
          ),
          platform: 'android-arm64',
          abi: 'openmuse-docx-ffi@1',
        ),
        {OfficeCapability.view},
      );
      expect(
        registry.capabilities(
          OfficeFormat.word,
          'android-arm64',
          abi: 'wrong-abi',
        ),
        isEmpty,
      );
    },
  );

  test('multi-format engine dispatches exactly and never falls back', () async {
    final word = _FormatEngine(OfficeFormat.word);
    final sheet = _FormatEngine(OfficeFormat.sheet);
    final router = MultiFormatOfficeEngine({
      OfficeFormat.word: word,
      OfficeFormat.sheet: sheet,
    });
    final inspected = await router.inspect(OfficeFormat.sheet, const [1]);
    expect(inspected.format, OfficeFormat.sheet);
    expect(sheet.inspections, 1);
    expect(word.inspections, 0);
    expect(() => router.inspect(OfficeFormat.pdf, const [1]), throwsStateError);
  });
}

final class _FormatEngine implements OfficeEnginePort {
  _FormatEngine(this.format);
  final OfficeFormat format;
  int inspections = 0;

  @override
  String get abi => 'test@1';

  @override
  Future<OfficeEngineInspection> inspect(
    OfficeFormat requested,
    List<int> bytes,
  ) async {
    inspections++;
    return OfficeEngineInspection(
      format: format,
      profile: 'view-only',
      paragraphs: const ['value'],
      capabilities: const {OfficeCapability.view},
    );
  }

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) => throw UnsupportedError('view only');
}
