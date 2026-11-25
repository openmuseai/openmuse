# 协作传输合同：内部预览 0.1

独立的纯 Dart package，不依赖 Flutter、文件系统、网络实现或内容 AST。智能文档 native/Web 共用它；独立 Word/Workbook 等可声明自己的 schema/engine，不需要成为智能文档子块。

当前交付三个边界：

* `CollaborationBinding`：中立的 protocol/engine/encoding/schema/epoch 描述，带严格版本/字段解析和兼容判断。
* `CollaborationProvider`：读取不透明状态、显式初始化状态、交换字节和关闭。领域负责校验/合并，provider 不持有第二份可写正文。
* `CollaborationPresenceSlot` / `CollaborationPresenceUser`：对当前协作域内已认证、尚在线的会话做纯用户聚合；稳定排序、去重、冲突和上限校验。身份认证、时效和资源位置不由聚合函数推断。

`exchange` 成功只表示接受，不提供 durable ACK。公开会话权限、订阅/awareness、持久回执、迁移屏障和通用 runtime 尚未在这个预览实现；它不是完整的平台 wire 协议。现有 strict resource 合同没有增加字段。

真实消费者包括智能文档、独立 Dataset 和共享成员界面。AppFlowy 的 HTTP / WS2 适配分别放在 `muse_appflowy_transport` / `muse_appflowy_realtime`，内容领域声明自己的 profile；当前复用原 Document0、Database1 和 Row4。没有把 Cloud 的 `collab_type=0` 当作通用 Word/Workbook provider。契约不定义段落、单元格或 Flutter 类型。

运行 `dart analyze` 与 `dart test`。测试覆盖独立文档/工作簿绑定的表示、未知版本/字段拒绝，以及 engine/encoding/schema/protocol/epoch 的隐式切换拒绝；工作簿实例是合同 fixture，不代表已实现工作簿领域协作。
