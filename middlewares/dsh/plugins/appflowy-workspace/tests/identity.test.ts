import { mkdtemp, mkdir, readFile, realpath, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import {
  AppFlowyWorkspacePinnedError,
  applyHintFile,
  applyWorkspaceHint,
  bindAppFlowyWorkspace,
  bindHostWorkspace,
  getLastWorkspaceHint,
  parseHint,
  sanitizeWorkspaceId,
  type DshWorkspace,
  type DshWorkspaceRegistry
} from "../src/identity.js";

const fakeRegistry = (): DshWorkspaceRegistry & { deleted: string[]; prepended: string[] } => {
  const items: Array<DshWorkspace & { title: string }> = [];
  const registry: DshWorkspaceRegistry & { deleted: string[]; prepended: string[] } = {
    deleted: [],
    prepended: [],
    async create(path, title) {
      const existing = items.find(item => item.path === path);
      if (existing !== undefined) return existing;
      const workspace: DshWorkspace & { title: string } = {
        id: `ws-${items.length + 1}`,
        path,
        title: title ?? "untitled",
        async setTitle(next) {
          workspace.title = next;
        }
      };
      items.unshift(workspace);
      return workspace;
    },
    async resolveByPath(path) {
      return items.find(item => item.path === path);
    },
    get(id) {
      return items.find(item => item.id === id);
    },
    async delete(id) {
      const index = items.findIndex(item => item.id === id);
      if (index < 0) return false;
      items.splice(index, 1);
      registry.deleted.push(id);
      return true;
    },
    list() {
      return [...items];
    },
    async insertBefore(id, beforeId) {
      const from = items.findIndex(item => item.id === id);
      if (from < 0) throw new Error(`unknown workspace ${id}`);
      const [item] = items.splice(from, 1);
      if (item === undefined) throw new Error(`unknown workspace ${id}`);
      if (beforeId === undefined) {
        items.push(item);
      } else {
        const to = items.findIndex(entry => entry.id === beforeId);
        if (to < 0) {
          items.splice(from, 0, item);
          throw new Error(`unknown anchor ${beforeId}`);
        }
        items.splice(to, 0, item);
      }
      registry.prepended.push(id);
      return items.map(entry => entry.id);
    }
  };
  return registry;
};

const envKeys = [
  "MUSE_APPFLOWY_DSH_WORKSPACE",
  "MUSE_APPFLOWY_WORKSPACE_TITLE",
  "MUSE_APPFLOWY_DSH_WORKSPACE_ROOT",
  "MUSE_APPFLOWY_WORKSPACE_HINT",
  "DSH_HOME"
] as const;

const snapshotEnv = (): Record<string, string | undefined> =>
  Object.fromEntries(envKeys.map(key => [key, process.env[key]]));

const restoreEnv = (snapshot: Record<string, string | undefined>): void => {
  for (const key of envKeys) {
    const value = snapshot[key];
    if (value === undefined) delete process.env[key];
    else process.env[key] = value;
  }
};

describe("AppFlowy DSH workspace binding", () => {
  const previous = snapshotEnv();
  afterEach(() => restoreEnv(previous));

  it("creates a pinned AppFlowy workspace and still allows deleting others", async () => {
    const directory = await mkdtemp(join(tmpdir(), "muse-appflowy-ws-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE = directory;
    process.env.MUSE_APPFLOWY_WORKSPACE_TITLE = "AppFlowy";
    const registry = fakeRegistry();
    const extra = await registry.create(`${await realpath(directory)}-scratch`, "scratch");
    const pinned = await bindAppFlowyWorkspace(registry);
    expect(pinned.title).toBe("AppFlowy");
    expect(pinned.path).toBe(await realpath(directory));
    await expect(registry.delete(pinned.id)).rejects.toBeInstanceOf(AppFlowyWorkspacePinnedError);
    expect(registry.list().some(item => item.id === pinned.id)).toBe(true);
    await expect(registry.delete(extra.id)).resolves.toBe(true);
    expect(registry.deleted).toEqual([extra.id]);
    expect(registry.list().map(item => item.id)).toEqual([pinned.id]);
  });

  it("reuses an existing registration at the AppFlowy path", async () => {
    const directory = await mkdtemp(join(tmpdir(), "muse-appflowy-ws-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE = directory;
    const registry = fakeRegistry();
    const first = await bindAppFlowyWorkspace(registry);
    const second = await bindAppFlowyWorkspace(registry);
    expect(second.id).toBe(first.id);
    expect(registry.list()).toHaveLength(1);
  });

  it("parses and sanitizes AppFlowy workspace hints", () => {
    expect(parseHint(`{"appflowyWorkspaceId":" ws-1 ","title":" Docs "}`)).toEqual({
      appflowyWorkspaceId: "ws-1",
      title: "Docs"
    });
    expect(parseHint("{}")).toBeUndefined();
    expect(parseHint("not-json")).toBeUndefined();
    expect(sanitizeWorkspaceId("a/b c")).toBe("a_b_c");
  });

  it("projects each AppFlowy workspace id onto its own pinned DSH cwd", async () => {
    const home = await mkdtemp(join(tmpdir(), "muse-appflowy-home-"));
    process.env.DSH_HOME = home;
    const root = await mkdtemp(join(tmpdir(), "muse-appflowy-root-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE;
    const registry = fakeRegistry();
    const first = await applyWorkspaceHint(registry, {
      appflowyWorkspaceId: "alpha",
      title: "Alpha"
    });
    const second = await applyWorkspaceHint(registry, {
      appflowyWorkspaceId: "beta",
      title: "Beta"
    });
    expect(first.path).toBe(await realpath(join(root, "alpha")));
    expect(second.path).toBe(await realpath(join(root, "beta")));
    expect(first.title).toBe("Alpha");
    expect(second.title).toBe("Beta");
    expect(registry.list()[0]?.id).toBe(second.id);
    await expect(registry.delete(first.id)).rejects.toBeInstanceOf(AppFlowyWorkspacePinnedError);
    await expect(registry.delete(second.id)).rejects.toBeInstanceOf(AppFlowyWorkspacePinnedError);
    const readme = await readFile(join(first.path, "README.md"), "utf8");
    expect(readme).toContain("Muse tools");
    expect(readme).toContain("muse_workspace_list_views");
  });

  it("renames and prepends when the same AppFlowy workspace is rebound", async () => {
    process.env.DSH_HOME = await mkdtemp(join(tmpdir(), "muse-appflowy-home-"));
    const root = await mkdtemp(join(tmpdir(), "muse-appflowy-rename-"));
    process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT = root;
    const registry = fakeRegistry();
    const original = await applyWorkspaceHint(registry, {
      appflowyWorkspaceId: "alpha",
      title: "Old"
    });
    await registry.create(join(root, "other"), "other");
    const updated = await applyWorkspaceHint(registry, {
      appflowyWorkspaceId: "alpha",
      title: "New"
    });
    expect(updated.id).toBe(original.id);
    expect(updated.title).toBe("New");
    expect(registry.list()[0]?.id).toBe(updated.id);
  });

  it("applies a Flutter hint file under DSH_HOME", async () => {
    const home = await mkdtemp(join(tmpdir(), "muse-dsh-home-"));
    process.env.DSH_HOME = home;
    delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE;
    delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    delete process.env.MUSE_APPFLOWY_WORKSPACE_HINT;
    const hintDir = join(home, "bindings");
    await mkdir(hintDir, { recursive: true });
    await writeFile(
      join(hintDir, "current-appflowy-workspace.json"),
      JSON.stringify({ appflowyWorkspaceId: "from-file", title: "From file" })
    );
    const registry = fakeRegistry();
    const bound = await applyHintFile(registry);
    expect(bound?.title).toBe("From file");
    expect(bound?.path).toBe(await realpath(join(home, "appflowy-workspaces", "from-file")));
    expect(getLastWorkspaceHint()?.appflowyWorkspaceId).toBe("from-file");
  });

  it("pins a default DSH cwd at boot when no hint file exists", async () => {
    const home = await mkdtemp(join(tmpdir(), "muse-dsh-empty-"));
    process.env.DSH_HOME = home;
    delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE;
    delete process.env.MUSE_APPFLOWY_DSH_WORKSPACE_ROOT;
    delete process.env.MUSE_APPFLOWY_WORKSPACE_HINT;
    const registry = fakeRegistry();
    const bound = await bindHostWorkspace(registry);
    expect(bound.title).toBe("AppFlowy");
    expect(bound.path).toBe(await realpath(join(home, "appflowy-workspaces", "default")));
    expect(registry.list()).toHaveLength(1);
  });
});
