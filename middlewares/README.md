# middlewares

DSH plugins and runtime used by DSH Office. Do not modify `vendors/deepseek-harness` (that tree is workspace-only and not in the public clone).

| Path | Contents |
|---|---|
| [`dsh/`](dsh/README.md) | `@muse/*` packages (`core/` + `plugins/`) + 移动端宿主侧 Flutter 库 (`mobile/`) |
| [`dsh/deploy/`](dsh/deploy/README.md) | Remote DSH container and compose for local/dev |
| [`scripts/`](scripts/) | Build packages, image, stage runtime, run sidecar |
| [`docs/remote-dsh/`](docs/remote-dsh/ARCHITECTURE.zh-CN.md) | Architecture notes |
| [`docs/remote-dsh/MULTITENANCY.zh-CN.md`](docs/remote-dsh/MULTITENANCY.zh-CN.md) | Multi-tenant instance-pool plan (mobile/web remote) |
| [`docs/remote-dsh/RUNTIME-COMPARISON.zh-CN.md`](docs/remote-dsh/RUNTIME-COMPARISON.zh-CN.md) | Docker vs systemd-run vs supervisord (resources/concurrency/ops) |

```bash
./middlewares/scripts/build-muse-packages.sh
./middlewares/scripts/run-dsh-appflowy.sh          # local sidecar :3080
./middlewares/scripts/build-dsh-image.sh --load    # muse-dsh:local
./middlewares/dsh/deploy/dev-up.sh                 # DSH container only
./middlewares/scripts/measure-dsh-capacity.sh      # RSS/CPU baseline of the running remote DSH
```

Package map: [dsh/TECH_MAP.zh-CN.md](dsh/TECH_MAP.zh-CN.md).

Production multi-service deploy lives in the private `local/deploy/` workspace, not in this tree.
