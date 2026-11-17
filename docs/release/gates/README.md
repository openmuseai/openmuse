# 发行门禁

发行前必须通过的硬性门槛：依赖锁与 SBOM 闭包、iOS 未签名构建的可验证性。执行流水线见 [release/pipeline](../pipeline/README.zh-CN.md)，历史发行记录见 [release/runs](../runs/)。

## 文档

| 文档 | 内容 |
| --- | --- |
| [OpenMuse Distribution Lock、SBOM 与闭包门禁](DISTRIBUTION-LOCK-SBOM-CLOSURE-GATE.zh-CN.md) | 依赖锁定、SBOM 生成与闭包完整性检查 |
| [iOS Unsigned Build Gate](IOS-UNSIGNED-BUILD-GATE.zh-CN.md) | 无签名 iOS 构建的允许范围与验证方式 |

## 相关领域

- 上架流水线：[release/pipeline](../pipeline/README.zh-CN.md)
- 账号发布门槛：[identity/AUTH-RELEASE-GATE](../../identity/AUTH-RELEASE-GATE.zh-CN.md)
- Sandbox 生产安全：[sandbox/SANDBOX-PRODUCTION-SECURITY-GATE](../../sandbox/SANDBOX-PRODUCTION-SECURITY-GATE.zh-CN.md)
- 代码来源审计：[architecture/CODE-REUSE-PROVENANCE](../../architecture/CODE-REUSE-PROVENANCE.zh-CN.md)
