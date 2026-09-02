import { mkdirSync, watch, type FSWatcher } from "node:fs";
import { mkdir, readFile, realpath, writeFile } from "node:fs/promises";
import { homedir } from "node:os";
import { basename, dirname, join } from "node:path";
import type { Context } from "@deepseek-ai/cordis";

/** Display title used when the AppFlowy hint omits a name. */
export const APPFLOWY_WORKSPACE_TITLE = "AppFlowy";

/** Thrown when a caller tries to unregister an AppFlowy-bound DSH workspace. */
export class AppFlowyWorkspacePinnedError extends Error {
  constructor(readonly workspaceId: string) {
    super("AppFlowy workspace cannot be deleted");
    this.name = "AppFlowyWorkspacePinnedError";
  }
}

/** Hint written by the Flutter shell when the current AppFlowy workspace changes. */
export interface AppFlowyWorkspaceHint {
  readonly appflowyWorkspaceId: string;
  readonly title: string;
  readonly updatedAt?: number;
}

/** Minimal DSH workspaceRegistry surface used by the AppFlowy binder. */
export interface DshWorkspace {
  readonly id: string;
  readonly path: string;
  readonly title: string;
  setTitle?(title: string): Promise<void>;
}

export interface DshWorkspaceRegistry {
  create(path: string, title?: string): Promise<DshWorkspace>;
  resolveByPath(path: string): Promise<DshWorkspace | undefined>;
  get(id: string): DshWorkspace | undefined;
  delete(id: string): Promise<boolean>;
  list(): DshWorkspace[];
  insertBefore?(id: string, beforeId?: string): Promise<readonly string[]>;
}

type HostContext = Context & { workspaceRegistry: DshWorkspaceRegistry };

const pinnedByRegistry = new WeakMap<DshWorkspaceRegistry, Set<string>>();

export const dshHomeDir = (): string => {
  const fromEnv = process.env.DSH_HOME?.trim();
  return fromEnv && fromEnv.length > 0 ? fromEnv : join(homedir(), ".dsh");
};

/** Root of per-AppFlowy-workspace DSH cwd directories. */
export const appFlowyWorkspacesRoot = (): string => {
  const fromEnv = process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT?.trim();
  if (fromEnv && fromEnv.length > 0) return fromEnv;
  return join(dshHomeDir(), "appflowy-workspaces");
};

export const hintFilePath = (): string => {
  const fromEnv = process.env.MUSE_APPFLOWY_WORKSPACE_HINT?.trim();
  if (fromEnv && fromEnv.length > 0) return fromEnv;
  return join(dshHomeDir(), "bindings", "current-appflowy-workspace.json");
};

/** Directory used as the DSH cwd for a single-workspace override (tests / legacy). */
export const appFlowyWorkspaceDir = (): string =>
  process.env.MUSE_APPFLOWY_DSH_WORKSPACE
  ?? join(appFlowyWorkspacesRoot(), "default");

export const appFlowyWorkspaceTitle = (): string => {
  const fromEnv = process.env.MUSE_APPFLOWY_WORKSPACE_TITLE?.trim();
  return fromEnv && fromEnv.length > 0 ? fromEnv : APPFLOWY_WORKSPACE_TITLE;
};

export const sanitizeWorkspaceId = (id: string): string => {
  const cleaned = id.replace(/[^A-Za-z0-9._-]+/g, "_").replace(/^_+|_+$/g, "");
  return cleaned.length > 0 ? cleaned.slice(0, 128) : "workspace";
};

export const workspaceDirForId = (appflowyWorkspaceId: string): string =>
  join(appFlowyWorkspacesRoot(), sanitizeWorkspaceId(appflowyWorkspaceId));

export const parseHint = (raw: string): AppFlowyWorkspaceHint | undefined => {
  let data: unknown;
  try {
    data = JSON.parse(raw) as unknown;
  } catch {
    return undefined;
  }
  if (data === null || typeof data !== "object") return undefined;
  const record = data as Record<string, unknown>;
  const id = typeof record.appflowyWorkspaceId === "string"
    ? record.appflowyWorkspaceId.trim()
    : "";
  if (id.length === 0) return undefined;
  const title = typeof record.title === "string" && record.title.trim().length > 0
    ? record.title.trim()
    : APPFLOWY_WORKSPACE_TITLE;
  if (typeof record.updatedAt === "number") {
    return { appflowyWorkspaceId: id, title, updatedAt: record.updatedAt };
  }
  return { appflowyWorkspaceId: id, title };
};

const pinPath = (registry: DshWorkspaceRegistry, canonical: string): void => {
  let pinned = pinnedByRegistry.get(registry);
  if (pinned === undefined) {
    pinned = new Set<string>();
    pinnedByRegistry.set(registry, pinned);
    const originalDelete = registry.delete.bind(registry);
    registry.delete = async (id: string): Promise<boolean> => {
      const workspace = registry.get(id);
      if (workspace !== undefined && pinned!.has(workspace.path)) {
        throw new AppFlowyWorkspacePinnedError(workspace.id);
      }
      return originalDelete(id);
    };
  }
  pinned.add(canonical);
};

const writeReadme = async (canonical: string, title: string, appflowyWorkspaceId?: string): Promise<void> => {
  await writeFile(
    join(canonical, "README.md"),
    [
      `# ${title}`,
      "",
      "This is the DSH working directory bound to an AppFlowy workspace.",
      ...(appflowyWorkspaceId === undefined ? [] : [`AppFlowy workspace id: ${appflowyWorkspaceId}`]),
      "AppFlowy pages are not files in this folder. glob/ls will only see README.md.",
      "List pages with muse_workspace_list_views (Cloud folder collab).",
      "Newly created or imported pages appear on the next list call.",
      "Documents are read and written through Muse tools, not as files here.",
      ""
    ].join("\n")
  );
};

const prepend = async (registry: DshWorkspaceRegistry, workspace: DshWorkspace): Promise<void> => {
  const insertBefore = registry.insertBefore?.bind(registry);
  if (insertBefore === undefined) return;
  const first = registry.list()[0];
  if (first === undefined || first.id === workspace.id) return;
  await insertBefore(workspace.id, first.id);
};

/**
 * Ensure a DSH workspace exists at `directory`, pin it against delete, and
 * optionally rename / prepend it. AppFlowy documents stay on Muse tools.
 */
export const bindAppFlowyWorkspaceAt = async (
  registry: DshWorkspaceRegistry,
  input: { directory: string; title: string; appflowyWorkspaceId?: string }
): Promise<DshWorkspace> => {
  await mkdir(input.directory, { recursive: true });
  const canonical = await realpath(input.directory);
  await writeReadme(canonical, input.title, input.appflowyWorkspaceId);
  pinPath(registry, canonical);

  const existing = await registry.resolveByPath(canonical);
  if (existing !== undefined) {
    if (existing.title !== input.title && existing.setTitle !== undefined) {
      await existing.setTitle(input.title);
    }
    await prepend(registry, existing);
    return existing;
  }
  const created = await registry.create(canonical, input.title);
  pinPath(registry, created.path);
  await prepend(registry, created);
  return created;
};

/**
 * Ensure the AppFlowy-bound DSH workspace exists and cannot be deleted.
 * Uses `MUSE_APPFLOWY_DSH_WORKSPACE` when set (tests); otherwise the default cwd.
 */
export const bindAppFlowyWorkspace = async (
  registry: DshWorkspaceRegistry
): Promise<DshWorkspace> =>
  bindAppFlowyWorkspaceAt(registry, {
    directory: appFlowyWorkspaceDir(),
    title: appFlowyWorkspaceTitle()
  });

let lastBoundHint: AppFlowyWorkspaceHint | undefined;

/** Last AppFlowy workspace successfully pinned on this Host (Web bind or hint file). */
export const getLastWorkspaceHint = (): AppFlowyWorkspaceHint | undefined => lastBoundHint;

/** Test helper: drop the in-memory bind so list APIs see NO_WORKSPACE. */
export const resetLastWorkspaceHint = (): void => {
  lastBoundHint = undefined;
};

const persistHint = async (hint: AppFlowyWorkspaceHint): Promise<void> => {
  const path = hintFilePath();
  const body: Record<string, unknown> = {
    appflowyWorkspaceId: hint.appflowyWorkspaceId,
    title: hint.title
  };
  if (hint.updatedAt !== undefined) body.updatedAt = hint.updatedAt;
  const next = JSON.stringify(body);
  try {
    if (await readFile(path, "utf8") === next) return;
  } catch {
    /* missing or unreadable — write */
  }
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, next);
};

export const applyWorkspaceHint = async (
  registry: DshWorkspaceRegistry,
  hint: AppFlowyWorkspaceHint
): Promise<DshWorkspace> => {
  const bound = await bindAppFlowyWorkspaceAt(registry, {
    directory: workspaceDirForId(hint.appflowyWorkspaceId),
    title: hint.title,
    appflowyWorkspaceId: hint.appflowyWorkspaceId
  });
    lastBoundHint = hint;
    try {
      await persistHint(hint);
    } catch {
      /* bind already succeeded; hint file is best-effort for Host restart */
    }
    return bound;
};

export const applyHintFile = async (
  registry: DshWorkspaceRegistry
): Promise<DshWorkspace | undefined> => {
  let raw: string;
  try {
    raw = await readFile(hintFilePath(), "utf8");
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
  const hint = parseHint(raw);
  if (hint === undefined) return undefined;
  return applyWorkspaceHint(registry, hint);
};

export const watchAppFlowyWorkspaceHint = (
  registry: DshWorkspaceRegistry,
  onError: (error: unknown) => void = () => undefined
): (() => void) => {
  const hint = hintFilePath();
  mkdirSync(dirname(hint), { recursive: true });
  let timer: ReturnType<typeof setTimeout> | undefined;
  const kick = (): void => {
    if (timer !== undefined) clearTimeout(timer);
    timer = setTimeout(() => {
      void applyHintFile(registry).catch(onError);
    }, 50);
  };
  const watcher: FSWatcher = watch(dirname(hint), { persistent: false }, (_event, filename) => {
    if (filename !== null && filename !== basename(hint)) return;
    kick();
  });
  return () => {
    if (timer !== undefined) clearTimeout(timer);
    watcher.close();
  };
};

export const name = "@muse/plugin-appflowy-workspace";
export const inject = ["workspaceRegistry"];

/**
 * Pin a DSH cwd at Host boot. Hint file (Desktop / previous Web bind) wins;
 * otherwise the default AppFlowy directory so the Web Client is never empty.
 */
export const bindHostWorkspace = async (
  registry: DshWorkspaceRegistry
): Promise<DshWorkspace> => {
  const fromHint = await applyHintFile(registry);
  if (fromHint !== undefined) return fromHint;
  return bindAppFlowyWorkspace(registry);
};

export const apply = async (ctx: Context): Promise<void> => {
  const registry = (ctx as HostContext).workspaceRegistry;
  if (registry === undefined) {
    throw new Error("DSH workspaceRegistry is required to bind the AppFlowy workspace");
  }
  await bindHostWorkspace(registry);
  const stop = watchAppFlowyWorkspaceHint(registry, error => {
    console.error("[muse-appflowy-workspace]", error);
  });
  ctx.effect(() => stop, "muse.appflowy.workspaceHintWatch");
};
