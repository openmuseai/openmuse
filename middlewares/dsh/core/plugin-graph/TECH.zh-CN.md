# `@muse/plugin-graph` 技术说明

**路径：** `packages/core/plugin-graph`  
**角色：** v2 Composition Plan：校验 manifest、在 Host 描述符上规划 Facet 激活，不执行业务。

## 定位

控制平面原语。输出 `CompositionPlanV2`（digest、accepted/rejected plugins、planned facets）。**尚未**接管 AppFlowy `FlowyRunner` 启动链。

`src/v2.ts`：`MusePluginManifestV2`、`HostDescriptorV2`、`planCompositionV2`、`validateManifestV2`。  
`src/index.ts`：运行时 `MusePluginGraph`（activate/drain，按 artifact 命名空间互斥）。

依赖 `@muse/plugin-facets` 做 descriptor 校验。不依赖 DSH、不认识 AppFlowy。

## 验证

```bash
cd packages/core/plugin-graph && pnpm check
```
