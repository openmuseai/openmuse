import { readFileSync } from 'fs';
import { resolve } from 'path';
import { WorkspaceUiFacet } from '../dsh-workspace-facet';
import { buildMarkdownSurfaceEnvelope, buildMarkdownSelectionEnvelope } from '../dsh-hybrid';

const golden = JSON.parse(readFileSync(resolve(process.cwd(), '../packages/plugins/dsh-appflowy/fixtures/presentation-v1.json'), 'utf8'));

describe('shared Web / native / Host Facet contract', () => {
  it('produces the plugin-owned markdown payload and document scope', () => {
    const envelope = buildMarkdownSurfaceEnvelope({ workspaceId: 'ws-1', viewId: 'view-1', title: '文档 😀', revision: 1, now: 1000 });
    expect(envelope.payload).toEqual(golden.context.payload);
    expect(envelope.scopeRef).toBe(golden.context.scopeRef);
    expect(envelope.contextSchemaDigest).toBe(golden.context.contextSchemaDigest);
    const selection = buildMarkdownSelectionEnvelope({ workspaceId: 'ws-1', viewId: 'view-1', selectedText: '😀'.repeat(2050), collapsed: false, revision: 1, now: 1000 });
    const payload = selection.payload as { selectedText: string; selectedBlockRefs: string[] };
    expect(Array.from(payload.selectedText)).toHaveLength(2048);
    expect(payload.selectedBlockRefs).toEqual([]);
  });
  it('awaits navigation, caches the receipt and rejects duplicate IDs with different content', async () => {
    let complete!: () => void;
    const toView = jest.fn(() => new Promise<void>(resolve => { complete = resolve; }));
    const facet = new WorkspaceUiFacet({ workspaceId: 'ws-1', toView, now: 2000 });
    const first = facet.dispatch(golden.intent);
    const repeated = facet.dispatch(golden.intent);
    expect(toView).toHaveBeenCalledTimes(1);
    let finished = false;
    void first.then(() => { finished = true; });
    await Promise.resolve(); expect(finished).toBe(false);
    complete();
    expect((await first).status).toBe('applied');
    expect(await repeated).toEqual(await first);
    expect((await facet.dispatch({ ...golden.intent, payload: { viewId: 'foreign' } })).reasonCode).toBe('INTENT_ID_REUSED');
  });
  it('fails closed for plugin, digest, scope, stale and async navigation failures', async () => {
    const toView = jest.fn(async () => { throw new Error('navigation failed'); });
    for (const override of [{ pluginId: 'foreign.plugin' }, { intentSchemaDigest: `sha256:${'0'.repeat(64)}` }, { scopeRef: 'workspace.foreign' }, { expiresAt: 1 }]) {
      const facet = new WorkspaceUiFacet({ workspaceId: 'ws-1', toView, now: 2000 });
      expect((await facet.dispatch({ ...golden.intent, ...override })).status).not.toBe('applied');
    }
    expect(toView).not.toHaveBeenCalled();
    const facet = new WorkspaceUiFacet({ workspaceId: 'ws-1', toView, now: 2000 });
    expect((await facet.dispatch(golden.intent)).reasonCode).toBe('NAVIGATION_FAILED');
    facet.close();
    expect((await facet.dispatch(golden.intent)).status).toBe('surface-closed');
  });
});
