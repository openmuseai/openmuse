# Shared helpers for Muse macOS AppFlowy + DSH local builds.
# Sourced by scripts in ../ ; not meant to be executed directly.

_MUSE_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

muse_root() {
  if [[ -n "${MUSE_ROOT:-}" ]]; then
    printf '%s\n' "$MUSE_ROOT"
    return
  fi
  local dir cand
  dir="$(cd "${_MUSE_LIB_DIR}/../.." && pwd -P)"
  if [[ -d "${dir}/middlewares/dsh" && -d "${dir}/frontend/client" ]]; then
    printf '%s\n' "$dir"
    return
  fi
  cand="${_MUSE_LIB_DIR}"
  while [[ "$cand" != "/" ]]; do
    if [[ -d "${cand}/middlewares/dsh" && -d "${cand}/frontend/client" ]]; then
      printf '%s\n' "$cand"
      return
    fi
    cand="$(cd "${cand}/.." && pwd -P)"
  done
  echo "cannot find Muse repo root from ${_MUSE_LIB_DIR}" >&2
  return 1
}

# shellcheck source=muse-paths.sh
source "${_MUSE_LIB_DIR}/muse-paths.sh"
eval "$(python3 "${_MUSE_LIB_DIR}/brand_config.py" --export-env)"

muse_macos_profile() {
  case "$(uname -m)" in
    arm64) printf '%s\n' "development-mac-arm64" ;;
    x86_64) printf '%s\n' "development-mac-x86_64" ;;
    *)
      echo "unsupported macOS arch: $(uname -m)" >&2
      return 1
      ;;
  esac
}

muse_flutter_home() {
  if [[ -n "${FLUTTER_HOME:-}" ]]; then
    printf '%s\n' "$FLUTTER_HOME"
    return
  fi
  if [[ -x "${HOME}/sdks/flutters/flutter/bin/flutter" ]]; then
    printf '%s\n' "${HOME}/sdks/flutters/flutter"
    return
  fi
  local resolved
  resolved="$(command -v flutter 2>/dev/null || true)"
  if [[ -n "$resolved" ]]; then
    printf '%s\n' "$(cd "$(dirname "$resolved")/.." && pwd)"
    return
  fi
  echo "Flutter SDK not found. Set FLUTTER_HOME to the 3.27.x SDK." >&2
  return 1
}

muse_export_toolchain() {
  local root flutter_home
  root="$(muse_root)"
  flutter_home="$(muse_flutter_home)"
  export MUSE_ROOT="$root"
  export FLUTTER_HOME="$flutter_home"
  export PATH="${HOME}/.pub-cache/bin:${flutter_home}/bin:${PATH}"
}

# macOS GUI apps (Finder → .app) inherit only /usr/bin:/bin:/usr/sbin:/sbin,
# which hides nvm-managed node/pnpm. Resolve them so the DSH sidecar can boot
# regardless of how AppFlowy was launched.
muse_ensure_node() {
  if command -v node >/dev/null 2>&1 && command -v pnpm >/dev/null 2>&1; then
    return 0
  fi
  local candidate
  for candidate in \
    "${HOME}/.volta/bin" \
    "${HOME}/.local/share/pnpm" \
    "${HOME}/Library/pnpm" \
    "${HOME}/.asdf/shims" \
    "/opt/homebrew/bin" \
    "/usr/local/bin"; do
    [[ -d "$candidate" ]] || continue
    export PATH="${candidate}:${PATH}"
  done
  if command -v node >/dev/null 2>&1 && command -v pnpm >/dev/null 2>&1; then
    return 0
  fi
  local best best_key best_version
  best=""
  best_key="000"
  local dir version key
  for dir in "${HOME}/.nvm/versions/node/"*/bin; do
    [[ -d "$dir" ]] || continue
    version="$(basename "$(dirname "$dir")")"
    version="${version#v}"
    key="$(awk -F. '{printf "%03d%03d%03d", $1, $2, $3}' <<<"$version")"
    if [[ "$key" > "$best_key" ]]; then
      best_key="$key"
      best="$dir"
    fi
  done
  if [[ -n "$best" ]]; then
    export PATH="${best}:${PATH}"
    if command -v node >/dev/null 2>&1 && command -v pnpm >/dev/null 2>&1; then
      return 0
    fi
  fi
  echo "node and pnpm are required to start the DSH sidecar; none found in nvm, Volta, asdf, Homebrew, or pnpm homes." >&2
  return 1
}

muse_require_flutter_327() {
  local version
  version="$(flutter --version 2>/dev/null | head -1 || true)"
  if [[ "$version" != *"3.27."* ]]; then
    echo "Local AppFlowy macOS debug builds need Flutter 3.27.x; got: ${version:-unknown}" >&2
    echo "Set FLUTTER_HOME to \$HOME/sdks/flutters/flutter (not the 3.44 SDK)." >&2
    return 1
  fi
  echo "Flutter: $version"
}

muse_package_dirs() {
  # Paths relative to middlewares/dsh/. Dependency order: contracts first, DSH bundle last.
  printf '%s\n' \
    core/protocol/host-bridge \
    core/contract-resource \
    core/contract-presentation \
    core/contract-engine-session \
    core/resource-host \
    core/dsh-resource-presentation \
    core/plugin-facets \
    core/plugin-kit \
    core/plugin-graph \
    core/context-broker \
    core/contract-document \
    plugins/appflowy-view-reference \
    plugins/appflowy-view-rename \
    plugins/dsh-resource-presentation-host \
    plugins/dsh-client-ui-resource-open \
    plugins/dsh-tool-resource-present \
    plugins/appflowy-markdown \
    plugins/appflowy-workspace \
    plugins/dsh-mobile-surface \
    plugins/dsh-mobile-input \
    plugins/dsh-appflowy
}

muse_link_dsh_packages() {
  local dest="$1"
  local pkg_root
  pkg_root="$(muse_packages_root)"
  mkdir -p "$dest"
  ln -sfn "$pkg_root/plugins/dsh-appflowy" "$dest/dsh-appflowy"
  ln -sfn "$pkg_root/core/protocol/host-bridge" "$dest/host-bridge"
  ln -sfn "$pkg_root/core/contract-resource" "$dest/contract-resource"
  ln -sfn "$pkg_root/core/contract-presentation" "$dest/contract-presentation"
  ln -sfn "$pkg_root/core/contract-engine-session" "$dest/contract-engine-session"
  ln -sfn "$pkg_root/core/resource-host" "$dest/resource-host"
  ln -sfn "$pkg_root/core/dsh-resource-presentation" "$dest/dsh-resource-presentation"
  ln -sfn "$pkg_root/core/plugin-kit" "$dest/plugin-kit"
  ln -sfn "$pkg_root/core/plugin-facets" "$dest/plugin-facets"
  ln -sfn "$pkg_root/core/plugin-graph" "$dest/plugin-graph"
  ln -sfn "$pkg_root/core/context-broker" "$dest/context-broker"
  ln -sfn "$pkg_root/core/contract-document" "$dest/contract-document"
  ln -sfn "$pkg_root/plugins/appflowy-markdown" "$dest/plugin-appflowy-markdown"
  ln -sfn "$pkg_root/plugins/appflowy-workspace" "$dest/plugin-appflowy-workspace"
  ln -sfn "$pkg_root/plugins/appflowy-view-reference" "$dest/plugin-appflowy-view-reference"
  ln -sfn "$pkg_root/plugins/appflowy-view-rename" "$dest/plugin-appflowy-view-rename"
  ln -sfn "$pkg_root/plugins/dsh-resource-presentation-host" "$dest/dsh-resource-presentation-host"
  ln -sfn "$pkg_root/plugins/dsh-client-ui-resource-open" "$dest/dsh-client-ui-resource-open"
  ln -sfn "$pkg_root/plugins/dsh-tool-resource-present" "$dest/dsh-tool-resource-present"
  ln -sfn "$pkg_root/plugins/dsh-mobile-surface" "$dest/dsh-mobile-surface"
  ln -sfn "$pkg_root/plugins/dsh-mobile-input" "$dest/dsh-mobile-input"
}

# Copy (not symlink) Muse packages so a packed .app does not depend on the
# source tree. Destination layout matches node_modules/@muse/<short-name>.
# Only runtime files: package.json, dist/, optional patch/README. Never copy
# rust/target or node_modules (those blew a 2.5GB host-bridge into the .app).
muse_copy_dsh_packages() {
  local dest="$1"
  local pkg_root src name pkg
  pkg_root="$(muse_packages_root)"
  mkdir -p "$dest"
  while IFS= read -r src; do
    case "$src" in
      plugins/dsh-appflowy) name="dsh-appflowy" ;;
      core/protocol/host-bridge) name="host-bridge" ;;
      core/contract-resource) name="contract-resource" ;;
      core/contract-presentation) name="contract-presentation" ;;
      core/contract-engine-session) name="contract-engine-session" ;;
      core/resource-host) name="resource-host" ;;
      core/dsh-resource-presentation) name="dsh-resource-presentation" ;;
      core/plugin-kit) name="plugin-kit" ;;
      core/plugin-facets) name="plugin-facets" ;;
      core/plugin-graph) name="plugin-graph" ;;
      core/context-broker) name="context-broker" ;;
      core/contract-document) name="contract-document" ;;
      plugins/appflowy-markdown) name="plugin-appflowy-markdown" ;;
      plugins/appflowy-workspace) name="plugin-appflowy-workspace" ;;
      plugins/appflowy-view-reference) name="plugin-appflowy-view-reference" ;;
      plugins/appflowy-view-rename) name="plugin-appflowy-view-rename" ;;
      plugins/dsh-resource-presentation-host) name="dsh-resource-presentation-host" ;;
      plugins/dsh-client-ui-resource-open) name="dsh-client-ui-resource-open" ;;
      plugins/dsh-tool-resource-present) name="dsh-tool-resource-present" ;;
      plugins/dsh-mobile-surface) name="dsh-mobile-surface" ;;
      plugins/dsh-mobile-input) name="dsh-mobile-input" ;;
      *) name="$(basename "$src")" ;;
    esac
    pkg="$pkg_root/$src"
    echo "    copy $src -> $dest/$name"
    rm -rf "$dest/$name"
    mkdir -p "$dest/$name"
    cp "$pkg/package.json" "$dest/$name/"
    if [[ -f "$pkg/cordis.patch.yml" ]]; then
      cp "$pkg/cordis.patch.yml" "$dest/$name/"
    fi
    if [[ -f "$pkg/README.md" ]]; then
      cp "$pkg/README.md" "$dest/$name/"
    fi
    if [[ ! -d "$pkg/dist" ]]; then
      echo "missing built dist for $src (run middlewares/scripts/build-muse-packages.sh)" >&2
      return 1
    fi
    rsync -a "$pkg/dist/" "$dest/$name/dist/"
  done < <(muse_package_dirs)
}

# After muse packages sit under <harness>/node_modules/@muse, give each one
# node_modules so ESM realpath resolution can see @muse/*, @deepseek-ai/*, ajv.
muse_wire_muse_node_modules() {
  local harness="$1"
  local root node
  root="$(muse_root)"
  node="$harness/../node/bin/node"
  if [[ ! -x "$node" ]]; then
    node="$(command -v node)"
  fi
  python3 "${root}/scripts/lib/wire-muse-node-modules.py" "$harness" "$root" "$node"
}

muse_node_version() {
  printf '%s\n' "22.19.0"
}

muse_node_dist_name() {
  local arch
  case "$(uname -m)" in
    arm64) arch="arm64" ;;
    x86_64) arch="x64" ;;
    *)
      echo "unsupported macOS arch for bundled Node: $(uname -m)" >&2
      return 1
      ;;
  esac
  printf '%s\n' "node-v$(muse_node_version)-darwin-${arch}"
}

# Download official Node into cache_dir and copy bin+lib into dest_dir (…/node).
muse_stage_node() {
  local dest="$1"
  local cache="${2:-}"
  local root version name tarball url
  root="$(muse_root)"
  version="$(muse_node_version)"
  name="$(muse_node_dist_name)"
  if [[ -z "$cache" ]]; then
    cache="$(muse_dist_dir)/cache"
  fi
  mkdir -p "$cache" "$dest"
  tarball="$cache/${name}.tar.gz"
  if [[ ! -f "$tarball" ]]; then
    url="https://nodejs.org/dist/v${version}/${name}.tar.gz"
    echo "==> Downloading $url"
    curl -fsSL "$url" -o "$tarball"
  fi
  local extract
  extract="$(mktemp -d "${TMPDIR:-/tmp}/muse-node.XXXXXX")"
  tar -xzf "$tarball" -C "$extract"
  rsync -a --delete "$extract/${name}/" "$dest/"
  rm -rf "$extract"
  if [[ ! -x "$dest/bin/node" ]]; then
    echo "bundled node missing at $dest/bin/node" >&2
    return 1
  fi
  echo "Staged Node $version at $dest/bin/node"
  # dshmarket installs community plugins with pnpm; GUI .app PATH is only
  # /usr/bin:/bin, so the bundled Node tree must expose corepack shims.
  if [[ -x "$dest/bin/corepack" ]]; then
    "$dest/bin/node" "$dest/bin/corepack" enable >/dev/null 2>&1 || true
  fi
}

# Ad-hoc sign nested frameworks then the bundle. Hardened Runtime on the
# main executable without disable-library-validation makes dyld reject
# CocoaPods frameworks after a download (App Translocation).
muse_codesign_app() {
  local app="$1"
  local exe="$app/Contents/MacOS/${BRAND_MACOS_APP}"
  local entitlements
  entitlements="$(muse_flutter_dir)/macos/Runner/Release.entitlements"
  if [[ ! -x "$exe" ]]; then
    echo "cannot codesign, missing $exe" >&2
    return 1
  fi
  local item
  if [[ -d "$app/Contents/Frameworks" ]]; then
    while IFS= read -r item; do
      codesign --force -s - --options runtime --timestamp=none "$item"
    done < <(
      find "$app/Contents/Frameworks" -name '*.dylib' 2>/dev/null
      find "$app/Contents/Frameworks" -name '*.xpc' 2>/dev/null
      find "$app/Contents/Frameworks" -name '*.app' 2>/dev/null
      find "$app/Contents/Frameworks" -maxdepth 1 -name '*.framework' 2>/dev/null
    )
  fi
  if [[ -f "$entitlements" ]]; then
    codesign --force -s - --options runtime --timestamp=none --entitlements "$entitlements" "$exe"
    codesign --force -s - --options runtime --timestamp=none --entitlements "$entitlements" "$app"
  else
    codesign --force -s - --options runtime --timestamp=none "$exe"
    codesign --force -s - --options runtime --timestamp=none "$app"
  fi
  codesign --verify --deep --strict "$app"
}

# npm package for https://github.com/dsh-market/dsh-market. Override with
# MUSE_DSHMARKET_SPEC=dshmarket@x.y.z. Never add this to DSH package.json.
muse_dshmarket_spec() {
  printf '%s\n' "${MUSE_DSHMARKET_SPEC:-dshmarket@1.31.1}"
}

muse_dshmarket_version() {
  local spec
  spec="$(muse_dshmarket_spec)"
  printf '%s\n' "${spec##*@}"
}

# Wire dshmarket's optional/required peers to the harness copies (relative
# links so a packed .app stays relocatable). ESM resolves from realpath.
muse_wire_dshmarket() {
  local harness="$1"
  local dest="$harness/node_modules/dshmarket"
  local scoped="$dest/node_modules/@deepseek-ai"
  if [[ ! -d "$dest" ]]; then
    echo "dshmarket missing at $dest" >&2
    return 1
  fi
  mkdir -p "$scoped"
  ln -sfn ../../../../vendor/cordis "$scoped/cordis"
  ln -sfn ../../../../vendor/schemastery "$scoped/schemastery"
  if [[ -d "$harness/packages/settings/settings" ]]; then
    ln -sfn ../../../../packages/settings/settings "$scoped/dsh-settings"
  fi
}

# Fetch dshmarket from npm into <harness>/node_modules/dshmarket.
# Install happens in /tmp so a parent pnpm-workspace.yaml cannot swallow it.
muse_stage_dshmarket() {
  local harness="$1"
  local root spec version dest cache tarball have extract
  root="$(muse_root)"
  spec="$(muse_dshmarket_spec)"
  version="$(muse_dshmarket_version)"
  dest="$harness/node_modules/dshmarket"
  cache="$(muse_dist_dir)/cache"
  if [[ -f "$dest/package.json" && -f "$dest/lib/index.js" ]]; then
    have="$(python3 -c "import json; print(json.load(open('$dest/package.json'))['version'])" 2>/dev/null || true)"
    if [[ "$have" == "$version" ]]; then
      echo "dshmarket $have already staged at $dest"
      muse_wire_dshmarket "$harness"
      return 0
    fi
  fi
  muse_ensure_node || return 1
  if ! command -v npm >/dev/null 2>&1; then
    echo "npm is required to stage $spec" >&2
    return 1
  fi
  mkdir -p "$cache" "$harness/node_modules"
  tarball="$cache/dshmarket-${version}.tgz"
  if [[ ! -f "$tarball" ]]; then
    echo "==> Fetching $spec from npm"
    (cd "$cache" && npm pack "$spec" >/dev/null)
  fi
  if [[ ! -f "$tarball" ]]; then
    echo "npm pack did not produce $tarball" >&2
    return 1
  fi
  echo "==> Staging $spec into $dest"
  extract="$(mktemp -d "${TMPDIR:-/tmp}/muse-dshmarket.XXXXXX")"
  tar -xzf "$tarball" -C "$extract" --strip-components=1
  (
    cd "$extract"
    npm install --omit=dev --omit=peer --ignore-scripts --no-package-lock --no-workspaces >/dev/null
  )
  rm -rf "$dest"
  mkdir -p "$dest"
  rsync -a "$extract/" "$dest/"
  rm -rf "$extract"
  if [[ ! -f "$dest/lib/index.js" ]]; then
    echo "staged dshmarket is missing lib/index.js" >&2
    return 1
  fi
  muse_wire_dshmarket "$harness"
  echo "Staged dshmarket $version at $dest"
}

# Loader baseUrl is the profile directory; ESM does not consult NODE_PATH.
muse_link_dshmarket() {
  local harness="$1"
  local dest="$2"
  if [[ ! -d "$harness/node_modules/dshmarket" ]]; then
    return 0
  fi
  mkdir -p "$dest"
  ln -sfn "$harness/node_modules/dshmarket" "$dest/dshmarket"
}
