# Remote DSH Gateway 生产形态

契约原文：[REMOTE_CONTRACT.md](../../development-plan-v2/06-remote-mobile-web-runtime/REMOTE_CONTRACT.md)。本文只描述 **如何接到 AppFlowy-Cloud 部署**，不改合同语义。

## 1. 身份

`Session` ≠ `Device` ≠ `Attachment` ≠ TCP 连接。Web 浏览器标签和 Android 设备是不同 `deviceId`。同一用户可同时 attach。

## 2. 生产认证

V2-06 的 HMAC Authority 是协议参考实现。生产必须：

- 由 Cloud BFF 签发（校验 GoTrue JWT 之后）；
- 持久化撤销（Redis 或 postgres）；
- `kid` 轮换；
- token 不含模型 key。

nginx 不把 Cloud JWT 原样转给 DSH 当模型凭据。

## 3. 传输

| Lane | 承载 | 浏览器 / Android |
|---|---|---|
| state | 有序领域投影 | WSS 或 SSE + cursor ack |
| receipt | Tool 结果 | 同 state 恢复规则 |
| control | selection/focus TTL；Web 亦可经 DSH Client postMessage 同构 | 可丢 |

Web 的 workspace bind / context / Intent 优先走 DSH Web Client（V2-09 W4–W6）。Gateway control lane 仅当 Client 无公开 seam 时作为 W5 备选。领域 apply 不走 control。

公网只走 HTTPS/WSS；DSH 容器不映射到 `0.0.0.0:3080`。

## 4. Cloud nginx 计划路径

| 路径 | 后端 | 阶段 |
|---|---|---|
| `https://dsh.<domain>/` | muse-dsh:3080 | D1 |
| `https://<domain>/api/muse/dsh/device-token` | Cloud BFF | W2（已实现签发/校验/撤销） |
| `https://<domain>/api/muse/document/*` | Cloud Domain（当前 fail-closed） | W3 |
| `wss://dsh.<domain>/muse/v2` | Gateway | A2 / 后续 |

未实现的路径不得写进「已部署」清单。

## 5. Composition

Gateway 与 Planner 对 Android/Web 返回的 plan **必须**拒绝 `dsh-native`。Desktop 仍可 local-node。同一 Plugin ID、同一 Agent digest。
