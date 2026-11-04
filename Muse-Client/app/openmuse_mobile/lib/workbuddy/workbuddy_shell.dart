import '../mobile_plugins/mobile_apps_page.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter_markdown_plus/flutter_markdown_plus.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:muse_speech_core/muse_speech_core.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import 'package:openmuse_speech_input/openmuse_speech_input.dart';
import '../speech_model_manager.dart';
import '../native_dsh_page.dart';
import '../mobile_plugin_detail_page.dart';
import '../plugin_interaction_remote.dart';
import '../mobile_plugins/mobile_workbench_page.dart';
import '../mobile_plugins/mobile_my_page.dart';
import '../mobile_plugins/paired_mobile_context.dart';
import 'workbuddy_controller.dart';
import 'workbuddy_icons.dart';
import 'workbuddy_models.dart';
import 'workbuddy_theme.dart';

final class _SavedDesktop {
  const _SavedDesktop({required this.kind, this.deviceRef, this.workspaceId});

  final String kind;
  final String? deviceRef;
  final String? workspaceId;

  Map<String, Object?> toJson() => {
    'kind': kind,
    if (deviceRef != null) 'deviceRef': deviceRef,
    if (workspaceId != null) 'workspaceId': workspaceId,
  };

  factory _SavedDesktop.fromJson(Map<String, Object?> json) {
    final deviceRef = json['deviceRef'];
    final workspaceId = json['workspaceId'];
    final workspace = workspaceId is String && workspaceId.isNotEmpty
        ? workspaceId
        : null;
    if (json['kind'] == 'cloud') {
      return _SavedDesktop(kind: 'cloud', workspaceId: workspace);
    }
    if (deviceRef is! String || deviceRef.isEmpty) {
      throw const FormatException('saved desktop is missing deviceRef');
    }
    return _SavedDesktop(
      kind: 'desktop',
      deviceRef: deviceRef,
      workspaceId: workspace,
    );
  }
}

final class WorkBuddyShell extends StatefulWidget {
  const WorkBuddyShell({
    super.key,
    required this.controller,
    this.authentication,
    this.pairedDesktop,
    this.catalog,
    this.cloudLabel,
    this.onSignOut,
    this.onDeleteAccount,
    this.speechRecognition,
    this.debugSpeechSource,
    this.remoteWorkbench,
    this.mobileMarketPlugin,
    this.deviceSelectionStore,
    this.pluginRegistry,
    this.resources,
    this.resourceFailure,
    this.onRetryResources,
    this.themeMode = ThemeMode.system,
    this.onThemeMode,
    this.onOpenWord,
    this.onOpenPdf,
    this.onOpenSpreadsheet,
  });

  final WorkBuddyController controller;
  final OpenMuseAuthenticationController? authentication;
  final PairedDesktopMobileController? pairedDesktop;
  final WorkspaceCatalogPort? catalog;
  final String? cloudLabel;
  final Future<void> Function()? onSignOut;
  final Future<void> Function()? onDeleteAccount;
  final SpeechRecognitionPort? speechRecognition;

  /// Debug-only E2E seam supplied by the Mobile Host. It is never populated
  /// from a plugin wire request or other untrusted input.
  final SpeechAudioSource? debugSpeechSource;
  final RemoteWorkbenchController? remoteWorkbench;
  final OpenMuseMobilePluginMarket? mobileMarketPlugin;
  final SecureValueStore? deviceSelectionStore;
  final OpenMusePluginRegistry? pluginRegistry;
  final NativeResourceCenter? resources;
  final Object? resourceFailure;
  final VoidCallback? onRetryResources;
  final ThemeMode themeMode;
  final ValueChanged<ThemeMode>? onThemeMode;
  final VoidCallback? onOpenWord;
  final VoidCallback? onOpenPdf;
  final VoidCallback? onOpenSpreadsheet;

  @override
  State<WorkBuddyShell> createState() => _WorkBuddyShellState();
}

final class _WorkBuddyShellState extends State<WorkBuddyShell>
    with SingleTickerProviderStateMixin, WidgetsBindingObserver {
  late final AnimationController _drawer;
  final _composer = TextEditingController();
  final _focus = FocusNode();
  final _workbenchKey = GlobalKey<MobileWorkbenchPageState>();
  final _appsEditing = ValueNotifier<bool>(false);
  final _managementReturn = AppsReturnController();
  bool _voiceMode = false;
  bool _holdingVoice = false;
  bool _voiceReleased = false;
  bool _cancelVoiceOnRelease = false;
  bool _speechOpening = false;
  bool _stopSpeechWhenOpened = false;
  bool _cancelSpeechWhenOpened = false;
  SpeechSession? _speechSession;
  StreamSubscription<SpeechEvent>? _speechEvents;
  SpeechDraftBuffer? _speechDraft;
  final NativeDshSessionHandle _sessionHandle = NativeDshSessionHandle();
  final List<String> _recentNativeSessionIds = [];
  String? _nativeConnectionKey;
  String? _pendingSessionId;
  String? _pendingPrompt;
  String? _pendingOriginalText;
  bool _desktopBindOffered = false;
  bool _initialAuthResolved = false;
  int _desktopBindAttempts = 0;
  Timer? _desktopRenewalTimer;
  Future<void>? _desktopRestoreTask;
  String? _desktopRestoreDeviceRef;
  bool _submitting = false;
  bool _appForeground = true;
  DateTime? _backgroundedAt;

  SpeechModelManager? _speechModels;
  WorkBuddyController get controller => widget.controller;

  @override
  void initState() {
    super.initState();
    controller.beforeLeavingApps = () async =>
        await _workbenchKey.currentState?.requestLeave() ?? true;
    WidgetsBinding.instance.addObserver(this);
    _drawer = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 240),
      value: controller.drawerOpen ? 1 : 0,
    );
    controller.addListener(_syncDrawer);
    controller.addListener(_syncPluginContexts);
    widget.authentication?.addListener(_syncAccount);
    final initial = widget.authentication?.snapshot;
    _initialAuthResolved =
        initial == null ||
        initial.phase == OpenMuseAuthenticationPhase.signedOut ||
        initial.isAuthenticated;
    widget.pairedDesktop?.addListener(_syncDevices);
    widget.pairedDesktop?.addListener(_syncPluginContexts);
    _syncAccount();
    _syncDevices();
    _syncPluginContexts();
    _sessionHandle.addListener(_syncSession);
    unawaited(_loadCloud());
    if (widget.debugSpeechSource != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) unawaited(_startVoice(source: widget.debugSpeechSource));
      });
    }
  }

  bool _sessionUpdateScheduled = false;
  void _syncSession() {
    if (!mounted) return;
    // Attach/detach may run while a child is built or unmounted. Defer the
    // parent's repaint until Flutter has finished that frame.
    if (SchedulerBinding.instance.schedulerPhase ==
        SchedulerPhase.persistentCallbacks) {
      if (_sessionUpdateScheduled) return;
      _sessionUpdateScheduled = true;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _sessionUpdateScheduled = false;
        if (mounted) setState(() {});
      });
    } else {
      setState(() {});
    }
  }

  void _syncPluginContexts() {
    final registry = widget.pluginRegistry;
    if (registry == null) return;
    final connection = widget.pairedDesktop?.snapshot.connection;
    for (final descriptor in registry.descriptors) {
      final plugin = registry.plugin(descriptor.id);
      if (plugin is PairedMobileContextContributor) {
        (plugin as PairedMobileContextContributor).updatePairedContext(
          connection: connection,
          workspaces: controller.workspaces,
          selectedWorkspaceId: controller.selectedWorkspace.id,
        );
      }
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final speech = widget.speechRecognition;
    if (_speechModels == null && speech is OpenMuseSpeechInputPlugin) {
      _speechModels = SpeechModelManager(
        speech.localModel,
        beforeRemove: speech.releaseLocalEngine,
        downloadOrigin: RuntimeConfigScope.maybeOf(
          context,
        )?.serviceUri(ServiceId.cloud),
      );
    }
  }

  @override
  void didUpdateWidget(covariant WorkBuddyShell oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.authentication != widget.authentication) {
      oldWidget.authentication?.removeListener(_syncAccount);
      widget.authentication?.addListener(_syncAccount);
      _syncAccount();
    }
    if (oldWidget.pairedDesktop != widget.pairedDesktop) {
      oldWidget.pairedDesktop?.removeListener(_syncDevices);
      oldWidget.pairedDesktop?.removeListener(_syncPluginContexts);
      _desktopRenewalTimer?.cancel();
      _nativeConnectionKey = null;
      widget.pairedDesktop?.addListener(_syncDevices);
      widget.pairedDesktop?.addListener(_syncPluginContexts);
      _syncDevices();
    }
    if (oldWidget.pluginRegistry != widget.pluginRegistry) {
      _syncPluginContexts();
    }
    if (oldWidget.catalog != widget.catalog) unawaited(_loadCloud());
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed) {
      final wasAway =
          _backgroundedAt != null &&
          DateTime.now().difference(_backgroundedAt!) >=
              const Duration(seconds: 30);
      _backgroundedAt = null;
      _appForeground = true;
      unawaited(_restoreDesktopConnection(force: wasAway));
    } else if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.hidden) {
      _appForeground = false;
      _backgroundedAt ??= DateTime.now();
    }
  }

  void _syncDrawer() {
    if (!mounted) return;
    _rememberNativeSession();
    _persistPairedSelection();
    if (controller.drawerOpen) {
      _drawer.forward();
    } else {
      _drawer.reverse();
    }
  }

  String? _appsAccountSubject;
  void _syncAccount() {
    final snapshot = widget.authentication?.snapshot;
    final identity = snapshot?.identity;
    if (_appsAccountSubject != identity?.subject) {
      _appsAccountSubject = identity?.subject;
      controller.appsSection = AppsSection.overview;
      _appsEditing.value = false;
    }
    final firstResolvedState =
        !_initialAuthResolved &&
        (snapshot?.phase == OpenMuseAuthenticationPhase.signedOut ||
            snapshot?.isAuthenticated == true ||
            snapshot?.phase == OpenMuseAuthenticationPhase.failure);
    if (firstResolvedState) _initialAuthResolved = true;
    final becameSignedIn = identity != null && !controller.signedIn;
    controller.setAccount(signedIn: identity != null, name: identity?.email);
    if (identity == null) {
      _desktopBindOffered = false;
      _desktopBindAttempts = 0;
      return;
    }
    unawaited(_loadCloud());
    if (becameSignedIn) {
      unawaited(_restoreSavedDesktop(identity.subject, !firstResolvedState));
    }
  }

  String? _savedWorkspaceId;
  String? _persistedSelectionToken;

  String _desktopSelectionKey(String subject) =>
      'openmuse.mobile.desktop.${base64Url.encode(utf8.encode(subject))}';

  Future<void> _restoreSavedDesktop(String subject, bool interactive) async {
    final store = widget.deviceSelectionStore;
    String? raw;
    if (store != null) {
      try {
        raw = await store.read(_desktopSelectionKey(subject));
        if (raw == null) {
          await Future<void>.delayed(const Duration(milliseconds: 250));
          if (!mounted ||
              widget.authentication?.snapshot.identity?.subject != subject) {
            return;
          }
          raw = await store.read(_desktopSelectionKey(subject));
        }
      } on Object catch (error) {
        debugPrint('OpenMuse Desktop selection restore: $error');
      }
    }
    if (!mounted ||
        widget.authentication?.snapshot.identity?.subject != subject) {
      return;
    }
    final saved = raw == null ? null : _parseSavedDesktop(raw);
    if (saved == null) {
      if (interactive) _offerDesktopBind();
      return;
    }
    _desktopBindOffered = true;
    _savedWorkspaceId = saved.workspaceId;
    if (saved.kind == 'cloud') {
      controller.selectDevice(kCloudDeviceId);
      _applySavedWorkspace();
      return;
    }
    final deviceRef = saved.deviceRef;
    if (deviceRef == null || deviceRef.isEmpty) {
      if (interactive) _offerDesktopBind();
      return;
    }
    final id = 'paired.$deviceRef';
    final matches = widget.pairedDesktop?.devices.where(
      (device) => device.deviceRef == deviceRef,
    );
    final known = matches == null || matches.isEmpty ? null : matches.first;
    controller.upsertDevice(
      WbDevice(
        id: id,
        name: known?.displayName ?? 'Desktop',
        kind: WbDeviceKind.local,
        online: known?.online ?? false,
      ),
    );
    controller.selectDevice(id);
    _applySavedWorkspace();
    unawaited(_restoreDesktopConnection(deviceRef: deviceRef, force: true));
  }

  void _persistPairedSelection() {
    final saved = _selectionToSave();
    if (saved == null) return;
    final token = '${saved.kind}|${saved.deviceRef}|${saved.workspaceId}';
    if (token == _persistedSelectionToken) return;
    _persistedSelectionToken = token;
    unawaited(_writeDesktopSelection(saved));
  }

  _SavedDesktop? _selectionToSave() {
    final id = controller.selectedDeviceId;
    if (id == kCloudDeviceId) {
      return _SavedDesktop(
        kind: 'cloud',
        workspaceId: _persistableWorkspaceId(),
      );
    }
    if (!id.startsWith('paired.') || id == kPendingDeviceId) return null;
    final deviceRef = id.substring('paired.'.length);
    if (deviceRef.isEmpty || deviceRef == 'pending') return null;
    return _SavedDesktop(
      kind: 'desktop',
      deviceRef: deviceRef,
      workspaceId: _persistableWorkspaceId(),
    );
  }

  String? _persistableWorkspaceId() {
    final id = controller.selectedWorkspaceId;
    if (id.startsWith('dsh.workspace.') || id.startsWith('cloud.')) {
      if (id != 'cloud.pending') return id;
    }
    return _savedWorkspaceId;
  }

  void _applySavedWorkspace() {
    final id = _savedWorkspaceId;
    if (id == null) return;
    if (!controller.workspaces.any((workspace) => workspace.id == id)) return;
    controller.selectWorkspace(id);
  }

  Future<void> _rememberDesktop(String deviceRef) {
    final workspaceId = controller.selectedDeviceId == 'paired.$deviceRef'
        ? _persistableWorkspaceId()
        : _savedWorkspaceId;
    return _writeDesktopSelection(
      _SavedDesktop(
        kind: 'desktop',
        deviceRef: deviceRef,
        workspaceId: workspaceId,
      ),
    );
  }

  Future<void> _writeDesktopSelection(_SavedDesktop saved) async {
    final subject = widget.authentication?.snapshot.identity?.subject;
    final store = widget.deviceSelectionStore;
    if (subject == null || store == null) return;
    if (saved.workspaceId != null &&
        saved.workspaceId != 'cloud.pending' &&
        (saved.workspaceId!.startsWith('dsh.workspace.') ||
            saved.workspaceId!.startsWith('cloud.'))) {
      _savedWorkspaceId = saved.workspaceId;
    }
    try {
      await store.write(
        _desktopSelectionKey(subject),
        jsonEncode(saved.toJson()),
      );
    } on Object catch (error) {
      debugPrint('OpenMuse Desktop selection save: $error');
    }
  }

  _SavedDesktop? _parseSavedDesktop(String raw) {
    if (!raw.startsWith('{')) {
      if (raw.isEmpty) return null;
      return _SavedDesktop(kind: 'desktop', deviceRef: raw);
    }
    try {
      final decoded = jsonDecode(raw);
      if (decoded is! Map) return null;
      return _SavedDesktop.fromJson(decoded.cast<String, Object?>());
    } on Object {
      return null;
    }
  }

  void _offerDesktopBind() {
    if (_desktopBindOffered ||
        !mounted ||
        widget.pairedDesktop?.directory == null) {
      return;
    }
    if (widget.pairedDesktop?.snapshot.connection != null) {
      _desktopBindOffered = true;
      return;
    }
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted || _desktopBindOffered || !controller.signedIn) return;
      if (widget.pairedDesktop?.snapshot.connection != null) {
        _desktopBindOffered = true;
        return;
      }
      final route = ModalRoute.of(context);
      if (route != null && !route.isCurrent && _desktopBindAttempts < 20) {
        _desktopBindAttempts += 1;
        _offerDesktopBind();
        return;
      }
      _desktopBindOffered = true;
      unawaited(_promptBindDesktop());
    });
  }

  Future<void> _promptBindDesktop() async {
    final localeStrings = stringsOf(context);

    if (!mounted || widget.pairedDesktop?.snapshot.connection != null) return;
    unawaited(
      widget.pairedDesktop?.directory?.refresh() ?? Future<void>.value(),
    );
    await showModalBottomSheet<void>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      isScrollControlled: true,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => _DesktopBindSheet(
        paired: widget.pairedDesktop,
        selectedDeviceId: controller.selectedDeviceId,
        onConnect: (device) async {
          if (!device.online || !device.supportsPairedDesktop) {
            final deviceId = 'paired.${device.deviceRef}';
            controller.upsertDevice(
              WbDevice(
                id: deviceId,
                name: device.displayName,
                kind: WbDeviceKind.local,
                online: device.online,
              ),
            );
            controller.selectDevice(deviceId);
            unawaited(_rememberDesktop(device.deviceRef));
            if (mounted) {
              ScaffoldMessenger.of(this.context).showSnackBar(
                SnackBar(
                  content: Text(
                    (localeStrings
                        .mobileDesktopSelectedWorkspacesWillLoadWhen38e1c99f()),
                  ),
                ),
              );
            }
            if (context.mounted) Navigator.pop(context);
            unawaited(
              _restoreDesktopConnection(
                deviceRef: device.deviceRef,
                force: true,
              ),
            );
            return;
          }
          final connected = await widget.pairedDesktop?.connectDevice(device);
          final connection = widget.pairedDesktop?.snapshot.connection;
          if (connected == true && connection != null && mounted) {
            final deviceId = 'paired.${connection.deviceRef}';
            controller.selectDevice(deviceId);
            unawaited(_rememberDesktop(connection.deviceRef));
            await _publishDesktopCatalog(connection, deviceId);
            if (context.mounted) Navigator.pop(context);
            return;
          }
          final message = widget.pairedDesktop?.snapshot.failureMessage;
          final deviceId = 'paired.${device.deviceRef}';
          controller.upsertDevice(
            WbDevice(
              id: deviceId,
              name: device.displayName,
              kind: WbDeviceKind.local,
              online: device.online,
            ),
          );
          controller.selectDevice(deviceId);
          unawaited(_rememberDesktop(device.deviceRef));
          if (mounted && message != null && message.isNotEmpty) {
            ScaffoldMessenger.of(
              this.context,
            ).showSnackBar(SnackBar(content: Text(message)));
          }
          if (context.mounted) Navigator.pop(context);
          unawaited(
            _restoreDesktopConnection(deviceRef: device.deviceRef, force: true),
          );
        },
      ),
    );
  }

  void _syncDevices() {
    final paired = widget.pairedDesktop;
    if (paired == null) return;
    final connection = paired.snapshot.connection;
    final connectionKey = connection == null
        ? null
        : '${connection.deviceRef}:${connection.grantRef}';
    if (_nativeConnectionKey != connectionKey) {
      _nativeConnectionKey = connectionKey;
      _recentNativeSessionIds.clear();
      _desktopRenewalTimer?.cancel();
      if (connection != null) {
        final renewAt =
            connection.expiresAtMs - const Duration(minutes: 1).inMilliseconds;
        final delayMs = math.max(
          0,
          renewAt - DateTime.now().millisecondsSinceEpoch,
        );
        _desktopRenewalTimer = Timer(
          Duration(milliseconds: delayMs),
          () => unawaited(_restoreDesktopConnection(force: true)),
        );
      }
    }
    for (final device in paired.devices.where(
      (device) => device.kind == AccountDeviceKind.desktop,
    )) {
      controller.upsertDevice(
        WbDevice(
          id: 'paired.${device.deviceRef}',
          name: device.displayName,
          kind: WbDeviceKind.local,
          online: device.online,
        ),
      );
    }
    if (connection != null) {
      final matching = paired.devices.where(
        (device) => device.deviceRef == connection.deviceRef,
      );
      controller.upsertDevice(
        WbDevice(
          id: 'paired.${connection.deviceRef}',
          name: connection.deviceName,
          kind: WbDeviceKind.local,
          online:
              paired.directory == null ||
              (matching.isNotEmpty && matching.first.online),
        ),
      );
    }
    _rememberNativeSession();
  }

  Future<void> _restoreDesktopConnection({
    String? deviceRef,
    bool force = false,
  }) {
    final paired = widget.pairedDesktop;
    if (paired == null ||
        paired.directory == null ||
        !mounted ||
        widget.authentication?.snapshot.isAuthenticated == false) {
      return Future<void>.value();
    }
    final connection = paired.snapshot.connection;
    final ref =
        deviceRef ??
        connection?.deviceRef ??
        (controller.selectedDeviceId.startsWith('paired.')
            ? controller.selectedDeviceId.substring('paired.'.length)
            : null);
    if (ref == null) return Future<void>.value();
    if (!force &&
        connection != null &&
        connection.deviceRef == ref &&
        connection.expiresAtMs >
            DateTime.now()
                .add(const Duration(minutes: 1))
                .millisecondsSinceEpoch) {
      return Future<void>.value();
    }
    final pending = _desktopRestoreTask;
    if (pending != null) {
      if (_desktopRestoreDeviceRef == ref) return pending;
      return pending.then(
        (_) => _restoreDesktopConnection(deviceRef: ref, force: force),
      );
    }
    _desktopRestoreDeviceRef = ref;
    return _desktopRestoreTask = _retryDesktopConnection(paired, ref)
        .whenComplete(() {
          _desktopRestoreTask = null;
          _desktopRestoreDeviceRef = null;
        });
  }

  PairedDesktopConnection? _selectedDesktopConnection() {
    final selectedId = controller.selectedDeviceId;
    final connection = widget.pairedDesktop?.snapshot.connection;
    if (connection == null ||
        selectedId != 'paired.${connection.deviceRef}' ||
        connection.expiresAtMs <=
            DateTime.now()
                .add(const Duration(minutes: 1))
                .millisecondsSinceEpoch) {
      return null;
    }
    return connection;
  }

  Future<void> _retryDesktopConnection(
    PairedDesktopMobileController paired,
    String deviceRef,
  ) async {
    final originalGrantRef = paired.snapshot.connection?.grantRef;
    var attempted = false;
    while (mounted &&
        _appForeground &&
        identical(widget.pairedDesktop, paired)) {
      if (widget.authentication?.snapshot.isAuthenticated == false ||
          controller.selectedDeviceId != 'paired.$deviceRef') {
        return;
      }
      final current = paired.snapshot.connection;
      if (attempted &&
          current?.deviceRef == deviceRef &&
          current?.grantRef != originalGrantRef &&
          paired.snapshot.failureMessage == null &&
          current!.expiresAtMs >
              DateTime.now()
                  .add(const Duration(minutes: 1))
                  .millisecondsSinceEpoch) {
        return;
      }
      try {
        await paired.directory?.reconcile();
        if (!mounted ||
            !_appForeground ||
            !identical(widget.pairedDesktop, paired)) {
          return;
        }
        final devices = paired.devices.where(
          (device) => device.deviceRef == deviceRef,
        );
        if (devices.isNotEmpty && devices.first.online) {
          attempted = true;
          final connected = await paired.connectDevice(
            devices.first,
            force: true,
          );
          if (!mounted || !identical(widget.pairedDesktop, paired)) return;
          final connection = paired.snapshot.connection;
          if (connected && connection != null) {
            unawaited(_publishDesktopCatalog(connection, 'paired.$deviceRef'));
            return;
          }
        }
      } on Object catch (error) {
        debugPrint('OpenMuse Desktop reconnect: $error');
      }
      await Future<void>.delayed(const Duration(seconds: 8));
    }
  }

  void _rememberNativeSession() {
    if (_nativeConnectionKey == null) return;
    final task = controller.openTask;
    if (task == null || !task.id.startsWith('dsh.session.')) return;
    final id = task.id.substring('dsh.session.'.length);
    _recentNativeSessionIds.remove(id);
    _recentNativeSessionIds.add(id);
    const maxRetainedSessions = 4;
    if (_recentNativeSessionIds.length > maxRetainedSessions) {
      _recentNativeSessionIds.removeAt(0);
    }
  }

  Future<void> _loadCloud() async {
    final catalog = widget.catalog;
    final signedIn = widget.authentication?.snapshot.isAuthenticated ?? false;
    if (catalog == null || !signedIn) return;
    try {
      final items = await catalog.listWorkspaces();
      if (!mounted) return;
      controller.replaceCloudWorkspaces([
        for (final item in items)
          if (item.placement == WorkspacePlacement.cloud)
            WbWorkspace(
              id: 'cloud.${item.workspaceRef}',
              name: item.title,
              deviceId: kCloudDeviceId,
            ),
      ]);
      _applySavedWorkspace();
    } on Object {
      // Cloud catalog is optional. Local computer and its workspaces stay usable.
    }
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _desktopRenewalTimer?.cancel();
    _speechModels?.dispose();
    controller.removeListener(_syncDrawer);
    controller.removeListener(_syncPluginContexts);
    widget.authentication?.removeListener(_syncAccount);
    widget.pairedDesktop?.removeListener(_syncDevices);
    widget.pairedDesktop?.removeListener(_syncPluginContexts);
    _drawer.dispose();
    final speechSession = _speechSession;
    if (speechSession != null) {
      unawaited(_cancelSpeechOnDispose(speechSession));
    }
    unawaited(_speechEvents?.cancel());
    controller.beforeLeavingApps = null;
    _appsEditing.dispose();
    _managementReturn.dispose();
    _composer.dispose();
    _focus.dispose();
    _sessionHandle
      ..removeListener(_syncSession)
      ..dispose();
    super.dispose();
  }

  Future<void> _cancelSpeechOnDispose(SpeechSession session) async {
    try {
      await widget.speechRecognition?.cancel(session.ref);
    } on Object {
      // Plugin deactivation may have won the shutdown race.
    }
  }

  Future<void> _openLogin() async {
    final authentication = widget.authentication;
    if (authentication == null) {
      debugPrint('OpenMuse mobile: login aborted, authentication is null');
      return;
    }
    debugPrint(
      'OpenMuse mobile: pushing login route phase=${authentication.snapshot.phase.name}',
    );
    try {
      await Navigator.of(context).push<void>(
        MaterialPageRoute(
          builder: (_) => _MobileLoginRoute(
            authentication: authentication,
            cloudLabel: widget.cloudLabel,
          ),
        ),
      );
      debugPrint('OpenMuse mobile: login route popped');
    } catch (error, stackTrace) {
      debugPrint(
        'OpenMuse mobile: login route failed type=${error.runtimeType} error=$error\n$stackTrace',
      );
    }
  }

  Future<void> _chooseDevice() async {
    final localeStrings = stringsOf(context);

    if (widget.pairedDesktop?.directory != null) {
      await _promptBindDesktop();
      return;
    }
    final picked = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => _ChoiceSheet(
        title: (localeStrings.mobileChooseADevice6d041181()),
        children: [
          for (final device in controller.devices)
            if (device.id != kPendingDeviceId &&
                device.kind != WbDeviceKind.cloud)
              _DeviceStatusRow(
                key: ValueKey('device-${device.id}'),
                id: device.id,
                name: device.name,
                online: device.online,
                current: device.id == controller.selectedDeviceId,
                connected:
                    widget.pairedDesktop?.snapshot.connection?.deviceRef ==
                    device.id.replaceFirst('paired.', ''),
                onBind: device.online
                    ? () => Navigator.pop(context, device.id)
                    : null,
              ),
        ],
      ),
    );
    if (picked == null || !mounted) return;
    if (picked == kCloudDeviceId) {
      if (widget.authentication?.snapshot.isAuthenticated != true) {
        await _openLogin();
        if (!mounted ||
            widget.authentication?.snapshot.isAuthenticated != true) {
          return;
        }
        await _loadCloud();
      }
      controller.selectDevice(picked);
      _showCloudDeveloping();
      return;
    }
    if (picked.startsWith('paired.')) {
      final ref = picked.substring('paired.'.length);
      final matches = widget.pairedDesktop?.devices.where(
        (item) => item.deviceRef == ref,
      );
      final device = matches == null || matches.isEmpty ? null : matches.first;
      if (device != null) {
        final previousConnection = widget.pairedDesktop?.snapshot.connection;
        final hasCatalog = controller
            .workspacesFor(picked)
            .any((workspace) => workspace.id.startsWith('dsh.workspace.'));
        final connected = await widget.pairedDesktop?.connectDevice(
          device,
          force: true,
        );
        final connection = widget.pairedDesktop?.snapshot.connection;
        if (connected == true && connection != null && mounted) {
          controller.selectDevice(picked);
          unawaited(_rememberDesktop(connection.deviceRef));
          if (!hasCatalog ||
              previousConnection?.grantRef != connection.grantRef) {
            await _publishDesktopCatalog(connection, picked);
          }
          return;
        }
        final message = widget.pairedDesktop?.snapshot.failureMessage;
        if (mounted && message != null && message.isNotEmpty) {
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text(message)));
        }
        return;
      }
    }
    controller.selectDevice(picked);
  }

  Future<void> _publishDesktopCatalog(
    PairedDesktopConnection connection,
    String deviceId,
  ) async {
    final localeStrings = stringsOf(context);

    final client = DshNativeGatewayClient(
      origin: Uri.parse(connection.session.origin),
      bootstrapPath: connection.session.path,
      allowInsecureLoopback: connection.session.allowInsecureLoopback,
      allowInsecurePrivateNetworkForTesting:
          connection.session.allowInsecurePrivateNetworkForTesting,
    );
    try {
      await client.initialize();
      final values = await Future.wait<Object>([
        client.listWorkspaces(),
        client.listSessions(),
      ]);
      final workspaces = values[0] as List<DshNativeWorkspaceSummary>;
      final sessions = values[1] as List<DshNativeSessionSummary>;
      if (!mounted) return;
      final byId = {for (final session in sessions) session.sessionId: session};
      controller.replacePairedCatalog(
        deviceId: deviceId,
        workspaces: [
          for (final workspace in workspaces)
            WbWorkspace(
              id: 'dsh.workspace.${workspace.workspaceId}',
              name: workspace.title,
              deviceId: deviceId,
            ),
        ],
        sessions: [
          for (final workspace in workspaces)
            for (final sessionId in workspace.sessionIds)
              if (byId[sessionId] case final session?)
                WbTask(
                  id: 'dsh.session.$sessionId',
                  title:
                      session.title ??
                      (session.blank
                          ? (localeStrings.mobileNewConversationBcec2f30())
                          : (localeStrings
                                .mobilePreviousConversation196d79ba())),
                  workspaceId: 'dsh.workspace.${workspace.workspaceId}',
                  deviceId: deviceId,
                  status: session.running
                      ? WbTaskStatus.running
                      : WbTaskStatus.completed,
                  showInTaskList: true,
                  updatedAt: session.updatedAt,
                  messages: const [],
                ),
        ],
      );
      _applySavedWorkspace();
    } catch (error) {
      debugPrint('OpenMuse desktop catalog: $error');
    } finally {
      client.close();
    }
  }

  Future<void> _openDesktopSession(WbTask task) async {
    final connection = widget.pairedDesktop?.snapshot.connection;
    if (connection == null || !task.id.startsWith('dsh.session.')) return;
    setState(() {
      _pendingSessionId = null;
      _pendingPrompt = null;
    });
    controller.openTaskById(task.id);
  }

  Future<void> _chooseWorkspace() async {
    final localeStrings = stringsOf(context);

    final spaces = controller.workspacesFor(controller.selectedDeviceId);
    final picked = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => _ChoiceSheet(
        title: (localeStrings.mobileChooseAWorkspace017a91b6()),
        children: [
          if (spaces.isEmpty)
            ListTile(
              title: Text(
                (localeStrings
                    .mobileNoWorkspacesAreAvailableOnThisDevice08eae7ea()),
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
            ),
          for (final workspace in spaces)
            ListTile(
              key: ValueKey('workspace-${workspace.id}'),
              title: Text(
                workspace.name,
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                ),
              ),
              subtitle: workspace.deviceId == kCloudDeviceId
                  ? Text(
                      (localeStrings.mobileComingSoonDdd8280c()),
                      style: wbSub,
                    )
                  : null,
              trailing: workspace.id == controller.selectedWorkspaceId
                  ? WbIcon(WbGlyph.check, size: 18)
                  : null,
              onTap: () => Navigator.pop(context, workspace.id),
            ),
        ],
      ),
    );
    if (picked != null) {
      controller.selectWorkspace(picked);
      if (controller.selectedWorkspace.deviceId == kCloudDeviceId) {
        _showCloudDeveloping();
      }
    }
  }

  Future<void> _openRunSettings() => showModalBottomSheet<void>(
    context: context,
    backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
    barrierColor: const Color(0x66000000),
    isScrollControlled: true,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(22)),
    ),
    builder: (context) => ListenableBuilder(
      listenable: controller,
      builder: (context, _) => _RunSettingsSheet(
        deviceName: controller.selectedDevice.kind == WbDeviceKind.cloud
            ? (stringsOf(context).mobileCloud118c7f15())
            : controller.selectedDevice.name,
        workspaceName: controller.selectedWorkspace.name,
        onDevice: () async {
          await _chooseDevice();
        },
        onWorkspace: _chooseWorkspace,
      ),
    ),
  );

  Future<void> _submit() async {
    if (_submitting) return;
    _submitting = true;
    try {
      await _submitOnce();
    } finally {
      _submitting = false;
    }
  }

  Future<void> _submitOnce() async {
    final localeStrings = stringsOf(context);

    final text = _composer.text.trim();
    if (text.isEmpty && (widget.resources?.draft.isEmpty ?? true)) return;
    if (controller.selectedDeviceId == kPendingDeviceId) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text((localeStrings.mobileConnectToDesktopFirstF10d2933())),
          action: SnackBarAction(
            label: localeStrings.commonDesktopSetupGuide(),
            onPressed: () => unawaited(openDesktopSetupGuide(context)),
          ),
        ),
      );
      return;
    }
    if (controller.selectedDeviceId == kCloudDeviceId) {
      _showCloudDeveloping();
      return;
    }
    var connection = _selectedDesktopConnection();
    if (connection == null &&
        controller.selectedDeviceId.startsWith('paired.')) {
      final deviceRef = controller.selectedDeviceId.substring('paired.'.length);
      try {
        await _restoreDesktopConnection(
          deviceRef: deviceRef,
          force: true,
        ).timeout(const Duration(seconds: 12));
      } on TimeoutException {
        // The retry continues in the background; the draft stays in the composer.
      }
      if (!mounted) return;
      connection = _selectedDesktopConnection();
      if (connection == null) {
        final failure = widget.pairedDesktop?.snapshot.failureMessage;
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              failure == null || failure.isEmpty
                  ? (localeStrings
                        .mobileConnectingToDesktopYourMessageHasBee706b5e0c())
                  : (localeStrings.mobileDesktopIsDisconnected2ddace9d(
                      value1: failure,
                    )),
            ),
          ),
        );
        return;
      }
    }
    if (connection != null) {
      final targetDeviceRef = connection.deviceRef;
      final intendedWorkspace = controller.selectedWorkspaceId,
          intendedTask = controller.openTaskId;
      String prompt = text;
      if (widget.resources?.draft.isEmpty == false) {
        try {
          prompt = await widget.resources!.preparePrompt(
            text,
            key: resourceOperationKey('prompt'),
            expectedPeer: targetDeviceRef,
          );
        } on Object catch (error) {
          if (mounted) showResourceError(context, error);
          return;
        }
        if (!mounted) return;
        if (controller.selectedDeviceId != 'paired.$targetDeviceRef' ||
            controller.selectedWorkspaceId != intendedWorkspace ||
            controller.openTaskId != intendedTask) {
          showResourceError(context, const ResourceFailure('STALE_GENERATION'));
          return;
        }
      }
      final task = controller.openTask;
      if (task != null && task.id.startsWith('dsh.session.')) {
        if (!_sessionHandle.ready) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(
                (localeStrings
                    .mobileConnectingToTheDesktopSessionWaitBef8b792e99()),
              ),
            ),
          );
          return;
        }
        _composer.clear();
        _focus.unfocus();
        try {
          await _sessionHandle.send(prompt);
          widget.resources?.draft.clear();
        } on Object catch (error) {
          if (mounted) {
            _composer.text = text;
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  (localeStrings.mobileUnableToSendF0d76162(value1: error)),
                ),
              ),
            );
          }
          unawaited(
            _restoreDesktopConnection(deviceRef: targetDeviceRef, force: true),
          );
        }
        return;
      }
      if (!controller.selectedWorkspace.id.startsWith('dsh.workspace.')) {
        await _publishDesktopCatalog(
          connection,
          'paired.${connection.deviceRef}',
        );
        if (!mounted) return;
      }
      final workspaceId = controller.selectedWorkspace.id;
      if (!workspaceId.startsWith('dsh.workspace.')) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              (localeStrings.mobileChooseADesktopWorkspaceFirst11a8134f()),
            ),
          ),
        );
        return;
      }
      _composer.clear();
      _focus.unfocus();
      var sessionConnection = connection;
      final paired = widget.pairedDesktop;
      final devices = paired?.devices.where(
        (device) => device.deviceRef == targetDeviceRef,
      );
      if (paired != null && devices != null && devices.isNotEmpty) {
        final renewed = await paired.connectDevice(devices.first, force: true);
        if (!renewed ||
            paired.snapshot.connection?.deviceRef != targetDeviceRef) {
          if (mounted) {
            _composer.text = text;
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  (localeStrings
                      .mobileDesktopAuthorizationHasExpiredReconnec2671ccbf()),
                ),
              ),
            );
          }
          return;
        }
        sessionConnection = paired.snapshot.connection!;
      }
      final client = DshNativeGatewayClient(
        origin: Uri.parse(sessionConnection.session.origin),
        bootstrapPath: sessionConnection.session.path,
        allowInsecureLoopback: sessionConnection.session.allowInsecureLoopback,
        allowInsecurePrivateNetworkForTesting:
            sessionConnection.session.allowInsecurePrivateNetworkForTesting,
      );
      try {
        await client.initialize();
        final created = await client.createSession(
          workspaceId: workspaceId.substring('dsh.workspace.'.length),
        );
        await _publishDesktopCatalog(
          sessionConnection,
          'paired.${sessionConnection.deviceRef}',
        );
        if (!mounted) return;
        setState(() {
          _pendingSessionId = created.sessionId;
          _pendingPrompt = prompt;
          _pendingOriginalText = text;
        });
        controller.openTaskById('dsh.session.${created.sessionId}');
      } on Object catch (error) {
        if (mounted) {
          _composer.text = text;
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(
                (localeStrings.mobileUnableToCreateADesktopSession370dc1c1(
                  value1: error,
                )),
              ),
            ),
          );
        }
      } finally {
        client.close();
      }
      return;
    }
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          (localeStrings.mobileConnectToAnOnlineDesktopFromTheDeviB83a25f3()),
        ),
      ),
    );
  }

  Future<void> _startVoice({SpeechAudioSource? source}) async {
    final localeStrings = stringsOf(context);

    final speech = widget.speechRecognition;
    if (speech == null || _speechOpening || _speechSession != null) return;
    if (_speechModels?.busy == true) {
      await _openSpeechModels();
      return;
    }
    if (source is! SpeechFileSource) {
      final probe = await speech.probe();
      if (probe.localEngine == SpeechEngineAvailability.needsModel && mounted) {
        await _offerSpeechModel();
        return;
      }
      if (!mounted || _speechOpening || _speechSession != null) return;
      if (probe.permission != SpeechPermissionState.granted) {
        final proceed = await _confirmMicrophoneUse();
        if (!proceed || !mounted || _speechOpening || _speechSession != null) {
          return;
        }
      }
    }

    final selection = _composer.selection;
    final textLength = _composer.text.length;
    final rawStart = selection.isValid ? selection.start : textLength;
    final rawEnd = selection.isValid ? selection.end : textLength;
    _speechDraft = SpeechDraftBuffer.begin(
      text: _composer.text,
      selectionStart: rawStart.clamp(0, textLength),
      selectionEnd: rawEnd.clamp(0, textLength),
    );
    _speechOpening = true;
    _stopSpeechWhenOpened = false;
    _cancelSpeechWhenOpened = false;
    _cancelVoiceOnRelease = false;
    _voiceReleased = false;
    setState(() => _holdingVoice = true);

    try {
      final session = await speech.open(
        SpeechStartRequest(
          requestId:
              'mobile-${DateTime.now().microsecondsSinceEpoch.toRadixString(16)}',
          source: source ?? const SpeechMicrophoneSource(),
          context: const SpeechContext(localeHints: ['zh-CN', 'en-US']),
        ),
      );
      if (!mounted) {
        await speech.cancel(session.ref);
        return;
      }
      _speechSession = session;
      _speechEvents = session.events.listen(_onSpeechEvent);
      if (_cancelSpeechWhenOpened) {
        await speech.cancel(session.ref);
        if (_speechSession == session) _restoreSpeechDraft();
      } else if (_stopSpeechWhenOpened) {
        await speech.stop(session.ref);
      }
    } on SpeechException catch (error) {
      _restoreSpeechDraft();
      if (error.code == 'model_missing') {
        await _offerSpeechModel();
      } else {
        _showSpeechFailure(error.safeMessage);
      }
    } on Object {
      _restoreSpeechDraft();
      _showSpeechFailure(
        (localeStrings.mobileUnableToStartDictationTryAgainLater01330b7f()),
      );
    } finally {
      _speechOpening = false;
    }
  }

  Future<void> _openSpeechModels() => Navigator.of(context).push<void>(
    MaterialPageRoute(
      builder: (_) => MobileSpeechModelsPage(manager: _speechModels),
    ),
  );

  Future<void> _offerSpeechModel() async {
    final s = stringsOf(context);
    final install = await showModalBottomSheet<bool>(
      context: context,
      showDragHandle: true,
      builder: (context) => SafeArea(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(24, 8, 24, 24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                s.mobileModels(),
                style: const TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 12),
              Text(
                s.mobileModelInstallPrompt(),
                style: const TextStyle(height: 1.5),
              ),
              const SizedBox(height: 8),
              Text(
                s.mobileModelDescription(),
                style: TextStyle(
                  fontSize: 13,
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 24),
              SizedBox(
                width: double.infinity,
                child: FilledButton.icon(
                  key: const ValueKey('wb-install-speech-model'),
                  onPressed: _speechModels == null
                      ? null
                      : () => Navigator.pop(context, true),
                  icon: const Icon(Icons.download_rounded, size: 18),
                  label: Text(s.mobileInstallModel()),
                ),
              ),
            ],
          ),
        ),
      ),
    );
    if (install == true && mounted) {
      unawaited(_speechModels!.install());
      await _openSpeechModels();
    }
  }

  Future<bool> _confirmMicrophoneUse() async {
    final localeStrings = stringsOf(context);

    final proceed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        key: const ValueKey('wb-microphone-purpose'),
        title: Text((localeStrings.mobileMicrophoneAccess86ee6902())),
        content: Text(
          (localeStrings.mobileOpenmuseUsesTheMicrophoneToTurnSpeec80d628f6()),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text((localeStrings.mobileNotNow5c258359())),
          ),
          TextButton(
            key: const ValueKey('wb-microphone-continue'),
            onPressed: () => Navigator.pop(context, true),
            child: Text((localeStrings.mobileContinueDcd5e1f3())),
          ),
        ],
      ),
    );
    return proceed ?? false;
  }

  Future<void> _stopVoice() async {
    final localeStrings = stringsOf(context);

    if (_cancelVoiceOnRelease) {
      await _cancelVoice();
      return;
    }
    if (mounted && _holdingVoice) setState(() => _voiceReleased = true);
    _stopSpeechWhenOpened = true;
    final speech = widget.speechRecognition;
    final session = _speechSession;
    if (speech == null || session == null) return;
    try {
      await speech.stop(session.ref);
    } on SpeechException catch (error) {
      _restoreSpeechDraft();
      _showSpeechFailure(error.safeMessage);
    } on Object {
      _restoreSpeechDraft();
      _showSpeechFailure(
        (localeStrings.mobileUnableToFinishDictationTryAgainLaterF198d1db()),
      );
    }
  }

  Future<void> _cancelVoice() async {
    final localeStrings = stringsOf(context);

    _cancelSpeechWhenOpened = true;
    final speech = widget.speechRecognition;
    final session = _speechSession;
    if (speech == null || session == null) return;
    try {
      await speech.cancel(session.ref);
      if (_speechSession == session) _restoreSpeechDraft();
    } on Object {
      _restoreSpeechDraft();
      _showSpeechFailure(
        (localeStrings.mobileUnableToCancelDictationTryAgainLater0bdbf4b0()),
      );
    }
  }

  void _updateVoiceGesture(LongPressMoveUpdateDetails details) {
    final cancel = details.offsetFromOrigin.dy < -72;
    if (cancel != _cancelVoiceOnRelease && mounted) {
      setState(() => _cancelVoiceOnRelease = cancel);
    }
  }

  void _toggleInputMode() {
    if (_holdingVoice || _speechOpening) return;
    final nextVoiceMode = !_voiceMode;
    if (nextVoiceMode) {
      _focus.unfocus();
      unawaited(SystemChannels.textInput.invokeMethod<void>('TextInput.hide'));
    }
    setState(() => _voiceMode = nextVoiceMode);
    if (!nextVoiceMode) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _focus.requestFocus();
      });
    }
  }

  void _onSpeechEvent(SpeechEvent event) {
    if (!mounted || event.session != _speechSession?.ref) return;
    switch (event.kind) {
      case SpeechEventKind.partial:
        _applySpeechSnapshot(_speechDraft?.applyPartial(event.text ?? ''));
      case SpeechEventKind.finalResult:
        final result = event.text?.trim() ?? '';
        if (result.isEmpty) {
          _restoreSpeechDraft();
          _showSpeechFailure(
            (stringsOf(context).mobileNoSpeechWasRecognizedTryAgain8a4b2ce5()),
          );
          break;
        }
        _applySpeechSnapshot(_speechDraft?.applyFinal(result));
        if (widget.debugSpeechSource != null) {
          debugPrint('OPENMUSE_ASR_E2E_FINAL=$result');
        }
        _finishSpeech(revealText: true);
      case SpeechEventKind.error:
        _restoreSpeechDraft();
        _showSpeechFailure(
          event.safeMessage ??
              (stringsOf(
                context,
              ).mobileSpeechRecognitionFailedTryAgain965b77bb()),
        );
      case SpeechEventKind.state:
        if (event.phase == SpeechSessionPhase.cancelled) {
          _restoreSpeechDraft();
        }
    }
  }

  void _applySpeechSnapshot(SpeechDraftSnapshot? snapshot) {
    if (snapshot == null) return;
    _composer.value = TextEditingValue(
      text: snapshot.text,
      selection: TextSelection(
        baseOffset: snapshot.selectionStart,
        extentOffset: snapshot.selectionEnd,
      ),
    );
  }

  void _restoreSpeechDraft() {
    _applySpeechSnapshot(_speechDraft?.cancel());
    _finishSpeech();
  }

  void _finishSpeech({bool revealText = false}) {
    unawaited(_speechEvents?.cancel());
    _speechEvents = null;
    _speechSession = null;
    _speechDraft = null;
    _stopSpeechWhenOpened = false;
    _cancelSpeechWhenOpened = false;
    _cancelVoiceOnRelease = false;
    if (mounted) {
      setState(() {
        _holdingVoice = false;
        _voiceReleased = false;
        if (revealText) _voiceMode = false;
      });
    }
  }

  void _showSpeechFailure(String message) {
    debugPrint('OPENMUSE_ASR_E2E_ERROR=$message');
    if (!mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(message)));
  }

  Future<void> _choosePermission() async {
    final localeStrings = stringsOf(context);

    final options = _sessionHandle.options?.permissions ?? const [];
    if (options.isEmpty) {
      _showSessionControlUnavailable(
        (localeStrings.mobileOpenADesktopSessionBeforeChangingWorC6752606()),
      );
      return;
    }
    final current = _sessionHandle.snapshot?.permissionPreset;
    final picked = await showModalBottomSheet<DshNativePermissionOption>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => _ChoiceSheet(
        title: (localeStrings.mobileWorkspacePermissionsEb2b5fb3()),
        children: [
          for (final option in options)
            ListTile(
              key: ValueKey('wb-permission-${option.value}'),
              leading: Icon(
                option.value == 'danger-full-access'
                    ? Icons.warning_amber_rounded
                    : Icons.admin_panel_settings_outlined,
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
              title: Text(
                _permissionLabel(context, option.value),
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                ),
              ),
              subtitle: option.description == null
                  ? null
                  : Text(option.description!, style: wbSub),
              trailing: current == option.value
                  ? WbIcon(WbGlyph.check, size: 18)
                  : null,
              onTap: () => Navigator.pop(context, option),
            ),
        ],
      ),
    );
    if (picked == null || picked.value == current) return;
    if (!mounted) return;
    if (picked.value == 'danger-full-access' || picked.value == 'auto') {
      final confirmed = await showDialog<bool>(
        context: context,
        builder: (context) => AlertDialog(
          backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
          title: Text(
            picked.value == 'auto'
                ? (localeStrings.mobileEnableAutomaticApproval424e38bb())
                : (localeStrings.mobileEnableFullAccess91562a40()),
            style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
          ),
          content: Text(
            picked.description ??
                (localeStrings
                    .mobileThisExpandsTheToolsAndFileOperations6adfe213()),
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: Text((localeStrings.mobileCancelCda6010b())),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(context, true),
              child: Text((localeStrings.mobileConfirm7f18937e())),
            ),
          ],
        ),
      );
      if (confirmed != true) return;
    }
    try {
      await _sessionHandle.selectPermission(picked.value);
    } on Object catch (error) {
      _showSessionControlUnavailable(
        (localeStrings.mobileUnableToChangePermissionsD8c46f1a(value1: error)),
      );
    }
  }

  Future<void> _chooseModel() async {
    final localeStrings = stringsOf(context);

    final options = _sessionHandle.options?.models ?? const [];
    if (options.isEmpty) {
      _showSessionControlUnavailable(
        (localeStrings.mobileDesktopDidNotReturnAnAvailableModel5472c170()),
      );
      return;
    }
    final snapshot = _sessionHandle.snapshot;
    final picked = await showModalBottomSheet<DshNativeModelOption>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      isScrollControlled: true,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => _ChoiceSheet(
        title: (localeStrings.mobileChooseAModelD992696e()),
        children: [
          for (final option in options)
            ListTile(
              key: ValueKey('wb-model-${option.provider}-${option.id}'),
              leading: Icon(
                Icons.auto_awesome_outlined,
                color: Theme.of(context).colorScheme.onSurfaceVariant,
              ),
              title: Text(
                option.name,
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                ),
              ),
              subtitle: Text(
                option.description == null
                    ? option.providerName
                    : '${option.providerName} · ${option.description}',
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: wbSub,
              ),
              trailing:
                  snapshot?.provider == option.provider &&
                      snapshot?.model == option.id
                  ? WbIcon(WbGlyph.check, size: 18)
                  : null,
              onTap: () => Navigator.pop(context, option),
            ),
        ],
      ),
    );
    if (picked == null) return;
    if (!mounted) return;
    String? effort;
    if (picked.efforts.isNotEmpty) {
      effort = await showModalBottomSheet<String>(
        context: context,
        backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        builder: (context) => _ChoiceSheet(
          title: (localeStrings.mobileReasoningEffort73c10416()),
          children: [
            for (final value in picked.efforts)
              ListTile(
                key: ValueKey('wb-effort-${value.id}'),
                title: Text(
                  value.name,
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.onSurface,
                  ),
                ),
                subtitle: value.description == null
                    ? null
                    : Text(value.description!, style: wbSub),
                trailing: snapshot?.reasoningEffort == value.id
                    ? WbIcon(WbGlyph.check, size: 18)
                    : null,
                onTap: () => Navigator.pop(context, value.id),
              ),
          ],
        ),
      );
      if (effort == null) return;
    }
    try {
      await _sessionHandle.selectModel(picked, effort: effort);
    } on Object catch (error) {
      _showSessionControlUnavailable(
        (localeStrings.mobileUnableToSwitchModels602eabea(value1: error)),
      );
    }
  }

  Future<void> _showAttachmentActions() async {
    final localeStrings = stringsOf(context);

    await showModalBottomSheet<void>(
      context: context,
      backgroundColor: Theme.of(context).colorScheme.surfaceContainer,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => _ChoiceSheet(
        title: (localeStrings.mobileAddContent2b54df48()),
        children: [
          if (widget.resources != null) ...[
            ListTile(
              leading: const Icon(Icons.folder_outlined),
              title: Text(localeStrings.resourcecenterBrowseLocal()),
              onTap: () {
                Navigator.pop(context);
                controller.selectTab(WbTab.home);
              },
            ),
          ],
          if (widget.onOpenWord != null)
            ListTile(
              leading: const Icon(Icons.description_outlined),
              title: Text((localeStrings.mobileWordDocument793e039c())),
              subtitle: Text(
                (localeStrings.mobileOpenDocxFilesOfflineF0b63475()),
              ),
              onTap: () {
                Navigator.pop(context);
                widget.onOpenWord!();
              },
            ),
          if (widget.onOpenPdf != null)
            ListTile(
              leading: const Icon(Icons.picture_as_pdf_outlined),
              title: Text((localeStrings.mobilePdfDocument54149cdd())),
              subtitle: Text(
                (localeStrings.mobileOpenPdfFilesOffline8ea93832()),
              ),
              onTap: () {
                Navigator.pop(context);
                widget.onOpenPdf!();
              },
            ),
          if (widget.onOpenSpreadsheet != null)
            ListTile(
              leading: const Icon(Icons.table_chart_outlined),
              title: Text((localeStrings.mobileSpreadsheet2a8d5279())),
              subtitle: Text(
                (localeStrings.mobileOpenXlsxAndCsvFilesOfflineBd039729()),
              ),
              onTap: () {
                Navigator.pop(context);
                widget.onOpenSpreadsheet!();
              },
            ),
          ListTile(
            leading: Icon(
              Icons.folder_outlined,
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
            title: Text(
              (localeStrings.mobileAddAFileFromDesktop90bc2c94()),
              style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
            ),
            subtitle: Text(
              (localeStrings
                  .mobileAttachmentsAreComingInAFutureUpdate7e6cb9d3()),
              style: wbSub,
            ),
            enabled: false,
          ),
        ],
      ),
    );
  }

  void _showSessionControlUnavailable(String message) {
    if (!mounted) return;
    ScaffoldMessenger.of(
      context,
    ).showSnackBar(SnackBar(content: Text(message)));
  }

  Future<void> _openMobileMarket({bool initialMine = false}) async {
    if (RuntimeConfigScope.maybeOf(
          context,
        )?.isEnabled(AppFeature.pluginMarket) ==
        false) {
      return;
    }
    final registered = widget.pluginRegistry?.plugin(
      'com.openmuse.mobile-plugin-market',
    );
    final market = widget.pluginRegistry == null
        ? widget.mobileMarketPlugin
        : registered is OpenMuseMobilePluginMarket
        ? registered
        : null;
    if (market == null) return;
    controller.closeDrawer();
    final connection = widget.pairedDesktop?.snapshot.connection;
    final desktop = connection == null
        ? null
        : PairedDesktopMarketClient(
            origin: Uri.parse(connection.session.origin),
            grantRef: connection.grantRef,
            expiresAtMs: connection.expiresAtMs,
          );
    Widget inventory(VoidCallback? onDiscover) => market.buildMarketPage(
      desktop: initialMine ? null : desktop,
      mode: initialMine
          ? MobileMarketMode.installed
          : MobileMarketMode.discover,
      onDiscover: onDiscover,
      pluginRegistry: widget.pluginRegistry,
      onOpenBuiltIn: _openBuiltInPlugin,
    );
    if (initialMine) {
      await Navigator.of(context).push<void>(
        InstalledPluginsRoute(
          buddy: controller,
          returnToInventory: _managementReturn,
          inventory: (onDiscover) => inventory(onDiscover),
        ),
      );
    } else {
      await controller.selectAppsSection(AppsSection.discover);
    }
  }

  void _openBuiltInPlugin(OpenMusePlugin plugin) {
    if (plugin is! OpenMuseMobileWorkbenchContributor) return;
    Navigator.of(context).push<void>(
      MaterialPageRoute(
        builder: (_) => MobilePluginDetailPage(
          plugin: plugin,
          contribution: plugin as OpenMuseMobileWorkbenchContributor,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.sizeOf(context).width;
    final drawerWidth = width * 0.84;
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: Theme.of(context).brightness == Brightness.dark
          ? SystemUiOverlayStyle.light
          : SystemUiOverlayStyle.dark,
      child: Scaffold(
        backgroundColor: Theme.of(context).scaffoldBackgroundColor,
        resizeToAvoidBottomInset: true,
        body: AnimatedBuilder(
          animation: _drawer,
          builder: (context, _) {
            final shift = drawerWidth * _drawer.value;
            return Stack(
              children: [
                Transform.translate(
                  offset: Offset(shift, 0),
                  child: _MainColumn(
                    controller: controller,
                    pairedDesktop: widget.pairedDesktop,
                    mobileMarket: widget.mobileMarketPlugin,
                    pluginRegistry: widget.pluginRegistry,
                    onOpenBuiltIn: _openBuiltInPlugin,
                    homePage: widget.resources == null
                        ? ResourceCenterPlaceholder(
                            error: widget.resourceFailure,
                            onRetry: widget.onRetryResources,
                          )
                        : ResourceCenterView(
                            controller: widget.resources!.controller,
                            onAddToConversation: (record) async {
                              try {
                                widget.resources!.draft.attach(record);
                                controller.selectTab(WbTab.tasks);
                              } on Object catch (error) {
                                if (mounted) showResourceError(context, error);
                              }
                            },
                          ),
                    resourceDraft: widget.resources?.draft,
                    workbenchKey: _workbenchKey,
                    appsEditing: _appsEditing,
                    managementReturn: _managementReturn,
                    workbenchPage: widget.pluginRegistry == null
                        ? Center(
                            child: Text(
                              (stringsOf(
                                context,
                              ).mobileTheWorkbenchIsUnavailable9160d296()),
                            ),
                          )
                        : MobileWorkbenchPage(
                            key: _workbenchKey,
                            embedded: true,
                            onEditingChanged: (value) =>
                                _appsEditing.value = value,
                            registry: widget.pluginRegistry!,
                            values:
                                widget.deviceSelectionStore ??
                                const FlutterSecureValueStore(),
                            subject:
                                widget
                                    .authentication
                                    ?.snapshot
                                    .identity
                                    ?.subject ??
                                'local',
                            scope:
                                '${controller.selectedDeviceId}.'
                                '${controller.selectedWorkspaceId}',
                            onDiscover: () => controller.selectAppsSection(
                              AppsSection.discover,
                            ),
                            onManagePlugins: () =>
                                unawaited(_openMobileMarket(initialMine: true)),
                          ),
                    minePage: MobileMyPage(
                      themeMode: widget.themeMode,
                      onThemeMode: widget.onThemeMode ?? (_) {},
                      workspaces: controller.workspacesFor(
                        controller.selectedDeviceId,
                      ),
                      deviceName: controller.selectedDevice.name,
                      authentication: widget.authentication,
                      onSignIn: () => unawaited(_openLogin()),
                      onSignOut: widget.onSignOut == null
                          ? null
                          : () => unawaited(widget.onSignOut!()),
                      onDeleteAccount: widget.onDeleteAccount,
                      registryProviders:
                          widget.mobileMarketPlugin?.registryProviders,
                      speechModels: _speechModels,
                      onDevices: () => unawaited(_chooseDevice()),
                      onPlugins: () =>
                          unawaited(_openMobileMarket(initialMine: true)),
                    ),
                    composer: _composer,
                    focus: _focus,
                    voiceMode: _voiceMode,
                    holdingVoice: _holdingVoice,
                    voiceReleased: _voiceReleased,
                    cancellingVoice: _cancelVoiceOnRelease,
                    onMenu: () {
                      final connection =
                          widget.pairedDesktop?.snapshot.connection;
                      final deviceId = connection == null
                          ? null
                          : 'paired.${connection.deviceRef}';
                      if (connection != null &&
                          deviceId == controller.selectedDeviceId) {
                        unawaited(
                          _publishDesktopCatalog(connection, deviceId!),
                        );
                      }
                      controller.toggleDrawer();
                    },
                    onRunSettings: _openRunSettings,
                    onReconnect: () {
                      final selectedId = controller.selectedDeviceId;
                      if (!selectedId.startsWith('paired.')) return;
                      unawaited(
                        _restoreDesktopConnection(
                          deviceRef: selectedId.substring('paired.'.length),
                          force: true,
                        ),
                      );
                    },
                    onSubmit: _submit,
                    onInputModeToggle: _toggleInputMode,
                    onVoiceDown: () => unawaited(_startVoice()),
                    onVoiceUp: () => unawaited(_stopVoice()),
                    onVoiceMove: _updateVoiceGesture,
                    onNewTask: () async {
                      setState(() {
                        _pendingSessionId = null;
                        _pendingPrompt = null;
                      });
                      controller.startNewTask();
                      await _openRunSettings();
                      if (mounted) _focus.requestFocus();
                    },
                    conversationBuilder: _nativeConversation,
                    sessionHandle: _sessionHandle,
                    onPermission: _choosePermission,
                    onModel: _chooseModel,
                    onAttachment: _showAttachmentActions,
                  ),
                ),
                if (_drawer.value > 0)
                  Positioned(
                    left: 0,
                    top: 0,
                    bottom: 0,
                    width: drawerWidth,
                    child: _DrawerPanel(
                      controller: controller,
                      onNewTask: () async {
                        controller.startNewTask();
                        await _openRunSettings();
                        if (mounted) _focus.requestFocus();
                      },
                      onDevice: _chooseDevice,
                      onOpenSession: _openDesktopSession,
                    ),
                  ),
                if (_drawer.value > 0.95)
                  Positioned(
                    left: drawerWidth,
                    right: 0,
                    top: 0,
                    bottom: 0,
                    child: GestureDetector(
                      key: const ValueKey('wb-drawer-scrim'),
                      behavior: HitTestBehavior.opaque,
                      onTap: controller.closeDrawer,
                    ),
                  ),
                if (widget.pairedDesktop?.snapshot.connection
                    case final connection?)
                  if (controller.selectedDeviceId ==
                      'paired.${connection.deviceRef}')
                    PairedPluginInteractionLayer(connection: connection),
              ],
            );
          },
        ),
      ),
    );
  }

  Widget? _nativeConversation() {
    final connection = widget.pairedDesktop?.snapshot.connection;
    if (connection == null) return null;
    final tasksBySessionId = {
      for (final task in controller.tasks)
        if (task.id.startsWith('dsh.session.') &&
            task.deviceId == 'paired.${connection.deviceRef}')
          task.id.substring('dsh.session.'.length): task,
    };
    final cached = _recentNativeSessionIds
        .where(tasksBySessionId.containsKey)
        .toList(growable: false);
    if (cached.isEmpty) return null;
    final selectedTask = controller.openTask;
    final selectedId =
        selectedTask != null &&
            selectedTask.deviceId == 'paired.${connection.deviceRef}' &&
            selectedTask.id.startsWith('dsh.session.')
        ? selectedTask.id.substring('dsh.session.'.length)
        : null;
    final index = cached.indexOf(selectedId ?? '');
    final workspaceNames = {
      for (final workspace in controller.workspaces)
        workspace.id: workspace.name,
    };
    return IndexedStack(
      index: index < 0 ? 0 : index,
      children: [
        for (final sessionId in cached)
          NativeDshPage(
            key: ValueKey(
              'wb-native-session-${connection.grantRef}-$sessionId',
            ),
            session: connection.session,
            workspaceTitle:
                workspaceNames[tasksBySessionId[sessionId]!.workspaceId] ?? '',
            requestedSessionId: sessionId,
            active: selectedId == sessionId && controller.tab == WbTab.tasks,
            embedded: true,
            showComposer: false,
            handle: selectedId == sessionId && controller.tab == WbTab.tasks
                ? _sessionHandle
                : null,
            initialPrompt: _pendingSessionId == sessionId
                ? _pendingPrompt
                : null,
            onInitialPromptDeclined: (_) {
              if (!mounted || _pendingSessionId != sessionId) return;
              setState(() {
                if (_composer.text.isEmpty) {
                  _composer.text = _pendingOriginalText ?? '';
                }
                _pendingPrompt = null;
                _pendingOriginalText = null;
                _pendingSessionId = null;
              });
            },
            onInitialPromptConsumed: () {
              if (!mounted || _pendingSessionId != sessionId) return;
              widget.resources?.draft.clear();
              setState(() {
                _pendingSessionId = null;
                _pendingPrompt = null;
                _pendingOriginalText = null;
              });
            },
            onConnectionFailure: () {
              if (widget.pairedDesktop?.snapshot.connection?.grantRef !=
                  connection.grantRef) {
                return;
              }
              unawaited(
                _restoreDesktopConnection(
                  deviceRef: connection.deviceRef,
                  force: true,
                ),
              );
            },
          ),
      ],
    );
  }

  void _showCloudDeveloping() {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          (stringsOf(context).mobileCloudConversationsAreComingSoonEcd03ff2()),
        ),
      ),
    );
  }
}

final class _MainColumn extends StatelessWidget {
  const _MainColumn({
    required this.controller,
    required this.pairedDesktop,
    required this.mobileMarket,
    required this.pluginRegistry,
    required this.onOpenBuiltIn,
    required this.homePage,
    required this.workbenchPage,
    required this.workbenchKey,
    required this.appsEditing,
    required this.managementReturn,
    this.resourceDraft,
    required this.minePage,
    required this.composer,
    required this.focus,
    required this.voiceMode,
    required this.holdingVoice,
    required this.voiceReleased,
    required this.cancellingVoice,
    required this.onMenu,
    required this.onRunSettings,
    required this.onReconnect,
    required this.onSubmit,
    required this.onInputModeToggle,
    required this.onVoiceDown,
    required this.onVoiceUp,
    required this.onVoiceMove,
    required this.onNewTask,
    required this.conversationBuilder,
    required this.sessionHandle,
    required this.onPermission,
    required this.onModel,
    required this.onAttachment,
  });

  final WorkBuddyController controller;
  final PairedDesktopMobileController? pairedDesktop;
  final OpenMuseMobilePluginMarket? mobileMarket;
  final OpenMusePluginRegistry? pluginRegistry;
  final ValueChanged<OpenMusePlugin> onOpenBuiltIn;
  final ResourceConversationDraft? resourceDraft;
  final Widget homePage;
  final Widget workbenchPage;
  final GlobalKey<MobileWorkbenchPageState> workbenchKey;
  final ValueListenable<bool> appsEditing;
  final ValueListenable<VoidCallback?> managementReturn;
  final Widget minePage;
  final TextEditingController composer;
  final FocusNode focus;
  final bool voiceMode;
  final bool holdingVoice;
  final bool voiceReleased;
  final bool cancellingVoice;
  final VoidCallback onMenu;
  final VoidCallback onRunSettings;
  final VoidCallback onReconnect;
  final VoidCallback onSubmit;
  final VoidCallback onInputModeToggle;
  final VoidCallback onVoiceDown;
  final VoidCallback onVoiceUp;
  final ValueChanged<LongPressMoveUpdateDetails> onVoiceMove;
  final VoidCallback onNewTask;
  final Widget? Function() conversationBuilder;
  final NativeDshSessionHandle sessionHandle;
  final VoidCallback onPermission;
  final VoidCallback onModel;
  final VoidCallback onAttachment;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: Listenable.merge([controller, ?resourceDraft]),
    builder: (context, _) {
      final installedMarket = pluginRegistry?.plugin(
        'com.openmuse.mobile-plugin-market',
      );
      final activeMarket = pluginRegistry == null
          ? mobileMarket
          : installedMarket is OpenMuseMobilePluginMarket
          ? installedMarket
          : null;
      final showComposer = controller.tab == WbTab.tasks;
      final light = Theme.of(context).brightness == Brightness.light;
      final snapshot = sessionHandle.snapshot;
      final conversation = conversationBuilder();
      final selectedTask = controller.openTask;
      final nativeTaskSelected =
          selectedTask != null && selectedTask.id.startsWith('dsh.session.');
      return AnnotatedRegion<SystemUiOverlayStyle>(
        value: light ? SystemUiOverlayStyle.dark : SystemUiOverlayStyle.light,
        child: ColoredBox(
          color: Theme.of(context).scaffoldBackgroundColor,
          child: SafeArea(
            child: Stack(
              children: [
                Column(
                  children: [
                    if (showComposer)
                      _HomeHeader(
                        deviceName:
                            controller.selectedDevice.kind == WbDeviceKind.cloud
                            ? (stringsOf(context).mobileCloud118c7f15())
                            : controller.selectedDevice.name,
                        workspaceName: controller.selectedWorkspace.name,
                        onMenu: onMenu,
                        onSubtitle: onRunSettings,
                      ),
                    if (showComposer &&
                        controller.selectedDeviceId.startsWith('paired.') &&
                        pairedDesktop != null)
                      ListenableBuilder(
                        listenable: pairedDesktop!,
                        builder: (context, _) {
                          final connection = pairedDesktop!.snapshot.connection;
                          final connected =
                              connection != null &&
                              controller.selectedDeviceId ==
                                  'paired.${connection.deviceRef}' &&
                              connection.expiresAtMs >
                                  DateTime.now()
                                      .add(const Duration(minutes: 1))
                                      .millisecondsSinceEpoch;
                          if (connected) return const SizedBox.shrink();
                          final snapshot = pairedDesktop!.snapshot;
                          final message = snapshot.connecting
                              ? (stringsOf(
                                  context,
                                ).mobileConnectingToDesktop19a52f17())
                              : snapshot.failureMessage ??
                                    (stringsOf(
                                      context,
                                    ).mobileDesktopIsDisconnected5be7fb00());
                          return Padding(
                            padding: const EdgeInsets.fromLTRB(20, 0, 20, 4),
                            child: Row(
                              children: [
                                Expanded(
                                  child: Text(
                                    message,
                                    key: const ValueKey(
                                      'wb-desktop-connection-status',
                                    ),
                                    style: TextStyle(
                                      color: Theme.of(
                                        context,
                                      ).colorScheme.onSurfaceVariant,
                                      fontSize: 13,
                                    ),
                                  ),
                                ),
                                TextButton(
                                  key: const ValueKey('wb-reconnect-desktop'),
                                  onPressed: snapshot.connecting
                                      ? null
                                      : onReconnect,
                                  child: Text(
                                    (stringsOf(
                                      context,
                                    ).mobileReconnect87c50d4d()),
                                  ),
                                ),
                              ],
                            ),
                          );
                        },
                      ),
                    if (showComposer &&
                        controller.selectedDevice.kind == WbDeviceKind.cloud)
                      Padding(
                        padding: EdgeInsets.fromLTRB(20, 0, 20, 4),
                        child: Align(
                          alignment: Alignment.centerLeft,
                          child: Text(
                            (stringsOf(
                              context,
                            ).mobileCloudConversationsAreComingSoonEcd03ff2()),
                            key: ValueKey('wb-cloud-developing'),
                            style: TextStyle(
                              color: Theme.of(
                                context,
                              ).colorScheme.onSurfaceVariant,
                              fontSize: 13,
                            ),
                          ),
                        ),
                      ),
                    Expanded(
                      key: const ValueKey('wb-body-pages'),
                      child: _TabBody(
                        controller: controller,
                        onNewTask: onNewTask,
                        conversation: conversation,
                        homePage: homePage,
                        workbenchPage: workbenchPage,
                        workbenchKey: workbenchKey,
                        appsEditing: appsEditing,
                        managementReturn: managementReturn,
                        minePage: minePage,
                        discoverPage:
                            RuntimeConfigScope.maybeOf(
                                  context,
                                )?.isEnabled(AppFeature.pluginMarket) ==
                                false
                            ? Center(
                                child: Text(
                                  stringsOf(context).commonFeatureUnavailable(),
                                ),
                              )
                            : activeMarket?.buildMarketPage(
                                embedded: true,
                                mode: MobileMarketMode.discover,
                                pluginRegistry: pluginRegistry,
                                onOpenBuiltIn: onOpenBuiltIn,
                                onManageWorkbench: () => controller
                                    .selectAppsSection(AppsSection.overview),
                                desktop:
                                    pairedDesktop?.snapshot.connection == null
                                    ? null
                                    : PairedDesktopMarketClient(
                                        origin: Uri.parse(
                                          pairedDesktop!
                                              .snapshot
                                              .connection!
                                              .session
                                              .origin,
                                        ),
                                        grantRef: pairedDesktop!
                                            .snapshot
                                            .connection!
                                            .grantRef,
                                        expiresAtMs: pairedDesktop!
                                            .snapshot
                                            .connection!
                                            .expiresAtMs,
                                      ),
                              ),
                      ),
                    ),
                    if (showComposer && resourceDraft != null)
                      ConstrainedBox(
                        constraints: BoxConstraints(
                          maxHeight: MediaQuery.viewInsetsOf(context).bottom > 0
                              ? 110
                              : 230,
                        ),
                        child: SingleChildScrollView(
                          child: ResourceDraftStrip(draft: resourceDraft!),
                        ),
                      ),
                    if (showComposer)
                      _Composer(
                        hasAttachments: resourceDraft?.isEmpty == false,
                        onPasteResource: resourceDraft == null
                            ? null
                            : () async {
                                try {
                                  await resourceDraft!.paste();
                                } on Object catch (error) {
                                  if (context.mounted) {
                                    showResourceError(context, error);
                                  }
                                }
                              },
                        controller: composer,
                        focus: focus,
                        voiceMode: voiceMode,
                        holding: holdingVoice,
                        onSubmit: onSubmit,
                        onInputModeToggle: onInputModeToggle,
                        onVoiceDown: onVoiceDown,
                        onVoiceUp: onVoiceUp,
                        onVoiceMove: onVoiceMove,
                        onPermission: onPermission,
                        onModel: onModel,
                        onAttachment: onAttachment,
                        onCancel: sessionHandle.cancel,
                        permissionLabel: _permissionLabel(
                          context,
                          snapshot?.permissionPreset,
                        ),
                        modelLabel: _workBuddyModelLabel(
                          context,
                          snapshot?.model,
                          snapshot?.reasoningEffort,
                        ),
                        running: snapshot?.running ?? false,
                        enabled: !nativeTaskSelected || sessionHandle.ready,
                      ),
                    _TabBar(controller: controller),
                  ],
                ),
                if (holdingVoice)
                  Positioned(
                    left: 0,
                    right: 0,
                    bottom: 0,
                    child: IgnorePointer(
                      child: _VoiceHold(
                        cancelling: cancellingVoice,
                        released: voiceReleased,
                        composer: composer,
                      ),
                    ),
                  ),
              ],
            ),
          ),
        ),
      );
    },
  );
}

final class _HomeHeader extends StatelessWidget {
  const _HomeHeader({
    required this.deviceName,
    required this.workspaceName,
    required this.onMenu,
    required this.onSubtitle,
  });

  final String deviceName;
  final String workspaceName;
  final VoidCallback onMenu;
  final VoidCallback onSubtitle;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.fromLTRB(16, 8, 16, 4),
    child: Row(
      children: [
        _CircleButton(
          key: const ValueKey('wb-menu'),
          onTap: onMenu,
          child: WbIcon(WbGlyph.menu, size: 18),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('OpenMuse', key: ValueKey('wb-title'), style: wbTitle),
              const SizedBox(height: 3),
              GestureDetector(
                key: const ValueKey('wb-device-workspace'),
                onTap: onSubtitle,
                behavior: HitTestBehavior.opaque,
                child: Row(
                  children: [
                    WbIcon(
                      WbGlyph.desktop,
                      size: 14,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                    const SizedBox(width: 4),
                    Flexible(
                      child: Text(
                        deviceName,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: wbSub,
                      ),
                    ),
                    Text('  |  ', style: wbSub),
                    Flexible(
                      child: Text(
                        workspaceName,
                        key: const ValueKey('wb-workspace-label'),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: wbSub,
                      ),
                    ),
                    const SizedBox(width: 2),
                    WbIcon(
                      WbGlyph.chevron,
                      size: 12,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ],
    ),
  );
}

final class _TabBody extends StatelessWidget {
  const _TabBody({
    required this.controller,
    required this.onNewTask,
    required this.conversation,
    required this.discoverPage,
    required this.homePage,
    required this.workbenchPage,
    required this.workbenchKey,
    required this.appsEditing,
    required this.managementReturn,
    required this.minePage,
  });

  final WorkBuddyController controller;
  final VoidCallback onNewTask;
  final Widget? conversation;
  final Widget? discoverPage;
  final Widget homePage;
  final Widget workbenchPage;
  final GlobalKey<MobileWorkbenchPageState> workbenchKey;
  final ValueListenable<bool> appsEditing;
  final ValueListenable<VoidCallback?> managementReturn;
  final Widget minePage;

  @override
  Widget build(BuildContext context) {
    final task = controller.openTask;
    final nativeSelected =
        controller.tab == WbTab.tasks &&
        task != null &&
        task.id.startsWith('dsh.session.');
    final ordinary = _ordinaryTab(context);
    final cachedConversation = conversation;
    return Stack(
      fit: StackFit.expand,
      children: [
        Offstage(offstage: nativeSelected, child: ordinary),
        if (cachedConversation != null)
          Offstage(offstage: !nativeSelected, child: cachedConversation),
      ],
    );
  }

  Widget _ordinaryTab(BuildContext context) {
    final pages = <Widget>[
      homePage,
      controller.openTask != null &&
              !controller.openTask!.id.startsWith('dsh.session.')
          ? _Conversation(task: controller.openTask!)
          : const _HomeEmpty(),
      MobileAppsPage(
        key: ValueKey('apps-center-${controller.accountName ?? 'local'}'),
        controller: controller,
        overview: workbenchPage,
        discovery:
            discoverPage ??
            Center(
              child: Text(
                stringsOf(context).mobileDiscoveryIsUnavailable52d7df72(),
              ),
            ),
        workbenchKey: workbenchKey,
        editing: appsEditing,
        managementReturn: managementReturn,
      ),
      minePage,
    ];
    return IndexedStack(
      index: controller.tab.index,
      children: [
        for (var i = 0; i < pages.length; i++)
          TickerMode(enabled: controller.tab.index == i, child: pages[i]),
      ],
    );
  }
}

final class _HomeEmpty extends StatelessWidget {
  const _HomeEmpty();

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constraints) {
      final height = constraints.maxHeight;
      final showSlogan = height >= 145;
      final gap = height >= 300 ? 28.0 : 12.0;
      final imageHeight = showSlogan
          ? (height - gap - 64).clamp(0.0, 156.0)
          : height.clamp(0.0, 120.0);
      return Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Image.asset(
              'assets/brand.png',
              key: const ValueKey('wb-mascot'),
              width: 176,
              height: imageHeight,
              fit: BoxFit.contain,
              errorBuilder: (_, _, _) => SizedBox(
                width: 176,
                height: imageHeight,
                child: CustomPaint(painter: _MascotFallbackPainter()),
              ),
            ),
            if (showSlogan) ...[
              SizedBox(height: gap),
              FittedBox(
                fit: BoxFit.scaleDown,
                child: Text(
                  (stringsOf(context).mobileCreateWithOpenmuseC794b360()),
                  key: ValueKey('wb-slogan'),
                  maxLines: 1,
                  style: wbSlogan,
                ),
              ),
            ],
          ],
        ),
      );
    },
  );
}

final class _Conversation extends StatefulWidget {
  const _Conversation({required this.task});

  final WbTask task;

  @override
  State<_Conversation> createState() => _ConversationState();
}

final class _ConversationState extends State<_Conversation> {
  final _scroll = ScrollController();

  @override
  void dispose() {
    _scroll.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Stack(
    children: [
      ListView(
        controller: _scroll,
        padding: const EdgeInsets.fromLTRB(18, 8, 18, 56),
        children: [
          Center(
            child: Text(
              (stringsOf(context).mobileAiGeneratedContentB3eeebbf()),
              key: ValueKey('wb-ai-disclaimer'),
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
                fontSize: 12,
              ),
            ),
          ),
          const SizedBox(height: 16),
          for (final message in widget.task.messages) ...[
            if (message.fromUser)
              Align(
                alignment: Alignment.centerRight,
                child: Container(
                  constraints: BoxConstraints(
                    maxWidth: MediaQuery.sizeOf(context).width * 0.78,
                  ),
                  margin: const EdgeInsets.only(bottom: 18, left: 36),
                  padding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 12,
                  ),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.surfaceContainerHigh,
                    borderRadius: BorderRadius.circular(18),
                  ),
                  child: Text(
                    message.text,
                    style: wbBody.copyWith(fontSize: 15),
                  ),
                ),
              )
            else ...[
              _StatusRow(status: widget.task.status),
              const SizedBox(height: 8),
              Divider(
                height: 1,
                color: Theme.of(context).colorScheme.outlineVariant,
              ),
              const SizedBox(height: 16),
              if (message.text.isNotEmpty) Text(message.text, style: wbBody),
              for (final block in message.blocks) ...[
                const SizedBox(height: 14),
                Text(
                  block.heading ? '# ${block.text}' : block.text,
                  style: block.heading
                      ? wbBody.copyWith(
                          fontSize: 22,
                          fontWeight: FontWeight.w700,
                          height: 1.3,
                        )
                      : block.file
                      ? wbBody.copyWith(
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                          fontSize: 15,
                        )
                      : wbBody,
                ),
              ],
              const SizedBox(height: 12),
              Divider(
                height: 1,
                color: Theme.of(context).colorScheme.outlineVariant,
              ),
              const SizedBox(height: 8),
            ],
          ],
        ],
      ),
      Positioned(
        right: 8,
        bottom: 8,
        child: _CircleButton(
          key: const ValueKey('wb-scroll-end'),
          onTap: () {
            if (!_scroll.hasClients) return;
            _scroll.animateTo(
              _scroll.position.maxScrollExtent,
              duration: const Duration(milliseconds: 240),
              curve: Curves.easeOut,
            );
          },
          child: WbIcon(WbGlyph.chevronDown, size: 18),
        ),
      ),
    ],
  );
}

final class _StatusRow extends StatelessWidget {
  const _StatusRow({required this.status});

  final WbTaskStatus status;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: 8),
    child: Row(
      children: [
        WbIcon(
          status == WbTaskStatus.completed ? WbGlyph.check : WbGlyph.alarm,
          size: 16,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
        const SizedBox(width: 8),
        Text(
          status == WbTaskStatus.completed
              ? (stringsOf(context).mobileCompleted267fbd00())
              : (stringsOf(context).mobileInProgress9591c222()),
          key: const ValueKey('wb-task-status'),
          style: TextStyle(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
            fontSize: 14,
          ),
        ),
        const SizedBox(width: 4),
        WbIcon(
          WbGlyph.chevron,
          size: 12,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
      ],
    ),
  );
}

final class _Composer extends StatelessWidget {
  const _Composer({
    this.hasAttachments = false,
    this.onPasteResource,
    required this.controller,
    required this.focus,
    required this.voiceMode,
    required this.holding,
    required this.onSubmit,
    required this.onInputModeToggle,
    required this.onVoiceDown,
    required this.onVoiceUp,
    required this.onVoiceMove,
    required this.onPermission,
    required this.onModel,
    required this.onAttachment,
    required this.onCancel,
    required this.permissionLabel,
    required this.modelLabel,
    required this.running,
    required this.enabled,
  });

  final bool hasAttachments;
  final VoidCallback? onPasteResource;
  final TextEditingController controller;
  final FocusNode focus;
  final bool voiceMode;
  final bool holding;
  final VoidCallback onSubmit;
  final VoidCallback onInputModeToggle;
  final VoidCallback onVoiceDown;
  final VoidCallback onVoiceUp;
  final ValueChanged<LongPressMoveUpdateDetails> onVoiceMove;
  final VoidCallback onPermission;
  final VoidCallback onModel;
  final VoidCallback onAttachment;
  final Future<void> Function() onCancel;
  final String permissionLabel;
  final String modelLabel;
  final bool running;
  final bool enabled;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.fromLTRB(16, 6, 16, 8),
    child: DecoratedBox(
      decoration: BoxDecoration(
        color: Theme.of(context).scaffoldBackgroundColor,
        borderRadius: BorderRadius.circular(27),
        border: Border.all(
          color: holding ? Colors.white : Theme.of(context).colorScheme.outline,
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.fromLTRB(10, 4, 8, 5),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              children: [
                IconButton(
                  key: const ValueKey('wb-input-mode-toggle'),
                  tooltip: voiceMode
                      ? (stringsOf(context).mobileSwitchToKeyboard2f4cc281())
                      : (stringsOf(context).mobileSwitchToDictation740f6546()),
                  onPressed: enabled && !holding ? onInputModeToggle : null,
                  visualDensity: VisualDensity.compact,
                  padding: const EdgeInsets.all(6),
                  constraints: const BoxConstraints(
                    minWidth: 36,
                    minHeight: 36,
                  ),
                  icon: Icon(
                    voiceMode
                        ? Icons.keyboard_alt_outlined
                        : Icons.mic_none_outlined,
                    size: 22,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
                const SizedBox(width: 4),
                Expanded(
                  child: voiceMode
                      ? GestureDetector(
                          key: const ValueKey('wb-voice-hold'),
                          behavior: HitTestBehavior.opaque,
                          onLongPressStart: enabled
                              ? (_) => onVoiceDown()
                              : null,
                          onLongPressMoveUpdate: enabled ? onVoiceMove : null,
                          onLongPressEnd: enabled ? (_) => onVoiceUp() : null,
                          onLongPressCancel: enabled ? onVoiceUp : null,
                          child: AnimatedContainer(
                            duration: const Duration(milliseconds: 160),
                            height: 42,
                            alignment: Alignment.center,
                            decoration: BoxDecoration(
                              color: holding
                                  ? Theme.of(
                                      context,
                                    ).colorScheme.primaryContainer
                                  : Theme.of(
                                      context,
                                    ).colorScheme.surfaceContainerHigh,
                              borderRadius: BorderRadius.circular(21),
                            ),
                            child: Text(
                              holding
                                  ? (stringsOf(
                                      context,
                                    ).mobileReleaseToFinishSlideUpToCancel5755f1f0())
                                  : (stringsOf(
                                      context,
                                    ).mobileHoldToSpeakF4dcaf7f()),
                              style: TextStyle(
                                color: holding
                                    ? Theme.of(
                                        context,
                                      ).colorScheme.onPrimaryContainer
                                    : Theme.of(context).colorScheme.onSurface,
                                fontSize: 15,
                                fontWeight: FontWeight.w500,
                              ),
                            ),
                          ),
                        )
                      : TextField(
                          key: const ValueKey('wb-composer'),
                          controller: controller,
                          focusNode: focus,
                          enabled: enabled,
                          contextMenuBuilder: (context, editable) =>
                              AdaptiveTextSelectionToolbar.buttonItems(
                                anchors: editable.contextMenuAnchors,
                                buttonItems: [
                                  ...editable.contextMenuButtonItems,
                                  if (onPasteResource != null)
                                    ContextMenuButtonItem(
                                      label: stringsOf(
                                        context,
                                      ).resourcecenterPaste(),
                                      onPressed: () {
                                        editable.hideToolbar();
                                        onPasteResource!();
                                      },
                                    ),
                                ],
                              ),
                          minLines: 1,
                          maxLines: 4,
                          style: TextStyle(
                            color: Theme.of(context).colorScheme.onSurface,
                            fontSize: 16,
                          ),
                          cursorColor: Theme.of(context).colorScheme.primary,
                          textInputAction: TextInputAction.send,
                          decoration: InputDecoration(
                            isDense: true,
                            border: InputBorder.none,
                            hintText: (stringsOf(
                              context,
                            ).mobileMessage97302219()),
                            hintStyle: TextStyle(
                              color: Color(0xFF8A8A8A),
                              fontSize: 16,
                            ),
                          ),
                          onSubmitted: (_) => onSubmit(),
                        ),
                ),
                IconButton(
                  key: const ValueKey('wb-plus'),
                  tooltip: (stringsOf(context).mobileAddAFileOrImageBbfddc0f()),
                  onPressed: enabled ? onAttachment : null,
                  icon: WbIcon(
                    WbGlyph.plus,
                    size: 22,
                    color: Theme.of(context).colorScheme.onSurface,
                  ),
                ),
              ],
            ),
            Row(
              children: [
                Flexible(
                  child: _ComposerChip(
                    key: const ValueKey('wb-permission'),
                    icon: Icons.admin_panel_settings_outlined,
                    label: permissionLabel,
                    onTap: enabled ? onPermission : null,
                  ),
                ),
                const SizedBox(width: 4),
                Expanded(
                  child: _ComposerChip(
                    key: const ValueKey('wb-model'),
                    icon: Icons.auto_awesome_outlined,
                    label: modelLabel,
                    onTap: enabled ? onModel : null,
                  ),
                ),
                const SizedBox(width: 6),
                if (running)
                  IconButton.filledTonal(
                    key: const ValueKey('wb-stop'),
                    tooltip: (stringsOf(
                      context,
                    ).mobileStopGenerating21733a02()),
                    onPressed: onCancel,
                    icon: Icon(Icons.stop_rounded, size: 20),
                  )
                else
                  ValueListenableBuilder<TextEditingValue>(
                    valueListenable: controller,
                    builder: (context, value, _) => IconButton.filled(
                      key: const ValueKey('wb-send'),
                      tooltip: (stringsOf(context).mobileSend8457aed9()),
                      onPressed:
                          enabled &&
                              (value.text.trim().isNotEmpty || hasAttachments)
                          ? onSubmit
                          : null,
                      style: IconButton.styleFrom(
                        backgroundColor: Theme.of(context).colorScheme.primary,
                        disabledBackgroundColor: Theme.of(
                          context,
                        ).colorScheme.surfaceContainerHigh,
                        foregroundColor: Theme.of(
                          context,
                        ).colorScheme.onPrimary,
                        disabledForegroundColor: Theme.of(
                          context,
                        ).colorScheme.onSurfaceVariant,
                      ),
                      icon: Icon(Icons.arrow_upward_rounded, size: 20),
                    ),
                  ),
              ],
            ),
          ],
        ),
      ),
    ),
  );
}

final class _ComposerChip extends StatelessWidget {
  const _ComposerChip({
    super.key,
    required this.icon,
    required this.label,
    required this.onTap,
  });

  final IconData icon;
  final String label;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => InkWell(
    borderRadius: BorderRadius.circular(18),
    onTap: onTap,
    child: Padding(
      padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 7),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            icon,
            size: 15,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
          const SizedBox(width: 4),
          Flexible(
            child: Text(
              label,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
                fontSize: 12,
              ),
            ),
          ),
          const SizedBox(width: 2),
          Icon(
            Icons.expand_more,
            size: 14,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ],
      ),
    ),
  );
}

String _permissionLabel(BuildContext context, String? value) => switch (value) {
  'read-only' => (stringsOf(context).mobileReadOnlyCb269872()),
  'workspace-write' => (stringsOf(
    context,
  ).mobileWorkspaceWriteAccessFae86ef7()),
  'danger-full-access' => (stringsOf(context).mobileFullAccess6d8c6618()),
  'auto' => (stringsOf(context).mobileAutomaticApproval02a34125()),
  final value? when value.isNotEmpty => value,
  _ => (stringsOf(context).mobileWorkspacePermissionsB43b081a()),
};

String _workBuddyModelLabel(
  BuildContext context,
  String? model,
  String? effort,
) {
  final value = model?.trim();
  if (value == null || value.isEmpty) {
    return (stringsOf(context).mobileChooseAModelD992696e());
  }
  return effort == null || effort.isEmpty ? value : '$value · $effort';
}

final class _VoiceHold extends StatefulWidget {
  const _VoiceHold({
    required this.cancelling,
    required this.released,
    required this.composer,
  });

  final bool cancelling;
  final bool released;
  final TextEditingController composer;

  @override
  State<_VoiceHold> createState() => _VoiceHoldState();
}

final class _VoiceHoldState extends State<_VoiceHold>
    with SingleTickerProviderStateMixin {
  late final AnimationController _animation = AnimationController(
    vsync: this,
    duration: const Duration(milliseconds: 900),
  )..repeat();

  @override
  void dispose() {
    _animation.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final height = (MediaQuery.sizeOf(context).height * 0.38).clamp(
      250.0,
      340.0,
    );
    final accent = widget.cancelling
        ? const Color(0xFFFF8D85)
        : const Color(0xFF7FF5DF);
    return AnimatedContainer(
      key: const ValueKey('wb-voice-overlay'),
      duration: const Duration(milliseconds: 180),
      height: height,
      decoration: BoxDecoration(
        borderRadius: const BorderRadius.vertical(top: Radius.circular(34)),
        gradient: LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: widget.cancelling
              ? const [Color(0x001F2026), Color(0xB35C3438), Color(0xFF7B3F43)]
              : const [Color(0x0018CDB0), Color(0xC82CD4B8), Color(0xFF35CDB5)],
          stops: const [0, 0.26, 1],
        ),
        boxShadow: [
          BoxShadow(
            color: accent.withValues(alpha: 0.22),
            blurRadius: 42,
            spreadRadius: 12,
            offset: const Offset(0, -16),
          ),
        ],
      ),
      child: Semantics(
        liveRegion: true,
        label: widget.cancelling
            ? (stringsOf(context).mobileReleaseToCancelDictation75875019())
            : widget.released
            ? (stringsOf(context).mobileFinishingSpeechRecognition8a8c1c0b())
            : (stringsOf(
                context,
              ).mobileListeningReleaseToFinishOrSlideUpTo8cca1710()),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(28, 62, 28, 34),
          child: Column(
            children: [
              AnimatedSwitcher(
                duration: const Duration(milliseconds: 140),
                child: Text(
                  widget.cancelling
                      ? (stringsOf(context).mobileReleaseToCancelC48c1de4())
                      : widget.released
                      ? (stringsOf(
                          context,
                        ).mobileFinishingRecognition0072ccbb())
                      : (stringsOf(
                          context,
                        ).mobileReleaseToFinishSlideUpToCancel354d09d6()),
                  key: ValueKey((widget.cancelling, widget.released)),
                  textAlign: TextAlign.center,
                  style: TextStyle(
                    color: widget.cancelling
                        ? const Color(0xFFFFE7E4)
                        : const Color(0xFF062E28),
                    fontSize: 18,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              ValueListenableBuilder<TextEditingValue>(
                valueListenable: widget.composer,
                builder: (context, draft, _) => Padding(
                  padding: const EdgeInsets.only(top: 20),
                  child: Text(
                    draft.text.trim(),
                    key: const ValueKey('wb-voice-transcript'),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      color: widget.cancelling
                          ? const Color(0xFFFFE7E4)
                          : const Color(0xFF06352E),
                      fontSize: 16,
                    ),
                  ),
                ),
              ),
              Expanded(
                child: Center(
                  child: AnimatedBuilder(
                    animation: _animation,
                    builder: (context, _) => SizedBox(
                      width: double.infinity,
                      height: 72,
                      child: CustomPaint(
                        key: const ValueKey('wb-voice-waveform'),
                        painter: _VoiceWaveformPainter(
                          phase: _animation.value,
                          color: widget.cancelling
                              ? const Color(0xFFFFD8D4)
                              : Colors.white,
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _VoiceWaveformPainter extends CustomPainter {
  const _VoiceWaveformPainter({required this.phase, required this.color});

  final double phase;
  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    const bars = 27;
    final spacing = size.width / bars;
    final paint = Paint()
      ..color = color
      ..strokeWidth = math.min(5, spacing * 0.42)
      ..strokeCap = StrokeCap.round;
    for (var index = 0; index < bars; index += 1) {
      final x = spacing * (index + 0.5);
      final wave = (math.sin(index * 0.77 + phase * math.pi * 2) + 1) / 2;
      final envelope = 0.62 + 0.38 * math.sin(index / (bars - 1) * math.pi);
      final barHeight = 12 + 46 * wave * envelope;
      canvas.drawLine(
        Offset(x, (size.height - barHeight) / 2),
        Offset(x, (size.height + barHeight) / 2),
        paint,
      );
    }
  }

  @override
  bool shouldRepaint(covariant _VoiceWaveformPainter oldDelegate) =>
      oldDelegate.phase != phase || oldDelegate.color != color;
}

final class _TabBar extends StatelessWidget {
  const _TabBar({required this.controller});

  final WorkBuddyController controller;

  @override
  Widget build(BuildContext context) {
    final items = [
      (WbTab.home, WbGlyph.home, stringsOf(context).resourcecenterHome()),
      (WbTab.tasks, WbGlyph.bubble, (stringsOf(context).mobileTasks49bc7474())),
      (WbTab.apps, WbGlyph.apps, stringsOf(context).mobileApps()),
      (
        WbTab.mine,
        WbGlyph.person,
        (stringsOf(context).mobileAccountA10e8da3()),
      ),
    ];
    return Padding(
      padding: const EdgeInsets.only(bottom: 4, top: 2),
      child: Row(
        children: [
          for (final item in items)
            Expanded(
              child: _TabButton(
                label: item.$3,
                glyph: item.$2,
                selected: controller.tab == item.$1,
                onTap: () => controller.selectTab(item.$1),
              ),
            ),
        ],
      ),
    );
  }
}

final class _TabButton extends StatelessWidget {
  const _TabButton({
    required this.label,
    required this.glyph,
    required this.selected,
    required this.onTap,
  });

  final String label;
  final WbGlyph glyph;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final color = selected
        ? Theme.of(context).colorScheme.primary
        : Theme.of(context).colorScheme.onSurfaceVariant;
    return GestureDetector(
      key: ValueKey('wb-tab-$label'),
      onTap: onTap,
      behavior: HitTestBehavior.opaque,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 6),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            WbIcon(glyph, size: 22, color: color),
            const SizedBox(height: 4),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 3),
              child: FittedBox(
                fit: BoxFit.scaleDown,
                child: Text(
                  label,
                  maxLines: 1,
                  style: TextStyle(color: color, fontSize: 11),
                ),
              ),
            ),
            const SizedBox(height: 4),
            AnimatedContainer(
              duration: const Duration(milliseconds: 160),
              width: selected ? 22 : 0,
              height: 2.5,
              decoration: BoxDecoration(
                color: Theme.of(context).colorScheme.primary,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

final class _CircleButton extends StatelessWidget {
  const _CircleButton({super.key, required this.onTap, required this.child});

  final VoidCallback onTap;
  final Widget child;

  @override
  Widget build(BuildContext context) => Material(
    color: Theme.of(context).colorScheme.surfaceContainer,
    shape: const CircleBorder(),
    child: InkWell(
      customBorder: const CircleBorder(),
      onTap: onTap,
      child: SizedBox(width: 40, height: 40, child: Center(child: child)),
    ),
  );
}

final class _DrawerPanel extends StatelessWidget {
  const _DrawerPanel({
    required this.controller,
    required this.onNewTask,
    required this.onDevice,
    required this.onOpenSession,
  });

  final WorkBuddyController controller;
  final VoidCallback onNewTask;
  final VoidCallback onDevice;
  final ValueChanged<WbTask> onOpenSession;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) {
      final spaces = controller.workspacesFor(controller.selectedDeviceId);
      return Material(
        color: Theme.of(context).colorScheme.surfaceContainer,
        child: SafeArea(
          child: Padding(
            padding: const EdgeInsets.fromLTRB(18, 8, 18, 8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                InkWell(
                  key: const ValueKey('wb-drawer-device'),
                  onTap: onDevice,
                  child: Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Row(
                      children: [
                        WbIcon(
                          WbGlyph.desktop,
                          size: 22,
                          color: Color(0xFF7DDEA8),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            controller.selectedDevice.kind == WbDeviceKind.cloud
                                ? (stringsOf(context).mobileCloud118c7f15())
                                : controller.selectedDevice.name,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              color: Theme.of(context).colorScheme.onSurface,
                              fontSize: 18,
                              fontWeight: FontWeight.w700,
                            ),
                          ),
                        ),
                        WbIcon(
                          WbGlyph.chevronDown,
                          size: 16,
                          color: Theme.of(context).colorScheme.onSurfaceVariant,
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 10),
                OutlinedButton(
                  key: const ValueKey('wb-new-task'),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: Theme.of(context).colorScheme.onSurface,
                    side: BorderSide(
                      color: Theme.of(context).colorScheme.outlineVariant,
                    ),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(28),
                    ),
                    padding: const EdgeInsets.symmetric(vertical: 14),
                  ),
                  onPressed: onNewTask,
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      WbIcon(WbGlyph.bubblePlus, size: 18),
                      SizedBox(width: 8),
                      Text(
                        (stringsOf(context).mobileNewTaskE8e54ef0()),
                        style: TextStyle(fontSize: 16),
                      ),
                    ],
                  ),
                ),
                const SizedBox(height: 18),
                Expanded(
                  child: ListView(
                    padding: EdgeInsets.zero,
                    children: [
                      _SectionLabel(
                        label: (stringsOf(context).mobileRecentTasksCdbbd048()),
                        expanded: controller.tasksExpanded,
                        onTap: controller.toggleTasks,
                      ),
                      if (controller.tasksExpanded)
                        for (final task in controller.recentTasks)
                          _DrawerRow(
                            key: ValueKey('wb-recent-task-${task.id}'),
                            title: task.title,
                            indent: 0,
                            onTap: () => task.id.startsWith('dsh.session.')
                                ? onOpenSession(task)
                                : controller.openTaskById(task.id),
                          ),
                      const SizedBox(height: 8),
                      _SectionLabel(
                        label: (stringsOf(
                          context,
                        ).mobileWorkspaces3d22293a(value1: spaces.length)),
                        expanded: controller.spacesExpanded,
                        onTap: controller.toggleSpaces,
                      ),
                      if (controller.spacesExpanded)
                        for (final workspace in spaces) ...[
                          _WorkspaceRow(
                            workspace: workspace,
                            expanded: controller.expandedWorkspaces.contains(
                              workspace.id,
                            ),
                            onToggle: () =>
                                controller.toggleWorkspace(workspace.id),
                            onPlus: () {
                              controller.selectWorkspace(workspace.id);
                              onNewTask();
                            },
                          ),
                          if (controller.expandedWorkspaces.contains(
                            workspace.id,
                          ))
                            for (final task in controller.tasksIn(workspace.id))
                              _DrawerRow(
                                key: ValueKey('wb-space-task-${task.id}'),
                                title: task.title,
                                indent: 28,
                                onTap: () => task.id.startsWith('dsh.session.')
                                    ? onOpenSession(task)
                                    : controller.openTaskById(task.id),
                              ),
                        ],
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      );
    },
  );
}

final class _SectionLabel extends StatelessWidget {
  const _SectionLabel({
    required this.label,
    required this.expanded,
    required this.onTap,
  });

  final String label;
  final bool expanded;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) => InkWell(
    onTap: onTap,
    child: Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Text(
            label,
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              fontSize: 14,
            ),
          ),
          const SizedBox(width: 4),
          WbIcon(
            expanded ? WbGlyph.chevronDown : WbGlyph.chevron,
            size: 12,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ],
      ),
    ),
  );
}

final class _DrawerRow extends StatelessWidget {
  const _DrawerRow({
    super.key,
    required this.title,
    required this.indent,
    this.onTap,
  });

  final String title;
  final double indent;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => InkWell(
    onTap: onTap,
    child: Padding(
      padding: EdgeInsets.fromLTRB(indent, 11, 0, 11),
      child: Row(
        children: [
          Expanded(
            child: Text(
              title,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurface,
                fontSize: 16,
              ),
            ),
          ),
        ],
      ),
    ),
  );
}

final class _WorkspaceRow extends StatelessWidget {
  const _WorkspaceRow({
    required this.workspace,
    required this.expanded,
    required this.onToggle,
    required this.onPlus,
  });

  final WbWorkspace workspace;
  final bool expanded;
  final VoidCallback onToggle;
  final VoidCallback onPlus;

  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.symmetric(vertical: 4),
    child: Row(
      children: [
        WbIcon(
          WbGlyph.folder,
          size: 18,
          color: Theme.of(context).colorScheme.onSurfaceVariant,
        ),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            workspace.name,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurface,
              fontSize: 16,
            ),
          ),
        ),
        if (workspace.deviceId == kCloudDeviceId)
          Padding(
            padding: EdgeInsets.only(right: 4),
            child: Text(
              (stringsOf(context).mobileComingSoonDdd8280c()),
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurfaceVariant,
                fontSize: 12,
              ),
            ),
          ),
        IconButton(
          onPressed: onToggle,
          icon: WbIcon(
            expanded ? WbGlyph.chevronDown : WbGlyph.chevron,
            size: 14,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        IconButton(
          key: ValueKey('wb-space-plus-${workspace.id}'),
          onPressed: onPlus,
          icon: WbIcon(
            WbGlyph.plus,
            size: 16,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
      ],
    ),
  );
}

final class _RunSettingsSheet extends StatelessWidget {
  const _RunSettingsSheet({
    required this.deviceName,
    required this.workspaceName,
    required this.onDevice,
    required this.onWorkspace,
  });

  final String deviceName;
  final String workspaceName;
  final VoidCallback onDevice;
  final VoidCallback onWorkspace;

  @override
  Widget build(BuildContext context) => SafeArea(
    top: false,
    child: Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 18),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  (stringsOf(context).mobileTaskSettings2fc94999()),
                  key: ValueKey('wb-run-settings'),
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.onSurface,
                    fontSize: 17,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              IconButton(
                key: const ValueKey('wb-run-settings-close'),
                onPressed: () => Navigator.pop(context),
                icon: WbIcon(
                  WbGlyph.close,
                  size: 18,
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          _SettingCard(
            key: const ValueKey('wb-pick-device'),
            label: (stringsOf(context).mobileDevice37d06f01()),
            value: deviceName,
            showDesktop: true,
            onTap: onDevice,
          ),
          const SizedBox(height: 10),
          _SettingCard(
            key: const ValueKey('wb-pick-workspace'),
            label: (stringsOf(context).mobileWorkspaceCbc7c9de()),
            value: workspaceName,
            onTap: onWorkspace,
          ),
          const SizedBox(height: 8),
        ],
      ),
    ),
  );
}

final class _SettingCard extends StatelessWidget {
  const _SettingCard({
    super.key,
    required this.label,
    required this.value,
    required this.onTap,
    this.showDesktop = false,
  });

  final String label;
  final String value;
  final VoidCallback onTap;
  final bool showDesktop;

  @override
  Widget build(BuildContext context) => Material(
    color: const Color(0xFF2A2A2A),
    borderRadius: BorderRadius.circular(16),
    child: InkWell(
      borderRadius: BorderRadius.circular(16),
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 18),
        child: Row(
          children: [
            Text(
              label,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurface,
                fontSize: 16,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  if (showDesktop) ...[
                    WbIcon(
                      WbGlyph.desktop,
                      size: 16,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                    const SizedBox(width: 6),
                  ],
                  Flexible(
                    child: Text(
                      value,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      textAlign: TextAlign.right,
                      style: TextStyle(
                        color: Theme.of(context).colorScheme.onSurfaceVariant,
                        fontSize: 15,
                      ),
                    ),
                  ),
                  const SizedBox(width: 4),
                  WbIcon(
                    WbGlyph.chevron,
                    size: 14,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

final class _ChoiceSheet extends StatelessWidget {
  const _ChoiceSheet({required this.title, required this.children});

  final String title;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) => SafeArea(
    child: SingleChildScrollView(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(8, 12, 8, 8),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 4, 12, 8),
              child: Text(
                title,
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurface,
                  fontSize: 16,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
            ...children,
          ],
        ),
      ),
    ),
  );
}

final class _MascotFallbackPainter extends CustomPainter {
  const _MascotFallbackPainter();

  @override
  void paint(Canvas canvas, Size size) {
    final metal = Paint()..color = const Color(0xFFE8E8EA);
    final dark = Paint()..color = const Color(0xFF2A2C30);
    final eye = Paint()..color = const Color(0xFF3DFFC2);
    final line = Paint()
      ..color = const Color(0xFF9AA0A6)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;
    final w = size.width;
    final h = size.height;
    canvas.drawOval(
      Rect.fromCenter(
        center: Offset(w * 0.3, h * 0.22),
        width: w * 0.16,
        height: h * 0.22,
      ),
      metal,
    );
    canvas.drawOval(
      Rect.fromCenter(
        center: Offset(w * 0.66, h * 0.22),
        width: w * 0.16,
        height: h * 0.22,
      ),
      metal,
    );
    canvas.drawCircle(Offset(w * 0.48, h * 0.4), w * 0.24, metal);
    canvas.drawCircle(Offset(w * 0.22, h * 0.42), w * 0.1, metal);
    canvas.drawCircle(Offset(w * 0.74, h * 0.42), w * 0.1, metal);
    canvas.drawCircle(Offset(w * 0.22, h * 0.42), w * 0.045, dark);
    canvas.drawCircle(Offset(w * 0.74, h * 0.42), w * 0.045, dark);
    canvas.drawOval(
      Rect.fromCenter(
        center: Offset(w * 0.48, h * 0.42),
        width: w * 0.3,
        height: h * 0.22,
      ),
      dark,
    );
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromCenter(
          center: Offset(w * 0.41, h * 0.42),
          width: 7,
          height: 18,
        ),
        const Radius.circular(3),
      ),
      eye,
    );
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromCenter(
          center: Offset(w * 0.56, h * 0.42),
          width: 7,
          height: 18,
        ),
        const Radius.circular(3),
      ),
      eye,
    );
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromLTWH(w * 0.34, h * 0.58, w * 0.28, h * 0.22),
        const Radius.circular(10),
      ),
      metal,
    );
    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromLTWH(w * 0.3, h * 0.66, w * 0.5, h * 0.28),
        const Radius.circular(4),
      ),
      metal,
    );
    canvas.drawCircle(Offset(w * 0.55, h * 0.78), 5, dark);
    canvas.drawLine(Offset(w * 0.34, h * 0.9), Offset(w * 0.76, h * 0.9), line);
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}

final class _MobileLoginRoute extends StatefulWidget {
  const _MobileLoginRoute({required this.authentication, this.cloudLabel});

  final OpenMuseAuthenticationController authentication;
  final String? cloudLabel;

  @override
  State<_MobileLoginRoute> createState() => _MobileLoginRouteState();
}

final class _MobileLoginRouteState extends State<_MobileLoginRoute> {
  bool _closing = false;

  @override
  void initState() {
    super.initState();
    widget.authentication.addListener(_onAuthenticationChanged);
    _onAuthenticationChanged();
  }

  @override
  void dispose() {
    widget.authentication.removeListener(_onAuthenticationChanged);
    super.dispose();
  }

  void _onAuthenticationChanged() {
    if (_closing || !widget.authentication.snapshot.isAuthenticated) return;
    _closing = true;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) Navigator.of(context).pop();
    });
  }

  @override
  Widget build(BuildContext context) => OpenMuseLoginScreen(
    authentication: widget.authentication,
    cloudLabel: widget.cloudLabel,
  );
}

Future<void> openDesktopSetupGuide(BuildContext context) async {
  final guide = await rootBundle.loadString(
    'packages/openmuse_resource_center_host/assets/welcome/OpenMuse-User-Guide.md',
  );
  final start = guide.indexOf('## 8. How to connect a Desktop');
  final end = guide.indexOf('\n## 9.', start < 0 ? 0 : start);
  final section = start >= 0 && end > start
      ? guide.substring(start, end)
      : guide;
  if (!context.mounted) return;
  await Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => Scaffold(
        appBar: AppBar(
          title: Text(stringsOf(context).commonDesktopSetupGuide()),
        ),
        body: SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: MarkdownBody(
            data: section,
            selectable: true,
            styleSheet: MarkdownStyleSheet(
              p: const TextStyle(fontSize: 16, height: 1.6),
            ),
            onTapLink: (_, href, _) async {
              final uri = Uri.tryParse(href ?? '');
              if (uri == null || !{'https', 'mailto'}.contains(uri.scheme)) {
                return;
              }
              await launchUrl(uri, mode: LaunchMode.externalApplication);
            },
          ),
        ),
      ),
    ),
  );
}

final class _DesktopBindSheet extends StatelessWidget {
  const _DesktopBindSheet({
    required this.paired,
    required this.selectedDeviceId,
    required this.onConnect,
  });

  final PairedDesktopMobileController? paired;
  final String selectedDeviceId;
  final Future<void> Function(AccountDevice device) onConnect;

  @override
  Widget build(BuildContext context) {
    final source = paired;
    if (source == null) return _body(context, const []);
    return ListenableBuilder(
      listenable: source,
      builder: (context, _) => _body(
        context,
        source.devices
            .where((device) => device.kind == AccountDeviceKind.desktop)
            .toList(growable: false),
      ),
    );
  }

  Widget _body(BuildContext context, List<AccountDevice> desktops) {
    return SafeArea(
      child: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 18, 20, 12),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Row(
                children: [
                  Text(
                    (stringsOf(context).mobileConnectDesktopA9885c8c()),
                    key: ValueKey('wb-bind-desktop'),
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.onSurface,
                      fontSize: 18,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  IconButton(
                    key: const ValueKey('wb-refresh-desktops'),
                    tooltip: (stringsOf(context).mobileRefresh20ada9a3()),
                    visualDensity: VisualDensity.compact,
                    padding: const EdgeInsets.all(6),
                    constraints: const BoxConstraints(
                      minWidth: 32,
                      minHeight: 32,
                    ),
                    onPressed: paired?.directory?.snapshot.loading == true
                        ? null
                        : () => unawaited(
                            paired?.directory?.refresh() ??
                                Future<void>.value(),
                          ),
                    icon: Icon(
                      Icons.refresh,
                      size: 20,
                      color: Theme.of(context).colorScheme.onSurface,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 8),
              Text(
                (stringsOf(
                  context,
                ).mobileSignInToTheSameAccountOnYourComput3e0a75c4()),
                style: wbSub,
              ),
              TextButton.icon(
                key: const ValueKey('desktop-setup-guide'),
                icon: const Icon(Icons.help_outline),
                label: Text(stringsOf(context).commonDesktopSetupGuide()),
                onPressed: () => openDesktopSetupGuide(context),
              ),
              const SizedBox(height: 12),
              if (desktops.isEmpty &&
                  paired?.directory?.snapshot.loading != true)
                Text(
                  (stringsOf(
                    context,
                  ).mobileNoDesktopIsAvailableOnThisAccountOp8ae08e25()),
                  style: wbSub,
                )
              else
                for (final device in desktops)
                  _DeviceStatusRow(
                    key: ValueKey('wb-desktop-${device.deviceRef}'),
                    id: 'paired.${device.deviceRef}',
                    name: device.displayName,
                    online: device.online,
                    current: selectedDeviceId == 'paired.${device.deviceRef}',
                    connected:
                        paired?.snapshot.connection?.deviceRef ==
                            device.deviceRef &&
                        (paired?.snapshot.connection?.expiresAtMs ?? 0) >
                            DateTime.now()
                                .add(const Duration(minutes: 1))
                                .millisecondsSinceEpoch,
                    connecting:
                        paired?.snapshot.connecting == true &&
                        selectedDeviceId == 'paired.${device.deviceRef}',
                    refreshing: paired?.directory?.snapshot.loading == true,
                    onBind: device.online && device.supportsPairedDesktop
                        ? () => onConnect(device)
                        : null,
                  ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _DeviceStatusRow extends StatelessWidget {
  const _DeviceStatusRow({
    super.key,
    required this.id,
    required this.name,
    required this.online,
    required this.current,
    required this.connected,
    required this.onBind,
    this.connecting = false,
    this.refreshing = false,
  });

  final String id;
  final String name;
  final bool online;
  final bool current;
  final bool connected;
  final bool connecting;
  final bool refreshing;
  final VoidCallback? onBind;

  static const onlineDot = Color(0xFF34C759);
  static const offlineDot = Color(0xFF8E8E93);

  @override
  Widget build(BuildContext context) {
    final trailing = refreshing
        ? SizedBox(
            width: 16,
            height: 16,
            child: CircularProgressIndicator(
              strokeWidth: 2,
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          )
        : !online
        ? Text((stringsOf(context).mobileOffline054e29cd()), style: wbSub)
        : connected
        ? Text(
            (stringsOf(context).mobileConnectedAfe95341()),
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              fontSize: 14,
            ),
          )
        : connecting
        ? Text(
            (stringsOf(context).mobileConnecting3ac1dd38()),
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              fontSize: 14,
            ),
          )
        : TextButton(
            key: ValueKey('wb-device-bind-$id'),
            onPressed: onBind,
            child: Text(
              current
                  ? (stringsOf(context).mobileReconnectF80f36fd())
                  : (stringsOf(context).mobileConnect99f44b5c()),
            ),
          );
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        children: [
          Container(
            key: ValueKey('wb-device-dot-$id'),
            width: 8,
            height: 8,
            decoration: BoxDecoration(
              color: online ? onlineDot : offlineDot,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 10),
          Expanded(
            child: Text(
              name,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onSurface,
                fontSize: 16,
              ),
            ),
          ),
          trailing,
        ],
      ),
    );
  }
}
