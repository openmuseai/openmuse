/// Signed product policy for allocating columns around the document viewport.
/// The shell owns only presentation. User panel preferences remain independent
/// from temporary width constraints.
final class PdfShellLayoutPolicy {
  PdfShellLayoutPolicy.fromJson(Map<String, dynamic>? json)
    : mobileBelow = (json?['mobileBelow'] as num?)?.toDouble() ?? 600,
      compactBelow = (json?['compactBelow'] as num?)?.toDouble() ?? 900,
      minDocumentWidth = (json?['minDocumentWidth'] as num?)?.toDouble() ?? 400;

  final double mobileBelow;
  final double compactBelow;
  final double minDocumentWidth;

  PdfShellColumns resolve({
    required double width,
    required double aiPreferredWidth,
    required double aiMinimumWidth,
    required double aiRailWidth,
    required double sidebarWidth,
    required bool aiRequested,
    required bool aiExplicit,
    required bool sidebarRequested,
  }) {
    final mobile = width < mobileBelow;
    if (mobile) {
      return PdfShellColumns(
        mobile: true,
        compact: true,
        aiPanel: false,
        sidebar: false,
        aiWidth: aiPreferredWidth,
      );
    }
    final availableForAi = width - minDocumentWidth;
    final canFitAi = availableForAi >= aiMinimumWidth;
    final aiWidth = canFitAi
        ? aiPreferredWidth.clamp(aiMinimumWidth, availableForAi).toDouble()
        : aiPreferredWidth;
    final canFitBoth =
        width - aiWidth - (sidebarRequested ? sidebarWidth : 0) >=
        minDocumentWidth;
    final aiPanel = aiRequested && canFitAi && (canFitBoth || aiExplicit);
    final sidebar =
        sidebarRequested &&
        width - (aiPanel ? aiWidth : aiRailWidth) - sidebarWidth >=
            minDocumentWidth;
    return PdfShellColumns(
      mobile: false,
      compact: width < compactBelow,
      aiPanel: aiPanel,
      sidebar: sidebar,
      aiWidth: aiWidth,
    );
  }
}

final class PdfShellColumns {
  const PdfShellColumns({
    required this.mobile,
    required this.compact,
    required this.aiPanel,
    required this.sidebar,
    required this.aiWidth,
  });

  final bool mobile;
  final bool compact;
  final bool aiPanel;
  final bool sidebar;
  final double aiWidth;
}
