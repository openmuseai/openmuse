# 02-development · 需求开发与变更冻结

负责人：研发负责人。状态与证据单独保存在每次发布的 `stages/02-development.json`。

## 输入与目标

把需求映射到 Muse-Client、Muse-Plugin-Ecosystem、Muse-Server 与 Muse-WebSite 的具体变更。

前置节点：01-scope。

## 执行步骤

- 完成品牌替换，同时保留上游版权、许可证及所需源码提供义务；维护本期插件白名单。
- 首次协议页、异步恢复同意/地区、Notes/Slides 编辑保存、键盘布局、长按粘贴、缺少 Desktop 的指南跳转均有验收点。
- 固定跨仓库 commit、锁文件、工具链、原生内核和 Web bundle 的摘要；禁止以构建完成后的目录清单冒充构建时快照。
- 正式构建用 `lib/main.dart`。隔离测试应用和 QA 入口，不能把测试 target 当作普通发行包。
- 新发布从审查后的源码快照构建；修改代码后递增构建号并重新验证受影响阶段。

## 输出与验收

变更清单、源码审查、各仓库 commit/dirty 状态、依赖与工具链清单。未来正式发布要求 sourceFrozen=true；历史脏工作树案例只作事实复盘。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
