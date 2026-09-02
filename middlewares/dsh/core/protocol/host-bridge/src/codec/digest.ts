import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex, concatBytes, utf8ToBytes } from "@noble/hashes/utils.js";
import type { GrantDigest, InputDigest, SchemaDigest } from "../contract/brands.js";
import { canonicalizeJson } from "./canonical.js";

const digest = <T extends string>(prefix: string, value: unknown): T => {
  const bytes = concatBytes(utf8ToBytes(prefix), canonicalizeJson(value));
  return `sha256:${bytesToHex(sha256(bytes))}` as T;
};

export const digestSchema = (value: unknown): SchemaDigest => digest<SchemaDigest>("muse-schema-v1\0", value);
export const digestInput = (value: unknown): InputDigest => digest<InputDigest>("muse-input-v1\0", value);
export const digestGrant = (value: unknown): GrantDigest => digest<GrantDigest>("muse-grant-v1\0", value);
