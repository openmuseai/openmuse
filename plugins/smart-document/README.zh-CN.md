# 智能文档资源插件（第一阶段实验入口）

实现公开 SDK 的资源 creator/handler/context 贡献，使用 `.musedoc` / `application/vnd.openmuse.smart-document+json`；内容引擎与编辑面位于 `Muse-Plugin-Ecosystem/plugins/markdown/smart_document`。

宿主注入受权的 `providerFactory` 时可接原 AppFlowy-Cloud；默认本机模式。资源字节只保存 objectId/docState，不写入账号、Cloud endpoint 或 token。私有缓存按资源/账号身份隔离，历史快照不自动接最新 Cloud。

本阶段通过独立的真实 SDK workbench 验证，不默认改动用户生产 Host 的发行组合。macOS native artifact 可由包 build hook 自动生成；其他平台须等产物门完成后启用。

完整运行方式和限制见 [实现说明](../../../Muse-Plugin-Ecosystem/plugins/markdown/smart_document/README.zh-CN.md)，实际结果见 [验收报告](../../../Muse-Plugin-Ecosystem/plugins/markdown/docs/smart-document/v1/17-PHASE1-ACCEPTANCE.zh-CN.md)。
