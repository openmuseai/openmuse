# 同账号多端协同产品稿与交互稿

> 状态：V1 同账号直连已实现并完成真机关键路径验收
> 日期：2026-09-30
> 参考：用户提供的 6 张 WorkBuddy 手机端截图，仅用于交互分析，不作为代码或业务指令。

## 1. 产品结论

OpenMuse Mobile 是 Desktop Agent 的远程控制台，不是桌面 UI 的缩小版。用户登录同一账号后，
在线 Desktop 已属于账号信任域，**不再输入配对码**。用户选择在线电脑即可查看该电脑公开给
多端协同的 Workspace、全部 DSH 对话及状态，并可在选定 Workspace 中创建任务。

同账号认证仍不等于无限权限：Cloud 为每次访问签发或协调短期、可撤销、绑定
`account + requesterDevice + targetDevice + workspace + expiresAt` 的访问凭证。未来跨账号共享
使用单独的邀请/配对关系，不复用同账号直连入口。

## 2. 截图交互拆解

1. **设备级导航**：侧栏顶部是当前电脑、在线绿点和设备切换入口；任务与空间都属于当前电脑。
2. **任务级状态**：列表同时包含已完成、正在运行、等待输入的对话；状态变化原位更新。
3. **跨端同一对话**：电脑发起与手机发起的任务进入同一列表，详情页展示相同历史和运行步骤。
4. **手机续聊/停止**：运行中的任务仍可发送补充指令，发送按钮切换为停止动作。
5. **选择执行空间**：新任务先继承当前 Workspace，也可通过底部 Sheet 切换电脑上的 Workspace。
6. **职责边界**：文件、模型、Skill、Shell 和 DSH 均在目标 Desktop 执行；Mobile 只发送意图并呈现状态。

## 3. 信息架构

```text
账号
├─ Cloud Workspace
└─ 我的设备
   ├─ MacBook Pro · online
   │  ├─ 任务
   │  │  ├─ running / waiting_input
   │  │  └─ completed / failed / stopped
   │  └─ Workspace
   │     ├─ openmuse-io
   │     └─ Office
   └─ Office PC · offline（可见但不可进入）
```

## 4. Mobile 主流程

### 4.1 首次进入

1. GoTrue 登录。
2. 展示账号下设备快照；在线 Desktop 带绿点，离线 Desktop 保留最后在线时间。
3. 点击在线 Desktop 直接申请同账号访问凭证并进入设备工作台；不出现验证码。
4. 点击离线 Desktop 显示原因与“等待设备上线”，不创建无效连接。

### 4.2 设备工作台

- 顶栏：设备图标、设备名、在线状态、切换设备。
- 首要动作：“新建任务”。
- “任务”区域：所有会话，状态由目标 Desktop DSH 实时投影。
- “空间”区域：Desktop Host 授权的 Workspace，而不是扫描整台电脑。
- 断线：保留最后快照，展示“正在重连”；禁止发送，不把列表瞬间清空。

### 4.3 新建任务

1. 点击“新建任务”。
2. 默认选择当前 Workspace；点击 Workspace 打开底部选择 Sheet。
3. 输入指令并提交；Mobile 生成幂等 `requestId`。
4. Desktop DSH 接受后，任务立即出现在 Mobile 与 Desktop 列表。
5. 接收 `session.created/status/event`，显示思考、工具步骤、等待输入和最终结果。

### 4.4 续聊和停止

- 进入运行/完成任务均显示完整历史。
- 对运行中任务发送新消息时，DSH 决定进入 steering 或 queue，Mobile 不自行猜测。
- “停止”必须命中同一 `sessionRef + generation`，避免停止已重启的新任务。

## 5. Desktop 交互

“设置 → 账号与设备 → 多端协同”展示：

- 当前登录账号与本机稳定设备 ID；
- Cloud Presence 与实时通道状态：在线、重连中、离线；
- “允许同账号移动端访问”开关（V1 默认开启，后续可持久化关闭）；
- 当前账号其他设备；
- 不再显示或生成同账号配对码。

Desktop 退出登录、撤销设备或关闭多端协同时，已有访问凭证立即/尽快失效。

## 6. 状态与文案

| 状态 | UI | 可进入 | 可发消息 |
|---|---|---:|---:|
| online | 绿点“在线” | 是 | 是 |
| reconnecting | 黄点“正在重连” | 使用已有快照 | 否 |
| stale | 灰点“连接不稳定” | 只读 | 否 |
| offline | 空心灰点“离线” | 否 | 否 |
| revoked | 从默认列表移除 | 否 | 否 |

## 7. V1 实现边界

本次落地同账号免码访问、鲁棒 Presence、设备实时事件通道、WorkBuddy 风格设备工作台入口，
以及真实 Desktop DSH Web 客户端中的会话/消息/运行状态同步。原生任务列表的独立 DSH
Projection、跨公网 opaque relay、Push 唤醒和跨账号关系属于后续迭代；这些能力已有协议槽位，
但不能用 loopback/ADB 验收冒充生产公网能力。

## 8. 真机交互验收记录

| 用例 | 操作 | 可观察结果 | 结果 |
|---|---|---|---|
| 同账号发现 | Mac 与 Android 登录同一 GoTrue 账号 | 手机显示 Mac 在线绿点，无配对码 | PASS |
| 设备工作台 | 点击在线 Mac | 展示“新建任务”“全部对话”和 Project Workspace | PASS |
| 数据面故障隔离 | Mac Gateway 端口预先被占用 | Mac 仍上线；入口明确显示服务恢复状态 | PASS |
| 自动恢复 | 释放 Gateway 端口，不重启 App | Host 自动 bind 并更新 Workspace origin | PASS |
| 完成任务 | 手机向 Mac DSH 发送确定性 marker | 同一会话显示用户消息及 assistant 响应 | PASS |
| 运行中任务 | 模型响应延迟 45 秒 | 详情显示“深度求索中”和停止按钮 | PASS |
| 运行中重进 | 任务执行时返回工作台再进入“全部对话” | 仍显示同一运行中会话，而非创建新会话 | PASS |
| 完成收敛 | 等待延迟响应结束 | 原会话原位变为完成态并保留完整历史 | PASS |

验收截图保存在构建目录 `target/mobile-dsh-*.png`，不作为产品运行时依赖。macOS 验收包包含
完整 DSH、Node runtime 和插件 closure；Android 端只承担远程控制台职责，没有内嵌 Node/DSH。
