import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';

/// Last-resort provider for resources without a domain semantic provider.
///
/// It never claims that bytes are semantic objects. A changed digest produces
/// one explicit `binaryOnly` change so Host can show a truthful fallback and
/// offer a domain plugin through Reopen With… later.
final class MuseBinaryMetadataDiffProvider implements MuseSemanticDiffProvider {
  @override
  String get semanticProviderId => 'muse.diff.binary-metadata.v1';

  @override
  String get semanticProviderVersion => '1.0.0';

  @override
  bool supportsSemanticDiff(MuseResourceRef resource) => true;

  @override
  Future<MuseSemanticChangeSet> compareSemantic({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  }) async {
    final changed = base.contentDigest != target.contentDigest;
    return MuseSemanticChangeSet(
      schema: 'muse.diff.binary-metadata.changeset.v1',
      providerId: semanticProviderId,
      providerVersion: semanticProviderVersion,
      comparisonId: comparison.id,
      resource: comparison.resource,
      changes: changed
          ? [
              MuseSemanticChange(
                id: 'binary:${base.contentDigest}:${target.contentDigest}',
                kind: MuseUniversalChangeKind.modify,
                semanticPath: 'binary',
                label: 'Binary content changed',
                before: [
                  const MuseSemanticNodeAnchor(
                    nodeId: 'content',
                    nodeType: 'binary',
                  ),
                ],
                after: [
                  const MuseSemanticNodeAnchor(
                    nodeId: 'content',
                    nodeType: 'binary',
                  ),
                ],
                attribution: MuseChangeAttribution(actor: comparison.actor),
                properties: {
                  'beforeDigest': base.contentDigest,
                  'afterDigest': target.contentDigest,
                  'beforeBytes': base.byteLength,
                  'afterBytes': target.byteLength,
                  'mediaType': comparison.resource.mediaType,
                },
              ),
            ]
          : const [],
      quality: const MuseDiffQuality(
        kind: MuseDiffQualityKind.binaryOnly,
        reason: 'No domain semantic provider is installed',
      ),
      summary: {
        'changed': changed,
        'beforeBytes': base.byteLength,
        'afterBytes': target.byteLength,
      },
    );
  }
}
