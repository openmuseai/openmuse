# 独立的 AppFlowy 对象状态传输

传输原 HTTP read/create/full-sync；领域注入原类型、binding、工作区、设备和认证。不包含文档、表格、字段、存储或编辑器，响应成功只表示服务接受，不表示写盘 ACK。

GET 在 `http.ClientException`（例如暂停客户端错过 keep-alive FIN）时最多重试一次，每次重新取得认证，并保持原作用域和禁止重定向。HTTP 拒绝、配额错误、类型错误不因此重试；POST/create/full-sync 不自动重放，以免把不明确的写入结果当成幂等事务。关闭后不启动重试。

401/403 和原业务授权错误先于缺失判断，原 tombstone -4 是明确的 OBJECT_DELETED；只有受权的缺失结果才可交给领域决定是否创建。库不决定对象恢复或重建。

`dart test` 覆盖类型与 binding、路由/重定向、删除、读重试上限和写入不重放。真实隔离 Cloud 的暂停恢复与 Native/Web 往返测试在智能文档 Dataset 工作台执行。
