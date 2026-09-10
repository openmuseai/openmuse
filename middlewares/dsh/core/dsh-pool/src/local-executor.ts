import { spawn, type ChildProcess } from "node:child_process";
import { request as httpRequest } from "node:http";
import type { Executor, InstanceHandle, InstanceSpec, InspectResult } from "./types.js";
import { extractLaunchToken } from "./tenant.js";

const children = new Map<string, ChildProcess>();

const probe = (port: number, path: string): Promise<boolean> =>
  new Promise(resolve => {
    const req = httpRequest({ host: "127.0.0.1", port, path, method: "GET", timeout: 500 }, res => {
      res.resume();
      resolve((res.statusCode ?? 500) < 500);
    });
    req.on("error", () => resolve(false));
    req.on("timeout", () => {
      req.destroy();
      resolve(false);
    });
    req.end();
  });

export class LocalProcessExecutor implements Executor {
  readonly kind = "local" as const;

  async start(spec: InstanceSpec): Promise<InstanceHandle> {
    const argv = spec.command ?? ["node", "-e", "process.stdin.resume()"];
    const child = spawn(argv[0] ?? "node", argv.slice(1), {
      cwd: spec.cwd,
      env: { ...process.env, ...spec.env, PORT: String(spec.port), DSH_LOOPBACK_PORT: String(spec.port) },
      stdio: ["ignore", "pipe", "pipe"]
    });
    let logs = "";
    child.stdout?.on("data", chunk => {
      logs += String(chunk);
    });
    child.stderr?.on("data", chunk => {
      logs += String(chunk);
    });
    const instanceRef = `inst.${spec.tenantHash}`;
    children.set(instanceRef, child);
    const startedAt = Date.now();
    const pid = child.pid;
    const pending: InstanceHandle = {
      instanceRef,
      executor: "local",
      port: spec.port,
      startedAt,
      nodeId: "local",
      ...(pid === undefined ? {} : { pid })
    };
    await this.waitReady(pending, 15_000);
    return {
      ...pending,
      launchToken: extractLaunchToken(logs) ?? "local.dev"
    };
  }

  async stop(handle: InstanceHandle): Promise<void> {
    const child = children.get(handle.instanceRef);
    children.delete(handle.instanceRef);
    if (child === undefined || child.killed) return;
    child.kill("SIGTERM");
    await new Promise<void>(resolve => {
      const timer = setTimeout(() => {
        child.kill("SIGKILL");
        resolve();
      }, 2_000);
      child.once("exit", () => {
        clearTimeout(timer);
        resolve();
      });
    });
  }

  async inspect(handle: InstanceHandle): Promise<InspectResult> {
    const child = children.get(handle.instanceRef);
    if (child === undefined || child.exitCode !== null) return { alive: false };
    return { alive: true, rssBytes: 0 };
  }

  async waitReady(handle: InstanceHandle, timeoutMs: number): Promise<void> {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      if (await probe(handle.port, "/muse/v1/parent-bridge/capabilities")) return;
      if (await probe(handle.port, "/")) return;
      await new Promise(resolve => setTimeout(resolve, 50));
    }
    throw new Error("INSTANCE_START_TIMEOUT");
  }
}
