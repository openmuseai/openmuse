# Host Project Workspace ↔ DSH Workspace 绑定：开发计划与测试矩阵

状态：P0 已实施；P1-A（面板数据面）、P1-B（panel 客户端插件、RLO 生产端/解析与回执、watcher 去重、只读 Mount 写保护）已实施，测试矩阵 33 项，端到端验证见 §7
范围：09（产品文档）T6 决策落地、10（方案设计）§4–§5 与 §10 P0 的工程化拆解
前置：`09-dsh-binding-product-prd.md`、`10-dsh-binding-architecture-design.md`、`08-workspace-platform-roadmap.md` §2.6、`05-resource-protocol-and-tools.md` §3.1

## 1. 完成定义（DoD）

P0 视为完成，当且仅当：

1. DSH 侧按 `muse.workspace/binding/v1` 文档逐 Mount 绑定：每个 Mount 一个 DSH workspace，active Mount 置顶，其余按 `order`，重复标题可区分。
2. 路径只经 `bindings/materialized/<mountRef>.path` 过界；binding 文档本身不含任何设备路径（测试守护）。
3. 不可 materialize 的 Mount（cloud/sftp/unknown）绑定为只读投影目录并标注，不造假能力（Q-1）。
4. 映射表（`mountRef ↔ dshWorkspaceId`）落在 Host 侧，来自 DSH receipt，并作为下次发布的复用提示（Q-2、Q-6）。
5. Host 删除 Project Workspace 不删除 DSH workspace/会话（Q-3）；升级时一次性清理旧 DSH home 遗留记录（Q-5）。
6. 引用允许小文本内联，上限 8 KiB（Q-4，P1 落地）。
7. 系统提示词按绑定分叉：真实目录 Mount 与托管 scratch 目录给模型不同事实。
8. 测试矩阵 §4 全部 T 项有可复现证据；本机端到端跑通（§5）。

## 2. 工作分解（P0）

| ID | 任务 | 产物 | 状态 |
|---|---|---|---|
| P0-1 | binding 协议与校验 | `plugins/appflowy-workspace/src/binding.ts` | 完成 |
| P0-2 | materialization（host-path / 只读投影） | `src/materialize.ts` | 完成 |
| P0-3 | receipt 回执 | `src/receipt.ts` | 完成 |
| P0-4 | 遗留清理 | `src/prune.ts` | 完成 |
| P0-5 | 多 Mount 绑定、排序、prompt 分叉 | `src/identity.ts`、`src/prompt.ts`、`src/dsh.ts` | 完成 |
| P0-6 | Host 发布器（binding v1 + locator + 映射表 + 原子写 + tmp 清扫） | `lib/plugins/dsh_agent/dsh_workspace_bridge.dart` | 完成 |
| P0-7 | active Mount 持久化与联动发布 | `lib/workspace_platform/{domain,infrastructure,application}` | 完成 |
| P0-8 | 测试矩阵与端到端验证 | 本文件、`tests/*`、`test/workspace_platform/*` | 完成 |

### 2.1 P1 联动切片 A：面板数据面（本次完成）

| ID | 任务 | 产物 | 状态 |
|---|---|---|---|
| P1-1 | 绑定快照、事件总线与切挂载语义（DSH 侧） | `plugins/appflowy-workspace/src/panel-api.ts`、`src/events.ts` | 完成 |
| P1-2 | 面板入口：桌面与 Web 共用一条实现 | `plugins/dsh-appflowy/src/panel-routes.ts`、`src/webview.ts` | 完成 |
| P1-3 | 切挂载回传与 Host 采纳 | `POST /muse/v1/workspace/active` + `workspace-active-intent.json` + `dsh_workspace_bridge.dart` | 完成 |
| P1-4 | 测试与端到端（§7.5） | `tests/panel-api.test.ts`、`dsh_binding_bridge_test.dart` | 完成 |

仍属 P1 未做：RCX 引用往返（`resource.reference` contribution → `muse/resource-reference` 会话事件 → composer chip，归属 §5.3 客户端插件与 `agent-file-references-and-open-routing.md`）、Host 侧打开承载器（`target.open` 的 Host 半部，随 Flutter 重编落地）。

## 3. 实施顺序与理由

| 顺序 | 步骤 | 理由 |
|---|---|---|
| 1 | 先 DSH 侧（P0-1..P0-5） | 文档是契约，先让消费端能解析、能在 fake registry 上验证，再让 Host 生产 |
| 2 | 再 Host 侧（P0-6、P0-7） | Host 的 golden 与 DSH 的解析用同一组向量（§4 T-05），避免两边各自解释 |
| 3 | 最后物化 + 重启 + 端到端（P0-8） | 运行中的 app 加载的是 profile 副本，必须重建并重启 sidecar 才能验证 |

## 4. 测试矩阵

层级：**U** 单元、**C** 契约（跨端 golden）、**E** 端到端、**F** 失败注入、**R** 回归。
命令中的 `<pkg>` = `openmuse/middlewares/dsh/plugins/appflowy-workspace`，`<app>` = `openmuse/frontend/client/frontend/appflowy_flutter`。

| ID | 层 | 被测对象 | 触发 / 输入 | 期望 | 证据 |
|---|---|---|---|---|---|
| T-01 | U | `parseBinding` 合法文档 | 两个 local Mount、`activeMountRef` 指向第二个 | 解析成功；Mount 按 `order` 升序；local 默认 `host-path` | `tests/binding.test.ts` ✓ |
| T-02 | U | INV-1 无设备路径 | Mount 带 `root` 键 | 整份文档被拒；错误含 "device paths are not allowed" | `tests/binding.test.ts` ✓ |
| T-03 | U | 其它非法形态 | 未知字段 / 重复 `mountRef` / 未声明的 `activeMountRef` / 协议不符 / 非法 JSON | 全部拒绝且逐条报错 | `tests/binding.test.ts` ✓ |
| T-04 | U | 默认推导 | cloud 文档省略 `providerKind`/`materialization` | `cloud` + `read-only-projection` | `tests/binding.test.ts` ✓ |
| T-05 | C | locator 文件名向量 | `mount:abc`、`mount/with/slash`、`中文 ref`、空串、`..dash-dot_ok`、200×`a` | 两边同结果：`mount_abc`/`mount_with_slash`/`ref`/`mount`/`..dash-dot_ok`/128 字符 | TS `tests/binding.test.ts` ✓ + Dart `test/workspace_platform/dsh_binding_bridge_test.dart` |
| T-06 | F | `materializeMount`(host-path) | 无 locator / 相对路径 / 目录不存在 / 非目录 | `MOUNT_LOCATOR_MISSING` / `NOT_ABSOLUTE` / `PATH_MISSING` / `NOT_DIRECTORY`；不创建目录 | `tests/binding.test.ts` ✓ |
| T-07 | U | `materializeMount`(投影) | cloud Mount | 生成 DSH 托管目录 + README；`managedDirectory=true`、`readOnly=true` | `tests/binding.test.ts` ✓ |
| T-08 | E | 多 Mount 应用 | 2 local Mount，active=helix，registry 预置 1 条旧 DSH home 记录 + 1 条用户记录 | helix、frontend 各得一个 workspace；顺序 [helix, frontend, 用户]；receipt 全 `bound`；旧记录被删；用户记录保留 | `tests/binding.test.ts` ✓ |
| T-09 | U | 幂等 | 同一 revision 连续应用两次 | 第二次返回同一对象；不重复 create；workspace 数不变 | `tests/binding.test.ts` ✓ |
| T-10 | F | 部分失败 | helix 缺 locator | frontend 仍绑定；`failures` 记录 helix；receipt 标 `failed` + code | `tests/binding.test.ts` ✓ |
| T-11 | U | 标题去重 | 两个 Mount 同名 `src` | 第二个标题为 `My Workspace · src` | `tests/binding.test.ts` ✓ |
| T-12 | F | 非法文档不部分应用 | 文档含 `path` | 抛 `BindingDocumentError`；不创建任何 workspace；receipt 带 `documentErrors`、`bindingRevision=-1` | `tests/binding.test.ts` ✓ |
| T-13 | U | 遗留清理 | 旧 DSH home 记录（无/有会话）、当前 root、keepPaths、用户目录；删除被拒 | 无会话旧记录删除；当前 root/keepPaths/用户目录不动；有会话只上报不删（`force` 才删）；被拒时容忍并记录 | `tests/prune.test.ts` ✓ |
| T-14 | U | prompt 分叉 | 绑定含 host-path / 仅托管目录 / 未绑定 | 分别返回项目提示词、scratch 提示词、scratch 提示词 | `tests/binding.test.ts` ✓ |
| T-15 | R | 既有行为 | 原 25 项（identity/catalog/cloud/scope） | 全绿 | `pnpm --dir <pkg> exec vitest run` ✓ |
| T-16 | U | Host 发布器 | 2 local + 1 cloud，其中 1 只读 | 文档无路径；locator 只写 local；只读 Mount `requestedMode=read-only`；cloud `read-only-projection` 且无 locator | `dsh_binding_bridge_test.dart` ✓ |
| T-17 | C | 映射表 | receipt 写回 id 后再发布 | 文档带 `dshWorkspaceId` 提示；`state` 文件 revision 递增、`mountWorkspaceIds` 含 id | 同上 ✓ |
| T-18 | U | 清扫 | 旧 `*.tmp`（3 小时前）+ 已移除 Mount 的 locator | tmp 被删；多余 locator 被删 | 同上 ✓ |
| T-19 | U | active Mount | 挂载后默认、`select` 切换、卸载回落、重启恢复 | 默认首个 local；切换后持久化；卸载回落到首个 local；重启后仍是上次选择 | `workspace_version_e2e_test.dart` ✓ |
| T-20 | E | 本机端到端 | 构建 → 物化 profile → 重启 app → 发布 binding | `storages/workspace.json` 出现 2 个 workspace 且 primary 首序；receipt/日志/清理符合预期 | §7.2 ✓ |
| T-21 | F | 失败注入（运行期） | locator 指向的目录被删 / 文档被写坏 / app 重启 | 缺目录 → 该 Mount 失败可见且不阻塞其它；坏文档 → 拒绝 + receipt 记录 + 回落 hint/scratch；重启后 binding 保持 | §7.2 ✓ |
| T-22 | U | 面板快照 | 未绑定 / 2 Mount 已绑定 / 1 Mount 失败 | 409 `NO_BINDING`；快照含 `bindingRevision`/`activeMountRef`/`appliedAt`/`mounts`（bound 与 failed 同列）；序列化中无设备路径、无 `path` 键 | `tests/panel-api.test.ts` ✓ |
| T-23 | U | 切挂载校验 | 空 `mountRef` / 未知 / 失败 Mount / 尚未绑定 | 400 `MOUNT_REF_REQUIRED`、404 `MOUNT_NOT_IN_BINDING`、409 `MOUNT_UNAVAILABLE`、409 `NO_BINDING` | 同上 ✓ |
| T-24 | E | 切挂载生效（真实 sidecar） | `POST /active` 切到另一个 Mount，再切回 | 200 `idempotent:false`；registry 顺序为 [目标, 另一 Mount, 用户 workspace]；快照 `activeMountRef` 同步；意图文件写出；同值重复 POST → `idempotent:true` | §7.5 ✓ |
| T-25 | U | 事件面 | 订阅后触发 apply / reject / activate | 收到 `binding.applied`、`mount.unavailable`、`binding.rejected`、`mount.activated`；抛异常订阅者被隔离；退订后订阅数归零 | `tests/panel-api.test.ts` ✓ |
| T-26 | E | SSE 真实流 | `curl -N .../binding/events` 期间做两次切挂载 | 连上先收 `event: snapshot`，随后两条 `event: mount.activated`；心跳为 `event: ping` | §7.5 ✓ |
| T-27 | U | Host 采纳意图 | 意图比 state 新 / 更旧 / 属于别的 workspace / mountRef 不在集合 / 协议不符 | 仅"同 workspace + mountRef 在集合内 + 比 `state.updatedAt` 新"时覆盖 `activeMountRef`，并把 `requestedAt` 记为新的 `updatedAt`；其余一律忽略 | `dsh_binding_bridge_test.dart` ✓ |
| T-28 | R | 面板入口回归 | 桌面加载新 `dsh-appflowy` 后取 `/` | 200 且仍含 WebView focus-flap、`__DSH_BOOT__`、`data-muse-surface` 三段注入；binding 仍应用；无插件加载错误 | §7.5 ✓ |
| T-29 | U+H | panel 客户端插件半部（PBU-01/02） | 无绑定（409）/ 三 Mount（bound+writable、bound+read-only、failed）/ 点击 chip / 点击 Mount 行 | 有绑定才渲染 chip 且文案为 `标题 · active Mount`；切换器列出全部 Mount 并标 `当前·可写`/`只读`/`不可用（code）`；failed Mount 行 disabled；点行先 `POST /active {mountRef}`（同源凭证）再 `startSession(dshWorkspaceId)`，工作区未出现在列表时报可读错误 | `test/client-half.test.mjs` ✓ |
| T-30 | E | chip/switcher 实测（真实实例） | 重启后在 panel 头部读取 chip，点开切换器，点第二个 Mount | chip 显示 `My Workspace · helix`；切换器列出 `helix 当前 · 可写` 与 `frontend`；点行后 DSH 工作区顺序与 `workspace-active-intent.json` 同步变化 | §7.6 ✓ |
| T-31 | U+E | watcher 双触发去重（去重摘要） | 绑定文档原子替换为**同字节**内容 / 追加 1 字节 / 还原 | 同字节 → 不再 `binding applied`（去重）；字节变化 → 重新 apply；还原 → 再次 apply；无重复物化 | §7.6 ✓ + `binding.test.ts` |
| T-32 | U | `resolveWithinBinding`（RLO-02/03） | 无绑定 / Mount 内路径 / 同名前缀兄弟目录 / 相对路径（按 active Mount）/ 第二 Mount / 路径形态 `resourceRef` / 不透明 `resourceRef` / 未知 `mountRef` / 只读 Mount + `edit` / 空请求 | 409 `NO_BINDING`；同根内 → 授权并带 `audit=resolve.legacy`；越界 → 403 `OPEN_DENIED`（不回落任意绝对路径）；最长根优先；路径形态 ref → 400 `RESOURCE_REF_INVALID`；`edit` + 只读 → 403 `MOUNT_READ_ONLY`；空请求 → 400 `PATH_REQUIRED`；返回值不含设备路径 | `resolve.test.ts` ✓ |
| T-33 | E | `POST /muse/v1/target.open` 实测 | helix 文件 / frontend 文件 / `C:\Windows\win.ini` / 路径形态 ref / 未知 mountRef / 不透明 ref（带 sessionId）/ 空 body | 200 + 终态回执（`via=mount-ref|resource-ref|legacy-path`，`status` + `reasonCode`）；越界 403 `OPEN_DENIED`；400 `RESOURCE_REF_INVALID`；404 `MOUNT_NOT_IN_BINDING`；带 sessionId 的不透明 ref 已进入 presentation 分发（回执 `failed`/`Error`，非静默） | §7.6 ✓ |
| T-34 | U | 只读 Mount 写保护（per-path） | `write`/`edit`/`str_replace_editor` 目标在只读 Mount 内 / 仅同名前缀的兄弟目录 / `str_replace_editor view` / 非 fs 工具 / 经 `ctx.fs` 与词法两条路径 | 仅命中只读 Mount（含子路径、相对路径按 session cwd）时 `deny`，理由含 Mount 显示名且**不含** `[sandbox: …]` 升级提示；其余 `next()` 放行 | `readonly-tools.test.ts` ✓ |

### 4.1 执行命令

```bash
# DSH 侧：类型检查 + 单测 + 构建
pnpm --dir openmuse/middlewares/dsh/plugins/appflowy-workspace check

# DSH 侧面板入口（构建 + 相关 suite）
pnpm --dir openmuse/middlewares/dsh/plugins/dsh-appflowy build
pnpm --dir openmuse/middlewares/dsh/plugins/dsh-appflowy exec vitest run tests/parent-bridge.test.ts tests/webview.test.ts

# Host 侧：定向测试（Flutter 3.27.4 / Dart 3.6.2）
D:\muse\flutter-3.27.4\bin\flutter.bat test ^
  test/workspace_platform/dsh_binding_bridge_test.dart ^
  test/workspace_platform/workspace_version_e2e_test.dart

# Host 侧：静态分析（仅改动面）
D:\muse\flutter-3.27.4\bin\dart.bat analyze lib/plugins/dsh_agent lib/workspace_platform test/workspace_platform
```

## 5. 端到端验证 runbook（本机 Windows）

### 5.1 物化到运行副本

运行中的 app 加载的是 **profile 副本**，closure 只作播种源：

```
D:\install\OpenMuse\muse\closure\node_modules\@muse\plugin-appflowy-workspace\dist\  (播种源)
%APPDATA%\OpenMuse\dsh\profiles\web\node_modules\@muse\plugin-appflowy-workspace\dist\  (实际加载)
```

1. `pnpm --dir <pkg> build` 生成 `dist\src\*.js`。
2. 备份并覆盖上述两处的 `dist\src\`。
3. 不删除 `profiles\web\.muse-seeded`：代际未变则不触发全量重播种；若代际变化（重装/D SH 版本变化/dsh-appflowy 重建），closure 副本已是新版，重播种仍然正确。

### 5.2 重启

面板内的 "Reload" 只重载 WebView，不重启 sidecar。必须退出并重启 app（sidecar 由 Host 拉起），确认 `127.0.0.1:3080` 重新监听。

### 5.3 断言（发布后）

| 检查 | 位置 | 期望 |
|---|---|---|
| workspace 注册 | `%APPDATA%\OpenMuse\dsh\storages\workspace.json` | 每个 Mount 一条记录；primary 在最前 |
| 绑定回执 | `%APPDATA%\OpenMuse\dsh\bindings\workspace-binding-receipt.json` | `mounts[]` 全 `bound`，`prunedWorkspaceIds` 含旧记录 |
| 路径过界 | `bindings\materialized\<mountRef>.path` | 仅 local Mount 有；文档本身无路径 |
| 日志 | `%APPDATA%\OpenMuse\dsh-sidecar.log` | `[muse-appflowy-workspace] binding applied` |
| 遗留清理 | `storages\workspace.json` | 旧 DSH home（`DSH Office\...\appflowy-workspaces\...`）记录消失 |
| 提示词 | 新会话 system prompt | 出现项目提示词（"one Mount of the Host Project Workspace"） |

### 5.4 失败注入

| 注入 | 期望 |
|---|---|
| 删除某 Mount 的 locator 或目录 | 该 Mount 记 `failed`，其它 Mount 仍绑定，Host UI/日志可见 |
| 把 binding 文档改成非法 JSON | 文档被拒，receipt 记 `documentErrors`，回落 hint/scratch，会话仍可用 |
| 重启 app | 绑定由文档重放（幂等），不产生重复 workspace |

### 5.5 回滚

恢复 `dist\src\` 备份、删除 `bindings\workspace-binding.json`（或改回旧 hint）、删除 `bindings\materialized\*`，重启 app。DSH 侧记录与会话按 Q-3 保留，不丢数据。

## 6. 风险与回滚

| 风险 | 缓解 |
|---|---|
| profile 副本被下次重播种覆盖 | 同时更新 closure 副本（§5.1） |
| 文档写坏导致面板空工作区 | 解析失败整份拒绝 + 回落 hint/scratch，不部分应用 |
| 两个发布者（workspace 切换 / Project Workspace 打开）互相覆盖 | 同一文件、同一发布器；`publish` 保留同 workspaceRef 的 Mounts |
| 清理误删用户记录 | 只清理 `appflowy-workspaces` 托管根且非当前 root 的记录；带会话默认不删 |
| 只读 Mount 的写保护 | P0 仅投影与 UI/提示词层标注；DSH 侧 sandbox 策略在 P1 落地（不宣称已实现） |

## 7. 结果记录（2026-09-20，本机 Windows）

### 7.1 自动化结果

| 层 | 命令 | 结果 |
|---|---|---|
| DSH 侧 | `pnpm --dir openmuse/middlewares/dsh/plugins/appflowy-workspace check` | typecheck 0 error；6 个测试文件 41 项通过（既有 25 + 新增 16）；`tsc` 构建成功 |
| Host 侧测试 | `flutter test test/workspace_platform/dsh_binding_bridge_test.dart test/workspace_platform/workspace_version_e2e_test.dart` | 8 项通过（bridge 7 项 + workspace E2E 1 项，后者含 active Mount 持久化与重启恢复断言） |
| Host 侧分析 | `dart analyze lib/plugins/dsh_agent lib/workspace_platform test/workspace_platform` | 本次新增/改动代码 0 issue（剩余 3 条 info 在改动面之外的既有文件） |

### 7.2 端到端（真实运行实例）

环境：安装版 `D:\install\OpenMuse`；`DSH_HOME=%APPDATA%\OpenMuse\dsh`；Host 定义 `...\workspace-platform-v1\definitions\917dc134-47af-4756-8101-45396a6e41d1.json`（frontend order 0、helix order 1，使用其真实 `mountRef` 与 `rootLocator`）。

产物：`bindings\workspace-binding.json`（948 B，不含任何设备路径）、`bindings\materialized\mount_*.path` ×2、`bindings\workspace-binding-receipt.json`。

| 场景 | 操作 | 实测结果 |
|---|---|---|
| 首次绑定（boot 重放） | 物化 dist → 重启 app | `binding applied`：helix → 新建 `4ae63006…`，frontend → 复用既有 `ba80ad2f…`；`failures: []`；`pruned: [d7b4e252…]`（旧 `DSH Office` 托管记录） |
| 面板可见 | 打开 `127.0.0.1:3080` | 侧栏"工作区"依序为 `helix`、`frontend`、`work`：两个 Mount 各占一个 DSH workspace，primary 在最前 |
| 注册表与顺序 | `storages\workspace.json` | `workspaceIds` = [4ae63006(helix), ba80ad2f(frontend), 8821cbb9(work)]；标题被改写为 Mount `displayName`；旧托管记录消失 |
| 回执 | `workspace-binding-receipt.json` | 两条 `state:"bound"`，含 `dshWorkspaceId`、`mode:"host-path"`；`prunedWorkspaceIds` 含旧记录 |
| 失败注入 A（目录缺失） | 删除 helix locator → revision 2 | helix `state:"failed"`、`code:"MOUNT_LOCATOR_MISSING"`；frontend 仍 `bound`；helix 的 workspace 记录保留（Q-3）；helix 之外无副作用 |
| 失败注入 B（坏文档） | revision 3 带 `path` 键 | 整份拒绝：receipt `bindingRevision:-1` + `documentErrors`；日志 `falling back to the legacy hint`；无部分应用 |
| 恢复 | 还原 locator → revision 4 | 两条重新 `bound`，workspace id 与首次相同（按路径复用，无重复记录） |
| 重启幂等 | 再次重启 app | 重放 revision 4：同一组 id、顺序不变、`failures: []`、`pruned: []` |
| tmp 清扫 | 按同一策略清理 bindings 目录 | 15 个 1 小时以上的孤儿 `*.tmp` 被清理，1 个 1 小时内的保留（与 `_sweepTemporaries` 规则一致） |

### 7.3 未在实例上验证的部分（诚实边界）

| 项 | 原因 | 替代证据 |
|---|---|---|
| Host 侧 v1 发布器实时写出（文档 + locator + 映射表 + tmp 清扫 + hint 退役） | 安装版 app 的 Dart 早于本次改动，重编整包 Flutter 不在本次范围 | `dsh_binding_bridge_test.dart` 7 项；实例中的文档/locator 就是该发布器契约的实例 |
| 提示词分叉在真实会话中的生效 | 会话存储不落 system prompt，无法从磁盘取证 | 选择逻辑单测（T-14）；线上日志证明 `getAppliedWorkspaceBinding()` 已含 host-path Mount |
| 旧 Host 遗留 hint 的退役 | 安装版 app 仍在写 `current-appflowy-workspace.json`（633 B，含 mounts）；新插件在有 v1 文档时忽略它 | 单测 T-16/T-17；本次运行日志未出现 hint 绑定 |
| 只读 Mount 对 agent 写盘的强制 | P0 只做投影语义与标注，DSH 侧 sandbox 策略属 P1 | 设计 §4.4、§8 已声明，文档不宣称已实现 |

### 7.4 遗留问题与下一步

1. 新 Host 首次发布时 `bindingRevision` 从 Host 侧映射表续号；本次实例中该文件由手写 fixture 建立（revision 1→4 由 DSH 侧消费）。
2. watcher 对同一次变更可能触发两次 apply（结果幂等、日志可见）；P1 可用 revision + 内容摘要去重。
3. P1：panel 客户端插件（chip/switcher + 切挂载 `startSession(workspaceId)`）、RLO/RCX 链路（见 `../../agent-file-references-and-open-routing.md`）。
4. 观测到的既有问题（与本次改动无关）：实例中 15:22 的会话存在 `400 param_wrong`，发生在重新部署之前。

### 7.5 P1-A 面板数据面实测结果（同日）

物化：`plugin-appflowy-workspace` 与 `dsh-appflowy` 两份 `dist` 同步进 profile 与 closure 后重启 app（备份见 §5.1 流程）。

| 场景 | 操作 | 实测结果 |
|---|---|---|
| 快照 | `GET /muse/v1/workspace/binding?token=` | 200：`protocol=muse.dsh/workspace-binding-panel/v1`、`bindingRevision=4`、`activeMountRef=helix`、两个 Mount `state=bound` 且带 `dshWorkspaceId`；响应内无任何设备路径 |
| 校验 | `POST /muse/v1/workspace/active` `{"mountRef":"mount:unknown"}` | 404 `MOUNT_NOT_IN_BINDING` |
| 切换 | 同端点 `{"mountRef":"mount:914e922d…"}`（frontend） | 200 `{"ok":true,"activeMountRef":"mount:914e922d…","idempotent":false}`；`storages/workspace.json` 顺序变为 `frontend > helix > work`；快照 `activeMountRef` 同步；`bindings/workspace-active-intent.json` 写出 `muse.workspace/active-intent/v1`（含 `requestedAt`） |
| 还原 | 切回 helix | 200；顺序回到 `helix > frontend > work` |
| SSE | `curl -N .../binding/events` 覆盖上述两次切换 | 连上先收 `event: snapshot`（完整快照），随后两条 `event: mount.activated`，`detail` 含 `workspaceRef/mountRef/dshWorkspaceId` |
| 入口归属 | 先前 404 的定位过程 | `parent-bridge` 在 Desktop 直接 `return`（`parentBridgeEnabled()` 只认 `MUSE_DOCUMENT_CLOUD_URL`）；改由 `@muse/dsh-appflowy/webview` 注册后端点即为 200/404/400 的预期行为 |
| 桌面回归 | 取 `/?token=` | 200（38.8 KB），index 仍含 WebView focus-flap、`__DSH_BOOT__`、`data-muse-surface` 三段注入 → 未回归 model picker 修复 |
| 启动 | 重启 app | `binding applied`（revision 4，2 Mount，`failures: []`）；无插件加载错误 |

边界：桌面 app 的 Dart 仍是重编前的二进制，因此"Host 采纳面板选择"这一段由 `dsh_binding_bridge_test.dart`（T-27）覆盖；线上验证到"意图文件 + DSH 侧顺序 + SSE 事件"，下一次 Host 发布才会把面板选择固化进 Host 真源。

自动化（本轮）：DSH 侧 `appflowy-workspace` 6→7 文件 41→49 项通过；`dsh-appflowy` 构建通过、`tests/webview.test.ts` + `tests/parent-bridge.test.ts` 22 项通过（该包 `pnpm typecheck` 仍有 2 个既有测试文件的 cordis `Context` 类型同一性报错，与本次改动无关）；Host 侧 `dsh_binding_bridge_test.dart` 7→9 项、`workspace_version_e2e_test.dart` 1 项通过；`dart analyze` 改动面 0 issue。

### 7.6 P1-B 实施结果（panel 客户端插件 / RLO 生产端与解析 / watcher 去重 / 只读写保护，同日）

物化与重启：新增客户端插件包 `@muse/dsh-client-ui-workspace-binding`（`dsh.client` 行 + `cordis.patch.yml` 行）与 `plugin-appflowy-workspace`、`dsh-appflowy` 的 `dist` 一并同步进 profile 与 closure 后重启 app 验证。

| 场景 | 操作 | 实测结果 |
|---|---|---|
| chip 渲染（T-29） | 重启后在 panel 会话头读取 chip | `My Workspace · helix`（Project Workspace 标题 + active Mount），与 `GET /muse/v1/workspace/binding` 的 `title`/`activeMountRef` 一致；快照 409 时不渲染 |
| 切换器（T-29） | 点 chip 展开 | 列出该 binding 的全部 Mount，标记 `当前 · 可写` / `只读` / `不可用（code）`；failed Mount 行 `disabled` |
| 切换（T-30，真实点击） | 点 `frontend` 行 | `POST /muse/v1/workspace/active` → 200 `idempotent:false`；面板快照顺序变 `frontend > helix`、`activeMountRef` 指向 frontend；`workspace-active-intent.json` 的 `mountRef` 同步为该 Mount（`requestedAt=1789904740433`）；随后调用 `uiWorkspace.startSession(frontend 的 dshWorkspaceId)` |
| 还原 | 同端点切回 helix | 200 `idempotent:false` → 快照 `helix > frontend`、intent `requestedAt=1789904819808`；实例回到初始状态 |
| 点击路径缺陷（两处，已修） | 首轮实测"chip 消失"与"点行无反应" | ① 重构时残留 `rootRef` 引用 → 副作用抛错使 React 卸载 chip；② 外部点击判定用 chip 按钮而非外层容器 → 按行先关面板、`click` 不触发。两处均已由 `test/client-half.test.mjs` 的用例复现并锁定 |
| watcher 去重（T-31） | 绑定文档原子替换为**同字节**内容 / 追加 1 字节 / 还原 | 同字节 → 不再 `binding applied`；字节变化 → 重新 apply；还原 → 再次 apply（apply 计数 9→10→11，同字节不动） |
| RLO 解析（T-32/T-33） | `POST /muse/v1/target.open`：helix 文件 / frontend 文件 / 越界 / 路径形态 ref / 幽灵 mountRef / 不透明 ref / 空 body | 200 + 终态回执（helix 与 frontend 的文件分别解析到各自 Mount，`via` 分别为 `legacy-path`/`mount-ref`）；越界 403 `OPEN_DENIED` + receipt `rejected`；`RESOURCE_REF_INVALID` 400；`MOUNT_NOT_IN_BINDING` 404；`{}` → 400 `PATH_REQUIRED` |
| RLO 分发 | 不透明 ref + `sessionId` | 回执 `status:"failed" reasonCode:"Error"`：已进入 presentation 消费者并交由 Host Bridge 分发；Host 侧承载器未实现，故为失败回执而非静默 |
| 只读写保护（T-34） | 单测：`write`/`edit`/`str_replace_editor` 目标落在只读 Mount 内 / 仅同名前缀的兄弟目录 / `view` / 非 fs 工具 | 仅命中只读 Mount（含子路径、相对路径按 session cwd）时 deny，理由含 Mount 显示名且不含 `[sandbox: …]` 升级提示；其余放行 |

自动化（P1-B）：客户端插件 4 项（`dsh-client-ui-workspace-binding/test/client-half.test.mjs`，覆盖 chip/切换器/切换动作/外部点击回归）；`appflowy-workspace` 7→9 文件 49→68 项（新增 `resolve.test.ts` 11 项、`readonly-tools.test.ts` 11 项）；`dsh-appflowy` 构建通过。

诚实边界（P1-B）：

1. **Host 半部未构建**：`target.open` 的承载器（`TargetRouter`/present-open-bridge/WebView2 JS 通道）与"Host 采纳面板意图"仍随下一次 Flutter 重编落地，因此实例上"点击链接 → 引擎打开"只到"进入分发并返回失败回执"，"面板切换 → Host 真源"只到意图文件（Dart 侧单测覆盖）。
2. **RCX 未实现**：`resource.reference` contribution 类型、`muse/resource-reference` 会话事件、composer 引用 chip、`excerpt ≤ 8 KiB` 截断均未落地（PRD §09 的 RCX-01..04 已标注"未落地"）。
3. **只读写保护的范围**：仅在 `tools/pre-execute` 层拦截（`write`/`edit`/`str_replace_editor` + bash 启发式）；DSH 写入策略只有"单一 mode + 单一 workspaceRoot"，没有 per-path deny-list，`danger-full-access` 会话下经 shell 的任意写入无法按路径拦截，需要引擎侧多根写入策略才能闭环。
4. **只读 Mount 未能线上验证**：当前 binding 的两个 Mount 都可写，只读路径由单测覆盖。
5. **`unsupported`/`timed-out` 分支**未在实例上分别构造（服务缺失与 5 s 超时），由代码路径与单测保证。
