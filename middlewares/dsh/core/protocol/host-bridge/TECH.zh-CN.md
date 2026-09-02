# `@muse/host-bridge` 技术说明

**路径：** `packages/core/protocol/host-bridge`  
**角色：** 两端共用的 **元协议**。Everything is Plugin 图里的「底层协议」就是它。

## 定位

负责 `hello / discover / bind / invoke / subscribe / event / policy / cancel / status` 的 wire 合同、codec、admission、有界 framing，以及参考运输（UDS、InProcess）。

**认：** envelope 动词、bindingId、schemaDigest、deadline、Host/Plugin 身份。  
**不认：** Markdown、viewId、iframe、WKWebView、Tool 名、AppFlowy workspace、parent-hello。

协议真源：`schemas/v1/`。TS 与 Rust 只是投影；改语义必须改 schema + golden fixture。

## 为何必须保留

没有它，Desktop UDS 与 Web Cloud invoke 会再次分叉，Plugin 也无法 discover/bind。可改的是 **用法**（入站也走同一套动词），不是删包。详见 [PLUGIN_PEER §5.1](../../../../docs/platforms/PLUGIN_PEER_ARCHITECTURE.zh-CN.md)。

## 公共表面

| 导出 | 用途 |
|---|---|
| `@muse/host-bridge` | 类型、codec、digest、limits、InProcess / Desktop 运输 |
| `@muse/host-bridge/dsh` | Cordis `MuseHostService`（`ctx.museHost`），依赖先注入的 `museHostConnector` |
| `@muse/host-bridge/testing` | 测试夹具 |
| `schemas/*` | JSON Schema |
| `cordis.patch.yml` | 只挂 Bridge Service，不注册领域 Tool |

Rust workspace：`rust/`（contract）、`host-registry`、`host-runtime`、`host-transport`、`host-policy`、`host-events`。AppFlowy `flowy-core` 以 path 依赖这些 crate，作为本机 Domain Host 的服务端。

## 关键模块

| 路径 | 职责 |
|---|---|
| `src/contract/` | envelope 类型与错误码 |
| `src/codec/` | JCS、SHA-256、schema digest |
| `src/dsh/service.ts` | Cordis 客户端：hello、事件泵、invoke、断线 |
| `src/dsh/connector.ts` | `MuseHostConnectorService` 抽象；具体打开 UDS 的是装配包 connector |
| `src/transport/` | InProcess、Node desktop UDS 客户端 |

## 边界

- 不 import `@muse/plugin-*`、不 import AppFlowy、不读任意文件路径解析外部 schema（调用方注入 `SchemaResolver`）。
- `scripts/check-boundaries.mjs` 禁止产品/领域词与 harness 私有路径。
- 运输实现留在本包是实用选择：协议包带 reference transport。业务 HTTP 门面（parent-bridge）**不许**并进来。

## 验证

```bash
cd packages/core/protocol/host-bridge && pnpm check
```
