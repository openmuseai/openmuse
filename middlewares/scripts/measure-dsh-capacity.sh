#!/bin/bash
set -euo pipefail

# measure-dsh-capacity.sh
# ---------------------------------------------------------------------------
# Measure the running remote DSH service (container or host process) RSS/CPU
# and print a capacity estimate for a pool host. Companion to
# docs/remote-dsh/MULTITENANCY.zh-CN.md §10.
#
# Usage:
#   ./measure-dsh-capacity.sh                          # sample 120s idle
#   ./measure-dsh-capacity.sh --seconds 300            # longer sample
#   ./measure-dsh-capacity.sh --total-mem-mb 4096 --overhead-mb 1400
#
# Prereqs: the muse-dsh container must be UP (./middlewares/dsh/deploy/dev-up.sh)
#          or a local `dsh web` sidecar listening on DSH_PORT (default 3080).
# Output: per-sample summed RSS of all DSH node processes + min/avg/max, and
#         memory-based instance-count formula with your measured avg plugged in.
# ---------------------------------------------------------------------------

SECONDS=120
TOTAL_MB=4096
OVERHEAD_MB=1400
DSH_PORT="${DSH_PORT:-3080}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --seconds) SECONDS="$2"; shift 2 ;;
    --total-mem-mb) TOTAL_MB="$2"; shift 2 ;;
    --overhead-mb) OVERHEAD_MB="$2"; shift 2 ;;
    -h|--help) sed -n '2,20p' "$0"; exit 0 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

# --- collect PIDs -----------------------------------------------------------
PIDS=()
if docker ps --format '{{.Names}}' 2>/dev/null | grep -q '^muse-dsh$'; then
  mapfile -t PIDS < <(docker top muse-dsh -eo pid 2>/dev/null | tail -n +2)
  echo "source: container muse-dsh ($((${#PIDS[@]})) processes)"
elif command -v lsof >/dev/null 2>&1; then
  mapfile -t PIDS < <(lsof -tiTCP:"${DSH_PORT}" -sTCP:LISTEN 2>/dev/null || true)
  echo "source: host process listening on :${DSH_PORT} ($((${#PIDS[@]})) pids)"
fi

if [[ ${#PIDS[@]} -eq 0 ]]; then
  echo "ERROR: no muse-dsh container and nothing listening on :${DSH_PORT}." >&2
  echo "Start it first: ./middlewares/dsh/deploy/dev-up.sh  (needs Docker + DEEPSEEK_API_KEY)" >&2
  exit 1
fi

# --- sample loop ------------------------------------------------------------
rss_kb_sum() {
  local total=0 pid
  for pid in "${PIDS[@]}"; do
    local kb
    kb="$(ps -o rss= -p "${pid}" 2>/dev/null || echo 0)"
    total=$((total + kb))
  done
  printf '%s' "$total"
}

cpu_sum() {
  local total=0 pid
  for pid in "${PIDS[@]}"; do
    local pct
    pct="$(ps -o pcpu= -p "${pid}" 2>/dev/null || echo 0)"
    total=$(awk -v a="$total" -v b="$pct" 'BEGIN{printf "%.1f", a+b}')
  done
  printf '%s' "$total"
}

min_rss_mb=1000000; max_rss_mb=0; sum_rss_mb=0; samples=0
while [[ $SECONDS -gt 0 ]]; do
  rss_mb=$(awk -v kb="$(rss_kb_sum)" 'BEGIN{printf "%.1f", kb/1024}')
  cpu=$(cpu_sum)
  min_rss_mb=$(awk -v a="$min_rss_mb" -v b="$rss_mb" 'BEGIN{print (b<a)?b:a}')
  max_rss_mb=$(awk -v a="$max_rss_mb" -v b="$rss_mb" 'BEGIN{print (b>a)?b:a}')
  sum_rss_mb=$(awk -v a="$sum_rss_mb" -v b="$rss_mb" 'BEGIN{printf "%.1f", a+b}')
  samples=$((samples + 1))
  printf 'sample %3d  sumRSS=%6.1f MiB  cpu= %5.1f%%\n' "$samples" "$rss_mb" "$cpu"
  sleep 1
  SECONDS=$((SECONDS - 1))
done

avg_rss_mb=$(awk -v s="$sum_rss_mb" -v n="$samples" 'BEGIN{printf "%.1f", s/n}')

cat <<EOF

================= RESULT =================
samples      : $samples x 1s
sum RSS      : min=${min_rss_mb} MiB  avg=${avg_rss_mb} MiB  max=${max_rss_mb} MiB (all DSH node processes)
CPU          : see per-sample line above (idle sample ≈ near 0)

--- capacity estimate (pool model, per tenant instance) ---
usable memory = TOTAL - OVERHEAD = ${TOTAL_MB} - ${OVERHEAD_MB} = $((TOTAL_MB - OVERHEAD_MB)) MiB
memory-based  = floor(usable / avg RSS) = $(( (TOTAL_MB - OVERHEAD_MB) / $(awk -v a="$avg_rss_mb" 'BEGIN{printf "%d", (a<1)?1:a}') )) concurrent READY instances

CPU bound    : 2 cores => ACTIVE_QUOTA ≈ 2 (an active agent task is CPU-bound:
               LSP / code-runtime / reasoning; idle instances cost ~0 CPU).
Recommended  : READY_QUOTA = min(memory-based, 3), ACTIVE_QUOTA = 2, queue the rest.

NOTE: OVERHEAD_MB default 1400 assumes nginx + gateway/BFF + GoTrue/Postgres
      co-located; raise it if more services share the box. Re-run with the
      numbers you want: --total-mem-mb / --overhead-mb.
EOF