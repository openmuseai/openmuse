enum OfficeFormat { word, sheet, slides, pdf }

enum OfficeCapability { view, edit, export }

final class OfficeArtifact {
  const OfficeArtifact({
    required this.format,
    required this.platform,
    required this.abi,
    required this.digest,
    required this.capabilities,
    required this.originalFormatExportVerified,
  });
  final OfficeFormat format;
  final String platform, abi, digest;
  final Set<OfficeCapability> capabilities;
  final bool originalFormatExportVerified;
}

final class OfficeFormatRegistry {
  const OfficeFormatRegistry(this.artifacts);
  final List<OfficeArtifact> artifacts;
  Set<OfficeCapability> capabilities(OfficeFormat format, String platform) {
    final matches = artifacts.where(
      (item) =>
          item.format == format &&
          item.platform == platform &&
          item.digest.startsWith('sha256:'),
    );
    if (matches.isEmpty) return const {};
    final artifact = matches.single;
    final result = {...artifact.capabilities};
    if (!artifact.originalFormatExportVerified) {
      result.remove(OfficeCapability.edit);
      result.remove(OfficeCapability.export);
    }
    return result;
  }

  String commitExport({
    required String expectedRevision,
    required String currentRevision,
    required List<int> bytes,
  }) {
    if (expectedRevision != currentRevision || bytes.isEmpty)
      throw StateError('office export conflict');
    return 'revision:${bytes.length}';
  }
}
