import { describe, expect, it } from "vitest";
import { HARD_LIMITS, negotiateVersion, ProtocolViolation } from "../src/index.js";

describe("protocol negotiation", () => {
  it("selects the highest common minor, feature intersection and lower limits", () => {
    const selected = negotiateVersion(
      { versions: [{ major: 1, minMinor: 0, maxMinor: 2 }], features: ["muse.events", "muse.extra"], limits: HARD_LIMITS },
      {
        versions: [{ major: 1, minMinor: 0, maxMinor: 1 }],
        features: ["muse.events"],
        limits: { ...HARD_LIMITS, maxInputBytes: 4096 }
      }
    );
    expect(selected).toMatchObject({ major: 1, minor: 1, features: ["muse.events"] });
    expect(selected.limits.maxInputBytes).toBe(4096);
  });

  it("rejects a missing version intersection", () => {
    expect(() => negotiateVersion(
      { versions: [{ major: 1, minMinor: 2, maxMinor: 3 }], features: [], limits: HARD_LIMITS },
      { versions: [{ major: 1, minMinor: 0, maxMinor: 1 }], features: [], limits: HARD_LIMITS }
    )).toThrowError(ProtocolViolation);
  });
});
