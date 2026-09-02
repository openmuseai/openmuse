import { snapshotJson } from "../contract/json.js";
import type { JsonObject, JsonValue } from "../contract/types.js";
import { HARD_LIMITS } from "../codec/limits.js";

export interface ConformanceCase {
  readonly name: string;
  readonly valid: boolean;
  readonly category?: string;
  readonly negotiated: boolean;
  readonly message: JsonValue;
}

const isObject = (value: JsonValue): value is JsonObject =>
  typeof value === "object" && value !== null && !Array.isArray(value);

export const parseConformanceCases = (raw: unknown): readonly ConformanceCase[] => {
  const value = snapshotJson(raw, {
    maxDepth: HARD_LIMITS.maxJsonDepth,
    maxContainerChildren: HARD_LIMITS.maxContainerChildren
  });
  if (!Array.isArray(value)) throw new TypeError("conformance fixture root must be an array");
  return Object.freeze(value.map((entry, index) => {
    if (!isObject(entry) || typeof entry.name !== "string" || typeof entry.valid !== "boolean" || !("message" in entry)) {
      throw new TypeError(`conformance fixture ${index} is malformed`);
    }
    return Object.freeze({
      name: entry.name,
      valid: entry.valid,
      negotiated: entry.negotiated === true,
      message: entry.message,
      ...(typeof entry.category === "string" ? { category: entry.category } : {})
    });
  }));
};
