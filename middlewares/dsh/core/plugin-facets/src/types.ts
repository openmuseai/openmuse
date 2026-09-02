import type { JsonValue } from "@muse/host-bridge";

export type FacetKind = "ui" | "domain" | "agent";
export type FacetRuntime = "flutter" | "rust-host" | "dsh-cordis";
export type ContractKind = "context" | "domain-event" | "presentation-intent" | "capability";
export type ContextLane = "control" | "state";
export type MutationOrigin = "ui-optimistic" | "remote-collab" | "external-command" | "recovery-replay" | "unknown";

export interface ContractRefV1 {
  readonly kind: ContractKind;
  readonly type: string;
  readonly schemaDigest: string;
  readonly required: boolean;
  readonly versionRange?: string;
}

export interface FacetDescriptorV1 {
  readonly facetKind: FacetKind;
  readonly artifactRef: string;
  readonly runtime: FacetRuntime;
  readonly requiresKernel: string;
  readonly publishes: readonly ContractRefV1[];
  readonly consumes: readonly ContractRefV1[];
  readonly optionalRequires?: readonly string[];
}

export interface PluginDescriptorV1 {
  readonly protocol: "muse.plugin-descriptor/v1";
  readonly pluginId: string;
  readonly version: string;
  readonly publisher?: { readonly id: string; readonly signatureRef?: string };
  readonly facets: readonly FacetDescriptorV1[];
  readonly metadata?: Readonly<Record<string, JsonValue>>;
}

export interface ContextContributionEnvelopeV1 {
  readonly protocol: "muse.context-contribution/v1";
  readonly pluginId: string;
  readonly pluginVersion: string;
  readonly facetInstanceRef: string;
  readonly surfaceInstanceRef: string;
  readonly surfaceKind: string;
  readonly scopeRef: string;
  readonly contextType: string;
  readonly contextSchemaDigest: string;
  readonly contextRevision: string;
  readonly epochRef: string;
  readonly lane: ContextLane;
  readonly capturedAt: number;
  readonly expiresAt: number;
  readonly payload: JsonValue;
}

export interface DomainChangeEnvelopeV1 {
  readonly protocol: "muse.domain-change/v1";
  readonly pluginId: string;
  readonly providerInstanceRef: string;
  readonly scopeRef: string;
  readonly resourceRef: string;
  readonly eventType: string;
  readonly eventSchemaDigest: string;
  readonly domainRevision: string;
  readonly epochRef: string;
  readonly commandRef?: string;
  readonly origin: MutationOrigin;
  readonly occurredAt: number;
  readonly payload: JsonValue;
}

export interface PresentationIntentEnvelopeV1 {
  readonly protocol: "muse.presentation-intent/v1";
  readonly pluginId: string;
  readonly targetSurfaceInstanceRef?: string;
  readonly scopeRef: string;
  readonly intentType: string;
  readonly intentSchemaDigest: string;
  readonly intentRef: string;
  readonly requestedAt: number;
  readonly expiresAt: number;
  readonly payload: JsonValue;
}

export interface PresentationIntentResultV1 {
  readonly protocol: "muse.presentation-intent-result/v1";
  readonly intentRef: string;
  readonly status: "applied" | "rejected" | "stale" | "not-found" | "not-supported" | "surface-closed" | "timed-out";
  readonly appliedSurfaceInstanceRef?: string;
  readonly observedDomainRevision?: string;
  readonly reasonCode?: string;
  readonly completedAt: number;
}

export type FacetWireValueV1 = PluginDescriptorV1 | ContextContributionEnvelopeV1 |
  DomainChangeEnvelopeV1 | PresentationIntentEnvelopeV1 | PresentationIntentResultV1;

export type FacetSchemaKind = "plugin-descriptor" | "context-contribution" | "domain-change" |
  "presentation-intent" | "presentation-intent-result";

export type CompositionStatus = "enabled" | "degraded" | "disabled";

export interface CompositionDecision {
  readonly status: CompositionStatus;
  readonly reasonCode?: "TYPE_MISMATCH" | "SCHEMA_DIGEST_MISMATCH" | "MISSING_FACET" |
    "KERNEL_TOO_OLD" | "SCOPE_DENIED";
}
