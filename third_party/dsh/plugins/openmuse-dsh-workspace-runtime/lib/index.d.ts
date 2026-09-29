import type { Context } from '@deepseek-ai/cordis';
import { FileSystem } from '@deepseek-ai/dsh-fs';
import { SandboxProvider, type SandboxMode } from '@deepseek-ai/dsh-sandbox';
import { SubprocessRuntime } from '@deepseek-ai/dsh-subprocess';

export interface ControlAttachment {
  tokenRef: string;
  runtimeRef: string;
  audience: string;
  generation: number;
  expiresAtMs: number;
}

export interface WorkspaceRuntimeTransport {
  call(envelope: unknown, options: { signal?: AbortSignal }): Promise<unknown>;
  stream(envelope: unknown, options: { signal?: AbortSignal }): AsyncIterable<string>;
  openProcess(envelope: unknown): unknown;
  watch?(envelope: unknown, changed: (error?: Error) => void, signal: AbortSignal): Promise<() => Promise<void>>;
}

export interface RuntimeProviderConfig {
  transport: WorkspaceRuntimeTransport;
  attachment: ControlAttachment;
  audience: string;
  now?: () => number;
  sandboxMode?: SandboxMode;
}

export declare class WorkspaceRuntimeControl {
  constructor(config: RuntimeProviderConfig);
  call(method: string, params?: object, signal?: AbortSignal): Promise<any>;
  stream(method: string, params?: object, signal?: AbortSignal): AsyncIterable<string>;
  openProcess(method: string, params?: object): any;
}
export declare class RemoteControlError extends Error { readonly code: string; }
export declare function validateAttachment(value: unknown, expectedAudience: string, nowMs?: number): Readonly<ControlAttachment>;
export declare class RemoteFileSystem extends FileSystem { constructor(ctx: Context, config: RuntimeProviderConfig | { control: WorkspaceRuntimeControl; sandboxMode?: SandboxMode }); }
export declare class RemoteSandboxProvider extends SandboxProvider { constructor(ctx: Context, config: RuntimeProviderConfig | { control: WorkspaceRuntimeControl }); }
export declare class RemoteSubprocessRuntime extends SubprocessRuntime { constructor(ctx: Context, config: RuntimeProviderConfig | { control: WorkspaceRuntimeControl }); }
export declare function apply(ctx: Context, config: RuntimeProviderConfig): void;
export declare const CONTROL_SCHEMA: 'openmuse.workspace-runtime.control@1';
export declare const REMOTE_RUNNER: '/.__openmuse__/sandbox-exec';
