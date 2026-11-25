# 内容快照持久化端口

从现有智能文档的存储实现提取的中立 package，供文档、数据集等领域注入使用，不定义内容 schema 或 CRDT。

`SnapshotStore` 负责完整 JSON 快照的读写；`FileSnapshotStore` 采用 flush + 同目录独立 staging + rename，`IndexedDbSnapshotStore` 等待事务完成。不同适配器并发写同一路径不会共享临时文件；它们仍需由领域会话处理语义版本/串行化，最后完成的完整快照胜出，不提供数据库事务或跨进程 CAS。

File/Browser 分开导出，浏览器数据库前缀可注入。原文档的兼容适配器继续使用原 `muse-smart-document:` 前缀，避免拆包导致现有离线内容失联。文件的可靠性边界、浏览器驱逐和多标签页所有权由使用方显式处理。
