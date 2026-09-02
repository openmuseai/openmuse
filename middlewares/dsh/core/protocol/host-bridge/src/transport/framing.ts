import { transportError } from "./types.js";

const PREFIX_BYTES = 4;

export const encodeLengthDelimitedFrame = (
  payload: Uint8Array,
  maxFrameBytes: number
): Uint8Array => {
  if (
    payload.byteLength === 0 ||
    payload.byteLength > maxFrameBytes ||
    payload.byteLength > 0xffff_ffff
  ) {
    throw transportError("INVALID_ENVELOPE", "transport frame exceeds limit");
  }
  const frame = new Uint8Array(PREFIX_BYTES + payload.byteLength);
  new DataView(frame.buffer).setUint32(0, payload.byteLength, false);
  frame.set(payload, PREFIX_BYTES);
  return frame;
};

/** Incremental four-byte big-endian length-delimited frame decoder shared by desktop carriers. */
export class LengthDelimitedFrameDecoder {
  #buffer = new Uint8Array(0);

  constructor(private readonly maxFrameBytes: number) {
    if (!Number.isSafeInteger(maxFrameBytes) || maxFrameBytes <= 0) {
      throw new TypeError("maxFrameBytes must be a positive safe integer");
    }
  }

  push(chunk: Uint8Array): readonly Uint8Array[] {
    if (chunk.byteLength === 0) return [];
    const joined = new Uint8Array(this.#buffer.byteLength + chunk.byteLength);
    joined.set(this.#buffer);
    joined.set(chunk, this.#buffer.byteLength);
    this.#buffer = joined;

    const frames: Uint8Array[] = [];
    while (this.#buffer.byteLength >= PREFIX_BYTES) {
      const length = new DataView(
        this.#buffer.buffer,
        this.#buffer.byteOffset,
        PREFIX_BYTES
      ).getUint32(0, false);
      if (length === 0 || length > this.maxFrameBytes) {
        this.#buffer = new Uint8Array(0);
        throw transportError("INVALID_ENVELOPE", "transport frame prefix is invalid");
      }
      if (this.#buffer.byteLength < PREFIX_BYTES + length) break;
      frames.push(this.#buffer.slice(PREFIX_BYTES, PREFIX_BYTES + length));
      this.#buffer = this.#buffer.slice(PREFIX_BYTES + length);
    }
    return frames;
  }

  finish(): void {
    if (this.#buffer.byteLength !== 0) {
      this.#buffer = new Uint8Array(0);
      throw transportError("INVALID_ENVELOPE", "transport ended with a partial frame");
    }
  }
}
