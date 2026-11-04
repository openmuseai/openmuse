#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
host_dir="$repo_root/app/openmuse_host"
dsh_cli="${OPENMUSE_DSH_CLI:-$repo_root/target/dsh-closure/node_modules/@deepseek-ai/dsh/lib/bin.js}"
node_bin="${OPENMUSE_MARKET_NODE:-node}"
node_abs="$(command -v "$node_bin")"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/openmuse-archify-market.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT

if [[ ! -f "$dsh_cli" ]]; then
  echo "DSH closure is missing: $dsh_cli" >&2
  exit 1
fi

cd "$host_dir"
common=(--profile market-e2e --dsh-home "$test_root/dsh" --dsh-cli "$dsh_cli" --node "$node_bin" --json)
dart run bin/openmuse.dart market list --json > "$test_root/catalog.json"

# Reproduce the GUI failure on a profile initialized without pnpm. The
# installer must recover this same profile after supplying its bundled pnpm.
set +e
DSH_HOME="$test_root/dsh" PATH="/usr/bin:/bin" \
  "$node_abs" "$dsh_cli" plugin --profile market-e2e add \
  @tt-a1i/archify-dsh@1.0.0 --save-exact \
  > "$test_root/missing-pnpm.log" 2>&1
missing_pnpm_status=$?
set -e
if [[ "$missing_pnpm_status" -ne 127 ]]; then
  echo "expected DSH missing-pnpm exit 127, got $missing_pnpm_status" >&2
  cat "$test_root/missing-pnpm.log" >&2
  exit 1
fi

dart run bin/openmuse.dart market plan archify "${common[@]}" > "$test_root/plan.json"
dart run bin/openmuse.dart market install archify "${common[@]}" > "$test_root/installed.json"
dart run bin/openmuse.dart market install archify "${common[@]}" > "$test_root/repeated.json"

DSH_HOME="$test_root/dsh" "$node_bin" "$dsh_cli" market-e2e --dump-config > "$test_root/composed.yml"

python3 - "$test_root" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
catalog = json.loads((root / 'catalog.json').read_text())
plan = json.loads((root / 'plan.json').read_text())
installed = json.loads((root / 'installed.json').read_text())
repeated = json.loads((root / 'repeated.json').read_text())
profile = json.loads((root / 'dsh/profiles/market-e2e/package.json').read_text())
assert catalog['schema'] == 'openmuse.market/v1'
assert any(p['id'] == 'archify' and p['ecosystem'] == 'dsh' for p in catalog['plugins'])
assert plan['action'] == 'install' and plan['installedVersion'] is None
assert installed['action'] == 'installed' and installed['installedVersion'] == '1.0.0'
assert repeated['action'] == 'already-installed'
assert profile['dependencies']['@tt-a1i/archify-dsh'] == '1.0.0'
assert '@tt-a1i/archify-dsh' in profile['dsh']['profile']['bundles']
assert 'archify-skill-filesystem' in (root / 'composed.yml').read_text()
print('Archify market E2E passed: failed pnpm 127 → plan → install → idempotent reinstall → DSH composition')
PY
