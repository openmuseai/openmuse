#!/bin/bash
if [[ "${ALLOW_DISABLE_DSH_PASSWORDS:-}" != "1" ]]; then
  echo "Refusing to run remote-disable-dsh-passwords.sh without ALLOW_DISABLE_DSH_PASSWORDS=1." >&2
  echo "P0 host channel uses nginx auth_request + device token; do not open the DSH UI." >&2
  exit 1
fi

# Uninstall the third-party dsh-passwords gateway if it is on this host.
# Does not rewrite nginx vhosts or AppFlowy-Cloud .env.
set -euo pipefail

APP_DIR="${APP_DIR:-/opt/muse-dsh}"
removed=0

disable_tree() {
  local root="$1"
  [[ -d "$root" ]] || return 0
  python3 - "$root" <<'PY'
import json, shutil, sys
from pathlib import Path
root = Path(sys.argv[1])
names = ("dsh-passwords", "@slywalker2006/dsh-passwords", "slywalker2006-dsh-passwords")
removed = 0
for base in [
    root / "profiles" / "web" / "node_modules",
    root / "profiles" / "node_modules",
    root / "plugins",
]:
    if not base.is_dir():
        continue
    for name in names:
        path = base / name
        if path.exists() or path.is_symlink():
            if path.is_dir() and not path.is_symlink():
                shutil.rmtree(path)
            else:
                path.unlink()
            print(f"removed {path}")
            removed += 1
    scoped = base / "@slywalker2006"
    if scoped.is_dir():
        for child in scoped.iterdir():
            if "password" in child.name.lower():
                if child.is_dir() and not child.is_symlink():
                    shutil.rmtree(child)
                else:
                    child.unlink()
                print(f"removed {child}")
                removed += 1
for manifest in root.rglob("package.json"):
    try:
        data = json.loads(manifest.read_text())
    except Exception:
        continue
    changed = False
    deps = data.get("dependencies") or {}
    for key in list(deps):
        if "dsh-passwords" in key:
            deps.pop(key)
            changed = True
    bundles = (data.get("dsh") or {}).get("profile", {}).get("bundles")
    if isinstance(bundles, list):
        filtered = [b for b in bundles if "dsh-passwords" not in str(b)]
        if filtered != bundles:
            data["dsh"]["profile"]["bundles"] = filtered
            changed = True
    if changed:
        manifest.write_text(json.dumps(data, indent=2) + "\n")
        print(f"unpinned dsh-passwords in {manifest}")
        removed += 1
print(f"tree_removed={removed}")
PY
}

echo "==> Searching host DSH homes for dsh-passwords"
for home in \
  /root/.dsh \
  /var/lib/muse-dsh \
  /home/*/dsh-passwords \
  /home/*/App_data/dsh-passwords \
  /opt/dsh-passwords \
  /opt/muse-dsh; do
  for match in $home; do
    [[ -e "$match" ]] || continue
    if [[ -d "$match" ]] && find "$match" -maxdepth 4 -iname '*dsh-passwords*' 2>/dev/null | grep -q .; then
      echo "    found under $match"
    fi
  done
done

disable_tree /root/.dsh || true
disable_tree /var/lib/muse-dsh || true

if docker ps --format '{{.Names}}' 2>/dev/null | grep -qx muse-dsh; then
  echo "==> Removing dsh-passwords inside muse-dsh container"
  docker exec muse-dsh sh -c '
    python3 - <<'"'"'PY'"'"'
import shutil
from pathlib import Path
home = Path("/var/lib/muse-dsh")
for path in home.rglob("*"):
    name = path.name.lower()
    if "dsh-passwords" in name and path.exists():
        if path.is_dir() and not path.is_symlink():
            shutil.rmtree(path)
        elif path.is_file() or path.is_symlink():
            path.unlink()
        print("removed", path)
PY
' || true
  echo "==> Restarting muse-dsh container (not Cloud)"
  docker restart muse-dsh
  for i in $(seq 1 20); do
    sleep 2
    st=$(docker inspect -f '{{.State.Health.Status}}' muse-dsh 2>/dev/null || true)
    [[ "$st" == healthy ]] && break
  done
  if ! curl -sf -m 3 http://127.0.0.1:3080/ >/dev/null; then
    echo "==> loopback-proxy missing; starting it in the container"
    docker exec -d muse-dsh node /muse/loopback-proxy.mjs || true
    sleep 1
  fi
fi

# Host-level official dsh (not muse-dsh): stop gateway if it bound 8022.
if command -v dsh >/dev/null 2>&1; then
  echo "==> dsh plugin remove dsh-passwords (host CLI)"
  dsh plugin --profile web remove dsh-passwords 2>/dev/null || \
    dsh plugin --profile web remove @slywalker2006/dsh-passwords 2>/dev/null || true
fi

pkill -f 'dsh-passwords' 2>/dev/null || true

echo "==> Local probe"
curl -sSI -m 5 http://127.0.0.1:3080/ | head -20 || true
echo "==> disable-dsh-passwords done"
