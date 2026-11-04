# 全球化接入（0.8.0）

本次将原来强制中文的离线 MCP Apps 编辑器改为宿主驱动。Desktop、Linux、Android、iOS 使用同一份离线 UI，预置英文和简体中文。首次地区选择、区域配置验签、缓存、语言资源下载由 Host 完成；插件仅接收公开的展示配置，不接收账号、令牌、服务地址或配置签名私钥。文稿仍在本机处理。插件不建立中国/美国之外的服务连接，也不自行推断数据地区。

## 配置资产与边界

| 文件 | 用途 |
| --- | --- |
| `config/globalization.json` | 默认值、离线语言、上游语言映射、参数上限、命名空间 |
| `config/package-metadata.json` | 仓库来源、稳定协议标识；协议 URI 不代表网络请求 |
| `i18n/integration.json` | 插件接入层中英文源文案 |
| `i18n/catalog.json` | 自动生成的 1,514 条消息契约，供配置后台注册 |
| `i18n/en.json`、`i18n/zh-Hans.json` | 自动生成的离线包，也是翻译包模板 |
| `web/src/presentation.ts` | 原子校验、语言回退、开关及参数快照 |
| `web/src/locale-adapter.tsx` | 构建时替换原版 i18n 入口，保留上游子模块原样 |

编辑 `integration.json` 或更新上游后运行 `npm run i18n:generate`；`build:web` 自动执行 `i18n:check`，生成文件失配会阻止构建。未改动 vendor 子模块。消息支持 `{name}` 文本插值，不执行 HTML/脚本，也不将上游包含 JSON 花括号的文案误当 ICU。新增消息 ID 或改变参数集合会改变契约哈希，需要新的插件包；已有 ID 的翻译和以下参数可在初次升级后动态下发。

## 可动态下发的配置

配置后台对应发行地区/平台的 scope，编辑 `snapshot.settings`，按已有草稿、审核、签名发布流程发布。`compatibility.requiredCapabilities` 加入 `settings.v1`。

```json
{
  "features": {
    "io.openmuse.genoffice-pptx.agent.enabled": true,
    "io.openmuse.genoffice-pptx.editing.enabled": true,
    "io.openmuse.genoffice-pptx.export.enabled": true
  },
  "parameters": {
    "io.openmuse.genoffice-pptx.historyLimit": 20,
    "io.openmuse.genoffice-pptx.maxFileBytes": 33554432,
    "io.openmuse.genoffice-pptx.defaultFileName": "Presentation.pptx"
  }
}
```

| 配置 | 默认及边界 | 生效方式 |
| --- | --- | --- |
| `agent.enabled` | `true` | 即时隐藏入口并拦截后续 Agent 请求 |
| `editing.enabled` | `true` | 拦截后续编辑、新建、撤销及重做，包括 MCP 工具调用 |
| `export.enabled` | `true` | 拦截后续保存/导出，包括 MCP 工具调用 |
| `historyLimit` | 30；整数 1–30 | 即时裁剪最早的撤销/重做记录 |
| `maxFileBytes` | 64 MiB；整数 1 KiB–64 MiB | 下次打开文稿时校验；不清除已打开文稿 |
| `defaultFileName` | `Presentation.pptx`；最多 180 字符、安全 `.pptx` 文件名 | 新建文稿使用；不改已打开文件名 |

开关是产品行为控制，不能替代 Host 的资源授权或服务端权限。新增业务行为仍需要实现和发版。配置不是可执行代码；不支持远程突破包内安全上限。缺省字段恢复包内默认，类型/范围错误则整批保留上一份有效展示配置。

## 语言包发布

1. 配置控制平台启动时，通过 `OPENMUSE_CONFIG_PLUGIN_REGISTRY_FILES` 注册本插件 `i18n/catalog.json` 的绝对路径（JSON 数组）。这是服务端受信契约文件，不由客户端上传替换。平台支持同时注册多个插件，不在 Host 中导入插件业务代码。
2. 从 `i18n/en.json` 拷贝翻译模板，设置目标 `locale`，保留 `namespace`、`sourceRevision`、`contractHash`。只需包含需要覆盖的消息；缺项逐条回退英文。中英文也能使用远程覆盖。
3. 如宿主尚未启用该语言，在相同 scope 的 `localization.enabledLocales` 与 `localeCatalog` 中添加语言元数据，并加入 `messages.dynamic.v1` 能力；同时提供 Host 自己的翻译包。插件已有的 21 种语言映射保存在配置数据中；签名目录还可新增其他 BCP 47 语言，不需要重编译插件。
4. 将插件包加入草稿 `packs`，加入 `plugin-messages.v1` 能力。后台检查命名空间、消息 ID、参数集合、契约版本及 512 KiB 包上限，然后审核、签名发布。可以在消息编辑页选择插件命名空间。
5. 发布器生成 `kind: pluginMessages`、`delivery: onDemand` 的签名资源引用，包含 SHA-256、大小及受信 URL。Host 在语言选择或插件打开时下载当前语言包，校验后缓存；再次离线打开可复用。

例：部分法语包（契约哈希从生成的 catalog 读取，不能使用占位值）：

```json
{
  "schemaVersion": 1,
  "sourceRevision": 1,
  "contractHash": "cf91153db9b78f4353a98063060855684f91d55c5e410a7de0198f74d489f823",
  "locale": "fr",
  "namespace": "plugin.genoffice_pptx",
  "messages": {
    "plugin.genoffice_pptx.ui.ribbonTabHome": "Accueil",
    "plugin.genoffice_pptx.integration.agent": "Assistant"
  }
}
```

下载失败时继续使用离线/上一份已验证资源，后续打开或宿主刷新可重试。不兼容消息契约会拒绝整批更新，并显示本地化提示。简体、繁体不互相误配；没有对应繁体包时回退英文。RTL 语言设置文档方向和格式化语言，不改写幻灯片本身的内容、段落方向或坐标；复杂 RTL 文稿和商业字体仍需地区设备验收。

## Host 协议

初始化和更新均使用标准 MCP Apps Host Context：`locale`、`theme` 与扩展 `openmuse: {schemaVersion: 1, settings, localeCatalog, languagePacks}`。Host 通过 `ui/notifications/host-context-changed` 推送，插件经 MCP SDK 接收，不接受任意窗口传来的配置消息。语言包在 Host 传输层验签/验哈希，插件再次验证自己拥有的消息契约。

本次同步修改 Muse-Client 的共享 MCP Host 桥和展示上下文适配层，移除桥内固定中文。Desktop/Linux 原生 View、Android PlatformView、iOS WebView 都更新现有连接，不通过重建 WebView 或重开文稿切换语言。更新保留当前文稿、页码、选区和撤销记录（显式降低历史上限时会裁剪记录）。若旧 Host 未提供语言则使用包内默认英文；只有升级后的 Host 才能下发新配置。

MCP server 暴露 `i18n://genoffice-pptx/catalog` 契约资源；MCPB 还包含独立的 `i18n/*.json` 和配置数据。服务端协议错误文本、上架描述及原始文稿内容不作为 UI 翻译处理。已退出主链路的 Flutter 只读预览改用同源生成消息，也允许调用方传入已验证的消息覆盖；遗留 A2UI 示例生成英文和简体中文版本，未重新接回运行链路。

## 本地验证

```sh
npm test
npm run pack:desktop
node scripts/serve_globalization_preview.mjs
cd flutter_renderer && flutter test
```

预览仅绑定随机本机回环端口，使用仓库测试 PPTX，不涉及账号和外部服务。可验证：编辑后切换中英/法语/阿拉伯语，覆盖英文文案，拒绝错误参数包，关闭编辑/导出，再开启后保存。保存产物在 `dist/globalization-preview/`，报告检查真实 PPTX 页数与修改文本。法语/阿拉伯语演示包只翻译少量按钮，用于证明动态下发，不能代表完整翻译质量。

本次验证覆盖插件构建与真实引擎回归、运行中中英/法语/RTL 切换、错误包回退、远程开关、真实 PPTX 保存，以及 Node 签名发布 → Dart 按需下载/损坏包拒绝/离线恢复。原生桌面和移动桥做静态检查；设备商店分发和各平台真机验收仍需发布流程完成。
