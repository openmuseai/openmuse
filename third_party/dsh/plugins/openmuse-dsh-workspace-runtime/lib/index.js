import { WorkspaceRuntimeControl } from './control.js';
import { RemoteFileSystem } from './fs.js';
import { RemoteSandboxProvider } from './sandbox.js';
import { RemoteSubprocessRuntime } from './subprocess.js';

export { CONTROL_SCHEMA, RemoteControlError, WorkspaceRuntimeControl, resolveControl, validateAttachment } from './control.js';
export { RemoteFileSystem } from './fs.js';
export { REMOTE_RUNNER, RemoteSandboxProvider } from './sandbox.js';
export { RemoteSubprocessRuntime } from './subprocess.js';

export const inject = [];

export function apply(ctx, config) {
  const control = new WorkspaceRuntimeControl(config);
  new RemoteFileSystem(ctx, { control, sandboxMode: config.sandboxMode });
  new RemoteSandboxProvider(ctx, { control });
  new RemoteSubprocessRuntime(ctx, { control });
}
