import { SubprocessRuntime } from '@deepseek-ai/dsh-subprocess';
import { resolveControl } from './control.js';

function withoutSignal(spec) {
  const { signal: _signal, ...wire } = spec;
  return wire;
}

function validateHandle(handle, terminal = false) {
  const required = terminal
    ? ['done', 'write', 'resize', 'inspectForeground', 'inspectActivity', 'signalForeground', 'terminate']
    : ['done', 'terminate', 'waitForExit'];
  if (!handle || typeof handle !== 'object' || required.some((key) => handle[key] === undefined)) {
    throw new Error(`remote transport returned an invalid ${terminal ? 'terminal' : 'subprocess'} handle`);
  }
  return handle;
}

export class RemoteSubprocessRuntime extends SubprocessRuntime {
  constructor(ctx, config) {
    super(ctx);
    this.control = resolveControl(config);
    this.live = new Set();
    ctx.effect(() => async () => {
      const handles = [...this.live];
      await Promise.allSettled(handles.map(async (handle) => {
        await handle.terminate();
        await handle.done.catch(() => {});
      }));
      this.live.clear();
    }, 'remote subprocess teardown');
  }

  resolveExecutable(command, env, signal) {
    return this.control.call('subprocess.resolveExecutable', { command, env }, signal);
  }

  terminalEnvironment(signal) {
    return this.control.call('subprocess.terminalEnvironment', {}, signal);
  }

  track(handle) {
    this.live.add(handle);
    Promise.resolve(handle.done).finally(() => this.live.delete(handle)).catch(() => {});
    return handle;
  }

  spawn(spec) {
    spec.signal?.throwIfAborted();
    const handle = validateHandle(this.control.openProcess('subprocess.spawn', { spec: withoutSignal(spec) }));
    if (spec.signal) spec.signal.addEventListener('abort', () => handle.terminate(), { once: true });
    return this.track(handle);
  }

  async spawnTerminal(spec) {
    spec.signal?.throwIfAborted();
    const handle = validateHandle(this.control.openProcess('subprocess.spawnTerminal', { spec: withoutSignal(spec) }), true);
    if (spec.signal) spec.signal.addEventListener('abort', () => { void handle.terminate(); }, { once: true });
    return this.track(handle);
  }
}

export default RemoteSubprocessRuntime;
