import { describe, expect, it } from "vitest";
import { MusePluginGraph, type FacetActivator } from "../src/index.js";

const digest = `sha256:${"a".repeat(64)}`;
const descriptor = (version: string, artifact = "mock/spreadsheet-ui") => ({
  protocol: "muse.plugin-descriptor/v1",
  pluginId: "muse.mock.spreadsheet",
  version,
  facets: [
    {
      facetKind: "ui", artifactRef: artifact, runtime: "flutter", requiresKernel: ">=1.0.0",
      publishes: [{ kind: "context", type: "mock.active-range", schemaDigest: digest, required: true }],
      consumes: []
    },
    {
      facetKind: "agent", artifactRef: "mock/spreadsheet-agent", runtime: "dsh-cordis", requiresKernel: ">=1.0.0",
      publishes: [],
      consumes: [{ kind: "context", type: "mock.active-range", schemaDigest: digest, required: true }]
    }
  ]
});

describe("MusePluginGraph", () => {
  it("shadow activates, cuts over generation, and reverse-disposes old facets", async () => {
    const disposed: string[] = [];
    const activator: FacetActivator = {
      activate: async (_plugin, facet, generation) => ({
        healthy: true,
        dispose: () => { disposed.push(`${generation}:${facet.facetKind}`); }
      })
    };
    const graph = new MusePluginGraph();
    expect((await graph.activate(descriptor("1.0.0"), activator)).activeEdges).toBe(1);
    const second = await graph.activate(descriptor("1.1.0"), activator);
    expect(second.generation).toBe(2);
    expect(disposed).toEqual(["1:agent", "1:ui"]);
    await graph.unload("muse.mock.spreadsheet");
    expect(graph.inventory()).toEqual([]);
  });

  it("keeps old generation active when shadow health fails", async () => {
    const graph = new MusePluginGraph();
    await graph.activate(descriptor("1.0.0"), {
      activate: async () => ({ healthy: true, dispose: () => undefined })
    });
    const failed = await graph.activate(descriptor("2.0.0"), {
      activate: async (_plugin, facet) => ({ healthy: facet.facetKind !== "agent", dispose: () => undefined })
    });
    expect(failed.status).toBe("blocked");
    expect(graph.inventory()[0]?.version).toBe("1.0.0");
  });

  it("blocks namespace collision across plugins", async () => {
    const graph = new MusePluginGraph();
    const activator: FacetActivator = {
      activate: async () => ({ healthy: true, dispose: () => undefined })
    };
    await graph.activate(descriptor("1.0.0"), activator);
    const other = { ...descriptor("1.0.0"), pluginId: "muse.other" };
    expect((await graph.activate(other, activator)).reasonCode).toBe("NAMESPACE_CONFLICT");
  });
});
