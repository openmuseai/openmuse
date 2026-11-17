# Mobile Cloud Workspace Service Integration

> 状态：Engineering Accepted；生产测试账号与真实 WebView 门禁待外部环境
>
> 子需求：M4 integration increment
>
> 日期：2026-09-29

## 1. 本增量消除的 fixture

旧 M4 只有 `CloudWorkspaceFlow` 内存状态机，Mobile composition 硬编码
`OpenMuse Account` 和 `cloud:welcome`。这既不会访问 Cloud Authority，也会让开发
fixture 被误认为真实账号。

本增量删除生产 composition 中的硬编码账号与 Workspace：默认 App 明确显示未登录；
登录/bootstrap 层必须提供 `OpenMuseSessionPort`、HTTPS API origin 和短期
`AccessTokenProvider`，再调用 `connectedCloudMobileComposition`。Token 只在请求时
注入 Authorization header，不写入 URL、domain state、日志或制品。

## 2. 领域与 adapter 边界

- `openmuse_mobile_core/cloud_service.dart`：Cloud catalog、proposal、receipt 合同和
  `CloudWorkspaceCoordinator`；只编排 M2 connector、M3 range client 与 revision /
  generation，不 import Flutter、`dart:io` 或 Cloud SDK。
- `openmuse_mobile_cloud`：唯一拥有 `HttpClient`、HTTPS/JSON、Bearer header、状态码
  映射与响应大小边界的 adapter；同时实现 `CloudWorkspaceService`、
  `DshRuntimeConnector`、`ResourceRangePort` 和 `OfficeResourceCommitPort`，但四个
  capability 在 Core 仍是独立 port。
- `app/openmuse_mobile`：composition root 将登录态和 adapter 注入 Host Shell；UI 只
  显示 placement、revision、Storage 可写性、DSH binding 状态及服务端 session 地址。

HTTP adapter 只接受 origin-only HTTPS URL，不跟随 redirect。Resource range 必须返回
请求的精确字节数；JSON 有 1 MiB 上限。401/403、409、424/503 分别映射为登录、
stale revision、Storage unavailable 领域错误。

Office commit 使用 `POST /v1/resources/commit`：正文是有界
`application/octet-stream`，resourceRef、expectedRevision、idempotency key 和 generation
分别进入专用 header，响应必须是包含 previous/new revision 的 receipt。它不使用
base64 JSON 扩张大文件，也不向 Engine 暴露 Bearer token。

Resource catalog 使用 `POST /v1/resources/list`，请求必须携带 workspaceRef、当前
workspace revision 和 generation。响应只形成 `CloudResourceRecord`，不能直接充当读取
权限；打开时仍必须单独调用 `/v1/resources/handles` 获取 audience-bound 短期 handle。

## 3. Service-backed TCK

统一入口：

```bash
./scripts/test_mobile_cloud_service.sh
```

测试启动 loopback HTTP service，不直接调用 fake 方法，走完：

```text
Bearer login → GET catalog → POST DSH session
→ issue ResourceHandle → bounded range
→ proposal(expectedRevision) → apply → receipt(newRevision)
```

TCK 同时验证生产拒绝 HTTP origin、缺失 token fail closed、请求顺序、generation、
workspaceRef 与 revision receipt。Flutter widget 测试验证默认制品不再出现 fixture
账号，以及注入 service 后 catalog 与 Remote DSH session 能到达 Mobile 页面。
同一 loopback TCK 还验证 DOCX bytes commit 的 header、原始正文和 CAS receipt。
Resource catalog TCK 另行验证 revision/generation scoped 请求；Flutter E2E 验证 catalog
条目不能绕过 handle issuance，并走到 receipt-backed DOCX 保存。

## 4. 尚未被本地测试替代的发布门禁

本地 service-backed TCK 证明客户端边界和完整请求链，不证明生产 Cloud 已部署。
以下证据仍必须来自实际环境：

- 真实测试账号的 OAuth/OIDC 登录与设备安全存储；
- ST2 Cloud Authority、X2 Workspace Runtime endpoint 的部署互操作；
- Remote DSH WebView 的 loaded / bridge-bound / workspace-attached 回调；
- proposal 审批 UI 与真实 revision receipt；
- 后台、弱网、token refresh 与 Storage 故障演练。

因此本增量可以替代 fixture account 作为工程集成基线，但在上述门禁完成前，路线图
不得把 M4 标记为 Production Accepted。
