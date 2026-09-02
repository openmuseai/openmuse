import type { JsonArray, JsonObject, JsonValue } from "./types.js";

export interface JsonTraversalLimits {
  readonly maxDepth: number;
  readonly maxContainerChildren: number;
}

export class JsonBoundaryError extends TypeError {
  public constructor(
    message: string,
    public readonly path: string
  ) {
    super(`${path}: ${message}`);
    this.name = "JsonBoundaryError";
  }
}

const hasUnpairedSurrogate = (value: string): boolean => {
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return true;
      index += 1;
    } else if (code >= 0xdc00 && code <= 0xdfff) {
      return true;
    }
  }
  return false;
};

const childPath = (path: string, key: string | number): string =>
  typeof key === "number" ? `${path}[${key}]` : `${path}.${key}`;

export const snapshotJson = (
  value: unknown,
  limits: JsonTraversalLimits,
  path = "$",
  depth = 0,
  active = new WeakSet<object>()
): JsonValue => {
  if (depth > limits.maxDepth) throw new JsonBoundaryError(`maximum depth ${limits.maxDepth} exceeded`, path);
  if (value === null || typeof value === "boolean") return value;
  if (typeof value === "string") {
    if (hasUnpairedSurrogate(value)) throw new JsonBoundaryError("unpaired UTF-16 surrogate is not valid JCS input", path);
    return value;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) throw new JsonBoundaryError("number must be finite", path);
    if (Object.is(value, -0)) throw new JsonBoundaryError("negative zero is not lossless JSON", path);
    return value;
  }
  if (typeof value !== "object") throw new JsonBoundaryError(`unsupported ${typeof value} value`, path);

  if (active.has(value)) throw new JsonBoundaryError("cyclic reference", path);
  active.add(value);
  try {
    if (Array.isArray(value)) {
      if (value.length > limits.maxContainerChildren) {
        throw new JsonBoundaryError(`container has more than ${limits.maxContainerChildren} children`, path);
      }
      if (Object.getOwnPropertySymbols(value).length > 0) throw new JsonBoundaryError("symbol properties are forbidden", path);
      const ownNames = Object.getOwnPropertyNames(value);
      const expectedNames = Array.from({ length: value.length }, (_, index) => String(index));
      const dataNames = ownNames.filter((name) => name !== "length");
      if (dataNames.length !== expectedNames.length || dataNames.some((name, index) => name !== expectedNames[index])) {
        throw new JsonBoundaryError("array must be dense and have no decorated properties", path);
      }
      const output: JsonValue[] = [];
      for (let index = 0; index < value.length; index += 1) {
        const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
        if (descriptor === undefined || !("value" in descriptor) || !descriptor.enumerable) {
          throw new JsonBoundaryError("array element must be an enumerable data property", childPath(path, index));
        }
        output.push(snapshotJson(descriptor.value, limits, childPath(path, index), depth + 1, active));
      }
      return Object.freeze(output) as JsonArray;
    }

    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      throw new JsonBoundaryError("object must use Object or null prototype", path);
    }
    if (Object.getOwnPropertySymbols(value).length > 0) throw new JsonBoundaryError("symbol properties are forbidden", path);
    const keys = Object.getOwnPropertyNames(value);
    if (keys.length > limits.maxContainerChildren) {
      throw new JsonBoundaryError(`container has more than ${limits.maxContainerChildren} children`, path);
    }
    const output: Record<string, JsonValue> = Object.create(null);
    for (const key of keys) {
      if (hasUnpairedSurrogate(key)) throw new JsonBoundaryError("object key contains an unpaired surrogate", childPath(path, key));
      const descriptor = Object.getOwnPropertyDescriptor(value, key);
      if (descriptor === undefined || !("value" in descriptor) || !descriptor.enumerable) {
        throw new JsonBoundaryError("object member must be an enumerable data property", childPath(path, key));
      }
      output[key] = snapshotJson(descriptor.value, limits, childPath(path, key), depth + 1, active);
    }
    return Object.freeze(output) as JsonObject;
  } finally {
    active.delete(value);
  }
};
