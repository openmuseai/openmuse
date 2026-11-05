# Muse Workspace Platform 产品需求文档

状态：PRD v1  
优先端：macOS Desktop；随后 Windows/Linux/Web；Mobile 仅浏览、选择和审批  
目标：把 Host 从“AppFlowy 页面列表”升级为 Agent Native Knowledge Platform 的通用工作区。

## 1. 产品定义

Project Workspace 是用户、人类协作者和 Agent 围绕一项工作共享的资源与执行边界。它以一个或多个目录型 Mount 为载体，Mount 可以来自本地、SSH、云端或 Muse 协作空间。任何文件格式都作为 Resource Entry 进入同一 Explorer、Tab、权限和 Agent 协议。

Workspace 本身不编辑文件，也不决定格式引擎。它负责：

- 定义资源范围和根目录；
- 管理 Provider 连接、生命周期、能力和健康状态；
- 提供统一目录树、查找、菜单和拖放入口；
- 为 DSH 建立受权 workspace binding；
- 将文件交给 Surface Orchestrator 和具体 Engine Adapter；
- 保存与团队共享的工作区配置，同时隔离设备私有状态和 credential。

## 2. 目标与非目标

### 2.1 目标

| ID | 目标 | 用户结果 | 核心指标 |
|---|---|---|---|
| W-G1 | 任意目录成为工作区 | 本地/SSH/云目录使用同一 Explorer | 成功挂载率 ≥99.5% |
| W-G2 | 任意文件成为一等资源 | 点击文件总能打开、降级或解释失败 | 有效打开结果率 ≥99.9% |
| W-G3 | Agent 与人共享同一边界 | DSH 看到的根、revision、权限与 Host 一致 | scope 不一致为 0 |
| W-G4 | 插件扩展而不改核心 | 新 Provider/菜单/格式无需修改 Explorer 核心 | 标准 Provider 接入 ≤5 人日 |
| W-G5 | 保留协作基座 | 用户、团队、角色和 AppFlowy 协作页面继续工作 | 既有协作回归通过率 100% |
| W-G6 | 远端体验可理解 | 用户清楚当前位置、连接、信任和执行域 | 远端误操作率 <0.1% |

### 2.2 非目标

- 不将所有文件上传到 Muse Cloud。
- 不用 AppFlowy Collab 同步任意二进制目录。
- 不承诺所有 Provider 都支持写入、watch、搜索、终端或协作。
- 不用一个万能文件 API 替代 Word、表格、Git、LSP 等领域合同。
- 不在本阶段实现 Ontology Runtime；只预留稳定事件和扩展字段。
- 不在首版提供跨 Provider 原子移动；默认执行 copy + verify + source delete proposal。

## 3. 用户场景

### S1：打开本地工程并与 DSH 协作

1. 用户选择“打开文件夹”，选择 `/repo/product`。
2. Host 创建 Project Workspace 和 Local Mount，先展示根节点骨架，再渐进列出目录。
3. 用户展开 `docs`，点击 `prd.docx`，文件在当前 Host 窗口的 Tab 中通过 iOffice 打开。
4. 用户点击 `src/retry.ts`，Helix Tab 打开；Tab 与 Explorer 选中状态同步。
5. 用户打开 DSH，当前 session 自动绑定该 Workspace；DSH 的 cwd 指向同一 Local Mount 根。
6. Agent 修改 `retry.ts` 后，Provider watch 产生 revision change，Explorer dirty/changed 装饰与 Tab 同步。

验收：Host、DSH、终端和文件 Tab 对同一资源产生同一个 canonical resourceRef；文件路径只在 Local Provider 内部出现。

### S2：连接 SSH 工程

1. 用户从 Workspace 菜单选择“Connect to SSH…”，选择已保存主机和远端目录。
2. Host 显示 fingerprint、认证方式、信任范围；credential 写入系统 Keychain，不进入 workspace 配置。
3. Remote Workspace Agent 启动后返回能力：list/read/write/watch/search/exec/lsp。
4. Explorer 使用同一树控件显示远端目录，并用 Remote badge 标明执行域。
5. DSH session、终端、LSP 和需要磁盘的插件在远端运行；Viewer 和 Host UI 在本地运行。
6. 断线后 Tab 保留最后已读取内容并进入 `stale/offline`，写入动作停止；重连后按 revision 校验恢复。

验收：断线不丢未提交编辑；未重连前不显示“保存成功”；远端路径不交给本机 Finder。

### S3：挂载云端资料库

1. 用户通过企业 Cloud Provider 选择团队目录。
2. Explorer 首次只读取目录 metadata；展开时分页加载。
3. PDF/图片/视频使用 Viewer 的 range/stream 数据面打开，不要求完整下载。
4. Provider 不支持原地编辑时，“Rename/Save”禁用并解释；“保存副本”写入允许的目标。
5. DSH 可在用户授权下读取指定 range 或生成摘要，但拿不到 OAuth refresh token。

验收：10 GB 目录挂载时不进行全量同步；首屏目录 P95 ≤1.5 秒；credential 泄漏为 0。

### S4：多根产品交付工作区

产品经理创建 `Payment v3` Workspace，包含：

- `Local · client-code`；
- `SSH · service-staging`；
- `Cloud · research-assets`；
- `Team Knowledge · PRD & Decisions`。

工作区定义向团队同步，但本地绝对路径通过每位成员的 mount binding 解析。缺失 binding 的根显示“需要连接”，不伪装为空目录。Agent 任务可声明使用哪些 Mount，危险操作必须逐 Mount 授权。

### S5：插件向 Explorer 注入动作

安装 Git、iOffice 或 CAD 插件后：

- Git 在仓库根菜单注入“Open Source Control”，在文件行注入 diff 状态和“Compare with HEAD”；
- iOffice 在 `.docx` 节点菜单注入“使用 iOffice 打开”；
- CAD 在受支持文件注入“生成缩略图”和“打开装配关系”；
- 动作只在 `when` 条件成立且 policy 允许时出现；插件卸载后菜单和 decoration 自动释放。

验收：插件不能直接修改 Explorer Widget；重复 action ID 被拒绝并有诊断。

### S6：保留 AppFlowy 协作知识

团队原有 Public/Private/Space 页面出现在 `Team Knowledge` Mount。Markdown/Database/Word 页面保持原有协作、分享、锁定、历史和权限。它们与本地文件共用 Tab 与菜单，但 Provider capability 不同：协作页面可 Share，本地文件可 Reveal in Finder。

## 4. 功能需求

### 4.1 Workspace 生命周期

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| WSP-01 | P0 | 新建空 Workspace、打开文件夹、打开最近 Workspace | 重启后可恢复，缺失根有明确状态 |
| WSP-02 | P0 | Add/Remove/Rebind Mount | 不删除源数据；Remove 前处理 dirty session |
| WSP-03 | P0 | Multi-root | ≥16 个根仍可独立展开、搜索、授权 |
| WSP-04 | P0 | Close/Reopen | 释放 watcher/connection/session；恢复 Tab 和展开状态 |
| WSP-05 | P1 | 可选 `.muse/workspace.json` | 可提交 Git；不含 credential、token、绝对用户路径 |
| WSP-06 | P1 | 团队共享 workspace definition | 每人独立完成 mount binding |
| WSP-07 | P1 | Duplicate/Export/Import definition | 只复制配置，不隐式复制文件内容 |

### 4.2 Explorer

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| EXP-01 | P0 | 目录展开/折叠、懒加载、分页 | 折叠不继续 I/O；过期响应被 generation 丢弃 |
| EXP-02 | P0 | 单击预览、双击固定、辅助键新 Tab | 与 Tab 选中和 Reveal 同步 |
| EXP-03 | P0 | 新建文件/目录、重命名、删除、复制、移动 | 仅 capability 支持时启用；写入校验 revision |
| EXP-04 | P0 | 上下文菜单、行内菜单、根菜单 | 使用公共插件贡献协议 |
| EXP-05 | P0 | Refresh、Reconnect、Collapse All | 不丢选择和已打开 Tab |
| EXP-06 | P0 | 文件图标、Provider badge、状态 decoration | 不只靠颜色表达状态 |
| EXP-07 | P1 | 多选、拖放、跨根复制 | 跨 Provider 显示传输进度和部分失败 |
| EXP-08 | P1 | Filter、Quick Open、Find in Workspace | Provider search 优先；fallback 索引受配额控制 |
| EXP-09 | P1 | watcher 合并与外部变化 | 100 ms 窗口合并；rename 保持资源连续性 |
| EXP-10 | P2 | 自定义 Explorer View | 插件只能提交投影节点，不接管核心资源树 |

### 4.3 Provider 与连接

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| PRO-01 | P0 | Local Provider | macOS/Windows/Linux 的 list/stat/read/range/write/watch |
| PRO-02 | P0 | AppFlowy Collab Provider | 保留页面协作与权限，映射为资源树 |
| PRO-03 | P1 | SSH Remote Agent Provider | 断线重连、host key、远端 exec/search/watch |
| PRO-04 | P1 | SFTP 降级 | 无 Agent 时支持浏览/读写，明确无 LSP/exec |
| PRO-05 | P2 | Cloud Provider SDK | OAuth/企业策略、etag、分页、range、share link |
| PRO-06 | P0 | Capability probe | 菜单、Agent 工具和 Engine 路由消费同一结果 |
| PRO-07 | P0 | Health/state | connecting/ready/degraded/offline/auth-required/error |

### 4.4 DSH

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| DSH-W01 | P0 | Session 绑定 Workspace | session event 持久化 workspaceRef + mount scopes |
| DSH-W02 | P0 | 目录/文件发现 | Agent 获得有界 catalog/page，不注入全树正文 |
| DSH-W03 | P0 | 文件点击与 Agent open 使用同一 ResourceRef | Host 打开结果可重放、可审计 |
| DSH-W04 | P0 | sandbox policy 按 Mount | local/remote/cloud 权限分别计算 |
| DSH-W05 | P1 | 执行位置路由 | shell/LSP/index 在具有 exec 能力的数据域运行 |
| DSH-W06 | P1 | change feed | Agent 只订阅任务范围内、过滤后的变化 |
| DSH-W07 | P1 | workspace handoff | 新会话/子 Agent 只继承显式 scope，不继承 credential |

### 4.5 菜单和插件

| ID | 优先级 | 需求 | 验收 |
|---|---|---|---|
| PLG-W01 | P0 | Command Registry | 动作身份、标题、icon、effect、handler 独立于 UI |
| PLG-W02 | P0 | Menu Contribution | Tab/Explorer/Command Palette 共用声明模型 |
| PLG-W03 | P0 | Context Keys | scheme、kind、capability、selection、trust、dirty、online |
| PLG-W04 | P0 | 生命周期 | plugin generation 卸载后命令、菜单、watcher 全释放 |
| PLG-W05 | P1 | 动态 capability | Provider 状态变化后菜单在 250 ms 内更新 |
| PLG-W06 | P1 | Agent callable command | 必须声明 effect、schema、approval 与 workspace scope |

## 5. 分端能力

| 能力 | Desktop | Web | Mobile |
|---|---|---|---|
| Local folder | 完整 | 浏览器授权目录/虚拟 Provider | 不支持或系统文件选择器单文件 |
| SSH | 完整 | 通过 Cloud/Remote Gateway | 只读/轻量操作 |
| Cloud Mount | 完整 | 完整 | 浏览、预览、上传 |
| 多根 Explorer | 完整 | 完整 | 简化树/最近资源 |
| 原生引擎 | iOffice/Helix/Viewer | Web Viewer/协作编辑器 | Viewer/系统预览 |
| DSH | 本地或远端执行 | 远端执行 | 会话、审批、轻量动作 |
| 拖放/批量文件 | 完整 | 浏览器能力允许范围 | 系统分享/上传 |
| Provider 管理 | 完整 | 云 Provider | 查看状态、重新登录 |

## 6. 非功能指标

- 打开含 100,000 项的本地 Workspace：不预扫正文，Explorer 可交互 P95 ≤800 ms。
- 展开 1,000 项目录：首批 200 项 P95 ≤300 ms（本地）、≤1.5 s（远端正常网络）。
- UI 每帧预算 16.7 ms；目录 I/O、排序和 diff 不在 Flutter UI isolate 阻塞。
- watcher burst 10,000 events 时内存增量 ≤100 MB，合并后 UI 更新 ≤20 次/秒。
- Workspace 恢复：本地 P95 ≤1.5 s；SSH 在认证已缓存时 P95 ≤5 s。
- 任何列表请求可取消；Mount 关闭后 2 s 内停止新 I/O，10 s 内完成或强制释放。
- credential、bearer URL、SSH private key 不进入日志、Collab、DSH event 或 `.muse` 文件。
- 资源写入冲突保护覆盖率 100%；无 revision 的 Provider 必须声明 `unsafe-write` 并要求确认。

## 7. 成功指标

- 30 天内使用过非 AppFlowy 页面资源的活跃 Workspace 比例。
- 每 Workspace 的 Mount 类型分布和多根使用率。
- Explorer 操作成功率、Provider reconnect 成功率、fallback 率。
- DSH 产生文件后在 Host 成功打开率、打开延迟和 scope 拒绝率。
- 插件贡献动作调用成功率和 policy 拒绝率。
- 既有 AppFlowy 协作页面用户的留存与回归缺陷数。

