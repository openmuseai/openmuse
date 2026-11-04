import 'dart:async';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart';
import 'package:url_launcher/url_launcher.dart';

import 'design_system.dart';
import 'layout/layout.dart';
import 'layout/surface_mutation_guard.dart';
import 'local_settings.dart';
import 'workbench_shell.dart';
import 'workspace_controller.dart';
import 'desktop_update.dart';

final class OpenMuseHostApp extends StatefulWidget {
  const OpenMuseHostApp({
    super.key,
    required this.registry,
    required this.workspace,
    this.settings,
    this.layoutController,
    this.layoutStore,
    this.mutationGuards,
    this.authentication,
    this.readyAccount,
    this.resources,
    this.updates,
    this.desktopUpdate,
    this.navigatorKey,
    this.localeController,
  });

  final OpenMuseUiLocale? localeController;
  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings? settings;
  final WorkbenchLayoutController? layoutController;
  final LayoutSnapshotWriter? layoutStore;
  final SurfaceMutationGuards? mutationGuards;
  final OpenMuseAuthenticationContributor? authentication;
  final ValueListenable<String?>? readyAccount;
  final ValueListenable<NativeResourceCenter?>? resources;
  final OpenMuseUpdateController? updates;
  final OpenMuseDesktopUpdate? desktopUpdate;
  final GlobalKey<NavigatorState>? navigatorKey;

  @override
  State<OpenMuseHostApp> createState() => _OpenMuseHostAppState();
}

final class _OpenMuseHostAppState extends State<OpenMuseHostApp>
    with WidgetsBindingObserver {
  late final _locale = widget.localeController ?? OpenMuseUiLocale();
  OpenMuseLocalSettings? _ownedSettings;
  final _ownedNavigatorKey = GlobalKey<NavigatorState>();
  GlobalKey<NavigatorState> get _navigatorKey =>
      widget.navigatorKey ?? _ownedNavigatorKey;
  late final _desktopUpdate = widget.desktopUpdate ?? OpenMuseDesktopUpdate();
  bool _updateBusy = false;
  File? _readyArchive;
  String? _readyHash;

  OpenMuseLocalSettings get _settings =>
      widget.settings ?? (_ownedSettings ??= OpenMuseLocalSettings());

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
  }

  @override
  void didChangeLocales(List<Locale>? locales) => _locale.updateSystemLocales(
    (locales ?? []).map((v) => v.toLanguageTag()),
  );

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed && widget.updates != null) {
      unawaited(widget.updates!.check());
    }
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _locale.dispose();
    _ownedSettings?.dispose();
    if (widget.desktopUpdate == null) _desktopUpdate.dispose();
    widget.updates?.dispose();
    super.dispose();
  }

  Future<void> _getUpdate(OpenMuseRelease release) async {
    if (!Platform.isMacOS || release.installKind != 'macos-zip') {
      await launchUrl(
        release.downloadUri,
        mode: LaunchMode.externalApplication,
      );
      return;
    }
    if (_updateBusy) return;
    final context = _navigatorKey.currentContext;
    if (context == null) return;
    _updateBusy = true;
    final localeStrings = stringsOf(context);
    try {
      File archive;
      if (_readyHash == release.sha256 &&
          _readyArchive != null &&
          await _readyArchive!.exists()) {
        archive = _readyArchive!;
      } else {
        _desktopUpdate.releaseSha256 = release.sha256;
        _desktopUpdate.status.value = OpenMuseDesktopUpdateStatus.downloading;
        archive = await _desktopUpdate.stage(release);
        _readyArchive = archive;
        _readyHash = release.sha256;
      }
      if (!mounted) return;
      _desktopUpdate.status.value = OpenMuseDesktopUpdateStatus.ready;
      final restartNow = await showDialog<bool>(
        context: _navigatorKey.currentContext!,
        barrierDismissible: false,
        builder: (dialogContext) => AlertDialog(
          key: const ValueKey('app-update-restart-dialog'),
          title: Text(localeStrings.updatesUpdateReadyEf88289a()),
          content: Text(
            localeStrings.updatesOpenmuseIsDownloadedAndVerifiedRestar14710fdb(
              value1: release.version,
            ),
          ),
          actions: [
            TextButton(
              key: const ValueKey('app-update-restart-later'),
              onPressed: () => Navigator.pop(dialogContext, false),
              child: Text(localeStrings.updatesRestartLaterD2da82eb()),
            ),
            FilledButton(
              key: const ValueKey('app-update-restart-now'),
              onPressed: () => Navigator.pop(dialogContext, true),
              child: Text(localeStrings.updatesRestartNow09959419()),
            ),
          ],
        ),
      );
      if (restartNow == true) {
        await _desktopUpdate.installAndRestart(archive, release);
      }
    } catch (error) {
      if (mounted) {
        _desktopUpdate.status.value = OpenMuseDesktopUpdateStatus.failed;
        ScaffoldMessenger.of(_navigatorKey.currentContext!).showSnackBar(
          SnackBar(
            content: Text(
              localeStrings.updatesUpdateFailed9d48e538(value1: error),
            ),
          ),
        );
      }
    } finally {
      _updateBusy = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    final preferences = _settings;
    return ListenableBuilder(
      listenable: Listenable.merge([preferences, _locale,?widget.resources]),
      builder: (context, _) => OpenMuseUiLocaleScope(
        controller: _locale,
        child: MaterialApp(
          navigatorKey: _navigatorKey,
          debugShowCheckedModeBanner: false,
          title: 'OpenMuse',
          locale: _locale.locale,
          localizationsDelegates: [
            OpenMuseStringsDelegate(_locale.catalog),
            ...OpenMuseAuthLocalizations.localizationsDelegates,
          ],
          supportedLocales: _locale.supportedLocales,
          builder: (context, child) => ResourceLinkRouting(open:(uri)async {
            final center=widget.resources?.value,navContext=_navigatorKey.currentContext;
            if(center==null || navContext==null)return;
            try{await openResourcePage(navContext,center.controller,ResourceReference.fromUri(uri));}
            on Object catch(error){if(navContext.mounted)showResourceError(navContext,error);}
          },child:OpenMuseRuntimeNotice(child: child ?? const SizedBox.shrink())),
          theme: buildOpenMuseTheme(),
          darkTheme: buildOpenMuseTheme(brightness: Brightness.dark),
          themeMode: preferences.themeMode,
          navigatorObservers: [DshPopupRouteObserver.instance],
          home: Builder(
            builder: (context) {
              final workbench = OpenMuseWorkbench(
                updates: widget.updates,
                desktopUpdate: _desktopUpdate,
                onGetUpdate: _getUpdate,
                resources:widget.resources?.value,
                registry: widget.registry,
                workspace: widget.workspace,
                settings: preferences,
                layoutController: widget.layoutController,
                layoutStore: widget.layoutStore,
                mutationGuards: widget.mutationGuards,
              );
              final auth = widget.authentication;
              if (auth == null) {
                return OpenMuseStartupUpdatePrompt(
                  controller: widget.updates,
                  onUpdate: _getUpdate,
                  child: workbench,
                );
              }
              final accountGate = widget.readyAccount == null
                  ? workbench
                  : ValueListenableBuilder<String?>(
                      valueListenable: widget.readyAccount!,
                      builder: (context, ready, _) {
                        final subject =
                            auth.authentication.snapshot.identity?.subject;
                        if (ready == null || ready != subject) {
                          return const Center(
                            child: CircularProgressIndicator(),
                          );
                        }
                        return KeyedSubtree(
                          key: ValueKey(subject),
                          child: workbench,
                        );
                      },
                    );
              return OpenMuseStartupUpdatePrompt(
                controller: widget.updates,
                onUpdate: _getUpdate,
                child: auth.buildAuthenticationGate(
                  context,
                  authenticatedChild: accountGate,
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
