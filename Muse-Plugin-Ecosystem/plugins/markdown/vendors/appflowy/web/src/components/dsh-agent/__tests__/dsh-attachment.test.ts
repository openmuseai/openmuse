import { readFileSync } from 'fs';
import { resolve } from 'path';
import {
  ATTACHMENT_ERROR_CODES,
  BIND_ACK_TIMEOUT_MS,
  attachmentError,
  createAttachment,
  isLiveGeneration,
  keepFrameOnFailed,
  placingCopy,
  queuedCopy,
  stageRequiresFrame,
} from '@/components/dsh-agent/dsh-attachment';

describe('dsh-attachment', () => {
  it('builds retryable identity errors', () => {
    const auth = attachmentError('ResolvingIdentity', 'NEED_AUTH');
    expect(auth.retryable).toBe(false);
    expect(auth.message).toMatch(/Sign in/);
    expect(attachmentError('ResolvingIdentity', 'NEED_WORKSPACE').retryable).toBe(false);
  });

  it('tracks generation liveness', () => {
    const att = createAttachment('att_test');
    expect(att.attachmentId).toBe('att_test');
    expect(isLiveGeneration({ ...att, generation: 3 }, 3)).toBe(true);
    expect(isLiveGeneration({ ...att, generation: 3 }, 2)).toBe(false);
  });

  it('copies queued and placing states', () => {
    expect(queuedCopy(2)).toMatch(/position 2/);
    expect(placingCopy()).toMatch(/Starting agent/);
  });

  it('requires a frame only after placement succeeds', () => {
    expect(stageRequiresFrame('Placing')).toBe(false);
    expect(stageRequiresFrame('Presenting')).toBe(true);
    expect(ATTACHMENT_ERROR_CODES).toContain('NEED_AUTH');
  });

  it('keeps the iframe on bind/bridge failures', () => {
    expect(keepFrameOnFailed('NO_DEVICE_TOKEN')).toBe(true);
    expect(keepFrameOnFailed('BRIDGE_SILENT')).toBe(true);
    expect(keepFrameOnFailed('SESSION_DENIED')).toBe(false);
    expect(BIND_ACK_TIMEOUT_MS).toBe(8000);
  });

  it('E2-T10 contract JSON keys are covered by the Host enum', () => {
    const raw = JSON.parse(
      readFileSync(
        resolve(
          process.cwd(),
          '../../middlewares/dsh/plugins/dsh-appflowy/contracts/dsh-attachment-error-codes.v1.json'
        ),
        'utf8'
      )
    ) as Record<string, Record<string, string>>;
    const keys = Object.values(raw).flatMap((group) => Object.keys(group));
    for (const key of keys) {
      expect(ATTACHMENT_ERROR_CODES).toContain(key);
    }
  });
});
