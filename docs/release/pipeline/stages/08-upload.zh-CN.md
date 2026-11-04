# 08-upload · 上传、后处理与渠道安装

负责人：发布与测试负责人。状态与证据单独保存在每次发布的 `stages/08-upload.json`。

## 输入与目标

将“交付成功”和“平台处理完成”作为两个独立状态，完成平台分发包验证。

前置节点：06-binary、07-materials。

## 执行步骤

- Apple 使用 Xcode、Transporter 或已配置官方 API 上传同一审核候选；校验目标 App ID、Team、Bundle ID 和构建号。
- 等待 Build Uploads 显示完成并能选择构建。出现 90683 等错误时保存错误，回到对应阶段修复、递增构建号并重新上传。
- 本次 Apple14 因 Notes FFI 缺陷排除，15 后处理失败，16 才处理完成；任何旧的 Upload succeeded 回执不得覆盖失败记录。
- Play 将 AAB 放内测轨道，从 Google Play 安装后确认 versionCode、installer 与分发签名，并完成文档/联动冒烟。
- TestFlight 安装并验证普通 App；开发者网页登录、Xcode 账号、测试员 Apple ID 和应用内账号是不同用途，按实际身份配置。
- 自动化上传受阻时先保留日志再使用官方替代工具；不通过反复传同一包代替分析错误。

## 输出与验收

upload receipt、processing receipt、商店构建 ID/URL、实际分发安装验收。

参考：[Apple 上传与处理](https://developer.apple.com/help/app-store-connect/manage-builds/upload-builds/)、[Google Play 发布](https://support.google.com/googleplay/android-developer/answer/9859751)。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
