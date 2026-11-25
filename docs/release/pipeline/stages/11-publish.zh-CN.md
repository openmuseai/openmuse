# 11-publish · 上架与分批发布

负责人：发布及运维负责人。状态与证据单独保存在每次发布的 `stages/11-publish.json`。

## 输入与目标

确认平台已通过后按发布方式执行，并从目标地区商店验证真实可见性。

前置节点：10-review。

## 执行步骤

- 核对获批版本、地区、价格、上线时间、服务配置及回退负责人。Apple 本次提交前最后核对的是 manual release。
- Apple 手动发布需执行发布动作；Play 本次 standard publishing、未开 managed publishing，批准后可能直接公开，不能假设还有第二次发布按钮。
- 按平台支持能力分批放量；首发/不同渠道的放量能力以当日控制台为准。
- 用商店公开链接和新用户实际安装确认上架，核对包版本、签名、首次协议、演示资源和服务区。
- 故障时暂停放量/推广，回退兼容的服务配置、禁用受影响功能或发布修复包；不要承诺商店能将客户端直接降级。
- 没有商店公开/安装证据时，不标记 live。

## 输出与验收

平台获批回执、发布动作/自动发布记录、真实商店链接、地区安装与上线监控。

参考：[Apple 发布方式](https://developer.apple.com/help/app-store-connect/manage-your-apps-availability/overview-of-publishing-your-app-on-the-app-store)、[Play 发布概览](https://google.play/business/publishingoverview/)。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
