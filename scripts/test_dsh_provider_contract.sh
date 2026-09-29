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
trap 'if [[ -n "$temporary_closure" ]]; then rm -rf "$temporary_closure"; fi' EXIT

OPENMUSE_DSH_CLOSURE="$dsh_closure" node scripts/dsh_provider_contract_tck.mjs
