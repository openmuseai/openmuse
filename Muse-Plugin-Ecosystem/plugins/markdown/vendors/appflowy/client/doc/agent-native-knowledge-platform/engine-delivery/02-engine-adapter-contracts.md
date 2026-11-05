# Engine Adapter 公共合同与 SDK 设计

## 1. 目标

公共合同要统一“发现、路由、会话和治理”，同时允许三类完全不同的数据面。SDK 不定义文档 AST、PTY 协议或 PDF 页面模型。

建议新增：

```text
Muse-Clients/middlewares/dsh/core/
├─ contract-resource/
├─ contract-presentation/
└─ contract-engine-session/

Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/
├─ muse_resource_contract/
├─ muse_engine_adapter/
└─ muse_surface_orchestrator/
```

Schema 是权威，TS/Rust/Dart 通过 golden fixtures 验证；opaque ID 使用 branded type。

F0 实施 ADR：业务打开事务使用 `requestRef/attemptRef`，与 Host Bridge 单次 envelope 的 `requestId` 区分；wire 时间统一为 epoch milliseconds；versioned `adapterRef` 使用 Host opaque charset 的 `adapterId~version`。下列示例已按这一权威 schema 更新。

## 2. EngineAdapterManifest

```json
{
  "manifestVersion": "muse.engine-adapter-manifest/v1",
  "adapterId": "muse.ioffice.word",
  "adapterVersion": "1.0.0",
  "engine": {
    "vendor": "ioffice",
    "engineId": "word",
    "engineVersion": "0.1.0"
  },
  "placements": ["desktop-local"],
  "platforms": [
    { "os": "macos", "arch": "arm64" }
  ],
  "formats": [
    {
      "formatId": "ooxml.word",
      "mimeTypes": [
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
      ],
      "extensionsHint": ["docx"]
    }
  ],
  "modes": ["view", "ephemeral-edit"],
  "materializations": ["bytes"],
  "contextProviders": ["page", "text-selection"],
  "risk": {
    "executesProcess": false,
    "executesActiveContent": false,
    "requiresGpu": false
  }
}
```

规则：

- extensions 是 hint，不是最终识别。
- Manifest 不能宣告当前运行可用；`probe` 负责验证产物、ABI、worker/GPU、内存和 policy。
- `edit` 与 `ephemeral-edit` 分开；无法导出保存的 ioffice Word 只能声明后者。
- `view` 不隐含 download/print/network。
- 每个 Adapter 版本固定其 engine/runtime digest，便于路由和审计。

## 3. Probe

输入：

```json
{
  "descriptor": {
    "resourceRef": "resource.opaque",
    "revision": "sha256:...",
    "format": { "formatId": "ooxml.word", "confidence": "verified" },
    "size": 482193,
    "security": { "classification": "internal", "activeContent": "unknown" }
  },
  "placement": {
    "kind": "desktop-local",
    "os": "macos",
    "arch": "arm64",
    "memoryBudgetBytes": 1073741824,
    "gpu": true
  },
  "requestedMode": "prefer-edit"
}
```

输出：

```json
{
  "available": true,
  "effectiveModes": ["view", "ephemeral-edit"],
  "materializationKinds": ["bytes"],
  "limits": { "maxBytes": 52428800 },
  "quality": { "fidelity": "native-partial", "startupClass": "warm-medium" },
  "warnings": ["save-unavailable"],
  "probeDigest": "sha256:..."
}
```

Probe 必须无副作用、可取消、有短 TTL 缓存。以下变化强制失效：adapter/runtime digest、Host placement、policy revision、GPU/worker/engine health。

## 4. EngineAdapter API

伪接口：

```text
probe(ProbeRequest) -> ProbeResult
open(OpenEngineSessionRequest) -> EngineSessionHandle
focus(sessionRef)
setActive(sessionRef, active)
navigate(sessionRef, AnchorHint) -> NavigateResult
getContext(sessionRef, ContextRequest) -> ContextSnapshot
requestCommit(sessionRef, CommitIntent) -> CommitReceipt
close(sessionRef, CloseReason) -> CloseReceipt
health() -> AdapterHealth
```

可选能力由 `capabilities` 声明，不以 method-not-found 猜测。read-only Adapter 的 `requestCommit` 不注册。

## 5. OpenEngineSessionRequest

```json
{
  "protocol": "muse.engine/open-session-request/v1",
  "requestRef": "preq.opaque",
  "attemptRef": "attempt.opaque",
  "resourceRef": "resource.opaque",
  "baseRevision": "sha256:...",
  "mode": "view",
  "materialization": {
    "kind": "bytes-handle",
    "handleRef": "materialization.opaque",
    "expiresAt": 1789466410000
  },
  "surface": {
    "windowRef": "window.primary",
    "placement": "main",
    "reuse": "compatible"
  },
  "anchorHint": null
}
```

Adapter 只能以 handle 请求数据，不解析 resourceRef。baseRevision 在整个 session 可见，提交时由 Host 重校验。

## 6. Session handle 与 receipt

```json
{
  "sessionRef": "engine-session.opaque",
  "surfaceInstanceRef": "surface.opaque",
  "protocol": "muse.engine/session-handle/v1",
  "adapterRef": "muse.ioffice.word~1.0.0",
  "resourceRef": "resource.opaque",
  "baseRevision": "sha256:...",
  "mode": "view",
  "generation": 1,
  "state": "ready"
}
```

Terminal receipt：

```json
{
  "protocol": "muse.presentation/receipt/v2",
  "requestRef": "preq.opaque",
  "attemptRef": "attempt.opaque",
  "result": "opened",
  "effectiveMode": "view",
  "sessionRef": "engine-session.opaque",
  "surfaceInstanceRef": "surface.opaque",
  "revision": "sha256:...",
  "warnings": [],
  "traceRef": "trace.opaque",
  "completedAt": 1789466400100
}
```

`opened | focused | fallback | unsupported | cancelled | denied | failed` 是 closed result。`fallback` 还要指出最终 Adapter/mode，不能只说主 Adapter 失败。

## 7. Materialization

| Kind | 适用 | 关键字段 |
|---|---|---|
| `bytes-handle` | ioffice、小型 Viewer | size、digest、readerRef、TTL |
| `read-file-handle` | 原生引擎只读 | path 只在同 Host Adapter 内可见、mode、TTL |
| `working-copy` | Helix/可写外部进程 | path、baseRevision、commitRef、dirty policy |
| `loopback-url` | Desktop Web Surface | URL 不进入日志、origin、range、TTL |
| `remote-url` | Web/Mobile | bearer/cookie audience、CORS/range、TTL |
| `stream-handle` | 大资源/模型 | seek/range、length、backpressure、cancel |

安全不变量：

- handle 绑定 resource、revision、consumer adapter、actor/workspace 和 mode；
- Adapter 不能把句柄转发到任意 origin/process；
- TTL 到期后现有读取按策略结束，新读取失败；
- commit 只接受 Host 签发的 writable working-copy/bytes handle；
- 清理可重入，Host 重启会回收遗留临时项。

## 8. Commit

```json
{
  "commitId": "commit.opaque",
  "sessionRef": "engine-session.opaque",
  "resourceRef": "resource.opaque",
  "expectedRevision": "sha256:...",
  "content": {
    "kind": "working-copy",
    "handleRef": "materialization.opaque",
    "digest": "sha256:..."
  },
  "intent": "user-save",
  "idempotencyKey": "idem.opaque"
}
```

结果：

```json
{
  "result": "committed",
  "commitId": "commit.opaque",
  "newRevision": "sha256:...",
  "providerReceiptRef": "receipt.opaque"
}
```

其他结果：`no-change | conflict | denied | unsupported | cancelled | failed`。Conflict 返回当前 revision 和可请求 compare 的安全引用，不返回真实路径。

提交不规定格式 mutation。ioffice 使用导出的 docx bytes；Helix 使用工作副本内容；Viewer 不注册 commit。

## 9. Context

统一 envelope：

```json
{
  "contextType": "surface.selection",
  "resourceRef": "resource.opaque",
  "revision": "sha256:...",
  "surfaceInstanceRef": "surface.opaque",
  "capturedAt": 1789466400000,
  "expiresAt": 1789466410000,
  "selector": {
    "provider": "muse.text-anchor/v1",
    "value": { "start": 120, "end": 148 }
  },
  "summary": "失败后最多自动重试三次",
  "truncated": false
}
```

限制：

- control lane 只发短状态/selection；正文使用 Resource snapshot。
- Context 带 revision、TTL、大小和 generation。
- read-only Viewer 的 page/zoom 也是事实 Context，不等于 Anchor。
- selector provider 是扩展点；本轮不创建 Ontology Anchor 实例。

## 10. Error taxonomy

| Code | Owner | Retry |
|---|---|---|
| `RESOURCE_NOT_FOUND` | Resource Host | 重新发现 |
| `RESOURCE_DENIED` | Policy | 请求权限后 |
| `REVISION_CONFLICT` | Provider | 重新比较/提交 |
| `FORMAT_MISMATCH` | Format Provider | 不重试同 Adapter |
| `ADAPTER_UNAVAILABLE` | Adapter probe | 可选 fallback |
| `ENGINE_ARTIFACT_MISSING` | Packaging | 不重试，熔断 |
| `ENGINE_START_FAILED` | Adapter/engine | 有界一次重试或 fallback |
| `MATERIALIZATION_EXPIRED` | Resource Host | 重新物化 |
| `SESSION_OWNERSHIP_CONFLICT` | Session runtime | focus/转交 |
| `SAVE_UNSUPPORTED` | Adapter | 不显示 Save |
| `ACTIVE_CONTENT_BLOCKED` | Policy/format | 安全 Viewer/下载 |
| `RESOURCE_TOO_LARGE` | Adapter limits | range/其他 Host |
| `CANCELLED` | Caller | terminal |

错误在跨进程/线边界校验；同进程 typed API 不重复做无意义 hostile validation。

## 11. Adapter contribution 生命周期

```text
plugin activate
  → register manifest
  → register probe/provider
  → health ready
plugin generation changes
  → old generation stops new opens
  → existing sessions drain/close
  → disposers run
  → registry removes contribution
```

重复 adapterId/version 是组合错误；同 format 多 Adapter 合法，由 Orchestrator 选择。卸载不得删除 Resource 或 Anchor hint。

## 12. TCK 要求

每个 Adapter 必须运行同一套：

1. manifest/schema/unknown version；
2. probe available/unavailable/timeout/cancel/cache invalidation；
3. open success/failure/fallback/duplicate/cancel；
4. ready/focus/background/close/dispose；
5. materialization TTL/revoke/consumer mismatch；
6. Context revision/TTL/truncation/generation；
7. permission revoke/Host shutdown/plugin reload；
8. commit/no-change/conflict/idempotency，仅适用于 write capability；
9. leak counter 为零；
10. Ontology Runtime 不安装时功能不受影响。
