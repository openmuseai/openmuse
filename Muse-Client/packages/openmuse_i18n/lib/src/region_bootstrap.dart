import 'dart:async';
import 'package:url_launcher/url_launcher.dart';
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'controller.dart';
import 'generated_strings.dart';
import 'legal_consent_page.dart';

/// Entry-point boundary: no account, telemetry, update or plugin adapter is
/// constructed until a durable installed region has been resolved.
final class OpenMuseRegionBootstrap extends StatefulWidget {
  const OpenMuseRegionBootstrap({
    super.key,
    required this.store,
    required this.boot,
    this.loadRelease = loadInstalledRelease,
    this.requiresLegalConsent = true,
  });
  final Future<ConfigStore> Function(InstalledRelease) store;
  final Future<Widget> Function(DeploymentBundle, String? initialLanguage) boot;
  final Future<InstalledRelease> Function() loadRelease;

  /// Mac App Store apps must not present an additional license at launch.
  /// Skipping this page never records agreement on the user's behalf.
  final bool requiresLegalConsent;
  @override
  State<OpenMuseRegionBootstrap> createState() => _RegionBootstrapState();
}

final class _RegionBootstrapState extends State<OpenMuseRegionBootstrap> {
  final _locale = OpenMuseLocaleController(
    systemLocales: WidgetsBinding.instance.platformDispatcher.locales.map(
      (v) => v.toLanguageTag(),
    ),
  );
  RegionSelection? _selection;
  String? _chosen;
  Widget? _app;
  InstalledRegion? _consentRegion;
  bool _legalChecked = false;
  bool _busy = true, _failed = false;
  @override
  void initState() {
    super.initState();
    unawaited(_load());
  }

  Future<void> _load() async {
    try {
      final release = await widget.loadRelease();
      final selection = RegionSelection(release, await widget.store(release));
      final restored = await selection.restore();
      if (!mounted) return;
      _locale.applyConfiguration(
        catalog: _locale.catalog,
        defaultTag: release.defaultLocale,
        enabledLocales: _locale.catalog.locales.map((v) => v.tag),
      );
      _selection = selection;
      if (restored != null) {
        await _boot(restored);
        return;
      }
      _locale.selectTag(release.defaultLocale);
      setState(() => _busy = false);
    } catch (_) {
      if (mounted) {
        setState(() {
          _busy = false;
          _failed = true;
        });
      }
    }
  }

  Future<void> _boot(InstalledRegion region) async {
    if (widget.requiresLegalConsent &&
        !await _selection!.hasAcceptedLegal(region)) {
      if (mounted) {
        setState(() {
          _consentRegion = region;
          _busy = false;
        });
      }
      return;
    }
    if (!mounted) return;
    setState(() {
      _consentRegion = null;
      _busy = true;
    });
    final app = await widget.boot(
      region.bundle!,
      _locale.followsSystem ? null : _locale.tag,
    );
    if (mounted) setState(() => _app = app);
  }

  Future<void> _continue() async {
    setState(() {
      _busy = true;
      _failed = false;
    });
    try {
      final region = await _selection!.choose(_chosen!);
      await _boot(region);
    } catch (_) {
      if (mounted) {
        setState(() {
          _busy = false;
          _failed = true;
        });
      }
    }
  }

  Future<void> _openLegal(
    InstalledRegion region,
    LinkId link,
    String locale,
  ) async {
    try {
      final opened = await launchUrl(
        region.bundle!.snapshot.linkUri(link, locale: locale),
        mode: LaunchMode.externalApplication,
      );
      if (!opened && mounted) setState(() => _failed = true);
    } on Object {
      if (mounted) setState(() => _failed = true);
    }
  }

  Future<void> _acceptLegal(InstalledRegion region) async {
    setState(() {
      _busy = true;
      _failed = false;
    });
    try {
      await _selection!.acceptLegal(region);
      await _boot(region);
    } catch (_) {
      if (mounted) {
        setState(() {
          _busy = false;
          _failed = true;
        });
      }
    }
  }

  @override
  void dispose() {
    _locale.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_app case final app?) return app;
    // Restore persisted choices before presenting an interactive onboarding
    // page. A returning user sees only the neutral launch surface during I/O.
    if (_busy && _consentRegion == null) {
      return MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: openMuseOnboardingTheme(Brightness.light),
        darkTheme: openMuseOnboardingTheme(Brightness.dark),
        home: Scaffold(
          key: const ValueKey('bootstrap-loading'),
          body: Center(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Image.asset(
                  'packages/openmuse_i18n/assets/brand/openmuse-mark.png',
                  width: 56,
                  height: 56,
                  excludeFromSemantics: true,
                ),
                const SizedBox(height: 24),
                const SizedBox(
                  width: 20,
                  height: 20,
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
              ],
            ),
          ),
        ),
      );
    }
    if (_consentRegion case final region?) {
      // The legal documents follow the release market, independently of the
      // device language. No service boot happens before durable acceptance.
      final english = _selection!.release.market != 'china';
      final locale = english ? 'en' : 'zh-Hans';
      final legalStrings = OpenMuseStrings.forLocale(locale);
      return MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: openMuseOnboardingTheme(Brightness.light),
        darkTheme: openMuseOnboardingTheme(Brightness.dark),
        locale: Locale(english ? 'en' : 'zh'),
        supportedLocales: const [Locale('en'), Locale('zh')],
        localizationsDelegates: GlobalMaterialLocalizations.delegates,
        home: OpenMuseLegalConsentPage(
          strings: legalStrings,
          regionLabel: region.label(locale),
          checked: _legalChecked,
          busy: _busy,
          failed: _failed,
          onChecked: (value) => setState(() => _legalChecked = value),
          onTerms: () => unawaited(_openLegal(region, LinkId.terms, locale)),
          onPrivacy: () =>
              unawaited(_openLegal(region, LinkId.privacy, locale)),
          onAccept: () => unawaited(_acceptLegal(region)),
          onDecline: () => setState(() => _legalChecked = false),
          onChangeRegion: () => setState(() {
            _consentRegion = null;
            _chosen = region.id;
            _legalChecked = false;
            _failed = false;
          }),
        ),
      );
    }
    return ListenableBuilder(
      listenable: _locale,
      builder: (context, _) => OpenMuseLocaleScope(
        controller: _locale,
        child: MaterialApp(
          debugShowCheckedModeBanner: false,
          theme: openMuseOnboardingTheme(Brightness.light),
          darkTheme: openMuseOnboardingTheme(Brightness.dark),
          locale: _locale.locale,
          supportedLocales: _locale.supportedLocales,
          localizationsDelegates: [
            OpenMuseStringsDelegate(_locale.catalog),
            ...GlobalMaterialLocalizations.delegates,
          ],
          home: Scaffold(
            body: SafeArea(
              child: Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 560),
                  child: SingleChildScrollView(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(
                          _locale.strings.commonRegionTitle(),
                          style: Theme.of(context).textTheme.headlineMedium,
                        ),
                        const SizedBox(height: 16),
                        Text(_locale.strings.commonRegionDescription()),
                        const SizedBox(height: 24),
                        if (_busy) ...[
                          const Center(child: CircularProgressIndicator()),
                          const SizedBox(height: 16),
                          Text(_locale.strings.commonRegionLoading()),
                        ] else ...[
                          for (final region
                              in _selection?.release.regions ??
                                  <InstalledRegion>[])
                            ListTile(
                              key: ValueKey('region-${region.id}'),
                              enabled: region.available,
                              title: Text(region.label(_locale.tag)),
                              subtitle: region.available
                                  ? null
                                  : Text(
                                      _locale.strings.commonRegionUnavailable(),
                                    ),
                              trailing: Icon(
                                _chosen == region.id
                                    ? Icons.radio_button_checked
                                    : Icons.radio_button_off,
                              ),
                              onTap: region.available
                                  ? () => setState(() => _chosen = region.id)
                                  : null,
                            ),
                          if (_failed)
                            Padding(
                              padding: const EdgeInsets.symmetric(vertical: 12),
                              child: Text(_locale.strings.commonRegionFailed()),
                            ),
                          FilledButton(
                            key: const ValueKey('region-continue'),
                            onPressed: _chosen != null
                                ? _continue
                                : _failed
                                ? () {
                                    setState(() {
                                      _busy = true;
                                      _failed = false;
                                    });
                                    unawaited(_load());
                                  }
                                : null,
                            child: Text(
                              _failed
                                  ? _locale.strings.commonRetry()
                                  : _locale.strings.commonRegionContinue(),
                            ),
                          ),
                          DropdownButton<String>(
                            value: _locale.tag,
                            items: [
                              for (final option in _locale.options)
                                DropdownMenuItem(
                                  value: option.tag,
                                  child: Text(option.label),
                                ),
                            ],
                            onChanged: (value) {
                              if (value != null) _locale.selectTag(value);
                            },
                          ),
                        ],
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
