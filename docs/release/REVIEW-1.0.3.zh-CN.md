# Review 1.0.3 预发布构建指引

固定入口：`config/release/review-1.0.3.json`，GitHub tag `v1.0.3-review.2`。
初始候选 `v1.0.3-review.1` 保留原 SHA 与失败回执；第二候选补齐移动测试依赖、真实媒体读取等待和 Linux 容器打包配置，不移动旧 tag。
Client MR #6 与 Review 插件 MR #2 分别保留，tag 固定客户端提交，配置另行固定插件的完整 SHA。
这是 GitHub 预发布候选。Desktop Word/PDF/Spreadsheet 引擎来源按用户要求延期，相关平台失败如实保留；不代表已完成全端正式发行或商店提交。

## 干净源码和生成资产

使用 `.github/actions/checkout-plugin-sources` 检出 Client 固定的 Ecosystem 与 15 个 Git 子模块；token 来自仓库 secret `MUSE_PLUGIN_READ_TOKEN`。
本机凭据只用于授权 GitHub 操作，不提交 `.account`、token、账号或本地路径覆盖文件。
目录必须是并列的 `Muse-Client` 和 `Muse-Plugin-Ecosystem`，不能使用原有脏目录替代固定源码验证。

```sh
python3 scripts/ci/verify_plugin_sources.py
rustup target add wasm32-unknown-unknown
cargo install --locked --version 0.2.128 wasm-bindgen-cli
python3 scripts/ci/prepare_plugin_assets.py
```

Web 构建前生成 realtime、document、dataset 三组真实 WASM 和加载器，不用空目录绕过 Flutter 资产检查。
Web 锁文件统一使用 `https://pub.dev`；构建脚本固定该 hosted 源并严格核对版本和包摘要，避免构建机默认镜像差异触发重新解析。
Linux 另外需要 runtime：按 markdown 插件 `upstream.json` 中的仓库及 SHA 检出 SiYuan 到 `plugins/markdown/vendors/siyuan`，安装 Node 24.16.0 和 pnpm 12.6.0，再运行：

```sh
python3 scripts/ci/prepare_plugin_assets.py --runtime
python3 scripts/ci/prepare_plugin_assets.py --runtime --verify-only
```

构建器只修改插件忽略的产物目录，保留固定上游源码；runtime.zip 必须包含真实页面资源、LICENSE、THIRD_PARTY_NOTICES、upstream.json 和覆盖全部内容的摘要目录。
生成资产摘要写到 `dist/plugin-assets.json`，GitHub Actions 归档该文件。

Mobile 的 Android/iOS 发布门禁安装 `services/config-control` 的锁定 npm 测试依赖（包括 linkedom），使用 Node 24.16.0；普通客户端门禁复用同一依赖。真实 loopback 媒体测试显式等待 HTTP Future，保留原有内容、图片和授权断言。
Linux 容器仅信任经过外部固定源码校验的 `/Muse-Client` 挂载目录，供打包记录源码 SHA；不设置通配信任目录。runtime 配置编译器也固定 pub.dev 并严格校验锁文件。

## Web 验证和构建

```sh
python3 scripts/ci/test_verify_plugin_sources.py
python3 scripts/ci/test_prepare_plugin_assets.py
python3 scripts/release/test_prepare_web_config.py
./scripts/ci/build-web.sh --region cn --force-rebuild
./scripts/ci/build-web.sh --region us --force-rebuild
```

两次构建都重新生成 DSH 源码，执行前端测试、Flutter analyze/test、Chrome 浏览器适配器测试，再生成地区固定的发布 ZIP、SHA256SUMS 与 build-info。
cn 使用 `/`，us 使用 `/app/`；浏览器验证应使用对应 base path 和固定服务地区，登录/配对走同账号真实 Desktop。
Review Web UI 与 Desktop 使用相同插件提交：检查仓库先行的来源层级、分支/MR/commit、文件树、上一/下一文件、字号、@@ 展开和上下收起、行号输入及行内意见。开发浏览器适配器与正式配对宿主的验收结果分别记录。
现有 Web 工作台可以登录、配对并浏览 Desktop 工作区及使用 DSH，但尚缺通用 MCP Apps 直达入口。独立 Review UI 的浏览器一致性验证不能替代从 Web 工作台打开 Review 的端到端验收；是否追加该宿主适配仍待范围确认。

## GitHub 构建与发布

先推开发分支并检查 MR #6 的当前 HEAD。可在 Actions 手动触发 `web-build.yml`，ref 选择该开发分支，`skip_tests=false`、`force_rebuild=true`。
验证候选成功后创建不可移动的上述 tag；推送 `v*` tag 会触发已有 Web、macOS、Windows、Android、iOS 发布工作流。Linux 单独通过 `linux-build.yml` 分发构建。
不得跳过测试或把失败平台标记为成功。iOS 未签名包、Android 内部分发签名与商店正式签名分别保留原有边界。

从同一 tag 的成功构建下载产物，核对 build-info 的 commit、地区、ZIP 根目录、所有 SHA256；发布 GitHub prerelease，附 cn/us Web 包、校验清单、固定 Review 插件包和真实测试/失败平台报告。不要上传其他 HEAD 的产物。
GitHub Release 不自动部署 Web 站点、不合并 MR，也不提交应用商店。回滚时使用上一已验证 tag 与对应地区包，不移动已经发布的 tag。
