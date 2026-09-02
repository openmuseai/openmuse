# Contributing to DSH Office

Thanks for contributing to **DSH Office** ([openmuseai/dsh-office](https://github.com/openmuseai/dsh-office)).

This project is derived from the AppFlowy ecosystem (AGPL-3.0). By submitting a change you agree to license it under **AGPL-3.0-only** (SPDX).

## Workflow

1. Fork and clone the repository.
2. Branch from `main`: `feat/…` or `fix/…`.
3. Before opening a PR:
   - Client: `cargo check` / `flutter analyze` in `frontend/client/frontend`.
   - Plugins: `pnpm check` in `middlewares/dsh`.
   - Do not add user-visible AppFlowy branding, appflowy.io URLs, or a login/account requirement.
4. Prefer [Conventional Commits](https://www.conventionalcommits.org/).

## Boundaries

- **Do not retarget upstream git dependencies** (`appflowy-editor`, `appflowy-board`, AppFlowy-plugins, AppFlowy-Collab, AppFlowy-Cloud, and similar). Keep their URLs and versions as-is.
- **Local-first**: no account system, no required login, no telemetry. New network identity features need an explicit PR discussion.
- Directory names such as `appflowy_flutter` and `flowy-*` are inherited internals; do not introduce them in user-facing copy.

## Security

Report vulnerabilities privately using [SECURITY.md](SECURITY.md). Do not file a public issue for an unfixed security bug.
