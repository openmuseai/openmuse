# Muse Host Registry

领域无关的 Rust Host capability registry。它不依赖 AppFlowy、DSH、Cordis、Flutter、ioffice 或具体 Provider。

该 crate 管理 Provider registration、冻结的 discover snapshot/cursor、精确且带 generation/TTL 的 binding、
invoke-time authority revalidation、JSON Schema input/output gate、provider/authority revoke、quiescent shutdown 及进程内
registry events。Descriptor/binding 的 wire 有效性直接复用相邻 contract crate 的 v1 schema。

Transport、wire receipt、policy 和领域 Provider 属于后续阶段；调用者身份必须由 Host/transport 注入，不能从 Provider
input 或 scope hint 推导。
