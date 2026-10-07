import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'account_device_directory.dart';
import 'desktop_outbound_relay.dart';
import 'paired_desktop_client.dart';
import 'paired_desktop_gateway.dart';
import 'paired_desktop_models.dart';

final class OpenMusePairedDesktopHostPlugin
    implements OpenMusePlugin, OpenMuseSettingsContributor {
  OpenMusePairedDesktopHostPlugin(
    this.gateway, {
    this.directory,
    this.relay,
  }) {
    gateway.onChanged = _changes.changed;
    directory?.addListener(_changes.changed);
    final outbound = relay;
    if (outbound != null) outbound.onChanged = _changes.changed;
  }
  final PairedDesktopGateway gateway;
  final AccountDeviceDirectoryController? directory;
  final DesktopOutboundRelay? relay;
  final _GatewayChanges _changes = _GatewayChanges();
  Timer? _gatewayRetry;
  int _gatewayFailureCount = 0;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.workspace.paired.host',
    name: 'Paired Desktop Host',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {
      'network.loopback.listen',
      'authentication.token.validate',
      'workspace.local.grant',
      'dsh.local.proxy',
    },
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    // Presence is the control plane and must remain available even when the
    // local data-plane port is temporarily occupied.
    await directory?.activate();
    await relay?.start();
    await _ensureGateway();
  }

  @override
  Future<void> deactivate() async {
    _gatewayRetry?.cancel();
    _gatewayRetry = null;
    await relay?.stop();
    directory?.dispose();
    await gateway.stop();
  }

  Future<void> _ensureGateway() async {
    if (gateway.running) return;
    try {
      await gateway.start();
      _gatewayFailureCount = 0;
      _gatewayRetry?.cancel();
      _gatewayRetry = null;
      // Republish the transport origin/capability after a successful retry.
      await directory?.reconcile();
    } catch (error) {
      debugPrint('OpenMuse paired gateway: $error');
      final delays = <int>[1, 2, 4, 8, 16, 30];
      final index = _gatewayFailureCount.clamp(0, delays.length - 1);
      _gatewayFailureCount += 1;
      _gatewayRetry?.cancel();
      _gatewayRetry = Timer(Duration(seconds: delays[index]), () {
        unawaited(_ensureGateway());
      });
    }
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Paired Desktop Host has no editor');

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;

  @override
  Widget buildSettings(BuildContext context) => ListenableBuilder(
    listenable: _changes,
    builder: (context, _) => Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(height: 18),
        const Text(
          '多端协同',
          style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        ),
        const SizedBox(height: 8),
        Text(
          '状态：${directory?.snapshot.registered == true ? '在线' : '离线'}',
          key: const ValueKey('desktop-device-presence'),
        ),
        const SizedBox(height: 4),
        Text(
          '实时通道：${directory?.snapshot.realtimeConnected == true
              ? '已连接'
              : directory?.snapshot.reconnecting == true
              ? '正在重连'
              : '未连接'}',
          key: const ValueKey('desktop-device-realtime'),
        ),
        const SizedBox(height: 4),
        Text(
          '传输：${gateway.running
              ? '已就绪'
              : gateway.lastError != null
              ? '端口不可用，正在重试'
              : '未启动'}',
        ),
        const SizedBox(height: 4),
        Text(
          '公网通道：${switch (relay?.phase) {
            null => '未启用',
            DesktopRelayPhase.attached => '已连接',
            DesktopRelayPhase.reconnecting => '正在重连',
            DesktopRelayPhase.connecting => '正在连接',
            DesktopRelayPhase.idle => '未连接',
          }}',
          key: const ValueKey('desktop-relay-phase'),
        ),
        const SizedBox(height: 8),
        const Text('同账号设备在线后可直接访问；跨账号授权将在后续版本通过独立配对流程提供。'),
        if (directory case final value?) ...[
          const SizedBox(height: 18),
          const Text(
            '同账号设备',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          AccountDeviceList(controller: value),
        ],
      ],
    ),
  );
}

final class _GatewayChanges extends ChangeNotifier {
  void changed() => notifyListeners();
}

@immutable
final class PairedDesktopMobileSnapshot {
  const PairedDesktopMobileSnapshot({
    this.connecting = false,
    this.connection,
    this.failureMessage,
  });
  final bool connecting;
  final PairedDesktopConnection? connection;
  final String? failureMessage;
}

final class PairedDesktopMobileController extends ChangeNotifier {
  PairedDesktopMobileController.direct(this._directClient)
    : directory = null,
      accessToken = null,
      requesterDeviceRef = null,
      allowInsecureLoopback = false,
      allowInsecurePrivateNetworkForTesting = false;

  PairedDesktopMobileController.discovered({
    required this.directory,
    required this.accessToken,
    required this.requesterDeviceRef,
    required this.allowInsecureLoopback,
    required this.allowInsecurePrivateNetworkForTesting,
  }) : _directClient = null {
    directory!.addListener(_directoryChanged);
  }

  final PairedDesktopClient? _directClient;
  final AccountDeviceDirectoryController? directory;
  final PairedAccessTokenProvider? accessToken;
  final String? requesterDeviceRef;
  final bool allowInsecureLoopback;
  final bool allowInsecurePrivateNetworkForTesting;
  PairedDesktopMobileSnapshot _snapshot = const PairedDesktopMobileSnapshot();
  PairedDesktopMobileSnapshot get snapshot => _snapshot;

  List<AccountDevice> get devices => directory?.snapshot.devices ?? const [];

  void _directoryChanged() => notifyListeners();

  Future<bool> pair(String code) async {
    final client = _directClient;
    if (client == null) {
      _publish(
        const PairedDesktopMobileSnapshot(failureMessage: '请先选择一台在线 Desktop。'),
      );
      return false;
    }
    return _pairWithClient(client, code);
  }

  Future<bool> connectDevice(AccountDevice device, {bool force = false}) async {
    if (!device.online) {
      _publish(
        const PairedDesktopMobileSnapshot(
          failureMessage: '这台 Desktop 当前离线，无法连接。',
        ),
      );
      return false;
    }
    if (!device.supportsPairedDesktop) {
      _publish(
        const PairedDesktopMobileSnapshot(
          failureMessage: '这台设备不支持 Desktop Workspace。',
        ),
      );
      return false;
    }
    final existing = _snapshot.connection;
    if (!force &&
        existing != null &&
        existing.deviceRef == device.deviceRef &&
        existing.expiresAtMs >
            DateTime.now()
                .add(const Duration(minutes: 1))
                .millisecondsSinceEpoch) {
      return true;
    }
    final client = PairedDesktopClient(
      origin: device.transportOrigin!,
      accessToken: accessToken!,
      deviceRef: requesterDeviceRef!,
      allowInsecureLoopback: allowInsecureLoopback,
      allowInsecurePrivateNetworkForTesting:
          allowInsecurePrivateNetworkForTesting,
    );
    try {
      return await _connectWithClient(client, device.deviceRef);
    } finally {
      client.close();
    }
  }

  Future<bool> _connectWithClient(
    PairedDesktopClient client,
    String targetDeviceRef,
  ) async {
    if (_snapshot.connecting) return false;
    _publish(
      PairedDesktopMobileSnapshot(
        connecting: true,
        connection: _snapshot.connection,
      ),
    );
    try {
      final connection = await client.connectSameAccount(
        targetDeviceRef: targetDeviceRef,
      );
      _publish(PairedDesktopMobileSnapshot(connection: connection));
      return true;
    } on PairedDesktopFailure catch (error) {
      _publish(PairedDesktopMobileSnapshot(failureMessage: error.safeMessage));
      return false;
    } catch (_) {
      _publish(
        const PairedDesktopMobileSnapshot(
          failureMessage: '无法连接 Desktop，请确认设备在线且网络可达。',
        ),
      );
      return false;
    }
  }

  Future<bool> _pairWithClient(
    PairedDesktopClient client,
    String code, {
    String? targetDeviceRef,
  }) async {
    if (_snapshot.connecting) return false;
    _publish(
      PairedDesktopMobileSnapshot(
        connecting: true,
        connection: _snapshot.connection,
      ),
    );
    try {
      final connection = await client.pair(
        pairingCode: code,
        targetDeviceRef: targetDeviceRef,
      );
      _publish(PairedDesktopMobileSnapshot(connection: connection));
      return true;
    } on PairedDesktopFailure catch (error) {
      _publish(PairedDesktopMobileSnapshot(failureMessage: error.safeMessage));
      return false;
    } catch (_) {
      _publish(
        const PairedDesktopMobileSnapshot(
          failureMessage: '无法连接 Desktop，请确认 Desktop 在线且已授权。',
        ),
      );
      return false;
    }
  }

  void disconnect() => _publish(const PairedDesktopMobileSnapshot());

  void _publish(PairedDesktopMobileSnapshot value) {
    _snapshot = value;
    notifyListeners();
  }

  @override
  void dispose() {
    directory?.removeListener(_directoryChanged);
    _directClient?.close();
    super.dispose();
  }
}

final class OpenMusePairedDesktopMobilePlugin implements OpenMusePlugin {
  OpenMusePairedDesktopMobilePlugin({required PairedDesktopClient client})
    : directory = null,
      controller = PairedDesktopMobileController.direct(client);

  OpenMusePairedDesktopMobilePlugin.discovered({
    required AccountDeviceDirectoryController directory,
    required PairedAccessTokenProvider accessToken,
    required String deviceRef,
    bool allowInsecureLoopback = false,
    bool allowInsecurePrivateNetworkForTesting = false,
  }) : directory = directory,
       controller = PairedDesktopMobileController.discovered(
         directory: directory,
         accessToken: accessToken,
         requesterDeviceRef: deviceRef,
         allowInsecureLoopback: allowInsecureLoopback,
         allowInsecurePrivateNetworkForTesting:
             allowInsecurePrivateNetworkForTesting,
       );
  final AccountDeviceDirectoryController? directory;
  final PairedDesktopMobileController controller;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.workspace.paired.mobile',
    name: 'Paired Desktop Workspace',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {
      'network.paired-desktop',
      'workspace.paired.read',
      'dsh.paired.attach',
    },
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    await directory?.activate();
  }

  @override
  Future<void> deactivate() async {
    controller.disconnect();
    controller.dispose();
    directory?.dispose();
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Paired Desktop Mobile has no editor');

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class AccountDeviceList extends StatelessWidget {
  const AccountDeviceList({
    super.key,
    required this.controller,
    this.onSelect,
    this.selectedDeviceRef,
    this.excludeDeviceRef,
  });

  final AccountDeviceDirectoryController controller;
  final ValueChanged<AccountDevice>? onSelect;
  final String? selectedDeviceRef;
  final String? excludeDeviceRef;

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: controller,
    builder: (context, _) {
      final snapshot = controller.snapshot;
      final devices = snapshot.devices
          .where((device) => device.deviceRef != excludeDeviceRef)
          .toList(growable: false);
      if (snapshot.loading && devices.isEmpty) {
        return const Center(child: CircularProgressIndicator());
      }
      return Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (devices.isEmpty) const Text('当前账号还没有其他设备。'),
          for (final device in devices)
            Card(
              margin: const EdgeInsets.only(bottom: 8),
              child: ListTile(
                key: ValueKey('account-device:${device.deviceRef}'),
                leading: Icon(
                  device.kind == AccountDeviceKind.desktop
                      ? Icons.computer_outlined
                      : Icons.phone_android_outlined,
                ),
                title: Text(device.displayName),
                subtitle: Text(
                  '${device.platform} · ${device.online ? '在线' : '离线'}',
                  key: ValueKey('device-presence:${device.deviceRef}'),
                ),
                trailing: device.online
                    ? const Icon(Icons.circle, size: 12, color: Colors.green)
                    : const Icon(Icons.circle_outlined, size: 12),
                selected: selectedDeviceRef == device.deviceRef,
                enabled:
                    device.online &&
                    (onSelect == null || device.supportsPairedDesktop),
                onTap:
                    onSelect == null ||
                        !device.online ||
                        !device.supportsPairedDesktop
                    ? null
                    : () => onSelect!(device),
              ),
            ),
          if (snapshot.failureMessage case final message?)
            Text(
              message,
              style: TextStyle(color: Theme.of(context).colorScheme.error),
            ),
          Align(
            alignment: Alignment.centerLeft,
            child: TextButton.icon(
              onPressed: snapshot.loading ? null : controller.refresh,
              icon: const Icon(Icons.refresh),
              label: const Text('刷新设备'),
            ),
          ),
        ],
      );
    },
  );
}
