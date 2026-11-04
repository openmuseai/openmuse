# GenOffice PPTX 0.8.0

第三方 PowerPoint 插件，复用 GenOffice 原版 React/Konva 编辑器、Desktop session builder、PPTX 引擎及编辑事务。Desktop 和 Android 在应用内的受控 MCP Apps View 运行同一份离线 UI。Host 仅处理标准协议、资源授权和文件回写，不引用插件业务代码。

```sh
git submodule update --init --recursive
npm ci
npm run pack:desktop
npm test
node scripts/verify_web.mjs /path/Sample_ppt.pptx
```

包：`dist/genoffice-pptx-0.8.0.mcpb`。单一插件 ID：`io.openmuse/genoffice-pptx`。Android 安装时发现标准 MCP 能力并核验资源，随后本地运行，不启动 Node 或回环服务，不上传打开的 PPTX。

- [原版管线、Host 边界和保真限制](DESKTOP_PIPELINE.zh-CN.md)
- [开发、打包、上架审核、更新及人工验收](DEVELOPMENT.zh-CN.md)
- [全球化、动态配置、语言包下发与验收](GLOBALIZATION.zh-CN.md)

预置中英文 1,514 条消息；语言、主题、语言包及功能开关通过 MCP Apps Host Context 更新。文案更新不重载编辑器。插件自身不设置业务服务域名，不自行请求语言服务，跟随宿主已选择的数据地区。

0.3.1 的简化 Flutter 画布已退出本版本运行链路。当前实现覆盖原版渲染和基础编辑保存；依赖 Electron/AI/系统剪贴板的全部菜单功能尚未完整移植，商业字体缺失仍可能产生跨平台差异。
