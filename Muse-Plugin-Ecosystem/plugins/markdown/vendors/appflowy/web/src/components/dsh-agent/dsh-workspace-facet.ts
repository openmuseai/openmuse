import type { MusePresentationIntent, MusePresentationIntentResult } from './dsh-origin';

export const SURFACE_OPEN_DIGEST = 'sha256:c6337b4511a9ef40dbae92c23784cec068a2ff355ac6bb8e95141328c34905f0';
export const SURFACE_REVEAL_DIGEST = 'sha256:f50404c3696300a2f00c7ac9ba5f413d48987d458669bf397fad0c6b85eabd8b';
export type ApplyIntentContext = {
  workspaceId?: string;
  toView?: (viewId: string, blockId?: string) => Promise<void> | void;
  now?: number;
};

/** React UI Facet. No postMessage, EventSource, Cloud auth, or Agent dependency. */
export class WorkspaceUiFacet {
  private closed = false;
  private readonly receipts = new Map<string, { input: string; result: Promise<MusePresentationIntentResult> }>();
  private readonly contracts = new Map([
    ['surface.open', SURFACE_OPEN_DIGEST], ['surface.revealRange', SURFACE_REVEAL_DIGEST],
  ]);
  constructor(private readonly context: ApplyIntentContext) {}
  close() { this.closed = true; this.receipts.clear(); }

  dispatch(intent: MusePresentationIntent): Promise<MusePresentationIntentResult> {
    const input = JSON.stringify(intent);
    const previous = this.receipts.get(intent.intentRef);
    if (this.closed) return Promise.resolve(this.result(intent, 'surface-closed', 'FACET_CLOSED'));
    if (previous) return previous.input === input ? previous.result : Promise.resolve(this.result(intent, 'rejected', 'INTENT_ID_REUSED'));
    if (this.receipts.size >= 256) return Promise.resolve(this.result(intent, 'rejected', 'INBOX_CAPACITY'));
    const result = this.apply(intent);
    this.receipts.set(intent.intentRef, { input, result });
    return result;
  }
  private result(intent: MusePresentationIntent, status: MusePresentationIntentResult['status'], reasonCode?: string): MusePresentationIntentResult {
    return { protocol: 'muse.presentation-intent-result/v1', intentRef: intent.intentRef,
      status, completedAt: this.context.now ?? Date.now(), ...(reasonCode ? { reasonCode } : {}) };
  }
  private async apply(intent: MusePresentationIntent): Promise<MusePresentationIntentResult> {
    const now = this.context.now ?? Date.now();
    if (intent.expiresAt <= now) return this.result(intent, 'stale', 'INTENT_EXPIRED');
    if (!this.context.workspaceId || intent.scopeRef !== `workspace.${this.context.workspaceId}` ||
        (intent.payload.workspaceId !== undefined && intent.payload.workspaceId !== this.context.workspaceId)) {
      return this.result(intent, 'rejected', 'SCOPE_MISMATCH');
    }
    if (intent.targetSurfaceInstanceRef && intent.targetSurfaceInstanceRef !== `surface.appflowy.workspace.${this.context.workspaceId}`) {
      return this.result(intent, 'not-found', 'SURFACE_NOT_FOUND');
    }
    if (intent.pluginId !== 'muse.appflowy.workspace' || !this.contracts.has(intent.intentType)) return this.result(intent, 'not-supported', 'FACET_NOT_FOUND');
    if (this.contracts.get(intent.intentType) !== intent.intentSchemaDigest) return this.result(intent, 'rejected', 'SCHEMA_DIGEST_MISMATCH');
    const viewId = typeof intent.payload.viewId === 'string' ? intent.payload.viewId.trim() : '';
    if (!viewId) return this.result(intent, 'rejected', 'VIEW_ID_REQUIRED');
    if (!this.context.toView) return this.result(intent, 'not-supported', 'NAVIGATION_UNAVAILABLE');
    const blockId = intent.intentType === 'surface.revealRange' && typeof intent.payload.blockId === 'string' ? intent.payload.blockId : undefined;
    if (intent.intentType === 'surface.revealRange' && !blockId) return this.result(intent, 'not-supported', 'RANGE_NOT_SUPPORTED');
    let timer: ReturnType<typeof setTimeout> | undefined;
    try {
      const expired = new Promise<'timed-out'>(resolve => { timer = setTimeout(() => resolve('timed-out'), Math.min(intent.expiresAt - now, 30_000)); });
      const status = await Promise.race([Promise.resolve(this.context.toView(viewId, blockId)).then(() => 'applied' as const), expired]);
      if (this.closed) return this.result(intent, 'surface-closed', 'FACET_CLOSED');
      return this.result(intent, status);
    } catch { return this.result(intent, 'rejected', 'NAVIGATION_FAILED'); }
    finally { clearTimeout(timer); }
  }
}
