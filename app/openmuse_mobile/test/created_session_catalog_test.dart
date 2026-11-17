import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';

void main() {
  test(
    'acknowledged creation opens without a catalog round trip and preserves other sessions',
    () {
      final controller = WorkBuddyController();
      addTearDown(controller.dispose);
      const device = 'paired.desktop';
      const workspace = WbWorkspace(
        id: 'dsh.workspace.w',
        name: 'Workspace',
        deviceId: device,
      );
      controller.upsertDevice(
        const WbDevice(id: device, name: 'Desktop', kind: WbDeviceKind.local),
      );
      controller.selectDevice(device);
      WbTask task(String id) => WbTask(
        id: 'dsh.session.$id',
        title: id,
        workspaceId: workspace.id,
        deviceId: device,
        status: WbTaskStatus.completed,
        messages: const [],
      );
      controller.replacePairedCatalog(
        deviceId: device,
        workspaces: [workspace],
        sessions: [task('old')],
      );
      controller.acceptCreatedPairedSession(task('created'));
      controller.openTaskById('dsh.session.created');
      expect(controller.openTask?.id, 'dsh.session.created');
      expect(controller.tasks.any((t) => t.id == 'dsh.session.old'), true);
      controller.acceptCreatedPairedSession(task('created'));
      expect(
        controller.tasks.where((t) => t.id == 'dsh.session.created').length,
        1,
      );
    },
  );
}
