# `@muse/contract-document` 技术说明

**路径：** `packages/core/contract-document`  
**角色：** `muse.document@2` 合同：snapshot / proposal / apply / status / commit event。

## 定位

文档领域的 **中立合同**，不是 AppFlowy 插件。Markdown Plugin、Rust Host、Cloud、Dart golden 共用同一套 operation 名与 JSON Schema。

`DOCUMENT_CONTRACT.family === "muse.document"`。操作：`document.current.query|propose|apply`、`document.command.status`。

Rust：`rust/`；Flutter：`muse_document_contract` 读 `fixtures/v2/roundtrip.json`。

**不包含：** Tool 注册、UDS、Cloud HTTP。那些分别在 markdown Plugin（含 `cloud.ts` / `host.ts`）、host-bridge。

## 验证

```bash
cd packages/core/contract-document && pnpm check
```
