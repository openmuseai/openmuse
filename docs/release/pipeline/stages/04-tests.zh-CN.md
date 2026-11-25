# 04-tests · 功能与多端测试

负责人：测试负责人。状态与证据单独保存在每次发布的 `stages/04-tests.json`。

## 输入与目标

验证需求和真实交互，再进入正式制品构建；保留失败尝试与修复后的证据。

前置节点：02-development、03-privacy。

## 执行步骤

- 在 Mobile 运行 flutter analyze、flutter test；共享 i18n/协议、资源/插件相关模块运行受影响测试。
- Android 与 iPhone 真机覆盖新装/升级、协议及地区持久化、Notes/Slides 编辑-保存-重开、图文指南/超链接、键盘显示/收起、Tasks 对齐、长按粘贴。
- 验证 US 注册/密码登录、权限拒绝、弱网/离线；账号删除用独立可删除测试账号。
- Mobile 与 Web 绑定同一 US 审核 Desktop；验证授权工作空间、Launch samples、任务发送与 AI 同意；不覆盖用户配置的模型。
- 隔离验收应用结束后停止 QA 进程，恢复普通 App；“Test finished”冻结画面不能作为产品交互结果。
- 用例记录设备、OS、构建号、实际 target、通过/失败、时间与证据路径。不能用模拟器或 Profile 测试替代真机 Release。

## 输出与验收

自动化结果、真机与多端联动报告、缺陷清单、修复回归证据。未覆盖项保留未完成状态。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
