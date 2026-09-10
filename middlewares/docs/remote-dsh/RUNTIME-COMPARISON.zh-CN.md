# Remote DSH 运行时形态对比：Docker / systemd-run / supervisord / 调度器

> 状态：v0.2；配套 [MULTITENANCY.zh-CN.md](MULTITENANCY.zh-CN.md) §5.3–5.5。
> 场景：每租户一个 DSH harness；宿主 **4C/4G** Linux（systemd）；Cloud 同机。
> 结论：**生产池 = 裸进程 + systemd-run（首选）/ supervisord（无 systemd 时）+ cgroup v2；Docker 只做构建/CI/验收。调度控制面自研；Nomad 仅作为未来跨机器的 Executor 候选，单机不引入。**

---

## 1. 技术原理：Docker / systemd / supervisord 不是同一层

| 形态 | 本质 | 提供的 OS 能力 |
|---|---|---|
| **Docker** | 镜像（可分发产物）+ OCI 运行时沙箱 + 守护进程托管 | namespace（pid/net/mnt/uts/ipc/user）+ cgroup v2 + seccomp/apparmor/cap-drop + overlayfs；dockerd→containerd→shim→runc |
| **systemd-run** | 把「一个进程」挂进 PID1 的 unit 与 cgroup 层级 | cgroup v2（`MemoryMax`/`CPUQuota`/`PidsMax`）+ 硬化（`User=`/`NoNewPrivileges`/`PrivateTmp`/`ProtectSystem`）；无 namespace、无镜像 |
| **supervisord** | 用户态进程保姆（fork/监控/重启/日志文件） | 无 OS 隔离；cgroup 限额需外部写 |

**同源点**：三者做资源限额都落到 **cgroup v2**——Docker `--memory` 与 systemd `MemoryMax` 写同一处。差别全在外壳。本方案需要的是限额 + 专属 UID + loopback，**不需要** per-tenant netns / overlayfs。

---

## 2. 资源消耗对比（估算，待 §10 实测回填）

假设：单实例 idle RSS ≈ 400MB；基础设施（nginx + Cloud GoTrue/Postgres/BFF + 薄 Pool）≈ 1.5GB；宿主 4GB。4 核只影响 CPU 并发，不影响下表内存。

### 2.1 常驻内存

| 项 | Docker | systemd-run | supervisord |
|---|---|---|---|
| 平台级常驻 | dockerd+containerd **~100–300MB**（中值 ~200MB） | **0**（PID1 已在） | supervisord **~10–30MB** |
| 每租户额外 | shim **~5–15MB** | **0** | 0 |
| 每租户本体 | harness ~400MB，三形态等价 | 相同 | 相同 |
| **宿主可用实例内存** | `4096 − 1500 − 200 = 2396MB` | `4096 − 1500 = 2596MB` | `4096 − 1500 − 30 = 2566MB` |

另：若再加 k3s/Nomad server，单机还要扣 80–400MB，4G 上不可接受（Nomad **仅 client、且跨机器时**另论，见 §5）。

### 2.2 磁盘与日志

| 项 | Docker | systemd-run | supervisord |
|---|---|---|---|
| 安装 | 镜像分层共享 + 每容器可写层 | 一份 `/opt/muse-dsh/runtime` + 每租户 `DSH_HOME` | 同左 |
| 日志 | json-file 双写，需 `log-opt max-size` | journald，`SystemMaxUse` 限额 | 明文 + `logfile_maxbytes` |
| 分摊 | 定期 GC image/log | unit 归属天然可清 | 按文件清 |

### 2.3 CPU

三形态 idle CPU ≈ 0；dockerd 持续 ~0.1–0.5%。agent 期由同一套 cgroup 限，无差异。

4 核相对旧文档 2 核：`ACTIVE_QUOTA` 从 2 提到 **3**（仍受 LSP 峰值与 4GB 余量约束，不是 4）。多余的核用于：(1) 第三个并发 agent；(2) 冷启动与 Cloud 同机少抢；(3) 不要用来多开 READY 常驻实例。

---

## 3. 并发量对比

### 3.1 同参数（idle RSS = 400MB）

| 指标 | Docker | systemd-run | supervisord |
|---|---|---|---|
| READY（内存） | `floor(2396/410) = 5` | `floor(2596/400) = 6` | `floor(2566/400) = 6` |
| ACTIVE（CPU，4 核） | **3**（cgroup 同源） | 3 | 3 |
| 挂起会话 | 受磁盘限制 | 同 | 同 |
| 冷启动（挂起→恢复） | 0.5–3s | **<1s** | <1s |

建议生产 `READY_QUOTA=3` 先留 GC/尖峰余量，不要按 floor 打满。

### 3.2 Docker 何时少一个实例

| idle RSS | Docker READY | systemd READY | 差距 |
|---|---|---|---|
| 350MB | 6 | 7 | 1 |
| 400MB | 5 | 6 | 1 |
| 500MB | 4 | 5 | 1 |
| 600MB | 3 | 4 | 1（都应加内存） |

4GB 上 Docker 的税是 **常驻 ~200MB + 每租户 shim + 更差的恢复延迟**，不是数量级。RSS ≥ 500MB 时加内存到 8GB 优先于形态争论。

### 3.3 恢复并发

移动端进页即 connect。systemd/local spawn <1s 直启，同样 READY 池能撑更高「在线但暂时不活跃」比例。

---

## 4. 运维指标

| 维度 | Docker | systemd-run | supervisord |
|---|---|---|---|
| 按租户摊账 | `docker stats` / cAdvisor | **cgroup 直读** `memory.current` / `cpu.stat` | cgroupfs + `ps` |
| 实时 | `docker stats` | `systemd-cgtop` / `systemctl status` | `supervisorctl` |
| 日志 | `docker logs` | `journalctl -u muse-dsh-<hash>` | 文件 |
| 健康 | `HEALTHCHECK` | Pool 探活 + `Restart=` | 自写 eventlistener |
| 配额热改 | `docker update` | `systemctl set-property MemoryMax=` | 手写 cgroupfs |
| 管理面故障 | dockerd 挂则全体失控 | PID1，无额外管理面 | supervisord OOM 后子进程命运不确定 |
| Prometheus | 需 cAdvisor | node_exporter 即可 | 脚本 |
| 升级 | `docker load` + 替换 | 目录替换 + 重启 unit | 同左 |

本方案采集项：`memory.current`、`cpu.stat`、冷启动耗时、最后活跃时间戳。systemd 形态 **零新组件**。

---

## 5. 调度器（控制面 vs 执行器）—— 开源选型

Docker/systemd 解决「进程怎么住在机器上」。**调度**还分两层，不能买一个 Nomad 代替全部：

| 层 | 问题 | 开源能否直接用 |
|---|---|---|
| **控制面** | 这个 AppFlowy 用户+工作区是否已有实例？idle 多久该杀？launch token 给谁？满了怎么排队？ | **不能**。必须自研薄 Pool（`@muse/dsh-pool`） |
| **执行器** | 在本机（或未来某台机）拉起/杀死/读 RSS | systemd-run 现在够；跨机器再评估 Nomad |

### 5.1 候选打分（单机 4C/4G，本期不做多机）

| 候选 | 额外 RSS | 无 Docker | 多机 | 与 tenant 语义 | 结论 |
|---|---|---|---|---|---|
| systemd-run | 0 | 是 | 否 | 我们包一层即可 | **生产执行器** |
| Nomad `raw_exec` | client ~50–80MB；server 若同机再 +50–100MB | 是 | **是** | 仍要自研控制面当 API 客户端 | **跨机器第一候选**；单机不引入 |
| k3s / k0s | ≥300MB + containerd | 否（回容器） | 是 | 差 | 拒绝 |
| Knative / OpenFaaS | 更大 | 否 | 是 | 错模型（函数 vs 长会话） | 拒绝 |
| supervisord | 10–30MB | 是 | 否 | 无 cgroup | 无 systemd 时后备 |
| Firecracker / Kata | 每租户数十 MB+ | 是 | 可 | 隔离过剩 | 拒绝 |
| Swarm / Mesos | 高 | 否 / 死 | — | 差 | 拒绝 |
| Temporal | 高 | — | — | 工作流，不是进程池 | 拒绝 |

### 5.2 Nomad 采用门槛（与 MULTITENANCY §5.5 一致）

1. `SystemdRunExecutor` 已在 Linux 通过功能验收；
2. 真正要第二台 DSH 机，或目标环境没有 systemd；
3. 实测 Nomad client < 80MB，且能 UID + 内存限额；
4. 不强制 Consul。

未满足则保持自研 Pool + systemd。接口上 `InstanceHandle.allocId` / 路由表 `nodeId` 预留即可，**不要提前实现多机放置算法**。

### 5.3 开发期（macOS）

开发机没有 systemd。P1 用 `LocalProcessExecutor`（`child_process`）验证 I0/I1/I2 语义；P2 必须在 Linux（可以是 4G 云主机或 CI VM）上才算生产执行器完成。禁止用 Docker 冒充 P2 验收——那会把已否决的 shim 税和启动延迟测回来。

---

## 6. 决策速查

| 场景 | 选择 | 理由 |
|---|---|---|
| 构建 / CI 镜像 / 可复现验收 | **Docker**（保留 `build-dsh-image.sh` / `dev-up.sh`） | 产物可分发；与生产 Executor 解耦 |
| 生产单机池（4C/4G） | **systemd-run** 瞬时 unit | 零常驻、cgroup 原生、指标直读 |
| macOS / 单元测试 | **LocalProcessExecutor** | 无 systemd |
| 无 systemd 的 Linux | supervisord + 脚本写 cgroupfs | 运维熟悉 |
| 未来 ≥2 台 DSH | Nomad `raw_exec` 作 Executor | 门槛见 §5.2 |
| 明确不用 | k3s / Knative / 微 VM / 生产 docker run | 控制面或外壳税在 4G 上是负收益 |

**unit 模板（可直接拷用）**：

```ini
# systemd-run 参数等价；不要为每个租户事先落盘静态 .service
[Service]
ExecStart=/opt/muse-dsh/runtime/node --import tsx/esm apps/cli/src/bin.ts \
          --profile web --patch /opt/muse-dsh/patch.yml --host 127.0.0.1 --port %i
User=<tenant-uid>
WorkingDirectory=/opt/muse-dsh/runtime
Environment=DSH_HOME=/srv/muse-dsh/<hash>/home
Environment=PORT=%i DSH_LOOPBACK_PORT=%i
Environment=MUSE_REQUIRE_HOST_AUTH=1
Environment=MUSE_DOCUMENT_CLOUD_URL=https://<cloud-origin>
Environment=DEEPSEEK_API_KEY=…
MemoryMax=512M
CPUWeight=100
PidsMax=512
Restart=on-failure
StartLimitBurst=5
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ReadWritePaths=/srv/muse-dsh/<hash>
```

对应 `docker run`：`--memory 512m --cpus 0.5 --user <uid> --read-only` ——能力同源，多付守护进程与 shim。

---

## 7. 结论

- **资源**：每租户本体相同；Docker 多付 ~200MB 平台常驻 + shim；systemd 把这 200MB 还给第 6 个实例的余量或 GC。
- **并发**：4C 把 ACTIVE 提到 3；READY 仍是内存题。Docker 通常少 1 个 READY，恢复慢一个数量级。
- **调度**：不要为单机引入 Nomad/k3s。自研薄 Pool + 可替换 Executor；跨机器时再把 Nomad 接到同一接口。
- **运维**：systemd 零新采集组件。Docker 仅构建/CI。
