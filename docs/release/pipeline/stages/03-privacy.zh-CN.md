# 03-privacy · 隐私、权限及政策整改

负责人：产品与隐私负责人。状态与证据单独保存在每次发布的 `stages/03-privacy.json`。

## 输入与目标

依据实际代码、SDK、网络请求和服务存储更新隐私资料，技术检查与平台申报必须一致。

前置节点：02-development。

## 执行步骤

- 检查首次同意前是否启动账号/业务网络或读取剪贴板；拒绝及保存失败不能越过同意步骤。
- 海外 App、服务条款、隐私政策、支持和删除页面使用英文；地区化语言与数据路由分别验证。
- 登录、账号删除、数据保留、AI 内容举报、向具体 AI 服务商共享前的告知与同意逐项复核。
- iOS 审核实际链接的 SDK；本次 file_picker 引入相册 API，最终 Info.plist 必须含准确的 NSPhotoLibraryUsageDescription。麦克风、本地网络及 SDK 隐私清单一并核对。
- Play Data safety、Apple App Privacy、目标受众、内容分级、加密/出口与适用地区资格按当前功能和当日控制台填写。新增国家、SDK、支付或 UGC 时重新评估。

## 输出与验收

数据/SDK/权限清单、英文政策版本和 URL 快照、申报复核记录、删除与举报证据。法律适用性需由负责人员判断，脚本通过不代表全部法规结论。

参考：[Apple 用途说明](https://developer.apple.com/documentation/bundleresources/information-property-list/nsphotolibraryusagedescription)、[Apple 审核指南](https://developer.apple.com/app-store/review/guidelines/)、[Play User Data](https://support.google.com/googleplay/android-developer/answer/10144311)。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
