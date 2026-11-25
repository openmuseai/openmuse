# Apple App Store 渠道

Bundle ID `com.openmuseai.mobile`，App ID `6827192902`，Team `SDPXS943DW`。Apple Distribution 备案证书 SHA1 为 `12c7ff3a2a764f0442a08b9fe77ec056b81841f6`，现有 Profile 为 OpenMuse Mobile App Store 2026；每次检查有效期和最终 IPA 的真实签名。

1. **构建**：`build-ios-store.sh --market us` 使用英文与 US 服务配置，普通入口 `lib/main.dart`；同一 ASC 版本选一份构建。不要仅因换地区改变 Bundle ID。
2. **归档验证**：最终 Info.plist 的相册/麦克风/本地网络用途说明、所有 SDK 隐私要求；Notes 六个 FFI 导出；assets 无桌面原生可执行文件。实际 Release 包开关重开图文笔记，防止 Profile 正常却 Archive 被 strip。
3. **素材**：按当前支持设备/尺寸准备 iPhone 和 iPad 截图；申报年龄、隐私、内容权利、加密、价格、地区、支持与隐私 URL。只做 US 首发不能沿用含 France/EU 的默认申报答案。
4. **上传**：Xcode / Transporter / 官方 API 均需 Apple 后处理。记录每次交付和处理状态；不能用 Upload succeeded 替代 Complete。
5. **正式送审**：选已处理的正确构建，保存，添加以供审核，最终提交后保留 submission ID/确认。开发者网页登录与 Xcode 上传登录相互独立；iPhone 无需登录开发者 Apple ID 才能让构建显示。
6. **发布**：本次最后核对为手动发布。审核通过回执和实际发布/公开安装回执分别归档。

已知案例：14 的 Notes FFI 缺陷；15 缺少 NSPhotoLibraryUsageDescription 导致 90683 后处理失败；16 修复后处理完成并送审。旧构建保持 excluded，不删除失败记录。

官方：[上传](https://developer.apple.com/help/app-store-connect/manage-builds/upload-builds/)、[提交](https://developer.apple.com/help/app-store-connect/manage-submissions-to-app-review/submit-an-app)、[发布方式](https://developer.apple.com/help/app-store-connect/manage-your-apps-availability/overview-of-publishing-your-app-on-the-app-store)。
