import type { BridgeEventPayload, JsonValue } from "@muse/host-bridge";
import {
  validateFacetValue,
  type ContextContributionEnvelopeV1
} from "@muse/plugin-facets";

export interface ContextProjection {
  readonly pluginId: string;
  readonly contextType: string;
  readonly schemaDigest: string;
  readonly priority: number;
  readonly maxTokens: number;
  render(envelope: ContextContributionEnvelopeV1): string | undefined;
}

export interface BrokerInventoryItem {
  readonly pluginId: string;
  readonly surfaceInstanceRef: string;
  readonly contextType: string;
  readonly revision: string;
  readonly expiresAt: number;
  readonly bytes: number;
  readonly status: "ready" | "unprojected" | "projection-error";
}

type ProjectionKey = string;
type SnapshotKey = string;

interface SnapshotRecord {
  readonly envelope: ContextContributionEnvelopeV1;
  readonly bytes: number;
  status: BrokerInventoryItem["status"];
}

export interface MuseContextBrokerOptions {
  readonly maxTotalTokens?: number;
  readonly maxPayloadBytes?: number;
  readonly clock?: () => number;
}

const keyOf = (pluginId: string, contextType: string, digest: string): ProjectionKey =>
  `${pluginId}\u0000${contextType}\u0000${digest}`;

const snapshotKeyOf = (envelope: ContextContributionEnvelopeV1): SnapshotKey =>
  `${envelope.surfaceInstanceRef}\u0000${keyOf(envelope.pluginId, envelope.contextType, envelope.contextSchemaDigest)}`;

const estimateTokens = (text: string): number => Math.ceil(text.length / 4);

export class MuseContextBroker {
  private readonly projections = new Map<ProjectionKey, ContextProjection>();
  private readonly snapshots = new Map<SnapshotKey, SnapshotRecord>();
  private readonly pinned = new Set<string>();
  private readonly maxTotalTokens: number;
  private readonly maxPayloadBytes: number;
  private readonly clock: () => number;

  constructor(options: MuseContextBrokerOptions = {}) {
    this.maxTotalTokens = options.maxTotalTokens ?? 2_000;
    this.maxPayloadBytes = options.maxPayloadBytes ?? 64 * 1024;
    this.clock = options.clock ?? Date.now;
  }

  registerProjection(projection: ContextProjection): () => void {
    if (!Number.isSafeInteger(projection.priority) || projection.maxTokens <= 0) {
      throw new TypeError("projection priority/maxTokens are invalid");
    }
    const key = keyOf(projection.pluginId, projection.contextType, projection.schemaDigest);
    if (this.projections.has(key)) throw new Error("projection already registered");
    this.projections.set(key, projection);
    return () => {
      this.projections.delete(key);
      for (const [snapshotKey, record] of this.snapshots) {
        if (keyOf(
          record.envelope.pluginId,
          record.envelope.contextType,
          record.envelope.contextSchemaDigest
        ) === key) this.snapshots.delete(snapshotKey);
      }
    };
  }

  pinSurface(surfaceInstanceRef: string): () => void {
    this.pinned.add(surfaceInstanceRef);
    return () => this.pinned.delete(surfaceInstanceRef);
  }

  ingest(event: BridgeEventPayload): void {
    if (event.data.eventKind === "stream.gap") return;
    if (event.data.eventKind !== "provider.event") return;
    if (event.data.eventType === "surface.closed") {
      const payload = event.data.payload as { surfaceInstanceRef?: unknown };
      if (typeof payload.surfaceInstanceRef === "string") this.removeSurface(payload.surfaceInstanceRef);
      return;
    }
    if (event.data.eventType !== "context.updated") return;
    const raw = event.data.payload as JsonValue;
    const envelope = validateFacetValue<ContextContributionEnvelopeV1>("context-contribution", raw);
    const bytes = Buffer.byteLength(JSON.stringify(raw));
    if (bytes > this.maxPayloadBytes || envelope.expiresAt <= this.clock()) return;
    const key = snapshotKeyOf(envelope);
    const previous = this.snapshots.get(key)?.envelope;
    if (previous !== undefined && previous.epochRef === envelope.epochRef &&
      BigInt(envelope.contextRevision) <= BigInt(previous.contextRevision)) return;
    this.snapshots.set(key, {
      envelope,
      bytes,
      status: this.projections.has(keyOf(
        envelope.pluginId,
        envelope.contextType,
        envelope.contextSchemaDigest
      )) ? "ready" : "unprojected"
    });
  }

  /**
   * Ingest a ContextContribution envelope without a Bridge provider event.
   * Used by the Web parent-bridge (iframe postMessage → Host HTTP).
   */
  ingestContribution(payload: JsonValue): void {
    const now = this.clock();
    this.ingest({
      subscriptionId: "subscription.parent-bridge" as never,
      cursor: `cursor.parent-bridge.${now}` as never,
      occurredAt: now,
      hostGeneration: "appflowy.web.1" as never,
      data: {
        eventKind: "provider.event",
        descriptorId: "appflowy.web.parent" as never,
        descriptorRevision: "1" as never,
        eventType: "context.updated",
        schemaDigest: `sha256:${"0".repeat(64)}` as never,
        payload
      }
    });
  }

  render(surfaceInstanceRef?: string): string {
    this.expire();
    const grouped = new Map<string, SnapshotRecord[]>();
    for (const record of this.snapshots.values()) {
      const group = grouped.get(record.envelope.surfaceInstanceRef) ?? [];
      group.push(record);
      grouped.set(record.envelope.surfaceInstanceRef, group);
    }
    const selected = surfaceInstanceRef ??
      [...this.pinned].find(ref => grouped.has(ref)) ??
      [...grouped.keys()][0];
    if (selected === undefined) return "";
    const records = grouped.get(selected) ?? [];
    const rendered: Array<{ priority: number; maxTokens: number; text: string }> = [];
    for (const record of records) {
      const projection = this.projections.get(keyOf(
        record.envelope.pluginId,
        record.envelope.contextType,
        record.envelope.contextSchemaDigest
      ));
      if (projection === undefined) continue;
      try {
        const value = projection.render(record.envelope);
        if (value === undefined || value.length === 0) continue;
        record.status = "ready";
        rendered.push({ priority: projection.priority, maxTokens: projection.maxTokens, text: value });
      } catch {
        record.status = "projection-error";
      }
    }
    rendered.sort((left, right) => right.priority - left.priority || left.text.localeCompare(right.text));
    let remaining = this.maxTotalTokens;
    const contributions: string[] = [];
    for (const item of rendered) {
      const allowance = Math.min(remaining, item.maxTokens);
      if (allowance <= 0) break;
      const text = estimateTokens(item.text) <= allowance
        ? item.text
        : item.text.slice(0, allowance * 4);
      contributions.push(text);
      remaining -= estimateTokens(text);
    }
    if (contributions.length === 0) return "";
    return `<muse-context surface="${selected}">\n${contributions.join("\n\n")}\n</muse-context>`;
  }

  inventory(): readonly BrokerInventoryItem[] {
    this.expire();
    return [...this.snapshots.values()].map(record => ({
      pluginId: record.envelope.pluginId,
      surfaceInstanceRef: record.envelope.surfaceInstanceRef,
      contextType: record.envelope.contextType,
      revision: record.envelope.contextRevision,
      expiresAt: record.envelope.expiresAt,
      bytes: record.bytes,
      status: record.status
    }));
  }

  removeSurface(surfaceInstanceRef: string): void {
    for (const [key, record] of this.snapshots) {
      if (record.envelope.surfaceInstanceRef === surfaceInstanceRef) this.snapshots.delete(key);
    }
    this.pinned.delete(surfaceInstanceRef);
  }

  dispose(): void {
    this.projections.clear();
    this.snapshots.clear();
    this.pinned.clear();
  }

  private expire(): void {
    const now = this.clock();
    for (const [key, record] of this.snapshots) {
      if (record.envelope.expiresAt <= now) this.snapshots.delete(key);
    }
  }
}
