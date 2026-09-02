import { Context, Service } from "@deepseek-ai/cordis";
import type { BridgeEventPayload, JsonValue } from "@muse/host-bridge";
import { MuseContextBroker, type ContextProjection, type BrokerInventoryItem } from "./index.js";

declare module "@deepseek-ai/cordis" {
  interface Context {
    museContextBroker: MuseContextBrokerService;
    systemPrompt: {
      context(input: { name: string; order: number; text: () => string }): () => void;
    };
  }
}

export class MuseContextBrokerService extends Service {
  static inject = ["museHost", "systemPrompt"];
  private readonly broker: MuseContextBroker;

  constructor(ctx: Context) {
    super(ctx, "museContextBroker");
    this.broker = new MuseContextBroker();
  }

  async *[Service.init](): AsyncGenerator<() => void, void, void> {
    const offEvent = this.ctx.on("museHost/event" as never, ((event: BridgeEventPayload) => {
      this.broker.ingest(event);
    }) as never);
    const offPrompt = this.ctx.systemPrompt.context({
      name: "muse:surface-context",
      order: 80,
      text: () => this.broker.render()
    });
    yield () => {
      offPrompt();
      offEvent();
      this.broker.dispose();
    };
  }

  registerProjection(projection: ContextProjection): () => void {
    return this.broker.registerProjection(projection);
  }

  pinSurface(surfaceInstanceRef: string): () => void {
    return this.broker.pinSurface(surfaceInstanceRef);
  }

  inventory(): readonly BrokerInventoryItem[] {
    return this.broker.inventory();
  }

  ingestContribution(payload: JsonValue): void {
    this.broker.ingestContribution(payload);
  }

  removeSurface(surfaceInstanceRef: string): void {
    this.broker.removeSurface(surfaceInstanceRef);
  }
}

export default MuseContextBrokerService;
