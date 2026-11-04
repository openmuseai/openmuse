# 多端 App 地区、动态参数与语言整改

本轮三端整改范围、内核边界和执行顺序见 [多端全球化整改计划](/Users/mac/src/openmuse-io/Muse-Client/docs/GLOBALIZATION-MULTIPLATFORM-REMEDIATION-PLAN.zh-CN.md)。Word/PDF/表格保留产品层，未 ready 内核不纳入首发；现有市场、安装和源管理能力保留。本文的市场关闭配置是开关机制示例，不代表本轮统一首发默认。

当前开放中国与美国。中国发行固定中国区；海外发行首次启动明确选择服务地区，目前仅美国可选，欧洲/亚太不携带服务地址且不可选择。这个选择先持久化，然后初始化配置、账号、更新、插件和业务服务。语言不决定数据区，地区选择不复制账号 token，也不自动迁移数据。

## 1. 配置清单与边界

| 配置项 | 代码/数据入口 | 下发后生效方式 |
|---|---|---|
| 产品、发行、市场、地区目录、签名公钥、服务/资源 allow-list | `packages/openmuse_runtime_config/assets/releases/*.json`、`InstalledRelease`、`DeploymentBundle` | 安装信任输入；构建确定，普通远程配置不能扩大信任 |
| Auth/Cloud/relay/插件仓库/更新服务 | 地区 bundle 的 `snapshot.deployment.services`，原生 `HostEndpointConfig`、`MobileEndpointConfig` | 可在安装 allow-list 内下发；活跃账号绑定不热换，下次建立进程会话生效 |
| 语义路由、法律/支持/备案链接与文本、更新通道 | `snapshot.navigation/links/support/regulatory/updates` | 展示项在线刷新；更新端点/通道随会话分代暂存 |
| 业务开关 | `snapshot.settings.features` | 在线生效；已接入插件市场、新安装入口、更新提示 |
| 配置轮询间隔 | `settings.parameters["runtime.refreshIntervalSeconds"]` | 在线重排定时器，60–1800 秒，缺省 300 秒 |
| 其他公开参数 | `snapshot.settings.parameters`，`ClientSettings.parameter<T>` | 客户端或插件已有代码按类型读取；缺失/不匹配用调用方默认值 |
| 已知消息修复、新语言目录与语言包 | `localization.localeCatalog`、签名资源 manifest | 字符串在线更新；非内置语言按需下载后激活 |
| 系统/用户语言、主题、字号、窗口与工作区布局 | 本地偏好，`OpenMuseLocaleController`、各宿主 local settings | 属于用户选择，不由普通运营配置覆盖 |
| 开发 endpoint、固定配对码、E2E 路径 | debug-only define/environment、测试注入入口 | 不用于生产跨地区切换 |
| 插件签名 key、Office 产品 key、移动市场签名入口、原生权限声明 | 发行构建/宿主安装信任与插件供应链 | 单独审查的安装能力；不能作为随意远程参数降低校验 |
| API token、设备私钥、支付与账户权益 | 安全存储及服务端授权 | 禁止进入公共配置与语言包；开关不能授予权限 |

首批已实现的开关与参数：

```json
{
  "settings": {
    "features": {
      "plugins.market.enabled": false,
      "updates.prompts.enabled": true
    },
    "parameters": {
      "runtime.refreshIntervalSeconds": 60,
      "com.example.plugin.pageSize": 20
    }
  }
}
```

快照需声明 `settings.v1` 能力。后台有两个开关和刷新间隔控件，也可编辑 JSON。草稿保存、独立审批、发布后，在线客户端按当前轮询周期拉取，全部验签与资源校验通过后原子激活；不需要再构建 App。回滚仍发布更高 sequence 的恢复版本。

客户端 `runtime.presentation` 与 `RuntimeConfigScope` 为同一展示快照；原生入口不能只在启动时把 flags 拷贝成常量。插件通过 `context.configuration` 监听变更并读取参数。开关控制新的入口/操作，不承诺取消已完成授权的在途操作。更新提示开关不取消在途安装，也不是关闭配置刷新或规避强制版本策略的通用开关。

参数契约允许 boolean/number/string，键名与数量、字符串长度受限；特定操作参数还执行类型与范围校验。公共配置不存秘密。新的参数值和已有行为开关可以不发版；新增客户端执行逻辑、原生权限、渲染引擎或信任根仍需相应安装能力。

## 2. 国内/海外构建

规范发行输入是 `assets/releases/china.json` 或 `overseas.json`。每个可用地区含独立部署快照、公钥和 allow-list；不可用地区无 bundle。当前生产发现 URL 与签名公钥仍需正式配置，禁止把本地演示密钥填入生产文件。

```sh
# 国内固定中国区
bash scripts/release/prepare-runtime-config.sh china /tmp/china-defines.json
# 海外首次选择；目前只开放美国
bash scripts/release/prepare-runtime-config.sh global-us /tmp/overseas-defines.json
# 在目标 App 目录执行
flutter build web --release --dart-define-from-file=/tmp/overseas-defines.json
```

产物包含单行 `OPENMUSE_RELEASE_JSON`。macOS、Android、iOS 与 Linux 打包入口已接到同一编译器。`OPENMUSE_RELEASE_CONFIG_FILE` 可提供另一份经审核的完整发行目录；`OPENMUSE_RELEASE_MARKET` 只能为 `china` 或 `overseas`，不匹配即拒绝。旧的完整 deployment define 可替换同一地区 bundle，不能混入另一市场。

开发环境可用 `--dart-define=OPENMUSE_RELEASE_MARKET=china|overseas` 从安装资产读取；旧的 `OPENMUSE_DEPLOYMENT_PROFILE=global-us|china` 保留兼容，并采用对应市场的选择规则。其他企业部署使用显式受信任目录，不从当前语言/IP 猜地区。

配置缓存、语言偏好与 token 继续按部署身份隔离。保存地区绑定失效、地区撤销或偏好损坏时重新选择，不静默回退到美国。当前不提供登录态跨区切换按钮；后续切换应退出账号、释放连接、重新初始化完整上下文，不能重用原 token。

Web 仍使用声明的同源 auth/Cloud/relay 网关。其反向代理必须与该站点发行目录的地区一致；语言或客户端参数无法把一个美国 BFF 变成欧洲 BFF。未来新增可用 Web 地区时还需配置区域站点跳转和服务端网关部署，不能只在列表中改成可选。

## 3. 非内置语言按需下载

中英文为离线内置语言。签名快照可增加 `messages.dynamic.v1`，通过 `localization.localeCatalog` 登记其他语言的规范 tag、自名称、方向、格式 locale、aliases、UI key，并在 `enabledLocales` 启用。语言必须是本安装 Flutter Material 支持的渲染能力，不能通过字符串包安装代码、原生翻译 delegate 或字体引擎。

```json
{
  "localeCatalog": [{
    "tag": "fr", "label": "Français", "direction": "ltr",
    "formatLocale": "fr", "aliases": ["fr-FR"], "uiKey": "fr"
  }]
}
```

后台发布包仍使用 `{schemaVersion, sourceRevision, locale, namespace, messages}`。新语言的普通消息资源由发布器标为 `delivery: "onDemand"`，不放入必需 `bundleRefs`；配置默认语言的资源必须随快照准备好。新语言必须有至少一份非空语言包，不会把仅登记的空语言当成已翻译。

- 首次选择地区：恢复该区签名缓存并尝试该区 feed，随后按用户偏好/系统语言准备需要的包。
- 设置中选择语言：仅从活动签名 manifest 指定的 HTTPS 地址下载，验 hash、namespace、消息 ID、参数与 sourceRevision 后提交同一缓存记录，再切换界面并保存偏好。
- 跟随系统：系统语言改变或新语言被发布后按同一路径补包；不会改成显式偏好。
- 下载失败：保留原界面、导航、草稿与账号，并显示可重试状态；失败内容不写入缓存。
- 重启断网：已有语言包重新验签/验摘要后恢复；缺少单条译文时按模板语言回退。

SDK 串行化配置刷新与语言补包，避免旧语言下载覆盖较新的快照。语言选择采用意图序号，慢下载不会覆盖用户后来做出的另一选择。当前资源总预算 4 MiB、单包 512 KiB、最多 32 个资源；更多语言/namespace 规模需要升级分发索引，不以移除大小限制解决。

## 4. 插件多语言适配

已增加 SDK `OpenMusePluginContext.localizations` 和 `configuration`，Desktop/Linux/Mobile 宿主注入只读适配器。原生插件订阅 Listenable，在语言/配置变化时重建显示；不自行创建另一套全局 locale，也不能通过插件语言选择改变地区。

```dart
final languages = context.localizations;
final configuration = context.configuration;
final label = languages?.message('common.save') ?? 'Save';
final pageSize = configuration?.parameter<int>(
  'com.example.plugin.pageSize', fallback: 20,
) ?? 20;
// activate: addListener；deactivate: removeListener。
```

第一方已有语言契约的插件继续使用宿主 typed strings。HTML 插件通过 `namespaceData` 接收相同的受校验消息 AST，使用现有 host-owned HTML bridge 更新标记节点；保留输入框、编辑器、用户文件名与文档正文。后续远程 WebView/MCP iframe 使用相同数据格式和带 session/origin 绑定的消息通道，不向不可信页面开放任意 host command。

第三方插件的目标协议：

1. 插件包携带 `i18n/contract.json` 与默认 en/zh-Hans 包，namespace 为 `plugin.<manifest-id>`；定义插件版本、契约 revision、允许的消息 ID 与参数。宿主只从已验证插件产物登记这个契约。
2. 外部语言覆盖同时绑定 plugin ID、版本/契约 hash、locale、namespace、包摘要；由该产品信任的发布器签名，不允许插件直接把未知 URL 注册进宿主配置。
3. 校验顺序为产物签名/授权 → 版本与契约 → 资源摘要 → 消息参数/格式 → 原子激活；各插件失败独立回退自己的默认语言，不回退整个 Host。
4. 插件展示名称、菜单、面板标题、错误码说明与权限说明使用 key；ID、命令名、权限标识和用户数据保持稳定。插件权限与账号权益始终由对应授权层裁决。

当前实现完成第一方共享消息契约、只读 SDK 接口与 HTML bridge；第三方任意插件的契约注册、按版本分发和隔离签名 scope 尚未实现，不能将陌生插件语言包直接并入 Host catalog。新增该供应链能力需单独集成和回归。

## 5. 数据边界与上线条件

地区配置是服务路由及会话隔离的一部分，不是法律合规证明。账号数据库、对象存储、AI 推理、日志/埋点、崩溃、备份、运维访问及子处理方均要按产品的数据边界落实。公开插件与更新资源可以有单独的公共分发策略，不能由界面语言推断私有数据存放位置。

GDPR 对向 EEA 外传输规定条件和保障，并非一概禁止出境；本产品可以选择更严格的区域隔离策略。依据：[EDPB 国际传输指南](https://www.edpb.europa.eu/sme/be-compliant/international-data-transfers_en)、[欧委会国际传输规则](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/rules-international-data-transfers_en)。实际适用依据与部署边界由对应业务/合规 owner 确认。

上线需先部署中国、美国各自的配置 HTTPS 分发入口，准备安装公钥与 allow-list，再构建一次带这些信任输入的新版本。此后本文列出的已接入参数、开关、资源与文案可在线签名下发，不再因改值重新发版。当前本地开发服务为参考控制面，不会自动把正式 App 连接到 loopback 或开发签名根。


## 6. 本次验证

- SDK：17 项测试通过，覆盖地区隔离、签名、缓存、重放、原子提交。
- 语言/启动/插件适配：15 项通过；包含新语言真实发布器互通、按需下载失败保留原语言、离线恢复、系统语言跟随，以及持久化失败不启动业务服务。
- 移动 App：68 项通过；新增已挂载界面动态开关回归，验证开关变化不丢失输入草稿。
- 配置服务：10 项通过；包含动态参数范围/类型、审批签名、HTTP、灰度和恢复。
- Desktop / Linux：相关入口和工作台静态检查通过，各 11 项端点测试通过。
- Web：相关入口静态检查、3 项 VM UI 测试与 6 项 Chrome 存储/认证测试通过；浏览器专用测试在 VM 中按设计跳过。
- 移动插件安装：4 项通过，验证动态禁止安装后无需重建 store 即可重新启用，并保留校验链。
- 更新组件：3 项通过；Plugin SDK 测试及相关静态检查通过。
- 已生成国内、海外两份编译输入，校验可用地区分别只有 `cn` 与 `us`；脚本语法、文案生成与硬编码门禁通过。

本地后台演示草稿已保存并验证（市场关闭、刷新 60 秒），未提交审批或发布。没有构建商店签名二进制、修改正式签名根或部署生产服务。
