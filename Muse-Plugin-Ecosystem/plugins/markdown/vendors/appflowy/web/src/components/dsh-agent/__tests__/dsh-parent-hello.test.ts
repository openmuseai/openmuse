import { describe, expect, it } from '@jest/globals';
import { parentHelloMessage } from '@/components/dsh-agent/dsh-origin';

describe('parent-hello token lane', () => {
  it('omits deviceToken when none is provided', () => {
    expect(parentHelloMessage('dark', 'en')).toEqual({
      source: 'muse.appflowy-web',
      type: 'parent-hello',
      theme: 'dark',
      locale: 'en',
    });
  });

  it('includes a short-lived device token without model keys', () => {
    const msg = parentHelloMessage('dark', 'en', { token: 'aaa.bbb', kid: 'v1', expiresAt: 1, deviceId: 'web.1' });
    expect(msg.deviceToken).toBe('aaa.bbb');
    expect(msg.deviceId).toBe('web.1');
    expect(JSON.stringify(msg)).not.toContain('sk-');
    expect(JSON.stringify(msg)).not.toContain('api_key');
  });

  it('can bind a workspace without embedding tokens in the workspace fields', () => {
    const msg = parentHelloMessage(undefined, undefined, undefined, {
      workspaceRef: 'ws-9',
      workspaceTitle: 'Team',
    });
    expect(msg.workspaceRef).toBe('ws-9');
    expect(msg.deviceToken).toBeUndefined();
  });
});
