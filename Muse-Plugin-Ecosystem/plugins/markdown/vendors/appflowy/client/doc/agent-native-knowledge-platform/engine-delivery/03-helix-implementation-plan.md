# Helix Engine Adapter：实施、开发与测试方案

状态：**目标实施计划**  
上游：`vendors/helix` 25.7.1  
首发目标：macOS arm64 Desktop  
后续平台：Windows x86_64、Linux x86_64/arm64  
依赖：公共 G0/G1；不依赖 Ontology Runtime。

## 1. 产品定位

Helix 是 Muse 的专业文本/代码编辑 Surface，不是通用文件查看器，也不是默认 Agent shell：

- 用户选择代码/文本资源时提供高性能模态编辑；
- 保留 Helix 的 TUI、selection、tree-sitter 和可选 LSP 能力；
- 运行在 Host 管理的真实 PTY 中；
- DSH 可以请求打开/聚焦并读取 Resource snapshot，但默认不能向用户会话注入按键；
- 对非本地资源使用 working copy，保存后通过 Resource commit 回写。

首发不做：Helix-as-library、自绘 compositor、多人实时协同、Mobile 编辑、Agent 接管用户键盘、Ontology CodeSymbol 抽取。

## 2. 目标组件

```text
DSH OpenResource Consumer
      │ Host Bridge control
      ▼
Host Helix Adapter Provider
  ├─ Resource authorize/materialize
  ├─ EngineSession state/receipt
  ├─ PTY owner + process cleanup
  ├─ working-copy watcher/commit
  └─ short context/status
      │ raw PTY stream
      ▼
Flutter Terminal Surface
  ├─ VT renderer / IME / clipboard
  ├─ status strip: resource, mode, dirty, commit
  └─ focus/resize/close
      │
      ▼
hx --config <host-config> <materialized-file>
HELIX_RUNTIME=<bundled-runtime>
```

建议代码落点：

```text
Muse-Clients/middlewares/dsh/plugins/dsh-engine-helix/
Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_helix_surface/
Muse-Clients/frontend/client/frontend/appflowy_flutter/lib/plugins/helix/
Muse-Clients/frontend/client/scripts/engine/helix/
```

Vendor 保持 Muse 无知。构建脚本可从 `vendors/helix` 产出固定 digest 的 `hx` 与 runtime。

## 3. 关键决策

### H-D1：真实 PTY，不把 Helix 当 library

当前 `helix-term` 构造终端 Backend 并消费 terminal event。首版运行原始 `hx`，避免长期维护 Helix UI fork。

### H-D2：控制面与 PTY 数据面分离

Host Bridge 承载 spawn/focus/resize intent、状态、commit 和 receipt；PTY 原始字节走本地 WebSocket/Unix stream handle。禁止将重绘帧编码成 Host Bridge event。

### H-D3：工作副本为默认写入模型

只有 Resource Provider 明确返回 `directWriteSafe=true` 才可传真实 workspace path。云资源、AppFlowy blob、DSH 产物和临时资源全部使用 working copy。

### H-D4：`:w` 与 Resource commit 联动

Helix 写 working copy 后，Host watcher等待文件稳定（原子 rename/连续 hash 归一），将 session 标记 dirty 并自动发起 `user-save` commit。Flutter 外层状态条展示 saving/committed/conflict。若可靠捕获无法通过 H0 spike，则 v1 改为显式“提交到 Muse”按钮，`:w` 只保存工作副本。

### H-D5：不默认开启 LSP/DAP

首发配置禁用未打包的 language server 和 DAP，只认证 tree-sitter/基础编辑。LSP 是独立 H4 capability，必须声明 server、进程、workspace 与网络策略。

## 4. 阶段 H0：可行性与安全 Spike

### 4.1 阶段目标

证明构建产物、PTY、Flutter 输入渲染、working-copy 保存检测和打包签名五个最高风险点。不得在 Spike 前建设完整 UI。

### 4.2 方案设计

- 使用 release profile 构建 `hx`，记录 source commit、Rust toolchain、binary digest。
- `HELIX_RUNTIME` 指向临时 staging 中的 runtime；使用裁剪 `languages.toml`。
- 通过 DSH `subprocess.spawnTerminal` 或最薄 provider 启动真实 PTY。
- 临时 stream bridge 只监听 loopback，首帧认证、单 session 单 owner。
- Flutter Spike 使用候选 VT renderer；若 IME/宽字符失败，立即验证 WebView+xterm.js 备选。
- working copy 测试 Helix `:w` 的 truncate/rename/write 行为和 watcher 去抖。

### 4.3 开发任务

| ID | 任务 | 产物 |
|---|---|---|
| H0-D1 | 可重复构建 `hx` | build script、toolchain/digest report |
| H0-D2 | runtime/grammar 盘点 | 首发语言清单、体积和许可证 |
| H0-D3 | PTY spawn/resize/exit spike | 证明日志、最小 provider test |
| H0-D4 | Flutter VT/IME spike | demo page、输入兼容报告 |
| H0-D5 | working-copy watcher spike | 保存序列记录、去抖方案 |
| H0-D6 | macOS 签名/公证 spike | bundle smoke 和验证脚本草案 |
| H0-D7 | threat model | argv/path/stream/token/process-tree 风险表 |

### 4.4 测试矩阵

| ID | 测试 | 预期 | 类型 |
|---|---|---|---|
| H0-T1 | `hx --health` 使用 bundled runtime | 无 runtime missing | process smoke |
| H0-T2 | 打开 Rust/TS/Markdown fixture | 内容正确，首发 grammar 高亮 | manual/screenshot |
| H0-T3 | PTY resize 80×24→160×50 | Helix 布局重排，无僵死 | integration |
| H0-T4 | 中文 IME、emoji、CJK 宽字符 | 候选窗可用，光标/字符列对齐 | manual |
| H0-T5 | Esc/Tab/方向/Ctrl/Cmd/Alt | 组合键无 Shell/App 抢占 | manual |
| H0-T6 | `:w` 与原子 rename/连续写 | watcher 只产生一个稳定 dirty version | integration |
| H0-T7 | stream token 错误/重复 attach | 拒绝且不泄露 session | security |
| H0-T8 | 退出/kill Host/kill hx | PTY 与进程树在期限内清理 | fault |
| H0-T9 | symlink/`..`/绝对外部路径 | materialization 前拒绝 | security |
| H0-T10 | 签名后的 app 启动 hx/grammar | 可执行并加载，不触发 quarantine | package |

### 4.5 Gate H0

Go：

- hx/runtime 可重复构建和签名；
- VT renderer 或 xterm.js 备选至少一条满足中文输入；
- PTY resize/exit/cleanup 可测；
- working-copy 保存检测有确定方案；
- 无 shell 字符串拼接。

No-go：无法可靠捕获用户输入、进程无法完整清理、签名产物无法加载或保存检测可能漏写。No-go 时只保留 system-open，不进入 H1。

## 5. 阶段 H1：统一只读打开

### 5.1 阶段目标

从 DSH/工作区/最近资源通过统一 route 打开 text/code Resource，建立只读 Helix EngineSession。先验证公共链路，禁止保存。

### 5.2 方案设计

- Manifest 支持 `text/*` 与显式认证的 code format；mode 仅 `view`。
- Host 把内容物化为权限为只读的单文件工作副本。
- 启动参数固定为已验证 `hx`、`--config`、文件路径；cwd 为受控 workspace/mirror 根。
- Flutter Surface 绑定 PTY stream、Surface lease 和 EngineSession generation。
- status strip 显示只读、resource 安全名称、line/column（首版可从 TUI title/外层状态近似；无法可靠解析则不伪报）。
- DSH 接收 opened/focused/failed receipt；旧 Sidebar 可 fallback。

### 5.3 开发任务

| ID | 任务 | 目标文件/包 |
|---|---|---|
| H1-D1 | Helix Adapter Manifest + probe | `dsh-engine-helix` |
| H1-D2 | OpenResource Provider 适配 | Host provider/plugin |
| H1-D3 | read-only materialization | Resource Host |
| H1-D4 | PTY stream handle 与 attach | Host plugin |
| H1-D5 | Flutter Helix Surface | `muse_helix_surface` |
| H1-D6 | route/receipt/error UI | Orchestrator + locale |
| H1-D7 | lifecycle disposer/leak counters | Host/Flutter |
| H1-D8 | feature flag 与 fallback | composition/config |

### 5.4 测试矩阵

| ID | 场景 | 预期 | 层 |
|---|---|---|---|
| H1-T1 | Manifest schema/unsupported platform | probe unavailable，原因稳定 | contract |
| H1-T2 | DSH card/mention/ProducedFile 同 ref | 都进入 Helix 或同一 fallback | snapshot/E2E |
| H1-T3 | 只读 working copy 内尝试 `:w` | 写失败/只读提示，原资源不变 | E2E |
| H1-T4 | 同资源重复打开 | focus 现有兼容 Surface | integration |
| H1-T5 | 关闭 Surface | stream、PTY、materialization 全释放 | lifecycle |
| H1-T6 | 打开中取消 | 无 late Surface，进程清理 | fault |
| H1-T7 | hx 缺失/runtime 缺失 | probe unavailable，不到 start | package |
| H1-T8 | hx 启动后崩溃 | terminal failed receipt，可 Viewer fallback | fault |
| H1-T9 | 10 MiB/100 MiB 文本 | 按限制打开或明确 too-large | performance |
| H1-T10 | 权限撤销 | stream 关闭、Surface 脱敏、进程终止 | security |
| H1-T11 | Host Bridge payload 检查 | 无 PTY 原始帧和真实 path | security |
| H1-T12 | Ontology 包未安装 | 行为完全相同 | composition |

### 5.5 Gate H1

- 只读 golden path、取消、崩溃、撤权、重复打开全部通过。
- 0 个未释放 PTY/stream/materialization。
- DSH 关键快照记录 request 与 terminal result。
- 未出现 Save/可编辑误导。

## 6. 阶段 H2：可写会话与 Resource commit

### 6.1 阶段目标

允许用户编辑受支持的文本资源，保证 `:w` 后真实 Resource commit、revision、冲突和恢复可靠。

### 6.2 方案设计

#### Direct-safe

仅适用于 Host-local workspace file Provider：

- Provider 在 open 时确认路径、symlink、workspace、writable、watch 语义；
- Helix 直接写 source；
- Provider 用文件 identity/hash 生成新 revision 和 ResourceEvent；
- 外部写入与 Helix 写入都可观察。

#### Working-copy

默认：

- `baseRevision=R1`，工作副本只对 session owner 可写；
- watcher 将每次稳定内容标记为 workingCopyRevision；
- 自动/显式 commit 使用 expected R1；
- committed 后更新 base 为 R2；
- conflict 时冻结自动 commit，保留副本并展示 compare/retry/save-as/discard。

Close 规则：

- clean：直接关闭。
- dirty 未提交：阻止关闭并给出提交/保留草稿/放弃。
- commit in flight：等待/取消关闭。
- conflict：必须选择保留 working copy 或放弃，不自动删除。

### 6.3 开发任务

| ID | 任务 |
|---|---|
| H2-D1 | working-copy materialization 与状态存储 |
| H2-D2 | file watcher 稳定性/rename/去抖 |
| H2-D3 | Resource commit + idempotency + base revision |
| H2-D4 | direct-safe Provider capability |
| H2-D5 | Flutter dirty/saving/committed/conflict UI |
| H2-D6 | compare/retry/save-as/discard |
| H2-D7 | close/restart 残留 working copy 恢复 |
| H2-D8 | resource.changed event 与 origin/causal ID |
| H2-D9 | 审计和 metrics，不记录正文/path |

### 6.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| H2-T1 | working copy `:w` | 一次 commit，真实资源内容更新 |
| H2-T2 | 多次快速 `:w` | 每个稳定版本顺序提交，无旧版本覆盖新版本 |
| H2-T3 | commit 重放同 idempotency key | 不双写，receipt 一致 |
| H2-T4 | 外部在 R1 后改为 R2 | Helix commit 返回 conflict，R2 不被覆盖 |
| H2-T5 | conflict 后 retry/rebase | 用户复核后新 commit 成功 |
| H2-T6 | commit 期间断线/Host crash | 重启由 receipt/digest 判断结果，不盲目重写 |
| H2-T7 | dirty 关闭 | 明确提示，选择行为正确 |
| H2-T8 | 放弃 dirty | 原资源不变，working copy 可按策略清理 |
| H2-T9 | direct-safe symlink 变化 | 拒绝继续写，转 conflict/denied |
| H2-T10 | 文件权限变只读 | commit denied，保留 working copy |
| H2-T11 | 二进制/NUL 文件误路由 | probe 拒绝或 view-only，不损坏 |
| H2-T12 | 编码/换行/BOM | 未编辑内容 round-trip；编辑后按声明策略 |
| H2-T13 | 大文件保存 | 内存和延迟在预算内或提前拒绝 |
| H2-T14 | resource.changed 订阅 | 仅真实 commit 后发，dirty 不发 |

### 6.5 Gate H2

- 所有冲突/崩溃 fixture 无数据损坏。
- working-copy recovery 能找回未提交用户内容。
- direct-safe 与 working-copy 用 capability 区分，不靠路径猜测。
- H2 默认内部灰度；达到错误率门槛后才把 Manifest mode 从 view 扩为 edit。

## 7. 阶段 H3：DSH 与上下文协作

### 7.1 阶段目标

DSH 能安全请求打开/聚焦 Helix，并获得有界、可记录的文本上下文；不获得用户 PTY 控制权。

### 7.2 方案设计

- UI Consumer 统一入口，Agent tool 只接受已发现 resourceRef/anchorHint。
- 当前文件、revision、line/column/selection 通过短 TTL context contribution 发布。
- 文本内容通过 Resource snapshot(selector, maxBytes)，不从 PTY 屏幕抓取。
- DSH session event 记录 open intent、route result、context ref 和 commit terminal result。
- 用户 PTY input 与 Agent terminal input 使用不同 capability；后者当前不提供。

### 7.3 开发任务

| ID | 任务 |
|---|---|
| H3-D1 | DSH OpenResource UI/Agent Consumers |
| H3-D2 | Helix selection/context provider 可行性实现 |
| H3-D3 | Resource text snapshot 与 line anchor |
| H3-D4 | session event/snapshot fixture |
| H3-D5 | focus/selection TTL 与隐私过滤 |
| H3-D6 | model tool schema 拒绝 path/engineId/terminal bytes |

若无法从原始 Helix 稳定得到 selection，H3 首版只发布 resourceRef/revision/focus；禁止解析 ANSI 屏幕猜光标。selection 延后到受支持的 Helix extension/IPC。

### 7.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| H3-T1 | Agent 使用 catalog 返回 ref 打开 | 成功并有 durable result event |
| H3-T2 | Agent 猜测 resourceRef/path | schema/authority 拒绝 |
| H3-T3 | 请求 line anchor | 定位或明确 unsupported，不误定位 |
| H3-T4 | Context 超 TTL/失焦 | 不再进入模型上下文 |
| H3-T5 | selection 超大小 | 截断并带标记，正文不进入 event |
| H3-T6 | 读取不可见资源 | denied，搜索/错误不泄露内容 |
| H3-T7 | Agent 尝试 PTY input | capability 不存在/拒绝 |
| H3-T8 | session replay | 可重建请求和结果，不需要 PTY raw stream |

### 7.5 Gate H3

- model-visible 输入可从 session log 重建。
- 没有屏幕抓取、路径或 PTY secret 进入模型。
- 用户打开/编辑功能在没有 Agent Consumer 时仍可用。

## 8. 阶段 H4：多平台、LSP 与发布强化

### 8.1 阶段目标

完成 Windows/Linux 产物、运行时/grammar 管理、可选 LSP、安全更新和性能发布门。

### 8.2 方案设计

- 构建矩阵固定 OS/arch/toolchain/source commit，产物生成 SBOM/digest。
- runtime queries/themes 与 grammar 按语言包分层；基础语言随包，扩展包签名下载。
- LSP capability 明确 server argv、workspace、网络、资源预算和 kill tree。
- 配置写入 Muse userData，不读取/覆盖用户全局 Helix 配置；允许受控导入偏好。
- 每个平台 runtime probe 检查 binary、ABI、runtime、grammar 和可执行策略。

### 8.3 开发任务

| ID | 任务 |
|---|---|
| H4-D1 | macOS x64/Windows/Linux 构建与 package stage |
| H4-D2 | grammar language packs、签名、缓存/回滚 |
| H4-D3 | 可选 LSP Provider/Policy |
| H4-D4 | 配置迁移与版本隔离 |
| H4-D5 | benchmark、crash telemetry、adapter熔断 |
| H4-D6 | 安装/升级/卸载与旧 session drain |

### 8.4 测试矩阵

| 维度 | 矩阵 |
|---|---|
| OS/arch | macOS arm64/x64；Windows x64；Linux x64/arm64 |
| 输入 | US/中文/日文 IME；CJK/emoji/combining；键盘布局 |
| Shell/PTY | zsh/bash/pwsh；resize；signal；process tree |
| Grammar | 内置、下载、签名失败、版本回滚、缺 query |
| LSP | 未安装、启动成功、超时、恶意输出、workspace escape、kill |
| Upgrade | adapter/runtime N→N+1；已有 session drain；配置迁移 |
| 性能 | 冷/暖启动、10/100 MiB、超长行、连续 resize、8 小时泄漏 |

### 8.5 Gate H4

每个平台独立认证，不因 macOS 通过自动宣告其他平台可用。LSP 没有通过安全/资源门时，基础 Helix 仍保持可用。

## 9. 总测试与发布指标

| 指标 | 目标 |
|---|---:|
| route decision | local p95 ≤ 50 ms |
| 暖启动到 PTY 可输入 | Small p95 ≤ 800 ms；Medium p95 ≤ 1.5 s |
| resize 可见响应 | p95 ≤ 100 ms |
| working-copy save 到 Host 接收 | p95 ≤ 500 ms |
| local commit | p95 ≤ 1 s（≤10 MiB） |
| crash-free EngineSessions | ≥ 99.8% |
| duplicate commit | 0 |
| revision overwrite | 0 |
| session/PTY/materialization 泄漏 | 0（测试结束） |

## 10. PR/工作包建议

1. H0 build/runtime report。
2. H0 PTY stream spike。
3. H0 Flutter renderer spike。
4. H1 manifest/probe/read-only materialization。
5. H1 Flutter Surface/lifecycle。
6. H1 DSH UI consumer/fallback。
7. H2 working-copy commit。
8. H2 conflict/recovery UI。
9. H3 Agent consumer/context/logging。
10. H4 platform/package/LSP 分别独立 PR。

每个 PR 修改 DSH 非平凡行为时，按其仓库规则补 Agent Note、相关 snapshot、unit/E2E，并使用完整 Service Definition/Provider/Consumer seam。

## 11. 回滚

- 关闭 `helixAdapter.enabled` 后停止新 session，现有 session提示保存/导出工作副本后 drain。
- edit 发生异常可降级为 view-only，不删除 working copy。
- binary/runtime 更新失败回滚到上一 digest；新旧 session 不混用 runtime。
- Helix 不可用时 text 资源回落 open-file-viewer 或系统应用。

