# Google Play 渠道

包名 `com.openmuseai.mobile`；海外现有首发范围为 US。正式上传 AAB，通过 `build-android-store.sh --lane play` 构建。当前备案证书 SHA-256 为 `7e6cefb279eeb3f73890c35ecb8e941bb7630c8a6adda9a010d1d9d997ef22c4`；签名变化需重新审查安装升级与备案关系。

1. **构建前**：核对 Play Console 当前目标 SDK、设备/ABI、原生库页对齐要求和测试资格；不将本文日期视为永久有效要求。
2. **最终包**：核对 AAB 版本号/签名；分清 upload key 与 app signing key；从 Play 内测实际安装并核对安装来源、版本及分发证书。
3. **资料**：英语名称、短/长描述、图标、feature graphic、真实截图、支持/隐私/账号删除 URL、App access、Data safety、目标受众和 IARC。一般用户与内容适合所有人不等于专门面向儿童。
4. **提交**：保存轨道草稿后，还需在 Publishing overview 明确发送审核变更，确认 Changes in review，并保存版本/轨道和日期。
5. **发布**：记录 managed publishing 是否开启。本次历史首发为 standard publishing，审核批准后可能直接公开；不要套用 Apple 的手动发布假设。

本次参考：`docs/release/overseas-2026-11-02/evidence/google-play-build14-upload.json`、`google-play-device-install14.json`、`google-play-production-submitted.txt`。旧截图与材料只按原构建 provenance 复用。

官方：[发布 App](https://support.google.com/googleplay/android-developer/answer/9859751)、[发布概览](https://google.play/business/publishingoverview/)、[用户数据](https://support.google.com/googleplay/android-developer/answer/10144311)。
