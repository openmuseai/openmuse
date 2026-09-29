import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  test('unimplemented and unverified capabilities are not advertised', () {
    const registry = OfficeFormatRegistry([
      OfficeArtifact(
        format: OfficeFormat.word,
        platform: 'android-arm64',
        abi: 'jni@1',
        digest: 'sha256:a',
        capabilities: {
          OfficeCapability.view,
          OfficeCapability.edit,
          OfficeCapability.export,
        },
        originalFormatExportVerified: false,
      ),
    ]);
    expect(registry.capabilities(OfficeFormat.word, 'android-arm64'), {
      OfficeCapability.view,
    });
    expect(registry.capabilities(OfficeFormat.sheet, 'android-arm64'), isEmpty);
  });
  test('export uses expected revision CAS and nonempty bytes', () {
    const registry = OfficeFormatRegistry([]);
    expect(
      registry.commitExport(
        expectedRevision: 'r1',
        currentRevision: 'r1',
        bytes: [1, 2],
      ),
      'revision:2',
    );
    expect(
      () => registry.commitExport(
        expectedRevision: 'r1',
        currentRevision: 'r2',
        bytes: [1],
      ),
      throwsStateError,
    );
  });
}
