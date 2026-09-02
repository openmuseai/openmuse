# middlewares

DSH plugins and runtime used by DSH Office. Do not modify `vendors/deepseek-harness` (that tree is workspace-only and not in the public clone).

| Path | Contents |
|---|---|
| [`dsh/`](dsh/README.md) | `@muse/*` packages (`core/` + `plugins/`) |
| [`dsh/deploy/`](dsh/deploy/README.md) | Remote DSH container and compose for local/dev |
| [`scripts/`](scripts/) | Build packages, image, stage runtime, run sidecar |
| [`docs/remote-dsh/`](docs/remote-dsh/ARCHITECTURE.zh-CN.md) | Architecture notes |

```bash
./middlewares/scripts/build-muse-packages.sh
./middlewares/scripts/run-dsh-appflowy.sh          # local sidecar :3080
./middlewares/scripts/build-dsh-image.sh --load    # muse-dsh:local
./middlewares/dsh/deploy/dev-up.sh                 # DSH container only
```

Package map: [dsh/TECH_MAP.zh-CN.md](dsh/TECH_MAP.zh-CN.md).

Production multi-service deploy lives in the private `local/deploy/` workspace, not in this tree.
