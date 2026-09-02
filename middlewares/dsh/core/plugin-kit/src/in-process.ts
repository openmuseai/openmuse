import type { JsonValue } from "@muse/host-bridge";

/** Context the InProcess composition host passes into a Domain provider. */
export interface InProcessInvokeContext {
  readonly boundWorkspaceId?: string;
  readonly documentFocus?: { readonly workspaceId: string; readonly viewId: string };
  readonly accessToken?: string;
  readonly deviceId?: string;
  readonly cloudBaseUrl?: string;
}

export type InProcessInvokeResult =
  | { readonly ok: true; readonly value: JsonValue }
  | { readonly ok: false; readonly code: string; readonly message: string };

export interface InProcessOperationSchema {
  readonly operationId: string;
  readonly effect: string;
  readonly inputSchema: { readonly sha256: string };
  readonly outputSchema: { readonly sha256: string };
  readonly cancellable: boolean;
  readonly idempotency: string;
}

/** Domain Facet projection for an InProcess (Cloud/E2E) Host. Assembly wires these; Bridge stays generic. */
export interface InProcessDomainProvider {
  readonly descriptor: {
    readonly descriptorId: string;
    readonly revision: string;
    readonly familyId: string;
    readonly contractVersion: { readonly major: number; readonly minor: number };
    readonly providerInstanceId: string;
    readonly operations: readonly InProcessOperationSchema[];
    readonly events: readonly unknown[];
  };
  readonly bindingId: string;
  prepareInput?(input: JsonValue, ctx: InProcessInvokeContext): JsonValue;
  invoke(request: {
    readonly operationId: string;
    readonly input: JsonValue;
    readonly ctx: InProcessInvokeContext;
  }): Promise<InProcessInvokeResult>;
}
