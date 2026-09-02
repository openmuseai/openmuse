import type { BridgeError, BridgeErrorCode, JsonValue } from "../contract/types.js";

export class ProtocolViolation extends Error {
  public readonly bridgeError: BridgeError;

  public constructor(
    code: BridgeErrorCode,
    message: string,
    details?: Readonly<Record<string, JsonValue>>,
    retryable = false
  ) {
    super(message);
    this.name = "ProtocolViolation";
    this.bridgeError = Object.freeze({
      kind: "bridge",
      code,
      message,
      retryable,
      ...(details === undefined ? {} : { details })
    });
  }
}

export const bridgeFailure = (
  code: BridgeErrorCode,
  message: string,
  details?: Readonly<Record<string, JsonValue>>,
  retryable = false
): BridgeError => new ProtocolViolation(code, message, details, retryable).bridgeError;
