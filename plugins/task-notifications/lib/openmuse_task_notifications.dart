import 'dart:async';
import 'dart:collection';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

enum NotificationAuthorization { notDetermined, granted, denied, unavailable }

abstract interface class TaskNotificationPort {
  Future<NotificationAuthorization> authorization();
  Future<NotificationAuthorization> requestAuthorization();
  Future<void> openSettings();
  Future<void> show({
    required int id,
    required String title,
    required String body,
    required String channelName,
  });
  Future<void> clear();
  Future<bool> takeActivation();
  void setActivationHandler(VoidCallback? handler);
}

final class MethodChannelTaskNotifications implements TaskNotificationPort {
  static const _channel = MethodChannel('io.openmuse/task_notifications');
  static MethodChannelTaskNotifications? _owner;
  VoidCallback? _activation;
  Future<NotificationAuthorization> _status(String method) async {
    final value = await _channel.invokeMethod<String>(method);
    return NotificationAuthorization.values.firstWhere(
      (v) => v.name == value,
      orElse: () => NotificationAuthorization.unavailable,
    );
  }

  @override
  Future<NotificationAuthorization> authorization() => _status('authorization');
  @override
  Future<NotificationAuthorization> requestAuthorization() =>
      _status('request');
  @override
  Future<void> openSettings() => _channel.invokeMethod<void>('settings');
  @override
  Future<void> show({
    required int id,
    required String title,
    required String body,
    required String channelName,
  }) => _channel.invokeMethod<void>('show', {
    'id': id,
    'title': title,
    'body': body,
    'channelName': channelName,
  });
  @override
  Future<void> clear() => _channel.invokeMethod<void>('clear');
  @override
  Future<bool> takeActivation() async =>
      await _channel.invokeMethod<bool>('takeActivation') ?? false;
  @override
  void setActivationHandler(VoidCallback? handler) {
    _activation = handler;
    if (handler != null) {
      _owner = this;
      _channel.setMethodCallHandler((call) async {
        if (call.method == 'activated') _activation?.call();
      });
    } else if (identical(_owner, this)) {
      _owner = null;
      _channel.setMethodCallHandler(null);
    }
  }
}

/// Owns notification policy only. Tasks and document contents remain elsewhere.
final class OpenMuseTaskNotifications extends ChangeNotifier
    implements OpenMusePlugin {
  OpenMuseTaskNotifications({
    required this.readPreference,
    required this.writePreference,
    TaskNotificationPort? port,
  }) : port = port ?? MethodChannelTaskNotifications() {
    this.port.setActivationHandler(_activated);
    ready = _restore();
  }
  final Future<String?> Function() readPreference;
  final Future<void> Function(String value) writePreference;
  final TaskNotificationPort port;
  late final Future<void> ready;
  bool enabled = false;
  bool busy = false;
  bool failed = false;
  bool _disposed = false;
  bool _active = true;
  String? _scope;
  int _epoch = 0;
  int _serial = 0;
  final _highWater = LinkedHashMap<String, int>();
  Future<void> _nativeQueue = Future<void>.value();
  NotificationAuthorization authorization =
      NotificationAuthorization.unavailable;
  VoidCallback? onOpenTasks;

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.task-notifications',
    name: 'Task notifications',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    panels: [
      OpenMusePanelContribution(
        id: 'task-notifications.settings',
        region: OpenMuseSurfaceRegion.dialog,
      ),
    ],
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {
    if (_disposed) return;
    _active = true;
    port.setActivationHandler(_activated);
    await ready;
  }

  @override
  Future<void> deactivate() async {
    _active = false;
    port.setActivationHandler(null);
    setScope(null);
    await _nativeQueue;
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      buildPanel(context, 'task-notifications.settings')!;
  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'task-notifications.settings'
      ? TaskNotificationsSettings(notifications: this)
      : null;

  void _changed() {
    if (!_disposed) notifyListeners();
  }

  Future<void> _restore() async {
    try {
      enabled = await readPreference() == 'true';
    } on Object {
      failed = true;
    }
    await refreshAuthorization();
  }

  Future<void> refreshAuthorization() async {
    try {
      authorization = await port.authorization();
    } on Object {
      authorization = NotificationAuthorization.unavailable;
    }
    _changed();
  }

  Future<void> setEnabled(bool value) async {
    if (busy || _disposed) return;
    busy = true;
    failed = false;
    _changed();
    final previous = enabled;
    try {
      if (value) authorization = await port.requestAuthorization();
      await writePreference(value.toString());
      if (_disposed) return;
      enabled = value;
      if (!value) {
        _epoch++;
        await _enqueue(port.clear);
      }
    } on Object {
      enabled = previous;
      failed = true;
    } finally {
      busy = false;
      _changed();
    }
  }

  void setScope(String? scope) {
    if (!_active || _disposed) scope = null;
    if (_scope == scope) return;
    _scope = scope;
    _epoch++;
    _highWater.clear();
    unawaited(_enqueue(port.clear));
  }

  void clearScope(String? expected) {
    if (_scope == expected) setScope(null);
  }

  Future<void> _enqueue(Future<void> Function() work) {
    final result = _nativeQueue.then((_) async {
      if (!_disposed) await work();
    });
    _nativeQueue = result.catchError((Object _) {
      failed = true;
      _changed();
    });
    return _nativeQueue;
  }

  Future<void> observe({
    required String sessionId,
    required int sequence,
    required bool succeeded,
    required OpenMuseStrings strings,
  }) async {
    if (_disposed ||
        !_active ||
        _scope == null ||
        sessionId.isEmpty ||
        sessionId.length > 256 ||
        sequence <= 0)
      return;
    if (sequence <= (_highWater[sessionId] ?? 0)) return;
    _highWater.remove(sessionId);
    _highWater[sessionId] = sequence;
    while (_highWater.length > 256) {
      _highWater.remove(_highWater.keys.first);
    }
    // Outcomes while disabled are consumed, never replayed when enabled later.
    if (!enabled) return;
    final epoch = _epoch;
    await _enqueue(() async {
      final status = await port.authorization();
      if (_disposed || epoch != _epoch || !enabled || _scope == null) return;
      authorization = status;
      _changed();
      if (status != NotificationAuthorization.granted) return;
      await port.show(
        id: (++_serial % 2147483646) + 1,
        title: strings.format(
          succeeded ? 'notifications.completed' : 'notifications.failed',
        ),
        body: strings.format('notifications.open_result'),
        channelName: strings.format('notifications.title'),
      );
    });
  }

  void _activated() {
    if (!_disposed && _scope != null) onOpenTasks?.call();
  }

  Future<void> consumeActivation() async {
    try {
      if (await port.takeActivation()) _activated();
    } on Object {
      /* Unsupported platforms do not affect the task list. */
    }
  }

  @override
  void dispose() {
    _disposed = true;
    _epoch++;
    _scope = null;
    port.setActivationHandler(null);
    unawaited(_nativeQueue.then((_) => port.clear()).catchError((Object _) {}));
    super.dispose();
  }
}

final class TaskNotificationsSettings extends StatefulWidget {
  const TaskNotificationsSettings({super.key, required this.notifications});
  final OpenMuseTaskNotifications notifications;
  @override
  State<TaskNotificationsSettings> createState() =>
      _TaskNotificationsSettingsState();
}

final class _TaskNotificationsSettingsState
    extends State<TaskNotificationsSettings>
    with WidgetsBindingObserver {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    unawaited(widget.notifications.refreshAuthorization());
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.resumed) {
      unawaited(widget.notifications.refreshAuthorization());
    }
  }

  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context), n = widget.notifications;
    return Scaffold(
      appBar: AppBar(title: Text(s.format('notifications.title'))),
      body: ListenableBuilder(
        listenable: n,
        builder: (context, _) => ListView(
          padding: const EdgeInsets.all(20),
          children: [
            SwitchListTile.adaptive(
              key: const ValueKey('task-notifications-switch'),
              title: Text(s.format('notifications.results')),
              subtitle: Text(s.format('notifications.description')),
              value: n.enabled,
              onChanged: n.busy
                  ? null
                  : (value) async {
                      if (value) {
                        final accepted = await showDialog<bool>(
                          context: context,
                          builder: (context) => AlertDialog(
                            title: Text(s.format('notifications.title')),
                            content: Text(
                              s.format('notifications.explanation'),
                            ),
                            actions: [
                              TextButton(
                                onPressed: () => Navigator.pop(context, false),
                                child: Text(s.format('notifications.cancel')),
                              ),
                              TextButton(
                                onPressed: () => Navigator.pop(context, true),
                                child: Text(s.format('notifications.continue')),
                              ),
                            ],
                          ),
                        );
                        if (accepted != true) return;
                      }
                      await n.setEnabled(value);
                    },
            ),
            const SizedBox(height: 16),
            ListTile(
              leading: const Icon(Icons.notifications_none),
              title: Text(s.format('notifications.system')),
              subtitle: Text(s.format('notifications.${n.authorization.name}')),
            ),
            if (n.authorization == NotificationAuthorization.denied)
              FilledButton.tonal(
                key: const ValueKey('task-notifications-settings'),
                onPressed: () async {
                  try {
                    await n.port.openSettings();
                  } on Object {
                    n.failed = true;
                    n._changed();
                  }
                },
                child: Text(s.format('notifications.open_settings')),
              ),
            if (n.failed ||
                n.authorization == NotificationAuthorization.unavailable)
              TextButton(
                onPressed: n.refreshAuthorization,
                child: Text(s.format('notifications.retry')),
              ),
            if (n.failed)
              Padding(
                padding: const EdgeInsets.all(12),
                child: Text(s.format('notifications.error')),
              ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 20),
              child: Text(
                s.format('notifications.explanation'),
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
