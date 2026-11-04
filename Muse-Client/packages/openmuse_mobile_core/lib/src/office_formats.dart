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

final class OfficeEngineInspection {
  const OfficeEngineInspection({
    required this.format,
    required this.profile,
    required this.paragraphs,
    required this.capabilities,
  });
  final OfficeFormat format;
  final String profile;
  final List<String> paragraphs;
  final Set<OfficeCapability> capabilities;
}

/// Bytes are supplied through Resource handles by the caller. The engine does
/// not receive Workspace, S3, DSH, account, or credential capabilities.
abstract interface class OfficeEnginePort {
  String get abi;
  Future<OfficeEngineInspection> inspect(OfficeFormat format, List<int> bytes);
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  );
}

/// Dispatches each format to an independently packaged engine. Engines never
/// receive a fallback format: an absent binding fails closed.
final class MultiFormatOfficeEngine implements OfficeEnginePort {
  MultiFormatOfficeEngine(Map<OfficeFormat, OfficeEnginePort> engines)
    : _engines = Map.unmodifiable(engines) {
    if (_engines.isEmpty) throw ArgumentError('at least one Office engine');
  }

  final Map<OfficeFormat, OfficeEnginePort> _engines;

  @override
  String get abi => 'openmuse-office-router@1';

  Set<OfficeFormat> get formats => Set.unmodifiable(_engines.keys);

  @override
  Future<OfficeEngineInspection> inspect(OfficeFormat format, List<int> bytes) {
    final engine = _engines[format];
    if (engine == null) throw StateError('Office format engine unavailable');
    return engine.inspect(format, bytes);
  }

  @override
  Future<List<int>> exportSimple(
    OfficeFormat format,
    List<int> originalBytes,
    List<String> paragraphs,
  ) {
    final engine = _engines[format];
    if (engine == null) throw StateError('Office format engine unavailable');
    return engine.exportSimple(format, originalBytes, paragraphs);
  }
}

final class OfficeFormatRegistry {
  const OfficeFormatRegistry(this.artifacts);
  final List<OfficeArtifact> artifacts;
  Set<OfficeCapability> capabilities(
    OfficeFormat format,
    String platform, {
    String? abi,
  }) {
    final matches = artifacts.where(
      (item) =>
          item.format == format &&
          item.platform == platform &&
          (abi == null || item.abi == abi) &&
          RegExp(r'^sha256:[0-9a-f]{64}$').hasMatch(item.digest),
    );
    if (matches.length != 1) return const {};
    final artifact = matches.single;
    final result = {...artifact.capabilities};
    if (!artifact.originalFormatExportVerified) {
      result.remove(OfficeCapability.edit);
      result.remove(OfficeCapability.export);
    }
    return result;
  }

  Set<OfficeCapability> admittedDocumentCapabilities({
    required OfficeEngineInspection inspection,
    required String platform,
    required String abi,
  }) => capabilities(
    inspection.format,
    platform,
    abi: abi,
  ).intersection(inspection.capabilities);

  void validateExportCommit({
    required String expectedRevision,
    required String currentRevision,
    required List<int> bytes,
  }) {
    if (expectedRevision != currentRevision || bytes.isEmpty) {
      throw StateError('office export conflict');
    }
  }
}
