import type { BridgeLimits, BridgeMessageV1, NegotiatedProtocol } from "../contract/types.js";
import { decodeMessage } from "./decode.js";
import { HARD_LIMITS } from "./limits.js";

const encoder = new TextEncoder();

export const encodeMessage = (
  message: BridgeMessageV1,
  negotiated?: NegotiatedProtocol,
  limits: BridgeLimits = HARD_LIMITS
): Uint8Array => {
  const detached = decodeMessage(message, negotiated, limits);
  const bytes = encoder.encode(JSON.stringify(detached));
  if (bytes.byteLength > limits.maxMessageBytes) throw new RangeError("encoded message exceeds negotiated byte limit");
  return bytes;
};
