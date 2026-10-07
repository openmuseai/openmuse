#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
host_root="$repo_root/app/openmuse_host"
dart_bin="${DART_BIN:-dart}"
python_bin="${PYTHON_BIN:-python3}"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/openmuse-cli-e2e.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT

"$python_bin" -c 'import sys; assert sys.version_info >= (3, 10)'

cd "$host_root"
"$dart_bin" build cli -t bin/openmuse.dart -o "$test_root/build"
cli="$test_root/build/bundle/bin/openmuse"
cd "$repo_root/plugins/easel"
"$dart_bin" run bin/pack.dart --easel "$repo_root/third_party/Easel" --output "$test_root/dist"
"$python_bin" - "$test_root"/dist/com.openmuse.easel-*.omplugin <<'PY'
import sys, zipfile
assert len(sys.argv) == 2
with zipfile.ZipFile(sys.argv[1]) as package:
    names = package.namelist()
    assert not any('openclaw' in name or 'easel-web' in name for name in names)
PY
OPENMUSE_PYTHON="$python_bin" "$cli" plugin install \
  --catalog "$test_root/dist/catalog.json" \
  --plugin com.openmuse.easel \
  --workspace "$test_root/workspace" \
  --data-dir "$test_root/data"
"$cli" list --data-dir "$test_root/data" > "$test_root/plugins.txt"
"$cli" plugin commands --data-dir "$test_root/data" > "$test_root/commands.txt"
"$cli" commands --json --data-dir "$test_root/data" > "$test_root/commands.json"
rg -q 'com.openmuse.easel' "$test_root/plugins.txt"
rg -q 'com.openmuse.cli' "$test_root/plugins.txt"
rg -q 'easel/douyin/plan' "$test_root/commands.txt"
rg -q 'easel/douyin/selftest' "$test_root/commands.txt"
rg -q 'easel/video/process' "$test_root/commands.json"
"$cli" easel douyin check --data-dir "$test_root/data" > "$test_root/check.txt"
rg -q 'chromium 内核' "$test_root/check.txt"
ffmpeg -hide_banner -loglevel error -y -f lavfi -i 'testsrc2=size=640x360:rate=30:duration=2' -f lavfi -i 'sine=frequency=440:duration=2' -c:v libx264 -c:a aac "$test_root/input.mp4"
"$cli" easel video process --input "$test_root/input.mp4" --output "$test_root/processed.mp4" --data-dir "$test_root/data" > "$test_root/process.txt" 2> "$test_root/process.err"
rg -q '"stage": "ready"' "$test_root/process.txt"
test -s "$test_root/processed.cover.png"
ffprobe -v error -show_entries format=duration -of csv=p=0 "$test_root/processed.mp4" | awk '{ if ($1 < 3.5) exit 1 }'

"$cli" easel douyin plan \
  --title 'CLI 离线验收' --content '只生成步骤' \
  --data-dir "$test_root/data" > "$test_root/plan.txt"
rg -q '发布类型' "$test_root/plan.txt"
"$cli" easel douyin selftest \
  --data-dir "$test_root/data" > "$test_root/selftest.txt"
rg -q 'selftest 通过' "$test_root/selftest.txt"

if "$cli" easel douyin plan \
  --exec --data-dir "$test_root/data" > "$test_root/unsafe.txt" 2>&1; then
  echo 'unsafe --exec unexpectedly succeeded' >&2
  exit 1
fi
rg -q '不支持参数 --exec' "$test_root/unsafe.txt"

echo 'OpenMuse CLI / Easel E2E passed'
