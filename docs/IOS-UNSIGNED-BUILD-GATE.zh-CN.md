# iOS Unsigned Build Gate

## 1. 目的与边界

M8 在没有 Apple Team、distribution certificate 和 provisioning profile 的环境中，只接受 **unsigned engineering build gate**。它证明当前 Mobile 源码能产生 iPhoneOS arm64 release bundle，并检查 bundle identity、Privacy Manifest 和禁止打包的 Desktop/runtime 内容；它不产生 IPA，也不代表 TestFlight 或 iOS 真机通过。

签名身份和 TestFlight 是发布信任链的一部分，不能用 ad-hoc 签名、模拟器或 `--no-codesign` 结果替代。

## 2. 运行方式

```bash
./scripts/test_ios_beta.sh
```

脚本依次执行 Flutter analyze/test、`flutter build ios --release --no-codesign`，然后 fail closed 检查：

- `CFBundleIdentifier == io.openmuse.mobile`；
- Runner、App.framework、Flutter.framework 是 device `arm64`；
- App 与 Flutter Privacy Manifest 可被 `plutil` 解析；
- bundle 不包含 Node、Helix、DSH closure、sandbox worker 或 `libnode`；
- App 确实没有签名，避免把该 artifact 误当成可分发构建。

证据输出到 `target/ios-beta/`：架构、plist lint、签名结果、逐文件 SHA-256、bundle 报告及 `OpenMuse-iOS-Beta-unsigned.app.zip`。

## 3. 2026-09-29 结果

| 项目 | 结果 |
|---|---|
| Flutter analyze/test | PASS |
| iPhoneOS release build | PASS，14.1 MB Runner.app |
| Bundle ID | `io.openmuse.mobile` |
| Version/build | `1.0.0 (1)` |
| Device architecture | `arm64` |
| Privacy plist | PASS |
| Desktop/runtime scan | PASS |
| 签名 | 按设计不存在 |
| unsigned zip SHA-256 | `2f7c93f7f4e0de57fce46edfb68c1a7b2061a6eb392c20304ebc331e82047de7` |

## 4. TestFlight 前剩余门禁

- 配置归属 OpenMuse 的 Apple Team、distribution certificate、App ID 与 provisioning profile；
- 使用固定版本号生成 signed archive/IPA，验证 entitlements 和签名链；
- App Store Connect 上传与自动校验通过；
- iPhone/iPad TestFlight 真机完成登录、Cloud Workspace/Remote DSH、前后台恢复和断网恢复；
- 验证动态字体、VoiceOver、picker/share/speech 以及审核隐私声明；
- 与 Android 连接态门禁使用同一套 DSH/Resource fixtures，并确认不存在运行时下载执行代码。
