import { describe, expect, it } from "vitest";
import { extractIngressToken, issueDeviceToken, verifyDeviceTokenSigned, createMuseBff, redactLogText } from "../src/muse-bff.js";

describe("muse-bff tokens", () => {
  it("extracts bearer, device header, then cookie", () => {
    expect(extractIngressToken("Bearer abc.def", undefined, undefined)).toBe("abc.def");
    expect(extractIngressToken(undefined, "device.token", undefined)).toBe("device.token");
    expect(extractIngressToken(undefined, undefined, "theme=dark; access_token=cookie.jwt")).toBe(
      "cookie.jwt"
    );
    expect(extractIngressToken(undefined, undefined, "theme=dark")).toBeUndefined();
  });

  it("E3-T8 strips tokens from log text", () => {
    expect(redactLogText("https://openmuseai.com/u/ab/?token=secret.jwt")).toContain("token=redacted");
    expect(redactLogText("Bearer abc.def")).toBe("Bearer redacted");
    expect(redactLogText("access_token=cookie.jwt")).toBe("access_token=redacted");
  });

  it("issues a two-segment HMAC token that round-trips", () => {
    const token = issueDeviceToken("secret", "v1", "sess", "web.1", "actor-1", 9_999_999_999_999, "nonce-1");
    expect(token.startsWith("sk-")).toBe(false);
    expect(token.split(".").length).toBe(2);
    const claims = verifyDeviceTokenSigned("secret", "v1", token, 1);
    expect(claims.actorRef).toBe("actor-1");
    expect(claims.deviceId).toBe("web.1");
    expect(claims.kid).toBe("v1");
  });
});

describe("muse-bff http", () => {
  const listen = async () => {
    const members = new Map<string, { workspaceId: string; title: string; role: string }>([
      ["user-1:ws-1", { workspaceId: "ws-1", title: "Team", role: "owner" }]
    ]);
    const server = createMuseBff({
      gotrueUrl: "http://gotrue.test",
      tokenSecret: "secret",
      tokenKid: "v1",
      tokenTtlSecs: 900,
      nowMs: () => 1_000,
      verifyJwt: async token => (token === "jwt-ok" ? "user-1" : undefined),
      queryWorkspace: async (uuid, workspaceId) => members.get(`${uuid}:${workspaceId}`)
    });
    await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
    const addr = server.address();
    if (!addr || typeof addr === "string") throw new Error("listen");
    return { server, base: `http://127.0.0.1:${addr.port}` };
  };

  it("rejects anonymous ingress and issues a device token for a JWT", async () => {
    const { server, base } = await listen();
    try {
      const denied = await fetch(`${base}/api/muse/dsh/ingress-auth`);
      expect(denied.status).toBe(401);
      const issued = await fetch(`${base}/api/muse/dsh/device-token`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ deviceId: "web.1" })
      });
      expect(issued.status).toBe(200);
      const body = (await issued.json()) as { code: number; data: { token: string; deviceId: string } };
      expect(body.code).toBe(0);
      expect(body.data.deviceId).toBe("web.1");
      expect(body.data.token.split(".").length).toBe(2);
      const authed = await fetch(`${base}/api/muse/dsh/ingress-auth`, {
        headers: { "x-muse-device-token": body.data.token }
      });
      expect(authed.status).toBe(200);
      const current = await fetch(`${base}/api/muse/workspace/current`, {
        method: "POST",
        headers: {
          authorization: `Bearer ${body.data.token}`,
          "x-muse-device-id": "web.1",
          "content-type": "application/json"
        },
        body: JSON.stringify({ workspaceId: "ws-1" })
      });
      expect(current.status).toBe(200);
      const projection = (await current.json()) as { code: number; data: { workspaceId: string } };
      expect(projection.data.workspaceId).toBe("ws-1");
      const foreign = await fetch(`${base}/api/muse/workspace/current`, {
        method: "POST",
        headers: { authorization: `Bearer ${body.data.token}`, "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-other" })
      });
      expect(foreign.status).toBe(400);
      const session = await fetch(`${base}/api/muse/dsh/session/open`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-1" })
      });
      expect(session.status).toBe(503);
    } finally {
      server.close();
    }
  });

  it("proxies session/open to the pool after membership check", async () => {
    const members = new Map<string, { workspaceId: string; title: string; role: string }>([
      ["user-1:ws-1", { workspaceId: "ws-1", title: "Team", role: "owner" }]
    ]);
    const calls: Array<{ url: string; body: Record<string, unknown> }> = [];
    const server = createMuseBff({
      gotrueUrl: "http://gotrue.test",
      tokenSecret: "secret",
      tokenKid: "v1",
      tokenTtlSecs: 900,
      nowMs: () => 1_000,
      verifyJwt: async token => (token === "jwt-ok" ? "user-1" : undefined),
      queryWorkspace: async (uuid, workspaceId) => members.get(`${uuid}:${workspaceId}`),
      poolUrl: "http://127.0.0.1:13079",
      fetchImpl: (async (url, init) => {
        const parsed = JSON.parse(String(init?.body ?? "{}")) as Record<string, unknown>;
        calls.push({ url: String(url), body: parsed });
        return new Response(
          JSON.stringify({
            sessionRef: "session.abc",
            webUrl: "https://openmuseai.com/u/abcd/",
            nodeId: "local"
          }),
          { status: 200, headers: { "content-type": "application/json" } }
        );
      }) as typeof fetch
    });
    await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
    const addr = server.address();
    if (!addr || typeof addr === "string") throw new Error("listen");
    const base = `http://127.0.0.1:${addr.port}`;
    try {
      const denied = await fetch(`${base}/api/muse/dsh/session/open`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-other" })
      });
      expect(denied.status).toBe(400);
      expect(calls).toHaveLength(0);
      const opened = await fetch(`${base}/api/muse/dsh/session/open`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-1", deviceId: "web.1" })
      });
      expect(opened.status).toBe(200);
      const body = (await opened.json()) as { code: number; data: { webUrl: string; sessionRef: string } };
      expect(body.code).toBe(0);
      expect(body.data.webUrl).toContain("/u/");
      expect(calls[0]?.url).toBe("http://127.0.0.1:13079/internal/session/open");
      expect(calls[0]?.body.accountRef).toBe("user-1");
      expect(calls[0]?.body.workspaceRef).toBe("ws-1");
      expect(calls[0]?.body.deviceId).toBe("web.1");
    } finally {
      server.close();
    }
  });

  it("E3-T3/T4 logs X-Muse-Attachment-Id without the bearer", async () => {
    const logs: Array<Record<string, unknown>> = [];
    const server = createMuseBff({
      gotrueUrl: "http://gotrue.test",
      tokenSecret: "secret",
      tokenKid: "v1",
      tokenTtlSecs: 900,
      nowMs: () => 1_000,
      verifyJwt: async token => (token === "jwt-ok" ? "user-1" : undefined),
      queryWorkspace: async () => ({ workspaceId: "ws-1", title: "Team", role: "owner" }),
      poolUrl: "http://127.0.0.1:13079",
      onAccessLog: entry => logs.push(entry),
      fetchImpl: (async () =>
        new Response(JSON.stringify({ sessionRef: "session.abc", webUrl: "https://openmuseai.com/u/abcd/" }), {
          status: 200,
          headers: { "content-type": "application/json" }
        })) as typeof fetch
    });
    await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
    const addr = server.address();
    if (!addr || typeof addr === "string") throw new Error("listen");
    const base = `http://127.0.0.1:${addr.port}`;
    try {
      const withoutHeader = await fetch(`${base}/api/muse/dsh/session/open`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-1", deviceId: "web.1" })
      });
      expect(withoutHeader.status).toBe(200);
      expect(logs.at(-1)?.attachmentId).toBeUndefined();
      const withHeader = await fetch(`${base}/api/muse/dsh/session/open`, {
        method: "POST",
        headers: {
          authorization: "Bearer jwt-ok",
          "content-type": "application/json",
          "x-muse-attachment-id": "att_trace-1"
        },
        body: JSON.stringify({ workspaceId: "ws-1", deviceId: "web.1" })
      });
      expect(withHeader.status).toBe(200);
      expect(logs.at(-1)?.attachmentId).toBe("att_trace-1");
      expect(JSON.stringify(logs.at(-1))).not.toMatch(/jwt-ok|Bearer/);
    } finally {
      server.close();
    }
  });

  it("E4-T1/T2 returns 501 CLOUD_COLLAB_ADAPTER_NOT_WIRED instead of NOT_FOUND", async () => {
    const { server, base } = await listen();
    try {
      const tokenRes = await fetch(`${base}/api/muse/dsh/device-token`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ deviceId: "web.1" })
      });
      const tokenBody = (await tokenRes.json()) as { data: { token: string } };
      for (const path of ["/api/muse/workspace/tree", "/api/muse/document/query"] as const) {
        const res = await fetch(`${base}${path}`, {
          method: "POST",
          headers: {
            authorization: `Bearer ${tokenBody.data.token}`,
            "x-muse-device-id": "web.1",
            "content-type": "application/json"
          },
          body: JSON.stringify({ workspaceId: "ws-1" })
        });
        expect(res.status).toBe(501);
        const body = (await res.json()) as { code: number; message: string };
        expect(body.code).toBe(1067);
        expect(body.message).toContain("CLOUD_COLLAB_ADAPTER_NOT_WIRED");
        expect(body.message).not.toContain("NOT_FOUND");
      }
      const anon = await fetch(`${base}/api/muse/workspace/tree`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-1" })
      });
      expect(anon.status).toBe(401);
    } finally {
      server.close();
    }
  });

  it("E4-T3 still returns workspace/current 200", async () => {
    const { server, base } = await listen();
    try {
      const current = await fetch(`${base}/api/muse/workspace/current`, {
        method: "POST",
        headers: { authorization: "Bearer jwt-ok", "content-type": "application/json" },
        body: JSON.stringify({ workspaceId: "ws-1" })
      });
      expect(current.status).toBe(200);
      const body = (await current.json()) as { code: number; data: { workspaceId: string } };
      expect(body.code).toBe(0);
      expect(body.data.workspaceId).toBe("ws-1");
    } finally {
      server.close();
    }
  });
});
