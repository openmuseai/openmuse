# `@muse/plugin-facets` 技术说明

**路径：** `packages/core/plugin-facets`  
**角色：** 跨 Flutter / Rust / DSH 的 **领域无关 Facet envelope**。

## 定位

一个逻辑 Plugin 在三个运行时各挂 Facet。本包只定义那些 Facet 之间传递的 **稳定信封**，不定义文档/表格等业务 payload。

信封（v1）：

| 种类 | 用途 |
|---|---|
| Plugin / Facet descriptor | 组合元数据 |
| Context Contribution | UI/Domain → Agent 投影原料 |
| Domain Change | revision 通知，不是 Intent |
| Presentation Intent / result | Agent → UI（打开视图、滚动到块） |
| contract-edge | 版本兼容判定 |

业务 schema 仍由各 Plugin 拥有，用 `type + schemaDigest` 引用。

## 公共表面

- `schemas/v1/`、`fixtures/v1/`（TS/Rust/Dart 共用判定）
- `src/` AJV 校验
- `rust/` 投影；AppFlowy Dart 包 `muse_plugin_facets` 读同一套 fixture

依赖 `@muse/host-bridge` 仅用于 digest/JSON 原语，不引入运输。

## 边界

禁止出现 Markdown、EditorState、DSH session DTO。新增可选字段可以；删除或改语义必须新 major。

## 验证

```bash
cd packages/core/plugin-facets && pnpm check
```
Dart：`AppFlowy/frontend/appflowy_flutter/packages/muse_plugin_facets`。
