# `@muse/dsh-runtime` 技术说明

**路径：** `packages/core/dsh-runtime`  
**角色：** 把 Composition Plan **物化成 DSH profile 行**（Cordis 插件列表），无 AppFlowy 类型。

## 定位

`materializeDshProfile`：按 plan 中的 DSH Facet 解析 `moduleByFacet`，生成 `{ name, config }` 行，供官方 loader 加载。另有 `MuseCompositionService` / `MuseDiagnosticsService` 作为 Cordis 控制面原型。

兼容常量钉在官方 DSH `0.1.0-rc.7` / Cordis `4.0.1`。

**现状：** 桌面/Web 实际启动仍用 `dsh-appflowy` 的静态 `cordis.patch.yml`，不走本包物化。本包是 v2 控制平面预留。

## 验证

```bash
cd packages/core/dsh-runtime && pnpm check
```
