import { SandboxProvider, SandboxUnavailableError } from '@deepseek-ai/dsh-sandbox';
import { resolveControl } from './control.js';

const REMOTE_RUNNER = '/.__openmuse__/sandbox-exec';

export class RemoteSandboxProvider extends SandboxProvider {
  constructor(ctx, config) {
    super(ctx);
    this.control = resolveControl(config);
  }

  async confine(argv, policy, signal) {
    if (!Array.isArray(argv) || argv.length === 0 || argv.some((part) => typeof part !== 'string')) {
      throw new SandboxUnavailableError(policy.mode, 'invalid argv');
    }
    try {
      const result = await this.control.call('sandbox.prepare', { argv, policy }, signal);
      if (!result || typeof result.launchToken !== 'string' || result.launchToken.length === 0
        || result.enforcement !== 'full') {
        throw new Error('runtime did not attest full enforcement');
      }
      return {
        argv: [REMOTE_RUNNER, result.launchToken, '--', ...argv],
        enforcement: 'full',
        denialSignatures: Array.isArray(result.denialSignatures) ? result.denialSignatures : ['OPENMUSE_SANDBOX_DENIED'],
        runnerFailureRules: [{
          allowedExitCodes: [125],
          fatalSignatures: ['OPENMUSE_REMOTE_RUNNER_FAILURE'],
        }],
      };
    } catch (error) {
      if (error instanceof SandboxUnavailableError) throw error;
      throw new SandboxUnavailableError(policy.mode, error instanceof Error ? error.message : String(error));
    }
  }
}

export { REMOTE_RUNNER };
export default RemoteSandboxProvider;
