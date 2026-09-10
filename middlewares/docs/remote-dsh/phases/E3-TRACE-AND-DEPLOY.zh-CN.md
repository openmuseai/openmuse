# E3 追踪与部署合同 — 设计 / 实现 / 测试

> 阶段：E3。依赖 E0 的 `attachmentId`。可与 E1 部分并行（nginx 片段不依赖 bind RPC）。
> 范围：可恢复追踪性、仓库与现网配置合一、实例 env 探针、排障 runbook。不管池调度算法。

---

## 1. 设计

### 1.1 问题

- 失败无法从 UI 对到 BFF/池/实例日志。
- 仓库 `cloud-same-origin-dsh.conf` 的 `auth_request` 指向 `:8000`，现网必须 `:8010`。下次 `remote-infra-setup` 会静默回退。
- `inject_web_config` 把 `/dsh/` 既当 CSP 又当 iframe 世界。
- 实例缺 `MUSE_DOCUMENT_CLOUD_URL` 时 parent-bridge 不注入，Host 只能看到 `BRIDGE_SILENT`。
- BFF `session/open` 无关联 id（`muse-bff.ts` 只转发 account/workspace/deviceId）。

### 1.2 决策

**追踪**

```text
Host 生成 attachmentId (uuid)
  → session/open Header: X-Muse-Attachment-Id
  → BFF 日志同一字段，并转发到 pool JSON `attachmentId`（可选）
  → journald：pool + muse-dsh-<hash>.service
  → parent-bridge requestId ≠ attachmentId，但 reply 两边都记
UI：折叠显示 attachmentId，一键复制
禁止：日志打印任何 token
```

**部署合同（嵌入）**

| 项 | 合同 |
|---|---|
| nginx `/api/muse` 与 `auth_request` | 同一 BFF upstream（变量 `MUSE_BFF`，默认 `127.0.0.1:8010`） |
| `/dsh/` | 可留 401/502；注释写明禁止 iframe |
| `/u/` | pool proxy `:13080`；`auth_request` 同 BFF |
| inject | `frame-src 'self' https://<apex>`；agent URL path=/dsh 不得被 Host mount |
| instance.env | 必须 `MUSE_DOCUMENT_CLOUD_URL`、`MUSE_REQUIRE_HOST_AUTH=1`、模型 key；**禁止** `PORT`/`DSH_HOME` |
| 切转 | 先 Host E0，再谈入口；禁止先停入口后不改 Host（已发生过） |

**可恢复 runbook**（写入本阶段 §6 短节，运维只跟 code）：

| code | 下一步 |
|---|---|
| NEED_AUTH | 登录；看 Cookie |
| NEED_WORKSPACE | 打开 `/app/<id>` |
| POOL_UNAVAILABLE | `systemctl status dsh-pool muse-dsh-bff` |
| INGRESS_DENIED | cookie 是否在 GET `/u/` 之前；auth_request 是否打 BFF |
| BRIDGE_SILENT | 实例 env Cloud URL；capabilities |
| NO_DEVICE_TOKEN | BFF device-token 路由 |
| QUEUED / COLD_START | 等；不要重启用户浏览器以外的「刷新能好」当成修复 |

### 1.3 不变量

| ID | 陈述 |
|---|---|
| D1 | 每次 Failed UI 含非空 `attachmentId` |
| D2 | `session/open` 请求带 `X-Muse-Attachment-Id` |
| D3 | 仓库 nginx 片段 grep 不到写死的 `127.0.0.1:8000/api/muse` 作为 dsh auth（必须变量或 8010） |
| D4 | `instance.env` 模板不含 `PORT=` |
| D5 | 结构化日志字段无 `access_token` / `token=` 查询串 |

### 1.4 不做

- 不替换官方 Cloud 镜像。
- 不做全链路 APM 采购；journal + 浏览器足够 E3。
- 不预编译 tsx（可另开池任务）。

---

## 2. 实现

| 路径 | 改动 |
|---|---|
| `frontend/web/.../dsh-session.ts` | open/close/heartbeat 附加 `X-Muse-Attachment-Id` |
| `frontend/web/.../DshAgentPanel.tsx` | 复制 id；`console.debug` 结构化（prod 可用 `debug` 开关） |
| `middlewares/dsh/core/dsh-pool/src/muse-bff.ts` | 读 header，access log 一行 JSON：id、action、status、workspaceHash（勿打完整 token） |
| `dsh-pool/src/server.ts` / `pool.ts` | 可选透传 attachmentId 到日志 |
| `deploy/nginx/cloud-same-origin-dsh.conf` | `auth_request` 改 BFF；注释现网 |
| `deploy/scripts/remote-infra-setup.sh` 或等价 | 与片段一致 |
| `local/scripts/deploy_common.py` | 与 E0 同一 inject 合同 |
| `start-instance.sh` / `instance.env` 模板 | Cloud URL 必填检查；拒绝 source 含 PORT |
| `docs/remote-dsh/RUNBOOK-EMBEDDING.zh-CN.md` | **新建**短 runbook（从本阶段 §6 抽出也可） |

---

## 3. 测试方案

- BFF 单测：open 时无 header 仍 200（兼容）；有 header 则日志钩子收到 id（可注入 logger）。
- Web：open fetch 的 headers 含 id。
- 静态：CI grep nginx 片段。
- py：inject 无 key、CSP apex。
- shellcheck/start-instance：fixture env 含 PORT 时仍被实例参数覆盖（已有行为）+ 缺 Cloud URL 打警告（新）。

---

## 4. 测试矩阵

| ID | 场景 | 期望 | 类型 |
|---|---|---|---|
| E3-T1 | Failed 面板 | `data-attachment-id` 匹配 uuid | 组件 |
| E3-T2 | session/open fetch | Header `X-Muse-Attachment-Id` 等于面板 id | 单测 |
| E3-T3 | BFF 无 header | 仍代理 pool（不 400） | 单测 |
| E3-T4 | BFF 有 header | logger 含同一 id、不含 Bearer 明文 | 单测 |
| E3-T5 | nginx 片段 | auth_request 目标为 BFF 变量或 `:8010` | grep CI |
| E3-T6 | inject `/dsh/` | CSP origin 无 path；无 sk- | py（扩 W1-05） |
| E3-T7 | instance env 模板 | 无 `PORT=`、无 `DSH_HOME=` | grep |
| E3-T8 | 日志 fixture 含 token | 红线测试：序列化函数剥离 query token | 单测 |
| E3-T9 | Retry | 新 generation，attachmentId 策略文档化（保持或 att_new）；矩阵锁定一种 | 组件 |

**锁定：** 用户点 Retry **保持** attachmentId、generation++，便于把两次 open 打在同一附件上。新打开面板才新 id。

---

## 5. 生产验收

| ID | 操作 | 期望 |
|---|---|---|
| E3-P1 | 触发 NEED_WORKSPACE 或一次失败 | UI 能复制 id |
| E3-P2 | 成功 open | BFF/docker logs 能 grep 该 id |
| E3-P3 | `nginx -T` 或现网 conf | `/internal/muse-dsh-auth` 打向 BFF 而非官方 `:8000` 的 muse（0.16.5 没有该路由） |
| E3-P4 | 按 runbook 演练一次 INGRESS_DENIED | 5 分钟内定位到 cookie 或 auth_request，不重启池「碰运气」 |

---

## 6. Runbook 摘要

1. 看面板 `code` + `stage` + `attachmentId`。
2. Network Fetch：有无 `session/open`、状态码、随后有无 `GET /u/`。
3. 无 POST → Host（E0 矩阵），不是 systemd。
4. 有 POST 无 GET `/u/` → 排队/冷启动/webUrl。
5. 有 GET 401 → L0 cookie / BFF ingress-auth。
6. 200 HTML 无联动 → E1 capabilities / bind 状态码。
7. Desktop 问题走 sidecar 日志 + `NEED_API_KEY`，不要查 `/u/`。

---

## 7. 实现记录（2026-09-10）

| 项 | 状态 |
|---|---|
| nginx `cloud-same-origin-dsh.conf` | `auth_request` → `127.0.0.1:8010`（BFF） |
| `X-Muse-Attachment-Id` | Web `openDshSession` 已发送；Retry **保持** id |
| 面板 `data-attachment-id` + 复制 | E0/E3 已挂 |
| BFF access log | `attachmentId` / `workspaceHash` / `action` / `status`；无 Bearer |
| `instance.env.example` | 无 `PORT=` / `DSH_HOME=` |
| `start-instance.sh` | source 后恢复 systemd 的 PORT/DSH_HOME；缺 Cloud URL 警告 |
| [RUNBOOK-EMBEDDING.zh-CN.md](../RUNBOOK-EMBEDDING.zh-CN.md) | 已建 |

现网（2026-09-10）：`/api/muse` 与 `/internal/muse-dsh-auth` 均 `proxy_pass` BFF `127.0.0.1:8010`（E3-P3）。BFF 容器 bind-mount `/opt/muse-dsh/muse-bff`；已更新 `muse-bff.js` / `muse-bff-start.js` 后仅 `docker restart muse-dsh-bff`。无 header 的 open 仍 401 且打 `{"action":"session.open","status":401}`；带 `X-Muse-Attachment-Id` 的 401 日志含同一 `attachmentId`。`deploy_config` 现从 `local/deploy/environments/` 解析 `production.env`。E3-P1/P4 需登录面板复制 id。

