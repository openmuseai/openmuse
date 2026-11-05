# Host Project Workspace ↔ DSH Workspace 绑定与资源联动 PRD

状态：PRD v1（2026-09-20）  
优先端：Windows/macOS Desktop（本地 Mount 全能力）；随后 Web（浏览器授权目录/虚拟 Provider）；Mobile 仅会话、选择与审批  
范围：Host（AppFlowy Flutter/Rust）、DSH（harness + sidecar + 右侧 panel）、Workspace Provider 协议、资源引用与打开链路  
前置：[README.md](./README.md)（四层模型）、[04-domain-model-and-provider-contract.md](./04-domain-model-and-provider-contract.md)、[05-system-architecture-and-dsh.md](./05-system-architecture-and-dsh.md) §3（`DshWorkspaceBindingV1`）、[08-local-p0-implementation-and-acceptance.md](./08-local-p0-implementation-and-acceptance.md) §2.6（现状边界）、[../03-universal-resource-protocol.md](../03-universal-resource-protocol.md)（统一资源协议）、[../../agent-file-references-and-open-routing.md](../../agent-file-references-and-open-routing.md)（打开路由与目标标记的归属设计）  
不冲突声明：本文场景编号 `SC-*`，需求编号 `WBD-*` / `RLO-*` / `RCX-*` / `PBU-*` / `MPT-*`，与 01 的 `S1–S6`、`WSP/EXP/PRO/DSH-W/PLG-W` 不重叠。

## 1. 背景与问题

### 1.1 现状（Local P0 已实现）

Local P0 已经打通"Host 有一个 Project Workspace，DSH panel 的 agent 在其中一个真实目录里工作"：

| 环节 | 现状 | 证据 |
|---|---|---|
| Host 侧定义 | Project Workspace 以 `workspaceRef` 持久化，含多个 Mount | `Application Support/OpenMuse/workspace-platform-v1/definitions/<workspaceRef>.json`，实测含 `frontend`、`helix` 两个本地 Mount |
| Host→DSH 传递 | Flutter 写 best-effort hint 文件，字段 `{appflowyWorkspaceId,title,updatedAt,projectRoot?,mounts[]}`，无协议版本、无 ack | `lib/plugins/dsh_agent/dsh_workspace_bridge.dart` |
| DSH 侧绑定 | `@muse/plugin-appflowy-workspace` 只读取 `projectRoot` 并注册为一个 DSH workspace，pin 防删除 | `middlewares/dsh/plugins/appflowy-workspace/src/identity.ts` |
| 结果 | `mounts.first` = DSH primary cwd；其余 Mount 只留在 hint 的 metadata 里，DSH 完全不可见 | 08 §2.6 原文："当前多根 Workspace 以第一个 Local Mount 作为 DSH primary cwd，其余 Mount 保留在 hint metadata，等待 Provider Broker v2" |

也就是说：**链路存在，但只覆盖了"第一个本地 Mount 的目录"**。这与 05 §3.1 已经定义的 `DshWorkspaceBindingV1`（`workspaceRef` + `mountScopes[]` + `primaryMountRef`）之间还有一代差距，与 07 F5 的退出标准"Host 与 DSH scope 差异为 0"也还不一致。

### 1.2 用户可感知的问题

| 编号 | 问题 | 用户可见表现 |
|---|---|---|
| P-1 | Mount 未全量映射 | 在 Host 里挂了 `frontend` + `helix`，agent 只认得 `frontend`，让它改 `helix` 里的东西它找不到 |
| P-2 | 选中项不联动 | 在 Host 侧栏点中 `helix`，panel 里的会话仍然工作在 `frontend`，用户以为"切换了工作区" |
| P-3 | panel 内不可见 | panel 里没有任何地方显示"agent 现在工作在哪个 Host 工作区/挂载"，新会话落点也不可知 |
| P-4 | 会话归属混乱 | DSH workspace 列表里混有历史遗留目录（改名前的 DSH_HOME、旧桌面目录），新会话可能落到与当前项目无关的目录 |
| P-5 | 资源联动断裂 | DSH 里点产出文件行/行内链接只落到 DSH 自带的 Sidebar 文本预览：打开消息**没有生产端**、Windows 面板**没有 JS 通道**、桌面**没有下发通道**；Host 里复制文本/文件到 DSH 输入框不携带来源信息，模型不知道内容出自哪个文件 |
| P-6 | 多端不一致 | 绑定机制是 Desktop 专用的 hint 文件；Web 走 `workspace.bind`、Mobile 无路径，三端语义不一致，行为无法用同一套验收 |

### 1.3 为什么必须走协议，而不是"把 path 传过去"

用户直觉是对的：Desktop 上 Host 的"导入文件夹"和 DSH 的"选择目录"本质是同一件事。但**不能把 Host 的绝对路径直接交给 DSH 当身份或权限**，原因来自本系列既有决策：

1. README §3 决策 2：不把本地路径、SSH 路径或云盘 object key 当全局身份，统一使用 `workspaceRef/mountRef/entryRef/resourceRef`。
2. 04 §1.2：定义中不允许出现"未经抽象的设备绝对路径"；共享定义里的本地 Mount 用 `bindingKey`，由每台设备解析实际 locator。
3. 05 §3.1：`cwd` 只是某个 execution domain 的**兼容投影**，不是权限真源；工具调用时由 binding + session policy 解析目标 Provider。
4. 05 §7.3：**DSH supplied path 必须先在 session binding 内解析成 resourceRef**。
5. 01 S1 验收：Host、DSH、终端和文件 Tab 对同一资源必须产生同一个 canonical `resourceRef`；文件路径只在 Local Provider 内部出现。

因此本轮的产品要求是：**DSH 侧走一次与 Host 等价的"挂载流程"，但输入是 Mount 身份与 materialization 句柄，而不是 UI 传来的 path**。三端共用同一套协议，只是 Local Provider 恰好能 materialize 成本机路径。

## 2. 产品定义

### 2.1 目标

| 编号 | 目标 |
|---|---|
| G-1 | Host Project Workspace 的每个可用 Mount，在 DSH 侧都有一个与之对应的 DSH Workspace，且**一一对应、可双向追溯** |
| G-2 | Host 侧的"当前选中 Mount"成为 DSH 的 active workspace；切换 Mount 时 panel 自动切到该 workspace 的新会话 |
| G-3 | 绑定关系以协议表达（`workspaceRef/mountRef/resourceRef` + binding + materialization），三端同一套语义，Desktop 只是 Local Provider 的一种 materialization 结果 |
| G-4 | DSH 内点击资源链接，在 Host 对应 Mount/Workspace 的 Tab 中用正确引擎打开，并落到行/锚点 |
| G-5 | Host 内选择文本/文件插入 DSH 输入框时，携带 Host 侧资源元信息（哪个 Mount、哪个文件/页面、哪一段），模型可据此用协议工具读取内容 |
| G-6 | panel 内可感知并切换当前绑定的工作区，用户不再需要猜"agent 在哪个目录" |

### 2.2 非目标（本轮不做）

- 不做 SSH/SFTP/Cloud Mount 的完整 Provider 实现（沿用 07 F7/F8），本轮只保证协议与 UI 不为它们造假能力（"No fake parity"）。
- 不把文件树、文件内容或 100k 文件名持续注入 DSH prompt（05 §3.3 硬约束）。
- 不改 DSH 内核的 session/workspace 数据格式（通过 DSH 既有 workspace registry 与 client 服务接入）。
- 不做 AppFlowy Page 树到 DSH 目录的镜像（Page 仍通过 `muse-collab` Provider 与 catalog 暴露）。
- 不在本轮实现 Ontology Runtime 与 Artifact/Feature 关系。

### 2.3 术语

| 术语 | 含义 |
|---|---|
| Project Workspace | Host 的工作区（`workspaceRef`），含多个 Mount，是任务与权限边界 |
| Mount | Provider 暴露的一个目录根（`mountRef`），Desktop 上即"导入的文件夹" |
| Entry | Provider 树中的可导航位置（`entryRef`，菜单与选择使用） |
| Resource | 内容身份（`resourceRef`，打开与编辑使用） |
| DSH Workspace | DSH 侧的持久工作区记录（目录 + 标题 + session 归属）；agent 的 cwd 与 session 分组单位 |
| Active Mount | Host 当前选中的 Mount，映射为 DSH 的 primary workspace |
| Binding | `muse.dsh/workspace-binding/v1`：session 级的 workspace + mountScopes + grants |

## 3. 对应关系模型（核心）

### 3.1 四层对齐

```text
Host / Provider 层                        DSH 层
Account Space                             （无对应；会话与凭据维度）
  └─ Project Workspace (workspaceRef)  ──►  Binding (muse.dsh/workspace-binding/v1)
       ├─ Mount local (mountRef)       ──►  DSH Workspace #1  (primary，可 materialize 为 host-path)
       ├─ Mount local (mountRef)       ──►  DSH Workspace #2
       └─ Mount cloud (mountRef)       ──►  DSH Workspace #3  (无 exec：只读/浏览投影或不可绑定)
            └─ Entry (entryRef)        ──►  （不进入 DSH workspace 概念）资源通过 resourceRef 访问
```

### 3.2 映射规则

| # | Host 概念 | DSH 概念 | 规则 |
|---|---|---|---|
| M-1 | Project Workspace | Binding | 一个 Project Workspace 对应一条 session binding；binding 记录 `workspaceRef` 与全部 `mountScopes` |
| M-2 | 可用 Mount（可 materialize 成目录） | DSH Workspace | **一一对应**：每个 Mount 一个 DSH workspace；`mountRef` 与 DSH workspace id 双向可查 |
| M-3 | Active Mount | primary DSH Workspace | 新建会话默认落在 primary workspace；`primaryMountRef` 落在 binding 上 |
| M-4 | Mount 顺序 `order` | DSH workspace 顺序 | 按 `order` 排列，primary 置顶 |
| M-5 | Mount `displayName` | DSH workspace 标题 | 标题 = `displayName`；必要时加 Project Workspace 前缀去重（DSH 标题唯一性由 Host 保证） |
| M-6 | Mount `readOnly` | DSH capability 与沙箱 | `read-only` → 不授予写能力，Host 侧写操作必须被 Provider 拒绝，不能只在 UI 隐藏 |
| M-7 | 不可 materialize 的 Mount（无 exec/无路径） | DSH Workspace（只读投影） | **绑定只读投影**（Q-1 决策）：投影目录在 `$DSH_HOME/workspace-projections/<mountRef>/`，写能力显式拒绝、UI 标注"只读投影"；不造假能力 |
| M-8 | Session | DSH Session | session 归属其 cwd 所在的 DSH workspace；历史会话随 workspace 保留，不因 Host 侧解绑而删除 |
| M-9 | Entry / Resource | `resourceRef` | 不作为 DSH workspace 概念；通过 `muse.resource@1` 描述/读取/materialize，供工具与引用使用 |

### 3.3 为什么是"Mount ↔ DSH Workspace"而不是"Project Workspace ↔ DSH Workspace"

DSH 的 workspace 语义是**单一目录 + session 归属**（DSH 侧一条 workspace 记录只有 `path/title/sessionIds`）。一个 Project Workspace 有 N 个根目录，二者数量天然不对等：

- 若强行"1 Project = 1 DSH Workspace"，DSH 只能绑定其中一个目录，其余 Mount 又要回到"不可见"的旧状态，与 G-1 冲突。
- 若把 N 个 Mount 合成一个 DSH workspace，需要 DSH 侧支持多根 cwd（内核不支持，且会破坏 session 归属与沙箱边界）。

所以产品上采用：**Project Workspace 是分组与归属，Mount 是 DSH workspace 的对等单位**。用户在 panel 里看到的是"当前项目工作区下有 2 个 agent 工作区：frontend、helix"，这与 Host Explorer 的结构一致，也与 DSH 原生"按 workspace 分组会话"一致。

Q-6 已确认（2026-09-20）：采用**每 Mount ↔ 一个 DSH Workspace**；Project Workspace 承担归属与分组。每个 DSH workspace 都在 Host 侧映射表（Q-2）里记录其 `workspaceRef` 与 `mountRef`，panel 据此显示"My Workspace · frontend"。不采用 Project 级 1:1 的原因：

| 读法 | DSH workspace 的目录 | 切换活动挂载的语义 | 代价 |
|---|---|---|---|
| **采用**：每 Mount 一个 | 该 Mount 的真实目录 / 只读投影 | 切到该 Mount 的 DSH workspace 并开新会话（SC-3 保留） | 一个 Project 在 panel 里表现为多个工作区（按 Project 归组） |
| 不采用：每 Project 一个 | 投影根（`$DSH_HOME/workspace-projections/<workspaceRef>/`，内部以 junction/symlink 聚合各 Mount） | 不换 workspace，只改默认 scope | agent cwd 与用户看到的真实目录脱钩；与 08 §2.6/§4 的"cwd = canonical 真实目录"验收冲突，需改验收 |

### 3.4 命名、顺序与去重

- 标题默认取 Mount `displayName`（Desktop 上通常是文件夹名）；若同 Project Workspace 内名称冲突，追加 `-2/-3` 或短哈希后缀。
- 若两个 Project Workspace 各有一个同名 Mount，允许同名（DSH 侧标题唯一性只在同一 Project Workspace 的绑定批次内强制，冲突时追加 Project Workspace 标题前缀）。
- 顺序：`primary` 置顶，其余按 Mount `order`。
- 解绑（Mount 被移除、磁盘不可用、Provider offline）时**保留** DSH workspace 与其会话，只标记状态；用户可手动删除。

## 4. 用户场景

### SC-1 首次打开带多个 Mount 的 Project Workspace
前置：Host 有 `My Workspace`，Mount = `frontend`(order 0)、`helix`(order 1)，DSH panel 未使用过。  
流程：用户打开 Project Workspace → Host 发布 binding → DSH 侧为两个 Mount 各注册一个 workspace → panel 显示绑定状态。  
期望：DSH workspace 列表出现 `frontend`、`helix` 两项（`frontend` 为 primary 且置顶），无重复注册、无"只有 README 的空目录"。  
验收：`workspace.json` 中出现两条记录，path 分别等于两个 Mount 的真实根；panel 内可见当前绑定为 `frontend`。

### SC-2 新增 Mount（导入文件夹）
前置：SC-1 状态，panel 已打开并有一个正在进行的会话。  
流程：用户在 Host Explorer 通过 `Add Mount` 导入第三个文件夹。  
期望：**不重启**、不清空当前会话的情况下，DSH 侧新增对应 workspace；当前会话不受影响（cwd 不变）。  
验收：新 workspace 1000 ms 内出现在 DSH 侧；当前会话仍可继续对话；无重复记录。

### SC-3 切换 Active Mount → panel 自动开新会话
流程：用户在 Host Explorer 点选 `helix` 下的任意节点（或点选 Mount 行）→ Host 更新 active mount → panel 切到 `helix` 的 workspace 并开一个空白新会话。  
期望：panel 标题/会话区显示新会话，且该会话的 cwd = `helix` 根；原 `frontend` 会话保留在历史中可回切。  
验收：新会话 cwd 等于 `helix` 根；`primaryMountRef` = `helix` 的 `mountRef`；切换耗时 ≤ 800 ms（本地）。

### SC-4 移除 Mount / 磁盘离线
流程：用户在 Host 移除 `helix` Mount，或 `helix` 磁盘离线后打开工作区。  
期望：DSH 侧对应 workspace 与历史会话**不删除**；标记为"已解绑/离线"，新会话不再默认落在它上面；Host 侧不出现孤儿引用。  
验收：workspace 记录仍在，panel 中该工作区显示不可用状态；重新挂载同一目录后恢复为可写并复用同一 DSH workspace（按 locator 复用，不新建重复项）。

### SC-5 DSH 内点击资源链接 → Host 打开
流程：agent 在回答中给出 `src/main.dart:42` 链接（或 diff/change 链接），用户点击。  
期望：Host 在**该资源所属 Mount 对应的工作区**里打开文件 Tab，定位到第 42 行；若文件属于第二个 Mount，也能正确路由（不出现"在工作区里找不到文件"）。  
验收：打开成功且引擎选择正确（代码 → Helix，文档 → iOffice/Viewer）；同一资源在 Host/DSH 两侧解析为同一个 `resourceRef`。

### SC-6 Host 选中文本插入 DSH 输入框
流程：用户在 Host 中选中一段代码/文档文字（或在 Explorer 中选中一个文件），执行"发送给 Agent / 添加到对话"。  
期望：DSH 输入框出现一个引用 chip，标明来源（文件/页面名 + 可选行范围/章节）；用户补充提问后发送，模型能看到结构化的资源引用（而非只有一段裸文本）。  
验收：消息中携带 `resourceRef` + 锚点 + 来源显示名；模型可用协议工具读取内容；Host 侧不把绝对路径写入模型上下文。

### SC-7 Host 选中文件（多文件）插入
流程：用户在 Explorer 多选文件后插入 DSH 输入框。  
期望：输入框出现多个引用 chip，各自可单独删除；发送后每条引用都可被模型解析。  
验收：chip 数量与选择一致；删除单个 chip 不影响其余；引用解析失败时给出可读错误而不是静默丢弃。

### SC-8 历史会话与工作区归属
流程：用户第二天打开同一 Project Workspace，panel 恢复上次的 workspace 与会话。  
期望：恢复到上次 active Mount 对应的 workspace；历史会话仍在各自 workspace 下；不出现"新会话落到旧桌面目录"。  
验收：DSH 侧无新增无关 workspace；历史遗留 workspace（改名前的 DSH_HOME、与当前 Project 无关的目录）不再被默认选中。

### SC-9 只读 Mount
流程：Project Workspace 含一个 `read-only` Mount，agent 在 panel 中被绑定到它。  
期望：agent 可以读取/列举，任何写操作被拒绝并给出原因；panel 显示只读标识。  
验收：写工具调用返回明确拒绝（不是静默成功）；Host 侧文件 mtime 不变。

### SC-10 多端（Web/Mobile）
流程：用户在 Web 端打开同一 Project Workspace（浏览器授权目录/虚拟 Provider）。  
期望：绑定流程与 Desktop 一致（Mount ↔ DSH Workspace），但 materialization 模式不同；panel 显示"远端执行"或"仅浏览"的准确能力，不出现"可以 exec 但实际不行"的假能力。  
验收：同一套验收脚本在三端跑通，除 materialization 与 exec 能力差异外无行为分叉。

## 5. 功能需求

### 5.1 绑定（WBD）

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| WBD-01 | P0 | Host 发布 binding 文档（含 `workspaceRef`、全部 Mount 的 `mountRef/displayName/order/readOnly/materialization`、`primaryMountRef`、`revision`），携带协议版本 | 文档可被 DSH 侧校验；旧版本可识别并降级 |
| WBD-02 | P0 | DSH 侧为每个可 materialize 的 Mount 建立/复用 DSH workspace，`mountRef ↔ dshWorkspaceId` 双向可查 | 重复发布不产生重复 workspace；重命名 Mount 只改标题 |
| WBD-03 | P0 | Active Mount 变化时更新 primary 并置顶；panel 自动新开该 workspace 的会话 | 见 SC-3 验收 |
| WBD-04 | P0 | 绑定 workspace 必须被保护，不允许 DSH UI 误删 | 删除尝试被拒绝并提示"由 Host 绑定管理" |
| WBD-05 | P0 | Mount 移除/离线、Project Workspace 被删除（Q-3）都不删除 DSH workspace 与会话，只标记解绑 | 见 SC-4 验收 |
| WBD-06 | P0 | 绑定必须可重放与可恢复：panel 启动时按当前 binding 恢复（进程重启、DSH sidecar 重启后一致） | 重启后 workspace 集合与 primary 与重启前一致 |
| WBD-07 | P0 | 绑定失败（路径不存在、权限不足、Provider offline）必须显式上报并可见，不得静默跳过 | panel 与 Host 侧均有可读状态 |
| WBD-08 | P1 | 支持策略：`trust`/`readOnly`/忽略规则（`ignoreProfileRefs`）随 binding 下传并生效 | 只读 Mount 写操作被拒（SC-9） |
| WBD-09 | P1 | 提供 binding 变更事件（新增/移除/切换/重命名/离线），panel 增量更新无需刷新 | 变更 1 s 内反映到 panel |
| WBD-10 | P1 | 旧 hint 文件与 Web `workspace.bind` 语义收敛到同一 binding 文档（transport adapter 不携带业务语义） | 三端同一 schema、同一验收脚本 |

### 5.2 DSH → Host 打开（RLO）

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| RLO-01 | P1 | DSH 内资源链接以 `resourceRef` + 锚点（行/范围/章节）表达，不再以裸路径为身份；生产端按打开路由设计稿的通路 A（宿主 opener 覆盖 → `target.open`）落地 | 消息中不含设备绝对路径 |
| RLO-02 | P1 | Host 依据 binding 判定资源所属 Mount/工作区，路由到对应 Tab 与引擎 | 第二个 Mount 的文件也能打开（SC-5）。**DSH 侧已落地**：`resolveWithinBinding` 按 Mount 根做包含判定（最长根优先），实测 helix 与 frontend 两个 Mount 的文件分别解析到各自 Mount；Tab 与引擎选择仍在 Host 半部（随 Flutter 重编） |
| RLO-03 | P1 | 路径兼容适配器：agent 产出的工作区内相对路径先在 binding 内解析为 `resourceRef`，解析失败给可读错误 | 伪造/越界路径被拒绝并审计。**DSH 侧已落地**：相对路径按 active Mount 解析；越界 403 `OPEN_DENIED(MOUNT_NOT_IN_BINDING)` 且不回落任意绝对路径；命中时审计 `resolve.legacy`（含 bindingRevision 与 Mount 名） |
| RLO-04 | P1 | 打开动作产生 terminal receipt 并记录到会话 | 打开失败不显示成功。**已落地（DSH 侧）**：`POST /muse/v1/target.open` 对每个 `intentRef` 恰好返回一个 `muse.presentation-intent-result/v2` 终态（`opened/focused/fallback/unsupported/rejected/timed-out/failed`），拒绝与失败都带 `reasonCode`，无回执路径不返回成功 |
| RLO-05 | P2 | diff/change 链接（comparisonId/changeId）走同一链路进入 Version/Diff 视图 | 现有 diff 深链能力不回退 |
| RLO-06 | P1 | 补齐链路前置：打开生产端、Windows 面板 JS 通道、桌面 host→client 下发（不复活 parent-bridge HTTP） | Windows 上点击第二个 Mount 的文件链接可打开并定位。**DSH 侧落地**：面板入网端点 `/muse/v1/target.open` 直接调用 presentation 消费者（实测已进入 Host Bridge 分发并得到失败回执，未静默）；Host 承载器与 WebView2 JS 通道待 Flutter 半部 |
| RLO-07 | P1 | 打开/拒绝/超时必须返回回执（`IntentReceipt` 语义），失败不得静默 | 失败时用户与日志都能看到原因。**已落地（DSH 侧）**：5 s 分发超时 → `timed-out/HOST_TIMEOUT`；服务缺失 → `unsupported/HOST_UNAVAILABLE`；无会话作用域 → `unsupported/NO_SESSION_SCOPE` |

### 5.3 Host → DSH 引用与元信息（RCX）

**承载通道约束（实现前必读）**：Host→DSH 的引用必须走既有 `context.contribute` / `MuseContextContributionV1` 通道；现有 `muse.native-capability/v1` 通道按构造禁止 `path/contentUri/base64/intent` 键，**不得为传递来源而放宽该禁列**。payload 字段集对齐 `ResourceDescriptorV1` 与 `present` 工具 schema（只带 `resourceRef`，不带设备路径）。该功能当前零实现（无消息类型、无生产端、无消费端）。

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| RCX-01 | P1 | Host 选中的文本/文件以引用形式插入 DSH 输入框，携带 `resourceRef`、来源显示名、锚点（行范围/章节/页）、所属 `mountRef` | 见 SC-6 验收。**未落地**：DSH 侧尚无 `resource.reference` contribution 类型、`muse/resource-reference` 会话事件与 composer chip；Host 侧贡献仍只有 `appflowy.workspace`/markdown/word 三种上下文 |
| RCX-02 | P1 | 引用以结构化块随消息发送，模型可见"来源 + 可用能力"，而不是未标注的裸文本 | 消息可被 session log 重建（模型可见 ⟺ 可记录）。**未落地**（依赖 RCX-01） |
| RCX-03 | P1 | 引用内容按需读取（协议工具/materialize）；**允许小文本内联**（`excerpt` ≤ 8 KiB，Host 侧截断并标 `truncated`，Q-4），超限只发 descriptor | 大文件不导致请求体积爆炸。**未落地**：无 `excerpt`/8 KiB 预算实现（仅文档/目录快照已有 `truncated` 机制） |
| RCX-04 | P1 | 引用 chip 可删除、可多个、失败可见 | 见 SC-7 验收。**未落地**（chip 属 RCX-01 的客户端插件） |
| RCX-05 | P2 | 引用携带 revision，内容变更后可在 panel 内提示"来源已更新" | 变更提示可复现 |
| RCX-06 | P2 | Explorer 提供 `Ask Agent about this file/folder`、`Start Agent in this folder`（folder 成为 primary root） | 与 05 §3.5 的五个动作对齐 |

### 5.4 Panel 工作区呈现（PBU）

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| PBU-01 | P1 | panel 内显示当前绑定：Project Workspace 标题 + active Mount 名称 + 状态（ready/只读/离线） | 用户无需猜测 agent 工作目录。**已落地**：`GET /muse/v1/workspace/binding`（无路径快照）+ `/binding/events`（SSE）+ 客户端插件 `@muse/dsh-client-ui-workspace-binding` 的 header chip（`标题 · active Mount`，只读态带 `只读` 角标，SSE 事件驱动刷新） |
| PBU-02 | P1 | panel 内提供切换器，列出该 Project Workspace 的全部 Mount（含状态），点击即切换并新开会话 | 切换等价于 Host 侧点选（同一 binding 更新）。**已落地**：chip 展开切换器（`当前 · 可写` / `只读` / `不可用（code）`，failed Mount 行禁用），点行 → `POST /muse/v1/workspace/active` → DSH 重排 + `workspace-active-intent.json`（Host 下次发布采纳）→ 快照刷新后 `uiWorkspace.startSession(dshWorkspaceId)` 复用或新建该 Mount 工作区的会话 |
| PBU-03 | P2 | 新建会话时的默认落点与该 Project Workspace 的 active Mount 一致（hero 选择器不出现无关历史工作区） | SC-8 验收 |
| PBU-04 | P2 | 历史遗留工作区（非本 Project 绑定、路径已失效）不在默认选择面出现，可在"更多"里管理 | 列表干净且不丢历史会话 |

### 5.5 多端（MPT）

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| MPT-01 | P1 | 绑定文档与资源引用 schema 不含设备路径；每端由自己的 Provider 解析 locator | schema 校验拒绝路径字段 |
| MPT-02 | P1 | materialization 模式显式声明（`host-path` / `semantic-snapshot` / `loopback-url` / 只读投影），UI 据此显示能力 | 无 exec 环境不显示终端能力 |
| MPT-03 | P2 | Mobile 仅会话/选择/审批：能看到绑定与引用，不要求本地挂载 | 无路径环境下不报错 |

## 6. UX 规格

### 6.1 "把 Project Workspace 暴露进 panel UI"具体指什么

这是对上一轮问题 3 的回答。DSH panel 内部目前只有 DSH 自己的界面：一个新会话的 hero 选择器、以及宽度足够时才出现的侧栏（按 workspace 分组会话）。**Host 的 Project Workspace、Mount 名称、绑定状态在 panel 里完全不可见**，用户只能从 Host 侧栏"推断"agent 在哪工作。因此 PBU 要求在 panel 内增加两个出口：

1. **状态 chip（PBU-01）**：常驻显示一行"📁 My Workspace · frontend（只读）"，点击展开详情（Mount 列表、path 的**显示名**、能力、绑定状态）。回答"agent 现在在哪个目录工作"。
2. **切换器（PBU-02）**：chip 展开后列出该 Project Workspace 的全部 Mount，点击某项 = 等价于在 Host Explorer 里点选该 Mount：更新 active mount → 切到对应 DSH workspace → 自动新开会话。

两者都不做也"能用"（纯后台绑定 + Host 侧切换），但用户会持续遇到 P-3/P-4：不知道落点、不知道历史会话归属。建议至少做 PBU-01。

放置位置（复用 DSH 既有 slot，不新造 UI 框架）：

| 位置 | 内容 | 说明 |
|---|---|---|
| 会话头部/composer 附带区 | 绑定 chip | 始终可见，带只读/离线状态点 |
| chip 展开面板 | Mount 切换列表 | 每项显示 displayName + provider badge + 状态 |
| 新会话 hero | 默认 workspace = active Mount | 不出现与当前 Project 无关的历史工作区 |

### 6.2 Host Explorer 侧

- Mount 行显示"Agent 已绑定 / 未绑定 / 只读"标记，与 panel 一致。
- 点选 Mount 或任意 Entry 都可能改变 active Mount；改变时给出不打扰的反馈（"Agent 已切换到 helix"）。
- 右键菜单新增 `Ask Agent about this file/folder`、`Start Agent in this folder`、`Add to current Agent context`（对齐 05 §3.5）。

### 6.3 状态与错误文案（示例）

| 情况 | 文案方向 |
|---|---|
| 未绑定 | "此挂载未接入 Agent"（并说明原因：Provider 不支持或无执行环境） |
| 只读 | "Agent 可读取，不能修改" |
| 离线/已解绑 | "挂载不可用，历史会话仍可查看" |
| 绑定失败 | "无法接入 <挂载名>：<原因>"，提供重试与查看日志入口 |

## 7. 分端能力矩阵

| 能力 | Desktop（本地 Mount） | Web（浏览器授权目录/虚拟 Provider） | Mobile |
|---|---|---|---|
| Mount ↔ DSH Workspace 一一对应 | 完整 | 完整（虚拟/授权目录） | 不适用（无本地挂载） |
| materialization | `host-path` | `semantic-snapshot` / `loopback-url` | 无 |
| exec / 终端 | 有 | 远端执行（有远端 Agent 时） | 无 |
| DSH→Host 打开 | 完整（Helix/iOffice/Viewer） | 已支持引擎的只读/预览 | 系统预览 |
| Host→DSH 引用 | 完整 | 完整 | 选择文本 → 引用 |
| 切换 active Mount | 完整 | 完整 | 只读展示 |

## 8. 权限、信任与安全

- **权限真源**：binding + session policy；`cwd` 仅作兼容投影。工具调用时按 Mount 对应 Provider 重新验证 actor/mount/policy，不缓存权限决定（05 §7.1）。
- **路径不外泄**：模型上下文、session log、审计日志不得出现设备绝对路径、credential、短期 bearer handle；只记录 `workspaceRef/mountRef/entryRef/resourceRef/requestRef`。
- **信任**：新增 Mount 单独评估信任；`restricted` 工作区不得静默把未信任根并入。
- **只读**：`read-only` Mount 的写能力在 Provider 与 policy 两层拒绝，UI 隐藏只是提示。
- **审计**：绑定/解绑/切换/打开/引用写入审计流（operation、effect、resultCode、generation、latencyMs）。

## 9. 指标

| 指标 | 目标 |
|---|---|
| Mount→DSH Workspace 映射准确率 | 100%（可 materialize 的 Mount 全部映射，无重复、无遗漏） |
| 新增/移除 Mount 到 panel 可见延迟 | P95 ≤ 1 s |
| Active Mount 切换到新会话就绪 | P95 ≤ 800 ms（本地） |
| 资源打开成功率（DSH 链接点击） | ≥ 99%（工作区内存在的文件） |
| 引用插入成功率 | ≥ 99%，失败必须可见 |
| "找不到文件/路径"类失败（多根场景） | 降为 0 |
| 误落到无关工作区的新会话 | 0 |

## 10. 验收与发布门槛

沿用 08 的验收矩阵风格，新增行并纳入同一 suite：

| 主题 | 自动化 | 构建 | 手工 UI |
|---|---|---|---|
| Mount ↔ DSH Workspace 一一对应 | 契约测试 + DSH 侧单测 | 通过 | 挂 2 个目录后 panel 可见两项 |
| Active Mount 切换 | Host 单测 + E2E | 通过 | 点选 helix → panel 新会话 cwd = helix |
| 解绑/离线保留会话 | E2E | 通过 | 移除挂载后历史会话仍在 |
| DSH→Host 打开（多根） | E2E | 通过 | 点击第二个 Mount 的文件链接可打开并定位 |
| Host→DSH 引用与元信息 | 单测 + E2E | 通过 | 插入文本/文件后 chip 与来源正确 |
| 只读 Mount 拒绝写 | 单测（拒绝路径） | 通过 | 写操作给出明确拒绝 |
| 多端 schema 一致性 | schema/golden fixtures | 通过 | Web 端行为与 Desktop 对齐 |

发布门槛：

1. WBD-01..07 全部通过，且 07 F5 的"Host 与 DSH scope 差异为 0"验收项有一份可复现记录。
2. 旧 hint 与 Web `workspace.bind` 的兼容路径有测试证据；升级不产生重复 workspace。
3. 本文档不宣称未实现的 Provider（SSH/Cloud）能力。
4. 只读投影（Q-1）的写拒绝有单测证据，投影目录可从 Host 重建。
5. 升级时的一次性遗留清理（Q-5）在真实旧数据上演练过，并有审计与回滚记录。

## 11. 决策记录与开放问题

已确认（2026-09-20）：

| # | 决策 | 落点 |
|---|---|---|
| Q-1 | 不可 materialize 的 Mount（如纯 Cloud）**绑定只读投影**，不采用"不绑定" | M-7、SC-10、PBU-01 |
| Q-2 | DSH workspace 与 `mountRef` 的映射表放 **Host 侧**（与 definition 同源、可审计、多端可读；DSH 侧只做缓存与兜底） | 设计文档 §4.5 |
| Q-3 | Host 侧删除 Project Workspace 时，DSH 侧 workspace 与会话**保留**，仅标记解绑 | WBD-05、SC-4 |
| Q-4 | 引用**允许小文本内联**（`excerpt` ≤ 8 KiB，Host 侧截断并标记） | RCX-03 |
| Q-5 | 历史遗留 workspace 在升级时**一次性清理**（白名单 + 审计 + 可回滚） | 发布门槛 5、设计文档 §5.2 |
| Q-6 | 映射基数采用**每 Mount ↔ 一个 DSH Workspace**；Project Workspace 负责归属与分组，映射表记录 `workspaceRef`/`mountRef` | §3.2、§3.3 |

无待确认项。
