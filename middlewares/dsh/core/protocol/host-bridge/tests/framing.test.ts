import { describe, expect, it } from "vitest";
import {
  LengthDelimitedFrameDecoder,
  encodeLengthDelimitedFrame,
  unixDomainSocketEndpoint,
  windowsNamedPipeEndpoint
} from "../src/index.js";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

describe("desktop carrier framing", () => {
  it("decodes partial chunks and multiple frames without mixing boundaries", () => {
    const first = encodeLengthDelimitedFrame(encoder.encode("one"), 16);
    const second = encodeLengthDelimitedFrame(encoder.encode("two"), 16);
    const joined = new Uint8Array(first.length + second.length);
    joined.set(first); joined.set(second, first.length);
    const frames = new LengthDelimitedFrameDecoder(16);
    expect(frames.push(joined.slice(0, 2))).toEqual([]);
    expect(frames.push(joined.slice(2, 8)).map(value => decoder.decode(value))).toEqual(["one"]);
    expect(frames.push(joined.slice(8)).map(value => decoder.decode(value))).toEqual(["two"]);
    expect(() => frames.finish()).not.toThrow();
  });

  it("rejects zero, oversized and partial frames", () => {
    expect(() => encodeLengthDelimitedFrame(new Uint8Array(), 16)).toThrow();
    expect(() => encodeLengthDelimitedFrame(encoder.encode("large"), 4)).toThrow();
    const oversized = new Uint8Array([0, 0, 0, 17]);
    expect(() => new LengthDelimitedFrameDecoder(16).push(oversized)).toThrow();
    const partial = new LengthDelimitedFrameDecoder(16);
    partial.push(new Uint8Array([0, 0, 0]));
    expect(() => partial.finish()).toThrow();
  });

  it("freezes platform endpoint and peer-identity semantics", () => {
    expect(unixDomainSocketEndpoint("/tmp/muse.sock")).toMatchObject({
      kind: "unix_domain_socket", peerIdentitySource: "peer_credentials"
    });
    expect(windowsNamedPipeEndpoint("\\\\.\\pipe\\muse")).toMatchObject({
      kind: "windows_named_pipe", peerIdentitySource: "named_pipe_client_token"
    });
    expect(() => unixDomainSocketEndpoint("relative.sock")).toThrow();
  });
});
