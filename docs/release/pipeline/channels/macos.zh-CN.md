# macOS 双渠道发行

核对日期：2026-11-04。执行实例：[2026-11 macOS](../../runs/2026-11-macos/README.zh-CN.md)。
使用相同的 12 阶段，分别记录 `macos-store-us` 和 `macos-direct-us`，不能共用签名包或测试结论。

| 要求 | Mac App Store | 官网直发 |
| --- | --- | --- |
| 身份 | macOS App ID、Store profile、适用的 Store 应用签名；以 Xcode 导出验证为准 | Developer ID Application；PKG 另需 Installer 身份 |
| 隔离 | App Sandbox 必须启用；选择目录后的持久授权需 security-scoped bookmarks 验证 | 验证实际权限与 Hardened Runtime，按 helper 需要给最小例外 |
| 更新 | 只通过 Mac App Store；移除自更新检查、安装器及入口 | 可保留更新器；下载包校验、签名身份、版本检查与回滚均需验收 |
| 插件及子进程 | 审查下载执行代码、Node/CLI/Helix、JIT、后台任务；先定能力白名单 | 所有嵌套 Mach-O/框架/helper 都需签名；验证动态库和进程退出 |
| 制品 | Xcode Archive/Store 导出制品，上传 ASC 并等待处理 | 经公证、装订并验证 Gatekeeper 的 DMG/应用，以及最终哈希 |
| 发布 | ASC 手动发布，保留审核及公开安装回执 | 下载页及更新 feed 手动切换，保留下载验证及回滚目标 |

## 按阶段执行

1. **01–02**：确认 US/英文范围、价格、最低 OS、CPU 架构、Bundle ID 和商店记录。当前候选 `com.openmuseai.office`，不能直接复用移动端 `com.openmuseai.mobile` 的记录或 profile。冻结 Client、插件生态、ioffice 和关联服务；现有脏工作树不能作正式构建来源。
2. **03**：盘点真实链接的 SDK/privacy manifests/required-reason APIs、麦克风及本地网络等权限。用到才添加准确用途说明；验证首次同意、拒绝路径、删除账号、AI 共享及政策与网络数据路由。Store 首次许可界面还要核对 2.4.5(vi)。
3. **04**：执行 Host analyze/test、受影响 Rust/插件测试；在真正签名 Release 上覆盖新装、升级、离线、文件选择、退出后进程清理、Notes/Slides 保存重开、账号及 AI 流程。Store 额外验证沙盒文件授权的跨重启持久性。测试全部声明支持的架构与 OS。
4. **05–06**：直发可复用 `scripts/package_macos.sh` 构建基础包，但它当前使用 ad-hoc 签名，**不是正式分发入口**。所有资源注入完成后，从最内层代码到外层应用使用正确身份签名；不能用 `codesign --deep --force` 当作签名策略。Store 需要独立沙盒配置、能力裁剪和 Xcode Archive/导出路径，不能只把 Release.entitlements 的 false 改为 true。
5. **07**：真实 Mac 截图 1–10 张，16:10，Apple 当前接受 1280×800、1440×900、2560×1600、2880×1800。逐图记录来源构建与 SHA-256；准备图标、英文文案、审核步骤、隐私/支持 URL。官网文案只描述直发版实测能力。
6. **08–12**：Store 上传后等待 Complete，再提审、跟进、手动发布；直发记录 notarytool Accepted、装订及 Gatekeeper 验证，再发布下载链接和 feed。两边都做公开下载/安装验证、观察和归档。

## 本地准备检查

```sh
python3 scripts/release/preflight-macos.py --app dist/OpenMuse.app \
  --report docs/release/runs/2026-11-macos/evidence/preflight-macos.json
python3 scripts/release/pipeline.py check --run docs/release/runs/2026-11-macos
python3 scripts/release/pipeline.py check --run docs/release/runs/2026-11-macos --gate 08-upload
```

预检是只读诊断，发现阻断返回 1；没有发现也仍需人工及签名包验收。源码字符串触发项是复核提示，不能证明功能可达性或审核结论。

现有构建脚本仅供基础包开发验证（**执行前先通过 05-build 关口**）：

```sh
OPENMUSE_RELEASE_MARKET=overseas OPENMUSE_DEPLOYMENT_PROFILE=us \
  bash scripts/package_macos.sh --profile release --components none \
  --build-name 1.0.0 --build-number "$MACOS_BUILD"
```

构建号由商店历史分配，`--components none` 不代表本期功能已确定。默认包仍带桌面运行时，必须盘点最终闭包。

## 直发签名后的公证及 DMG

以下是已签名、已验收应用的操作模板，不负责签名，也没有在本次准备中执行。
钥匙串凭据由负责人配置；不要把账号密码写入命令、仓库或普通日志。
`SIGNED_APP`、`DMG_STAGE`、`DMG_PATH` 和 `NOTARY_PROFILE` 必须指向本次候选；暂存目录先由负责人准备，包含应用和 Applications 链接。

```sh
codesign --verify --deep --strict --verbose=2 "$SIGNED_APP"
hdiutil create -volname OpenMuse -srcfolder "$DMG_STAGE" -ov -format UDZO "$DMG_PATH"
xcrun notarytool submit "$DMG_PATH" --keychain-profile "$NOTARY_PROFILE" --wait --output-format json
# 仅在 Accepted 后继续；失败则保存 submission ID 并取回 notarytool log。
xcrun stapler staple "$DMG_PATH"
xcrun stapler validate "$DMG_PATH"
spctl --assess --type open --context context:primary-signature --verbose=2 "$DMG_PATH"
shasum -a 256 "$DMG_PATH"
```

从最终 DMG 安装到干净 Mac 后再运行 `spctl --assess --type execute` 检查实际应用。
保留在线/离线首次启动证据；本机开发目录内启动不等于 Gatekeeper 安装验收。

## 构建回执字段

沿用 `buildRecord` 的 `status=store-candidate`、`dirtySource=false`、`testsPassed=true`、最终制品 `sha256`。
这里的 `store-candidate` 是现有 pipeline 的统一候选状态，直发也使用它。
macOS 增加 `distributionChannel`（精确 lane）、`teamId`；Store 增加 `sandboxEnabled=true`；直发增加
`signingIdentityType="Developer ID Application"`、`hardenedRuntime=true`、
`notarization={"status":"Accepted","submissionId":"真实ID","stapled":true}`。
这些字段必须引用实际签名、公证和安装报告；手填字段不是验证证据。
哈希在装订完成后重新计算。上传关口缺这些字段会保持阻断。

官方依据：[Mac 审核要求 2.4.5](https://developer.apple.com/app-store/review/guidelines/)、[截图尺寸](https://developer.apple.com/help/app-store-connect/reference/app-information/screenshot-specifications/)、[上传与处理](https://developer.apple.com/help/app-store-connect/manage-builds/upload-builds/)、[Developer ID 与公证](https://developer.apple.com/developer-id/)。
