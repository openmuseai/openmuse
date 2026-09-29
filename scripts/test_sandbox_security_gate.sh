#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo +1.85.1 fmt --package openmuse-sandbox-security -- --check
cargo +1.85.1 test -p openmuse-sandbox-security
cargo +1.85.1 clippy -p openmuse-sandbox-security --all-targets -- -D warnings

./scripts/test_workspace_sandbox.sh >/dev/null
image="openmuse/x1-local-runtime:tck"
docker run --rm --platform linux/amd64 --network none --read-only --cap-drop ALL \
  --security-opt no-new-privileges --pids-limit 16 --memory 128m --cpus 0.5 \
  --user 10001:10001 "$image" python3 -c '
import subprocess
children = []
try:
    for _ in range(100):
        children.append(subprocess.Popen(["sleep", "5"]))
except (OSError, BlockingIOError):
    pass
finally:
    for child in children:
        child.terminate()
    for child in children:
        child.wait()
assert len(children) < 100, "PID ceiling did not stop the process burst"
'
printf '%s\n' 'sandbox production security gate: passed'
