# Muse DSH AppFlowy Bundle

**Composition bundle, not a logical Plugin.** Workspace and markdown Domain live in `packages/plugins/appflowy-*`. `parent-bridge` is Web/Remote Tx only and does not register on Desktop. See [docs/ROLE.zh-CN.md](docs/ROLE.zh-CN.md) and [TECH.zh-CN.md](TECH.zh-CN.md).

This installable DSH bundle mounts the Muse Host Bridge and AppFlowy capability plugins. The default connector reads AppFlowy Core's private per-user launch descriptor and communicates with the Rust Host over an authenticated local UDS transport.

On startup it also registers a DSH workspace titled `AppFlowy` (cwd `~/.muse/dsh-workspaces/appflowy` unless `MUSE_APPFLOWY_DSH_WORKSPACE` is set). DSH's existing create / rename / delete remain available for every other workspace; the AppFlowy registration cannot be deleted. Documents still move through Muse tools, not as files in that directory.

## Development Fixture

`InProcessAppFlowyConnector` remains exported only for isolated Loader/model acceptance tests. It returns bounded fixture Markdown and is never selected by the production Cordis patch.
