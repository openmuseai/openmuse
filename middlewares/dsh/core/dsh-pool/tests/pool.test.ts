import { mkdtemp } from "node:fs/promises";
import { createServer, request as httpRequest } from "node:http";
import { connect } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { FakeExecutor, isJwtAuthorization } from "../src/executor.js";
import { LocalProcessExecutor } from "../src/local-executor.js";
import { InstancePool } from "../src/pool.js";
import { createControlServer } from "../src/server.js";
import { createPoolProxy, rewriteApiTrustHeaders, stripUpstreamAuth } from "../src/proxy.js";
import { systemdRunArgv, SystemdRunExecutor } from "../src/systemd-executor.js";
import { readCgroupMetrics } from "../src/metrics.js";
import { extractLaunchToken, tenantHashOf, tenantKeyOf, uidForTenant, webUrlOf } from "../src/tenant.js";
import type { PoolOptions } from "../src/types.js";

const options = async (overrides: Partial<PoolOptions> = {}): Promise<PoolOptions> => ({
  readyQuota: 2,
  activeQuota: 2,
  idleTtlMs: 1_000,
  portBase: 18000,
  homeRoot: await mkdtemp(join(tmpdir(), "muse-pool-")),
  publicBase: "https://example.com",
  tenantSalt: "salt",
  nodeId: "local",
  memoryMaxBytes: 512 * 1024 * 1024,
  cpuWeight: 100,
  ...overrides
});

describe("api trust header rewrite", () => {
  it("rewrites /api Host and Origin to the tenant loopback authority", () => {
    const rewritten = rewriteApiTrustHeaders(
      { host: "openmuseai.com", origin: "https://openmuseai.com", cookie: "x" },
      13082,
      "/api/settings.describe"
    );
    expect(rewritten.host).toBe("127.0.0.1:13082");
    expect(rewritten.origin).toBe("http://127.0.0.1:13082");
    expect(rewritten.cookie).toBe("x");
  });

  it("leaves HTML and other non-api paths on the public Host", () => {
    const headers = { host: "openmuseai.com", origin: "https://openmuseai.com" };
    expect(rewriteApiTrustHeaders(headers, 13082, "/")).toEqual(headers);
    expect(rewriteApiTrustHeaders(headers, 13082, "/assets/index.js")).toEqual(headers);
  });
});

describe("tenant addressing", () => {
  it("hashes account+workspace with salt and exposes a 128-bit public id", () => {
    const a = tenantKeyOf("user-1", "ws-1", "salt");
    const b = tenantKeyOf("user-1", "ws-2", "salt");
    expect(a).not.toBe(b);
    expect(tenantHashOf(a)).toHaveLength(32);
    expect(uidForTenant(a)).toBeGreaterThanOrEqual(16000);
    expect(extractLaunchToken("dsh web: http://127.0.0.1:1/?token=abcDEF123")).toBe("abcDEF123");
    expect(webUrlOf("https://example.com/", "ab".repeat(16), "tok")).toContain("/u/");
    expect(webUrlOf("https://example.com/", "ab".repeat(16), "tok")).toContain("token=tok");
  });
});

describe("instance pool", () => {
  it("opens the same tenant idempotently and isolates two tenants", async () => {
    const executor = new FakeExecutor();
    const pool = new InstancePool(executor, await options());
    const first = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    const again = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d2" });
    const other = await pool.open({ accountRef: "b", workspaceRef: "w1", deviceId: "d1" });
    expect(first.instanceRef).toBe(again.instanceRef);
    expect(first.webUrl).toBe(again.webUrl);
    expect(other.instanceRef).not.toBe(first.instanceRef);
    expect(executor.started).toHaveLength(2);
    expect(first.nodeId).toBe("local");
  });

  it("queues when ACTIVE quota is exhausted and resumes after close+tick", async () => {
    const executor = new FakeExecutor();
    const pool = new InstancePool(executor, await options({ readyQuota: 4, activeQuota: 1 }));
    const hot = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    const queued = await pool.open({ accountRef: "b", workspaceRef: "w2", deviceId: "d2" });
    expect(hot.instanceRef).toBeDefined();
    expect(queued.queuePosition).toBe(1);
    expect(queued.webUrl).toBeUndefined();
    const again = await pool.open({ accountRef: "b", workspaceRef: "w2", deviceId: "d2" });
    expect(again.queuePosition).toBe(1);
    expect(pool.queueLength()).toBe(1);
    await pool.close(hot.sessionRef, "d1");
    await pool.tick();
    expect(pool.queueLength()).toBe(0);
  });

  it("suspends idle instances after TTL and resumes on the same home", async () => {
    let now = 1_000;
    const executor = new FakeExecutor();
    const pool = new InstancePool(executor, await options({ idleTtlMs: 50, now: () => now }));
    const open = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    await pool.close(open.sessionRef, "d1");
    now = 2_000;
    await pool.tick();
    const row = pool.snapshot()[0];
    expect(row?.state).toBe("suspended");
    expect(executor.stopped).toHaveLength(1);
    const resumed = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    expect(resumed.instanceRef).toBeDefined();
    expect(executor.started).toHaveLength(2);
    expect(executor.started[1]?.homeDir).toBe(executor.started[0]?.homeDir);
  });

  it("evicts LRU idle tenant when READY quota is full", async () => {
    const executor = new FakeExecutor();
    const pool = new InstancePool(executor, await options({ readyQuota: 1, activeQuota: 4 }));
    const first = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    await pool.close(first.sessionRef, "d1");
    const second = await pool.open({ accountRef: "b", workspaceRef: "w2", deviceId: "d2" });
    expect(second.instanceRef).toBeDefined();
    expect(pool.snapshot().filter(row => row.state === "suspended")).toHaveLength(1);
  });

  it("reaps a dead occupant so the next tenant can start", async () => {
    const executor = new FakeExecutor();
    const pool = new InstancePool(executor, await options({ readyQuota: 1, activeQuota: 1 }));
    await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    executor.alive = false;
    await pool.tick();
    const next = await pool.open({ accountRef: "b", workspaceRef: "w2", deviceId: "d2" });
    expect(next.webUrl).toBeDefined();
    expect(next.queuePosition).toBeUndefined();
    expect(executor.started).toHaveLength(2);
  });

  it("queues a different tenant when quota is full, even if that tenant has an orphan unit", async () => {
    const executor = new FakeExecutor();
    const tenantKey = tenantKeyOf("a", "w1", "salt");
    const hash = tenantHashOf(tenantKey);
    executor.orphans.set(hash, { port: 13081, unitName: `muse-dsh-${hash}` });
    const pool = new InstancePool(executor, await options({ readyQuota: 1, activeQuota: 1 }));
    const occupied = await pool.open({ accountRef: "b", workspaceRef: "w2", deviceId: "other" });
    expect(occupied.webUrl).toBeDefined();
    const queued = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    expect(queued.queuePosition).toBe(1);
    expect(queued.webUrl).toBeUndefined();
    expect(executor.started).toHaveLength(1);
  });

  it("adopts the live unit for this tenant without starting a second process", async () => {
    const executor = new FakeExecutor();
    const tenantKey = tenantKeyOf("a", "w1", "salt");
    const hash = tenantHashOf(tenantKey);
    executor.orphans.set(hash, { port: 13081, unitName: `muse-dsh-${hash}` });
    const pool = new InstancePool(executor, await options({ readyQuota: 1, activeQuota: 1 }));
    const opened = await pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" });
    expect(opened.webUrl).toContain(hash);
    expect(opened.queuePosition).toBeUndefined();
    expect(executor.started).toHaveLength(0);
    expect(pool.status().instances).toEqual([
      expect.objectContaining({ tenantHash: hash, state: "ready", port: 13081 })
    ]);
  });

  it("only starts one process for concurrent opens of the same tenant", async () => {
    class SlowStart extends FakeExecutor {
      override async start(spec: Parameters<FakeExecutor["start"]>[0]) {
        await new Promise(resolve => setTimeout(resolve, 40));
        return super.start(spec);
      }
    }
    const executor = new SlowStart();
    const pool = new InstancePool(executor, await options({ readyQuota: 1, activeQuota: 1 }));
    const [first, second] = await Promise.all([
      pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d1" }),
      pool.open({ accountRef: "a", workspaceRef: "w1", deviceId: "d2" })
    ]);
    expect(new URL(first.webUrl ?? "").pathname).toBe(new URL(second.webUrl ?? "").pathname);
    expect(first.webUrl).toBeDefined();
    expect(second.webUrl).toBeDefined();
    expect(executor.started).toHaveLength(1);
  });
});

describe("control HTTP + proxy", () => {
  const servers: Array<ReturnType<typeof createServer>> = [];
  afterEach(() => {
    for (const server of servers) server.close();
    servers.length = 0;
  });

  it("session/open is loopback JSON and proxy strips Cloud JWT cookies", async () => {
    const pool = new InstancePool(new FakeExecutor(), await options());
    const control = createControlServer(pool);
    servers.push(control);
    await new Promise<void>(resolve => control.listen(0, "127.0.0.1", resolve));
    const addr = control.address();
    if (addr === null || typeof addr === "string") throw new Error("no port");
    const opened = await (await fetch(`http://127.0.0.1:${addr.port}/internal/session/open`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ accountRef: "a", workspaceRef: "w", deviceId: "d" })
    })).json() as { webUrl?: string };
    expect(opened.webUrl).toContain("/u/");

    const backend = createServer((req, res) => {
      expect(req.headers.cookie).toBeUndefined();
      expect(req.headers.authorization).toBeUndefined();
      res.writeHead(200, { "content-type": "text/plain" });
      res.end("tenant-ok");
    });
    servers.push(backend);
    await new Promise<void>(resolve => backend.listen(0, "127.0.0.1", resolve));
    const backendAddr = backend.address();
    if (backendAddr === null || typeof backendAddr === "string") throw new Error("no backend");
    const hash = new URL(opened.webUrl ?? "").pathname.split("/")[2] ?? "";
    const hijack = new InstancePool(new FakeExecutor(), await options());
    (hijack as unknown as { byKey: Map<string, { tenantHash: string; state: string; port: number }> }).byKey = new Map([
      ["k", { tenantHash: hash, state: "ready", port: backendAddr.port }]
    ]);
    const proxy = createPoolProxy(hijack);
    servers.push(proxy);
    await new Promise<void>(resolve => proxy.listen(0, "127.0.0.1", resolve));
    const proxyAddr = proxy.address();
    if (proxyAddr === null || typeof proxyAddr === "string") throw new Error("no proxy");
    const body = await (await fetch(`http://127.0.0.1:${proxyAddr.port}/u/${hash}/hello`, {
      headers: { cookie: "access_token=jwt", authorization: "Bearer aaa.bbb.ccc" }
    })).text();
    expect(body).toBe("tenant-ok");
    expect(isJwtAuthorization("Bearer aaa.bbb.ccc")).toBe(true);
    expect(isJwtAuthorization("Bearer aaa.bbb")).toBe(false);
    expect(stripUpstreamAuth({ cookie: "x", authorization: "Bearer a.b.c" }).cookie).toBeUndefined();
  });

  it("rewrites /api Host+Origin to tenant loopback and splices WebSocket upgrades", async () => {
    const hash = "ab".repeat(16);
    const seen: { host?: string; origin?: string; url?: string; cookie?: string; upgrade?: boolean } = {};
    const backend = createServer((req, res) => {
      seen.host = req.headers.host;
      seen.origin = req.headers.origin;
      seen.url = req.url;
      seen.cookie = req.headers.cookie;
      res.writeHead(200, { "content-type": "text/plain" });
      res.end("ok");
    });
    backend.on("upgrade", (req, socket) => {
      seen.upgrade = true;
      seen.host = req.headers.host;
      seen.origin = req.headers.origin;
      seen.url = req.url;
      seen.cookie = req.headers.cookie;
      socket.write("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n");
      socket.end();
    });
    servers.push(backend);
    await new Promise<void>(resolve => backend.listen(0, "127.0.0.1", resolve));
    const backendAddr = backend.address();
    if (backendAddr === null || typeof backendAddr === "string") throw new Error("no backend");
    const hijack = new InstancePool(new FakeExecutor(), await options());
    (hijack as unknown as { byKey: Map<string, { tenantHash: string; state: string; port: number }> }).byKey = new Map([
      ["k", { tenantHash: hash, state: "ready", port: backendAddr.port }]
    ]);
    const proxy = createPoolProxy(hijack);
    servers.push(proxy);
    await new Promise<void>(resolve => proxy.listen(0, "127.0.0.1", resolve));
    const proxyAddr = proxy.address();
    if (proxyAddr === null || typeof proxyAddr === "string") throw new Error("no proxy");

    await new Promise<void>((resolve, reject) => {
      const req = httpRequest({
        host: "127.0.0.1",
        port: proxyAddr.port,
        path: `/u/${hash}/api/host.describe`,
        method: "POST",
        headers: {
          host: "openmuseai.com",
          origin: "https://openmuseai.com",
          cookie: "access_token=jwt"
        }
      }, res => {
        res.resume();
        res.on("end", resolve);
      });
      req.on("error", reject);
      req.end();
    });
    expect(seen.host).toBe(`127.0.0.1:${backendAddr.port}`);
    expect(seen.origin).toBe(`http://127.0.0.1:${backendAddr.port}`);
    expect(seen.url).toBe("/api/host.describe");
    expect(seen.cookie).toBeUndefined();

    await new Promise<void>((resolve, reject) => {
      const sock = connect(proxyAddr.port, "127.0.0.1", () => {
        sock.write(
          `GET /u/${hash}/api/events.mux HTTP/1.1\r\n` +
          "Host: openmuseai.com\r\n" +
          "Upgrade: websocket\r\n" +
          "Connection: Upgrade\r\n" +
          "Origin: https://openmuseai.com\r\n" +
          "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n" +
          "Sec-WebSocket-Version: 13\r\n" +
          "Cookie: access_token=jwt\r\n\r\n"
        );
      });
      sock.on("data", buf => {
        expect(buf.toString()).toContain("101");
        expect(seen.upgrade).toBe(true);
        expect(seen.host).toBe(`127.0.0.1:${backendAddr.port}`);
        expect(seen.origin).toBe(`http://127.0.0.1:${backendAddr.port}`);
        expect(seen.url).toBe("/api/events.mux");
        expect(seen.cookie).toBeUndefined();
        sock.destroy();
        resolve();
      });
      sock.on("error", reject);
    });
  });
});

describe("executors", () => {
  it("builds systemd-run argv with MemoryMax, uid, and host-auth env", () => {
    const argv = systemdRunArgv({
      tenantKey: "ab".repeat(32),
      tenantHash: "ab".repeat(16),
      homeDir: "/srv/muse-dsh/ab/home",
      port: 13081,
      uid: 16001,
      memoryMaxBytes: 512 * 1024 * 1024,
      cpuWeight: 100,
      env: { MUSE_DOCUMENT_CLOUD_URL: "https://cloud" }
    }, "/opt/muse-dsh/runtime/node", ["--import", "tsx/esm", "apps/cli/src/bin.ts"]);
    expect(argv[0]).toBe("systemd-run");
    expect(argv).toContain("--no-block");
    expect(argv).toContain("--quiet");
    expect(argv).toContain("--property=StandardOutput=journal");
    expect(argv.some(value => value.startsWith("--uid="))).toBe(true);
    expect(argv).toContain("--property=MemoryMax=512M");
    expect(argv).toContain("--property=TasksMax=512");
    expect(argv).toContain("--property=Restart=no");
    expect(argv).toContain("--setenv=MUSE_REQUIRE_HOST_AUTH=1");
    expect(argv.at(-1)).toBe("apps/cli/src/bin.ts");
  });

  it("adds working-directory and EnvironmentFile when provided", () => {
    const argv = systemdRunArgv({
      tenantKey: "ab".repeat(32),
      tenantHash: "ab".repeat(16),
      homeDir: "/srv/muse-dsh/ab/home",
      port: 13081,
      memoryMaxBytes: 512 * 1024 * 1024,
      cpuWeight: 100,
      env: {},
      cwd: "/opt/muse-dsh/runtime/dsh",
      envFile: "/opt/muse-dsh/.env"
    }, "/opt/muse-dsh/runtime/start-instance.sh", []);
    expect(argv).toContain("--working-directory=/opt/muse-dsh/runtime/dsh");
    expect(argv).toContain("--property=EnvironmentFile=/opt/muse-dsh/.env");
    expect(argv.some(value => value.startsWith("--uid="))).toBe(false);
  });

  it("SystemdRunExecutor uses injected runner and stop is systemctl stop", async () => {
    const calls: string[][] = [];
    const executor = new SystemdRunExecutor("/opt/muse-dsh/runtime/node", ["bin"], async argv => {
      calls.push([...argv]);
      return { stdout: "running token=launchSY12", stderr: "" };
    });
    const handle = await executor.start({
      tenantKey: "ab".repeat(32),
      tenantHash: "cd".repeat(16),
      homeDir: "/tmp/x",
      port: 13081,
      uid: 16001,
      memoryMaxBytes: 512 * 1024 * 1024,
      cpuWeight: 100,
      env: {}
    });
    expect(handle.unitName).toBe("muse-dsh-" + "cd".repeat(16));
    expect(handle.launchToken).toBe("launchSY12");
    expect(calls[0]?.slice(0, 2)).toEqual(["systemctl", "stop"]);
    expect(calls[1]?.slice(0, 2)).toEqual(["systemctl", "reset-failed"]);
    expect(calls[2]?.[0]).toBe("systemd-run");
    await executor.stop(handle);
    expect(calls[3]?.slice(0, 2)).toEqual(["systemctl", "stop"]);
    const missing = await readCgroupMetrics({
      instanceRef: "inst.x",
      executor: "systemd",
      port: 1,
      startedAt: 0,
      unitName: "muse-dsh-missing"
    });
    expect(missing.alive).toBe(false);
  });

  it("LocalProcessExecutor starts the fixture harness and extracts a launch token", async () => {
    const executor = new LocalProcessExecutor();
    const home = await mkdtemp(join(tmpdir(), "muse-local-exec-"));
    const fixture = new URL("./fixtures/fake-harness.mjs", import.meta.url);
    const probe = createServer();
    await new Promise<void>(resolve => probe.listen(0, "127.0.0.1", resolve));
    const probeAddr = probe.address();
    const port = probeAddr !== null && typeof probeAddr === "object" ? probeAddr.port : 0;
    await new Promise<void>(resolve => probe.close(() => resolve()));
    const handle = await executor.start({
      tenantKey: "k",
      tenantHash: "c".repeat(32),
      homeDir: home,
      port,
      memoryMaxBytes: 64 * 1024 * 1024,
      cpuWeight: 100,
      env: {},
      command: [process.execPath, fixture.pathname]
    });
    expect(handle.launchToken).toBe("launch-fixture");
    await executor.stop(handle);
  });
});
