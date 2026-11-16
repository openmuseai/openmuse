# 可选代码导航服务（M1）

2026-11-17 更新：服务已由独立 [插件管理器](../code-navigation-manager/README.zh-CN.md) 接入市场安装、配置和启停；Dart、TS/JS 分别为可安装语言扩展。下面保留内核接口说明，产品当前范围以 [市场方案](../../docs/design/code-navigation-market/PRODUCT.zh-CN.md) 和验收记录为准。

独立资源宿主服务插件。默认关闭，启用后只在第一次查询时启动语言进程；不依赖 DSH 内部包、Flutter 或大模型。产品/指标/后续 SDK、设置 UI 与 Mobile 接线见 [完整方案](../../docs/design/code-navigation/PLAN.zh-CN.md)。目前不在应用默认发行装配中，不能把本包通过测试称为手机已有符号导航。

```js
import { createCodeNavigationPlugin } from '@openmuse/code-navigation-service';
import { stdioBackendFactory } from '@openmuse/code-navigation-service/stdio';

// owner 由已鉴权的资源宿主安装，实施 authorizeAndRead / assertCurrent /
// resolveLocation；profiles 只能来自宿主注册配置，不能来自远端请求。
const plugin = createCodeNavigationPlugin({
  owner,
  backendFactory: stdioBackendFactory(profiles),
});
await plugin.activate(); // 不启动 LSP
plugin.service.enable(); // 仍不启动 LSP
const result = await plugin.service.query(query, { access, signal });
await plugin.deactivate(); // 取消请求并等待实例退出
```

`authorizeAndRead` 返回受信 `{partition, workspaceKey, profile, rootUri, uri, languageId, version, text}`。partition 必须包含账号与授权代际；workspaceKey 为 canonical workspace 的不透明身份。`assertCurrent` 验证账号/代际/版本仍有效；`resolveLocation` 将语言服务器结果转换成逐个授权的统一资源引用，不能原样返回裸 URI。两端均不从 display location 推断路径。

Node ≥22.22.2（开发语言服务要求）。`npm ci && npm test` 执行状态机与真实 subprocess 协议测试；`npm run test:language` 另行执行真实 TypeScript 跨文件查询。TypeScript 与 typescript-language-server 仅 devDependency，不因安装服务自动变成产品语言集。

默认：2 个实例、8 个请求、单实例串行、5 分钟空闲回收、5 秒热查询/15 秒冷查询截止、1MiB 源文件、100 个目标位置。无持久文档 buffer、全库索引、结果缓存、后台内存采样或公开网络端口。退出、禁用和取消会停止受管语言进程；第三方程序不是沙箱，OS 资源隔离和恶意输出限制仍属发布门禁。

一个受管实例对应一个进程组，而不是保证只有一个 OS 进程：语言服务可能派生 worker。停止时检查并清理整个组。TypeScript 预设应配置 `disableAutomaticTypingAcquisition: true`、`tsserver.useSyntaxServer: 'never'` 并固定 tsserver 路径；真实测试发现默认自动语法服务在首个查询时可能只返回 import 声明，不能把该结果误认为跨文件定义。

依赖 `vscode-jsonrpc` 固定 8.2.1（MIT，Microsoft vscode-languageserver-node），仅复用标准协议传输；package-lock 固定完整性。仓库许可证适用于本插件自研实现，依赖许可证随发行 SBOM 保留。
