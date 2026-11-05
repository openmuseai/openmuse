# F0.3 Fixtures 与 Snapshot

## 1. 固定快照

| Fixture | 位置 | 覆盖 |
|---|---|---|
| UI intents | `dsh-client-ui-resource-open/fixtures/ui-intents.snapshot.json` | deliverable/mention/card 仅 cause 不同 |
| model-visible result | `dsh-tool-resource-present/fixtures/model-visible.snapshot.json` | Agent 安全字段白名单 |
| Presentation request/receipt | F0.1 `contract-presentation/fixtures/v2/messages.json` | wire v2 schema |

Fixture 只放 opaque ref 与安全元数据；禁止真实路径、URL bearer、docx bytes、PTY 帧或 token。

## 2. Fake Host

Fake `MuseHostServiceApi` 支持：

- compatible/incompatible descriptor；
- discover/bind payload 捕获；
- request/status/cancel invoke；
- terminal receipt；
- binding invalidation event。

Descriptor 的 schema digest 从公共 operation schema 计算，避免测试复制常量后自洽。

## 3. F0.2 Fake Resource

`resource.fake`：

| 字段 | 值 |
|---|---|
| displayName | `Architecture.docx` |
| mediaType | OOXML Word MIME |
| formatId | `office.docx` |
| revision | `rev.fake.1` |
| bytes | 3-byte synthetic payload |

该 fixture 只用于 ResourceHost `describe` 权限边界，不进入 Host Bridge envelope。

## 4. Session logs

成功 Agent snapshot 顺序：

```text
muse/resource-discovered
muse/presentation-requested
muse/presentation-terminal
turn/start
assistant/message(tool-call)
tool/result(model-safe projection)
turn/end
```

复制完整 events 作为 seed 创建新 Session，`deriveMessages()` 必须逐项相等。

## 5. 后续扩展 fixture

F0.4 增加 accepted→pending→terminal、cancel/late-ready、fallback attempt；F0.5 增加 Web
origin/audience/TTL 和 Mobile placement downgrade；F0.6 增加 1000 次 replay/soak corpus。
