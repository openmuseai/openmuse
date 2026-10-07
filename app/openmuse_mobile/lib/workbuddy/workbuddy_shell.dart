import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:muse_speech_contract/muse_speech_contract.dart';
import 'package:muse_speech_core/muse_speech_core.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_remote_workbench/openmuse_remote_workbench.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../native_dsh_page.dart';
import '../plugin_interaction_remote.dart';
import 'workbuddy_controller.dart';
import 'workbuddy_icons.dart';
import 'workbuddy_models.dart';
import 'workbuddy_theme.dart';

final class WorkBuddyShell extends StatefulWidget {
  const WorkBuddyShell({
    super.key,
    required this.controller,
    this.authentication,
    this.pairedDesktop,
    this.catalog,
    this.cloudLabel,
    this.onSignOut,
    this.speechRecognition,
    this.debugSpeechSource,
    this.remoteWorkbench,
  });

  final WorkBuddyController controller;
  final OpenMuseAuthenticationController? authentication;
  final PairedDesktopMobileController? pairedDesktop;
  final WorkspaceCatalogPort? catalog;
  final String? cloudLabel;
  final Future<void> Function()? onSignOut;
  final SpeechRecognitionPort? speechRecognition;

  /// Debug-only E2E seam supplied by the Mobile Host. It is never populated
  /// from a plugin wire request or other untrusted input.
  final SpeechAudioSource? debugSpeechSource;
  final RemoteWorkbenchController? remoteWorkbench;

  @override
  State<WorkBuddyShell> createState() => _WorkBuddyShellState();
}

final class _WorkBuddyShellState extends State<WorkBuddyShell>
    with SingleTickerProviderStateMixin {
  late final AnimationController _drawer;
  final _composer = TextEditingController();
  final _focus = FocusNode();
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

  WorkBuddyController get controller => widget.controller;

  @override
  void initState() {
    super.initState();
    _drawer = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 240),
      value: controller.drawerOpen ? 1 : 0,
    );
    controller.addListener(_syncDrawer);
    widget.authentication?.addListener(_syncAccount);
    widget.pairedDesktop?.addListener(_syncDevices);
    _syncAccount();
    _syncDevices();
    _sessionHandle.addListener(_syncSession);
    unawaited(_loadCloud());
    if (widget.debugSpeechSource != null) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) unawaited(_startVoice(source: widget.debugSpeechSource));
      });
    }
  }

  void _syncSession() {
    if (mounted) setState(() {});
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
      widget.pairedDesktop?.addListener(_syncDevices);
      _syncDevices();
    }
    if (oldWidget.catalog != widget.catalog) unawaited(_loadCloud());
  }

  void _syncDrawer() {
    if (!mounted) return;
    _rememberNativeSession();
    if (controller.drawerOpen) {
      _drawer.forward();
    } else {
      _drawer.reverse();
    }
  }

  void _syncAccount() {
    final identity = widget.authentication?.snapshot.identity;
    controller.setAccount(signedIn: identity != null, name: identity?.email);
    if (identity != null) unawaited(_loadCloud());
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
      controller.upsertDevice(
        WbDevice(
          id: 'paired.${connection.deviceRef}',
          name: connection.deviceName,
          kind: WbDeviceKind.local,
        ),
      );
    }
    _rememberNativeSession();
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
    } on Object {
      // Cloud catalog is optional. Local computer and its workspaces stay usable.
    }
  }

  @override
  void dispose() {
    controller.removeListener(_syncDrawer);
    widget.authentication?.removeListener(_syncAccount);
    widget.pairedDesktop?.removeListener(_syncDevices);
    _drawer.dispose();
    final speechSession = _speechSession;
    if (speechSession != null) {
      unawaited(_cancelSpeechOnDispose(speechSession));
    }
    unawaited(_speechEvents?.cancel());
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
          builder: (_) => OpenMuseLoginScreen(
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
    final picked = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: WbColors.sheet,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => _ChoiceSheet(
        title: '选择设备',
        children: [
          for (final device in controller.devices)
            ListTile(
              key: ValueKey('device-${device.id}'),
              leading: WbIcon(
                device.kind == WbDeviceKind.cloud
                    ? WbGlyph.project
                    : WbGlyph.desktop,
                size: 20,
                color: WbColors.textMuted,
              ),
              title: Text(
                device.kind == WbDeviceKind.local ? device.name : '云端',
                style: const TextStyle(color: WbColors.text),
              ),
              subtitle: Text(
                device.kind == WbDeviceKind.local
                    ? (device.online ? '本机电脑 · 在线' : '本机电脑 · 离线')
                    : '账号下的 Cloud Workspace',
                style: wbSub,
              ),
              trailing: device.id == controller.selectedDeviceId
                  ? const WbIcon(WbGlyph.check, size: 18, color: WbColors.text)
                  : null,
              onTap: () => Navigator.pop(context, device.id),
            ),
        ],
      ),
    );
    if (picked == null) return;
    if (picked == kCloudDeviceId &&
        widget.authentication?.snapshot.isAuthenticated != true) {
      await _openLogin();
      if (widget.authentication?.snapshot.isAuthenticated != true) return;
      await _loadCloud();
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
                  title: session.title ?? (session.blank ? '新对话' : '历史对话'),
                  workspaceId: 'dsh.workspace.${workspace.workspaceId}',
                  deviceId: deviceId,
                  status: session.running
                      ? WbTaskStatus.running
                      : WbTaskStatus.completed,
                  showInTaskList: true,
                  messages: const [],
                ),
        ],
      );
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
    final spaces = controller.workspacesFor(controller.selectedDeviceId);
    final picked = await showModalBottomSheet<String>(
      context: context,
      backgroundColor: WbColors.sheet,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => _ChoiceSheet(
        title: '选择工作空间',
        children: [
          if (spaces.isEmpty)
            const ListTile(
              title: Text(
                '当前设备还没有工作空间',
                style: TextStyle(color: WbColors.textDim),
              ),
            ),
          for (final workspace in spaces)
            ListTile(
              key: ValueKey('workspace-${workspace.id}'),
              title: Text(
                workspace.name,
                style: const TextStyle(color: WbColors.text),
              ),
              trailing: workspace.id == controller.selectedWorkspaceId
                  ? const WbIcon(WbGlyph.check, size: 18)
                  : null,
              onTap: () => Navigator.pop(context, workspace.id),
            ),
        ],
      ),
    );
    if (picked != null) controller.selectWorkspace(picked);
  }

  Future<void> _openRunSettings() => showModalBottomSheet<void>(
    context: context,
    backgroundColor: WbColors.sheet,
    barrierColor: const Color(0x66000000),
    isScrollControlled: true,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(22)),
    ),
    builder: (context) => ListenableBuilder(
      listenable: controller,
      builder: (context, _) => _RunSettingsSheet(
        deviceName: controller.selectedDevice.kind == WbDeviceKind.cloud
            ? '云端'
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
    final text = _composer.text.trim();
    if (text.isEmpty) return;
    final connection = widget.pairedDesktop?.snapshot.connection;
    if (connection != null) {
      final task = controller.openTask;
      if (task != null && task.id.startsWith('dsh.session.')) {
        if (!_sessionHandle.ready) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('正在连接 Desktop 会话，请稍候再发送。')),
          );
          return;
        }
        _composer.clear();
        _focus.unfocus();
        try {
          await _sessionHandle.send(text);
        } on Object catch (error) {
          if (mounted) {
            ScaffoldMessenger.of(
              context,
            ).showSnackBar(SnackBar(content: Text('发送失败：$error')));
          }
        }
        return;
      }
      final workspaceId = controller.selectedWorkspace.id;
      if (!workspaceId.startsWith('dsh.workspace.')) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('请先选择 Desktop 上的 Workspace。')),
        );
        return;
      }
      _composer.clear();
      _focus.unfocus();
      var sessionConnection = connection;
      final paired = widget.pairedDesktop;
      final devices = paired?.devices.where(
        (device) => device.deviceRef == connection.deviceRef,
      );
      if (paired != null && devices != null && devices.isNotEmpty) {
        final renewed = await paired.connectDevice(devices.first, force: true);
        if (!renewed || paired.snapshot.connection == null) {
          if (mounted) {
            _composer.text = text;
            ScaffoldMessenger.of(context).showSnackBar(const SnackBar(
              content: Text('Desktop 配对授权已失效，请重新连接后发送。'),
            ));
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
          _pendingPrompt = text;
        });
        controller.openTaskById('dsh.session.${created.sessionId}');
      } on Object catch (error) {
        if (mounted) {
          _composer.text = text;
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('创建 Desktop 会话失败：$error')));
        }
      } finally {
        client.close();
      }
      return;
    }
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('请先从设备列表连接一台在线 Desktop，再发送消息。')),
    );
  }

  Future<void> _startVoice({SpeechAudioSource? source}) async {
    final speech = widget.speechRecognition;
    if (speech == null || _speechOpening || _speechSession != null) return;

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
      _showSpeechFailure(error.safeMessage);
    } on Object {
      _restoreSpeechDraft();
      _showSpeechFailure('无法启动语音输入，请稍后重试。');
    } finally {
      _speechOpening = false;
    }
  }

  Future<void> _stopVoice() async {
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
      _showSpeechFailure('无法结束语音输入，请稍后重试。');
    }
  }

  Future<void> _cancelVoice() async {
    _cancelSpeechWhenOpened = true;
    final speech = widget.speechRecognition;
    final session = _speechSession;
    if (speech == null || session == null) return;
    try {
      await speech.cancel(session.ref);
      if (_speechSession == session) _restoreSpeechDraft();
    } on Object {
      _restoreSpeechDraft();
      _showSpeechFailure('无法取消语音输入，请稍后重试。');
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
          _showSpeechFailure('未识别到语音，请重试。');
          break;
        }
        _applySpeechSnapshot(_speechDraft?.applyFinal(result));
        if (widget.debugSpeechSource != null) {
          debugPrint('OPENMUSE_ASR_E2E_FINAL=$result');
        }
        _finishSpeech(revealText: true);
      case SpeechEventKind.error:
        _restoreSpeechDraft();
        _showSpeechFailure(event.safeMessage ?? '语音识别失败，请重试。');
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
    final options = _sessionHandle.options?.permissions ?? const [];
    if (options.isEmpty) {
      _showSessionControlUnavailable('请先打开一个 Desktop 会话，再修改 Workspace 权限。');
      return;
    }
    final current = _sessionHandle.snapshot?.permissionPreset;
    final picked = await showModalBottomSheet<DshNativePermissionOption>(
      context: context,
      backgroundColor: WbColors.sheet,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => _ChoiceSheet(
        title: 'Workspace 权限',
        children: [
          for (final option in options)
            ListTile(
              key: ValueKey('wb-permission-${option.value}'),
              leading: Icon(
                option.value == 'danger-full-access'
                    ? Icons.warning_amber_rounded
                    : Icons.admin_panel_settings_outlined,
                color: WbColors.textMuted,
              ),
              title: Text(
                _permissionLabel(option.value),
                style: const TextStyle(color: WbColors.text),
              ),
              subtitle: option.description == null
                  ? null
                  : Text(option.description!, style: wbSub),
              trailing: current == option.value
                  ? const WbIcon(WbGlyph.check, size: 18)
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
          backgroundColor: WbColors.sheet,
          title: Text(
            picked.value == 'auto' ? '启用自动审批？' : '启用完整访问？',
            style: const TextStyle(color: WbColors.text),
          ),
          content: Text(
            picked.description ?? '此权限会扩大 Desktop 上工具和文件操作的范围。',
            style: const TextStyle(color: WbColors.textMuted),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: const Text('取消'),
            ),
            FilledButton(
              onPressed: () => Navigator.pop(context, true),
              child: const Text('确认'),
            ),
          ],
        ),
      );
      if (confirmed != true) return;
    }
    try {
      await _sessionHandle.selectPermission(picked.value);
    } on Object catch (error) {
      _showSessionControlUnavailable('权限修改失败：$error');
    }
  }

  Future<void> _chooseModel() async {
    final options = _sessionHandle.options?.models ?? const [];
    if (options.isEmpty) {
      _showSessionControlUnavailable('当前 Desktop 没有返回可用模型目录。');
      return;
    }
    final snapshot = _sessionHandle.snapshot;
    final picked = await showModalBottomSheet<DshNativeModelOption>(
      context: context,
      backgroundColor: WbColors.sheet,
      isScrollControlled: true,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => _ChoiceSheet(
        title: '选择模型',
        children: [
          for (final option in options)
            ListTile(
              key: ValueKey('wb-model-${option.provider}-${option.id}'),
              leading: const Icon(
                Icons.auto_awesome_outlined,
                color: WbColors.textMuted,
              ),
              title: Text(
                option.name,
                style: const TextStyle(color: WbColors.text),
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
                  ? const WbIcon(WbGlyph.check, size: 18)
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
        backgroundColor: WbColors.sheet,
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        builder: (context) => _ChoiceSheet(
          title: '思考强度',
          children: [
            for (final value in picked.efforts)
              ListTile(
                key: ValueKey('wb-effort-${value.id}'),
                title: Text(
                  value.name,
                  style: const TextStyle(color: WbColors.text),
                ),
                subtitle: value.description == null
                    ? null
                    : Text(value.description!, style: wbSub),
                trailing: snapshot?.reasoningEffort == value.id
                    ? const WbIcon(WbGlyph.check, size: 18)
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
      _showSessionControlUnavailable('模型切换失败：$error');
    }
  }

  Future<void> _showAttachmentActions() async {
    await showModalBottomSheet<void>(
      context: context,
      backgroundColor: WbColors.sheet,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => const _ChoiceSheet(
        title: '添加内容',
        children: [
          ListTile(
            leading: Icon(Icons.folder_outlined, color: WbColors.textMuted),
            title: Text(
              '从 Desktop Workspace 引用文件',
              style: TextStyle(color: WbColors.text),
            ),
            subtitle: Text('附件协议将在下一阶段开放', style: wbSub),
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

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.sizeOf(context).width;
    final drawerWidth = width * 0.84;
    return AnnotatedRegion<SystemUiOverlayStyle>(
      value: SystemUiOverlayStyle.light,
      child: Scaffold(
        backgroundColor: WbColors.canvas,
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
                      onAccount: _openAccount,
                      onOpenSession: _openDesktopSession,
                      onOpenWorkbench: _openRemoteWorkbench,
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
                if (widget.pairedDesktop?.snapshot.connection case final connection?)
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
            onInitialPromptConsumed: () {
              if (!mounted || _pendingSessionId != sessionId) return;
              setState(() {
                _pendingSessionId = null;
                _pendingPrompt = null;
              });
            },
          ),
      ],
    );
  }

  void _openRemoteWorkbench() {
    controller.closeDrawer();
    final remote = widget.remoteWorkbench;
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => remote == null
            ? Scaffold(
                appBar: AppBar(title: const Text('远程工作台')),
                body: const RemoteWorkbenchUnavailable(),
              )
            : RemoteWorkbenchPage(controller: remote),
      ),
    );
  }

  Future<void> _openAccount() async {
    final signedIn = widget.authentication?.snapshot.isAuthenticated ?? false;
    await showModalBottomSheet<void>(
      context: context,
      backgroundColor: WbColors.sheet,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(18)),
      ),
      builder: (context) => SafeArea(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 18, 20, 12),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                signedIn
                    ? (controller.accountName?.trim().isNotEmpty == true
                          ? controller.accountName!
                          : 'OpenMuse 用户')
                    : '未登录',
                style: const TextStyle(
                  color: WbColors.text,
                  fontSize: 18,
                  fontWeight: FontWeight.w600,
                ),
              ),
              const SizedBox(height: 6),
              Text(signedIn ? '已登录' : '登录后可使用云端工作空间', style: wbSub),
              const SizedBox(height: 16),
              if (!signedIn)
                FilledButton(
                  key: const ValueKey('wb-sign-in'),
                  onPressed: () {
                    debugPrint('OpenMuse mobile: login button pressed');
                    Navigator.pop(context);
                    WidgetsBinding.instance.addPostFrameCallback((_) {
                      if (!mounted) {
                        debugPrint(
                          'OpenMuse mobile: login route skipped, shell unmounted',
                        );
                        return;
                      }
                      unawaited(_openLogin());
                    });
                  },
                  child: const Text('登录'),
                )
              else
                OutlinedButton(
                  key: const ValueKey('wb-sign-out'),
                  onPressed: () {
                    Navigator.pop(context);
                    final signOut = widget.onSignOut;
                    if (signOut != null) unawaited(signOut());
                  },
                  child: const Text('退出登录'),
                ),
            ],
          ),
        ),
      ),
    );
  }
}

final class _MainColumn extends StatelessWidget {
  const _MainColumn({
    required this.controller,
    required this.composer,
    required this.focus,
    required this.voiceMode,
    required this.holdingVoice,
    required this.voiceReleased,
    required this.cancellingVoice,
    required this.onMenu,
    required this.onRunSettings,
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
  final TextEditingController composer;
  final FocusNode focus;
  final bool voiceMode;
  final bool holdingVoice;
  final bool voiceReleased;
  final bool cancellingVoice;
  final VoidCallback onMenu;
  final VoidCallback onRunSettings;
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
    listenable: controller,
    builder: (context, _) {
      final showComposer = controller.tab == WbTab.tasks;
      final snapshot = sessionHandle.snapshot;
      final conversation = conversationBuilder();
      final selectedTask = controller.openTask;
      final nativeTaskSelected =
          selectedTask != null && selectedTask.id.startsWith('dsh.session.');
      return ColoredBox(
        color: WbColors.canvas,
        child: SafeArea(
          child: Stack(
            children: [
              Column(
                children: [
                  _HomeHeader(
                    deviceName:
                        controller.selectedDevice.kind == WbDeviceKind.cloud
                        ? '云端'
                        : controller.selectedDevice.name,
                    workspaceName: controller.selectedWorkspace.name,
                    onMenu: onMenu,
                    onSubtitle: onRunSettings,
                  ),
                  Expanded(
                    child: _TabBody(
                      controller: controller,
                      onNewTask: onNewTask,
                      conversation: conversation,
                    ),
                  ),
                  if (showComposer)
                    _Composer(
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
                        snapshot?.permissionPreset,
                      ),
                      modelLabel: _workBuddyModelLabel(
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
          child: const WbIcon(WbGlyph.menu, size: 18),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text('OpenMuse', key: ValueKey('wb-title'), style: wbTitle),
              const SizedBox(height: 3),
              GestureDetector(
                key: const ValueKey('wb-device-workspace'),
                onTap: onSubtitle,
                behavior: HitTestBehavior.opaque,
                child: Row(
                  children: [
                    const WbIcon(
                      WbGlyph.desktop,
                      size: 14,
                      color: WbColors.textDim,
                    ),
                    const SizedBox(width: 4),
                    Text(deviceName, maxLines: 1, style: wbSub),
                    const Text('  |  ', style: wbSub),
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
                    const WbIcon(
                      WbGlyph.chevron,
                      size: 12,
                      color: WbColors.textDim,
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
  });

  final WorkBuddyController controller;
  final VoidCallback onNewTask;
  final Widget? conversation;

  @override
  Widget build(BuildContext context) {
    final task = controller.openTask;
    final nativeSelected =
        controller.tab == WbTab.tasks &&
        task != null &&
        task.id.startsWith('dsh.session.');
    final ordinary = controller.tab == WbTab.tasks && task != null
        ? nativeSelected
              ? const SizedBox.shrink()
              : _Conversation(task: task)
        : _ordinaryTab();
    final cachedConversation = conversation;
    if (cachedConversation == null) return ordinary;
    return Stack(
      fit: StackFit.expand,
      children: [
        Offstage(offstage: nativeSelected, child: ordinary),
        Offstage(offstage: !nativeSelected, child: cachedConversation),
      ],
    );
  }

  Widget _ordinaryTab() => switch (controller.tab) {
    WbTab.tasks => const _HomeEmpty(),
    WbTab.experts => _SimpleList(
      title: '专家',
      lines: const ['通用助手', '编程', '写作', '研究'],
    ),
    WbTab.library => _SimpleList(
      title: '资料库',
      lines: const ['openmuse/README.md', 'muse-clients/README.md'],
    ),
    WbTab.schedule => const _SimpleList(title: '定时任务', lines: ['还没有定时任务']),
    WbTab.projects => _SimpleList(
      title: '项目',
      lines: [
        for (final workspace in controller.workspacesFor(
          controller.selectedDeviceId,
        ))
          workspace.name,
      ],
    ),
  };
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
              'assets/mascot.jpg',
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
              const FittedBox(
                fit: BoxFit.scaleDown,
                child: Text(
                  'OpenMuse，与你一起创造',
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
          const Center(
            child: Text(
              '内容由 AI 生成',
              key: ValueKey('wb-ai-disclaimer'),
              style: TextStyle(color: WbColors.textDim, fontSize: 12),
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
                    color: WbColors.bubble,
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
              const Divider(height: 1, color: WbColors.line),
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
                          color: const Color(0xFFD0D0D0),
                          fontSize: 15,
                        )
                      : wbBody,
                ),
              ],
              const SizedBox(height: 12),
              const Divider(height: 1, color: WbColors.line),
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
          child: const WbIcon(WbGlyph.chevronDown, size: 18),
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
          color: WbColors.textMuted,
        ),
        const SizedBox(width: 8),
        Text(
          status == WbTaskStatus.completed ? '已完成' : '进行中',
          key: const ValueKey('wb-task-status'),
          style: const TextStyle(color: WbColors.textMuted, fontSize: 14),
        ),
        const SizedBox(width: 4),
        const WbIcon(WbGlyph.chevron, size: 12, color: WbColors.textMuted),
      ],
    ),
  );
}

final class _Composer extends StatelessWidget {
  const _Composer({
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
        color: WbColors.canvas,
        borderRadius: BorderRadius.circular(27),
        border: Border.all(color: holding ? Colors.white : WbColors.border),
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
                  tooltip: voiceMode ? '切换到键盘输入' : '切换到语音输入',
                  onPressed: enabled && !holding ? onInputModeToggle : null,
                  visualDensity: VisualDensity.compact,
                  padding: const EdgeInsets.all(6),
                  constraints: const BoxConstraints(
                    minWidth: 36,
                    minHeight: 36,
                  ),
                  icon: voiceMode
                      ? const Icon(
                          Icons.keyboard_alt_outlined,
                          size: 22,
                          color: WbColors.textMuted,
                        )
                      : const WbIcon(
                          WbGlyph.voice,
                          size: 21,
                          color: WbColors.textMuted,
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
                                  ? const Color(0x263DD8BC)
                                  : const Color(0xFF25272D),
                              borderRadius: BorderRadius.circular(21),
                            ),
                            child: Text(
                              holding ? '松开完成 · 上滑取消' : '按住说话',
                              style: TextStyle(
                                color: holding
                                    ? const Color(0xFF8EF5E1)
                                    : WbColors.text,
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
                          minLines: 1,
                          maxLines: 4,
                          style: const TextStyle(
                            color: WbColors.text,
                            fontSize: 16,
                          ),
                          cursorColor: Colors.white,
                          textInputAction: TextInputAction.send,
                          decoration: const InputDecoration(
                            isDense: true,
                            border: InputBorder.none,
                            hintText: '发消息',
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
                  tooltip: '添加文件或图片',
                  onPressed: enabled ? onAttachment : null,
                  icon: const WbIcon(
                    WbGlyph.plus,
                    size: 22,
                    color: WbColors.text,
                  ),
                ),
              ],
            ),
            Row(
              children: [
                _ComposerChip(
                  key: const ValueKey('wb-permission'),
                  icon: Icons.admin_panel_settings_outlined,
                  label: permissionLabel,
                  onTap: enabled ? onPermission : null,
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
                    tooltip: '停止生成',
                    onPressed: onCancel,
                    icon: const Icon(Icons.stop_rounded, size: 20),
                  )
                else
                  ValueListenableBuilder<TextEditingValue>(
                    valueListenable: controller,
                    builder: (context, value, _) => IconButton.filled(
                      key: const ValueKey('wb-send'),
                      tooltip: '发送',
                      onPressed: enabled && value.text.trim().isNotEmpty
                          ? onSubmit
                          : null,
                      style: IconButton.styleFrom(
                        backgroundColor: const Color(0xFF94A9FF),
                        disabledBackgroundColor: const Color(0xFF343846),
                        foregroundColor: Colors.white,
                      ),
                      icon: const Icon(Icons.arrow_upward_rounded, size: 20),
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
          Icon(icon, size: 15, color: WbColors.textMuted),
          const SizedBox(width: 4),
          Flexible(
            child: Text(
              label,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(color: WbColors.textMuted, fontSize: 12),
            ),
          ),
          const SizedBox(width: 2),
          const Icon(Icons.expand_more, size: 14, color: WbColors.textDim),
        ],
      ),
    ),
  );
}

String _permissionLabel(String? value) => switch (value) {
  'read-only' => '只读',
  'workspace-write' => '工作区内修改',
  'danger-full-access' => '完整访问',
  'auto' => '自动审批',
  final value? when value.isNotEmpty => value,
  _ => '工作区权限',
};

String _workBuddyModelLabel(String? model, String? effort) {
  final value = model?.trim();
  if (value == null || value.isEmpty) return '选择模型';
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
            ? '松手取消语音输入'
            : widget.released
            ? '正在完成语音识别'
            : '正在聆听，松手完成，上滑取消',
        child: Padding(
          padding: const EdgeInsets.fromLTRB(28, 62, 28, 34),
          child: Column(
            children: [
              AnimatedSwitcher(
                duration: const Duration(milliseconds: 140),
                child: Text(
                  widget.cancelling
                      ? '松手取消'
                      : widget.released
                      ? '正在完成识别…'
                      : '松手完成，上滑取消',
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
    const items = [
      (WbTab.tasks, WbGlyph.bubble, '任务'),
      (WbTab.experts, WbGlyph.expert, '专家'),
      (WbTab.library, WbGlyph.library, '资料库'),
      (WbTab.schedule, WbGlyph.alarm, '定时任务'),
      (WbTab.projects, WbGlyph.project, '项目'),
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
    final color = selected ? Colors.white : WbColors.textDim;
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
            Text(label, style: TextStyle(color: color, fontSize: 11)),
            const SizedBox(height: 4),
            AnimatedContainer(
              duration: const Duration(milliseconds: 160),
              width: selected ? 22 : 0,
              height: 2.5,
              decoration: BoxDecoration(
                color: Colors.white,
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
    color: WbColors.chip,
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
    required this.onAccount,
    required this.onOpenSession,
    required this.onOpenWorkbench,
  });

  final WorkBuddyController controller;
  final VoidCallback onNewTask;
  final VoidCallback onDevice;
  final VoidCallback onAccount;
  final ValueChanged<WbTask> onOpenSession;
  final VoidCallback onOpenWorkbench;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) {
      final accountName = controller.accountName?.trim();
      final signedIn = controller.signedIn;
      final name = signedIn
          ? (accountName?.isNotEmpty == true ? accountName! : 'OpenMuse 用户')
          : '未登录';
      final mark = signedIn ? name.substring(0, 1) : '未';
      final spaces = controller.workspacesFor(controller.selectedDeviceId);
      final tasks = controller.taskList;
      return Material(
        color: WbColors.drawer,
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
                        const WbIcon(
                          WbGlyph.desktop,
                          size: 22,
                          color: Color(0xFF7DDEA8),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            controller.selectedDevice.kind == WbDeviceKind.cloud
                                ? '云端'
                                : controller.selectedDevice.name,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: const TextStyle(
                              color: WbColors.text,
                              fontSize: 18,
                              fontWeight: FontWeight.w700,
                            ),
                          ),
                        ),
                        const WbIcon(
                          WbGlyph.chevronDown,
                          size: 16,
                          color: WbColors.textDim,
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 10),
                OutlinedButton(
                  key: const ValueKey('wb-new-task'),
                  style: OutlinedButton.styleFrom(
                    foregroundColor: WbColors.text,
                    side: const BorderSide(color: WbColors.borderSoft),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(28),
                    ),
                    padding: const EdgeInsets.symmetric(vertical: 14),
                  ),
                  onPressed: onNewTask,
                  child: const Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      WbIcon(WbGlyph.bubblePlus, size: 18),
                      SizedBox(width: 8),
                      Text('新建任务', style: TextStyle(fontSize: 16)),
                    ],
                  ),
                ),
                const SizedBox(height: 18),
                Expanded(
                  child: ListView(
                    padding: EdgeInsets.zero,
                    children: [
                      _SectionLabel(
                        label: '任务 (${tasks.length})',
                        expanded: controller.tasksExpanded,
                        onTap: controller.toggleTasks,
                      ),
                      if (controller.tasksExpanded)
                        for (final task in tasks)
                          _DrawerRow(
                            key: ValueKey('wb-task-${task.id}'),
                            title: task.title,
                            indent: 8,
                            onTap: () => task.id.startsWith('dsh.session.')
                                ? onOpenSession(task)
                                : controller.openTaskById(task.id),
                          ),
                      const SizedBox(height: 8),
                      _SectionLabel(
                        label: '空间 (${spaces.length})',
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
                      const SizedBox(height: 8),
                      _DrawerRow(
                        key: const ValueKey('wb-remote-workbench'),
                        title: '远程工作台',
                        icon: WbGlyph.desktop,
                        indent: 0,
                        onTap: onOpenWorkbench,
                      ),
                      const _DrawerRow(
                        title: '助理',
                        icon: WbGlyph.personPlus,
                        indent: 0,
                      ),
                    ],
                  ),
                ),
                const Divider(color: WbColors.line, height: 1),
                InkWell(
                  key: const ValueKey('wb-account'),
                  onTap: onAccount,
                  child: Padding(
                    padding: const EdgeInsets.only(top: 12, bottom: 4),
                    child: Row(
                      children: [
                        CircleAvatar(
                          radius: 18,
                          backgroundColor: WbColors.avatar,
                          child: Text(
                            mark,
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 14,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ),
                        const SizedBox(width: 10),
                        Expanded(
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                name,
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                                style: const TextStyle(
                                  color: WbColors.text,
                                  fontSize: 15,
                                ),
                              ),
                              const SizedBox(height: 2),
                              Text(signedIn ? '已登录' : '点击登录', style: wbSub),
                            ],
                          ),
                        ),
                      ],
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
            style: const TextStyle(color: WbColors.textDim, fontSize: 14),
          ),
          const SizedBox(width: 4),
          WbIcon(
            expanded ? WbGlyph.chevronDown : WbGlyph.chevron,
            size: 12,
            color: WbColors.textDim,
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
    this.icon,
    this.onTap,
  });

  final String title;
  final double indent;
  final WbGlyph? icon;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => InkWell(
    onTap: onTap,
    child: Padding(
      padding: EdgeInsets.fromLTRB(indent, 11, 0, 11),
      child: Row(
        children: [
          if (icon != null) ...[
            WbIcon(icon!, size: 18, color: WbColors.textMuted),
            const SizedBox(width: 8),
          ],
          Expanded(
            child: Text(
              title,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(color: WbColors.text, fontSize: 16),
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
        const WbIcon(WbGlyph.folder, size: 18, color: WbColors.textMuted),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            workspace.name,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(color: WbColors.text, fontSize: 16),
          ),
        ),
        IconButton(
          onPressed: onToggle,
          icon: WbIcon(
            expanded ? WbGlyph.chevronDown : WbGlyph.chevron,
            size: 14,
            color: WbColors.textDim,
          ),
        ),
        IconButton(
          key: ValueKey('wb-space-plus-${workspace.id}'),
          onPressed: onPlus,
          icon: const WbIcon(WbGlyph.plus, size: 16, color: WbColors.textDim),
        ),
      ],
    ),
  );
}

final class _SimpleList extends StatelessWidget {
  const _SimpleList({required this.title, required this.lines});

  final String title;
  final List<String> lines;

  @override
  Widget build(BuildContext context) => ListView(
    padding: const EdgeInsets.fromLTRB(20, 12, 20, 20),
    children: [
      Text(title, style: wbTitle),
      const SizedBox(height: 12),
      for (final line in lines)
        Padding(
          padding: const EdgeInsets.symmetric(vertical: 12),
          child: Text(
            line,
            style: const TextStyle(color: WbColors.text, fontSize: 16),
          ),
        ),
    ],
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
              const Expanded(
                child: Text(
                  '任务运行设置',
                  key: ValueKey('wb-run-settings'),
                  style: TextStyle(
                    color: WbColors.text,
                    fontSize: 17,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              IconButton(
                key: const ValueKey('wb-run-settings-close'),
                onPressed: () => Navigator.pop(context),
                icon: const WbIcon(
                  WbGlyph.close,
                  size: 18,
                  color: WbColors.textDim,
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          _SettingCard(
            key: const ValueKey('wb-pick-device'),
            label: '设备',
            value: deviceName,
            showDesktop: true,
            onTap: onDevice,
          ),
          const SizedBox(height: 10),
          _SettingCard(
            key: const ValueKey('wb-pick-workspace'),
            label: '工作空间',
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
              style: const TextStyle(color: WbColors.text, fontSize: 16),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  if (showDesktop) ...[
                    const WbIcon(
                      WbGlyph.desktop,
                      size: 16,
                      color: WbColors.textMuted,
                    ),
                    const SizedBox(width: 6),
                  ],
                  Flexible(
                    child: Text(
                      value,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      textAlign: TextAlign.right,
                      style: const TextStyle(
                        color: WbColors.textMuted,
                        fontSize: 15,
                      ),
                    ),
                  ),
                  const SizedBox(width: 4),
                  const WbIcon(
                    WbGlyph.chevron,
                    size: 14,
                    color: WbColors.textDim,
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
                style: const TextStyle(
                  color: WbColors.text,
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
