import type { ProtocolError, Receipt } from "@muse/host-bridge";

export class MusePluginCompatibilityError extends Error {
  constructor(public readonly code: string, message: string) {
    super(message);
    this.name = "MusePluginCompatibilityError";
  }
}

export class MuseToolInvocationError extends Error {
  readonly code: string;
  readonly retryable: boolean;
  readonly receiptId: string | undefined;

  constructor(error: ProtocolError, receipt?: Receipt) {
    super(error.message);
    this.name = "MuseToolInvocationError";
    this.code = error.kind === "bridge" ? error.code : `${error.namespace}:${error.code}`;
    this.retryable = error.retryable;
    this.receiptId = receipt?.receiptId;
  }
}
