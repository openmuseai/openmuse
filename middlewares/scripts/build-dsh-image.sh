#!/usr/bin/env bash
# Build a linux/amd64 Remote DSH image (Node official image + pnpm install).
# Do not copy macOS bundled Node into this image.
#
# Usage:
#   middlewares/scripts/build-dsh-image.sh [--load] [--tag muse-dsh:local] [--platform linux/amd64]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

LOAD=false
TAG="${MUSE_DSH_IMAGE:-muse-dsh:local}"
PLATFORM="${DEPLOY_PLATFORM:-linux/amd64}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --load) LOAD=true ;;
    --tag)
      TAG="${2:?}"
      shift
      ;;
    --platform)
      PLATFORM="${2:?}"
      shift
      ;;
    -h|--help)
      sed -n '2,8p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

ROOT="$(muse_root)"
HARNESS="$(muse_harness_dir)"
PATCH="$(muse_dsh_patch)"
CONTEXT="$(muse_dist_dir)/dsh-image-context"
DOCKERFILE="$(muse_dsh_deploy_dir)/Dockerfile"

if [[ ! -d "$HARNESS" ]]; then
  echo "missing DSH harness: $HARNESS" >&2
  exit 1
fi
if [[ ! -f "$PATCH" ]]; then
  echo "missing $PATCH" >&2
  exit 1
fi
if [[ ! -f "$HARNESS/pnpm-lock.yaml" ]]; then
  echo "missing $HARNESS/pnpm-lock.yaml" >&2
  exit 1
fi

echo "==> Preparing image context at $CONTEXT"
rm -rf "$CONTEXT"
mkdir -p "$CONTEXT/dsh" "$CONTEXT/packages" "$CONTEXT/dshmarket-optional" "$CONTEXT/dsh-model-capabilities-optional"

rsync -a \
  --exclude '.git/' \
  --exclude 'node_modules/' \
  --exclude 'website/' \
  --exclude 'python/' \
  --exclude 'coverage/' \
  --exclude '.turbo/' \
  --exclude '.DS_Store' \
  "$HARNESS/" "$CONTEXT/dsh/"

muse_copy_dsh_packages "$CONTEXT/packages"
cp "$PATCH" "$CONTEXT/patch.yml"
cp "$(muse_dsh_deploy_dir)/entrypoint.sh" "$CONTEXT/entrypoint.sh"
cp "$(muse_dsh_deploy_dir)/scripts/seed-instance-settings.sh" "$CONTEXT/seed-instance-settings.sh"
cp "$(muse_dsh_deploy_dir)/scripts/merge-default-settings.py" "$CONTEXT/merge-default-settings.py"
mkdir -p "$CONTEXT/defaults"
cp "$(muse_dsh_deploy_dir)/defaults/settings.yaml" "$CONTEXT/defaults/settings.yaml"
cp "$(muse_dsh_deploy_dir)/loopback-proxy.mjs" "$CONTEXT/loopback-proxy.mjs"
cp "$(muse_dsh_deploy_dir)/wire-harness-aliases.py" "$CONTEXT/wire-harness-aliases.py"
chmod +x "$CONTEXT/entrypoint.sh" "$CONTEXT/seed-instance-settings.sh"

if [[ -d "$HARNESS/node_modules/dshmarket" ]]; then
  echo "==> Copying staged dshmarket into image context"
  rsync -a "$HARNESS/node_modules/dshmarket/" "$CONTEXT/dshmarket-optional/"
else
  echo "==> dshmarket not in harness node_modules (optional); image will omit it"
  : > "$CONTEXT/dshmarket-optional/.keep"
fi

CAP_SRC="$(muse_packages_root)/plugins/dsh-model-capabilities"
if [[ -f "$CAP_SRC/lib/index.js" ]]; then
  echo "==> Copying dsh-model-capabilities into image context"
  rsync -a --exclude '.git/' "$CAP_SRC/" "$CONTEXT/dsh-model-capabilities-optional/"
else
  echo "==> dsh-model-capabilities missing; image will omit it"
  : > "$CONTEXT/dsh-model-capabilities-optional/.keep"
fi

echo "==> docker buildx build --platform $PLATFORM -t $TAG --load"
BUILD_ARGS=()
if [[ -n "${NPM_REGISTRY:-}" ]]; then
  echo "==> using NPM_REGISTRY (value not printed)"
  BUILD_ARGS+=(--build-arg "NPM_REGISTRY=${NPM_REGISTRY}")
fi
if [[ -n "${DSH_BASE_IMAGE:-}" ]]; then
  echo "==> using DSH_BASE_IMAGE ${DSH_BASE_IMAGE}"
  BUILD_ARGS+=(--build-arg "BASE_IMAGE=${DSH_BASE_IMAGE}")
fi
docker buildx build --platform "$PLATFORM" -t "$TAG" --load -f "$DOCKERFILE" ${BUILD_ARGS[@]+"${BUILD_ARGS[@]}"} "$CONTEXT"

if [[ "$LOAD" == true ]]; then
  echo "Image loaded locally as $TAG"
else
  echo "Built $TAG (use docker save in middlewares/dsh/deploy, or --load for local compose)"
fi
