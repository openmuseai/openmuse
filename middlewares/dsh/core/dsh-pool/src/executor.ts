import type { Executor, InstanceHandle, InstanceSpec, InspectResult } from "./types.js";

export class FakeExecutor implements Executor {
  readonly kind = "local" as const;
  readonly started: InstanceSpec[] = [];
  readonly stopped: string[] = [];
  launchToken = "launch.fake";
  alive = true;
  rssBytes = 120 * 1024 * 1024;

  async start(spec: InstanceSpec): Promise<InstanceHandle> {
    this.started.push(spec);
    return {
      instanceRef: `inst.${spec.tenantHash}`,
      executor: "local",
      port: spec.port,
      startedAt: Date.now(),
      pid: 1,
      launchToken: this.launchToken,
      nodeId: "local"
    };
  }

  async stop(handle: InstanceHandle): Promise<void> {
    this.stopped.push(handle.instanceRef);
  }

  async inspect(): Promise<InspectResult> {
    return { alive: this.alive, rssBytes: this.rssBytes, memoryCurrentBytes: this.rssBytes };
  }

  orphans = new Map<string, { port: number; unitName: string; pid?: number }>();

  async findRunning(tenantHash: string): Promise<{ port: number; unitName: string; pid?: number } | undefined> {
    return this.orphans.get(tenantHash);
  }

  async waitReady(): Promise<void> {
    /* FakeExecutor is ready immediately. */
  }
}

export const isJwtAuthorization = (value: string): boolean =>
  value.startsWith("Bearer ") && value.slice(7).split(".").length === 3;
