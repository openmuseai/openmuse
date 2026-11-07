# 数据库资源插件

通过公开 Muse 插件 SDK 提供 `.musedataset` 创建、打开、保存、只读历史快照和 Agent 上下文提取。复用 `openmuse_dataset` 的原 AppFlowy Database/Row 内核与共享 Grid，不修改生产 Host 的内容格式分支。

构造时注入 `DatasetCore`、以资源身份隔离的 `SnapshotStore` 工厂，以及可选的协作 provider 工厂。凭证属于会话，不写入资源文件。Root 类型 1、Row 类型 4；内容、持久化与传输的依赖方向分开。历史快照禁用 live provider 并隔离当前缓存。

本插件是基础内部增量，当前仅开放文本、数字、复选框、链接与 Grid。智能文档内嵌、全部思源字段/视图、完整角色/分享/评论/历史与实时协作仍须按产品能力门验收。

验收工作台：`Muse-Plugin-Ecosystem/plugins/markdown/smart_document/flutter/example/lib/dataset_main.dart`。使用公开 ResourceCenterHub、LocalResourceProvider、ResourceEditorSession 和真实资源提交屏障；没有在产品用户流程展示传输参数。历史资源隔离和重复会话所有者拒绝有独立测试。
