import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { readFile } from "node:fs/promises";
import type { Executor, InstanceHandle, InstanceSpec, InspectResult } from "./types.js";

const execFileAsync = promisify(execFile);

export const systemdRunArgv = (spec: InstanceSpec, runtimeBin: string, runtimeArgs: readonly string[]): string[] => {
  const unit = `muse-dsh-${spec.tenantHash}`;
  const argv = [
    "systemd-run",
    `--unit=${unit}`,
    "--collect",
    "--no-block",
    "--quiet",
    "--property=StandardInput=null",
    "--property=StandardOutput=journal",
    "--property=StandardError=journal",
    "--property=MemoryMax=" + `${Math.max(1, Math.ceil(spec.memoryMaxBytes / (1024 * 1024)))}M`,
    `--property=CPUWeight=${spec.cpuWeight}`,
    "--property=TasksMax=512",
    "--property=Restart=on-failure",
    "--property=NoNewPrivileges=yes",
    "--property=PrivateTmp=yes"
  ];
  if (spec.cwd) argv.push(`--working-directory=${spec.cwd}`);
  if (spec.envFile) argv.push(`--property=EnvironmentFile=${spec.envFile}`);
  if (spec.uid !== undefined) argv.push(`--uid=${spec.uid}`);
  argv.push(
    `--setenv=DSH_HOME=${spec.homeDir}`,
    `--setenv=PORT=${spec.port}`,
    `--setenv=DSH_LOOPBACK_PORT=${spec.port}`,
    `--setenv=MUSE_REQUIRE_HOST_AUTH=1`
  );
  for (const [key, value] of Object.entries(spec.env)) {
    argv.push(`--setenv=${key}=${value}`);
  }
  argv.push(runtimeBin, ...runtimeArgs);
  return argv;
};

export type RunFn = (argv: readonly string[]) => Promise<{ stdout: string; stderr: string }>;

const defaultRun: RunFn = async argv => {
  const [bin, ...args] = argv;
  if (bin === undefined) throw new Error("SYSTEMD_ARGV_EMPTY");
  return execFileAsync(bin, args, { encoding: "utf8" });
};

export class SystemdRunExecutor implements Executor {
  readonly kind = "systemd" as const;
  constructor(
    private readonly runtimeBin: string,
    private readonly runtimeArgs: readonly string[],
    private readonly run: RunFn = defaultRun
  ) {}

  async start(spec: InstanceSpec): Promise<InstanceHandle> {
    const unitName = `muse-dsh-${spec.tenantHash}`;
    await this.run(["systemctl", "stop", unitName]).catch(() => ({ stdout: "", stderr: "" }));
    await this.run(["systemctl", "reset-failed", unitName]).catch(() => ({ stdout: "", stderr: "" }));
    const argv = systemdRunArgv(spec, this.runtimeBin, this.runtimeArgs);
    const { stdout, stderr } = await this.run(argv);
    return {
      instanceRef: `inst.${spec.tenantHash}`,
      executor: "systemd",
      port: spec.port,
      startedAt: Date.now(),
      unitName,
      launchToken: /token=([A-Za-z0-9._~-]+)/.exec(`${stdout}\n${stderr}`)?.[1] ?? "pending",
      nodeId: "local"
    };
  }

  async stop(handle: InstanceHandle): Promise<void> {
    if (handle.unitName === undefined) return;
    await this.run(["systemctl", "stop", handle.unitName]).catch(() => undefined);
  }

  async inspect(handle: InstanceHandle): Promise<InspectResult> {
    const unit = handle.unitName ?? `muse-dsh-${handle.instanceRef.replace(/^inst\./u, "")}`;
    const path = `/sys/fs/cgroup/system.slice/${unit}.service`;
    try {
      const current = await readFile(`${path}/memory.current`, "utf8");
      const rss = Number.parseInt(current.trim(), 10);
      return { alive: true, memoryCurrentBytes: rss, rssBytes: rss };
    } catch {
      return { alive: false };
    }
  }

  async waitReady(handle: InstanceHandle, timeoutMs: number): Promise<void> {
    const deadline = Date.now() + timeoutMs;
    const { LocalProcessExecutor } = await import("./local-executor.js");
    const probe = new LocalProcessExecutor();
    while (Date.now() < deadline) {
      try {
        await probe.waitReady(handle, 200);
        return;
      } catch {
        /* retry until deadline */
      }
    }
    throw new Error("INSTANCE_START_TIMEOUT");
  }
}
