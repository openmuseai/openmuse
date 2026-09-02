import type { BridgeLimits } from "../contract/types.js";

export const HARD_LIMITS: BridgeLimits = Object.freeze({
  maxMessageBytes: 2 * 1024 * 1024,
  maxInlineSchemaBytes: 64 * 1024,
  maxResolvedSchemaBytes: 1024 * 1024,
  maxDiscoverPageBytes: 512 * 1024,
  maxDiscoverDescriptors: 256,
  maxInputBytes: 256 * 1024,
  maxOutputBytes: 1024 * 1024,
  maxErrorDetailsBytes: 32 * 1024,
  maxEventPayloadBytes: 256 * 1024,
  maxJsonDepth: 64,
  maxContainerChildren: 10_000
});

const limitKeys = Object.keys(HARD_LIMITS) as (keyof BridgeLimits)[];

export const lowerLimits = (left: BridgeLimits, right: BridgeLimits): BridgeLimits => {
  const result = {} as Record<keyof BridgeLimits, number>;
  for (const key of limitKeys) result[key] = Math.min(left[key], right[key], HARD_LIMITS[key]);
  return Object.freeze(result) as unknown as BridgeLimits;
};

export const assertWithinHardLimits = (limits: BridgeLimits): void => {
  for (const key of limitKeys) {
    if (!Number.isSafeInteger(limits[key]) || limits[key] <= 0 || limits[key] > HARD_LIMITS[key]) {
      throw new RangeError(`${key} must be a positive integer no greater than ${HARD_LIMITS[key]}`);
    }
  }
};
