# Pinned third-party build inputs

These files are source inputs of **this** repository. A macOS build does not
read another product's checkout.

| Component | Pinned input | License / integrity |
|---|---|---|
| Helix 25.07.1 (`079a789e`) with OpenMuse nonmodal prototype | `helix/` source, `plugins/helix/assets/engines/helix/hx` and `runtime/` | MPL-2.0 source license in `helix/LICENSE`; Universal `hx` SHA-256 `cb81c9fba915c66f1d2c5b42debb7585eaffa64d74c3f9d2f06570c0781acae8` (`openmuse-nonmodal.3`) |
| DeepSeek Harness 0.1.7-rc.1 | `dsh/package.json`, `dsh/package-lock.json` | Official npm `next` release, pinned by exact version and registry integrity; MIT (`dsh/LICENSE`) |
| dsh-model-capabilities 0.5.0-openmuse.3 | `dsh/plugins/dsh-model-capabilities/` and its local tarball | MIT LICENSE retained; profile-settings adaptation documented in `README.openmuse.md` |
| openmuse-dsh-bridge 0.1.5 | `dsh/plugins/openmuse-dsh-bridge/` and its local tarball | First-party AGPL-3.0; token-gated local Workspace registry, active Mount, Host file-open, selection-copy and path menu bridge |
| Node.js 22.19.0 | `node/v22.19.0/node-v22.19.0-darwin-{arm64,x64}.tar.gz` | Official `SHASUMS256.txt` retained here; Node's bundled `LICENSE` is extracted into the app |

The retained DSH 0.1.0-rc.7 tarballs were assembled from source revision
`99f6f02fecdb7dff40c3fbc9470f5907c29f74ca` but are no longer build
inputs. The release script builds the current closure from the exact public
0.1.7-rc.1 npm version plus the repository-local model-plugin tarball. Every
registry package is locked by integrity. A fresh machine needs npm registry
access, but no other source checkout. Generated `node_modules`, Flutter
outputs and Universal Node remain under ignored `target/` or app build paths.

`scripts/build_helix_fork.sh` rebuilds both macOS architectures from the
repository-local Helix source without fetching grammars. The `standard-nonmodal`
profile is still an experimental engine milestone and disabled in release UI;
see `docs/HELIX-NONMODAL-EDITOR-DESIGN.zh-CN.md` for outstanding gates.

The macOS archive is ad-hoc signed for testing. Product release still requires
notarization, complete third-party notices/SBOM, and separate Windows checks.
