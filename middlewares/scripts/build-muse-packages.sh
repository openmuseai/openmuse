#!/usr/bin/env bash
# Build Muse TS packages in dependency order, then symlink them into DSH.
#
# Usage:
#   middlewares/scripts/build-muse-packages.sh [--skip-tests]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

SKIP_TESTS=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-tests) SKIP_TESTS=true ;;
    -h|--help)
      sed -n '2,6p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

muse_export_toolchain
ROOT="$(muse_root)"
cd "$ROOT"

echo "==> Building Muse packages"
while IFS= read -r name; do
  dir="$(muse_packages_root)/$name"
  echo "---- $name"
  (cd "$dir" && pnpm build)
  if [[ "$SKIP_TESTS" == false && -f "$dir/package.json" ]]; then
    if grep -q '"test":' "$dir/package.json"; then
      (cd "$dir" && pnpm test)
    fi
  fi
done < <(muse_package_dirs)

echo "==> Linking @muse packages into DSH resolver paths"
DSH_HOME="${DSH_HOME:-$HOME/.dsh}"
muse_link_dsh_packages "$(muse_harness_dir)/node_modules/@muse"
muse_link_dsh_packages "$DSH_HOME/profiles/node_modules/@muse"
muse_link_dsh_packages "$DSH_HOME/profiles/web/node_modules/@muse"
echo "Muse packages ready."
