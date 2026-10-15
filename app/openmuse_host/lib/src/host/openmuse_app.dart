import 'dart:async';
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
    this.updates,
    this.desktopUpdate,
  });

  final OpenMusePluginRegistry registry;
  final LocalWorkspaceController workspace;
  final OpenMuseLocalSettings? settings;
  final WorkbenchLayoutController? layoutController;
  final LayoutSnapshotWriter? layoutStore;
  final SurfaceMutationGuards? mutationGuards;
  final OpenMuseAuthenticationContributor? authentication;
  final ValueListenable<String?>? readyAccount;
  final OpenMuseUpdateController? updates;
  final OpenMuseDesktopUpdate? desktopUpdate;

  @override
  State<OpenMuseHostApp> createState() => _OpenMuseHostAppState();
}

final class _OpenMuseHostAppState extends State<OpenMuseHostApp>
    with WidgetsBindingObserver {
  final _locale = OpenMuseUiLocale();
  OpenMuseLocalSettings? _ownedSettings;
  final _navigatorKey = GlobalKey<NavigatorState>();
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
    final zh = Localizations.localeOf(context).languageCode == 'zh';
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
          title: Text(zh ? '更新已就绪' : 'Update ready'),
          content: Text(
            zh
                ? 'OpenMuse ${release.version} 已下载并校验。立即重启以安装更新，或稍后在左下角再次点击更新。'
                : 'OpenMuse ${release.version} is downloaded and verified. Restart now to install, or use the update notice later.',
          ),
          actions: [
            TextButton(
              key: const ValueKey('app-update-restart-later'),
              onPressed: () => Navigator.pop(dialogContext, false),
              child: Text(zh ? '稍后重启' : 'Restart later'),
            ),
            FilledButton(
              key: const ValueKey('app-update-restart-now'),
              onPressed: () => Navigator.pop(dialogContext, true),
              child: Text(zh ? '立即重启' : 'Restart now'),
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
          SnackBar(content: Text(zh ? '更新失败：$error' : 'Update failed: $error')),
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
      listenable: Listenable.merge([preferences, _locale]),
      builder: (context, _) => OpenMuseUiLocaleScope(
        controller: _locale,
        child: MaterialApp(
          navigatorKey: _navigatorKey,
          debugShowCheckedModeBanner: false,
          title: 'OpenMuse',
          locale: _locale.locale,
          localizationsDelegates:
              OpenMuseAuthLocalizations.localizationsDelegates,
          supportedLocales: OpenMuseAuthLocalizations.supportedLocales,
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
