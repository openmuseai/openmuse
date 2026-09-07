import type { Stats } from "node:fs";
import { stat, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  unixDomainSocketEndpoint,
  windowsNamedPipeEndpoint
} from "@muse/host-bridge";

export interface NativeLaunch {
  readonly endpoint: string;
  readonly nonce: string;
  readonly hostGeneration: string;
  readonly runtimeInstanceId: string;
}

const validOpaque = (value: unknown): value is string =>
  typeof value === "string" && /^[A-Za-z0-9._~-]{1,128}$/u.test(value);

/** Sidecar looks up the Core-written launch descriptor. Windows has no getuid. */
export const launchPath = (env: NodeJS.ProcessEnv = process.env): string =>
  env.MUSE_APPFLOWY_LAUNCH_FILE
  ?? join(tmpdir(), `appflowy-muse-host-${process.getuid?.() ?? 0}.json`);

/**
 * Platform endpoint shape only. Unix stays absolute UDS paths; Windows stays
 * `\\.\pipe\...`. Calling this from Web/Cloud paths is a bug — those use
 * InProcess transport.
 */
export const assertNativeEndpoint = (
  endpoint: string,
  platform: NodeJS.Platform = process.platform
): void => {
  if (platform === "win32") {
    windowsNamedPipeEndpoint(endpoint);
    return;
  }
  unixDomainSocketEndpoint(endpoint);
};

/** POSIX 0600 + owner uid. Windows user temp has no meaningful mode bits. */
export const isPrivateLaunchFile = (
  metadata: Pick<Stats, "isFile" | "uid" | "mode">,
  platform: NodeJS.Platform = process.platform,
  getuid: (() => number) | undefined = process.getuid
): boolean => {
  if (!metadata.isFile()) return false;
  if (platform === "win32") return true;
  if (getuid !== undefined && metadata.uid !== getuid()) return false;
  return (metadata.mode & 0o077) === 0;
};

export const loadNativeLaunch = async (
  env: NodeJS.ProcessEnv = process.env,
  platform: NodeJS.Platform = process.platform
): Promise<NativeLaunch> => {
  const path = launchPath(env);
  const metadata = await stat(path);
  if (!isPrivateLaunchFile(metadata, platform)) {
    throw new Error("AppFlowy Muse launch descriptor is not private");
  }
  const value = JSON.parse(await readFile(path, "utf8")) as Partial<NativeLaunch>;
  if (
    typeof value.endpoint !== "string"
    || !validOpaque(value.hostGeneration)
    || !validOpaque(value.runtimeInstanceId)
    || typeof value.nonce !== "string"
    || value.nonce.length < 32
  ) {
    throw new Error("AppFlowy Muse launch descriptor is invalid");
  }
  try {
    assertNativeEndpoint(value.endpoint, platform);
  } catch {
    throw new Error("AppFlowy Muse launch descriptor is invalid");
  }
  return value as NativeLaunch;
};
