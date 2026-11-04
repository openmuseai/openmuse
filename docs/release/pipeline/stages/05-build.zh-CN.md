# 05-build · 分渠道构建与正式签名

负责人：构建负责人。状态与证据单独保存在每次发布的 `stages/05-build.json`。

## 输入与目标

在本机/受控构建机复用现有签名脚本，产出与版本、地区、源码和证书关联的候选制品。

前置节点：04-tests。

## 执行步骤

- Google Play：`bash scripts/release/build-android-store.sh --lane play --version X.Y.Z --build-number N`，产物 AAB。
- 国内 Android：同脚本 `--lane cn`，产物 APK；只有渠道确有差异才新建构建线，并记录差异。
- iOS：`bash scripts/release/build-ios-store.sh --market us --version X.Y.Z --build-number N`；国内用 `--market cn`。同一 ASC 版本只能选定一个构建。
- 私钥保留在公司密钥库/钥匙串；沿用备案身份。Android 上传证书与 Play 用户分发证书分别验证；Apple Distribution/Profile 匹配 Team 和 Bundle ID。
- 默认不使用 --allow-dirty/--skip-tests；两者会保留 internal-test-only 标记，不能自动转成正式候选。
- 保存脚本产生的 release.json、签名报告、运行配置校验和制品 SHA-256。常规 CI 的 Alpha/未签名包不能冒充商店包。

## 输出与验收

每条 lane 独立的 APK/AAB/IPA、构建日志摘要、签名报告、配置检查和 release.json。鸿蒙未达到功能与构建门槛时不执行空壳上架。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
