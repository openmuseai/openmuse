import 'package:flutter/material.dart';

import 'generated_strings.dart';

/// Bundled first-run UI. It creates no service clients and loads no remote assets.
ThemeData openMuseOnboardingTheme(Brightness brightness) {
  final dark = brightness == Brightness.dark;
  final primary = dark ? const Color(0xFF8FAFF5) : const Color(0xFF3867C8);
  final colors =
      ColorScheme.fromSeed(seedColor: primary, brightness: brightness).copyWith(
        primary: primary,
        onPrimary: dark ? const Color(0xFF142544) : Colors.white,
        surface: dark ? const Color(0xFF242830) : Colors.white,
        onSurface: dark ? const Color(0xFFEDF0F5) : const Color(0xFF20252D),
        onSurfaceVariant: dark
            ? const Color(0xFFAAB2C0)
            : const Color(0xFF5E6878),
        outlineVariant: dark
            ? const Color(0xFF3A414D)
            : const Color(0xFFE0E4EB),
      );
  return ThemeData(
    useMaterial3: true,
    colorScheme: colors,
    scaffoldBackgroundColor: dark
        ? const Color(0xFF191C21)
        : const Color(0xFFF6F7F9),
    textTheme: ThemeData(brightness: brightness).textTheme.apply(
      bodyColor: colors.onSurface,
      displayColor: colors.onSurface,
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        minimumSize: const Size(48, 54),
        padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
        textStyle: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(14)),
      ),
    ),
    textButtonTheme: TextButtonThemeData(
      style: TextButton.styleFrom(
        minimumSize: const Size(48, 48),
        textStyle: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
      ),
    ),
  );
}

final class OpenMuseLegalConsentPage extends StatelessWidget {
  const OpenMuseLegalConsentPage({
    super.key,
    required this.strings,
    required this.regionLabel,
    required this.checked,
    required this.busy,
    required this.failed,
    required this.onChecked,
    required this.onTerms,
    required this.onPrivacy,
    required this.onAccept,
    required this.onDecline,
    required this.onChangeRegion,
  });

  final OpenMuseStrings strings;
  final String regionLabel;
  final bool checked, busy, failed;
  final ValueChanged<bool> onChecked;
  final VoidCallback onTerms, onPrivacy, onAccept, onDecline, onChangeRegion;

  @override
  Widget build(BuildContext context) => Scaffold(
    body: SafeArea(
      child: LayoutBuilder(
        builder: (context, constraints) {
          final wide = constraints.maxWidth >= 960;
          if (wide) {
            return SingleChildScrollView(
              child: ConstrainedBox(
                constraints: BoxConstraints(minHeight: constraints.maxHeight),
                child: Center(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 56,
                      vertical: 48,
                    ),
                    child: ConstrainedBox(
                      constraints: const BoxConstraints(maxWidth: 1100),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(child: _introduction(context)),
                          const SizedBox(width: 80),
                          Expanded(
                            flex: 1,
                            child: Material(
                              color: Theme.of(context).colorScheme.surface,
                              shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(24),
                                side: BorderSide(
                                  color: Theme.of(
                                    context,
                                  ).colorScheme.outlineVariant,
                                ),
                              ),
                              child: Padding(
                                padding: const EdgeInsets.all(32),
                                child: Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.stretch,
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    _documents(context, wide: true),
                                    const SizedBox(height: 24),
                                    _decision(context),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            );
          }
          if (MediaQuery.textScalerOf(context).scale(14) > 21 ||
              constraints.maxHeight < 560) {
            // Large accessibility text must not let a fixed action area cover
            // the documents. In this mode the entire page scrolls together.
            return Center(
              child: ConstrainedBox(
                constraints: const BoxConstraints(maxWidth: 600),
                child: Material(
                  color: Theme.of(context).colorScheme.surface,
                  child: SingleChildScrollView(
                    child: Padding(
                      padding: const EdgeInsets.all(24),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          const _Brand(),
                          const SizedBox(height: 34),
                          _documents(context, wide: false),
                          const SizedBox(height: 24),
                          _decision(context),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
            );
          }
          // Actions remain reachable on short phones; the documents scroll
          // independently instead of pushing the consent controls off-screen.
          return Center(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 600),
              child: Material(
                color: Theme.of(context).colorScheme.surface,
                child: Column(
                  children: [
                    Expanded(
                      child: SingleChildScrollView(
                        padding: const EdgeInsets.fromLTRB(24, 28, 24, 24),
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            const _Brand(),
                            const SizedBox(height: 34),
                            _documents(context, wide: false),
                          ],
                        ),
                      ),
                    ),
                    DecoratedBox(
                      decoration: BoxDecoration(
                        border: Border(
                          top: BorderSide(
                            color: Theme.of(context).colorScheme.outlineVariant,
                          ),
                        ),
                      ),
                      child: Padding(
                        padding: const EdgeInsets.fromLTRB(24, 12, 24, 12),
                        child: _decision(context),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          );
        },
      ),
    ),
  );

  Widget _introduction(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const _Brand(),
        const SizedBox(height: 56),
        Text(
          strings.commonLegalTitle(),
          style: const TextStyle(
            fontSize: 40,
            height: 1.15,
            letterSpacing: -1.3,
            fontWeight: FontWeight.w600,
          ),
        ),
        const SizedBox(height: 18),
        Text(
          strings.commonLegalWorkspace(),
          style: TextStyle(
            fontSize: 18,
            height: 1.5,
            color: colors.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 40),
        for (final item in [
          (Icons.description_outlined, strings.commonLegalLocal()),
          (Icons.devices_outlined, strings.commonLegalDevices()),
          (Icons.tune_outlined, strings.commonLegalPermissions()),
        ])
          Padding(
            padding: const EdgeInsets.only(bottom: 20),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(item.$1, size: 20, color: colors.onSurfaceVariant),
                const SizedBox(width: 14),
                Expanded(
                  child: Text(
                    item.$2,
                    style: TextStyle(
                      fontSize: 15,
                      height: 1.4,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                ),
              ],
            ),
          ),
      ],
    );
  }

  Widget _documents(BuildContext context, {required bool wide}) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(
          wide ? strings.commonLegalIntro() : strings.commonLegalTitle(),
          style: const TextStyle(
            fontSize: 28,
            height: 1.2,
            fontWeight: FontWeight.w600,
            letterSpacing: -.6,
          ),
        ),
        const SizedBox(height: 14),
        Text(
          strings.commonLegalDescription(),
          style: TextStyle(
            fontSize: 15,
            height: 1.55,
            color: colors.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 24),
        Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    strings.commonLegalRegion(),
                    style: TextStyle(
                      fontSize: 12,
                      height: 1.4,
                      color: colors.onSurfaceVariant,
                    ),
                  ),
                  const SizedBox(height: 3),
                  Text(
                    regionLabel,
                    style: const TextStyle(
                      fontSize: 15,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),
            TextButton(
              key: const ValueKey('legal-change-region'),
              onPressed: busy ? null : onChangeRegion,
              child: Text(strings.commonLegalChangeRegion()),
            ),
          ],
        ),
        const SizedBox(height: 20),
        DecoratedBox(
          decoration: BoxDecoration(
            border: Border.all(color: colors.outlineVariant),
            borderRadius: BorderRadius.circular(18),
          ),
          child: Column(
            children: [
              _DocumentLink(
                key: const ValueKey('legal-terms'),
                title: strings.commonLegalTerms(),
                detail: strings.commonLegalTermsDetail(),
                icon: Icons.description_outlined,
                onTap: busy ? null : onTerms,
              ),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 18),
                child: Divider(height: 1, color: colors.outlineVariant),
              ),
              _DocumentLink(
                key: const ValueKey('legal-privacy'),
                title: strings.commonLegalPrivacy(),
                detail: strings.commonLegalPrivacyDetail(),
                icon: Icons.privacy_tip_outlined,
                onTap: busy ? null : onPrivacy,
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _decision(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        CheckboxListTile(
          key: const ValueKey('legal-checkbox'),
          value: checked,
          contentPadding: EdgeInsets.zero,
          controlAffinity: ListTileControlAffinity.leading,
          checkboxShape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(5),
          ),
          title: Text(
            strings.commonLegalAcknowledge(),
            style: const TextStyle(fontSize: 14, height: 1.45),
          ),
          onChanged: busy ? null : (value) => onChecked(value ?? false),
        ),
        if (failed)
          Padding(
            padding: const EdgeInsets.only(top: 4, bottom: 12),
            child: Semantics(
              liveRegion: true,
              child: Text(
                strings.commonLegalFailed(),
                style: TextStyle(
                  color: colors.error,
                  fontSize: 14,
                  height: 1.4,
                ),
              ),
            ),
          ),
        const SizedBox(height: 10),
        FilledButton(
          key: const ValueKey('legal-accept'),
          onPressed: checked && !busy ? onAccept : null,
          child: Text(
            busy
                ? strings.commonLegalContinuing()
                : strings.commonLegalAccept(),
          ),
        ),
        if (busy) const LinearProgressIndicator(minHeight: 2),
        const SizedBox(height: 2),
        TextButton(
          key: const ValueKey('legal-decline'),
          onPressed: busy ? null : onDecline,
          style: TextButton.styleFrom(foregroundColor: colors.onSurfaceVariant),
          child: Text(strings.commonLegalDecline()),
        ),
        Text(
          strings.commonLegalDecision(),
          textAlign: TextAlign.center,
          style: TextStyle(
            fontSize: 12,
            height: 1.4,
            color: colors.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}

final class _Brand extends StatelessWidget {
  const _Brand();
  @override
  Widget build(BuildContext context) => Row(
    children: [
      ClipRRect(
        borderRadius: BorderRadius.circular(10),
        child: Image.asset(
          'packages/openmuse_i18n/assets/brand/openmuse-mark.png',
          width: 36,
          height: 36,
          excludeFromSemantics: true,
        ),
      ),
      const SizedBox(width: 11),
      const Flexible(
        child: Text(
          'OpenMuse',
          style: TextStyle(
            fontSize: 21,
            fontWeight: FontWeight.w600,
            letterSpacing: -.6,
          ),
        ),
      ),
    ],
  );
}

final class _DocumentLink extends StatelessWidget {
  const _DocumentLink({
    super.key,
    required this.title,
    required this.detail,
    required this.icon,
    required this.onTap,
  });
  final String title, detail;
  final IconData icon;
  final VoidCallback? onTap;
  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Semantics(
      link: true,
      child: Material(
        type: MaterialType.transparency,
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(18),
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 19),
            child: Row(
              children: [
                Icon(icon, size: 22, color: colors.primary),
                const SizedBox(width: 14),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        title,
                        style: const TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        detail,
                        style: TextStyle(
                          fontSize: 12,
                          height: 1.4,
                          color: colors.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 12),
                Icon(
                  Icons.north_east_rounded,
                  size: 17,
                  color: colors.onSurfaceVariant,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
