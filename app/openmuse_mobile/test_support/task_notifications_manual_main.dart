import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_task_notifications/openmuse_task_notifications.dart';

/// Isolated native adapter fixture. No authentication, user documents, remote
/// task server or model is connected. Never use this target for a store build.
Future<void> main() async {
  if (!kDebugMode) throw UnsupportedError('Debug acceptance fixture only');
  WidgetsFlutterBinding.ensureInitialized();
  String? preference;
  final notifications = OpenMuseTaskNotifications(
    readPreference: () async => preference,
    writePreference: (value) async => preference = value,
  );
  await notifications.ready;
  notifications.setScope('qa:synthetic:local');
  runApp(MaterialApp(home: _NativeFixture(notifications: notifications)));
}

class _NativeFixture extends StatefulWidget {
  const _NativeFixture({required this.notifications});
  final OpenMuseTaskNotifications notifications;
  @override
  State<_NativeFixture> createState() => _NativeFixtureState();
}

class _NativeFixtureState extends State<_NativeFixture> {
  int sequence = 0, opened = 0;
  @override
  void initState() {
    super.initState();
    widget.notifications.onOpenTasks = () {
      if (mounted) setState(() => opened++);
    };
  }

  @override
  void dispose() {
    widget.notifications.dispose();
    super.dispose();
  }

  Future<void> _outcome(bool succeeded) => widget.notifications.observe(
    sessionId: 'synthetic-qa',
    sequence: ++sequence,
    succeeded: succeeded,
    strings: OpenMuseStrings.fallback,
  );
  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text('Native QA · activations $opened')),
    body: Column(
      children: [
        Wrap(
          spacing: 8,
          children: [
            TextButton(
              onPressed: () => _outcome(true),
              child: const Text('Synthetic success'),
            ),
            TextButton(
              onPressed: () => _outcome(false),
              child: const Text('Synthetic failure'),
            ),
            TextButton(
              onPressed: () => widget.notifications.setScope(null),
              child: const Text('Clear scope'),
            ),
          ],
        ),
        Expanded(
          child: TaskNotificationsSettings(notifications: widget.notifications),
        ),
      ],
    ),
  );
}
