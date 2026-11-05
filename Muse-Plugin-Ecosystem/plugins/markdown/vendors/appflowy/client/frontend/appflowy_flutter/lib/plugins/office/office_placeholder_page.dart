import 'package:appflowy/plugins/office/office_manifest.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';

/// Shown when the Folder slot exists but the engine Facet is not bound.
class OfficePlaceholderPage extends StatelessWidget {
  const OfficePlaceholderPage({super.key, required this.manifest});

  final OfficeManifest manifest;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return ColoredBox(
      color: theme.colorScheme.surface,
      child: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 480),
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                FlowyText(
                  '${manifest.menuName} plugin slot',
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                ),
                const VSpace(8),
                FlowyText(
                  manifest.engineBound
                      ? 'Engine failed to open this page.'
                      : 'Muse Host reserved layout=${manifest.layout.value} '
                          '(${manifest.pluginId}). The ${manifest.menuName} '
                          'engine is not bound yet — download or ship the '
                          'vendor Facet to edit. Blob stays under '
                          '${manifest.blobSubdir}/, not Document CRDT.',
                  lineHeight: 1.4,
                  maxLines: 8,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
