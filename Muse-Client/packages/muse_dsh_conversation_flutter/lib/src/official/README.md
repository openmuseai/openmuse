# 官方 DSH 对话组件

这些 Widget 只复刻已经钉住的官方插件样式，不执行插件里的 React。升级 DSH 时先对照下表的包版本和事件字段，再改对应目录。产品闭包钉在 `@deepseek-ai/dsh@0.1.7-rc.1`，见 `Muse-Client/third_party/dsh/package-lock.json`。

可读的 TypeScript 在 `vendors/deepseek-harness`，那棵树的插件版本是 `0.1.5-rc.2`。事件字段和中文文案以那份源码加上 `0.1.7-rc.1` 产品截图核对；闭包升级后不要只改版本号。

| 目录 | 官方包 | 钉住版本 | 事件 | 官方落点 |
| --- | --- | --- | --- | --- |
| `deliverables/` | `@deepseek-ai/dsh-client-ui-deliverables` | `0.1.7-rc.1` | `deliverables/presented`；工具名 `present` | `PresentedFileCard`、`Deliverables`、`PresentRow` |
| `todo/` | `@deepseek-ai/dsh-client-ui-conversation` | `0.1.7-rc.1` | `todo/write` | `TodoPanel`。工具行标题还参考 `@deepseek-ai/dsh-client-ui-tool@0.1.7-rc.1` 的 `todo_write` toolview |
| `goal/` | `@deepseek-ai/dsh-client-ui-goal` | `0.1.7-rc.1` | `goal/change` | `GoalBar` |

## 交互边界

交付物卡片的主按钮和菜单调用 Native Gateway `POST /openmuse-native/v1/session/presented/open`。服务端按 `sessionId + seq + index` 回读事件里的路径，再在 Desktop 上打开或显示所在文件夹。Mobile 不接收主机路径。官方主按钮是侧边栏预览；Mobile 没有那块侧边栏，所以主按钮对应官方菜单里的「用默认应用打开」。

超过 4 个文件时默认只显示前 4 个，和官方 `COLLAPSED_PRESENTED_COUNT` 一致。同一回合里后一次声明覆盖同一路径，卡片挂在该回合最后一条助手消息下面，而不是挂在事件自己的序号上。

任务面板默认收起，展开后列出整份快照。空列表、重复内容和未知状态不显示。

目标条只显示阶段和目标正文。完成和清除在官方界面也不占条。暂停、编辑、清除会改 Desktop 上的 goal revision，Native Gateway 还没有对应 command，因此这里不放会假装生效的按钮。

## 升级时要重看的字段

- `deliverables/presented.data`: `turn`、`callId`、`files[].path`、`files[].description`
- `todo/write.data.todos[]`: `content`、`status = pending | in_progress | completed`
- `goal/change.data`: `operation`、`goal.objective`、`goal.phase`、`goal.blockedReason.message`
