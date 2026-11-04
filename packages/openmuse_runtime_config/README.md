**OpenMuse runtime configuration**

本包实现配置契约、发行配置加载和远程签名配置 SDK。Desktop、Linux、Mobile 和 Flutter Web 已从发行快照读取首批服务、外链和会话命名空间。

四端入口现通过 `OpenMuseClientRuntime` 恢复缓存并后台刷新。语言与展示链接热更新；网络端点变化暂存到下一次建立会话，现有凭据不会复制到新端点。基础控制面、后台、签名发布和公共分发见 `services/config-control/README.md`；共享语言解释器与接入见 `packages/openmuse_i18n/README.md`。官网签名配置与 SSR 一致性接入见相邻 Muse-WebSite 仓库；账号/租户私有解析、生产基础设施和更多存量 UI 仍待推进。

**配置与构建**

`assets/deployments/` 保存部署数据。`default` 保留既有默认部署，`global-us` 对齐现有 US 商店参数，`china` 对齐原 Linux CN 构建参数。默认部署的数据区域使用 `legacy-default`，不从域名推断真实数据驻留位置。

正式多端发行优先编译 `assets/releases/china.json` / `overseas.json`，使用 `scripts/release/prepare-runtime-config.sh` 输出 `OPENMUSE_RELEASE_JSON`；地区选择与配置下发清单见 `docs/REGIONAL-APPS-RUNTIME-REMEDIATION.zh-CN.md`。以下 profile/bundle 接口保留低层 SDK 与自定义安装兼容用途。

选择兼容发行 profile：

```sh
flutter build web --release --dart-define=OPENMUSE_DEPLOYMENT_PROFILE=global-us
```

企业或其他部署使用完整 bundle；先验证并生成构建参数文件：

```sh
cd packages/openmuse_runtime_config
flutter pub get
dart run bin/compile_deployment.dart --input /absolute/path/deployment.json --output /tmp/openmuse-defines.json
```

然后在目标 Flutter 应用目录构建：

```sh
flutter build web --release --dart-define-from-file=/tmp/openmuse-defines.json
```

生成器会将 JSON 压缩为单行字符串，避免多行 dart-define 在编译器协议中被拆开。输入中不应包含秘密；所有 bundle 内容都会进入客户端。完整 schema 见 `../../contracts/runtime-config/v1/`；CLI 还执行跨字段、地址、语言回退和信任来源检查。

现有 macOS、iOS、Android 和 Linux 发行脚本接受 `OPENMUSE_DEPLOYMENT_DEFINES_FILE`，指向上述参数文件。显式 JSON 优先于 profile。服务器的 Secret 和数据库配置继续使用服务器部署系统。

原有独立 endpoint/update-channel dart-define 与原生进程环境覆盖仅在 debug 使用。生产提交完整发行 bundle，使地址、认证绑定、更新通道和会话命名空间一起受校验；发行辅助脚本将明确的通道输入合入该 bundle。

**客户端调用**

```dart
final config = (await loadDeploymentBundle()).snapshot;
final cloud = config.serviceUri(ServiceId.cloud);
final privacy = config.linkUri(LinkId.privacy, locale: selectedLocale);
final package = config.servicePath(ServiceId.pluginRegistry, ['packages', fileName]);
```

Web 的认证与 Cloud 可在 `web.servicePaths` 声明同源网关路径；`webServiceUri` 从当前页面的 origin 解析，避免套用原生 endpoint 后引入 CORS 或跨域凭据问题。应用 mount path 不用于推断认证服务路径。

原生认证存储使用 `authentication.sessionStorageKey`；Web 使用 `web.sessionStorageKey`。各部署明确指定不同 key，默认发行保留原有 key。配置 scope、认证绑定和会话 key 不能被远程候选配置改变。

`RuntimeConfigScope` 将宿主快照传给子界面；Mobile 关于页已据此解析隐私、条款、删除账号说明、备案链接和支持联系方式。缺少宿主配置的独立组件不会猜测生产链接。

**远程 SDK 的协议**

`bootstrap.discoveryUrls` 指向返回 compact JWS 的 HTTPS feed；参考服务位于 `services/config-control`。公共 JWS 不携带登录 token。`publicKeys` 是安装时信任的 `kid → unpadded base64url Ed25519 public key`，公钥长度为 32 字节。仓库 profile 尚未登记实际发布公钥或发现地址，因此不会接触未经部署的远程配置服务。

JWS protected header 严格为：

```json
{"alg":"EdDSA","typ":"openmuse.runtime-config+json","kid":"installed-key-id"}
```

签名输入使用 JWS 的原始编码字节。拒绝未知 key/算法/消息类型以及 jku、b64、crit 等未支持 header。载荷必须符合 v1 身份、build 与能力要求，并包含有效的 UTC `issuedAt`、`expiresAt`。所有服务 origin 和资源 origin 受安装时 allow-list 约束。

每个发布流严格递增 sequence；同 sequence 不能替换内容。恢复旧内容需发布新的更高 sequence。scope 包括产品、环境、发行、部署、认证绑定、数据区、平台和变体。跨 scope 的切换应重新创建受信任上下文，并由会话层处理凭据，不通过普通刷新完成。

资源通过签名 manifest 指定 URL、大小上限和 SHA-256。全部必需资源校验成功后，SDK 把 JWS 与资源放在同一缓存记录中提交，再替换活动快照。资源失败、磁盘写入失败或版本拒收不改变当前快照。

使用内存存储或原生 `FileConfigStore`。文件存储按同目录临时文件 + flush + rename 提交；每个部署/账号使用独立路径，且由一个客户端实例负责该存储。Web 入口使用 `BrowserConfigStore` 的 IndexedDB 事务；跨进程共享缓存锁和私有租户存储仍需专项接入。

`restore()` 可恢复过期但真实的配置，用于离线展示；`currentForNetwork` 对过期网络配置抛错。调用方不能直接拿 `current` 建云连接以绕过此检查。资源损坏时保留已认证的最高 sequence，避免借损坏降级到旧版本。

将专用、无账号凭据的 HTTP client 传给 `HttpConfigTransport`；不复用登录 API client。SDK 不添加 Authorization，不跟随 redirect，并限制时间和响应体积。`createConfigTransport()` 在浏览器上使用显式 `credentials: omit`、`redirect: error` 的有界流式 Fetch；不要用 XHR 的默认 cookie 行为代替它。

第一版语言解析只支持 schema 声明的 tag 子集、显式 alias 与保留脚本的地区回退，尚未接入完整 CLDR matching。它不会使新 Material/Cupertino 语言、字体或 ICU 格式数据自动出现；编译期语言开关已迁出业务代码；现有 ARB 的公开接口由生成的运行时适配器保留。

**验证命令**

```sh
flutter analyze
flutter test
dart run bin/compile_deployment.dart --input assets/deployments/global-us.json --output /tmp/openmuse-defines.json
flutter test --platform chrome test/runtime_config_browser_test.dart --dart-define-from-file=/tmp/openmuse-defines.json
```

浏览器测试使用生成的构建配置，覆盖实际编译参数、同源网关解析和 Ed25519 验签；VM 测试额外覆盖资产加载、缓存、错误资源、过期、重放、身份错配与原子激活。

仓库级 `python3 scripts/ci/check_globalization.py` 拦截新增 URL/Locale/构建语言字面量。它是增量盘点门禁，尚未替代 AST 规则；当前剩余 baseline 是保留在代码中的本地协议/隔离常量，基线更新应经过评审。


**语义路由与灰度**

可选 `navigation` 提供 `entryRouteId` 和 `routes`，每条路由包含本地 `path` 与已安装的 `handler`。目前 handler 为 `session`、`login`。重复路径、跨 origin、编码路径穿越及未知 handler 均拒收。Flutter Web 使用此映射生成入口路由，未知路径显示本地化不可用页面。

安装 bundle 可显式开启 `bootstrap.cohortRouting`。开启后，标准 `/active.jws` 发现路径使用 `/active/{0..99}.jws`；分组随机生成并与配置缓存一起持久化，公开请求不含账号或设备标识。关闭时兼容普通 compact-JWS feed。

`regulatory.appFilingNumber` 是可选展示数据；有明确配置值才显示。商店构建通过 `scripts/release/prepare-runtime-config.sh` 把发行 profile 与显式备案环境值编译为同一份受校验 bundle，不再使用构建语言或展示开关控制业务 UI。


当前原生 Cloud/relay 适配器要求服务地址是 origin，不能携带部署路径前缀；auth 可使用安全路径段组成的网关前缀。Web Cloud/auth 通过声明的同源 `servicePaths` 解析，Cloud 前缀在配对发现时保留；relay 仍要求根路径，与现有授权 cookie 和网关协议一致。发布器、SDK 与 schema 共同执行这些能力边界。

可选 `regulatory.websiteFilingNumber` 提供官网备案展示文本，与 `appFilingNumber` 一样仅作为受校验的不透明展示数据，不代表客户端核实了登记信息。

`settings.features` / `settings.parameters` 是有界的公开运行时数据，`settings.v1` 用于能力协商。`messages.dynamic.v1` 允许签名目录新增可渲染语言，`delivery: onDemand` 语言资源由 `loadLocaleResources` 验证并原子补入缓存。未下载的按需资源不阻止基础快照激活。
