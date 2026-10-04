import 'package:flutter/foundation.dart';

import 'workbuddy_models.dart';

final class WorkBuddyController extends ChangeNotifier {
  WorkBuddyController({List<WbWorkspace>? extraWorkspaces}) {
    _devices = const [
      WbDevice(id: kCloudDeviceId, name: '云端', kind: WbDeviceKind.cloud),
    ];
    _workspaces = [
      const WbWorkspace(
        id: 'cloud.pending',
        name: '登录后加载 Workspace',
        deviceId: kCloudDeviceId,
      ),
      ...?extraWorkspaces,
    ];
    _tasks = [];
  }

  late List<WbDevice> _devices;
  late List<WbWorkspace> _workspaces;
  late List<WbTask> _tasks;

  String selectedDeviceId = kCloudDeviceId;
  String selectedWorkspaceId = 'cloud.pending';
  WbTab tab = WbTab.tasks;
  bool drawerOpen = false;
  String? openTaskId;
  bool tasksExpanded = true;
  bool spacesExpanded = true;
  final Set<String> expandedWorkspaces = {};
  String? accountName;
  bool signedIn = false;
  int _serial = 0;
  final Map<String, String> _lastTaskByWorkspace = {};

  List<WbDevice> get devices => List.unmodifiable(_devices);
  List<WbWorkspace> get workspaces => List.unmodifiable(_workspaces);
  List<WbTask> get tasks => List.unmodifiable(_tasks);

  WbDevice get selectedDevice =>
      _devices.firstWhere((device) => device.id == selectedDeviceId);

  WbWorkspace get selectedWorkspace => _workspaces.firstWhere(
    (workspace) => workspace.id == selectedWorkspaceId,
    orElse: () => _workspaces.first,
  );

  List<WbWorkspace> workspacesFor(String deviceId) =>
      _workspaces.where((workspace) => workspace.deviceId == deviceId).toList();

  List<WbTask> get taskList => _tasks
      .where((task) => task.showInTaskList && task.deviceId == selectedDeviceId)
      .toList();

  List<WbTask> tasksIn(String workspaceId) => _tasks
      .where(
        (task) =>
            task.workspaceId == workspaceId &&
            task.deviceId == selectedDeviceId,
      )
      .toList();

  WbTask? get openTask {
    final id = openTaskId;
    if (id == null) return null;
    for (final task in _tasks) {
      if (task.id == id) return task;
    }
    return null;
  }

  void toggleDrawer() {
    drawerOpen = !drawerOpen;
    notifyListeners();
  }

  void closeDrawer() {
    if (!drawerOpen) return;
    drawerOpen = false;
    notifyListeners();
  }

  void selectTab(WbTab value) {
    tab = value;
    drawerOpen = false;
    if (value != WbTab.tasks) openTaskId = null;
    notifyListeners();
  }

  void selectDevice(String id) {
    if (!_devices.any((device) => device.id == id)) return;
    selectedDeviceId = id;
    final spaces = workspacesFor(id);
    if (spaces.isEmpty) {
      final created = WbWorkspace(
        id: 'ws.$id.default',
        name: id == kCloudDeviceId ? 'Cloud Workspace' : '任务',
        deviceId: id,
      );
      _workspaces = [..._workspaces, created];
      selectedWorkspaceId = created.id;
    } else if (!spaces.any(
      (workspace) => workspace.id == selectedWorkspaceId,
    )) {
      selectedWorkspaceId = spaces.first.id;
    }
    openTaskId = null;
    notifyListeners();
  }

  void selectWorkspace(String id) {
    final match = _workspaces.where((workspace) => workspace.id == id);
    if (match.isEmpty) return;
    final workspace = match.first;
    selectedDeviceId = workspace.deviceId;
    selectedWorkspaceId = workspace.id;
    final remembered = _lastTaskByWorkspace[id];
    openTaskId =
        remembered != null &&
            _tasks.any(
              (task) => task.id == remembered && task.workspaceId == id,
            )
        ? remembered
        : null;
    notifyListeners();
  }

  void toggleTasks() {
    tasksExpanded = !tasksExpanded;
    notifyListeners();
  }

  void toggleSpaces() {
    spacesExpanded = !spacesExpanded;
    notifyListeners();
  }

  void toggleWorkspace(String id) {
    if (!expandedWorkspaces.add(id)) expandedWorkspaces.remove(id);
    notifyListeners();
  }

  void startNewTask() {
    openTaskId = null;
    tab = WbTab.tasks;
    drawerOpen = false;
    notifyListeners();
  }

  void openTaskById(String id) {
    openTaskId = id;
    tab = WbTab.tasks;
    drawerOpen = false;
    final task = openTask;
    if (task != null) {
      selectedDeviceId = task.deviceId;
      selectedWorkspaceId = task.workspaceId;
      _lastTaskByWorkspace[task.workspaceId] = task.id;
    }
    notifyListeners();
  }

  WbTask send(String raw) {
    final text = raw.trim();
    final id = 'task.local.${++_serial}';
    final title = text.length > 18 ? '${text.substring(0, 18)}…' : text;
    final reply = _replyFor(text);
    final task = WbTask(
      id: id,
      title: title,
      workspaceId: selectedWorkspaceId,
      deviceId: selectedDeviceId,
      status: WbTaskStatus.completed,
      showInTaskList: true,
      messages: [
        WbMessage(fromUser: true, text: text),
        reply,
      ],
    );
    _tasks = [task, ..._tasks];
    openTaskId = id;
    tab = WbTab.tasks;
    notifyListeners();
    return task;
  }

  void continueTask(String raw) {
    final text = raw.trim();
    final current = openTask;
    if (current == null) {
      send(text);
      return;
    }
    final updated = current.copyWith(
      messages: [
        ...current.messages,
        WbMessage(fromUser: true, text: text),
        _replyFor(text),
      ],
    );
    _tasks = [
      for (final task in _tasks)
        if (task.id == current.id) updated else task,
    ];
    notifyListeners();
  }

  WbMessage _replyFor(String text) {
    if (text.toUpperCase().contains('README')) {
      return const WbMessage(fromUser: false, blocks: readmeReply);
    }
    final where = '${selectedDevice.name} / ${selectedWorkspace.name}';
    return WbMessage(
      fromUser: false,
      text: '已在 $where 接收这条任务。文件、模型和执行都留在所选设备上，手机只负责发起和查看结果。',
    );
  }

  void replacePairedCatalog({
    required String deviceId,
    required List<WbWorkspace> workspaces,
    required List<WbTask> sessions,
  }) {
    final previousWorkspaceId = selectedWorkspaceId;
    final wasSelected = selectedDeviceId == deviceId;
    _workspaces = [
      ..._workspaces.where((workspace) => workspace.deviceId != deviceId),
      ...workspaces,
    ];
    _tasks = [
      ..._tasks.where(
        (task) =>
            task.deviceId != deviceId || !task.id.startsWith('dsh.session.'),
      ),
      ...sessions,
    ];
    _lastTaskByWorkspace.removeWhere(
      (workspaceId, taskId) => !_tasks.any(
        (task) => task.id == taskId && task.workspaceId == workspaceId,
      ),
    );
    if (openTaskId != null && !_tasks.any((task) => task.id == openTaskId)) {
      openTaskId = null;
    }
    if (wasSelected && workspaces.isEmpty) {
      if (selectedDeviceId == deviceId) {
        selectedWorkspaceId = _workspaces.isEmpty
            ? previousWorkspaceId
            : _workspaces.first.id;
      }
    } else if (wasSelected) {
      selectedWorkspaceId =
          workspaces.any((workspace) => workspace.id == previousWorkspaceId)
          ? previousWorkspaceId
          : workspaces.first.id;
    }
    for (final workspace in workspaces) {
      expandedWorkspaces.add(workspace.id);
    }
    notifyListeners();
  }

  void replaceCloudWorkspaces(List<WbWorkspace> values) {
    final cloudValues = values.isEmpty
        ? const [
            WbWorkspace(
              id: 'cloud.pending',
              name: '暂无 Cloud Workspace',
              deviceId: kCloudDeviceId,
            ),
          ]
        : values;
    _workspaces = [
      ..._workspaces.where((workspace) => workspace.deviceId != kCloudDeviceId),
      ...cloudValues,
    ];
    if (selectedDeviceId == kCloudDeviceId &&
        !workspacesFor(
          kCloudDeviceId,
        ).any((workspace) => workspace.id == selectedWorkspaceId)) {
      final spaces = workspacesFor(kCloudDeviceId);
      if (spaces.isNotEmpty) selectedWorkspaceId = spaces.first.id;
    }
    notifyListeners();
  }

  void upsertDevice(WbDevice device) {
    final index = _devices.indexWhere((item) => item.id == device.id);
    if (index < 0) {
      _devices = [..._devices, device];
    } else {
      _devices = [
        for (final item in _devices)
          if (item.id == device.id) device else item,
      ];
    }
    notifyListeners();
  }

  void setAccount({required bool signedIn, String? name}) {
    this.signedIn = signedIn;
    accountName = name;
    notifyListeners();
  }
}
