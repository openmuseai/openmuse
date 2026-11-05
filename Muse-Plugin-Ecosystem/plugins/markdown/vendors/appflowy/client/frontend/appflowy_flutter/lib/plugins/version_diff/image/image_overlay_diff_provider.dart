import 'dart:typed_data';

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;

final class MuseImageOverlayDiffPayload {
  const MuseImageOverlayDiffPayload({
    required this.beforeBytes,
    required this.afterBytes,
    required this.beforeDigest,
    required this.afterDigest,
    required this.changed,
    this.beforeWidth,
    this.beforeHeight,
    this.afterWidth,
    this.afterHeight,
  });

  final Uint8List beforeBytes;
  final Uint8List afterBytes;
  final String beforeDigest;
  final String afterDigest;
  final bool changed;
  final int? beforeWidth;
  final int? beforeHeight;
  final int? afterWidth;
  final int? afterHeight;
}

/// Pixel-plane sample provider. It never reuses the text hunk protocol: the
/// semantic changeset only carries digests and pixel-region anchors.
final class MuseImageOverlayDiffProvider
    implements
        MuseDiffProvider<MuseImageOverlayDiffPayload>,
        MuseSemanticDiffProvider {
  static const supportedExtensions = {'png', 'jpg', 'jpeg', 'gif', 'webp'};

  static bool supportsFile(String path) => supportedExtensions.contains(
        p.extension(path).replaceFirst('.', '').toLowerCase(),
      );

  @override
  String get id => 'muse.diff.image-overlay.v1';

  @override
  String get semanticProviderId => id;

  @override
  String get semanticProviderVersion => '1.0.0';

  @override
  String get rendererType => 'muse.diff-viewer.image-overlay.v1';

  @override
  bool supports(MuseResourceRef resource) {
    return resource.mediaType.startsWith('image/') ||
        supportsFile(resource.locator);
  }

  @override
  bool supportsSemanticDiff(MuseResourceRef resource) => supports(resource);

  @override
  Future<MuseSemanticChangeSet> compareSemantic({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  }) async {
    final result = await compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: contentResolver,
    );
    return toSemanticChangeSet(
      comparison: comparison,
      payload: result.payload,
    );
  }

  @override
  Future<MuseResourceDiff<MuseImageOverlayDiffPayload>> compare({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  }) async {
    final beforeBytes = await contentResolver.resolve(base);
    final afterBytes = await contentResolver.resolve(target);
    final beforeHeader = MusePngHeader.tryParse(beforeBytes);
    final afterHeader = MusePngHeader.tryParse(afterBytes);
    final payload = MuseImageOverlayDiffPayload(
      beforeBytes: beforeBytes,
      afterBytes: afterBytes,
      beforeDigest: sha256.convert(beforeBytes).toString(),
      afterDigest: sha256.convert(afterBytes).toString(),
      changed: base.contentDigest != target.contentDigest,
      beforeWidth: beforeHeader?.width,
      beforeHeight: beforeHeader?.height,
      afterWidth: afterHeader?.width,
      afterHeight: afterHeader?.height,
    );
    return MuseResourceDiff(
      comparison: comparison,
      payload: payload,
      changeCount: payload.changed ? 1 : 0,
      summary:
          payload.changed ? 'Image pixels changed' : 'Images are identical',
    );
  }

  MuseSemanticChangeSet toSemanticChangeSet({
    required MuseComparison comparison,
    required MuseImageOverlayDiffPayload payload,
  }) {
    return MuseSemanticChangeSet(
      schema: 'muse.diff.image-overlay.changeset.v1',
      providerId: id,
      providerVersion: semanticProviderVersion,
      comparisonId: comparison.id,
      resource: comparison.resource,
      changes: payload.changed
          ? [
              MuseSemanticChange(
                id: 'image:${payload.beforeDigest}:${payload.afterDigest}',
                kind: MuseUniversalChangeKind.modify,
                semanticPath: 'image/pixels',
                label: 'Image overlay change',
                before: [
                  MusePageRegionAnchor(
                    page: 1,
                    left: 0,
                    top: 0,
                    right: (payload.beforeWidth ?? 1).toDouble(),
                    bottom: (payload.beforeHeight ?? 1).toDouble(),
                  ),
                ],
                after: [
                  MusePageRegionAnchor(
                    page: 1,
                    left: 0,
                    top: 0,
                    right: (payload.afterWidth ?? 1).toDouble(),
                    bottom: (payload.afterHeight ?? 1).toDouble(),
                  ),
                ],
                attribution: MuseChangeAttribution(actor: comparison.actor),
                properties: {
                  'beforeDigest': payload.beforeDigest,
                  'afterDigest': payload.afterDigest,
                  'beforeWidth': payload.beforeWidth,
                  'beforeHeight': payload.beforeHeight,
                  'afterWidth': payload.afterWidth,
                  'afterHeight': payload.afterHeight,
                },
              ),
            ]
          : const [],
      quality: const MuseDiffQuality(kind: MuseDiffQualityKind.exact),
      summary: {
        'changed': payload.changed,
        'beforeDigest': payload.beforeDigest,
        'afterDigest': payload.afterDigest,
      },
    );
  }
}

final class MusePngHeader {
  const MusePngHeader({required this.width, required this.height});

  final int width;
  final int height;

  static MusePngHeader? tryParse(Uint8List bytes) {
    if (bytes.length < 24) return null;
    const signature = [137, 80, 78, 71, 13, 10, 26, 10];
    for (var i = 0; i < signature.length; i++) {
      if (bytes[i] != signature[i]) return null;
    }
    if (bytes[12] != 73 ||
        bytes[13] != 72 ||
        bytes[14] != 68 ||
        bytes[15] != 82) {
      return null;
    }
    int dim(int offset) =>
        (bytes[offset] << 24) |
        (bytes[offset + 1] << 16) |
        (bytes[offset + 2] << 8) |
        bytes[offset + 3];
    return MusePngHeader(width: dim(16), height: dim(20));
  }
}
