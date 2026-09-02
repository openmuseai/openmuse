# Remote DSH (local / self-host)

Run the DSH sidecar as a container on loopback. Production multi-host playbooks are **not** in this open-source tree.

## Image

```bash
./middlewares/scripts/build-dsh-image.sh          # linux/amd64, tag muse-dsh:local
./middlewares/scripts/build-dsh-image.sh --load   # load into local Docker
```

Do not copy a macOS `stage-dsh-runtime.sh` tree into a Linux image (wrong Node ABI). The image uses `node:22.19.0-bookworm` and `pnpm install` on Linux.

## Local process (no Docker)

```bash
./middlewares/scripts/run-dsh-appflowy.sh
```

Listens on `127.0.0.1:3080` by default.

## Local Docker

```bash
./middlewares/dsh/deploy/dev-up.sh
./middlewares/dsh/deploy/dev-up.sh --rebuild
./middlewares/dsh/deploy/dev-up.sh --down
```

`dev-up.sh` copies `middlewares/dsh/deploy/.env` from `.env.dsh.local` or `local/.env.dsh.local` if present. Do not commit `.env`.

Manual:

```bash
./middlewares/scripts/build-dsh-image.sh --load --platform linux/arm64
# put DEEPSEEK_API_KEY in middlewares/dsh/deploy/.env
docker compose -f middlewares/dsh/deploy/docker-compose.yml up -d
```

## Safety

- Map the container to `127.0.0.1:${DSH_PORT}:3080`, not `0.0.0.0:3080`.
- Public TLS, if you add it, belongs in your own reverse proxy, not this repo.

## Android client (built on a developer machine)

```bash
./frontend/client/scripts/build-android-client.sh --debug
./frontend/client/scripts/install-android-client.sh
```
