#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

dsh_closure="${OPENMUSE_DSH_CLOSURE:-$repo_root/target/dsh-closure}"
temporary_closure=""
if [[ ! -f "$dsh_closure/node_modules/@deepseek-ai/dsh/package.json" ]]; then
  temporary_closure="$(mktemp -d)"
  dsh_closure="$temporary_closure/closure"
  python3 scripts/build_dsh_closure.py --out "$dsh_closure"
fi

test_root="$(mktemp -d)"
cleanup() {
  rm -rf "$test_root"
  if [[ -n "$temporary_closure" ]]; then rm -rf "$temporary_closure"; fi
}
trap cleanup EXIT

mkdir -p "$test_root/node_modules/@openmuse"
ln -s "$dsh_closure/node_modules/@deepseek-ai" "$test_root/node_modules/@deepseek-ai"
ln -s "$repo_root/third_party/dsh/plugins/openmuse-dsh-workspace-runtime" \
  "$test_root/node_modules/@openmuse/dsh-workspace-runtime"

node --preserve-symlinks --preserve-symlinks-main --test \
  "$test_root/node_modules/@openmuse/dsh-workspace-runtime/test/provider.test.mjs"
