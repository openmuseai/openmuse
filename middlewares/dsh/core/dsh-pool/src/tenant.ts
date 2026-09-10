import { createHash, createHmac } from "node:crypto";

export const tenantKeyOf = (accountRef: string, workspaceRef: string, salt: string): string =>
  createHash("sha256").update(`${salt}\0${accountRef}\0${workspaceRef}`).digest("hex");

/** 128-bit public id; not reversible to account/workspace without the salt. */
export const tenantHashOf = (tenantKey: string): string => tenantKey.slice(0, 32);

export const sessionRefOf = (tenantKey: string): string => `session.${tenantKey.slice(0, 16)}`;

export const uidForTenant = (tenantKey: string, min = 16000, span = 1000): number => {
  const n = Number.parseInt(tenantKey.slice(0, 8), 16);
  return min + (Number.isFinite(n) ? n % span : 0);
};

const LAUNCH_TOKEN = /(?:dsh web:|[?&]token=)([A-Za-z0-9._~-]{8,})/i;

export const extractLaunchToken = (text: string): string | undefined => {
  const match = LAUNCH_TOKEN.exec(text);
  return match?.[1];
};

export const webUrlOf = (publicBase: string, tenantHash: string, launchToken: string): string => {
  const base = publicBase.replace(/\/+$/u, "");
  const url = new URL(`${base}/u/${tenantHash}/`);
  if (launchToken && launchToken !== "pending") url.searchParams.set("token", launchToken);
  return url.toString();
};

export const hmacTenant = (secret: string, value: string): string =>
  createHmac("sha256", secret).update(value).digest("hex");
