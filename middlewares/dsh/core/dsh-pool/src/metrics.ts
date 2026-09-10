import { readFile } from "node:fs/promises";
import type { InstanceHandle, InspectResult } from "./types.js";

/** cgroup v2 counters when the unit exists; otherwise empty. Never throws. */
export const readCgroupMetrics = async (handle: InstanceHandle): Promise<InspectResult> => {
  const unit = handle.unitName;
  if (unit === undefined) return { alive: handle.pid !== undefined };
  const dir = `/sys/fs/cgroup/system.slice/${unit}.service`;
  try {
    const current = Number.parseInt((await readFile(`${dir}/memory.current`, "utf8")).trim(), 10);
    let cpuUsageUsec: number | undefined;
    try {
      const stat = await readFile(`${dir}/cpu.stat`, "utf8");
      const match = /usage_usec (\d+)/.exec(stat);
      if (match?.[1] !== undefined) cpuUsageUsec = Number.parseInt(match[1], 10);
    } catch {
      /* cpu.stat optional */
    }
    return {
      alive: true,
      memoryCurrentBytes: current,
      rssBytes: current,
      ...(cpuUsageUsec === undefined ? {} : { cpuUsageUsec })
    };
  } catch {
    return { alive: false };
  }
};
