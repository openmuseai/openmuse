import {
  validateFacetValue,
  decideContractEdge,
  type ContractRefV1,
  type FacetDescriptorV1,
  type PluginDescriptorV1
} from "@muse/plugin-facets";

export * from "./v2.js";

export type GraphStatus = "active" | "degraded" | "blocked" | "draining" | "disposed";

export interface FacetActivation {
  readonly healthy: boolean;
  dispose(): Promise<void> | void;
}

export interface FacetActivator {
  activate(plugin: PluginDescriptorV1, facet: FacetDescriptorV1, generation: number): Promise<FacetActivation>;
}

export interface PluginGraphEntry {
  readonly pluginId: string;
  readonly version: string;
  readonly generation: number;
  readonly status: GraphStatus;
  readonly facetCount: number;
  readonly activeEdges: number;
  readonly reasonCode?: string;
}

interface ActivePlugin {
  descriptor: PluginDescriptorV1;
  generation: number;
  status: GraphStatus;
  edges: number;
  activations: FacetActivation[];
}

export class MusePluginGraph {
  private readonly plugins = new Map<string, ActivePlugin>();
  private readonly namespaces = new Map<string, string>();
  private nextGeneration = 1;

  async activate(raw: unknown, activator: FacetActivator): Promise<PluginGraphEntry> {
    const descriptor = validateFacetValue<PluginDescriptorV1>("plugin-descriptor", raw);
    const generation = this.nextGeneration++;
    const namespaceKeys = descriptor.facets.map(facet => `${facet.runtime}:${facet.artifactRef}`);
    for (const namespace of namespaceKeys) {
      const owner = this.namespaces.get(namespace);
      if (owner !== undefined && owner !== descriptor.pluginId) {
        return this.blocked(descriptor, generation, "NAMESPACE_CONFLICT");
      }
    }

    const edges = compatibleEdges(descriptor);
    const shadow: FacetActivation[] = [];
    try {
      for (const facet of descriptor.facets) {
        const activation = await activator.activate(descriptor, facet, generation);
        if (!activation.healthy) throw new Error("UNHEALTHY_FACET");
        shadow.push(activation);
      }
    } catch {
      await disposeReverse(shadow);
      return this.blocked(descriptor, generation, "SHADOW_ACTIVATION_FAILED");
    }

    const previous = this.plugins.get(descriptor.pluginId);
    for (const namespace of namespaceKeys) this.namespaces.set(namespace, descriptor.pluginId);
    const active: ActivePlugin = {
      descriptor, generation, status: "active", edges, activations: shadow
    };
    this.plugins.set(descriptor.pluginId, active);
    if (previous !== undefined) {
      previous.status = "draining";
      await disposeReverse(previous.activations);
      previous.status = "disposed";
      for (const facet of previous.descriptor.facets) {
        const namespace = `${facet.runtime}:${facet.artifactRef}`;
        if (!namespaceKeys.includes(namespace)) this.namespaces.delete(namespace);
      }
    }
    return inventory(active);
  }

  async unload(pluginId: string): Promise<void> {
    const plugin = this.plugins.get(pluginId);
    if (plugin === undefined) return;
    plugin.status = "draining";
    await disposeReverse(plugin.activations);
    plugin.status = "disposed";
    this.plugins.delete(pluginId);
    for (const facet of plugin.descriptor.facets) {
      this.namespaces.delete(`${facet.runtime}:${facet.artifactRef}`);
    }
  }

  inventory(): readonly PluginGraphEntry[] {
    return [...this.plugins.values()].map(inventory);
  }

  private blocked(descriptor: PluginDescriptorV1, generation: number, reasonCode: string): PluginGraphEntry {
    return {
      pluginId: descriptor.pluginId,
      version: descriptor.version,
      generation,
      status: "blocked",
      facetCount: descriptor.facets.length,
      activeEdges: 0,
      reasonCode
    };
  }
}

const compatibleEdges = (descriptor: PluginDescriptorV1): number => {
  const publications = descriptor.facets.flatMap(facet => facet.publishes);
  const consumptions = descriptor.facets.flatMap(facet => facet.consumes);
  let count = 0;
  for (const consumption of consumptions) {
    const publication = publications.find(candidate =>
      candidate.kind === consumption.kind && candidate.type === consumption.type);
    if (decideContractEdge(publication as ContractRefV1 | undefined, consumption).status === "enabled") count++;
  }
  return count;
};

const disposeReverse = async (activations: readonly FacetActivation[]): Promise<void> => {
  for (const activation of [...activations].reverse()) await activation.dispose();
};

const inventory = (plugin: ActivePlugin): PluginGraphEntry => ({
  pluginId: plugin.descriptor.pluginId,
  version: plugin.descriptor.version,
  generation: plugin.generation,
  status: plugin.status,
  facetCount: plugin.descriptor.facets.length,
  activeEdges: plugin.edges
});
