import compatibilityCases from "../fixtures/v1/compatibility/messages.json" with { type: "json" };
import invalidCases from "../fixtures/v1/invalid/messages.json" with { type: "json" };
import validCases from "../fixtures/v1/valid/messages.json" with { type: "json" };
import { describe, expect, it } from "vitest";
import {
  assertResponseMatches,
  decodeMessage,
  encodeMessage,
  parseOpaqueId,
  ProtocolViolation,
  snapshotJson,
  type NegotiatedProtocol
} from "../src/index.js";
import { parseConformanceCases } from "../src/testing/index.js";

const negotiated: NegotiatedProtocol = {
  major: 1,
  minor: 0,
  features: new Set(["muse.events"]),
  hostSessionId: parseOpaqueId("host-session.1", "HostSessionId")
};

describe("wire conformance", () => {
  for (const fixture of parseConformanceCases(validCases)) {
    it(`accepts ${fixture.name}`, () => {
      const message = decodeMessage(fixture.message, fixture.negotiated ? negotiated : undefined);
      expect(message.kind).toBe((fixture.message as { readonly kind: string }).kind);
      const encoded = encodeMessage(message, fixture.negotiated ? negotiated : undefined);
      expect(decodeMessage(encoded, fixture.negotiated ? negotiated : undefined)).toEqual(message);
    });
  }

  for (const fixture of parseConformanceCases(invalidCases)) {
    it(`rejects ${fixture.name}`, () => {
      try {
        decodeMessage(fixture.message, fixture.negotiated ? negotiated : undefined);
        throw new Error("expected decode to fail");
      } catch (error) {
        expect(error).toBeInstanceOf(ProtocolViolation);
        expect((error as ProtocolViolation).bridgeError.code).toBe(fixture.category);
      }
    });
  }

  for (const fixture of parseConformanceCases(compatibilityCases)) {
    it(`forward reads ${fixture.name}`, () => {
      const message = decodeMessage(fixture.message, fixture.negotiated ? negotiated : undefined);
      expect(Object.hasOwn(message, "futureInformationalField")).toBe(false);
    });
  }
});

describe("correlation and lossless JSON", () => {
  it("rejects a response requestId mismatch", () => {
    const responseFixture = parseConformanceCases(validCases).find((entry) => entry.name === "invoke response");
    expect(responseFixture).toBeDefined();
    const response = decodeMessage(responseFixture!.message, negotiated);
    expect(() => assertResponseMatches(response, {
      requestId: parseOpaqueId("request.other", "RequestId"),
      hostSessionId: negotiated.hostSessionId
    })).toThrowError(ProtocolViolation);
  });

  it("rejects negative zero, cycles, sparse arrays and accessors", () => {
    expect(() => snapshotJson(-0, { maxDepth: 64, maxContainerChildren: 100 })).toThrow();
    const cyclic: Record<string, unknown> = {};
    cyclic.self = cyclic;
    expect(() => snapshotJson(cyclic, { maxDepth: 64, maxContainerChildren: 100 })).toThrow();
    const sparse = new Array(2);
    sparse[1] = true;
    expect(() => snapshotJson(sparse, { maxDepth: 64, maxContainerChildren: 100 })).toThrow();
    const accessor = Object.defineProperty({}, "secret", { enumerable: true, get: () => "value" });
    expect(() => snapshotJson(accessor, { maxDepth: 64, maxContainerChildren: 100 })).toThrow();
  });

  it("detaches mutable caller objects", () => {
    const source = { nested: { value: 1 } };
    const snapshot = snapshotJson(source, { maxDepth: 64, maxContainerChildren: 100 });
    source.nested.value = 2;
    expect(snapshot).toEqual({ nested: { value: 1 } });
    expect(Object.isFrozen(snapshot)).toBe(true);
  });
});
