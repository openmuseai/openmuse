#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo +1.85.1 fmt --package openmuse-workspace-sandbox -- --check
cargo +1.85.1 test -p openmuse-workspace-sandbox
cargo +1.85.1 clippy -p openmuse-workspace-sandbox --all-targets -- -D warnings

command -v docker >/dev/null 2>&1 || {
  echo "docker is required for the local-isolated acceptance test" >&2
  exit 1
}
docker info >/dev/null

image="openmuse/x1-local-runtime:tck"
docker build --platform linux/amd64 -t "$image" sandbox/local-runtime

fixture_root="$(mktemp -d)"
container_name="openmuse-x1-process-range-$$"
cleanup() {
  docker rm -f "$container_name" >/dev/null 2>&1 || true
  rm -rf "$fixture_root"
}
trap cleanup EXIT

workspace="$fixture_root/workspace"
runtime="$fixture_root/runtime"
mkdir -p "$workspace/src" "$runtime"
printf '%s\n' '[package]' 'name = "sandbox_fixture"' 'version = "0.1.0"' 'edition = "2021"' > "$workspace/Cargo.toml"
printf '%s\n' 'fn main() { println!("openmuse sandbox needle"); }' > "$workspace/src/main.rs"
chmod -R a+rwX "$workspace" "$runtime"

common_args=(
  --platform linux/amd64
  --network none
  --read-only
  --cap-drop ALL
  --security-opt no-new-privileges
  --pids-limit 64
  --memory 768m
  --cpus 1
  --user 10001:10001
  --mount "type=bind,src=$workspace,dst=/workspace"
  --mount "type=bind,src=$runtime,dst=/runtime,readonly"
  --tmpfs /tmp:rw,nosuid,nodev,noexec,mode=1777,size=134217728
  --tmpfs /home/agent:rw,nosuid,nodev,mode=0700,uid=10001,gid=10001,size=67108864
  --tmpfs /run/openmuse:rw,nosuid,nodev,noexec,mode=0700,uid=10001,gid=10001,size=16777216
  -e HOME=/home/agent
)

docker run --rm "${common_args[@]}" "$image" bash -c '
  set -euo pipefail
  test -d /workspace && test -d /runtime && test -d /home/agent && test -d /tmp && test -d /run/openmuse
  rg -q "openmuse sandbox needle" /workspace
  python3 -c "from pathlib import Path; assert Path(\"/workspace/src/main.rs\").is_file()"
  cargo check --offline --manifest-path /workspace/Cargo.toml
  test ! -e /host-home
  test ! -e /other-workspace
  test ! -r /root/.ssh
  test "$(find /sys/class/net -mindepth 1 -maxdepth 1 -printf x | wc -c)" -eq 1
'

docker run -d --name "$container_name" --init "${common_args[@]}" "$image" \
  bash -c 'sleep 300 & child=$!; printf "%s" "$child" > /run/openmuse/child.pid; wait "$child"' >/dev/null
for _ in 1 2 3 4 5; do
  if docker exec "$container_name" test -s /run/openmuse/child.pid; then
    break
  fi
  sleep 1
done
docker exec "$container_name" test -s /run/openmuse/child.pid
docker stop --time 2 "$container_name" >/dev/null
test "$(docker inspect -f '{{.State.Running}}' "$container_name")" = "false"
docker rm "$container_name" >/dev/null

printf '%s\n' 'workspace.sandbox@1 local runtime acceptance: passed'
