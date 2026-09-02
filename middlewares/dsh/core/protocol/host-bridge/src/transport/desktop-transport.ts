import type { WireEnvelope } from "../contract/types.js";
import { LengthDelimitedFrameDecoder, encodeLengthDelimitedFrame } from "./framing.js";
import { NodeDesktopCarrier } from "./node-desktop.js";
import { unixDomainSocketEndpoint, windowsNamedPipeEndpoint, type DesktopCarrier, type DesktopCarrierEndpoint } from "./desktop.js";
import {
  isTransportError,
  transportError,
  type MuseHostTransport,
  type RuntimeProof,
  type TransportConnection,
  type TransportLaunch,
  type TransportRequest
} from "./types.js";

type ControlRequest =
  | { type: "connect"; proof: RuntimeProof }
  | { type: "disconnect"; connection: TransportConnection }
  | { type: "unary"; connection: TransportConnection; deadlineAt: number; message: WireEnvelope }
  | { type: "stream"; connection: TransportConnection; deadlineAt: number; message: WireEnvelope };

type ControlResponse =
  | { ok: true; connection?: TransportConnection; message?: WireEnvelope }
  | { ok: false; error: { code: string; message: string } };

const codec = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });
const normalizeTransportCode = (code: string): ReturnType<typeof transportError>["code"] =>
  ["UNAUTHENTICATED", "FORBIDDEN", "HOST_GENERATION_STALE", "DEADLINE_EXCEEDED", "CANCELLED", "RATE_LIMITED", "UNAVAILABLE", "INVALID_ENVELOPE", "INTERNAL"].includes(code)
    ? code as ReturnType<typeof transportError>["code"]
    : "INTERNAL";

const endpointOf = (launch: TransportLaunch): DesktopCarrierEndpoint =>
  process.platform === "win32"
    ? windowsNamedPipeEndpoint(launch.endpoint)
    : unixDomainSocketEndpoint(launch.endpoint);

/**
 * Production desktop Bridge transport. Each bounded request uses one authenticated local carrier
 * exchange, while the opaque connection credential preserves Host-owned session limits.
 */
export class DesktopMuseHostTransport implements MuseHostTransport {
  readonly #endpoint: DesktopCarrierEndpoint;
  readonly #maxFrameBytes: number;
  #closed = false;
  readonly #lifecycle = new AbortController();

  constructor(
    readonly launch: TransportLaunch,
    private readonly carrier: DesktopCarrier = new NodeDesktopCarrier(),
    maxFrameBytes = 1024 * 1024
  ) {
    this.#endpoint = endpointOf(launch);
    if (!Number.isSafeInteger(maxFrameBytes) || maxFrameBytes <= 0) {
      throw new TypeError("maxFrameBytes must be a positive safe integer");
    }
    this.#maxFrameBytes = maxFrameBytes;
  }

  async connect(proof: RuntimeProof): Promise<TransportConnection> {
    const response = await this.#exchange({ type: "connect", proof });
    if (response.connection === undefined) throw transportError("INTERNAL", "Host omitted connection credential");
    if (response.connection.hostGeneration !== this.launch.hostGeneration) {
      throw transportError("HOST_GENERATION_STALE", "Host generation changed during connect");
    }
    return Object.freeze(response.connection);
  }

  async disconnect(connection: TransportConnection): Promise<void> {
    if (this.#closed) return;
    await this.#exchange({ type: "disconnect", connection }).catch(() => undefined);
  }

  async unary(request: TransportRequest): Promise<WireEnvelope> {
    if (request.signal?.aborted === true) throw transportError("CANCELLED", "desktop request cancelled");
    const response = await this.#exchange({
      type: "unary",
      connection: request.connection,
      deadlineAt: request.deadlineAt,
      message: request.message
    }, request.signal);
    if (response.message === undefined) throw transportError("INTERNAL", "Host omitted unary response");
    return response.message;
  }

  async *stream(request: TransportRequest): AsyncIterableIterator<WireEnvelope> {
    const callerCancelled = (): boolean => request.signal?.aborted === true;
    if (this.#closed) throw transportError("UNAVAILABLE", "desktop transport is closed");
    if (callerCancelled()) throw transportError("CANCELLED", "desktop stream cancelled");
    const controller = new AbortController();
    const abort = (): void => controller.abort();
    request.signal?.addEventListener("abort", abort, { once: true });
    this.#lifecycle.signal.addEventListener("abort", abort, { once: true });
    let connection: Awaited<ReturnType<DesktopCarrier["connect"]>> | undefined;
    const frames = new LengthDelimitedFrameDecoder(this.#maxFrameBytes);
    try {
      connection = await this.carrier.connect(this.#endpoint, controller.signal);
      const payload = codec.encode(JSON.stringify({
        type: "stream",
        connection: request.connection,
        deadlineAt: request.deadlineAt,
        message: request.message
      } satisfies ControlRequest));
      await connection.write(encodeLengthDelimitedFrame(payload, this.#maxFrameBytes));
      for await (const chunk of connection.readable) {
        for (const frame of frames.push(chunk)) {
          let response: ControlResponse;
          try {
            response = JSON.parse(decoder.decode(frame)) as ControlResponse;
          } catch {
            throw transportError("INVALID_ENVELOPE", "desktop Host returned invalid stream JSON");
          }
          if (!response.ok) throw transportError(normalizeTransportCode(response.error.code), response.error.message);
          if (response.message === undefined) {
            throw transportError("INTERNAL", "desktop Host omitted stream message");
          }
          yield response.message;
        }
      }
      frames.finish();
      if (!controller.signal.aborted && Date.now() < request.deadlineAt) {
        throw transportError("UNAVAILABLE", "desktop Host stream ended unexpectedly");
      }
    } catch (error) {
      if (controller.signal.aborted) {
        throw transportError(callerCancelled() ? "CANCELLED" : "UNAVAILABLE", "desktop stream cancelled");
      }
      if (isTransportError(error)) throw error;
      throw transportError("UNAVAILABLE", "desktop stream exchange failed");
    } finally {
      request.signal?.removeEventListener("abort", abort);
      this.#lifecycle.signal.removeEventListener("abort", abort);
      await connection?.close().catch(() => undefined);
    }
  }

  async close(): Promise<void> {
    this.#closed = true;
    this.#lifecycle.abort();
  }

  async #exchange(request: ControlRequest, callerSignal?: AbortSignal): Promise<Extract<ControlResponse, { ok: true }>> {
    if (this.#closed) throw transportError("UNAVAILABLE", "desktop transport is closed");
    const controller = new AbortController();
    const abort = (): void => controller.abort(callerSignal?.reason);
    callerSignal?.addEventListener("abort", abort, { once: true });
    let connection: Awaited<ReturnType<DesktopCarrier["connect"]>> | undefined;
    try {
      connection = await this.carrier.connect(this.#endpoint, controller.signal);
      const payload = codec.encode(JSON.stringify(request));
      await connection.write(encodeLengthDelimitedFrame(payload, this.#maxFrameBytes));
      const frames = new LengthDelimitedFrameDecoder(this.#maxFrameBytes);
      for await (const chunk of connection.readable) {
        for (const frame of frames.push(chunk)) {
          const response = JSON.parse(decoder.decode(frame)) as ControlResponse;
          if (!response.ok) {
            const code = response.error.code;
            throw transportError(
              normalizeTransportCode(code),
              response.error.message
            );
          }
          return response;
        }
      }
      throw transportError("UNAVAILABLE", "desktop Host closed without a response");
    } catch (error) {
      if (callerSignal?.aborted === true) throw transportError("CANCELLED", "desktop request cancelled");
      if (isTransportError(error)) throw error;
      throw transportError("UNAVAILABLE", "desktop Host exchange failed");
    } finally {
      callerSignal?.removeEventListener("abort", abort);
      await connection?.close().catch(() => undefined);
    }
  }
}
