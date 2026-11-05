# 开发计划、测试矩阵与验收门禁

## 1. 阶段状态

| 阶段 | 交付 | 状态 |
|---|---|---|
| VD-0 | 核心六概念与边界 ADR | 已完成 |
| VD-1 | 通用 Dart contract、DiffProvider Registry | 已完成 |
| VD-2 | 本地 CAS、Version sidecar、Audit ledger | 已完成 P0 |
| VD-3 | 文本/Markdown/代码 Semantic DiffProvider | 已完成 P0 |
| VD-4 | Host Diff Tab、Unified/Split Viewer | 已完成 P0 |
| VD-5 | 资源 Tab 菜单、历史、审计、DSH 共路由 | 已完成 P0 |
| VD-6 | 性能、错误态、可访问性加固 | 未完成 |
| VD-7 | Proposal、Decision、Mutation、三方合并 | 协议预埋，未实现 |
| VD-8 | AppFlowy Collab Adapter | 未实现 |
| VD-9 | Office/Media/CAD Domain Providers | 未实现 |
| VD-10 | SSH/Cloud VersionStore | 未实现 |

## 2. 后续阶段设计

### VD-6：产品化加固

开发项：

- Diff 计算移入 isolate，增加 cancellation token。
- 大文件阈值、二进制探测、编码检测。
- 虚拟化行渲染，避免构造全部 SelectableText。
- 语法高亮、搜索、键盘导航、无障碍 label。
- CAS 垃圾回收、配额与引用扫描。
- 完整性校验与用户态错误页。

完成指标：20k 行 P95 < 800 ms；10MB 不阻塞 UI thread；Viewer 滚动 P95 frame < 22 ms。

### VD-7：Proposal 与 Action

开发项：

- 持久 Proposal Store 和 provenance link。
- DSH `createProposal` 与 Host `openProposal`。
- Change Decision 状态机。
- 文本 patch IR，与展示 Diff 分离。
- base/proposed/current 三方合并和 ConflictSet Viewer。
- apply/rollback 审计事件。

完成指标：stale base 不覆盖当前文件；逐 Hunk 决定可复放；Crash 后 Proposal 状态恢复。

### VD-8：Team Knowledge Adapter

开发项：

- AppFlowy Public/Private/Space View → Resource。
- Collab snapshot/history → VersionStore。
- Share/lock/permission → capability。
- Markdown、Database、Word 各自 Domain Provider。
- 原页面 Tab 与本地文件共用版本菜单。

完成指标：不回归协作、分享、锁定、历史和权限；同一 Viewer shell 路由不同领域 renderer。

### VD-9：非文本领域

建议顺序：DOCX → PPTX → XLSX → PDF → 图片 → 音视频 → CAD/3D。

每个领域必须独立交付：

1. IR Schema 和稳定 ID 策略。
2. Golden corpus。
3. DiffProvider。
4. Renderer/ViewModel。
5. Action/merge 可行性说明。
6. 性能预算与安全边界。

没有领域 IR 与 Golden corpus 不进入 UI 开发。

### VD-10：远端版本后端

开发项：

- VersionStore SPI。
- SSH/SFTP adapter、对象存储 adapter、企业云 adapter。
- 缓存、离线、lease/CAS、重试、凭证隔离。
- 多设备审计顺序与幂等。

## 3. P0 自动化测试矩阵

| 层 | Case | 预期 | 自动化 |
|---|---|---|---|
| Contract | Registry 解析 Markdown | 命中文本 Provider | 已通过 |
| Contract | Registry 解析 MP4 | 不错误命中文本 Provider | 已通过 |
| Diff | replace + insert | 正确 additions/deletions | 已通过 |
| Diff | 同一输入重复计算 | Change ID 稳定 | 已通过 |
| Diff | Markdown heading | Hunk 带最近标题语义 | 已通过 |
| Diff | 单行 replace | 产生 inline changed span | 已通过 |
| Version | 保存版本 | Version metadata 与 Blob 写入 | 已通过 |
| Version | 重建 Repository | 历史和内容可恢复 | 已通过 |
| Version | 无 baseline 比较 | 返回空而非伪造比较 | 已通过 |
| Audit | 保存 + 比较 | 两类事件可查询 | 已通过 |
| Audit | Agent 比较归因 | Actor 类型持久化并恢复 | 已通过 |
| Audit | 增删改摘要与预览 | metadata 和彩色审计卡正确 | 已通过 |
| E2E service | 保存→编辑→比较 | Working Version 与 Hunk 正确 | 已通过 |
| Viewer | Split 首屏 | 基线/当前标题、统计、old/new 内容正确 | 已通过 |
| Viewer | 切换 Unified | UI state 切换正确 | 已通过 |
| Existing | Resource route matrix | 不回归三引擎路由 | 已通过 |
| Existing | Action Registry | 排序/替换/执行不回归 | 已通过 |

测试命令：

```bash
flutter test test/plugins/version_diff test/plugins/resource_surface
flutter analyze lib/plugins/version_diff \
  lib/plugins/resource_surface/resource_tab_actions.dart \
  lib/startup/plugin/plugin.dart \
  lib/startup/tasks/load_plugin.dart \
  lib/startup/deps_resolver.dart
```

## 4. 必补测试矩阵

### 4.1 算法

- 空→内容、内容→空、全相同、全替换。
- 重复行、长公共子序列、Unicode grapheme、CRLF、无 EOF newline。
- 1/20k/100k 行基准。
- 语言与 media type 冲突。
- stable ID 对非相关上下文插入的稳定性。

### 4.2 存储

- 并发 capture 同一文件。
- 写一半崩溃与临时文件恢复。
- Blob 摘要错误、sidecar 损坏、磁盘满、只读目录。
- CAS 去重、GC 引用完整性、配额。
- 符号链接与文件重命名后的 Resource identity 策略。

### 4.3 UI

- 720/1024/1440/超宽窗口。
- 明暗主题、高对比度、200% 字体。
- 10k Hunk 虚拟化。
- Tab 关闭/重开、比较 Tab 去重策略。
- 菜单异步执行期间 loading、重复点击、错误提示。
- 键盘-only 与 VoiceOver。

### 4.4 安全与权限

- DSH 越过 session workspace 的路径继续被拒绝。
- 只读 Provider 不显示 capture/apply。
- 审计不泄露内容和 secret。
- Provider 返回恶意 displayName/metadata 时正确转义。
- 远端内容大小与解压炸弹限制。

## 5. 手工端到端验收

1. 构建并启动 macOS Host。
2. 从 Host 手动打开 `sample.md`，确认在 Host Tab 内呈现。
3. Tab 菜单选择“保存当前版本”。
4. 编辑并保存该文件。
5. 选择“与上一保存版本比较”。
6. 确认 Diff 在 Host 同一窗口新 Tab 打开。
7. 确认首屏为 Split，检查基线/当前方向、Unified 切换、字符变化、折叠、上一/下一。
8. 打开“版本历史与审计”，确认 capture 与 comparison 事件；Agent/User 标签以及新增绿、删除红、修改蓝正确。
9. 在 DSH 对话中点击同一个文件路径，重复 3–8，确认使用同一路由和菜单。
10. 对 `.rs/.ts/.js/.java/.c` 各重复一次。

验收失败条件：另起原生窗口、Tab 显示引擎名、内容区被侧栏遮挡、Viewer 固定尺寸、无基线却显示无变化、审计记录正文。

## 6. 发布门禁

P0 可用于开发验证，但进入默认开启前必须满足：

- 静态分析 0 error/warning。
- 当前自动化矩阵全部通过。
- 完成大文件、编码、磁盘失败的用户态错误处理。
- CAS 有配额与可追踪 GC。
- 完成 macOS 手工验收并留存截图/日志。
- 安全评审确认本地路径、审计 metadata 和 DSH 边界。
