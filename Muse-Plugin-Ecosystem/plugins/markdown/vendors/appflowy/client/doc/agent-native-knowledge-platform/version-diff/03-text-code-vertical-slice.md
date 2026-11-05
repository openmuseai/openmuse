# 文本、Markdown 与代码纵向切片

## 1. 范围

本期覆盖 `txt/md/markdown` 和常见代码、配置、Web 文件，包括 Rust、Dart、TypeScript、JavaScript、Java、C/C++、Go、Python、Swift、Kotlin、Shell、SQL、HTML/CSS、JSON/YAML/TOML/XML 等。

入口是已经存在的 `MuseResourceFilePlugin`。DSH 点击文件路径与 Host 手动打开文件最终进入同一资源 Tab，因此二者自动获得相同版本菜单。

## 2. 用户流程

### 2.1 保存基线

1. 在 Host 打开代码或 Markdown 文件。
2. 打开文件 Tab 菜单。
3. 选择“保存当前版本”。
4. Host 读取当前 bytes、计算 SHA-256、写入 CAS Blob 和 Version sidecar。
5. UI 显示 digest 前 8 位；审计追加 `version.capture`。

### 2.2 比较

1. 文件发生修改。
2. 选择“与上一保存版本比较”。
3. 创建不可变 working Version；它不进入历史列表。
4. 创建 Comparison，交给文本 DiffProvider。
5. Provider 产生领域 payload。
6. Host 在当前窗口打开 `filename · Diff` Tab，首屏默认基线/当前左右 Split。
7. 审计追加 `comparison.create`，记录 Actor 类型、增删改摘要和最多 80 处文本变更预览。

若没有 committed baseline，明确提示先保存，不自动猜测基线。

### 2.3 历史与审计

- “版本”页按时间倒序展示保存说明、Actor、时间与 digest。
- 任一版本可“与当前比较”。
- “审计”页展示版本捕获和比较事件。
- 比较事件显示 User / Agent / System 身份；新增为绿色、删除为红色、修改为蓝色。
- 审计卡可展开查看逐 Change 的 before/after 行；正文不进入通用协议字段，仅作为文本 Provider metadata。

## 3. 文本语义模型

通用层看到 `ResourceDiff<MuseTextDiffPayload>`，只有文本域看到：

```text
MuseTextDiffPayload
  language
  additions / deletions
  hunks[]
    stable hunk id
    old/new ranges
    semantic label
    changes[]
      stable change id
      insert/delete/replace
    rows[]
      context/deletion/insertion
      old/new line coordinates
      optional inline spans
```

Change ID 由 Resource ID、位置和变更内容 fingerprint 生成；对相同 base/target 重算保持稳定。它可被审计评论、DSH 引用和未来 Action 共同使用。

## 4. Diff 算法

### 4.1 当前算法

- 将 CRLF 归一为 LF。
- 使用 Myers shortest-edit-path 计算 line edit script。
- 相邻非 equal 操作合并为 insert/delete/replace Change。
- 以 3 行上下文聚合 Hunk；间隔大于 6 行的变化拆成不同 Hunk。
- 单行 replace 用共同前缀/后缀生成字符级 inline span。
- Markdown 向前寻找最近 heading；代码向前寻找 class/function 等近似符号作为 semantic label。

### 4.2 已知限制

- 当前 code semantic label 基于轻量正则，不是 AST。
- 纯移动在 P0 表现为 delete + insert。
- EOF newline 变化目前不单独显示。
- 超大文件尚未启用 isolate/streaming/cancellation。
- 语法高亮尚未接入 Viewer。

P1 应引入 Tree-sitter/LSP symbol provider；Stable ID 组合 syntax path、normalized signature 与 content fingerprint，并对 rename/move 做显式分类。

## 5. Viewer

### 5.1 Host 集成

- `PluginType.diff` 是不可创建的 Host 插件类型。
- Diff 使用标准 Host Tab、关闭按钮和生命周期。
- `contentPadding = EdgeInsets.zero`，Viewer 自己适应 Host 内容尺寸。
- 不启动外部窗口，不显示底层 iOffice/Helix/Viewer 名称。

### 5.2 统一视图

- 同一行显示 old/new gutter。
- 删除用 `−`，新增用 `+`；颜色只作为辅助。
- replace 的字符变化使用更深背景。

### 5.3 并排视图

- 默认进入并排视图，左侧固定为基线，右侧固定为当前版本。
- 双栏标题显示版本 digest 和 Actor，避免方向歧义。
- old 与 new 两列对齐。
- 数量不一致时补空行，不篡改行号。
- Context 同时出现在两侧。

### 5.4 导航

- 顶部显示 Change、additions、deletions 与语言。
- 上一/下一按钮滚动到 Hunk。
- Hunk 可折叠。
- 相同内容显示明确空状态。

## 6. Proposal 演进

普通 Diff Viewer 保持只读。进入 Proposal 模式后，Renderer 可以在每个 Change 上显示 Accept/Reject，但按钮只发送：

```text
ChangeDecision(proposalId, changeId, accept|reject)
```

MutationProvider 负责生成 patch、校验 current、三方应用并创建结果 Version。Viewer 不持有文件写权限。这样相同 Diff Viewer 可以用于审计查看、Agent 提案、人工比较和回滚预览。

## 7. 代码映射

| 责任 | 文件 |
|---|---|
| 六概念协议、Provider Registry | `domain/version_diff_contract.dart` |
| 文本领域 ViewModel | `text/text_diff_models.dart` |
| Myers、Hunk、stable ID、语义标签 | `text/text_diff_provider.dart` |
| CAS、Version metadata、Audit | `application/text_version_repository.dart` |
| 保存/比较用例编排 | `application/text_version_diff_service.dart` |
| Host Diff 插件 | `presentation/text_diff_plugin.dart` |
| Unified/Split UI | `presentation/text_diff_viewer.dart` |
| 历史与审计 UI | `presentation/version_history_dialog.dart` |
| Tab 菜单 contribution | `resource_surface/resource_tab_actions.dart` |

## 8. 错误处理要求

- 非 UTF-8：返回 Provider 不支持或编码选择 UI；不得用 replacement character 静默比较。P0 当前会抛出解码错误，后续转为用户态错误。
- 文件被删除：保留历史 Version，当前 working 捕获失败并提示。
- 文件读取越权：不降级为外部进程读取。
- Blob 丢失/摘要不符：阻止比较并创建完整性告警。
- sidecar 损坏：忽略损坏项，保留其他历史；后台报告修复任务。
- 无基线：提示用户保存，不自动创建并展示“无变化”。
