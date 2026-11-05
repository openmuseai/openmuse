# F0.3 DSH Resource Presentation Seam

状态：**实现完成，待 F0 总 Gate**  
前置：F0.1 Contracts、F0.2 Host Resource Core  
后续：F0.4 Client Orchestrator + Fake Surface

本阶段把“打开任意资源”接入 DSH 的标准 Provider/Consumer 组合，不接真实
`vendors/helix`、`vendors/ioffice`、`vendors/open-file-viewer`。交付四个包：

| 角色 | 包 | 产物 |
|---|---|---|
| Service Definition | `@muse/dsh-resource-presentation` | request/status/cancel、durable journal、replay |
| Host Provider | `@muse/dsh-resource-presentation-host` | discover/bind/invoke、scope correlation、lifecycle |
| UI Consumer | `@muse/dsh-client-ui-resource-open` | deliverable/mention/card 统一 intent |
| Agent Consumer | `@muse/dsh-tool-resource-present` | recent-ref allowlist、model-safe result |

阶段文档：

- [DESIGN.md](DESIGN.md)：产品场景、端差异、协议与安全设计；
- [DEVELOPMENT.md](DEVELOPMENT.md)：实现拆分、代码位置与后续接缝；
- [FIXTURES.md](FIXTURES.md)：fixture、snapshot 与故障注入；
- [TEST-MATRIX.md](TEST-MATRIX.md)：功能/安全/生命周期/重放矩阵；
- [RESULTS.md](RESULTS.md)：实际验证结果；
- [RELEASE.md](RELEASE.md)：进入 F0.4 的 Gate、flag 与回滚。

Ontology Runtime 仍未实现；只保留事实事件和类型级扩展点。
