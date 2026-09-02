import { validateFacetValue } from "./contracts.js";
import type { ContextContributionEnvelopeV1 } from "./types.js";

export interface ContextFacetRoute {
  readonly pluginId: string;
  readonly contextType: string;
  readonly schemaDigest: string;
  readonly accept: (envelope: ContextContributionEnvelopeV1) => void;
}

/** Contract dispatch only. No transports, AppFlowy types, or Agent loop hooks. */
export class ContextFacetInbox {
  private readonly routes = new Map<string, ContextFacetRoute>();

  register(route: ContextFacetRoute): () => void {
    const key = this.key(route.pluginId, route.contextType);
    if (this.routes.has(key)) throw new Error("FACET_ALREADY_REGISTERED");
    this.routes.set(key, route);
    return () => { if (this.routes.get(key) === route) this.routes.delete(key); };
  }

  dispatch(raw: unknown, now = Date.now()): void {
    const envelope = validateFacetValue<ContextContributionEnvelopeV1>("context-contribution", raw);
    const route = this.routes.get(this.key(envelope.pluginId, envelope.contextType));
    if (route === undefined) throw new Error("FACET_NOT_FOUND");
    if (route.schemaDigest !== envelope.contextSchemaDigest) throw new Error("SCHEMA_DIGEST_MISMATCH");
    if (envelope.expiresAt <= now || envelope.expiresAt <= envelope.capturedAt) throw new Error("CONTEXT_EXPIRED");
    route.accept(envelope);
  }

  private key(plugin: string, type: string): string { return JSON.stringify([plugin, type]); }
}
