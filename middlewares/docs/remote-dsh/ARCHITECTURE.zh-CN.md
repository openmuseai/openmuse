# Remote DSH 生产架构

## 1. 与 Desktop sidecar 同构

生产 Host **不是**新 Agent。它与 macOS 打包使用同一套：

- `scripts/stage-dsh-runtime.sh`（Node + harness + `@muse/*` + `dshmarket` + `cordis.patch.yml`）
- 启动：`node --import tsx/esm apps/cli/src/bin.ts --profile web --patch <patch> --host 0.0.0.0 --port 3080`（容器内）；宿主机只映射 `127.0.0.1:3080`
- `allowRestart: false`（编排由 Docker/systemd 负责，避免市场插件杀掉 Node）
- 不修改 `agent/deepseek-harness` 源码

差异只有部署位置、绑定地址、密钥来源和前面的 TLS 终止。

## 2. 进程与数据

```text
宿主机 nginx :443  server_name dsh.example.com
        │
        ▼ 127.0.0.1:3080
Docker muse-dsh
        ├─ bundled node
        ├─ dsh harness (staged)
        ├─ patch.yml
        └─ volume: DSH_HOME=/var/lib/muse-dsh   # profile、会话，非源码
```

环境变量注入模型 key（名称以 DSH 实际上游为准，文档与脚本只引用 env **名**）。镜像构建上下文来自 stage 目录，`.env` 不进 `docker build`。

## 3. 多租户（分期）

| 阶段 | 模型 |
|---|---|
| D1（本期脚本） | 单容器、单 profile；靠 GoTrue 用户隔离 **尚未** 做强多租户 |
| 后续 | 每租户/每工作区独立 generation 或独立 Host；Gateway 按 account 路由 |

D1 可演示 Web iframe；账号级隔离必须在 BFF 与 Gateway 完成前，不得把该容器当多组织生产。架构上预留 `DSH_HOME` 分卷。

## 4. 健康与升级

- 健康：HTTP 探活 DSH web 端口（具体 path 以 harness 为准，脚本用 TCP/`/`）
- 升级：新镜像 `docker load` → compose up → 旧 generation 连接收到 stale handle（V2-06）
- 回滚：保留上一 tar 镜像；LKG 是 Host 镜像 digest + patch digest，不是浏览器缓存

## 5. 与 Gateway 的关系

DSH Web UI 仍由官方 Client 提供。Muse Gateway（`packages/core/remote-runtime`）在生产中应作为 **同一网络命名空间内的 sidecar 或由 Host 插件加载**，对 Flutter Android / 未来 Flutter Web 提供 `muse.remote/v2`。本期 D1 先把官方 Web 端口反代出去，与现有桌面 iframe 对齐；W2/A2 再把 Gateway 端口和 BFF 接到 Cloud nginx（`/api/muse/`）。

顺序不能颠倒：没有稳定 Host 进程，就没有可测的 token 与 stream。
