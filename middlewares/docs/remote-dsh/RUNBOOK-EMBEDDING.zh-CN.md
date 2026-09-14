# Host 嵌入 DSH 排障 Runbook

合同：[E3-TRACE-AND-DEPLOY](phases/E3-TRACE-AND-DEPLOY.zh-CN.md)。不要打印 JWT、`token=`、`DEEPSEEK_API_KEY`。

## 1. 先读面板

复制 `data-attachment-id`（或点标题栏 id）。记下 `data-attachment-stage` 与 `data-error-code`。

| code | 下一步 |
|---|---|
| NEED_AUTH | 登录；确认 Cookie `access_token` |
| NEED_WORKSPACE | 打开 `/app/<workspaceId>` |
| POOL_UNAVAILABLE | `systemctl status dsh-pool`；`docker ps` 看 `muse-dsh-bff`（现网 BFF 是 host 网络容器，systemd unit 可能 inactive） |
| INGRESS_DENIED | GET `/u/` 之前是否已有 cookie；`auth_request` 是否打 BFF `:8010` |
| BRIDGE_SILENT | 实例 env `MUSE_DOCUMENT_CLOUD_URL`；capabilities |
| HOST_UPGRADE_REQUIRED | Mobile：`GET …/parent-bridge/capabilities` 的 `nativeHttpSse` 必须 true；`instance.env` 要有 `MUSE_MOBILE_BRIDGE=1` 并重启 **unit** |
| NO_DEVICE_TOKEN | BFF `POST /api/muse/dsh/device-token` |
| BIND_REJECTED | iframe 子帧 `parent-bridge` 状态码。403 `SCOPE_MISMATCH` = 与已 attach Host 不是同一 workspace。409 `HOST_IN_USE` = 附件已满，不是「手机占了实例」 |
| QUEUED / COLD_START | 等；不要当修复去重启池 |
| NEED_API_KEY | **Desktop only**：本机填 key，不要查 `/u/` |
| SIDECAR_EXIT | Desktop sidecar 日志 |

## 2. Network（Fetch/XHR）

1. 有无 `POST /api/muse/dsh/session/open`。无 → Host（E0），不是 systemd。
2. 该 POST 是否带 `X-Muse-Attachment-Id`。BFF 日志应能 grep 同一 id。
3. 随后有无 `GET /u/<hash>/`。无 → 排队/冷启动/webUrl。
4. GET 401 → L0 cookie / ingress-auth。
5. 200 HTML 无联动 → 看 iframe initiator 的 `POST .../muse/v1/parent-bridge`。

## 3. 现网状态（2026-09-10）

- Web dist 已更新。硬刷新 `/app/<workspaceId>` 后，Host 应 POST `/api/muse/dsh/session/open`；配置里的 `/dsh/` 只给 CSP，不是 iframe src。
- nginx `/api/muse` 与 DSH `auth_request` 打 BFF `:8010`。共享 `:3080` 容器仍停；未登录 `/dsh/` 是 401（auth_request），不要因此 `docker compose up muse-dsh`。
- BFF stdout 一行 JSON：`action` / `status` / 可选 `attachmentId`。`docker logs muse-dsh-bff` 可 grep 面板复制的 id。
- 活实例的 `@muse/dsh-appflowy` parent-bridge 已含 RPC `bridge.reply`。新租户冷启动仍可能 >60s，属 Queued/COLD_START，不要当池挂了。
- iframe `GET /u/<hash>/` **502**：先 `ss -lptn | grep 13081` 与 `systemctl status muse-dsh-<hash>`。实例 crash loop（插件 `Cannot find module` / 缺导出）时池仍 502，不要 `docker compose up muse-dsh`。
- Mobile `HOST_UPGRADE_REQUIRED` / parent-bridge **403 MOBILE_DISABLED**：capabilities `nativeHttpSse:false`。设 `MUSE_MOBILE_BRIDGE=1` 后只重启 tenant unit，不要重启 `dsh-pool`。
- `GET /u/` 200 但黑屏：看 Network 里 `/assets/*.js` 是否打到站点根路径 404。正确应变为 `/u/<hash>/assets/…`（HTML 里 `./assets/`）。
- `transport failure for /api/host.listDirectory: HTTP 404`：DSH 的 `/api/host.*` 打到了官网 `/api`。硬刷新后应变成 `POST /u/<hash>/api/host.listDirectory`。iframe 注入会改写 fetch/XHR/WebSocket。

## 4. 不要做的事

- 不要把共享 `/dsh/` 当 iframe 源，也不要 `docker compose up muse-dsh`。
- 不要把官方 Cloud `:8000` 当成 `/api/muse`（0.16.5 没有该路由）。
- 不要用「刷新浏览器碰巧好了」当根因。
