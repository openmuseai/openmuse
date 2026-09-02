# `@muse/plugin-appflowy-markdown` 技术说明

**路径：** `packages/plugins/appflowy-markdown`  
**逻辑 Plugin：** `muse.appflowy.markdown`  
**合同族：** `muse.document@2`（来自 `@muse/contract-document`）

## 定位

DSH Agent Facet：把 **Host 当前选中文档** 暴露为 Tool。模型 **不能** 传入 document / view / workspace / grant id。作用域固定 `appflowy.selection=current`。

| Tool | operation | 效果 |
|---|---|---|
| `muse_document_read_current` | `document.current.query` | 读 snapshot |
| `muse_document_propose_markdown_edit` | `document.current.propose` | 预览，不写 |
| `muse_document_apply_approved_edit` | `document.current.apply` | 须 Host policy + grant |

另注册三条 Context 投影（surface / selection / viewport）到 `museContextBroker`。

写路径：Tool → Bridge invoke → Domain（本机 Rust 或 Cloud）。**不在本包写 CRDT。** Cloud apply 默认 fail-closed，除非 `MUSE_DOCUMENT_CLOUD_APPLY_ENABLED=1` 且 collab 真写成功（`src/cloud.ts` + `src/host.ts`）。

## 模块

- `src/index.ts`：definition、schema、Tool 名
- `src/dsh.ts`：`createMusePlugin` + 投影
- `src/cloud.ts` / `src/host.ts`：Cloud HTTP 与 E2E fixture（装配袋 connector 只插入 provider）
- `muse.plugin.json`：逻辑清单

## 验证

```bash
cd packages/plugins/appflowy-markdown && pnpm check
```
