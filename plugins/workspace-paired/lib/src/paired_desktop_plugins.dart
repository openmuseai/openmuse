import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'paired_desktop_client.dart';
import 'paired_desktop_gateway.dart';
import 'paired_desktop_models.dart';

final class OpenMusePairedDesktopHostPlugin
    implements OpenMusePlugin, OpenMuseSettingsContributor {
  OpenMusePairedDesktopHostPlugin(this.gateway) {
    gateway.onChanged = _changes.changed;
  }
  final PairedDesktopGateway gateway;
  final _GatewayChanges _changes = _GatewayChanges();

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
  Future<void> activate(OpenMusePluginContext context) => gateway.start();

  @override
  Future<void> deactivate() => gateway.stop();

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
          'Mobile 配对',
          style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        ),
        const SizedBox(height: 8),
        Text('状态：${gateway.running ? '等待连接' : '未启动'}'),
        if (gateway.pairingCode case final code?)
          SelectableText('配对码：$code', key: const ValueKey('pairing-code')),
        const SizedBox(height: 8),
        OutlinedButton(
          onPressed: gateway.running ? gateway.armPairing : null,
          child: const Text('生成新配对码'),
        ),
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
  PairedDesktopMobileController(this.client);
  final PairedDesktopClient client;
  PairedDesktopMobileSnapshot _snapshot = const PairedDesktopMobileSnapshot();
  PairedDesktopMobileSnapshot get snapshot => _snapshot;

  Future<bool> pair(String code) async {
    if (_snapshot.connecting) return false;
    _publish(
      PairedDesktopMobileSnapshot(
        connecting: true,
        connection: _snapshot.connection,
      ),
    );
    try {
      final connection = await client.pair(pairingCode: code);
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
    client.close();
    super.dispose();
  }
}

final class OpenMusePairedDesktopMobilePlugin implements OpenMusePlugin {
  OpenMusePairedDesktopMobilePlugin({required PairedDesktopClient client})
    : controller = PairedDesktopMobileController(client);
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
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async => controller.disconnect();

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('Paired Desktop Mobile has no editor');

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}
