# Workspace Platform 开发计划

## 1. 总体策略

采用旁路新建、逐步切换，不在 `ViewPB` 上继续堆字段：

```text
现有 Account Space / AppFlowy Collab ──────────────── 保留
现有 Resource Tab / Surface / Engine Adapter ─────── 保留演进
临时 LOCAL RESOURCES ─────────────────────────────── 立即移除
新 Workspace Domain + Provider + Explorer ───────── 旁路实现
AppFlowy 页面树 ─────────────── AppFlowyCollabProvider 兼容接入
旧 SidebarFolder ─────────────── feature flag 切换，稳定后删除通用途径
```

每一阶段都必须交付：方案增量、API/schema、实现、单元/合同/集成测试、性能基线、迁移/回滚开关。Ontology Runtime 不进入本计划的实现范围。

## 2. 建议模块

### Rust

```text
frontend/rust-lib/
  muse-workspace-domain/       refs、definition、entry、capability、receipt
  muse-workspace-runtime/      manager、mount lease、tree cache、watch merge
  muse-workspace-provider/     Provider SPI + registry + TCK
  muse-workspace-local/        本地 Provider
  muse-workspace-collab/       AppFlowy Folder/View 适配器
  muse-workspace-ssh/          SSH Agent/SFTP client
  muse-workspace-cloud/        cloud adapter SDK（后期）
```

若初期不拆 crate，也必须按上述 module 边界组织；禁止把 Provider 逻辑继续写进 `flowy-folder`。

### Flutter

```text
lib/workspace_platform/
  application/                 WorkspaceBloc、ExplorerBloc、MountBloc
  domain/                      UI DTO、context keys、commands
  infrastructure/              Rust dispatch adapters、device state
  presentation/
    selector/ explorer/ rows/ menus/ status/ empty_state/

lib/workbench/
  commands/                    CommandRegistry
  menus/                       MenuContributionRegistry
  context_keys/                typed context evaluation
```

### DSH

```text
middlewares/dsh/core/
  contract-workspace-v2/
  workspace-binding/
  workspace-provider-broker/

middlewares/dsh/plugins/
  dsh-workspace-host-provider/
  dsh-client-ui-workspace/
```

## 3. 阶段计划

### F0：清理临时实现与冻结术语（2–3 天）

目标：移除误导性的 `LOCAL RESOURCES`，不破坏文件 Tab 和 DSH 打开。

代码：

- 删除 `MuseWorkspaceResourceController/Section` 及 DI；
- 恢复 AppFlowy New Page 的原职责；
- 保留 `MuseResourceSurfaceOpener`、Resource Router、Tabs 和菜单 registry；
- 文案中区分 Account Space、Project Workspace、Mount。

测试：

- DSH 点击文件仍在 Host Tab 打开；
- Helix/iOffice/Viewer Tab 菜单仍工作；
- 侧栏不再显示 LOCAL RESOURCES；
- AppFlowy 新建页面回归。

退出标准：没有第二套内存文件列表；无 orphan DI 或 import。

### F1：领域合同与 Provider TCK（1 周）

目标：冻结 refs、definition、capability、list/stat/watch/mutation 合同。

开发：

- Rust domain types 与 protobuf/serde schema；
- Provider Registry、generation、deadline、cancellation；
- fake/in-memory Provider；
- TCK：分页稳定性、revision、取消、watch overflow、path leakage；
- Dart/TS codegen 或手写只读镜像类型；
- ADR：entryRef vs resourceRef、definition persistence、unsafe write。

指标：10,000 fake entries 分页无重复/遗漏；取消后无 late state；合同 digest 三端一致。

### F2：Local Provider 与 Workspace Manager（2 周）

目标：真正以本地目录创建/打开/关闭 Workspace。

开发：

- local bind、canonicalization、stat/list/range/read、atomic write；
- create/rename/move/copy/delete；
- platform watcher 归一化和 overflow resync；
- Workspace definition/device binding/最近项持久化；
- trust restricted/trusted；
- metadata Tree Store 与 LRU。

测试矩阵：macOS APFS、Windows NTFS、Linux ext4；大小写、Unicode、长路径、symlink、权限、外部 rename、10k burst。

退出标准：100k entry fixture 首屏和内存达到 PRD 指标；无 UI isolate 文件 I/O。

### F3：Flutter Workspace Explorer（2 周）

目标：用新 Explorer 完成本地工作区 P0 交互。

开发：

- Workspace selector、welcome、multi-root header；
- 虚拟化目录树、懒加载、分页、展开状态；
- single/double/aux click、Preview/Pinned Tab；
- create/rename/delete/drag、Refresh/Collapse/Reveal；
- Provider status、trust、错误和 empty state；
- active Tab ↔ Explorer selection；
- feature flag `museWorkspacePlatform`。

测试：Golden（light/dark/high contrast）、keyboard、Semantics、100k tree scroll、窗口缩放、窄侧栏、RTL。

退出标准：Local Workspace 日常操作不依赖 `ViewPB`；旧页面侧栏仍可切回。

### F4：公共 Command/Menu Contribution（1 周）

目标：Tab 和 Explorer 使用同一插件命令系统。

开发：

- Command Registry、Menu Registry、Context Key evaluator；
- effect/policy/approval/telemetry interceptor；
- location、group/order、when/enabledWhen、submenu；
- generation/disposer/conflict diagnostics；
- 迁移 Tab 的 iOffice/Helix/Viewer 动作；
- 默认 Explorer 文件操作迁移为 builtin commands。

测试：排序、条件切换、卸载清理、重复 ID、恶意 schema、权限变化、慢插件 timeout。

退出标准：Explorer Widget 内不出现引擎专用 switch；插件卸载零残留。

### F5：DSH Workspace Binding（2 周）

目标：Host Explorer 与 DSH 共享同一 Workspace/Provider scope。

开发：

- `muse.workspace/catalog/v2` 和 binding contract；
- session 创建/切换 Workspace；
- DSH FileSystem adapter 与 Provider Broker；
- tree/list/read/range/search/watch 工具按 mount grant；
- deliverable 输出 resourceRef；兼容 path 转换；
- Explorer `Ask Agent/Start Agent Here/Add Context`；
- change receipt 在 Explorer/Tab/DSH 关联。

安全测试：伪造 entryRef、跨 Mount traversal、symlink escape、过期 binding、权限撤销、session replay、credential redaction。

退出标准：Host 与 DSH scope 差异为 0；所有文件点击沿 Resource Presentation seam。

### F6：AppFlowy Collab Provider 与迁移（2 周）

目标：把既有页面能力作为 Provider 纳入新 Workspace。

开发：

- Public/Private/Space 虚拟目录；
- ViewPB ↔ entry/resource refs；
- Folder listener → watch events；
- share/lock/history/realtime capability；
- 页面创建、移动、删除命令；
- Favorite/Recent/Trash 作为投影视图或 Command，不污染文件 Provider；
- 旧 latest view 和 Tab session 迁移。

回归：协作编辑、离线同步、分享、角色、锁定、数据库视图、Trash restore。

退出标准：启用新 Explorer 后，现有用户页面和权限无损；关闭 flag 可回旧 UI。

### F7：SSH Remote Workspace（3 周）

目标：完整 SSH 工作区与远端 DSH/工具链。

开发：

- SSH profile、Keychain、host key、proxy/jump host；
- Remote Workspace Agent bootstrap/version handshake；
- encrypted multiplex transport；
- remote Provider、DSH、shell、LSP、search、watch；
- reconnect/backoff、offline cache、dirty recovery；
- SFTP fallback 与 capability downgrade；
- Remote status UI、logs 和 update。

测试：高延迟/丢包/断线、远端重启、host key change、大目录、ARM/x64 Linux、无 Agent 权限、SFTP only。

退出标准：断线不丢编辑；所有 execution 明确位于 remote；P95 达标。

### F8：Cloud Provider SDK 与首个适配器（2–3 周）

目标：证明非 POSIX Provider 可进入同一 Workspace。

开发：

- OAuth credential broker、cursor/etag/delta、range handle；
- Cloud Provider SDK/TCK；
- 选择一个真实 Provider 完成 list/read/upload/download/rename/search；
- 限流、重试、离线 metadata、分享链接；
- Web/Mobile 同合同适配。

退出标准：核心无 cloud provider 特判；10 GB 虚拟目录不全量同步。

### F9：稳定性、迁移默认开启（2 周）

目标：取代旧 Sidebar 页面树作为默认 Desktop 工作区。

开发：

- 性能、泄漏、crash recovery、telemetry dashboard；
- definition schema migration；
- safe mode / provider quarantine；
- 文档、插件开发指南、管理员策略；
- 灰度 5% → 25% → 100%。

退出标准：两个稳定版本后删除临时/重复通用途径；AppFlowy Provider 仍保留其领域实现。

## 4. 跨阶段测试矩阵

| 维度 | 必测值 |
|---|---|
| Provider | fake、local、muse-collab、ssh-agent、sftp、cloud |
| OS | macOS arm64/x64、Windows x64、Linux x64 |
| 状态 | ready、readonly、offline、auth-required、degraded、revoked |
| 规模 | 0、1、200、10k、100k entries；0 B、32 MB、2 GB file |
| 文件 | code、Markdown、DOCX/XLSX/PPTX/PDF、image、HTML、URL、audio、video、unknown |
| 操作 | list/open/create/rename/move/copy/delete/search/watch/reconnect |
| Agent | read-only、workspace-write、approval、replay、sub-agent scope |
| 安全 | traversal、symlink、stale revision、expired handle、malicious plugin |
| UX | keyboard、screen reader、light/dark/HC、narrow window、large font |

## 5. 发布门槛

每阶段必须满足：

- 合同与 TCK 100% 通过；
- P0 E2E 100% 通过，无 flaky；
- P0/P1 安全问题为 0；
- 资源/连接泄漏 soak test 8 小时无持续增长；
- feature flag 可回退且不回滚用户数据 schema；
- telemetry 能区分 Provider、Mount、请求和错误，但不包含敏感 locator。

## 6. 风险与缓解

| 风险 | 缓解 |
|---|---|
| AppFlowy upstream 更新与大改冲突 | 新模块旁路；只在 Sidebar/Tabs/DI 建窄 seam |
| SSH Agent 打包复杂 | F7 独立；先完成 Local 和 SFTP capability downgrade |
| Flutter 大树性能 | flat visible row model + virtualization + Rust delta |
| 多 Provider 语义不一致 | capability-first，不做虚假一致性；TCK 分 mandatory/optional |
| 用户混淆 Space/Workspace | 产品术语迁移、首次说明、清晰 selector 层级 |
| DSH scope 与 Host 漂移 | binding 为唯一真源，工具调用时重验，不缓存权限决定 |
| 旧协作页面回归 | AppFlowyCollabProvider 兼容层、双 UI flag、全量回归 |
| 插件拖慢菜单 | 声明式贡献、预计算 context、invoke deadline、quarantine |

## 7. Definition of Done

Workspace Platform 完成需同时满足：

1. 用户能创建一个含 Local、SSH、Cloud、AppFlowy Collab Mount 的多根 Workspace；
2. Explorer 对所有根使用同一交互、菜单和视觉语言，同时诚实展示能力差异；
3. 任意支持的文件从 Explorer 或 DSH 都在 Host 原生 Tab 打开；
4. DSH 的读写、终端、LSP 与 sandbox 使用同一 Workspace Binding；
5. 插件能向 Explorer 和 Tab 注入命令，不修改核心 Widget；
6. 现有用户、角色、协作、分享、锁定和页面数据无损；
7. 临时 `LOCAL RESOURCES`、路径型权限和格式枚举扩张均已退出主路径；
8. Ontology hooks 保留但 Runtime 未被误实现或耦合到 Workspace 核心。

