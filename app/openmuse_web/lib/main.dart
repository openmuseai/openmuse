import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'auth/browser_auth.dart';
import 'auth/web_deployment.dart';
import 'auth/browser_locale.dart';
import 'auth/browser_config_store.dart';
import 'workbench/web_session_page.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  InstalledRelease? webRelease;
  runApp(
    OpenMuseRegionBootstrap(
      loadRelease: () async => webRelease = await loadWebInstalledRelease(),
      store: (release) async => BrowserConfigStore('region-${release.id}'),
      boot: (bundle, language) => bootOpenMuseWeb(
        bundle,
        language,
        regionLabels: webRelease!.regions
            .firstWhere(
              (value) =>
                  value.id == bundle.snapshot.json['scope']['dataRegion'],
            )
            .labels,
      ),
    ),
  );
}

Future<Widget> bootOpenMuseWeb(
  DeploymentBundle bundle,
  String? initialLanguage, {
  Map<String, String> regionLabels = const {},
}) async {
  final config = bundle.snapshot;
  final locale = OpenMuseUiLocale(
    defaultTag: config.defaultLocale,
    enabledLocales: config.enabledLocales,
    aliases: config.localeAliases,
    systemLocales: WidgetsBinding.instance.platformDispatcher.locales.map(
      (v) => v.toLanguageTag(),
    ),
    preferences: BrowserLocalePreferences(
      'openmuse.locale.${config.storageIdentity}',
    ),
  );
  await locale.restore();
  final requestedLanguage = initialLanguage ?? Uri.base.queryParameters['lang'];
  if (requestedLanguage != null &&
      config.enabledLocales.contains(requestedLanguage)) {
    locale.selectTag(requestedLanguage);
    await locale.saved;
  }
  const build = int.fromEnvironment('OPENMUSE_BUILD_NUMBER');
  final runtime = OpenMuseClientRuntime(
    bundle: bundle,
    transport: createConfigTransport(),
    store: BrowserConfigStore(config.storageIdentity),
    build: build,
    locale: locale,
  );
  await runtime.bootstrap();
  runtime.start();
  syncBrowserLocale(locale);
  locale.addListener(() => syncBrowserLocale(locale));
  return OpenMuseRuntimeHost(
    runtime: runtime,
    child: OpenMuseWebApp(
      config: runtime.client.current,
      localeController: locale,
      runtime: runtime,
      regionLabels: regionLabels,
    ),
  );
}

final class OpenMuseWebApp extends StatefulWidget {
  const OpenMuseWebApp({
    super.key,
    this.authentication,
    this.config,
    this.localeController,
    this.runtime,
    this.regionLabels = const {},
  });

  final Map<String, String> regionLabels;
  final RuntimeConfig? config;
  final OpenMuseUiLocale? localeController;
  final OpenMuseClientRuntime? runtime;

  /// Injection point for widget and integration tests.
  final GoTrueAuthenticationController? authentication;

  @override
  State<OpenMuseWebApp> createState() => _OpenMuseWebAppState();
}

final class _OpenMuseWebAppState extends State<OpenMuseWebApp>
    with WidgetsBindingObserver {
  late final _locale = widget.localeController ?? OpenMuseUiLocale();
  BrowserGoTrueProvider? _provider;
  late final GoTrueAuthenticationController _authentication;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _provider = widget.authentication == null
        ? BrowserGoTrueProvider(
            origin: widget.config?.webServiceUri(ServiceId.auth, Uri.base),
            networkGuard: widget.runtime?.checkNetwork,
          )
        : null;
    _authentication =
        widget.authentication ??
        GoTrueAuthenticationController(
          networkGuard: widget.runtime?.checkNetwork,
          provider: _provider!,
          store: BrowserAuthSessionStore(
            key: widget.config?.webAuthSessionStorageKey,
          ),
        );
    unawaited(_authentication.restore());
  }

  @override
  void didChangeLocales(List<Locale>? locales) => _locale.updateSystemLocales(
    (locales ?? []).map((v) => v.toLanguageTag()),
  );

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _locale.dispose();
    _provider?.close();
    if (widget.authentication == null) _authentication.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: _locale,
    builder: (context, _) => OpenMuseUiLocaleScope(
      controller: _locale,
      child: MaterialApp(
        locale: _locale.locale,
        supportedLocales: _locale.supportedLocales,
        builder: (context, child) =>
            OpenMuseRuntimeNotice(child: child ?? const SizedBox.shrink()),
        localizationsDelegates: [
          OpenMuseStringsDelegate(_locale.catalog),
          ...OpenMuseAuthLocalizations.localizationsDelegates,
        ],
        debugShowCheckedModeBanner: false,
        title: 'OpenMuse',
        theme: buildOpenMuseTheme(),
        initialRoute: widget.config?.navigation?.entry.path,
        onGenerateRoute: (settings) {
          final navigation =
              (RuntimeConfigScope.maybeOf(context) ?? widget.config)
                  ?.navigation;
          final route = navigation?.resolve(settings.name ?? '');
          return MaterialPageRoute<void>(
            settings: settings,
            builder: (context) {
              if (navigation != null && route == null) {
                return Scaffold(
                  body: Center(
                    child: Text(_locale.strings.commonRouteUnavailable()),
                  ),
                );
              }
              return _sessionGate(context);
            },
          );
        },
      ),
    ),
  );

  Widget _sessionGate(BuildContext context) => AnimatedBuilder(
    animation: _authentication,
    builder: (context, _) {
      final snapshot = _authentication.snapshot;
      if (snapshot.isAuthenticated) {
        return WebSessionPage(
          authentication: _authentication,
          config: widget.config,
        );
      }
      if (snapshot.phase == OpenMuseAuthenticationPhase.restoring) {
        return const Scaffold(body: Center(child: CircularProgressIndicator()));
      }
      return OpenMuseLoginScreen(
        authentication: _authentication,
        cloudLabel:
            widget.regionLabels[_locale.tag] ??
            widget.regionLabels.values.firstOrNull ??
            'OpenMuse',
        termsUri: (RuntimeConfigScope.maybeOf(context) ?? widget.config)
            ?.linkUri(LinkId.terms, locale: _locale.tag),
        privacyUri: (RuntimeConfigScope.maybeOf(context) ?? widget.config)
            ?.linkUri(LinkId.privacy, locale: _locale.tag),
      );
    },
  );
}
