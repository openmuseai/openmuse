import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_controller.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';

void main() {
  test('workspace selection keeps the catalog and restores its last task', () {
    final controller = WorkBuddyController();
    addTearDown(controller.dispose);
    const deviceId = 'paired.desktop-1';
    const first = WbWorkspace(
      id: 'dsh.workspace.first',
      name: 'First',
      deviceId: deviceId,
    );
    const second = WbWorkspace(
      id: 'dsh.workspace.second',
      name: 'Second',
      deviceId: deviceId,
    );
    final firstTask = WbTask(
      id: 'dsh.session.first-task',
      title: 'First task',
      workspaceId: first.id,
      deviceId: deviceId,
      status: WbTaskStatus.completed,
      messages: [],
    );
    final secondTask = WbTask(
      id: 'dsh.session.second-task',
      title: 'Second task',
      workspaceId: second.id,
      deviceId: deviceId,
      status: WbTaskStatus.completed,
      messages: [],
    );
    controller.upsertDevice(
      const WbDevice(id: deviceId, name: 'Desktop', kind: WbDeviceKind.local),
    );
    controller.selectDevice(deviceId);
    controller.replacePairedCatalog(
      deviceId: deviceId,
      workspaces: [first, second],
      sessions: [firstTask, secondTask],
    );

    controller.openTaskById(firstTask.id);
    controller.selectWorkspace(second.id);
    expect(controller.selectedWorkspaceId, second.id);
    expect(controller.openTask, isNull);
    controller.openTaskById(secondTask.id);
    controller.selectWorkspace(first.id);
    expect(controller.openTask?.id, firstTask.id);
    controller.selectWorkspace(second.id);
    expect(controller.openTask?.id, secondTask.id);
    expect(controller.workspacesFor(deviceId), hasLength(2));
    expect(controller.expandedWorkspaces, isEmpty);
  });
}
