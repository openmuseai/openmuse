# AppFlowy 项目分析文档

本目录包含对 **AppFlowy** 客户端仓库（`openmuseai/dsh-office`）的架构与功能分析，基于当前工作区源码（版本 `0.11.4`）整理。

## 文档索引

| 文档 | 说明 |
|------|------|
| [architecture-analysis.md](./architecture-analysis.md) | **主文档**：项目定位、整体架构图、仓库结构、生态关系 |
| [features-and-code-map.md](./features-and-code-map.md) | 各细分功能对应的 Flutter / Rust 代码路径与模块职责 |
| [dependencies.md](./dependencies.md) | 源码依赖、开源依赖、闭源/商业依赖分类汇总 |

## 分析范围

- **主仓库**：`/AppFlowy`（Flutter UI + Rust 后端 SDK）
- **关联生态**（同工作区其他仓库，客户端通过 git 依赖或协议对接）：
  - [AppFlowy-Collab](../AppFlowy-Collab) — 协作数据层（CRDT / Yrs）
  - [AppFlowy-Cloud](../AppFlowy-Cloud) — 云端同步与 API 服务
  - AppFlowy-Web — Web 端（Yjs + 相同 protobuf 协议）
  - 独立包：appflowy-editor、appflowy-board、AppFlowy-plugins 等

## 快速结论

AppFlowy 是 **Notion 的开源替代方案**，采用 **Flutter（UI）+ Rust（业务逻辑）** 双栈，通过 **FFI（dart-ffi）** 与 **Protobuf 事件总线（lib-dispatch）** 通信。数据层基于 **AppFlowy-Collab（Yrs CRDT）** 实现实时协作与本地持久化；可选对接 **AppFlowy-Cloud** 实现多端同步、AI 云服务与页面发布。
