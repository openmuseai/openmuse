import { createHmac } from "node:crypto";
import { mkdtemp, writeFile, chmod } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { createHostHmacApprovalRequester, expectedHostProofId } from "../src/approval.js";

describe("Host HMAC approval requester", () => {
  it("mints a proof from the private AppFlowy approval descriptor", async () => {
    const directory = await mkdtemp(join(tmpdir(), "muse-approval-"));
    const path = join(directory, "approval.json");
    const secret = "a".repeat(64);
    await writeFile(path, JSON.stringify({ secret }));
    await chmod(path, 0o600);
    const previous = process.env.MUSE_APPFLOWY_APPROVAL_FILE;
    process.env.MUSE_APPFLOWY_APPROVAL_FILE = path;
    try {
      const answer = await createHostHmacApprovalRequester().request({
        approvalId: "approval.1" as never,
        signal: AbortSignal.timeout(1_000)
      });
      expect(answer).toEqual({
        outcome: "approved",
        proofId: expectedHostProofId(secret, "approval.1")
      });
      expect(answer.outcome === "approved" && answer.proofId).toBe(
        `proof.${createHmac("sha256", secret).update("approval.1").digest("hex")}`
      );
    } finally {
      if (previous === undefined) delete process.env.MUSE_APPFLOWY_APPROVAL_FILE;
      else process.env.MUSE_APPFLOWY_APPROVAL_FILE = previous;
    }
  });
});
