import 'dart:async';

import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_file_chooser_host.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_broker.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_native_capability_host.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_control_host.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_error_codes.dart';
import 'package:muse_dsh_mobile/src/dsh_mobile_surface_state.dart';
import 'package:muse_dsh_mobile/src/dsh_remote_config.dart';
import 'package:muse_dsh_mobile/src/dsh_placement.dart';
import 'package:muse_dsh_mobile/src/dsh_session_api.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_webview_manager.dart';
import 'package:muse_dsh_mobile/src/webview/dsh_webview_session.dart';

class DshMobileCoordinator with WidgetsBindingObserver {
  DshMobileCoordinator({
    required this.scope,
    required this.notify,
    this.controlHost,
    this.endpoint,
    this.sessionWebUrl,
    this.sessionApi,
    this.sessionRef,
    this.sessionDeviceId,
    this.accessToken,
    this.requireRemoteSession = false,
    this.capabilityHost,
    this.fileChooserHost,
    this.sleep = _defaultSleep,
  });

  static Future<void> _defaultSleep(Duration duration) =>
      Future<void>.delayed(duration);

  final DshMobileScope scope;
  final VoidCallback notify;
  final DshMobileControlHost? controlHost;
  final DshRemoteConfig? endpoint;
  final Uri? sessionWebUrl;
  final DshSessionApi? sessionApi;
  String? sessionRef;
  final String? sessionDeviceId;
  final String? accessToken;
  final bool requireRemoteSession;
  final DshNativeCapabilityHost? capabilityHost;
  final DshFileChooserHost? fileChooserHost;
  final Future<void> Function(Duration duration) sleep;

  final session = DshWebViewSession();
  DshWebViewManager? _manager;
  DshNativeCapabilityBroker? _capabilityBroker;
  DshControlSession? _control;
  StreamSubscription? _network;
  Timer? _scopeCheck;
  String? _fatal;
  String? _bridgeWarning;
  int? _queuePosition;
  bool _bridgeConnected = false;
  bool _loading = false;
  bool _pageLoaded = false;
  int _controlGeneration = 0;
  DateTime? facetReadyAt;
  String? _openedDeviceId;

  DshNativeCapabilityBroker? get capabilityBroker => _capabilityBroker;

  DshMobileSurfaceState get surface => DshMobileSurfaceState(
        generation: session.generation,
        loading: _loading,
        documentReady: _pageLoaded,
        facetReady: _bridgeConnected,
        fatalMessage: _fatal,
        bridgeWarning: _bridgeWarning,
        queuePosition: _queuePosition,
        controller: _manager?.controller,
      );

  void start() {
    WidgetsBinding.instance.addObserver(this);
    _network = Connectivity().onConnectivityChanged.listen((state) {
      if (state == ConnectivityResult.none) {
        _degrade(DshMobileErrorCode.networkOffline.message);
      }
    });
    WidgetsBinding.instance.addPostFrameCallback((_) => connect());
  }

  Future<void> dispose() async {
    _scopeCheck?.cancel();
    _scopeCheck = null;
    WidgetsBinding.instance.removeObserver(this);
    await _network?.cancel();
    await _disconnect();
    await _manager?.dispose();
    _manager = null;
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.detached) {
      unawaited(_capabilityBroker?.emitLifecycle('background'));
      _degrade(DshMobileErrorCode.appBackgrounded.message);
    } else if (state == AppLifecycleState.resumed) {
      unawaited(_capabilityBroker?.emitLifecycle('foreground'));
    }
  }

  Future<void> connect() async {
    if (_loading) return;
    _loading = true;
    _fatal = null;
    _bridgeWarning = null;
    _pageLoaded = false;
    final previous = _manager;
    _manager = null;
    notify();
    await _disconnect();
    await previous?.dispose();
    final generation = session.generation;
    bool live() => session.isLive(generation) && scope.isCurrentScope();
    _queuePosition = null;
    try {
      if (requireRemoteSession && sessionApi == null) {
        throw StateError(DshMobileErrorCode.needAuth.code);
      }
      final config = await _resolveConfig(live);
      if (config == null ||
          scope.workspaceRef.isEmpty ||
          !scope.isCurrentScope()) {
        throw StateError('INVALID_WORKSPACE_OR_ENDPOINT');
      }
      _scopeCheck = Timer.periodic(const Duration(seconds: 1), (_) {
        if (session.isLive(generation) && !scope.isCurrentScope()) {
          _fail(DshMobileErrorCode.scopeChanged.message);
        }
      });
      final capability = capabilityHost;
      final broker = capability == null
          ? null
          : DshNativeCapabilityBroker(
              config: config,
              host: capability,
              generationOf: () => session.generation,
              isLive: live,
            );
      _capabilityBroker = broker;
      final manager = DshWebViewManager(
        config: config,
        session: session,
        isLive: live,
        onDocumentReadyChanged: (ready) {
          _pageLoaded = ready;
          notify();
        },
        onFatal: _fail,
        capabilityBroker: broker,
        fileChooserHost: fileChooserHost,
        ingressAccessToken: accessToken,
      );
      await manager.createAndLoad();
      if (!live()) return;
      _manager = manager;
      _loading = false;
      notify();
      if (live() && _bridgeWarning == null) {
        unawaited(_connectControl(config, generation));
      }
    } catch (error) {
      if (session.isLive(generation)) {
        if (error is StateError &&
            error.message == DshMobileErrorCode.needAuth.code) {
          _fail(DshMobileErrorCode.needAuth.message);
        } else {
          _fail(DshMobileErrorCode.configInvalid.message);
        }
      }
    }
  }

  Future<void> _disconnect() async {
    final api = sessionApi;
    final ref = sessionRef;
    final device = sessionDeviceId ?? _openedDeviceId;
    if (api != null && ref != null && device != null) {
      try {
        await api.close(sessionRef: ref, deviceId: device);
      } catch (_) {
        /* best-effort release */
      }
    }
    session.bump();
    _scopeCheck?.cancel();
    await _capabilityBroker?.cancelSpeech();
    await _disconnectControl();
  }

  Future<DshRemoteConfig?> _resolveConfig(bool Function() live) async {
    final api = sessionApi;
    if (api != null) {
      final device = sessionDeviceId ?? 'mobile.${scope.accountRef}';
      _openedDeviceId = device;
      final allowlist = DshPlacement.pageAllowlist(
        cloudOrigin: api.cloudOrigin,
        compiled: (endpoint ?? DshRemoteConfig.fromEnvironment())?.publicUri,
      );
      var opened = await api.open(
        workspaceRef: scope.workspaceRef,
        deviceId: device,
      );
      while (opened.isQueued) {
        if (!live()) return null;
        sessionRef = opened.sessionRef;
        _queuePosition = opened.queuePosition;
        notify();
        await sleep(Duration(milliseconds: opened.retryAfterMs ?? 10000));
        if (!live()) return null;
        opened = await api.open(
          workspaceRef: scope.workspaceRef,
          deviceId: device,
        );
      }
      final decision = DshPlacement.fromOpen(
        accessToken: accessToken ?? 'session-api',
        session: opened,
        allowlist: allowlist,
      );
      if (decision.isFailed) {
        throw StateError(decision.errorCode ?? DshMobileErrorCode.configInvalid.code);
      }
      sessionRef = decision.sessionRef;
      _queuePosition = null;
      return DshRemoteConfig.fromWebUrl(decision.webUrl!);
    }
    final resolved = sessionWebUrl;
    final config = resolved != null
        ? DshRemoteConfig.fromWebUrl(resolved.toString())
        : endpoint ?? DshRemoteConfig.fromEnvironment();
    if (config != null && DshPlacement.isSharedDshPath(config.publicUri)) {
      throw StateError(DshMobileErrorCode.configInvalid.code);
    }
    return config;
  }

  Future<void> _disconnectControl() async {
    ++_controlGeneration;
    _bridgeConnected = false;
    final control = _control;
    _control = null;
    await control?.close();
  }

  Future<void> _connectControl(
    DshRemoteConfig config,
    int pageGeneration,
  ) async {
    final host = controlHost;
    if (host == null) {
      _degrade(DshMobileErrorCode.bridgeUnavailable.message);
      return;
    }
    final generation = _controlGeneration;
    bool live() =>
        session.isLive(pageGeneration) &&
        generation == _controlGeneration &&
        scope.isCurrentScope();
    try {
      final control = await host.connect(
        DshControlSessionRequest(
          endpoint: config.publicUri,
          workspaceRef: scope.workspaceRef,
          workspaceTitle: scope.workspaceTitle,
          accountRef: scope.accountRef,
          isCloudAccount: scope.isCloudAccount,
          isLive: live,
          onDisconnected: () {
            if (live()) {
              _degrade(DshMobileErrorCode.bridgeDisconnected.message);
            }
          },
          onDegraded: (message) {
            if (live()) _degrade(message);
          },
        ),
      );
      if (!live()) {
        await control.close();
        return;
      }
      _control = control;
      _bridgeConnected = true;
      facetReadyAt = DateTime.now();
      notify();
    } on DshControlConnectException catch (error) {
      if (!live()) return;
      _degrade(_mapControlError(error));
    } catch (_) {
      if (live()) {
        _degrade(DshMobileErrorCode.bridgeUnavailable.message);
      }
    }
  }

  String _mapControlError(DshControlConnectException error) {
    if (error.code == 'CLOUD_LOGIN_REQUIRED') {
      return DshMobileErrorCode.cloudLoginRequired.message;
    }
    if (error.code == 'HOST_UPGRADE_REQUIRED' || error.status == 404) {
      return DshMobileErrorCode.hostUpgradeRequired.message;
    }
    if (error.code.startsWith('CLOUD_') ||
        error.code == 'STALE_REQUEST' ||
        error.code == 'CURRENT_REQUEST') {
      return DshMobileErrorCode.credentialUnavailable(error.code);
    }
    return DshMobileErrorCode.bridgeUnavailable.message;
  }

  void _fail(String message) {
    final previous = _manager;
    _manager = null;
    unawaited(_disconnect());
    unawaited(previous?.dispose() ?? Future<void>.value());
    _fatal = message;
    _loading = false;
    notify();
  }

  void _degrade(String message) {
    unawaited(_disconnectControl());
    _bridgeWarning = message;
    notify();
  }
}
