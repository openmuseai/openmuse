# `@muse/host-bridge`

技术说明：[TECH.zh-CN.md](TECH.zh-CN.md)。

Muse Host Bridge v1 的领域无关合同、codec、Host Registry 与 transport reference implementation。

本 package 负责：

- `hello/discover/bind/invoke/subscribe/policy/cancel/status`；
- JSON Schema 2020-12 wire validation；
- lossless JSON、RFC 8785 JCS、domain-separated SHA-256；
- opaque ID、版本协商、descriptor/binding admission；
- BridgeError/ProviderError、receipt、cancel/status；
- TypeScript 与 Rust golden conformance。
- runtime/connection identity、deadline/cancellation、unary/stream 生命周期与有界 framing；
- in-process reference transport、Node desktop client、Rust UDS Host carrier/session；
- Host-side 有界幂等执行/replay 原语。

它不包含 Cordis Service、AppFlowy 领域 Provider、Flutter 状态、CRDT/数据库访问或任何具体领域 capability。
外部 schema URI 只能通过调用方注入的 `SchemaResolver` 解析；package 不自行联网或读取任意文件。

## 验证

```bash
pnpm install --frozen-lockfile
pnpm check
```

Rust crate 也可单独验证：

```bash
cargo test --manifest-path rust/Cargo.toml
```

协议单一事实来源位于 `schemas/v1/`。TypeScript/Rust 类型只是其语言投影，不能单独改变 wire 语义。

## DSH Cordis 适配

`@muse/host-bridge/dsh` 提供 Root 级 `ctx.museHost` Service。native launcher 必须先提供
`ctx.museHostConnector`；endpoint、nonce 和 connection token 只存在于该启动器 seam，不进入 Cordis config、Plugin
manifest 或日志。

`cordis.patch.yml` 是可组合的 DSH bundle patch，只挂载 Bridge Service，不注册任何领域 Tool/Prompt。Service 负责 hello、
runtime/session identity、deadline/AbortSignal、事件转发、断线重连和 Fiber dispose drain；领域授权仍由 Host binding/policy
处理。
