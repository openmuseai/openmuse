# Muse Workspace Platform 设计总览

状态：目标方案 v1 + Local P0 纵向切片 + DSH 绑定 P0 均已实现（绑定 P0 于 2026-09-20 端到端验证）  
范围：Host（AppFlowy Flutter/Rust）、DSH、资源 Provider、工作区 UI、插件贡献协议  
前置：沿用 `muse.resource`、Surface Orchestrator、Engine Adapter 与 Host Bridge；Ontology Runtime 仅预留，不在本轮实现。

## 1. 结论

Muse 不再把 AppFlowy 的页面树等同于 Workspace。目标模型拆成四层：

```text
Account Space
  用户、团队、成员、角色、计费、协作策略
    └─ Project Workspace
       一项工作/项目的多根目录、打开状态、DSH 会话与工作区配置
         ├─ Mount: local:///repo
         ├─ Mount: ssh://host/repo
         ├─ Mount: cloud://provider/team/folder
         └─ Mount: muse-collab://space/pages
              └─ Resource Entry: 任意目录、文件、虚拟对象
```

- **Account Space** 继承现有 AppFlowy 用户空间、成员、角色、同步和协作能力。
- **Project Workspace** 是新的产品级工作区，是用户与 Agent 的任务边界，不绑定 Markdown 或某个文件系统。
- **Mount** 是可插拔 Provider 暴露的目录根；一个工作区可有多个根。
- **Resource Entry** 只携带安全元数据和 opaque identity，内容由 Provider 按权限、revision 和 range 提供。
- AppFlowy Page 不被删除，而是通过 `muse-collab` Provider 成为一种可协作资源。
- Workspace Explorer 和文件 Tab 共用一套 Command/Menu Contribution 协议。
- DSH 绑定 Project Workspace，而不是直接相信 UI 传入的本机路径。

## 2. 文档导航

| 文档 | 解决的问题 |
|---|---|
| [01-product-prd.md](./01-product-prd.md) | 用户场景、功能边界、分端能力、产品指标与验收 |
| [02-current-state-audit.md](./02-current-state-audit.md) | 当前 AppFlowy Workspace 的真实实现、可复用基座和替换边界 |
| [03-benchmark-and-technology-decisions.md](./03-benchmark-and-technology-decisions.md) | VS Code、Zed、JetBrains 的借鉴点和技术选型 |
| [04-domain-model-and-provider-contract.md](./04-domain-model-and-provider-contract.md) | Workspace/Mount/Entry/Provider 数据模型、状态机和合同 |
| [05-system-architecture-and-dsh.md](./05-system-architecture-and-dsh.md) | Host、DSH、远端执行、缓存、协作和安全架构 |
| [06-plugin-and-ux-design.md](./06-plugin-and-ux-design.md) | Explorer、菜单、Tab、命令系统和视觉交互规范 |
| [07-development-roadmap.md](./07-development-roadmap.md) | 分期开发计划、迁移、验收门槛、测试与发布策略 |
| [08-local-p0-implementation-and-acceptance.md](./08-local-p0-implementation-and-acceptance.md) | 本轮实现、Workspace/Version 联合 E2E 与剩余边界 |
| [09-dsh-binding-product-prd.md](./09-dsh-binding-product-prd.md) | Host Project Workspace ↔ DSH Workspace 绑定的产品文档：对应关系、场景 SC-*、需求 WBD/RLO/RCX/PBU/MPT 与验收 |
| [10-dsh-binding-architecture-design.md](./10-dsh-binding-architecture-design.md) | 绑定与资源联动的方案设计：现状断点、binding 协议与 materialization、模块设计、分期与测试 |
| [11-dsh-binding-development-plan-and-test-matrix.md](./11-dsh-binding-development-plan-and-test-matrix.md) | 绑定 P0 的开发计划、测试矩阵（T-01..T-21）与本机端到端验证结果 |

## 3. 关键决策

1. 不扩展 `ViewLayoutPB` 来枚举所有文件格式；格式由 Resource Descriptor 和 Engine Adapter 声明。
2. 不把本地路径、SSH 路径或云盘 object key 当全局身份；使用 `workspaceRef/mountRef/entryRef/resourceRef`。
3. 不把整个远端目录同步到 AppFlowy Collab；只同步工作区定义和明确加入协作的知识资源。
4. Explorer 按需分页并订阅变化，不把完整文件树一次性塞进 Flutter Bloc 或 DSH context。
5. UI 与数据面分离：Flutter 负责 Workbench；Provider 在资源所在执行域运行。
6. SSH 首选“远端 Workspace Agent + 本地 UI”，SFTP 仅作为浏览/传输降级。
7. 插件贡献声明命令和菜单位置，Host 负责条件、排序、权限和生命周期。
8. Ontology 只预留 `objectHints/semanticKind/changeEvent` 扩展字段，不建立对象图或影响传播。

## 4. 本轮代码边界

过渡性的 `LOCAL RESOURCES` 和“New local resource”已经移除。当前 Local P0 已经实现真正的目录 Mount、懒加载树、创建/导入/重命名/删除、持久化恢复、文件变化监听、DSH 主目录绑定以及 Resource Open/版本菜单复用。SSH/Cloud 和正式 `AppFlowyCollabProvider` 仍按路线图实施。
