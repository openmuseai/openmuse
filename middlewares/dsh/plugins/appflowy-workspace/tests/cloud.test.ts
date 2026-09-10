import { describe, expect, it } from "vitest";
import {
  CLOUD_WORKSPACE_CURRENT_PATH,
  CLOUD_WORKSPACE_TREE_PATH,
  WORKSPACE_CURRENT_OPERATION,
  WORKSPACE_TREE_OPERATION,
  interpretCloudWorkspaceResponse,
  invokeCloudWorkspace,
  pathForWorkspaceOperation
} from "../src/cloud.js";

describe("Cloud workspace adapter (W7 tree)", () => {
  it("maps workspace operations to Cloud HTTP paths", () => {
    expect(pathForWorkspaceOperation(WORKSPACE_CURRENT_OPERATION)).toBe(CLOUD_WORKSPACE_CURRENT_PATH);
    expect(pathForWorkspaceOperation(WORKSPACE_TREE_OPERATION)).toBe(CLOUD_WORKSPACE_TREE_PATH);
    expect(pathForWorkspaceOperation("workspace.unknown")).toBeUndefined();
  });

  it("accepts a bounded tree projection and never treats it as document bodies", () => {
    const result = interpretCloudWorkspaceResponse({
      code: 0,
      data: {
        protocol: "muse.workspace/tree/v1",
        workspaceId: "w1",
        rootViewId: "w1",
        truncated: false,
        items: [{
          viewId: "v1",
          parentViewId: "w1",
          title: "Getting started",
          layout: "document",
          isSpace: false,
          depth: 1
        }]
      }
    });
    expect(result.ok).toBe(true);
    if (!result.ok) throw new Error("expected ok");
    expect(JSON.stringify(result.value)).toContain("Getting started");
    expect(JSON.stringify(result.value)).not.toContain("text/markdown");
  });

  it("maps SCOPE_MISMATCH from Cloud error JSON", () => {
    const result = interpretCloudWorkspaceResponse({
      code: 1,
      message: "SCOPE_MISMATCH"
    });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("SCOPE_MISMATCH");
  });

  it("maps NOT_FOUND to CLOUD_COLLAB_ADAPTER_NOT_WIRED", () => {
    const result = interpretCloudWorkspaceResponse({ code: 1, message: "NOT_FOUND" });
    expect(result.ok).toBe(false);
    if (result.ok) throw new Error("expected failure");
    expect(result.code).toBe("UNAVAILABLE");
    expect(result.message).toBe("UNAVAILABLE: CLOUD_COLLAB_ADAPTER_NOT_WIRED");
  });

  it("posts tree query to Cloud", async () => {
    const fetchImpl = (async (_url: string, init?: RequestInit) => {
      const body = JSON.parse(String(init?.body)) as { operation?: string; workspaceId?: string };
      expect(body.operation).toBe(WORKSPACE_TREE_OPERATION);
      expect(body.workspaceId).toBe("w1");
      return new Response(
        JSON.stringify({
          code: 0,
          data: {
            protocol: "muse.workspace/tree/v1",
            workspaceId: "w1",
            rootViewId: "w1",
            truncated: false,
            items: []
          }
        }),
        { status: 200, headers: { "Content-Type": "application/json" } }
      );
    }) as unknown as typeof fetch;
    const result = await invokeCloudWorkspace({
      baseUrl: "https://app.example",
      operationId: WORKSPACE_TREE_OPERATION,
      payload: { workspaceId: "w1" },
      fetchImpl
    });
    expect(result.ok).toBe(true);
  });
});
