import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

import 'catalog.dart';
import 'controller.dart';

/// Owns one installation's authenticated runtime data and presentation state.
/// Network endpoint changes are staged for a new session rather than rebinding
/// an existing bearer token to a newly selected service.
final class OpenMuseClientRuntime extends ChangeNotifier {
  OpenMuseClientRuntime({
    required DeploymentBundle bundle,
    required ConfigTransport transport,
    required ConfigStore store,
    required int build,
    required this.locale,
    Set<String> capabilities = const {
      'config.v1',
      'messages.v1',
      'navigation.v1',
      'messages.dynamic.v1',
      'settings.v1',
      'plugin-messages.v1',
    },
    DateTime Function()? clock,
    int? cohort,
  }) : _baseline = locale.catalog {
    client = RuntimeConfigClient(
      bundle: bundle,
      transport: transport,
      store: store,
      build: build,
      capabilities: capabilities,
      clock: clock,
      cohort: cohort,
      validateResources: (config, resources) {
        _catalog(config, resources);
      },
    );
    _network = bundle.snapshot;
    _presentation = bundle.snapshot;
    locale.ensureLocale = (tag) async {
      await client.loadLocaleResources(tag);
      if (!_disposed) _apply();
    };
  }

  final OpenMuseLocaleController locale;
  final OpenMuseCatalog _baseline;
  late final RuntimeConfigClient client;
  late RuntimeConfig _network;
  late RuntimeConfig _presentation;
  Timer? _timer;
  Timer? _expiryTimer;
  bool _disposed = false;
  bool _attemptedRefresh = false;
  Duration? _pollOverride;
  Duration get refreshInterval =>
      _pollOverride ??
      Duration(seconds: _presentation.settings.refreshIntervalSeconds);
  int _catalogRevision = 0;
  Object? lastError;
  RuntimeConfig? pendingRestart;
  RuntimeConfig get presentation => _presentation;
  RuntimeConfig get network {
    client.currentForNetwork;
    if (_network.expiresAt case final expiry?) {
      if (!expiry.isAfter(client.effectiveTime)) {
        throw StateError('Session configuration expired; reconnect required');
      }
    }
    return _network;
  }

  bool get networkExpired =>
      client.networkUnavailable ||
      client.networkConfigExpired ||
      (_network.expiresAt != null &&
          !_network.expiresAt!.isAfter(client.effectiveTime));

  /// All credential-bearing adapters call this immediately before I/O.
  void checkNetwork() {
    network;
  }

  OpenMuseCatalog _catalog(
    RuntimeConfig config,
    Map<String, List<int>> resources,
  ) {
    final packs = <List<int>>[];
    final namespaces = <String>{};
    for (final raw in (config.json['resources'] as List? ?? const [])) {
      final entry = raw as Map;
      if (entry['kind'] == 'pluginMessages') {
        final bytes = resources[entry['id']];
        if (bytes == null && entry['delivery'] == 'onDemand') continue;
        if (bytes == null) {
          throw const FormatException('Missing plugin messages');
        }
        final pack = jsonDecode(utf8.decode(bytes));
        if (pack is! Map ||
            pack['schemaVersion'] != 1 ||
            pack['sourceRevision'] is! int ||
            pack['sourceRevision'] < 1 ||
            pack['locale'] != entry['locale'] ||
            pack['namespace'] != entry['namespace'] ||
            !(pack['namespace'] as String).startsWith('plugin.') ||
            pack['contractHash'] is! String ||
            !RegExp(
              r'^[a-f0-9]{64}$',
            ).hasMatch(pack['contractHash'] as String) ||
            pack['messages'] is! Map ||
            (pack['messages'] as Map).length > 4000 ||
            (pack['messages'] as Map).values.any(
              (v) => v is! String || v.length > 16000,
            )) {
          throw const FormatException('Invalid plugin message envelope');
        }
        continue;
      }
      if (entry['kind'] != 'messages') continue;
      if (!namespaces.add('${entry['locale']}/${entry['namespace']}')) {
        throw const FormatException('Duplicate language namespace resource');
      }
      final bytes = resources[entry['id']];
      if (bytes == null && entry['delivery'] == 'onDemand') continue;
      if (bytes == null) {
        throw const FormatException('Required messages missing');
      }
      final pack = jsonDecode(utf8.decode(bytes));
      if (pack is! Map ||
          pack['locale'] != entry['locale'] ||
          pack['namespace'] != entry['namespace']) {
        throw const FormatException(
          'Language manifest does not match resource',
        );
      }
      packs.add(bytes);
    }
    final refs =
        (config.json['localization'] as Map)['bundleRefs'] as List? ?? const [];
    for (final ref in refs) {
      if (!(config.json['resources'] as List? ?? const []).any(
        (r) =>
            r['id'] == ref &&
            {'messages', 'pluginMessages'}.contains(r['kind']),
      )) {
        throw const FormatException(
          'Language reference is not a message resource',
        );
      }
    }
    final directory =
        (config.json['localization'] as Map)['localeCatalog'] as List? ??
        const [];
    final catalog = _baseline.extendLocales(directory).overlay(packs);
    if (!catalog.locales
            .map((v) => v.tag)
            .toSet()
            .containsAll(config.enabledLocales) ||
        config.enabledLocales.any(
          (tag) => !GlobalMaterialLocalizations.delegate.isSupported(
            localeFromTag(tag),
          ),
        )) {
      throw const FormatException(
        'Language exceeds installed rendering capability',
      );
    }
    if (!catalog.hasTranslations(config.defaultLocale)) {
      throw const FormatException('Default language is not ready');
    }
    return catalog;
  }

  Future<void> preparePluginMessages(String tag) async {
    if (!(client.current.json['resources'] as List? ?? const []).any(
      (r) =>
          r['kind'] == 'pluginMessages' &&
          r['locale'] == tag &&
          !client.resources.containsKey(r['id']),
    )) {
      return;
    }
    await client.loadLocaleResources(tag);
    if (!_disposed) _apply();
  }

  Future<void> bootstrap() async {
    await restore();
    if (client.bundle.discoveryUris.isNotEmpty) await refresh();
    await locale.ensurePreferredLocale();
  }

  Future<void> restore() async {
    await client.restore();
    lastError = client.restoreError;
    if (_disposed) return;
    // No account adapters have been constructed yet; cached endpoints become
    // the initial session endpoints after installed policy verification.
    _network = client.current;
    _apply();
  }

  Future<void> refresh() async {
    _attemptedRefresh = true;
    try {
      await client.refresh();
      if (_disposed) return;
      lastError = null;
      final next = client.current;
      final sameEndpoints =
          jsonEncode(next.json['deployment']) ==
              jsonEncode(_network.json['deployment']) &&
          jsonEncode(next.json['web']) == jsonEncode(_network.json['web']) &&
          jsonEncode(next.json['updates']) ==
              jsonEncode(_network.json['updates']);
      if (sameEndpoints) {
        _network = next;
        pendingRestart = null;
      } else {
        pendingRestart = next;
      }
      _apply();
    } catch (error) {
      if (_disposed) return;
      lastError = error;
      notifyListeners();
    }
  }

  void _apply() {
    final next = client.current;
    final catalog = _catalog(
      next,
      client.resources,
    ).withRevision(++_catalogRevision);
    _presentation = next;
    if (_timer != null) {
      _timer!.cancel();
      _timer = Timer.periodic(refreshInterval, (_) => unawaited(refresh()));
    }
    locale.applyConfiguration(
      catalog: catalog,
      defaultTag: next.defaultLocale,
      enabledLocales: next.enabledLocales,
      aliases: next.localeAliases,
    );
    _expiryTimer?.cancel();
    final deadlines = [?_network.expiresAt, ?client.current.expiresAt]..sort();
    if (deadlines.isNotEmpty) {
      final remaining = deadlines.first.difference(client.effectiveTime);
      if (remaining > Duration.zero) {
        _expiryTimer = Timer(remaining, () {
          if (!_disposed) notifyListeners();
        });
      }
    }
    notifyListeners();
  }

  void start({Duration? interval}) {
    if (_disposed || _timer != null || client.bundle.discoveryUris.isEmpty) {
      return;
    }
    if (!_attemptedRefresh) unawaited(refresh());
    _pollOverride = interval;
    _timer = Timer.periodic(refreshInterval, (_) => unawaited(refresh()));
  }

  @override
  void dispose() {
    _disposed = true;
    locale.ensureLocale = null;
    _timer?.cancel();
    _expiryTimer?.cancel();
    super.dispose();
  }
}

final class OpenMuseRuntimeHost extends StatefulWidget {
  const OpenMuseRuntimeHost({
    super.key,
    required this.runtime,
    required this.child,
    this.disposeRuntime = true,
  });
  final OpenMuseClientRuntime runtime;
  static OpenMuseClientRuntime? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<_RuntimeSessionScope>()
      ?.notifier;
  final Widget child;
  final bool disposeRuntime;
  @override
  State<OpenMuseRuntimeHost> createState() => _OpenMuseRuntimeHostState();
}

final class _OpenMuseRuntimeHostState extends State<OpenMuseRuntimeHost> {
  @override
  void dispose() {
    if (widget.disposeRuntime) widget.runtime.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => _RuntimeSessionScope(
    runtime: widget.runtime,
    child: ListenableBuilder(
      listenable: widget.runtime,
      builder: (context, _) => RuntimeConfigScope(
        config: widget.runtime.presentation,
        child: widget.child,
      ),
    ),
  );
}

final class _RuntimeSessionScope
    extends InheritedNotifier<OpenMuseClientRuntime> {
  const _RuntimeSessionScope({
    required OpenMuseClientRuntime runtime,
    required super.child,
  }) : super(notifier: runtime);
}

/// Keep the navigator and local workspace mounted while showing runtime state.
final class OpenMuseRuntimeNotice extends StatelessWidget {
  const OpenMuseRuntimeNotice({super.key, required this.child});
  final Widget child;
  @override
  Widget build(BuildContext context) {
    final locale = OpenMuseLocaleScope.maybeOf(context);
    final runtime = context
        .dependOnInheritedWidgetOfExactType<_RuntimeSessionScope>()
        ?.notifier;
    if (runtime == null ||
        (!runtime.networkExpired &&
            runtime.pendingRestart == null &&
            locale?.loadingLanguage == null &&
            locale?.languageError == null)) {
      return child;
    }
    final strings = runtime.locale.strings;
    return Column(
      children: [
        Material(
          color: Theme.of(context).colorScheme.secondaryContainer,
          child: SafeArea(
            bottom: false,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
              child: Row(
                children: [
                  const Icon(Icons.cloud_off_outlined, size: 18),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      locale?.loadingLanguage != null
                          ? strings.commonLanguageDownloading()
                          : locale?.languageError != null
                          ? strings.commonLanguageDownloadFailed()
                          : runtime.networkExpired
                          ? (runtime.client.networkUnavailable
                                ? strings.commonConfigUnavailable()
                                : strings.commonConfigExpired())
                          : strings.commonConfigPending(),
                    ),
                  ),
                  TextButton(
                    onPressed: locale?.loadingLanguage != null
                        ? null
                        : () => unawaited(
                            locale?.languageError != null
                                ? locale!.retryLanguage()
                                : runtime.refresh(),
                          ),
                    child: Text(strings.commonRetry()),
                  ),
                ],
              ),
            ),
          ),
        ),
        Expanded(child: child),
      ],
    );
  }
}
