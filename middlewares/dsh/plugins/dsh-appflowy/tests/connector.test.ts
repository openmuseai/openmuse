import { Context } from "@deepseek-ai/cordis";
import { describe, expect, it } from "vitest";
import AppFlowyConnector, { InProcessAppFlowyConnector } from "../src/connector.js";

describe("AppFlowy DSH composition connector", () => {
  it("publishes a transport whose generation matches its launch proof", async () => {
    const connector = new InProcessAppFlowyConnector(new Context() as never);
    const opened = await connector.open();
    const connection = await opened.transport.connect(opened.proof);
    expect(connection.hostGeneration).toBe("appflowy.e2e.1");
    await opened.transport.close();
  });

  it("fails closed when AppFlowy has not published a native launch descriptor", async () => {
    const previousLaunch = process.env.MUSE_APPFLOWY_LAUNCH_FILE;
    const previousCloud = process.env.MUSE_DOCUMENT_CLOUD_URL;
    process.env.MUSE_APPFLOWY_LAUNCH_FILE = `/tmp/muse-missing-${process.pid}.json`;
    delete process.env.MUSE_DOCUMENT_CLOUD_URL;
    try {
      await expect(new AppFlowyConnector(new Context() as never).open()).rejects.toBeDefined();
    } finally {
      if (previousLaunch === undefined) delete process.env.MUSE_APPFLOWY_LAUNCH_FILE;
      else process.env.MUSE_APPFLOWY_LAUNCH_FILE = previousLaunch;
      if (previousCloud === undefined) delete process.env.MUSE_DOCUMENT_CLOUD_URL;
      else process.env.MUSE_DOCUMENT_CLOUD_URL = previousCloud;
    }
  });

  it("opens an in-process cloud adapter when MUSE_DOCUMENT_CLOUD_URL is set", async () => {
    const previous = process.env.MUSE_DOCUMENT_CLOUD_URL;
    process.env.MUSE_DOCUMENT_CLOUD_URL = "https://app.example";
    try {
      const opened = await new AppFlowyConnector(new Context() as never).open();
      expect(opened.proof.runtimeInstanceId).toBe("runtime.dsh-appflowy-cloud");
      await opened.transport.close();
    } finally {
      if (previous === undefined) delete process.env.MUSE_DOCUMENT_CLOUD_URL;
      else process.env.MUSE_DOCUMENT_CLOUD_URL = previous;
    }
  });

  it("discovers the Cloud workspace family alongside document markdown", async () => {
    const connector = new InProcessAppFlowyConnector(new Context() as never);
    const opened = await connector.open();
    const connection = await opened.transport.connect(opened.proof);
    const reply = await opened.transport.unary({
      connection,
      deadlineAt: Date.now() + 5_000,
      message: {
        protocol: "muse-bridge",
        major: 1,
        minor: 0,
        kind: "discover.request",
        requestId: "request.discover" as never,
        sentAt: Date.now(),
        payload: { pageSize: 10 }
      }
    });
    expect(reply.kind).toBe("discover.response");
    const payload = reply.payload as {
      ok?: boolean;
      value?: { descriptors?: { descriptorId?: string; familyId?: string }[] };
    };
    const descriptors = payload.value?.descriptors ?? [];
    expect(descriptors.map(item => item.descriptorId)).toEqual(
      expect.arrayContaining(["appflowy.document.local", "appflowy.workspace.cloud"])
    );
    expect(descriptors.map(item => item.familyId)).toEqual(
      expect.arrayContaining(["muse.workspace"])
    );
    await opened.transport.close();
  });
});
