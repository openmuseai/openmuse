#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
gate_tmp="$(mktemp -d "${TMPDIR:-/tmp}/openmuse-distribution-lock.XXXXXX")"
trap 'rm -rf "$gate_tmp"' EXIT

manifests=(
  --manifest "$repo_root/schemas/fixtures/plugin/v2/dsh-agent.json"
  --manifest "$repo_root/schemas/fixtures/plugin/v2/helix.json"
  --manifest "$repo_root/schemas/fixtures/plugin/v2/native-text-gate.json"
  --manifest "$repo_root/schemas/fixtures/plugin/v2/open-file-viewer.json"
)

cd "$repo_root"
python3 -m py_compile scripts/distribution_lock.py scripts/tests/test_distribution_lock.py
python3 -m unittest scripts.tests.test_distribution_lock -v

for run in first second; do
  python3 scripts/distribution_lock.py resolve \
    --capability distribution/targets/macos-aarch64.json \
    "${manifests[@]}" \
    --lock "$gate_tmp/macos-$run.lock.json" \
    --sbom "$gate_tmp/macos-$run.spdx.json" \
    --notices "$gate_tmp/macos-$run.NOTICES.md" \
    --source-date-epoch 0
done

cmp "$gate_tmp/macos-first.lock.json" "$gate_tmp/macos-second.lock.json"
cmp "$gate_tmp/macos-first.spdx.json" "$gate_tmp/macos-second.spdx.json"
cmp "$gate_tmp/macos-first.NOTICES.md" "$gate_tmp/macos-second.NOTICES.md"

for profile in android-aarch64 sandbox-linux-x86_64; do
  python3 scripts/distribution_lock.py resolve \
    --capability "distribution/targets/$profile.json" \
    "${manifests[@]}" \
    --lock "$gate_tmp/$profile.lock.json" \
    --sbom "$gate_tmp/$profile.spdx.json" \
    --notices "$gate_tmp/$profile.NOTICES.md" \
    --source-date-epoch 0
done
