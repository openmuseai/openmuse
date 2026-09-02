# `@muse/plugin-appflowy-view-reference` 技术说明

**路径：** `packages/plugins/appflowy-view-reference`  
**逻辑 Plugin：** `muse.appflowy.view-reference`

## 定位

只读：当前 Host 选中 View 的 **有界、脱敏结构元数据**。模型不能选 workspace/view id。scope hint：`appflowy.selection=current`。

不 import AppFlowy、CRDT、FS。invoke 全部经 `museHost`。

主要用于 Agent 知道「现在在哪一页」而不打开整棵树。列页面属于 **workspace** Plugin，不在本包。

## 验证

```bash
cd packages/plugins/appflowy-view-reference && pnpm check
```
