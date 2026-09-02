import { createConnection, type Socket } from "node:net";
import { transportError } from "./types.js";
import type { DesktopCarrier, DesktopCarrierConnection, DesktopCarrierEndpoint } from "./desktop.js";

class NodeDesktopCarrierConnection implements DesktopCarrierConnection {
  readonly readable: AsyncIterable<Uint8Array>;
  #closed = false;

  constructor(readonly endpoint: DesktopCarrierEndpoint, private readonly socket: Socket) {
    this.readable = this.#read();
  }

  async write(frame: Uint8Array): Promise<void> {
    if (this.#closed || this.socket.destroyed) {
      throw transportError("UNAVAILABLE", "desktop carrier is closed");
    }
    await new Promise<void>((resolve, reject) => {
      this.socket.write(frame, error => {
        if (error === null || error === undefined) resolve();
        else reject(transportError("UNAVAILABLE", "desktop carrier write failed"));
      });
    });
  }

  async close(): Promise<void> {
    if (this.#closed) return;
    this.#closed = true;
    if (this.socket.destroyed) return;
    await new Promise<void>(resolve => {
      this.socket.once("close", () => resolve());
      this.socket.destroy();
    });
  }

  async *#read(): AsyncIterableIterator<Uint8Array> {
    try {
      for await (const chunk of this.socket) yield new Uint8Array(chunk);
    } catch {
      if (!this.#closed) throw transportError("UNAVAILABLE", "desktop carrier read failed");
    }
  }
}

/** Node implementation for UDS on POSIX and named pipes on Windows. */
export class NodeDesktopCarrier implements DesktopCarrier {
  async connect(endpoint: DesktopCarrierEndpoint, signal: AbortSignal): Promise<DesktopCarrierConnection> {
    if (signal.aborted) throw transportError("CANCELLED", "carrier connect cancelled");
    const socket = createConnection(endpoint.address);
    socket.setNoDelay(true);
    await new Promise<void>((resolve, reject) => {
      const cleanup = (): void => {
        socket.removeListener("connect", connected);
        socket.removeListener("error", failed);
        signal.removeEventListener("abort", aborted);
      };
      const connected = (): void => { cleanup(); resolve(); };
      const failed = (): void => {
        cleanup(); socket.destroy();
        reject(transportError("UNAVAILABLE", "desktop carrier connect failed"));
      };
      const aborted = (): void => {
        cleanup(); socket.destroy();
        reject(transportError("CANCELLED", "carrier connect cancelled"));
      };
      socket.once("connect", connected);
      socket.once("error", failed);
      signal.addEventListener("abort", aborted, { once: true });
    });
    return new NodeDesktopCarrierConnection(endpoint, socket);
  }
}
