import { mkdir } from "node:fs/promises";
import { FakeExecutor } from "./executor.js";
import { LocalProcessExecutor } from "./local-executor.js";
import { SystemdRunExecutor } from "./systemd-executor.js";
import { InstancePool } from "./pool.js";
import { createControlServer } from "./server.js";
import { createPoolProxy } from "./proxy.js";
import type { Executor } from "./types.js";

const env = (name: string, fallback: string): string => process.env[name]?.trim() || fallback;

const executorOf = (): Executor => {
  const kind = env("MUSE_DSH_EXECUTOR", "local");
  if (kind === "systemd") {
    const args = env("MUSE_DSH_RUNTIME_ARGS", "")
      .split(" ")
      .map(part => part.trim())
      .filter(Boolean);
    return new SystemdRunExecutor(env("MUSE_DSH_RUNTIME_BIN", "/opt/muse-dsh/runtime/start-instance.sh"), args);
  }
  if (kind === "fake") return new FakeExecutor();
  return new LocalProcessExecutor();
};

const instanceEnv: Record<string, string> = {};
for (const key of ["DSH_TRUSTED_HOST", "MUSE_DOCUMENT_CLOUD_URL", "PATCH", "HARNESS"] as const) {
  const value = process.env[key]?.trim();
  if (value) instanceEnv[key] = value;
}
if (!instanceEnv.DSH_TRUSTED_HOST) instanceEnv.DSH_TRUSTED_HOST = "openmuseai.com,127.0.0.1";
if (!instanceEnv.HARNESS) instanceEnv.HARNESS = "/opt/muse-dsh/runtime/dsh";
if (!instanceEnv.PATCH) instanceEnv.PATCH = "/opt/muse-dsh/runtime/patch.yml";
if (!instanceEnv.MUSE_DSH_NODE_BIN) {
  instanceEnv.MUSE_DSH_NODE_BIN = env("MUSE_DSH_NODE_BIN", "/opt/muse-dsh/runtime/bin/node");
}
instanceEnv.MUSE_DSH_INSTANCE_ENV_FILE = env("MUSE_DSH_INSTANCE_ENV_FILE", "/opt/muse-dsh/instance.env");

const homeRoot = env("MUSE_DSH_HOME_ROOT", "/srv/muse-dsh");
await mkdir(homeRoot, { recursive: true });
const cwd = env("MUSE_DSH_HARNESS", instanceEnv.HARNESS);
const envFile = process.env.MUSE_DSH_SYSTEMD_ENV_FILE?.trim();
const pool = new InstancePool(executorOf(), {
  readyQuota: Number(env("MUSE_DSH_READY_QUOTA", "3")),
  activeQuota: Number(env("MUSE_DSH_ACTIVE_QUOTA", "3")),
  idleTtlMs: Number(env("MUSE_DSH_IDLE_TTL_MS", String(20 * 60 * 1000))),
  portBase: Number(env("MUSE_DSH_PORT_BASE", "13080")),
  homeRoot,
  publicBase: env("MUSE_DSH_PUBLIC_BASE", "https://localhost"),
  tenantSalt: env("MUSE_DSH_TENANT_SALT", "change-me"),
  nodeId: env("MUSE_DSH_NODE_ID", "local"),
  memoryMaxBytes: Number(env("MUSE_DSH_MEMORY_MAX_BYTES", String(512 * 1024 * 1024))),
  cpuWeight: Number(env("MUSE_DSH_CPU_WEIGHT", "100")),
  assignUid: env("MUSE_DSH_ASSIGN_UID", "") === "1",
  cwd,
  ...(envFile ? { envFile } : {}),
  instanceEnv
});

const controlPort = Number(env("MUSE_DSH_POOL_PORT", "13079"));
const control = createControlServer(pool);
const proxy = createPoolProxy(pool);
await new Promise<void>(resolve => control.listen(controlPort, "127.0.0.1", resolve));
await new Promise<void>(resolve => proxy.listen(controlPort + 1, "127.0.0.1", resolve));
setInterval(() => {
  void pool.tick();
}, 5_000).unref();
process.stdout.write(`dsh-pool control http://127.0.0.1:${controlPort} proxy http://127.0.0.1:${controlPort + 1}\n`);
