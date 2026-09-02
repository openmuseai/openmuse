import { describe, expect, it } from "vitest";
import type { BridgeEventPayload, JsonValue } from "@muse/host-bridge";
import { MuseContextBroker } from "../src/index.js";

const digest = `sha256:${"a".repeat(64)}`;
let cursor = 0;

const event = (revision: string, expiresAt = 2_000, payload: JsonValue = { selected: "hello" }): BridgeEventPayload => ({
  subscriptionId: "subscription.1" as never,
  cursor: `cursor.${++cursor}` as never,
  occurredAt: 1_000,
  hostGeneration: "generation.1" as never,
  data: {
    eventKind: "provider.event",
    descriptorId: "muse.ui-context" as never,
    descriptorRevision: "1" as never,
    eventType: "context.updated",
    schemaDigest: digest as never,
    payload: {
      protocol: "muse.context-contribution/v1",
      pluginId: "muse.test",
      pluginVersion: "1.0.0",
      facetInstanceRef: "facet.1",
      surfaceInstanceRef: "surface.1",
      surfaceKind: "test.editor",
      scopeRef: "workspace.1",
      contextType: "test.selection",
      contextSchemaDigest: digest,
      contextRevision: revision,
      epochRef: "epoch.1",
      lane: "state",
      capturedAt: 1_000,
      expiresAt,
      payload
    }
  }
});

describe("MuseContextBroker", () => {
  it("keeps latest revision and renders through a matching projection", () => {
    const broker = new MuseContextBroker({ clock: () => 1_000 });
    broker.registerProjection({
      pluginId: "muse.test",
      contextType: "test.selection",
      schemaDigest: digest,
      priority: 10,
      maxTokens: 50,
      render: envelope => `Selection: ${(envelope.payload as { selected: string }).selected}`
    });
    broker.ingest(event("2"));
    broker.ingest(event("1", 2_000, { selected: "stale" }));
    expect(broker.inventory()[0]?.revision).toBe("2");
    expect(broker.render()).toContain("Selection: hello");
  });

  it("expires snapshots and projection dispose clears owned data", () => {
    let now = 1_000;
    const broker = new MuseContextBroker({ clock: () => now });
    const dispose = broker.registerProjection({
      pluginId: "muse.test", contextType: "test.selection", schemaDigest: digest,
      priority: 1, maxTokens: 20, render: () => "visible"
    });
    broker.ingest(event("1", 1_100));
    expect(broker.render()).toContain("visible");
    now = 1_101;
    expect(broker.render()).toBe("");
    broker.ingest(event("2", 2_000));
    dispose();
    expect(broker.inventory()).toHaveLength(0);
  });

  it("isolates renderer errors and enforces the prompt budget", () => {
    const broker = new MuseContextBroker({ clock: () => 1_000, maxTotalTokens: 3 });
    broker.registerProjection({
      pluginId: "muse.test", contextType: "test.selection", schemaDigest: digest,
      priority: 1, maxTokens: 10, render: () => "abcdefghijklmnopqrstuvwxyz"
    });
    broker.ingest(event("1"));
    expect(broker.render()).toContain("abcdefghijkl");
    expect(broker.render()).not.toContain("mno");
  });

  it("pin selects one surface and close removes it", () => {
    const broker = new MuseContextBroker({ clock: () => 1_000 });
    broker.registerProjection({
      pluginId: "muse.test", contextType: "test.selection", schemaDigest: digest,
      priority: 1, maxTokens: 20, render: () => "context"
    });
    broker.ingest(event("1"));
    const disposePin = broker.pinSurface("surface.1");
    expect(broker.render()).toContain('surface="surface.1"');
    broker.removeSurface("surface.1");
    expect(broker.render()).toBe("");
    disposePin();
  });

  it("ingestContribution accepts a context envelope without a Bridge event", () => {
    const broker = new MuseContextBroker({ clock: () => 1_000 });
    broker.registerProjection({
      pluginId: "muse.test",
      contextType: "test.selection",
      schemaDigest: digest,
      priority: 1,
      maxTokens: 20,
      render: () => "from-web"
    });
    broker.ingestContribution({
      protocol: "muse.context-contribution/v1",
      pluginId: "muse.test",
      pluginVersion: "1.0.0",
      facetInstanceRef: "facet.1",
      surfaceInstanceRef: "surface.1",
      surfaceKind: "test.editor",
      scopeRef: "workspace.1",
      contextType: "test.selection",
      contextSchemaDigest: digest,
      contextRevision: "3",
      epochRef: "epoch.1",
      lane: "state",
      capturedAt: 1_000,
      expiresAt: 2_000,
      payload: { selected: "hello" }
    });
    expect(broker.inventory()[0]?.revision).toBe("3");
    expect(broker.render()).toContain("from-web");
  });
});
