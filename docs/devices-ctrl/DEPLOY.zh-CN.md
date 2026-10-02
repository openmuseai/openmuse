# 公网设备目录与 Desktop 转发：部署指南

> 日期：2026-10-01
>
> 协议见 [DESKTOP-OUTBOUND-RELAY.zh-CN.md](DESKTOP-OUTBOUND-RELAY.zh-CN.md)。
> 本文只写生产机上怎么发布、怎么验收，以及改了哪一段要重部署哪些服务。

## 1. 发布在哪台机器

服务跑在生产机上，由这台机器的 systemd 和 nginx 对外提供。开发机只负责执行部署脚本，
不转发流量，也不必保持在线。

| 对外地址 | 生产机上的进程 | 作用 |
|---|---|---|
| `https://openmuseai.com/api/muse/devices` | nginx `:443` → `127.0.0.1:8096` | 设备登记、心跳、列表、事件 |
| `https://openmuseai.com:8443` | nginx `:8443` → `127.0.0.1:8096` | Desktop 出站附着，以及 Mobile 打开 DSH |
| `https://openmuseai.com/api/muse` | 原 BFF `:8010` | 保持不变 |
| `https://openmuseai.com/gotrue` | 原 GoTrue `:9999` | 保持不变 |

`8096` 是生产机上的回环端口，用来避免明文端口直接暴露。公网客户端访问的是上面两个
HTTPS 地址。Desktop 本机的 `127.0.0.1:13180` 仍然只给这台电脑上的网关使用。

`8443` 使用现有的 `openmuseai.com` 证书。`link.openmuseai.com` 还没有解析，也没有写进证书。
云厂商安全组目前没有放行 TCP 8443 时，设备目录在 443 上已经对外，数据面从公网仍会超时。

不要用 `Muse-Server/AppFlowy-Cloud/scripts/deploy.py` 做这次发布。那个脚本按旧的
Cloud 镜像字段整包更新，字段和现网配置也不一致，而且不会装这个转发服务。

## 2. 一键部署

凭证放在仓库根目录的 `scripts/deploy.env`（已忽略，不要提交）。需要
`OPENMUSE_DEPLOY_HOST`、`OPENMUSE_DEPLOY_USER`、`OPENMUSE_DEPLOY_PASSWORD`。

在仓库根目录执行：

```bash
python3 Muse-Server/device-relay/deploy.py
```

脚本会在生产机上完成这些事，并且不打印数据库连接串或密码：

1. 上传 `relay_server.py` 和 `af_muse_device` 迁移。
2. 在 `/opt/openmuse-device-relay` 准备 Python 虚拟环境，安装 `psycopg`。
3. 从正在运行的 Cloud 容器读取数据库地址，把主机改成 Postgres 容器的地址，写入
   `/etc/openmuse-device-relay.env`（权限 `0600`）。
4. 执行 `CREATE TABLE IF NOT EXISTS`，不重建 Postgres，不改已有账号数据。
5. 安装并重启 systemd 单元 `openmuse-device-relay`。
6. 在现有 `:443` 站点加入更长前缀 `location /api/muse/devices`，并写入 `:8443` 虚拟主机。
   `location /api/muse` 仍指向 BFF。`nginx -t` 通过后才 reload。

只改了转发进程、还没改 nginx 时：

```bash
python3 Muse-Server/device-relay/deploy.py --code-only
```

只改了 nginx 发布、进程代码没变时：

```bash
python3 Muse-Server/device-relay/deploy.py --nginx-only
```

撤回 nginx 发布并停止转发进程（不删 `af_muse_device` 表）：

```bash
python3 Muse-Server/device-relay/deploy.py --rollback
```

第一次安装如果生产机没有 `python3-venv`，脚本会 `apt-get install`。这一步可能要几分钟。
成功时最后能看到 `service active` 和 `loopback 401`。

## 3. 一键测试

```bash
python3 Muse-Server/device-relay/test_public.py
```

它做两件事：

1. 在本机跑 `Muse-Server/device-relay/test_relay.py`：附着、断开后恢复、设备目录未登录拒绝。
2. 直连 `scripts/deploy.env` 里的生产机 IP，绕过本机代理 DNS：
   - `GET /api/muse/devices` 必须是 **401**，JSON `code` 为 **1011**。这是新转发服务的未登录响应。
   - `https://openmuseai.com:8443/v1/account/open` 必须连得上，并返回 401 或 405。

退出码：

| 码 | 含义 |
|---|---|
| 0 | 本地测试和两个公网入口都通过 |
| 1 | 本地测试失败，或 443 上的设备目录不是这个转发服务 |
| 2 | 443 已经正确，8443 从当前网络不可达 |

只要本机单元测试：

```bash
python3 Muse-Server/device-relay/test_public.py --local
```

只要公网探测：

```bash
python3 Muse-Server/device-relay/test_public.py --public
```

443 返回 **404** 表示请求还在旧的 BFF 或旧 Cloud 上，这次发布没有生效。
8443 超时而退出码为 2 时，生产机进程和主机防火墙通常已经放行，缺的是云安全组的 TCP 8443。

## 4. 改了什么，就要重部署和重测什么

下面的服务不要因为这次转发一起重启：Postgres、Redis、MinIO、GoTrue、muse-dsh、BFF、
Cloud 镜像、Admin、Worker、Search。现网 Cloud 镜像没有 `/api/muse/devices`，
设备目录由转发进程提供。只重编 Cloud 不会改变公网设备目录，除非将来把 nginx 的
`/api/muse/devices` 改回 `:8000`。

| 改动 | 重新部署 | 重新测试 |
|---|---|---|
| `Muse-Server/device-relay/relay_server.py`：附着、转发、设备目录、鉴权 | `deploy.py --code-only` | `test_public.py`。已连上的 Desktop 会按退避重新附着 |
| `remote_install.py` 里的 nginx 片段，或 443/8443 发布方式 | `deploy.py --nginx-only` | `test_public.py --public`。确认 `/api/muse` 仍走 BFF，而不是设备目录 |
| `migrations/20260930110000_af_muse_device.sql` | `deploy.py`（迁移是 `IF NOT EXISTS`，已有表不会被清空） | `test_public.py --public`；再用同一账号做一次设备注册 |
| `desktop_outbound_relay.dart`、网关公网 origin、Desktop host 接线 | 不部署服务器。重新编译并启动 Desktop，设置 `OPENMUSE_RELAY_PUBLIC_ORIGIN=https://openmuseai.com:8443`，GoTrue 为 `https://openmuseai.com/gotrue`，Cloud 为 `https://openmuseai.com` | `plugins/workspace-paired` 的 gateway 测试；验收 N2、N3、N6、N7、N8 |
| `paired_desktop_client.dart` 的公网重试 | 不部署服务器。重新安装 Mobile | 验收 N4：Desktop 附着断开时，45 秒内恢复则连接成功 |
| `gotrue_client.dart` 的 `/gotrue` 前缀 | 不部署服务器。Desktop 和 Mobile 都要重新编译 | `plugins/auth-gotrue` 的客户端测试；两端用公网账号登录 |
| `AppFlowy-Cloud` 的 `src/api/muse.rs` 设备路由 | 现网不要为它重编 Cloud。只有决定把 `/api/muse/devices` 从 `8096` 改回 `:8000` 时，才替换 Cloud 镜像并改 nginx | 改回之后再跑 `test_public.py --public`，并确认 BFF 的 `/api/muse` 没有被一起切走 |
| BFF、muse-dsh、GoTrue、Postgres 数据 | 不要为设备转发重启 | 抽查 `https://openmuseai.com/gotrue/health` 和原来的 `/api/muse` 仍按原合同响应 |

客户端编译使用 `/Users/mac/src/flutter`，不要用系统里更旧的 `flutter`。

Desktop 在公网模式下必须带上：

```text
OPENMUSE_GOTRUE_ORIGIN=https://openmuseai.com/gotrue
OPENMUSE_CLOUD_ORIGIN=https://openmuseai.com
OPENMUSE_RELAY_PUBLIC_ORIGIN=https://openmuseai.com:8443
```

Mobile 使用同样的 GoTrue 和 Cloud。它不自己附着；连哪台 Desktop 以设备目录里的
`transportOrigin` 为准。没有设置 `OPENMUSE_RELAY_PUBLIC_ORIGIN` 时，Desktop 继续只登记
本机 loopback，公网手机会发现设备但打不开数据面。

## 5. 验收对应关系

协议里的 N1–N8 里，脚本能自动覆盖的是 N1：匿名设备目录必须是 401 而不是 404。
N2–N8 需要同一账号的 Desktop 和 Mobile，不能用局域网或 ADB reverse 代替。
8443 的公网探测通过之前，N6 无法从公网完成。
