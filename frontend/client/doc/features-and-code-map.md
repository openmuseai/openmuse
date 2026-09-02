# AppFlowy 功能与代码映射

本文档按**产品功能域**列出对应的 Flutter UI 代码、Rust 后端代码及关键依赖。

---

## 1. 文档编辑（Document）

### 功能说明

块级富文本编辑器：标题、列表、待办、代码块、表格、图片、公式、Callout、Slash 命令、协同光标等。

### 代码路径

| 层级 | 路径 | 关键模块 |
|------|------|----------|
| **Flutter UI** | `frontend/appflowy_flutter/lib/plugins/document/` | `application/`（BLoC）、`presentation/editor_plugins/`（各块插件） |
| **Flutter 编辑器** | 外部包 `appflowy_editor` | 块模型、选区、事务 |
| **Rust 业务** | `frontend/rust-lib/flowy-document/` | `manager.rs`、`parser/`、`document_data.rs` |
| **协作模型** | AppFlowy-Collab `collab-document` | 文档 CRDT 结构 |
| **事件注册** | `flowy-document/src/event_map.rs` | `init(DocumentManager)` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `collab-document`、`collab-integrate`、`flowy-document-pub` |
| 开源 | `appflowy_editor`（git）、`indexmap`、`scraper` |
| 闭源 | 无（纯本地功能） |

---

## 2. 数据库 / 多维表格（Database）

### 功能说明

支持多种字段类型（文本、数字、日期、选择、关联、媒体等），三种视图布局：表格（Grid）、看板（Board）、日历（Calendar）。

### 代码路径

| 视图 | Flutter 路径 | Rust |
|------|--------------|------|
| **Grid** | `lib/plugins/database/grid/` | `flowy-database2/src/services/` |
| **Board** | `lib/plugins/database/board/` | 同上 + `appflowy_board`（git） |
| **Calendar** | `lib/plugins/database/calendar/` | 同上 |
| **字段/单元格** | `lib/plugins/database/application/cell/` | `flowy-database2/src/services/cell/` |
| **行文档** | `lib/plugins/database_document/` | 关联 `flowy-document` |
| **公共接口** | — | `flowy-database-pub/` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `collab-database`、`flowy-database-pub` |
| 开源 | `rust_decimal`、`rusty-money`、`csv`、`chrono-tz`、`moka`（缓存） |
| 闭源 | 无 |

---

## 3. 工作区与文件夹（Workspace / Folder）

### 功能说明

Workspace、Space、页面树、视图创建/重命名/移动、收藏、图标与封面、页面发布（Sites）、访客分享。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter Shell** | `lib/workspace/`（`application/`、`presentation/home/`） |
| **侧边栏** | `lib/workspace/presentation/home/menu/` |
| **设置** | `lib/features/settings/`、`lib/workspace/presentation/settings/` |
| **Sites 发布** | `lib/workspace/presentation/settings/pages/sites/` |
| **Rust** | `frontend/rust-lib/flowy-folder/` |
| **公共接口** | `flowy-folder-pub/`（含 Cloud Service trait） |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `collab-folder`、`client-api`（发布/分享 API） |
| 开源 | `regex`、`num_enum` |
| 闭源 | 页面发布、访客协作需 **AppFlowy Cloud**（可自托管开源版或商业版） |

---

## 4. 用户与认证（User / Auth）

### 功能说明

本地匿名模式、邮箱登录、OAuth、工作区切换、用户配置、数据迁移、加密。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter** | `lib/user/`（`application/`、`presentation/`） |
| **云环境配置** | `lib/env/cloud_env.dart` |
| **Rust** | `frontend/rust-lib/flowy-user/` |
| **服务层** | `flowy-user/src/services/authenticate_user.rs` |
| **迁移** | `flowy-user/src/migrations/` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `flowy-user-pub`、`client-api`、`collab-user` |
| 开源 | `diesel`（SQLite）、`lib-infra`（encryption feature） |
| 闭源 | Cloud 认证走 **GoTrue**（AppFlowy-Cloud 开源组件） |

---

## 5. 搜索（Search）

### 功能说明

工作区内全文搜索、命令面板（Command Palette）快速跳转。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter** | `lib/workspace/application/command_palette/` |
| **Rust** | `frontend/rust-lib/flowy-search/` |
| **索引** | `flowy-core/src/full_indexed_data_provider.rs`、`indexed_data_consumer.rs` |
| **即时索引** | `collab-integrate/src/instant_indexed_data_provider.rs` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `flowy-search-pub`、`collab-folder` |
| 开源 | **Tantivy** `0.24.1`（全文检索引擎） |
| 闭源 | 无 |

---

## 6. AI 功能

### 功能说明

- AI 写作（编辑器内）
- AI Chat 对话
- 本地 AI（Ollama）
- 云端 AI（AppFlowy Cloud）
- RAG：文档 Embedding + 向量检索

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter AI UI** | `lib/ai/`、`lib/plugins/ai_chat/` |
| **编辑器 AI** | `lib/plugins/document/presentation/editor_plugins/slash_menu/.../ai_writer_item.dart` |
| **Rust** | `frontend/rust-lib/flowy-ai/` |
| **本地 AI 控制器** | `flowy-ai/src/local_ai/` |
| **Chat** | `flowy-ai/src/chat/` |
| **Embedding** | `flowy-ai/src/embeddings/` |
| **向量库** | `flowy-sqlite-vec/` |
| **云端 AI 接口** | `flowy-core/src/deps_resolve/cloud_service_impl.rs`（`ChatCloudService` 等） |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `flowy-ai-pub`、`langchain-rust`（appflowy fork） |
| 开源 | `ollama-rs`、`text-splitter`、`reqwest`；可选 `lopdf`、`pulldown-cmark`（file_reader feature） |
| 闭源 / SaaS | **云端 AI API**（AppFlowy Cloud）；本地 Ollama 为开源自部署 |

---

## 7. 文件存储（Storage）

### 功能说明

图片/附件上传、本地文件缓存、云端对象存储（S3 兼容）。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter** | `lib/startup/tasks/file_storage_task.dart`、文档/数据库中的媒体单元格 |
| **Rust** | `frontend/rust-lib/flowy-storage/` |
| **上传器** | `flowy-storage/src/uploader.rs` |
| **缓存** | `flowy-storage/src/file_cache.rs` |
| **公共接口** | `flowy-storage-pub/`（`StorageCloudService` trait） |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `flowy-storage-pub`、`collab-importer` |
| 开源 | `mime_guess`、`reqwest` |
| 闭源 | 云端存储依赖 **AppFlowy-Cloud + S3**（AWS SDK 在 Cloud 侧） |

---

## 8. 同步与云端服务（Sync / Server）

### 功能说明

本地模式与 AppFlowy Cloud 模式切换；Collab 实时同步；全量/增量同步。

### 代码路径

| 层级 | 路径 |
|------|------|
| **模式选择** | `flowy-core/src/server_layer.rs` → `ServerProvider` |
| **云端实现** | `flowy-server/src/af_cloud/` |
| **本地实现** | `flowy-server/src/local_server/` |
| **Cloud 服务聚合** | `flowy-core/src/deps_resolve/cloud_service_impl.rs` |
| **同步插件** | `client-api::collab_sync::SyncPlugin` |
| **Collab 构建** | `collab-integrate/src/collab_builder.rs` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `client-api`、`client-api-entity`、`workspace-template`（均来自 AppFlowy-Cloud git） |
| 开源 | `collab-plugins`（含 RocksDB 本地插件） |
| 闭源 | 官方托管云为 SaaS；商业自托管为闭源 fork |

---

## 9. 日期与提醒（Date / Reminder）

### 功能说明

文档内日期块、日程提醒、通知推送。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter** | `lib/date/`、文档 `date_item` slash 菜单 |
| **Rust** | `flowy-date/`、`flowy-document/src/reminder.rs` |
| **协同提醒** | AppFlowy-Collab `collab-user` |

### 依赖

| 类型 | 依赖 |
|------|------|
| 开源 | `chrono`、`collab-user` |
| 闭源 | 无 |

---

## 10. 回收站（Trash）

### 功能说明

已删除视图的恢复与永久删除。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Flutter** | `lib/plugins/trash/` |
| **Rust** | `flowy-folder`（软删除逻辑） |

---

## 11. 导入导出（Import / Export）

### 功能说明

Notion 工作区导入、Markdown 导入/导出、CSV 数据库导入。

### 代码路径

| 层级 | 路径 |
|------|------|
| **Rust** | `collab-importer`（AppFlowy-Collab） |
| **存储层使用** | `flowy-storage`（导入文件处理） |
| **Flutter** | 设置 / 工作区相关 UI |

### 依赖

| 类型 | 依赖 |
|------|------|
| 源码 | `collab-importer` |
| 开源 | `zip`、`csv`（可选 feature） |

---

## 12. 启动与基础设施

### 代码路径

| 功能 | 路径 |
|------|------|
| 应用入口 | `lib/main.dart` → `startup/startup.dart` |
| 启动任务链 | `lib/startup/tasks/`（`load_plugin.dart`、`file_storage_task.dart` 等） |
| 依赖注入 | `lib/startup/deps_resolver.dart` + **GetIt** |
| 路由 | **go_router** |
| 状态管理 | **flutter_bloc** |
| 国际化 | `easy_localization` + `assets/translations/` |
| 后端初始化 | `packages/appflowy_backend/lib/appflowy_backend.dart` |
| Rust 核心启动 | `flowy-core/src/lib.rs` → `AppFlowyCore::new()` |

---

## 13. 事件总线模块注册一览

`flowy-core/src/module.rs` 的 `make_plugins()` 注册顺序：

```
flowy-user      → 用户/工作区事件
flowy-folder    → 文件夹/视图事件
flowy-database2 → 数据库事件
flowy-document  → 文档事件
flowy-date      → 日期事件
flowy-search    → 搜索事件
flowy-ai        → AI 事件
flowy-storage   → 存储事件
```

Flutter 侧通过 `appflowy_backend` 的 `FlowySDK` / `Dispatch` 发送与上述模块对应的 Protobuf 事件。

---

## 14. PluginType 与 ViewLayout 对应关系

| PluginType | ViewLayoutPB | 说明 |
|------------|--------------|------|
| `document` | Document | 普通文档页 |
| `grid` | Grid | 表格视图 |
| `board` | Board | 看板视图 |
| `calendar` | Calendar | 日历视图 |
| `databaseDocument` | — | 数据库行内文档 |
| `chat` | — | AI 对话视图 |
| `trash` | — | 回收站（不可用户创建） |

注册代码见：`lib/startup/tasks/load_plugin.dart`。
