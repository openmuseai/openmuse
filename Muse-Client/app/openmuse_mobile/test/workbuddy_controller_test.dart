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
    controller.selectTab(WbTab.home);
    expect(controller.openTask?.id,firstTask.id);
    expect(controller.selectedDeviceId,deviceId);
    controller.selectTab(WbTab.tasks);
    expect(controller.openTask?.id,firstTask.id);
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

  test('recent tasks are the five newest across workspaces', () {
    final controller = WorkBuddyController();
    addTearDown(controller.dispose);
    const firstDevice = 'paired.one';
    const secondDevice = 'paired.two';
    controller.upsertDevice(
      const WbDevice(id: firstDevice, name: 'One', kind: WbDeviceKind.local),
    );
    controller.upsertDevice(
      const WbDevice(id: secondDevice, name: 'Two', kind: WbDeviceKind.local),
    );
    WbTask task({
      required String id,
      required String deviceId,
      required String workspaceId,
      required int updatedAt,
    }) => WbTask(
      id: id,
      title: id,
      workspaceId: workspaceId,
      deviceId: deviceId,
      status: WbTaskStatus.completed,
      showInTaskList: true,
      updatedAt: updatedAt,
      messages: const [],
    );
    controller.replacePairedCatalog(
      deviceId: firstDevice,
      workspaces: const [
        WbWorkspace(id: 'dsh.workspace.a', name: 'A', deviceId: firstDevice),
      ],
      sessions: [
        task(
          id: 'dsh.session.a1',
          deviceId: firstDevice,
          workspaceId: 'dsh.workspace.a',
          updatedAt: 10,
        ),
        task(
          id: 'dsh.session.a2',
          deviceId: firstDevice,
          workspaceId: 'dsh.workspace.a',
          updatedAt: 50,
        ),
        task(
          id: 'dsh.session.a3',
          deviceId: firstDevice,
          workspaceId: 'dsh.workspace.a',
          updatedAt: 30,
        ),
      ],
    );
    controller.replacePairedCatalog(
      deviceId: secondDevice,
      workspaces: const [
        WbWorkspace(id: 'dsh.workspace.b', name: 'B', deviceId: secondDevice),
      ],
      sessions: [
        task(
          id: 'dsh.session.b1',
          deviceId: secondDevice,
          workspaceId: 'dsh.workspace.b',
          updatedAt: 40,
        ),
        task(
          id: 'dsh.session.b2',
          deviceId: secondDevice,
          workspaceId: 'dsh.workspace.b',
          updatedAt: 20,
        ),
        task(
          id: 'dsh.session.b3',
          deviceId: secondDevice,
          workspaceId: 'dsh.workspace.b',
          updatedAt: 5,
        ),
      ],
    );

    expect(controller.recentTasks.map((task) => task.id).toList(), [
      'dsh.session.a2',
      'dsh.session.b1',
      'dsh.session.a3',
      'dsh.session.b2',
      'dsh.session.a1',
    ]);
  });
}
