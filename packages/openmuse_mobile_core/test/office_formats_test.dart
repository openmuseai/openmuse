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
}
