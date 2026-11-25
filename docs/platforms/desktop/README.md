# Desktop

Desktop 工作台与内嵌编辑能力的实现域：三区工作台规格、自由布局、Helix 非模态编辑与 Windows 打开性能、独立工程门禁。Web 共源工作台验收见 [platforms/web](../web/README.md)。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Workbench 高保真规格](WORKBENCH-PARITY-SPEC.zh-CN.md) | 工作台布局、交互与旧产品对齐的黑盒规格 |
| [Workbench 自由布局](WORKBENCH-FREE-LAYOUT.zh-CN.md) | 面板拖拽、分栏与布局持久化设计 |
| [Helix 模态与 VS Code 式非模态编辑：引擎级设计](HELIX-NONMODAL-EDITOR-DESIGN.zh-CN.md) | Helix 内核上实现非模态编辑的架构与关键取舍 |
| [Windows Helix 打开文件性能分析](HELIX-WINDOWS-OPEN-PERF.zh-CN.md) | Windows 下打开大文件的耗时定位与优化记录 |
| [Desktop 独立工程门禁结果](DESKTOP-GATE-RESULTS.zh-CN.md) | 构建、测试、签名与依赖门禁的执行结果 |

## 相关领域

- UI / 交互验收矩阵：[platforms/web/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE](../web/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md)
- 桌面插件与 CLI：[plugins/OPENMUSE-CLI-PLUGIN-DESIGN](../../plugins/OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md)
- 发行门禁：[release/gates](../../release/gates/README.md)
