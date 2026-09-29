#!/usr/bin/env python3
"""Switch Git commit identity and GitHub push transport for this machine.

Python port of scripts/git-identity.sh: same commands, same config file, but it
runs on Windows, macOS and Linux with only the standard library (Python 3.8+).

Several GitHub accounts share one laptop. This file keeps name/email and SSH
keys in ~/.config/openmuse/git-identities.conf and applies one profile to the
current repository. Push tries SSH (port 22, then 443) and falls back to
https:// with OPENMUSE_TOKEN when GitHub SSH is blocked.

    python scripts/git_identity.py list
    python scripts/git_identity.py init
    python scripts/git_identity.py use alexixixi
    python scripts/git_identity.py status
    python scripts/git_identity.py ssh-test
    python scripts/git_identity.py push
    python scripts/git_identity.py push origin HEAD:main

Environment:
    OPENMUSE_GIT_IDENTITIES   override the identities file
    OPENMUSE_TOKEN            GitHub token used only for the HTTPS fallback
    OPENMUSE_SSH              ssh executable to use (default: ssh from PATH)

Deliberate differences from the bash original:
  * the state file is written next to the real git dir, so linked worktrees and
    submodules behave (bash assumed <root>/.git is a directory);
  * the GitHub slug parser also accepts ssh:// URLs and Host aliases such as
    git@github-alexixixi:owner/repo.git, not only github.com;
  * the HTTPS fallback carries the token in the URL with the credential helper
    disabled (bash needed `sh` for its helper function), and captured output is
    scrubbed before printing;
  * `list` outside a repository no longer prints a "not inside a git repository"
    error, and `status` additionally reports ssh_key/host_alias;
  * SSH failures caused by an unknown host key print a hint, because
    BatchMode=yes can never prompt.
"""

from __future__ import annotations

import configparser
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, NoReturn, Optional, Sequence, Tuple

DEFAULT_CONF = """\
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
"""

USAGE = """\
usage: git_identity.py <command>

  init       write ~/.config/openmuse/git-identities.conf if missing
  list       show configured profiles
  use <id>   apply a profile to this repository (local user.name/email)
  status     show the active profile and how git would talk to GitHub
  ssh-test   probe GitHub SSH on port 22 and 443
  push [...] git push with SSH, then HTTPS+OPENMUSE_TOKEN if SSH is blocked

Environment:
  OPENMUSE_GIT_IDENTITIES   override the identities file
  OPENMUSE_TOKEN            GitHub token used only for the HTTPS fallback
  OPENMUSE_SSH              ssh executable to use (default: ssh from PATH)
"""

HOST_KEY_HINT = (
    "hint: the host key is not in known_hosts yet (BatchMode=yes never prompts).\n"
    "hint: add it once with:  ssh-keyscan github.com >> ~/.ssh/known_hosts\n"
    "hint: verify it against https://docs.github.com/authentication/"
    "connecting-to-github-with-ssh/githubs-ssh-key-fingerprints"
)

SCP_RE = re.compile(r"^(?:(?P<user>[^@/]+)@)?(?P<host>[^:/]+):(?P<path>.+)$")
URL_RE = re.compile(
    r"^(?P<scheme>[A-Za-z][A-Za-z0-9+.-]*)://(?:[^@/]+@)?(?P<host>[^:/]+)(?::\d+)?/(?P<path>.+)$"
)


def eprint(*args: object) -> None:
    print(*args, file=sys.stderr)


def die(message: str, code: int = 1) -> NoReturn:
    eprint("git-identity: %s" % message)
    raise SystemExit(code)


def conf_path() -> Path:
    override = os.environ.get("OPENMUSE_GIT_IDENTITIES")
    if override:
        return Path(override).expanduser()
    return Path.home() / ".config" / "openmuse" / "git-identities.conf"


def ssh_exe() -> str:
    return os.environ.get("OPENMUSE_SSH") or "ssh"


# --------------------------------------------------------------------------- #
# git plumbing
# --------------------------------------------------------------------------- #
def git(
    args: Sequence[str],
    cwd: Optional[Path] = None,
    check: bool = True,
) -> subprocess.CompletedProcess:
    result = subprocess.run(
        ["git", *args],
        cwd=str(cwd) if cwd else None,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if check and result.returncode != 0:
        detail = (result.stderr or "").strip() or (result.stdout or "").strip()
        die(detail or "git %s failed (%d)" % (" ".join(args), result.returncode))
    return result


def try_repo_root() -> Optional[Path]:
    result = git(["rev-parse", "--show-toplevel"], check=False)
    top = (result.stdout or "").strip()
    return Path(top) if result.returncode == 0 and top else None


def repo_root() -> Path:
    root = try_repo_root()
    if root is None:
        die("not inside a git repository")
    return root


def git_dir(root: Path) -> Path:
    result = git(["rev-parse", "--absolute-git-dir"], cwd=root, check=False)
    out = (result.stdout or "").strip()
    return Path(out) if result.returncode == 0 and out else root / ".git"


def origin_url(root: Path) -> str:
    result = git(["remote", "get-url", "origin"], cwd=root, check=False)
    url = (result.stdout or "").strip()
    if result.returncode != 0 or not url:
        die("this repository has no 'origin' remote")
    return url


def current_branch(root: Path) -> str:
    result = git(["rev-parse", "--abbrev-ref", "HEAD"], cwd=root, check=False)
    return (result.stdout or "").strip() or "HEAD"


def github_slug(url: str) -> str:
    """Return 'owner/repo' for any GitHub-ish remote, Host aliases included."""
    host = path = ""
    match = URL_RE.match(url)
    if match:
        host, path = match.group("host"), match.group("path")
    else:
        match = SCP_RE.match(url)
        if match:
            host, path = match.group("host"), match.group("path")
    if not host or "github" not in host.lower():
        die("origin is not a GitHub URL: %s" % url)

    path = path.strip("/")
    if path.endswith(".git"):
        path = path[: -len(".git")]
    parts = [chunk for chunk in path.split("/") if chunk]
    if len(parts) < 2:
        die("cannot read owner/repo from origin: %s" % url)
    return "%s/%s" % (parts[0], parts[1])


def origin_https_url(slug: str) -> str:
    return "https://github.com/%s.git" % slug


def origin_ssh_url(slug: str) -> str:
    return "git@github.com:%s.git" % slug


def push_refspecs(root: Path, slug: str, args: Sequence[str]) -> List[str]:
    """Drop a leading remote name/URL, else fall back to the current branch."""
    remaining = list(args)
    known = {"origin", origin_url(root), origin_ssh_url(slug), origin_https_url(slug)}
    if remaining and remaining[0] in known:
        remaining = remaining[1:]
    if not remaining:
        return ["HEAD:refs/heads/%s" % current_branch(root)]
    return remaining


# --------------------------------------------------------------------------- #
# identities file
# --------------------------------------------------------------------------- #
Profiles = Dict[str, Dict[str, str]]


def load_profiles() -> Profiles:
    path = conf_path()
    if not path.is_file():
        return {}
    parser = configparser.RawConfigParser(delimiters=("=",))
    try:
        parser.read(path, encoding="utf-8")
    except (configparser.Error, OSError) as exc:
        die("cannot parse %s: %s" % (path, exc))
    return {section: dict(parser.items(section)) for section in parser.sections()}


def profile_get(profiles: Profiles, pid: str, key: str) -> str:
    return (profiles.get(pid) or {}).get(key, "").strip()


def ensure_conf() -> Path:
    path = conf_path()
    if path.is_file():
        return path
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(DEFAULT_CONF, encoding="utf-8")
    except OSError as exc:
        die("cannot write %s: %s" % (path, exc))
    print("wrote %s" % path)
    return path


def require_profile(pid: str) -> Profiles:
    ensure_conf()
    profiles = load_profiles()
    if not profile_get(profiles, pid, "name"):
        die("unknown profile: %s (see %s)" % (pid, conf_path()))
    return profiles


def expand_path(value: str) -> Path:
    return Path(value).expanduser() if value.startswith("~") else Path(value)


def state_file(root: Path) -> Path:
    return git_dir(root) / "openmuse-identity"


def active_id(root: Path) -> str:
    path = state_file(root)
    if not path.is_file():
        return ""
    try:
        return path.read_text(encoding="utf-8").strip()
    except OSError:
        return ""


# --------------------------------------------------------------------------- #
# ssh plumbing
# --------------------------------------------------------------------------- #
def quote_ssh_arg(arg: str) -> str:
    """Quote one argument for GIT_SSH_COMMAND (git splits it shell-style)."""
    if arg and not re.search(r"[\s\"'\\]", arg):
        return arg
    # Forward slashes: git's split_cmdline treats backslashes as escapes, which
    # would mangle a Windows -i path inside quotes.
    return '"%s"' % arg.replace("\\", "/").replace('"', '\\"')


def ssh_command(
    pid: str,
    profiles: Profiles,
    port: Optional[int] = None,
    hostname: Optional[str] = None,
) -> List[str]:
    cmd = [ssh_exe(), "-o", "BatchMode=yes", "-o", "IdentitiesOnly=yes", "-o", "ConnectTimeout=8"]
    key = profile_get(profiles, pid, "ssh_key") if pid else ""
    if key:
        cmd += ["-i", str(expand_path(key)).replace("\\", "/")]
    if port:
        cmd += ["-p", str(port)]
    if hostname:
        cmd += ["-o", "Hostname=%s" % hostname]
    return cmd


def host_key_hint_if_needed(exit_code: int, lines: Sequence[str]) -> None:
    if exit_code == 0:
        return
    if any("host key verification failed" in line.lower() for line in lines):
        eprint(HOST_KEY_HINT)


def probe_ssh(label: str, host: str, cmd: Sequence[str]) -> int:
    print("--- %s ---" % label)
    result = subprocess.run(
        [*cmd, "-T", "git@%s" % host],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
    )
    lines = ((result.stdout or "") + (result.stderr or "")).strip().splitlines()
    for line in lines[-8:]:
        print(line)
    if not lines:
        print("(no output; exit %d)" % result.returncode)
    host_key_hint_if_needed(result.returncode, lines)
    return result.returncode


def run_visible(cmd: Sequence[str], cwd: Optional[Path], env: Dict[str, str]) -> Tuple[int, str]:
    """Run a command, echo its combined output, return (exit code, output)."""
    result = subprocess.run(
        list(cmd), cwd=str(cwd) if cwd else None, env=env,
        capture_output=True, text=True, encoding="utf-8", errors="replace",
    )
    output = ((result.stdout or "") + (result.stderr or "")).strip()
    if output:
        print(output)
    return result.returncode, output


def try_ssh_push(root: Path, pid: str, profiles: Profiles, refs: Sequence[str], ssh_url: str) -> bool:
    attempts = (
        ("22", "git-identity: trying SSH git@github.com (port 22)", None, None),
        ("443", "git-identity: port 22 failed; trying ssh.github.com:443", 443, "ssh.github.com"),
    )
    for _, message, port, hostname in attempts:
        eprint(message)
        env = dict(os.environ)
        env["GIT_TERMINAL_PROMPT"] = "0"
        env["GIT_SSH_COMMAND"] = " ".join(
            quote_ssh_arg(arg) for arg in ssh_command(pid, profiles, port, hostname)
        )
        code, output = run_visible(["git", "push", ssh_url, *refs], root, env)
        if code == 0:
            return True
        host_key_hint_if_needed(code, output.splitlines())
    return False


def scrub(text: str, secrets: Sequence[str]) -> str:
    for secret in secrets:
        if secret:
            text = text.replace(secret, "***")
    return text


# --------------------------------------------------------------------------- #
# commands
# --------------------------------------------------------------------------- #
def cmd_init(argv: Sequence[str]) -> int:
    ensure_conf()
    return 0


def cmd_list(argv: Sequence[str]) -> int:
    ensure_conf()
    profiles = load_profiles()
    root = try_repo_root()
    current = active_id(root) if root is not None else ""
    print("identities file: %s" % conf_path())
    for pid in profiles:
        name = profile_get(profiles, pid, "name")
        email = profile_get(profiles, pid, "email")
        key = profile_get(profiles, pid, "ssh_key")
        marker = "*" if pid == current else " "
        line = "%s %s  %s <%s>" % (marker, pid, name, email)
        if key:
            line += "  key=%s" % key
        print(line)
    return 0


def cmd_use(argv: Sequence[str]) -> int:
    pid = argv[0] if argv else ""
    if not pid:
        die("usage: git_identity.py use <id>")
    profiles = require_profile(pid)
    root = repo_root()
    name = profile_get(profiles, pid, "name")
    email = profile_get(profiles, pid, "email")
    git(["config", "--local", "user.name", name], cwd=root)
    git(["config", "--local", "user.email", email], cwd=root)
    state = state_file(root)
    state.parent.mkdir(parents=True, exist_ok=True)
    state.write_text(pid + "\n", encoding="utf-8")
    print("this repository now commits as %s <%s> (profile %s)" % (name, email, pid))
    return 0


def cmd_status(argv: Sequence[str]) -> int:
    root = repo_root()
    profiles = load_profiles()
    pid = active_id(root)
    print("repository: %s" % root)
    print("origin: %s" % origin_url(root))
    print("profile: %s" % (pid or "(none; git uses user.name/email as configured)"))
    for key in ("user.name", "user.email"):
        result = git(["config", "--get", key], cwd=root, check=False)
        print("%s: %s" % (key, (result.stdout or "").strip()))
    if pid:
        print("ssh_key: %s" % (profile_get(profiles, pid, "ssh_key") or "(none)"))
        alias = profile_get(profiles, pid, "host_alias")
        if alias:
            print("host_alias: %s" % alias)
    return 0


def cmd_ssh_test(argv: Sequence[str]) -> int:
    root = try_repo_root()
    profiles = load_profiles()
    pid = argv[0] if argv else (active_id(root) if root is not None else "")
    if pid:
        require_profile(pid)
        profiles = load_profiles()
        print("profile: %s" % pid)
    probe_ssh("github.com:22", "github.com", ssh_command(pid, profiles))
    probe_ssh(
        "ssh.github.com:443",
        "ssh.github.com",
        ssh_command(pid, profiles, port=443, hostname="ssh.github.com"),
    )
    return 0


def cmd_push(argv: Sequence[str]) -> int:
    root = repo_root()
    pid = active_id(root)
    profiles = load_profiles()
    slug = github_slug(origin_url(root))
    refs = push_refspecs(root, slug, argv)

    if try_ssh_push(root, pid, profiles, refs, origin_ssh_url(slug)):
        print("git-identity: pushed over SSH")
        return 0

    eprint("git-identity: SSH to GitHub timed out or was refused; using OPENMUSE_TOKEN over HTTPS")
    token = os.environ.get("OPENMUSE_TOKEN", "")
    if not token:
        die("OPENMUSE_TOKEN is unset; cannot use HTTPS fallback")

    env = dict(os.environ)
    env["GIT_TERMINAL_PROMPT"] = "0"
    result = subprocess.run(
        ["git", "-c", "credential.helper=", "push",
         "https://x-access-token:%s@github.com/%s.git" % (token, slug), *refs],
        cwd=str(root), env=env, capture_output=True, text=True,
        encoding="utf-8", errors="replace",
    )
    output = scrub(((result.stdout or "") + (result.stderr or "")).strip(), [token])
    if output:
        print(output, file=sys.stdout if result.returncode == 0 else sys.stderr)
    if result.returncode != 0:
        die("HTTPS push failed (exit %d)" % result.returncode, result.returncode)
    print("git-identity: pushed over HTTPS")
    return 0


COMMANDS = {
    "init": cmd_init,
    "list": cmd_list,
    "use": cmd_use,
    "status": cmd_status,
    "ssh-test": cmd_ssh_test,
    "push": cmd_push,
}


def main(argv: Optional[Sequence[str]] = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    if not args or args[0] in ("-h", "--help", "help"):
        print(USAGE, end="")
        return 0
    command = COMMANDS.get(args[0])
    if command is None:
        die("unknown command: %s" % args[0])
    return command(args[1:])


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except KeyboardInterrupt:
        raise SystemExit(130)
