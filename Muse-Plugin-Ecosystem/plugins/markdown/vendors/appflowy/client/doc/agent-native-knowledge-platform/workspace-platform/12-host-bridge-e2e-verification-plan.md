# 12. Host Bridge 端到端验收计划：DSH → Host 打开、Host → DSH 引用

状态：**计划（代码未实现）**，2026-09-20。归属：[agent-file-references-and-open-routing.md](../../../agent-file-references-and-open-routing.md)（RLO/RCX 设计稿）与本文档系列 09/10/11（绑定侧）。
本文件只回答"要让两条链路在真实 UI 上跑通并验收，还差什么、按什么顺序补"。

## 1. 验收定义（用户口径）

| # | 链路 | UI 操作 | 通过判据 |
|---|---|---|---|
| E2E-1 | DSH → Host | 在 DSH 对话流里点击本回合产出文件的名字（inline mention / 产出文件行 / 交付卡片） | 不弹浏览器、不落到 DSH 内置查看器；Host 打开该资源（Helix/Word/查看器 surface），且打开动作有终态回执（成功可见；失败也有可读原因，不静默） |
| E2E-2 | Host → DSH | 在 Host 编辑器里选中**超长**文本复制 → 粘贴进 DSH 输入框 | 输入框里出现的不是裸文本，而是一个"引用块/chip"，携带 `resourceRef`（或 View 引用）、来源显示名、锚点（行范围/章节）、所属 `mountRef`；随消息发送时模型可见同一份结构化来源信息，session log 可重建 |

两条都必须通过真实 UI（点击/粘贴）验收，而不是只看单测。

## 2. 已核实现状（本轮实测，非推断）

### 2.1 DSH 侧（存在且可运行）

| 组件 | 位置 | 状态 |
|---|---|---|
| panel 打开生产端 | `middlewares/dsh/plugins/dsh-appflowy/src/panel-routes.ts`（`POST /muse/v1/target.open`） | 已实现；实测 200 + 终态回执（`via=mount-ref/resource-ref/legacy-path`），越界 403 `OPEN_DENIED`，带 `sessionId` 时进入分发 |
| 解析规则 | `plugins/appflowy-workspace/src/resolve.ts` | 已实现（Mount 根包含、最长根优先、只读拒绝、不透明 `resourceRef`） |
| `openResource` 服务与 journal | `core/dsh-resource-presentation` | 已实现（服务定义 + durable journal + wire） |
| Host Bridge Provider（F0.3） | `plugins/dsh-resource-presentation-host` | 已实现：`OpenResourceService` → discover → schema digest → scoped bind → `resource.presentation.request/status/cancel` invoke；单测 14+ 项 |
| Host Bridge 合同/传输 | `core/protocol/host-bridge` | 已实现：合同/codec/schema registry、`ctx.museHost` Service、**POSIX UDS 与 Windows 命名管道两种 carrier**（Rust `WindowsNamedPipeListener`、TS `net.createConnection(endpoint.address)` + `windowsNamedPipeEndpoint`）、nonce/token 鉴权、长度前缀 framing |
| 模型可见工具 | `plugins/dsh-tool-resource-present` | 已挂载；当前 boot 日志显示 `present_resource` 在活跃工具表内 |
| UI consumer（node） | `plugins/dsh-client-ui-resource-open` | 已实现（`openDeliverable/openMention/openCard` → `requestPresentation`），但 **`package.json` 没有 `dsh.client`**，即它是 node 包，不是浏览器插件 |

### 2.2 Host 侧（Dart 库只有单测，未接线；Rust carrier 已接线，见 §2.4-2）

| 组件 | 位置 | 状态 |
|---|---|---|
| Host 呈现编排 | `packages/muse_surface_orchestrator/lib/src/{orchestrator,host_service}.dart` | 已实现 + 单测；`MusePresentationHostService.handle(envelope)` 是 Host 侧唯一入口，**全仓只有测试引用它** |
| envelope 编解码 | `packages/muse_resource_bridge/lib/src/bridge.dart` | 已实现（`capability: resource.presentation`，request/status/cancel） |
| 打开落地 | `lib/plugins/resource_surface/*`（helix/ioffice/查看器 surface、tab action registry） | 已实现；`MuseLocalResourceRouter.resolve()` 仍以 `path` + `sessionCwd` 解析（RLO 设计稿 B9–B13 的老问题） |
| 应用启动 sidecar | `lib/plugins/dsh_agent/dsh_sidecar.dart` | 已实现：spawn 参数、`NODE_PATH`、`profiles/web` 播种、`clearLeakedHostPackages`、`.muse-seeded` 代际 |

### 2.3 模块解析机制（本轮排掉的一个假象）

- 运行期 `@deepseek-ai/*` 通过 `%APPDATA%\OpenMuse\dsh\profiles\node_modules\@deepseek-ai\*` 的 **junction** 指回安装目录 closure（已核实 `cordis`/`dsh-tools`/`dsh-sandbox-policy` 均可解析；`import("@deepseek-ai/cordis")` 从插件目录实测成功）。
- `dsh_sidecar.dart` 的 `seedClosurePlugins` 刻意**不把 host-owned `@deepseek-ai/*` 拷进 profile**（`isHostOwnedSeedPackage`），只由 `healProfilesModuleFallback` 写 junction；日志里历史 `failed to import loader entry … Cannot find package '@deepseek-ai/cordis'` 是 junction 未修复的旧 boot 残留，**当前 boot 已无该错误**。
- 因此不要用"把 `@deepseek-ai/*` 复制进 profile"的方式修解析：那会造成 cordis 双实例，服务注册表分裂。

### 2.4 三个真实缺口

1. **链路 1 的 UI 入口不存在"插件"，但存在可覆盖的宿主接缝**（本轮新发现，修正了先前判断）：
   - 点击产出文件名**今天就有 UI 通路**：`ui-deliverables` 把产出文件渲染成 mention，`open` 回调调用 `owner.openFile`（`packages/client/ui-deliverables/src/client/turn-deliverables.ts:214,221`）；
   - `openFile` 由 chat 视图注入，实现是**远端调用**：`ctx.remote.session.openWorkspacePath({ path: resolveWorkspacePath(cwd, path) })`（`packages/client/ui-chat/src/client/apply.ts:120-126`）；
   - host 侧 `SessionController.openWorkspacePath` 只做校验，真正落地是 `this.openPath(...)`，而 `openPath` 来自构造参数 `SessionControllerInternals.openPath`（`packages/api/session-controller/src/index.ts:77,130`；缺省为 `openNativePath`）。
   - 即 RLO 设计稿的"通路 A：宿主 opener 覆盖"接缝就是**启动器注入的 `openPath` / `canOpenPath`**，与 `ctx.museHostConnector` 同源（都由启动器/组装层提供）；`session-controller` 也是可被 overlay 覆盖的 Loader entry id（见 `apps/web/tests/produced-files.overlay.yml`）。
   - 结论：链路 1 的 DSH 侧**不需要新建客户端插件**，只要让 Muse 组装层提供的 `openPath` 走绑定解析 + 呈现意图（等价于现有 `POST /muse/v1/target.open` 的服务端路径，二者应共用同一实现）。
2. **链路 1 的 Host carrier 与 `ctx.museHost` 已经随产品运行**（本轮决定性核实，推翻了"app 内没有 carrier"的先前判断）：
   - Rust `DesktopHostServer` 已被 `flowy-core/src/muse_runtime.rs:51-104`（`AppFlowyMuseRuntime::start`）实例化，命名管道 endpoint 由 `muse_native_windows.rs:37-64` 生成，并把 launch descriptor 写到 `%TEMP%\appflowy-muse-host-0.json`；
   - **本机实测**：该文件存在（本次 boot 写入），内容 `{"endpoint":"\\\\.\\pipe\\appflowy-muse-host-25768-…","hostGeneration":"appflowy.local.1","nonce":"…","runtimeInstanceId":"runtime.dsh-appflowy.0"}`；`D:\install\OpenMuse\dart_ffi.dll` 内含 `appflowy-muse-host` / `appflowy.local.1` 字符串，即 carrier 已编译进在跑的二进制；
   - DSH 侧 connector（`dsh-appflowy/src/connector.ts` + `native-launch.ts`）读取同一路径，`@muse/host-bridge/dsh` 的 `museHost` 与 `@muse/dsh-resource-presentation-host` 的 `openResource` 因此具备连接条件；sidecar 日志无 `CONNECT_FAILED`。
   - **真正的缺口是 Host 端没有 `muse.resource-presentation`（contract major 2）能力提供者**：`flowy-core` 只注册了 5 个 provider（`appflowy.view-reference`、`muse.workspace`、`appflowy.view-rename`、`muse.document`、`muse.word`，`muse_view_reference.rs:51-81`），全仓与 `dart_ffi.dll` 都搜不到 `muse.resource-presentation`。于是 `HostOpenResourceService` 的 discover→`descriptorCompatible()` 过滤必然落空并抛 `CAPABILITY_UNAVAILABLE`，这正好解释实测到的 `status:"failed", reasonCode:"Error"`（不透明回执）。
   - 附带缺口（Rust dispatcher）：`cancel.request` / `status.request` 在 `host-runtime/src/lib.rs:531` 落到 `InvalidFrame` 兜底，未被服务。
   - 因此**不需要**为 carrier 重建 Flutter/自建 UDS 桥；阶段 B 变成"补一个 Rust 能力提供者 + 两条 wire kind"。
3. **链路 2 两侧都为空**：Host 剪贴板只有文本/HTML/AppFlowy-JSON/文件四种形态（`clipboard_service.dart`），没有引用与锚点元数据；DSH composer 只处理纯文本与文件粘贴（`ui-conversation/.../input/editor/keymap.ts`），没有"引用块"数据结构与渲染位。

## 3. 实施顺序（每阶段都有可验收产物）

### 阶段 A：链路 1 的 DSH 侧入口（无需重建 app，可先用"失败可见"验收）

1. 让 Muse 组装层提供 `SessionControllerInternals.openPath` / `canOpenPath`（或在 overlay 里替换 `session-controller` entry），实现为**绑定解析 + 呈现意图**，与 `POST /muse/v1/target.open` 共用同一服务端实现（resolve → presentation request → receipt）；对 `path` 只做兼容期入参，内部立即转成 `resourceRef|mountRef`。
2. 失败不得静默：opener 抛出的错误必须能在 UI 上看到原因（`reasonCode`），这也是阶段 A 的验收信号。
3. 验收：在 DSH 对话流点击产出文件名 → 面板出现回执提示（Host 未接线时应为 `failed/Error`），并可在 sidecar 日志/回执中看到该次 resolve —— 证明点击已进入统一资源协议，而不是 DSH 原生 opener（同时回归确认：非绑定路径点击仍被拒绝）。

### 阶段 B：补 Host 侧 `muse.resource-presentation` 能力（Rust，不改 Dart/carrier）

1. **schema 单一来源**：把 `resource.presentation.status` / `cancel` 的 input/output schema 从 TS 内联（`dsh-resource-presentation/src/wire.ts:13-50`）发布为 JSON 文件（`contract-presentation/schemas/v2/` 或 `dsh-resource-presentation/schemas/v2-ops/`），TS 改为读取，Rust 侧消费同一份文档；`request` 的 digest 已由 fixture 固定（输入 `sha256:275af040…95ba`，输出 `sha256:cfd6b830…6427`）。
2. **新增 provider**：`frontend/client/frontend/rust-lib/flowy-core/src/muse_presentation.rs`，实现 `CapabilityProvider`，descriptor 为 family `muse.resource-presentation` / `contract_major = 2` / 三个操作（`request`: `local_write` + cancellable + idempotency `required`；`status`: `read`；`cancel`: `local_write`），schema digest 必须与 DSH 侧逐字节一致；在 `muse_view_reference.rs:51-81` 注册并纳入 `extend([...])`。`flowy-core/Cargo.toml` 增加 `muse-contract-presentation`（`middlewares/dsh/core/contract-presentation/rust`，已提供 `PresentationRequestV2`/`PresentationReceiptV2`/`validate()`/`schema_digest()`）。
3. **落地语义**：`resourceRef`（Mount 作用域、不透明）→ Host 侧按自己认证到的连接 + 绑定 + 策略推导 Mount 权威 → 解析到文档/视图 → 触发已有 `resource_surface`（Helix / Word / 查看器）打开，并返回 `muse.presentation/receipt/v2` 终态回执。
4. **wire 补齐**：`host-runtime/src/lib.rs:531` 之前补 `cancel.request` / `status.request` 分支（`cancellationId` 映射到 pending `Cancellation`；`status` 走幂等/回执查询）。
5. 验收（E2E-1）：点击产出文件 → `openPath`（阶段 A）→ `openResource.request` → discover 命中新 provider → bind → invoke → Host 打开 surface；回执 `status=opened|focused`；失败路径分别给出 `MOUNT_NOT_IN_BINDING` / `OPEN_DENIED` / `CAPABILITY_UNAVAILABLE`。构建产物为 `dart_ffi.dll`（Rust）与 sidecar closure（TS），安装目录替换前先备份 `OpenMuse.exe` / `data/app.so`。

### 阶段 C：链路 2（Host → DSH 引用块）

1. **Host 复制侧**：在复制路径（`copy_and_paste` + `ClipboardState`）写入"Muse 引用"载荷：结构化 JSON（`resourceRef|viewRef`、显示名、锚点、`mountRef`、内容摘要或 `excerpt` ≤ 8 KiB + `truncated`），同时保留纯文本回退（外部应用仍可粘贴）。
2. **DSH 粘贴侧**：composer 粘贴拦截（扩展点见 §4 待补），把该载荷转成输入框内的引用 chip（可删除、可多个），并把同一结构随消息发送；模型侧通过 session event 可见（`ignorable: true`）。
3. 验收（E2E-2）：在 Host 选中超长文本复制 → 粘贴进 DSH 输入框 → 出现引用块（不是裸文本）→ 展开可见来源/锚点/`mountRef`；发送后 session log 能重建同一份元数据。

## 4. 构建与安装（本轮核实，阶段 B/验收要用）

- **Flutter SDK 不在 `D:\flutter`**：那是 pub cache（`PUB_CACHE=D:\Flutter\.cache`）。真正的 3.27.4 在 `D:\muse\flutter-3.27.4`（`muse_windows.flutter_home()` 的硬编码候选之一；`require_flutter_327()` 拒绝非 3.27.x）。
- **构建入口**：`frontend/client/scripts/pack-windows-client.py`，依次
  `cargo make --profile production-windows-x86 appflowy-core-release`（cwd `frontend`）→ `flutter pub get` → `flutter build windows --release` → 拷贝 `build/windows/x64/runner/Release/` → `dist/windows/OpenMuse/` → `build-muse-closure.py` + `stage_closure` → Inno Setup 生成 `setup.exe`。所有 Windows 命令经 `muse_windows.run_vs()`（自动 `vcvars64.bat` + `D:\Rust\.cargo\bin`）。
- **Rust 产物链**：`cargo make` 会把 `dart-ffi` 的 `staticlib` 临时改成 `cdylib`，产出 `rust-lib/target/x86_64-pc-windows-msvc/release/dart_ffi.dll`，再由 CMake `install(FILES …)` 放到 exe 旁。因此**改 Rust provider 只需重建 `dart_ffi.dll`**，Dart/FFI 声明无需改动（Host carrier 不走 `dart-ffi`）。
- **`D:\install\OpenMuse` 不是脚本目标**（全仓搜不到该路径）：它是运行生成的 `setup.exe` 时手选目录（`install-windows-client.py` 装到 `%LOCALAPPDATA%\Programs\OpenMuse`）。刷新方式＝重跑 `pack-windows-client.py` 后再跑该 setup.exe，或按已授权方案先备份 `OpenMuse.exe`/`data/app.so` 再替换。
- **Dart 侧无任何 socket 服务**（`ServerSocket|HttpServer|RawServerSocket|RawDatagramSocket` 在 `appflowy_flutter\lib` 命中 0）；app↔sidecar 现有通道只有 binding JSON 文件、只读 HTTP 探活与 WebView2 `postMessage`。因此 Host 承载仍以 Rust carrier 为唯一正解（Dart 无一等命名管道 API）。
- 新增 `@muse/*` 包需**同时**登记到 `muse_windows.py` 的 `PACKAGE_DIRS` 与 `build-muse-closure.py` 的 `MUSE_PACKAGE_DIRS`，否则不会进 closure/profile。

## 5. 验收手段（本轮实测结论，决定两条链路各自怎么"过"）

| 手段 | 实测结果 | 适用 |
|---|---|---|
| `SendInput` 鼠标（窗口相对坐标） | **可用**：点击 Project Workspace 树、绑定面板 chip/切换器均产生预期可见变化 | 面板与 Host 的**点击类**验收（链路 1） |
| `SendInput` 键盘（Unicode 注入与 VK 两种都试过） | **不可用**：点击 Host 文档编辑区后输入字母，页面无任何变化；`Ctrl+A`/`Ctrl+C` 后剪贴板仍为预先写入的哨兵值 | **不能**用合成按键做 Host 编辑器的选中/复制 |
| 窗口截图 + 裁剪 + 读图 | **可用**（`%TEMP%\muse-ui.ps1`：`Focus-Muse`/`Shot`/`Click-Muse`/`Key-Muse`/`Type-Muse`） | 两条链路的可见结果留证 |
| `appflowy_flutter/integration_test`（178 个 dart 文件，desktop runner，`flutter test -d windows`，SDK 在 `D:\muse\flutter-3.27.4`） | 存在且是仓库既有真机驱动方式 | **链路 2 的正确验收通道**（真实 app 内的选中/复制/粘贴） |
| 面板 HTTP 契约 + 客户端插件 harness（`node --test test/client-half.test.mjs`） | 可用（当前 4/4 通过） | 链路 1 的服务端与面板逻辑回归 |

结论：
- **链路 1（点击文件 → Host 打开）可以用"鼠标点击 + 截图观察"完成真实 UI 验收**：点击发生在 DSH 面板（WebView2）内，结果出现在 Host 窗口，两者都能截到证据。
- **链路 2（Host 选长文本复制 → DSH 粘贴出引用块）不能靠合成按键验收**：Host 编辑器收不到注入键盘。必须走 integration_test（或由用户手工执行一次）。因此链路 2 的"通过"定义需要包含一条 integration test：在真实 app 内选中文本 → 触发复制 → 断言剪贴板载荷/引用块渲染。
- DSH 面板是 WebView2，不是 Flutter widget 树，widget 测试无法触达；面板侧只能靠 HTTP 契约测试 + 客户端插件 harness + 截图。


## 6. 风险

| 风险 | 影响 | 处理 |
|---|---|---|
| cordis 双实例 | 服务注册表分裂、`ctx.museHost` 不可见 | 坚持 `profiles\node_modules` junction 机制；禁止把 host-owned 包复制进 profile |
| 命名管道被他人连接 | 本机越权打开文件 | 沿用 nonce + connection token、单实例、仅本机；token 只存在于启动器 seam 与环境变量，不入日志/配置 |
| Flutter 重建替换安装目录 | 用户当前环境被换掉 | 先备份 `OpenMuse.exe` + `data/app.so`；失败即回滚 |
| profile 重播种 | 插件变更不生效 | 需要时清 `profiles\web\.muse-seeded` 触发重播种（代际变化） |
| 只读 Mount + `mode=edit` | 误写用户文件 | 已有解析层拒绝（T-32）；Host 侧同样按 binding 的 `readOnly` 二次校验 |

## 8. 续做清单（截至 2026-09-20 20:22 的现场）

### 8.1 已实测为绿的验证证据（可复跑的命令）

| 范围 | 命令 | 结果 |
|---|---|---|
| DSH 解析/绑定/不透明 ref | `cd middlewares/dsh/plugins/appflowy-workspace && pnpm exec vitest run` | **68/68 passed**（9 files） |
| 展示契约 TS | `cd middlewares/dsh/core/dsh-resource-presentation && pnpm exec vitest run` | **9/9 passed**（journal 3 + wire 6） |
| 展示契约 Rust | `cd middlewares/dsh/core/contract-presentation/rust && cargo test` | **5/5 passed**（golden） |
| 绑定面板客户端插件 | `cd middlewares/dsh/plugins/dsh-client-ui-workspace-binding && node --test test/client-half.test.mjs` | **4/4 passed** |
| `dsh-appflowy` 类型与测试 | `pnpm exec tsc --noEmit` / `pnpm exec vitest run` | tsc 干净；**56/58**（2 个既有红灯，见 8.2） |

### 8.2 两个已知红灯（均非本轮改动引入，已定位到具体断言）

1. `dsh-appflowy/tests/approval.test.ts:22`：`expected {outcome:'unavailable'} to deeply equal {outcome:'approved'}` —— 依赖运行中 AppFlowy 的私有 approval 描述符，属环境依赖用例；本轮未改 `src/approval.ts`。
2. `dsh-appflowy/tests/mobile-layout-contract.test.ts:28`：`expected '0.1.2-alpha.3' to be '0.1.0-rc.7'` —— **版本钉子漂移**（vendor 进来的 DSH 客户端已是 `0.1.2-alpha.3`）。需要一次显式的决定/提交来更新钉子，不做静默修改。

### 8.3 剩余步骤（按顺序，全部为机械步骤）

1. 收齐三条实现链的自述与首尾文件清单（链路 1 Host Rust provider / 链路 1 DSH opener / 链路 2 引用块），核对 8.1 的全绿状态。
2. **集成检查点**：Host provider 必须能由 `anchorHint`（`provider: "muse.mount-relative-path/v1"`，值 `{mountRef, path}`）解析出目标，否则链路 1 会停在"ref 有了但 Host 认不出"。
3. 重建：`frontend/client/scripts/pack-windows-client.py`（`cargo make --profile production-windows-x86 appflowy-core-release` → `flutter build windows --release` → closure → Inno）；仅 Rust 变更时只需替换 `dart_ffi.dll`。
4. 安装到 `D:\install\OpenMuse`（**先备份 `OpenMuse.exe` 与 `data/app.so`**）；纯 JS 插件迭代可用 `%TEMP%\muse-deploy.ps1`（12 包 × closure+profile，`-WhatIf` 已验证 96 条目全命中，安装件统一为 `@muse/<name>` 作用域布局），并会清 `.muse-seeded` 触发重新播种。
5. 链路 1 真实 UI 验收：`%TEMP%\muse-ui.ps1` 的 `Focus-Muse` →（先截图重新定位产出文件行，面板会滚动）→ `Click-Muse` → `Shot`，判据是 Host 窗口出现该文件；服务端可另用 `POST /muse/v1/target.open` + 三种入参做回执比对（改造前基线：path 单发 → `unsupported/NO_SESSION_SCOPE`；path+session → 422 `RESOURCE_REF_REQUIRED`；path 形状 `resourceRef` → 400 `RESOURCE_REF_INVALID`）。
6. 链路 2 验收：**不要用合成按键**（§5 已证伪），走 `frontend/client/frontend/appflowy_flutter/integration_test`（178 dart 文件，`flutter test -d windows`，SDK `D:\muse\flutter-3.27.4`）在真实 app 内断言"选中长文本 → 复制 → 剪贴板含引用载荷"与"粘贴出现带 metaData 的引用块"，或由人工执行一次并留截图。
7. 细粒度提交（父仓库 3 组 + 本次两条链路各自提交；子模块 4 组 + 链路 2 提交），显式 pathspec，**排除** `docs/windows-ci.md`、`docs/windows-incremental-build.md`、`WINDOWS-HELIX-OPEN-LATENCY-ANALYSIS.md`。
8. 推送 → 用 REST API dispatch `windows-build.yml`（`gh` 不可用，token 取自 Git Credential Manager）→ 等构建 → 下载安装包 → 复验。

### 8.4 本轮仍在落盘的新文件（说明实现尚未封板）

`appflowy-workspace/src/ref.ts` + `tests/ref.test.ts`（不透明 ref 及其测试）、`appflowy-workspace/tests/{binding,panel-api,prune,readonly-tools,resolve}.test.ts` 与 `src/{binding,events,home,materialize,panel-api,prompt,prune,readonly,readonly-tools,receipt,resolve}.ts`、`dsh-appflowy/src/{locator,opener,open-intent,panel-routes}.ts` + `cordis.patch.yml`、`dsh-resource-presentation/{schemas,fixtures}/**`、新插件目录 `dsh-client-ui-resource-reference/`。


- 本轮完成的是**诊断**，其中两条修正了先前的判断，直接改变了实施方案：
  1. **"模块解析失败"是假象**：`@deepseek-ai/*` 由 `profiles\node_modules` 的 junction 提供，当前 boot 已无 `failed to import loader entry`；不得把 host-owned 包复制进 profile（§2.3）。
  2. **链路 1 的 DSH 侧不需要新客户端插件**：点击产出文件今天就走 `openFile` → 远端 `session.openWorkspacePath` → `SessionControllerInternals.openPath`，这正是设计稿的"通路 A"接缝（§2.4-1）。
- 同时确认：所选传输方案**不只是"已支持"，而是已经随产品在跑**——Rust `DesktopHostServer` 由 `flowy-core/src/muse_runtime.rs` 启动并写 launch descriptor，本机 `%TEMP%\appflowy-muse-host-0.json` 实测存在，管道 endpoint/nonce 齐全，`dart_ffi.dll` 内含 `appflowy-muse-host`；DSH 侧 connector 读同一路径。因此**不需要自建 carrier、也不需要为 carrier 重建 Flutter**，真正的缺口收敛为两件事：Host 端缺 `muse.resource-presentation`（major 2）Rust 能力提供者（含 `resourceRef` → 视图/文档 + 打开副作用 + 终态回执），以及 DSH 侧缺生产调用者（`present_resource` 被 `enabled: false` 关闭、`ctx.resourceOpenClient` 无产品调用者、`muse/resource-discovered` 证据无人产出）。
- 未实现任何链路代码，因此 E2E-1/E2E-2 **尚未通过 UI 验收**；不得在 PRD/测试矩阵中标记为已落地。
- 下一轮起点：阶段 A（§3，无需重建 app 即可取得第一个 UI 可验证里程碑），随后阶段 B（需要 Flutter 重建与安装替换，已获授权并先备份 `OpenMuse.exe` / `data/app.so`）。
