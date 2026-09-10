import { afterEach, describe, expect, it } from "vitest";
import { createCloudMarkdownProvider } from "../src/host.js";
import { interpretCloudDocumentResponse } from "../src/cloud.js";
import {
  parseMarkdownSnapshotPayload,
  rememberMarkdownSnapshot,
  resetLastMarkdownSnapshot,
  MARKDOWN_SNAPSHOT_CONTEXT_TYPE
} from "../src/snapshot.js";
import { APPFLOWY_MARKDOWN_READ_OPERATION } from "../src/index.js";

describe("markdown Host snapshot (E4)", () => {
  afterEach(() => {
    resetLastMarkdownSnapshot();
  });

  it("E4-T4 returns NO_CURRENT_SELECTION when no focused view", async () => {
    const provider = createCloudMarkdownProvider();
    const result = await provider.invoke({
      operationId: APPFLOWY_MARKDOWN_READ_OPERATION,
      input: {},
      ctx: {}
    });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("NO_CURRENT_SELECTION");
    expect(result.message).toContain("NO_CURRENT_SELECTION");
    expect(result.message).not.toContain("NOT_FOUND");
  });

  it("maps Cloud NOT_FOUND to CLOUD_COLLAB_ADAPTER_NOT_WIRED", () => {
    const result = interpretCloudDocumentResponse(
      APPFLOWY_MARKDOWN_READ_OPERATION,
      { code: 1, message: "NOT_FOUND" }
    );
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.message).toBe("UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED");
  });

  it("falls back to Host snapshot when Cloud is unwired and viewId matches", async () => {
    rememberMarkdownSnapshot({
      viewId: "view-1",
      workspaceId: "ws-1",
      text: "# Getting started\n\nHello.",
      truncated: false,
      byteLength: Buffer.byteLength("# Getting started\n\nHello.")
    });
    const previous = globalThis.fetch;
    globalThis.fetch = (async () =>
      new Response(JSON.stringify({ code: 1067, message: "UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED" }), {
        status: 501
      })
    ) as typeof fetch;
    try {
      const provider = createCloudMarkdownProvider();
      const result = await provider.invoke({
        operationId: APPFLOWY_MARKDOWN_READ_OPERATION,
        input: { viewId: "view-1", workspaceId: "ws-1" },
        ctx: { cloudBaseUrl: "http://cloud.test", documentFocus: { workspaceId: "ws-1", viewId: "view-1" } }
      });
      expect(result.ok).toBe(true);
      if (!result.ok) throw new Error("expected ok");
      expect(JSON.stringify(result.value)).toContain("Hello.");
      expect(JSON.stringify(result.value)).toContain("muse.document/snapshot/v2");
    } finally {
      globalThis.fetch = previous;
    }
  });

  it("does not serve a snapshot for a different viewId", async () => {
    rememberMarkdownSnapshot({
      viewId: "view-1",
      text: "secret-ish body",
      truncated: false,
      byteLength: 15
    });
    const previous = globalThis.fetch;
    globalThis.fetch = (async () =>
      new Response(JSON.stringify({ code: 1, message: "NOT_FOUND" }), { status: 404 })
    ) as typeof fetch;
    try {
      const provider = createCloudMarkdownProvider();
      const result = await provider.invoke({
        operationId: APPFLOWY_MARKDOWN_READ_OPERATION,
        input: { viewId: "view-other" },
        ctx: { cloudBaseUrl: "http://cloud.test" }
      });
      expect(result.ok).toBe(false);
      if (result.ok) throw new Error("expected failure");
      expect(result.message).toContain("CLOUD_COLLAB_ADAPTER_NOT_WIRED");
    } finally {
      globalThis.fetch = previous;
    }
  });

  it("rejects snapshot payloads that contain secret substrings", () => {
    expect(MARKDOWN_SNAPSHOT_CONTEXT_TYPE).toBe("markdown.snapshot");
    expect(parseMarkdownSnapshotPayload({
      viewId: "v1",
      text: "leaked access_token here"
    })).toBeUndefined();
  });
});
