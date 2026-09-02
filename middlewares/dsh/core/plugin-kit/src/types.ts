import type {
  CapabilityDescriptor,
  JsonValue,
  OperationDescriptor,
  Receipt,
  ScopeHint,
  SchemaResourceV1,
  ValidatedSchema,
  WireBinding
} from "@muse/host-bridge";
import type { MuseApprovalProofRequester } from "@muse/host-bridge/dsh";
import type { JsonSchemaNode, ObjectJsonSchema, ToolRunContext } from "@deepseek-ai/dsh-tools";

export interface MusePluginContractRange {
  readonly major: number;
  readonly minMinor: number;
  readonly maxMinor: number;
}

export interface MuseProviderOperationContract {
  readonly descriptor: CapabilityDescriptor;
  readonly operation: OperationDescriptor;
  readonly inputSchema: JsonValue;
  readonly outputSchema: JsonValue;
}

export interface MuseToolAdapter {
  /** Fail-closed semantic check owned by the Plugin, never by Bridge. */
  accepts(contract: MuseProviderOperationContract): boolean | Promise<boolean>;
  toProviderInput(args: JsonValue, contract: MuseProviderOperationContract): JsonValue | Promise<JsonValue>;
  fromProviderOutput(
    value: JsonValue,
    receipt: Receipt,
    contract: MuseProviderOperationContract
  ): JsonValue | Promise<JsonValue>;
}

export interface MuseToolContribution {
  readonly name: string;
  readonly description: string;
  readonly operationId: string;
  /** Plugin-owned, stable model contract. It is never copied from Provider blindly. */
  readonly parameters: ObjectJsonSchema;
  readonly output: JsonSchemaNode;
  readonly adapter: MuseToolAdapter;
  readonly timeoutMs?: number;
  readonly optional?: boolean;
  /** A write Tool must resolve Host policy before the final invoke. */
  readonly policy?: {
    readonly effect: "local_write" | "sync_write" | "external_side_effect";
  };
  render?(args: JsonValue, value: JsonValue): readonly { readonly type: "text"; readonly text: string }[];
}

export interface MuseCapabilityTarget {
  readonly familyId: string;
  readonly contract: MusePluginContractRange;
  readonly requiredOperations: readonly string[];
  readonly optionalOperations?: readonly string[];
  readonly optional?: boolean;
  readonly tools: readonly MuseToolContribution[];
}

export interface MusePluginDefinition {
  readonly pluginId: string;
  readonly version: string;
  readonly bridgeMajor: 1;
  readonly targets: readonly MuseCapabilityTarget[];
}

export interface MusePluginRuntimeConfig {
  readonly discoveryTimeoutMs?: number;
  readonly invocationTimeoutMs?: number;
  readonly pageSize?: number;
  readonly scopeHint?: ScopeHint;
  /** Deployment-owned approval channel. A Plugin cannot supply or forge it. */
  readonly approvalProofRequester?: MuseApprovalProofRequester;
  resolveSchema?(resource: SchemaResourceV1, signal: AbortSignal): Promise<unknown | string | Uint8Array>;
  onDiagnostic?(diagnostic: MusePluginDiagnostic): void;
}

export type MusePluginPhase = "discovering" | "active" | "incompatible" | "refreshing" | "disposed";

export interface MusePluginDiagnostic {
  readonly pluginId: string;
  readonly phase: MusePluginPhase;
  readonly code: string;
  /** Safe, human-readable compatibility detail. It must never contain credentials or request payloads. */
  readonly detail?: string;
  readonly toolNames: readonly string[];
}

export interface MusePreparedTool {
  readonly contribution: MuseToolContribution;
  readonly contract: MuseProviderOperationContract;
  readonly binding: WireBinding;
  readonly inputValidator: ValidatedSchema;
  readonly outputValidator: ValidatedSchema;
}

export interface MuseToolExecutionSource {
  readonly exec: ToolRunContext;
}
