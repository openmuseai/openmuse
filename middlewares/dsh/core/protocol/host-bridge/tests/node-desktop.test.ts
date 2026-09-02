import { randomUUID } from "node:crypto";
import { createServer } from "node:net";
import { describe, expect, it } from "vitest";
import { NodeDesktopCarrier, unixDomainSocketEndpoint } from "../src/index.js";

describe("Node desktop carrier", () => {
  it.skipIf(process.platform === "win32")(
    "connects to a UDS and preserves socket backpressure/read lifecycle",
    async () => {
      const path = `/tmp/muse-${randomUUID()}.sock`;
      const server = createServer(socket => {
        socket.once("data", data => socket.write(data));
      });
      await new Promise<void>((resolve, reject) => {
        server.once("error", reject);
        server.listen(path, resolve);
      });
      try {
        const connection = await new NodeDesktopCarrier().connect(
          unixDomainSocketEndpoint(path),
          new AbortController().signal
        );
        const iterator = connection.readable[Symbol.asyncIterator]();
        await connection.write(new TextEncoder().encode("ping"));
        const response = await iterator.next();
        expect(new TextDecoder().decode(response.value)).toBe("ping");
        await connection.close();
      } finally {
        await new Promise<void>(resolve => server.close(() => resolve()));
      }
    }
  );

  it("honors an already-aborted connection attempt", async () => {
    const controller = new AbortController();
    controller.abort();
    await expect(
      new NodeDesktopCarrier().connect(
        unixDomainSocketEndpoint("/tmp/unused-muse.sock"),
        controller.signal
      )
    ).rejects.toMatchObject({ code: "CANCELLED" });
  });
});
