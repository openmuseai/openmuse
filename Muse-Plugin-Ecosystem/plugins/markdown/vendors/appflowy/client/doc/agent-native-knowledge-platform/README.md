# Muse Agent 原生知识平台：总体设计

状态：**目标方案 v1（基于 2026-09-14 工作区代码审计）**  
性质：产品与技术基线，不表示所有能力已经实现。  
初稿来源：[agent-file-references-and-open-routing.md](../agent-file-references-and-open-routing.md)

## 1. 一句话定义

Muse 不是另一个 Office，也不是在 Office 旁边增加一个聊天框。Muse 要成为一个 **Agent 原生的知识构建、浏览、传播与行动平台**：任何文件、云文档、应用视图、代码、工作流、CAD 或三维模型都通过同一套资源协议被发现、授权、呈现和修改；其中有业务意义的内容再被投影为 Ontology 对象、关系与动作，使人和 Agent 能在同一业务世界中协作。

```text
任意内容与系统
  文件 · 云文档 · 代码 · 数据库 · CAD · 3D · 应用 · Workflow
                     │
                     ▼
              Universal Resource Fabric
        身份 · 授权 · 描述 · 读取 · 承载 · 回写 · 事件
                     │
          ┌──────────┴──────────┐
          ▼                     ▼
  Surface / Engine Layer     Ontology Runtime
  ioffice · Helix · Viewer   Object · Link · Action
  Markdown · Future Apps     Claim · Anchor · Impact
          │                     │
          └──────────┬──────────┘
                     ▼
              Human + Agent Workflows
       理解 · 提议 · 审批 · 执行 · 传播 · 反馈学习
```

## 2. 北极星原则

1. **格式无特权。** Markdown、Word、代码和 STEP 文件均是普通资源；核心协议不得出现扩展名白名单。
2. **资源不等于文件。** 文件路径只是某个 Host 私有的定位方式。跨边界只传不透明 `resourceRef`，不传任意路径、数据库主键或凭据。
3. **表现形式不等于知识。** 文档、代码和表格是知识的载体；需求、决策、风险、测试义务等才是可关联、可行动的知识对象。
4. **语义必须配套动作。** Ontology 不只描述“是什么”，也定义“允许做什么、由谁做、会影响什么、如何回写”。
5. **Agent 是受治理的参与者。** Agent 只能使用发现得到的引用和能力，写操作先提议、再审批、后提交；不能选择任意引擎、伪造对象 ID 或绕过策略。
6. **一套业务合同，多种传输。** Desktop UDS、Web HTTPS/postMessage、Mobile HTTPS/SSE 只是不同行程，不得产生不同业务语义。
7. **当前真源优先。** AppFlowy Collab、工作区文件系统、代码仓库及外部业务系统继续拥有真实数据；Ontology 保存统一语义、引用、派生状态与行动记录，不复制所有内容。
8. **渐进式落地。** 先把现有 Markdown、Word、DSH 交付物跑过同一资源链，再扩展 Ontology；不以一次性重写为前提。

## 3. 已冻结的核心决策

| ID | 决策 | 结果 |
|---|---|---|
| D1 | 复用现有 `@muse/host-bridge` 作为元协议 | 不新造第二套 Host/DSH 总线 |
| D2 | `resourceRef` 保持不透明字符串，Host 私有维护 locator | 与现有 Rust/TS/Dart 合同兼容，避免路径泄露 |
| D3 | Resource、Surface、Ontology 是三个不同限界上下文 | 文件打开不会污染业务本体模型 |
| D4 | 格式识别与引擎支持由插件注册，不进入核心 | 新增 CAD/3D 只增加 Provider、Adapter 与 Manifest |
| D5 | 所有写入采用 `propose → approve → apply → event` | 统一乐观并发、审批、幂等与审计 |
| D6 | 默认打开由 Host 路由策略决定，模型不能指定 `engineId` | 用户偏好可影响选择，但不能绕过能力与权限 |
| D7 | Anchor 是内容片段的跨格式统一引用 | 页、段落、单元格、代码符号、CAD 实体可以同构关联 |
| D8 | Ontology 建模现实与工作语义，不按来源系统复制表结构 | 避免部门孤岛、系统孤岛和“万能知识点”对象 |
| D9 | 变化先形成 ChangeSet 与 ImpactSet，再触发下游动作 | 产品、开发、测试联动可解释、可审核、可重放 |
| D10 | 未知格式必须安全降级 | `edit → annotate → view → metadata → download/system` |
| D11 | 原生引擎保持 vendor 身份，Muse 只写适配器 | ioffice、Helix、open-file-viewer 不被改造成业务插件 |
| D12 | Desktop、Web、Mobile 共用合同，按 Host placement 裁剪能力 | 远端云 Host 不提供“Finder 打开服务器文件”等错误语义 |

## 4. 文档地图

| 文档 | 面向角色 | 回答的问题 |
|---|---|---|
| [01-product-prd.md](01-product-prd.md) | 产品、设计、业务负责人 | 产品是什么、给谁用、核心流程和成功标准是什么 |
| [02-current-state-and-gap.md](02-current-state-and-gap.md) | 架构师、研发负责人 | 当前代码已经具备什么，初稿哪些假设需要修正 |
| [03-universal-resource-protocol.md](03-universal-resource-protocol.md) | 协议、Host、DSH 研发 | 任意格式如何共用发现、授权、读取、承载、回写协议 |
| [04-surface-routing-and-engine-adapters.md](04-surface-routing-and-engine-adapters.md) | 客户端、编辑器、插件研发 | ioffice、Helix、Viewer 及未来引擎如何接入与路由 |
| [05-ontology-domain-model.md](05-ontology-domain-model.md) | 产品建模、数据与平台研发 | “知识点”怎样成为可扩展、可行动的 Ontology |
| [06-agent-action-workflow-runtime.md](06-agent-action-workflow-runtime.md) | Agent、Workflow、后端研发 | 变化如何触发影响分析、提议、审批与自动化 |
| [07-user-experience.md](07-user-experience.md) | 产品、交互、客户端研发 | 用户如何打开、关联、审阅影响和跨端工作 |
| [08-security-governance-observability.md](08-security-governance-observability.md) | 安全、平台、运维 | 权限、审计、隐私、插件供应链和可观测性如何治理 |
| [09-implementation-roadmap.md](09-implementation-roadmap.md) | 研发管理、QA | 如何从当前代码分阶段交付，验收与退出条件是什么 |
| [10-end-to-end-scenarios.md](10-end-to-end-scenarios.md) | 全体 | 产品需求变更、未知格式与跨端场景如何端到端运行 |
| [11-decisions-and-open-questions.md](11-decisions-and-open-questions.md) | 决策者 | 哪些问题已经决定，哪些必须在 spike 后冻结 |
| [12-web-host-merge-analysis.md](12-web-host-merge-analysis.md) | 决策者、研发负责人 | 接近功能对等时，能否把 `frontend/web` 并进 Flutter Client |
| [13-web-host-forced-consolidation-reduced-scope-analysis.md](13-web-host-forced-consolidation-reduced-scope-analysis.md) | 决策者、研发负责人 | 强制收口且允许 Web 精简时的可行方案、工作量、风险和性能门 |

推荐阅读顺序：产品负责人读 01 → 05 → 07 → 10；技术负责人读 02 → 03 → 04 → 06 → 08 → 09。评估双前端收口时，功能对等场景读 12，Web 可精简场景读 13。

格式引擎的阶段级设计、开发计划和测试矩阵见：

- [engine-delivery/README.md](engine-delivery/README.md)：Host–DSH–Engine 协同以及 Helix、ioffice、open-file-viewer 专项实施方案。本轮只预埋 Ontology hook，不实现 Ontology Runtime。
- [workspace-platform/README.md](workspace-platform/README.md)：把 AppFlowy 页面型 Workspace 重构为支持本地、SSH、云端和协作资源的多根 Workspace Platform，包括 PRD、Provider 架构、DSH、插件菜单、UX 与开发路线图。

## 5. 状态标记

本套文档统一使用以下标记，避免把目标架构误读为已交付能力：

| 标记 | 含义 |
|---|---|
| **[已实现]** | 当前工作区中已有可定位的产品代码或测试 |
| **[部分实现]** | 已有主干能力，但端、格式或生命周期不完整 |
| **[拟新增]** | 本方案要求新增，当前代码不存在 |
| **[过渡]** | 为兼容现有交付保留，目标状态会被适配器替代 |

## 6. 不做什么

- 不把所有文件导入一个新的专有格式。
- 不要求 ioffice、Helix、open-file-viewer 共享内部文档模型。
- 不把 Ontology 做成“所有内容都塞进一个图数据库”的数据搬家工程。
- 不让每次按键、光标移动或自动保存都触发 Agent 工作流。
- 不让 Agent 绕开真实系统的权限、并发控制和审批。
- 不承诺所有格式都可编辑；一等公民指协议、引用、治理和降级一致，不代表能力相同。

## 7. 外部设计参考

Palantir 的公开资料强调 Ontology 同时包含语义对象/关系和行动/函数，并将数据、逻辑、行动与安全统一到面向人和 AI 的运行层。本方案借鉴这一原则，但按 Muse 的本地优先、插件化、多端和现有 Host Bridge 约束独立实现：

- [Ontology system](https://www.palantir.com/docs/foundry/architecture-center/ontology-system)
- [Ontology overview](https://www.palantir.com/docs/foundry/ontology/overview)
- [Action types](https://www.palantir.com/docs/foundry/action-types/overview)
- [Interfaces](https://www.palantir.com/docs/foundry/interfaces/implement-interface)
- [Ontology anti-patterns](https://www.palantir.com/docs/foundry/ontology/ontology-anti-patterns)
- [Automate](https://www.palantir.com/docs/foundry/automate)
