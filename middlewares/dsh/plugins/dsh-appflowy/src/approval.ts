import { createHmac } from "node:crypto";
import { readFile, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { MuseApprovalProofRequester } from "@muse/host-bridge/dsh";

const validOpaque = (value: unknown): value is string =>
  typeof value === "string" && /^[A-Za-z0-9._~-]{1,128}$/u.test(value);

const approvalPath = (): string => process.env.MUSE_APPFLOWY_APPROVAL_FILE
  ?? join(tmpdir(), `appflowy-muse-approval-${process.getuid?.() ?? 0}.json`);

const loadApprovalSecret = async (): Promise<string> => {
  const path = approvalPath();
  const metadata = await stat(path);
  if (!metadata.isFile() || (process.getuid !== undefined && metadata.uid !== process.getuid())) {
    throw new Error("AppFlowy Muse approval descriptor is not owned by the current user");
  }
  if ((metadata.mode & 0o077) !== 0) throw new Error("AppFlowy Muse approval descriptor is not private");
  const value = JSON.parse(await readFile(path, "utf8")) as { secret?: unknown };
  if (typeof value.secret !== "string" || value.secret.length < 32) {
    throw new Error("AppFlowy Muse approval descriptor is invalid");
  }
  return value.secret;
};

export const expectedHostProofId = (secret: string, approvalId: string): string =>
  `proof.${createHmac("sha256", secret).update(approvalId).digest("hex")}`;

/** Deployment-owned requester. Plugins never see the HMAC secret. */
export const createHostHmacApprovalRequester = (): MuseApprovalProofRequester => ({
  async request({ approvalId }) {
    if (!validOpaque(approvalId)) return { outcome: "unavailable" };
    try {
      const secret = await loadApprovalSecret();
      return { outcome: "approved", proofId: expectedHostProofId(secret, approvalId) };
    } catch {
      return { outcome: "unavailable" };
    }
  }
});
