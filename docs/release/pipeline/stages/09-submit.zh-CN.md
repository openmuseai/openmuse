# 09-submit · 正式提交审核

负责人：发布负责人。状态与证据单独保存在每次发布的 `stages/09-submit.json`。

## 输入与目标

完成控制台预检并执行正式提交；保存草稿和内测发布不等于送审。

前置节点：08-upload。

## 执行步骤

- 复核候选版本/构建号、截图、审核账号与步骤、隐私/数据安全、分级、价格和销售地区。
- Apple 选择本期已处理成功的构建并保存，然后添加以供审核并完成提交流程；记录提交成功页面和 submission ID。
- Play 在 Publishing overview 发送本期全部需要审核的变更，等待快速检查结果，确认 Changes in review。
- 记录 auto/manual/managed publishing 的实际设置，特别是首发批准后是否会自动公开。
- 如果遇到新法律协议或账号验证，交由具有权限的人员确认或完成；不把该动作埋在无人值守任务内。
- 不将控制台预检通过当作人工审核通过。当前历史单存在源码/分发验收等未闭合记录，保留事实；未来新单按 gate 严格阻断。

## 输出与验收

平台提交确认、提交/版本 ID、构建 ID、URL、观察时间、发布模式与准确截图/文本来源。

参考：[Apple 提交审核](https://developer.apple.com/help/app-store-connect/manage-submissions-to-app-review/submit-an-app)、[Play Publishing overview](https://google.play/business/publishingoverview/)。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
