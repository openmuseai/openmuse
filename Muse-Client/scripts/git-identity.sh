#!/usr/bin/env bash
# Switch Git commit identity and GitHub push transport for this machine.
#
# Several GitHub accounts share one laptop. This script keeps name/email and
# SSH keys in ~/.config/openmuse/git-identities.conf and applies one profile
# to the current repository. Push tries SSH (port 22, then 443) and falls
# back to https:// with OPENMUSE_TOKEN when GitHub SSH is blocked.
#
#   scripts/git-identity.sh list
#   scripts/git-identity.sh init
#   scripts/git-identity.sh use alexixixi
#   scripts/git-identity.sh status
#   scripts/git-identity.sh ssh-test
#   scripts/git-identity.sh push
#   scripts/git-identity.sh push origin HEAD:main
set -euo pipefail

CONF="${OPENMUSE_GIT_IDENTITIES:-$HOME/.config/openmuse/git-identities.conf}"

die() { echo "git-identity: $*" >&2; exit 1; }

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || die "not inside a git repository"
}

state_file() {
  echo "$(repo_root)/.git/openmuse-identity"
}

usage() {
  cat <<'EOF'
usage: git-identity.sh <command>

  init       write ~/.config/openmuse/git-identities.conf if missing
  list       show configured profiles
  use <id>   apply a profile to this repository (local user.name/email)
  status     show the active profile and how git would talk to GitHub
  ssh-test   probe GitHub SSH on port 22 and 443
  push [...] git push with SSH, then HTTPS+OPENMUSE_TOKEN if SSH is blocked

Environment:
  OPENMUSE_GIT_IDENTITIES   override the identities file
  OPENMUSE_TOKEN            GitHub token used only for the HTTPS fallback
EOF
}

ensure_conf() {
  if [[ -f "$CONF" ]]; then
    return
  fi
  mkdir -p "$(dirname "$CONF")"
  cat >"$CONF" <<'EOF'
# OpenMuse local Git identities. Keys stay in ~/.ssh; never copy them into a repo.
# ssh_key is optional. host_alias is an optional Host entry from ~/.ssh/config.

[alexixixi]
name=Alexixixi
email=Chelsea-Muse@outlook.com
ssh_key=~/.ssh/chelsea-muse

[openmuseai]
name=Chelsea
email=Tsingbei2024@163.com
ssh_key=~/.ssh/id_ed25519_openmuseai
host_alias=github-openmuseai

[heqixi]
name=heqixi
email=your_email@example.com
ssh_key=~/.ssh/id_ed25519_heqixi
host_alias=github-heqixi
EOF
  echo "wrote $CONF"
}

profile_ids() {
  [[ -f "$CONF" ]] || return 0
  awk '/^\[.+\]$/ { gsub(/[\[\]]/, ""); print }' "$CONF"
}

profile_get() {
  local id="$1" key="$2"
  awk -v id="$id" -v key="$key" '
    $0 == "[" id "]" { found=1; next }
    found && /^\[/ { exit }
    found && $0 ~ "^" key "=" {
      sub("^" key "=", "")
      print
      exit
    }
  ' "$CONF"
}

expand_path() {
  local value="$1"
  if [[ "$value" == ~* ]]; then
    echo "${value/#\~/$HOME}"
  else
    echo "$value"
  fi
}

require_profile() {
  local id="$1"
  ensure_conf
  local name
  name="$(profile_get "$id" name || true)"
  [[ -n "$name" ]] || die "unknown profile: $id (see $CONF)"
}

origin_url() {
  git -C "$(repo_root)" remote get-url origin
}

github_slug() {
  local url
  url="$(origin_url)"
  if [[ "$url" =~ github.com[:/]+([^/]+)/([^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
    return
  fi
  die "origin is not a GitHub URL: $url"
}

origin_https_url() {
  echo "https://github.com/$(github_slug).git"
}

origin_ssh_url() {
  echo "git@github.com:$(github_slug).git"
}

push_refspecs() {
  if [[ $# -eq 0 ]]; then
    echo "HEAD:refs/heads/$(git rev-parse --abbrev-ref HEAD)"
    return
  fi
  if [[ "$1" == origin || "$1" == "$(origin_url)" || "$1" == "$(origin_ssh_url)" || "$1" == "$(origin_https_url)" ]]; then
    shift
  fi
  if [[ $# -eq 0 ]]; then
    echo "HEAD:refs/heads/$(git rev-parse --abbrev-ref HEAD)"
    return
  fi
  printf '%s\n' "$@"
}

active_id() {
  local file
  file="$(state_file)"
  if [[ -f "$file" ]]; then
    cat "$file"
  fi
}

cmd_init() { ensure_conf; }

cmd_list() {
  ensure_conf
  local current
  current="$(active_id || true)"
  echo "identities file: $CONF"
  local id name email key
  for id in $(profile_ids); do
    name="$(profile_get "$id" name)"
    email="$(profile_get "$id" email)"
    key="$(profile_get "$id" ssh_key || true)"
    if [[ "$id" == "$current" ]]; then
      printf '* %s  %s <%s>' "$id" "$name" "$email"
    else
      printf '  %s  %s <%s>' "$id" "$name" "$email"
    fi
    [[ -n "$key" ]] && printf '  key=%s' "$key"
    echo
  done
}

cmd_use() {
  local id="${1:-}"
  [[ -n "$id" ]] || die "usage: git-identity.sh use <id>"
  require_profile "$id"
  local root name email
  root="$(repo_root)"
  name="$(profile_get "$id" name)"
  email="$(profile_get "$id" email)"
  git -C "$root" config --local user.name "$name"
  git -C "$root" config --local user.email "$email"
  echo "$id" >"$(state_file)"
  echo "this repository now commits as $name <$email> (profile $id)"
}

cmd_status() {
  local root
  root="$(repo_root)"
  echo "repository: $root"
  echo "origin: $(origin_url)"
  echo "profile: $(active_id || echo '(none; git uses user.name/email as configured)')"
  echo "user.name: $(git -C "$root" config --get user.name || true)"
  echo "user.email: $(git -C "$root" config --get user.email || true)"
}

cmd_ssh_test() {
  local id="${1:-$(active_id || true)}"
  local key=""
  if [[ -n "$id" ]]; then
    require_profile "$id"
    key="$(profile_get "$id" ssh_key || true)"
    echo "profile: $id"
  fi
  echo "--- github.com:22 ---"
  if [[ -n "$key" ]]; then
    ssh -o BatchMode=yes -o ConnectTimeout=8 -o IdentitiesOnly=yes \
      -i "$(expand_path "$key")" -T git@github.com 2>&1 | tail -8 || true
  else
    ssh -o BatchMode=yes -o ConnectTimeout=8 -T git@github.com 2>&1 | tail -8 || true
  fi
  echo "--- ssh.github.com:443 ---"
  if [[ -n "$key" ]]; then
    ssh -o BatchMode=yes -o ConnectTimeout=8 -o IdentitiesOnly=yes \
      -i "$(expand_path "$key")" -p 443 -T git@ssh.github.com 2>&1 | tail -8 || true
  else
    ssh -o BatchMode=yes -o ConnectTimeout=8 -p 443 -T git@ssh.github.com 2>&1 | tail -8 || true
  fi
}

try_ssh_push() {
  local id="${1:-}"
  shift
  local key=""
  local ssh_base="ssh -o BatchMode=yes -o IdentitiesOnly=yes -o ConnectTimeout=8"
  if [[ -n "$id" ]]; then
    key="$(profile_get "$id" ssh_key || true)"
    if [[ -n "$key" ]]; then
      ssh_base="$ssh_base -i $(expand_path "$key")"
    fi
  fi
  echo "git-identity: trying SSH git@github.com (port 22)" >&2
  if GIT_SSH_COMMAND="$ssh_base" git push "$(origin_ssh_url)" "$@"; then
    return 0
  fi
  echo "git-identity: port 22 failed; trying ssh.github.com:443" >&2
  if GIT_SSH_COMMAND="$ssh_base -p 443 -o Hostname=ssh.github.com" \
    git push "$(origin_ssh_url)" "$@"; then
    return 0
  fi
  return 1
}

cmd_push() {
  local root id
  root="$(repo_root)"
  id="$(active_id || true)"
  cd "$root"
  local -a refs=()
  local line
  while IFS= read -r line; do
    [[ -n "$line" ]] && refs+=("$line")
  done < <(push_refspecs "$@")
  if try_ssh_push "$id" "${refs[@]}"; then
    echo "git-identity: pushed over SSH"
    return 0
  fi
  echo "git-identity: SSH to GitHub timed out or was refused; using OPENMUSE_TOKEN over HTTPS" >&2
  [[ -n "${OPENMUSE_TOKEN:-}" ]] || die "OPENMUSE_TOKEN is unset; cannot use HTTPS fallback"
  git -c "credential.helper=" \
    -c "credential.helper=!f() { echo username=x-access-token; echo password=\${OPENMUSE_TOKEN}; }; f" \
    push "$(origin_https_url)" "${refs[@]}"
  echo "git-identity: pushed over HTTPS"
}

cmd="${1:-}"
shift || true
case "$cmd" in
  init) cmd_init ;;
  list) cmd_list ;;
  use) cmd_use "${1:-}" ;;
  status) cmd_status ;;
  ssh-test) cmd_ssh_test "${1:-}" ;;
  push) cmd_push "$@" ;;
  -h|--help|help|"") usage ;;
  *) die "unknown command: $cmd" ;;
esac
