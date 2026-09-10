# E4 Document 面 — 设计 / 实现 / 测试

> 阶段：E4。依赖 E1（HybridLive 之后才 contribute）与 E3（`/api/muse` 入口是 BFF）。合同：[HOST-EMBEDDING-DESIGN.zh-CN.md](../HOST-EMBEDDING-DESIGN.zh-CN.md) §4.5 / D1–D5。诊断：[HOST-EMBEDDING-ANALYSIS.zh-CN.md](../HOST-EMBEDDING-ANALYSIS.zh-CN.md) §4.5。
> 不管池执行器、不部署共享 `/dsh/`、不把 JWT 注入实例。

---

## 1. 设计

### 1.1 问题

工作区 **pin 成功** 之后，Agent 仍读不到 Host 页面：

- DSH cwd（`bindHostWorkspace`）只有 README.md — **预期**，那是 scratch。
- `GET /muse/v1/workspace/views` → `workspace.tree.query` → `POST https://openmuseai.com/api/muse/workspace/tree`。
- `muse_document_read_current` → `document.current.query` → `POST …/api/muse/document/query`。
- 生产 BFF 未实现这两条，catch-all `{code:1, message:NOT_FOUND}`。
- Host 上行的 `workspace.tree.ui` 只有 `expandedViewIds`，不是页面目录。
- 无焦点时本应 `NO_CURRENT_SELECTION`，被 404 盖住。
- bash 沙箱失败与 folder 无关，却被模型写成「工作区不可用」。

Muse 补丁 Cloud（`muse.rs`）里 tree/query **已经写过**，但现网官方 0.16.5 没有这些路由；设计已规定 BFF 是 `/api/muse` 的长期形态，所以缺口在 BFF + Host 投影，不在「再切 Cloud 镜像当补丁」。

### 1.2 决策

DocumentPort 两层（**目录面落在 `@muse/plugin-appflowy-workspace`，不是 Attachment SM**）：

```text
P0  Host 投影（不碰 collab 存储）
    HybridLive contribute:
      workspace.catalog.v1   { workspaceId, items: [{viewId,title,layout,parentViewId?}] }  ≤64 items
      markdown.snapshot.v1   { viewId, text, truncated, byteLength }  ≤32KB，仅当前页
    DSH 工具：
      tree.query  → 若 Cloud 501 则读 last catalog
      document.current.query → 无 viewId: NO_CURRENT_SELECTION
                            → Cloud 501: 读 last snapshot（须 viewId 匹配）
                            → 否则 CLOUD_COLLAB_ADAPTER_NOT_WIRED

P1  Canonical collab（BFF 实现或反代 Muse Cloud）
    POST /api/muse/workspace/tree
    POST /api/muse/document/query
    鉴权：device token → actor → 成员校验（已有 workspace/current）
    propose/apply 仍可保持 501，直到 APPLY_ENABLED
```

BFF **立即**（P0 的一部分）：对已知未接线路径返回 501，body `{code:1067, message:"UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED"}`。禁止 404 `NOT_FOUND`。

系统提示：DSH cwd ≠ AppFlowy folder；列页面用 workspace 工具，不用 bash。

### 1.3 不变量

DESIGN D1–D5。另：

| ID | 陈述 |
|---|---|
| E4-B1 | catalog / snapshot contribute 不含 token、不含文档全文以外的密钥子串 |
| E4-B2 | 模型不得向 `muse_document_read_current` 传入 viewId |
| E4-B3 | `ls` 成功且仅 README.md 不得被 Host UI 标为 Document 失败 |

### 1.4 不做

- 不为列目录重启 Docker `muse-dsh` 或官方 Cloud「补 /api/muse」。
- 不把 GoTrue JWT 写入实例 env。
- 不在 P0 实现 apply。
- 不把 Desktop UDS 改成走 BFF。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `dsh-pool/src/muse-bff.ts` | tree / document/* 未实现 → 501 1067；单测禁止 NOT_FOUND |
| `plugins/appflowy-workspace/src/catalog.ts` | Host `workspace.catalog` 记忆与 tree 投影 |
| `plugins/appflowy-workspace/src/host.ts` / `views.ts` | Cloud tree 未接线时 fallback catalog |
| `frontend/web/.../dsh-hybrid.ts` | `buildWorkspaceCatalogEnvelope`；可选当前页 snapshot（从已打开 editor，有界） |
| `DshAgentPanel.tsx` | HybridLive 投递 catalog（从 AppFlowy folder store 取标题列表，禁止整棵 CRDT） |
| `plugins/dsh-appflowy/src/parent-bridge.ts` | contribute 时 `rememberWorkspaceCatalogFromEnvelope`（inbox 未知类型也记住） |
| `plugins/appflowy-markdown/src/host.ts` | 无 viewId → `NO_CURRENT_SELECTION`；Cloud 未接线 → snapshot 或 `CLOUD_COLLAB_ADAPTER_NOT_WIRED` |
| DSH system prompt / tool description | cwd 是 scratch |
| P1（后） | BFF 读 Cloud folder collab / 或 nginx 把 tree+query 指到 Muse 补丁进程 |

Web catalog 数据源：AppFlowy-Web 已有 folder 视图缓存（侧栏）。只投影 `viewId/title/layout/parentViewId`，截断 64。不要把整份 collab JSON 塞进 postMessage。

---

## 3. 测试矩阵

| ID | 场景 | 期望 | 自动化 |
|---|---|---|---|
| E4-T1 | BFF `POST /api/muse/workspace/tree` 未接线 | HTTP 501，message 含 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`，**不是** `NOT_FOUND` | 单测 |
| E4-T2 | BFF `POST /api/muse/document/query` 未接线 | 同 T1 | 单测 |
| E4-T3 | BFF `workspace/current` 仍 200 | 回归，pin 不被 501 误伤 | 单测 |
| E4-T4 | 无 documentFocus 时 read | `NO_CURRENT_SELECTION` | 单测 markdown host |
| E4-T5 | 有 catalog contribute、Cloud 501 | `listBoundWorkspaceViews` 200，`source:"host.catalog"` | 单测 views |
| E4-T6 | catalog 含 `access_token` 子串 | 拒绝投递 / 502 UNAVAILABLE | 单测 hybrid |
| E4-T7 | HybridLive 之前 catalog spy=0 | A3/D5 | 组件测面板 |
| E4-T8 | `ls` fixture 仅 README | 工具说明或 prompt 含 scratch；不断言空工作区 | 单测或 golden prompt |

P1 接线后另加 T9：device token + 成员 → tree 200 且 items 来自 collab。

---

## 4. 生产验收

| ID | 步骤 | 通过 |
|---|---|---|
| E4-P1 | 打开面板到 HybridLive，Agent 问「有哪些页面」 | 得到 Host 侧栏标题，或明确 `CLOUD_COLLAB_ADAPTER_NOT_WIRED`；Network 里 tree 若打到 BFF 则 501 不是 404 |
| E4-P2 | 未打开任何文档时 `muse_document_read_current` | `NO_CURRENT_SELECTION` |
| E4-P3 | 打开一页后再 read | P0：snapshot 或 501；不得 `NOT_FOUND` |
| E4-P4 | 实例盘 `ls` | 仍可只有 README.md，Agent 不得称之为「工作区没有页面」 |

排障：BFF 日志 `action=workspace.tree` / `document.query` + status；禁止把 bash sandbox 错误当成 Document 面失败。
