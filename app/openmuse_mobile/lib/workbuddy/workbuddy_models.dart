import 'package:flutter/foundation.dart';

enum WbDeviceKind { local, cloud }

enum WbTab { tasks, experts, library, schedule, projects }

enum WbTaskStatus { running, completed }

@immutable
final class WbDevice {
  const WbDevice({
    required this.id,
    required this.name,
    required this.kind,
    this.online = true,
  });

  final String id;
  final String name;
  final WbDeviceKind kind;
  final bool online;
}

@immutable
final class WbBlock {
  const WbBlock(this.text, {this.heading = false, this.file = false});

  final String text;
  final bool heading;
  final bool file;
}

@immutable
final class WbMessage {
  const WbMessage({
    required this.fromUser,
    this.text = '',
    this.blocks = const [],
  });

  final bool fromUser;
  final String text;
  final List<WbBlock> blocks;
}

@immutable
final class WbTask {
  const WbTask({
    required this.id,
    required this.title,
    required this.workspaceId,
    required this.deviceId,
    required this.status,
    required this.messages,
    this.showInTaskList = false,
  });

  final String id;
  final String title;
  final String workspaceId;
  final String deviceId;
  final WbTaskStatus status;
  final List<WbMessage> messages;
  final bool showInTaskList;

  WbTask copyWith({
    String? title,
    WbTaskStatus? status,
    List<WbMessage>? messages,
    bool? showInTaskList,
  }) => WbTask(
    id: id,
    title: title ?? this.title,
    workspaceId: workspaceId,
    deviceId: deviceId,
    status: status ?? this.status,
    messages: messages ?? this.messages,
    showInTaskList: showInTaskList ?? this.showInTaskList,
  );
}

@immutable
final class WbWorkspace {
  const WbWorkspace({
    required this.id,
    required this.name,
    required this.deviceId,
  });

  final String id;
  final String name;
  final String deviceId;
}

const kLocalDeviceId = 'local.desktop';
const kPendingDeviceId = 'paired.pending';
const kCloudDeviceId = 'cloud';
const kTaskWorkspaceId = 'ws.tasks';
const kOpenMuseWorkspaceId = 'ws.openmuse-io';

const kLocalDeviceName = 'DESKTOP-FBRL8RL';
const readmeReply = <WbBlock>[
  WbBlock(
    '两个仓库各有一份 README，内容基本一致，openmuse/ 是更新更全的版本，muse-clients/ 是更早一版。内容如下：',
  ),
  WbBlock('openmuse/README.md', file: true),
  WbBlock('OpenMuse local desktop', heading: true),
  WbBlock(
    'This repository contains the clean-room OpenMuse desktop Host, plugin SDK, platform broker and first-party plugins. It contains no web product and does not inherit a legacy workspace format.',
  ),
  WbBlock('当前可执行部分包括桌面 Host、插件运行时，以及手机端对同一台电脑上 Workspace 与任务的远程控制。'),
];
