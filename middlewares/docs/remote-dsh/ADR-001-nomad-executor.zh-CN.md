# ADR-001：单机不引入 Nomad；跨机器再评估 raw_exec Executor

- 状态：**Accepted**（2026-09-09）
- 范围：Remote DSH 实例池调度

## 上下文

4C/4G 同机 Cloud。需要每租户一进程、idle 回收、等待室、launch token。未来可能多机，本期不做。

## 决策

1. 控制面（`@muse/dsh-pool`）自研，不替换为 Nomad/k3s Job spec。
2. 执行器接口可替换：现在 `local` / `systemd`；`nomad` 仅类型预留。
3. 仅当下列**全部**成立才实现 `NomadExecutor`：
   - `SystemdRunExecutor` 已在 Linux 功能验收；
   - 明确第二台 DSH 机器，或目标环境无 systemd；
   - Nomad client RSS 实测 < 80MB，且能 UID + 内存限额；
   - 不强制 Consul。

## 后果

- 单机零额外调度常驻。
- 跨机器时替换 Executor，session/open 契约不变。
- 拒绝 k3s/Knative/生产 docker run 作为实例载体。
