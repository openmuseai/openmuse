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

if [ ! -f "$cli" ] || [ ! -x "$node_bin" ]; then
  echo "stage_dsh: closure or node missing, bundle left unchanged"
  exit 0
fi

mkdir -p "$dest/node/bin"
if [ ! -f "$dest/node_modules/@deepseek-ai/dsh/lib/bin.js" ]; then
  rm -rf "$dest/node_modules"
  ln -sfn "$closure/node_modules" "$dest/node_modules"
fi
if [ ! -x "$dest/node/bin/node" ]; then
  ln -sfn "$node_bin" "$dest/node/bin/node"
fi
echo "stage_dsh: $dest"
