# 可安装的代码导航插件管理器

资源宿主安装和配置控制面，不依赖 DSH、Flutter、模型或公开网络服务。产品流程、协议及边界见 [产品规格](../../docs/design/code-navigation-market/PRODUCT.zh-CN.md) 和 [架构](../../docs/design/code-navigation-market/ARCHITECTURE.zh-CN.md)。

`NavigationPluginManager({directory, artifactsRoot, partition})` 仅由受信 Host 构造；RPC 不能选择这些路径或账号。管理三项 curated market artifact：代码导航服务、Dart 语言 profile、TS/JS profile 和固定 LS。安装实际复制并逐文件验证独立交付物，服务内核只从已安装目录加载。

```js
const manager = new NavigationPluginManager({ directory, artifactsRoot, partition });
await manager.manage('install', 'com.openmuse.code-navigation');
await manager.manage('enable', 'com.openmuse.code-navigation'); // 零 LS 进程
// 语言插件：install → configure(expectedRevision, values) → detect → enable。
// 资源 owner 再为 authoritative workspace 显式开启，首次 query 才运行。
await manager.stop();
```

配置/关闭/卸载按串行生命周期完成，停止失败不写成功 receipt。配置使用 revision 检查；账号各自独立 registry。语言卸载不会删除外部 Dart SDK 或工作区。服务有已安装语言依赖时禁止卸载。

发行物由 `scripts/stage-code-navigation-artifacts.py --write-archives --out ...` 从锁定依赖生成。正常打包用 `--from-archives` 验证归档和每个文件，不需要安装开发语言工具或执行 npm hooks。catalog 信任根随正常应用签名覆盖；这不是公共 registry 的发布工具。

测试：先恢复 `third_party/dsh/navigation-artifacts`，再设置 `OPENMUSE_TEST_DART` 指向已有 SDK 并运行 `npm test`。真实检测执行 initialize + definition 后清理；与双端 UI 验收分别记录。Node 最低要求 22.22.2。
