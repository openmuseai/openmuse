# 账号、认证与设备身份

账号与设备身份域：登录与鉴权插件、设备目录与 Presence、Desktop 配对、跨端测试矩阵、Broker 委派与审计。所有跨设备访问都先有稳定身份，再有一次性授权，最后留审计。

## 文档

| 文档 | 内容 |
| --- | --- |
| [账号设备目录、Presence 与 Desktop 配对](ACCOUNT-DEVICE-PRESENCE-PAIRING.zh-CN.md) | 账号下的设备模型、在线状态与配对流程，域入口 |
| [设备 Presence 与多端实时同步架构](DEVICE-PRESENCE-REALTIME-ARCHITECTURE.zh-CN.md) | Presence 通道、心跳、离线判定与实时同步 |
| [OpenMuse 登录、GoTrue 鉴权插件迁移设计](AUTH-GOTRUE-PLUGIN-MIGRATION.zh-CN.md) | 认证服务的插件化与迁移路径 |
| [认证、Cloud Workspace 与 DSH 跨端测试矩阵](AUTH-CROSS-DEVICE-TEST-MATRIX.zh-CN.md) | 跨端认证与访问的测试覆盖 |
| [OpenMuse 账号流程与发布门槛](AUTH-RELEASE-GATE.zh-CN.md) | 账号相关上线前必须满足的门禁 |
| [OpenMuse Broker Delegation、Policy 与 Audit](BROKER-DELEGATION-POLICY-AUDIT.zh-CN.md) | 能力调用的委派链、策略判定与审计记录 |

## 相关领域

- 设备连接与中继：[devices-ctrl](../devices-ctrl/README.md)
- 资源授权与 Ref/Handle：[workspace/WORKSPACE-RESOURCE-AUTHORITY-V1](../workspace/WORKSPACE-RESOURCE-AUTHORITY-V1.zh-CN.md)
- 插件权限声明：[plugins/PLUGIN-MANIFEST-V2](../plugins/PLUGIN-MANIFEST-V2.zh-CN.md)
