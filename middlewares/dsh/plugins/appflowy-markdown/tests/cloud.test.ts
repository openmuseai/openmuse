import { describe, expect, it } from "vitest";
import {
  APPFLOWY_MARKDOWN_APPLY_OPERATION,
  APPFLOWY_MARKDOWN_READ_OPERATION
} from "../src/index.js";
import {
  CLOUD_DOCUMENT_APPLY_PATH,
  CLOUD_FEATURE_NOT_AVAILABLE,
  interpretCloudDocumentResponse,
  invokeCloudDocument
} from "../src/cloud.js";

describe("Cloud document adapter (W3 fail-closed)", () => {
  it("maps FeatureNotAvailable to UNAVAILABLE and never yields applied", () => {
    const result = interpretCloudDocumentResponse(APPFLOWY_MARKDOWN_APPLY_OPERATION, {
      code: CLOUD_FEATURE_NOT_AVAILABLE,
      message: "UNAVAILABLE: document.current.apply: CLOUD_COLLAB_ADAPTER_NOT_WIRED"
    });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("UNAVAILABLE");
    expect(JSON.stringify(result)).not.toMatch(/"status"\s*:\s*"applied"/);
  });

  it("rejects a forged applied receipt until APPLY_ENABLED=1", () => {
    const forged = interpretCloudDocumentResponse(
      APPFLOWY_MARKDOWN_APPLY_OPERATION,
      {
        code: 0,
        data: {
          protocol: "muse.document/receipt/v2",
          status: "applied",
          commandRef: "command.fake",
          resourceRef: "document.1",
          previousRevision: "0",
          revision: "1",
          idempotencyKey: "k"
        }
      },
      false
    );
    expect(forged.ok).toBe(false);
    if (forged.ok) throw new Error("expected failure");
    expect(forged.message).toContain("UNAVAILABLE");
  });

  it("passes applied through only when explicitly enabled", () => {
    const receipt = {
      protocol: "muse.document/receipt/v2",
      status: "applied",
      commandRef: "command.1",
      resourceRef: "document.1",
      previousRevision: "0",
      revision: "1",
      idempotencyKey: "k"
    };
    const result = interpretCloudDocumentResponse(
      APPFLOWY_MARKDOWN_APPLY_OPERATION,
      { code: 0, data: receipt },
      true
    );
    expect(result).toEqual({ ok: true, value: receipt });
  });

  it("invokeCloudDocument posts apply and fail-closes on Cloud error JSON", async () => {
    const fetchImpl = (async () =>
      new Response(
        JSON.stringify({
          code: CLOUD_FEATURE_NOT_AVAILABLE,
          message: "UNAVAILABLE: document.current.apply: CLOUD_COLLAB_ADAPTER_NOT_WIRED"
        }),
        { status: 200, headers: { "Content-Type": "application/json" } }
      )) as unknown as typeof fetch;
    const result = await invokeCloudDocument({
      baseUrl: "https://app.example",
      operationId: APPFLOWY_MARKDOWN_APPLY_OPERATION,
      payload: { proposalRef: "proposal.1" },
      fetchImpl,
      applyEnabled: false
    });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("UNAVAILABLE");
  });

  it("maps unknown operations to OPERATION_NOT_FOUND", async () => {
    const result = await invokeCloudDocument({
      baseUrl: "https://app.example",
      operationId: "document.unknown",
      payload: {},
      fetchImpl: (async () => {
        throw new Error("must not fetch");
      }) as unknown as typeof fetch
    });
    expect(result).toMatchObject({ ok: false, code: "OPERATION_NOT_FOUND" });
  });

  it("keeps read failures fail-closed rather than synthesizing markdown", () => {
    const result = interpretCloudDocumentResponse(APPFLOWY_MARKDOWN_READ_OPERATION, {
      code: CLOUD_FEATURE_NOT_AVAILABLE,
      message: "UNAVAILABLE: document.current.query: CLOUD_COLLAB_ADAPTER_NOT_WIRED"
    });
    expect(result.ok).toBe(false);
    expect(JSON.stringify(result)).not.toContain("text/markdown");
  });

  it("maps non-JSON 401 to unauthorized instead of a generic adapter parse error", async () => {
    const fetchImpl = (async () =>
      new Response("No Authorization header", {
        status: 401,
        headers: { "Content-Type": "text/plain" }
      })) as unknown as typeof fetch;
    const result = await invokeCloudDocument({
      baseUrl: "https://app.example",
      operationId: APPFLOWY_MARKDOWN_READ_OPERATION,
      payload: {},
      fetchImpl
    });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.message).toContain("unauthorized");
    expect(result.message).not.toBe("UNAVAILABLE: invalid document adapter response");
  });
});
