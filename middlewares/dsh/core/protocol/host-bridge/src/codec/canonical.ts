import canonicalize from "canonicalize";
import { snapshotJson, type JsonTraversalLimits } from "../contract/json.js";
import type { JsonValue } from "../contract/types.js";
import { HARD_LIMITS } from "./limits.js";

const encoder = new TextEncoder();

export const canonicalizeJson = (
  value: unknown,
  limits: JsonTraversalLimits = {
    maxDepth: HARD_LIMITS.maxJsonDepth,
    maxContainerChildren: HARD_LIMITS.maxContainerChildren
  }
): Uint8Array => {
  const snapshot = snapshotJson(value, limits);
  const serialized = canonicalize(snapshot);
  if (serialized === undefined) throw new TypeError("value cannot be represented as canonical JSON");
  return encoder.encode(serialized);
};

export const canonicalJsonText = (value: JsonValue, limits?: JsonTraversalLimits): string =>
  new TextDecoder().decode(canonicalizeJson(value, limits));

export const utf8ByteLength = (value: string): number => encoder.encode(value).byteLength;
