import { describe, expect, it } from '@jest/globals';
import {
  MAX_DSH_POSTMESSAGE_BYTES,
  containsForbiddenDshFields,
  isOversizedDshMessage,
  parentHelloMessage,
  parseIntentDispatch,
  parseWorkspaceBind,
  workspaceBindMessage,
} from '@/components/dsh-agent/dsh-origin';
import {
  applyPresentationIntent,
  buildMarkdownSnapshotEnvelope,
  buildTreeUiEnvelope,
  buildWorkspaceCatalogEnvelope,
  buildWorkspaceFocusEnvelope,
  contributionMessageOrNull,
  expandedViewIdsFromStorage,
  flattenOutlineToCatalog,
} from '@/components/dsh-agent/dsh-hybrid';

describe('workspace bind protocol', () => {
  it('includes workspaceRef on hello without secrets', () => {
    const msg = parentHelloMessage('dark', 'en', undefined, {
      workspaceRef: 'ws-1',
      workspaceTitle: 'Docs',
    });
    expect(msg.workspaceRef).toBe('ws-1');
    expect(msg.workspaceTitle).toBe('Docs');
    expect(JSON.stringify(msg)).not.toContain('sk-');
    expect(JSON.stringify(msg)).not.toContain('refresh_token');
  });

  it('parses bind and rejects oversized or foreign-source payloads', () => {
    expect(parseWorkspaceBind(workspaceBindMessage('ws-2', 'Beta'))).toEqual({
      source: 'muse.appflowy-web',
      type: 'workspace.bind',
      workspaceRef: 'ws-2',
      workspaceTitle: 'Beta',
    });
    expect(parseWorkspaceBind(workspaceBindMessage('ws-2', 'Beta', { token: 'aaa.bbb', deviceId: 'web.1' }))).toEqual({
      source: 'muse.appflowy-web',
      type: 'workspace.bind',
      workspaceRef: 'ws-2',
      workspaceTitle: 'Beta',
      deviceToken: 'aaa.bbb',
      deviceId: 'web.1',
    });
    expect(parseWorkspaceBind({ source: 'evil', type: 'workspace.bind', workspaceRef: 'ws-2' })).toBeNull();
    expect(
      parseWorkspaceBind({
        source: 'muse.appflowy-web',
        type: 'workspace.bind',
        workspaceRef: 'x'.repeat(200),
      })
    ).toBeNull();
    expect(isOversizedDshMessage({ pad: 'n'.repeat(MAX_DSH_POSTMESSAGE_BYTES) })).toBe(true);
    expect(containsForbiddenDshFields({ access_token: 'nope' })).toBe(true);
  });
});

describe('context contribution builders', () => {
  it('keeps tree UI to an id list under the byte cap', () => {
    const envelope = buildTreeUiEnvelope({
      workspaceId: 'ws-1',
      expandedViewIds: expandedViewIdsFromStorage({ a: true, b: true, c: false }),
      revision: 2,
      now: 1_000,
    });
    expect(envelope.payload).toEqual({ workspaceId: 'ws-1', expandedViewIds: ['a', 'b'] });
    const message = contributionMessageOrNull(envelope);
    expect(message).not.toBeNull();
    expect(JSON.stringify(message)).not.toMatch(/yjs|sk-/);
    expect(isOversizedDshMessage(message)).toBe(false);
  });

  it('builds workspace.focus without document body', () => {
    const envelope = buildWorkspaceFocusEnvelope({
      workspaceId: 'ws-1',
      title: 'Docs',
      viewId: 'view-9',
      revision: 1,
      now: 1_000,
    });
    expect(JSON.stringify(envelope)).not.toContain('# heading');
    expect(envelope.contextType).toBe('workspace.focus');
  });

  it('projects Host outline titles into workspace.catalog without secrets', () => {
    const { items, truncated } = flattenOutlineToCatalog([
      {
        view_id: 'space-1',
        name: 'Team',
        layout: 0,
        extra: { is_space: true },
        children: [
          { view_id: 'doc-1', name: 'Getting started', layout: 0, children: [], parent_view_id: 'space-1' },
          { view_id: 'grid-1', name: 'Tasks', layout: 1, children: [], parent_view_id: 'space-1' },
        ],
      },
    ]);
    expect(truncated).toBe(false);
    expect(items.map((item) => item.title)).toEqual(['Team', 'Getting started', 'Tasks']);
    expect(items[2]?.layout).toBe('grid');
    const envelope = buildWorkspaceCatalogEnvelope({
      workspaceId: 'ws-1',
      outline: [{ view_id: 'v1', name: 'Getting started', layout: 0, children: [] }],
      revision: 1,
      now: 1_000,
    });
    expect(envelope.contextType).toBe('workspace.catalog');
    const message = contributionMessageOrNull(envelope);
    expect(message).not.toBeNull();
    expect(JSON.stringify(message)).not.toMatch(/access_token|yjs|sk-/);
    expect(
      contributionMessageOrNull(
        buildWorkspaceCatalogEnvelope({
          workspaceId: 'ws-1',
          outline: [{ view_id: 'v1', name: 'leaked access_token', layout: 0, children: [] }],
          revision: 1,
          now: 1_000,
        })
      )
    ).toBeNull();
  });

  it('builds a bounded markdown.snapshot without secrets', () => {
    const envelope = buildMarkdownSnapshotEnvelope({
      workspaceId: 'ws-1',
      viewId: 'view-1',
      text: '# Getting started\n\nHello.',
      revision: 1,
      now: 1_000,
    });
    expect(envelope.contextType).toBe('markdown.snapshot');
    expect(contributionMessageOrNull(envelope)).not.toBeNull();
    expect(
      contributionMessageOrNull(
        buildMarkdownSnapshotEnvelope({
          workspaceId: 'ws-1',
          viewId: 'view-1',
          text: 'leaked access_token',
          revision: 1,
          now: 1_000,
        })
      )
    ).toBeNull();
  });
});

describe('presentation intent handler', () => {
  it('rejects stale intents without navigating', async () => {
    const toView = jest.fn();
    const result = await applyPresentationIntent(
      {
        protocol: 'muse.presentation-intent/v1',
        pluginId: 'muse.appflowy.workspace',
        scopeRef: 'workspace.ws-1',
        intentType: 'surface.open',
        intentSchemaDigest: 'sha256:' + 'a'.repeat(64),
        intentRef: 'intent.1',
        requestedAt: 1,
        expiresAt: 10,
        payload: { viewId: 'view-1', workspaceId: 'ws-1' },
      },
      { workspaceId: 'ws-1', toView, now: 11 }
    );
    expect(result.status).toBe('stale');
    expect(toView).not.toHaveBeenCalled();
  });

  it('rejects a workspace mismatch and opens a view when in scope', async () => {
    const toView = jest.fn();
    const mismatch = await applyPresentationIntent(
      {
        protocol: 'muse.presentation-intent/v1',
        pluginId: 'muse.appflowy.workspace',
        scopeRef: 'workspace.other',
        intentType: 'surface.open',
        intentSchemaDigest: 'sha256:' + 'a'.repeat(64),
        intentRef: 'intent.2',
        requestedAt: 1,
        expiresAt: 100,
        payload: { viewId: 'view-1', workspaceId: 'other' },
      },
      { workspaceId: 'ws-1', toView, now: 10 }
    );
    expect(mismatch.status).toBe('rejected');
    expect(mismatch.reasonCode).toBe('SCOPE_MISMATCH');

    const opened = await applyPresentationIntent(
      {
        protocol: 'muse.presentation-intent/v1',
        pluginId: 'muse.appflowy.workspace',
        scopeRef: 'workspace.ws-1',
        intentType: 'surface.open',
        intentSchemaDigest: 'sha256:c6337b4511a9ef40dbae92c23784cec068a2ff355ac6bb8e95141328c34905f0',
        intentRef: 'intent.3',
        requestedAt: 1,
        expiresAt: 100,
        payload: { viewId: 'view-1', workspaceId: 'ws-1' },
      },
      { workspaceId: 'ws-1', toView, now: 10 }
    );
    expect(opened.status).toBe('applied');
    expect(toView).toHaveBeenCalledWith('view-1', undefined);
  });

  it('parses intent.dispatch from the DSH frame source only', () => {
    expect(
      parseIntentDispatch({
        source: 'muse.dsh-web',
        type: 'intent.dispatch',
        intent: {
          protocol: 'muse.presentation-intent/v1',
          pluginId: 'muse.appflowy.workspace',
          scopeRef: 'workspace.1',
          intentType: 'surface.revealRange',
          intentSchemaDigest: 'sha256:' + 'b'.repeat(64),
          intentRef: 'intent.4',
          requestedAt: 1,
          expiresAt: 99,
          payload: { viewId: 'v', blockId: 'b1' },
        },
      })?.intent.intentType
    ).toBe('surface.revealRange');
    expect(
      parseIntentDispatch({
        source: 'muse.appflowy-web',
        type: 'intent.dispatch',
        intent: { protocol: 'muse.presentation-intent/v1', intentType: 'surface.open', intentRef: 'x' },
      })
    ).toBeNull();
  });
});
