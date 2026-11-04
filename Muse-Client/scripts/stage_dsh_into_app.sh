#!/bin/sh
# Link the local DSH closure into a macOS app bundle.
# flutter run does not run package_macos.sh, so a debug bundle otherwise
# has no Contents/Resources/openmuse/dsh and the panel stays at
# "DSH runtime 尚未安装。"
set -eu

app_path="${1:?usage: stage_dsh_into_app.sh OpenMuse.app}"
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
closure="$repo_root/target/dsh-closure"
node_bin="$repo_root/target/node-v22.19.0-universal/node"
cli="$closure/node_modules/@deepseek-ai/dsh/lib/bin.js"
dest="$app_path/Contents/Resources/openmuse/dsh"

if [ ! -x "$node_bin" ]; then
  echo "stage_dsh: closure or node missing, bundle left unchanged"
  exit 0
fi

if [ ! -f "$cli" ] || [ ! -f "$closure/node_modules/.bin/pnpm" ] ||
   ! cmp -s "$repo_root/third_party/dsh/package.json" "$closure/package.json" ||
   ! cmp -s "$repo_root/third_party/dsh/package-lock.json" "$closure/package-lock.json"; then
  stale_closure="$closure.stale.$$"
  if [ -d "$closure" ]; then mv "$closure" "$stale_closure"; fi
  if python3 "$repo_root/scripts/build_dsh_closure.py" --out "$closure"; then
    if [ -d "$stale_closure" ]; then rm -rf "$stale_closure"; fi
  else
    rm -rf "$closure"
    if [ -d "$stale_closure" ]; then mv "$stale_closure" "$closure"; fi
    exit 1
  fi
fi

mkdir -p "$dest/node/bin"
if [ ! -f "$dest/node_modules/@deepseek-ai/dsh/lib/bin.js" ] ||
   [ ! -f "$dest/node_modules/.bin/pnpm" ]; then
  rm -rf "$dest/node_modules"
  ln -sfn "$closure/node_modules" "$dest/node_modules"
fi
if [ ! -x "$dest/node/bin/node" ]; then
  ln -sfn "$node_bin" "$dest/node/bin/node"
fi
echo "stage_dsh: $dest"
