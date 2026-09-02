#!/usr/bin/env python3
"""Deploy Muse Remote DSH: infra (nginx vhost), runtime (image+compose), app (new image)."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

def _bootstrap_root() -> Path:
    cur = Path(__file__).resolve()
    for path in [cur.parent, *cur.parents]:
        if (path / "scripts" / "deploy_config.py").is_file() and (path / "middlewares" / "dsh").is_dir():
            return path
    raise SystemExit("Muse repo root not found")


MUSE_ROOT = _bootstrap_root()
sys.path.insert(0, str(MUSE_ROOT))
from scripts.deploy_common import save_images
from scripts.deploy_config import add_env_argument, load_deploy_config

try:
    import paramiko
except ImportError:
    print("Installing paramiko...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-q", "paramiko"])
    import paramiko

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCRIPTS_DIR = PROJECT_ROOT / "scripts"
IMAGE_NAME = "muse-dsh:local"


def resolve_args(args: argparse.Namespace) -> argparse.Namespace:
    cfg = load_deploy_config(args.env)
    if args.host is None:
        args.host = cfg.host
    if args.user is None:
        args.user = cfg.user
    if args.password is None:
        args.password = cfg.password
    if args.dsh_host is None:
        args.dsh_host = cfg.dsh_host
    if args.dsh_port is None:
        args.dsh_port = cfg.dsh_port
    if args.app_dir is None:
        args.app_dir = cfg.dsh_app_dir
        if cfg.is_local:
            args.app_dir = str(MUSE_ROOT / "dist" / "muse-dsh-local")
    if args.web_origin is None:
        args.web_origin = (cfg.web_base_url or cfg.public_base_url).rstrip("/")
        if cfg.is_local and not cfg.web_base_url:
            args.web_origin = f"http://localhost:{cfg.web_dev_port}"
    if args.base_domain is None:
        args.base_domain = cfg.public_domain
    if args.platform is None:
        args.platform = cfg.deploy_platform
    args._config = cfg
    return args


def ssh_connect(host: str, user: str, password: str) -> paramiko.SSHClient:
    ssh = paramiko.SSHClient()
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    print(f"==> Connecting to {user}@{host}...")
    ssh.connect(host, username=user, password=password, timeout=30)
    return ssh


def run_remote(ssh: paramiko.SSHClient, cmd: str, timeout: int = 600) -> tuple[int, str, str]:
    print(f"    $ {cmd[:120]}{'...' if len(cmd) > 120 else ''}")
    stdin, stdout, stderr = ssh.exec_command(cmd, timeout=timeout)
    exit_code = stdout.channel.recv_exit_status()
    out = stdout.read().decode()
    err = stderr.read().decode()
    if out.strip():
        for line in out.strip().split("\n")[-20:]:
            print(f"    {line}")
    if exit_code != 0 and err.strip():
        print(f"    STDERR: {err.strip()[-500:]}")
    return exit_code, out, err


def upload_file(sftp: paramiko.SFTPClient, local_path: str, remote_path: str) -> None:
    size_mb = os.path.getsize(local_path) / (1024 * 1024)
    print(f"==> Uploading {os.path.basename(local_path)} ({size_mb:.1f} MB) -> {remote_path}")
    remote_dir = os.path.dirname(remote_path)
    current = ""
    for part in remote_dir.split("/"):
        if not part:
            continue
        current += f"/{part}"
        try:
            sftp.stat(current)
        except FileNotFoundError:
            sftp.mkdir(current)
    sftp.put(local_path, remote_path)


def upload_and_run_script(
    ssh: paramiko.SSHClient,
    sftp: paramiko.SFTPClient,
    script_name: str,
    remote_env: str,
    timeout: int = 1800,
) -> tuple[int, str, str]:
    local_script = str(SCRIPTS_DIR / script_name)
    remote_script = f"/tmp/{script_name}"
    upload_file(sftp, local_script, remote_script)
    run_remote(ssh, f"chmod +x {remote_script}")
    return run_remote(ssh, f"{remote_env} bash {remote_script}", timeout=timeout)


def remote_env(args: argparse.Namespace, extra: str = "") -> str:
    cloud_url = (args._config.public_base_url or "").rstrip("/")
    parts = [
        f"DSH_HOST={args.dsh_host}",
        f"BASE_DOMAIN={args.base_domain}",
        f"WEB_ORIGIN={args.web_origin}",
        f"APP_DOMAIN={getattr(args._config, 'app_domain', '') or ''}",
        f"APP_DIR={args.app_dir}",
        f"DSH_PORT={args.dsh_port}",
        f"IMAGE_NAME={IMAGE_NAME}",
        f"DSH_TRUSTED_HOST={args.dsh_host}",
        f"MUSE_DOCUMENT_CLOUD_URL={cloud_url}",
        extra,
    ]
    return " ".join(p for p in parts if p.strip())


def seed_remote_dsh_env(sftp: paramiko.SFTPClient, args: argparse.Namespace) -> None:
    """Upload model key from local .env.dsh.local without printing it."""
    local_secret = MUSE_ROOT / ".env.dsh.local"
    remote_path = f"{args.app_dir}/.env"
    existing = ""
    try:
        with sftp.open(remote_path, "r") as handle:
            existing = handle.read().decode()
    except FileNotFoundError:
        existing = ""
    values: dict[str, str] = {}
    for raw in existing.splitlines():
        if not raw.strip() or raw.strip().startswith("#") or "=" not in raw:
            continue
        key, value = raw.split("=", 1)
        values[key.strip()] = value.strip()
    if local_secret.exists():
        for raw in local_secret.read_text(encoding="utf-8").splitlines():
            if not raw.strip() or raw.strip().startswith("#") or "=" not in raw:
                continue
            key, value = raw.split("=", 1)
            key = key.strip()
            if key == "DEEPSEEK_API_KEY" and value.strip():
                values[key] = value.strip()
    values.setdefault("DSH_HOME", "/var/lib/muse-dsh")
    values.setdefault("HOST", "0.0.0.0")
    values.setdefault("PORT", "3080")
    values["DSH_TRUSTED_HOST"] = args.dsh_host
    cloud_url = (args._config.public_base_url or "").rstrip("/")
    if cloud_url:
        values["MUSE_DOCUMENT_CLOUD_URL"] = cloud_url
    body = "\n".join(f"{k}={v}" for k, v in values.items()) + "\n"
    with sftp.open(remote_path, "w") as handle:
        handle.write(body)
    sftp.chmod(remote_path, 0o600)
    has_key = bool(values.get("DEEPSEEK_API_KEY"))
    print(f"==> Remote DSH .env seeded (DEEPSEEK_API_KEY {'set' if has_key else 'MISSING'})")


def build_image() -> None:
    script = MUSE_ROOT / "middlewares" / "scripts" / "build-dsh-image.sh"
    print("==> Building Remote DSH image")
    subprocess.check_call(["bash", str(script), "--tag", IMAGE_NAME, "--platform", "linux/amd64"])


def save_image_tar(path: str) -> None:
    save_images([IMAGE_NAME], path)


def run_local(cmd: list[str], env: dict[str, str] | None = None) -> None:
    print(f"    $ {' '.join(cmd)}")
    merged = {**os.environ, **(env or {})}
    subprocess.check_call(cmd, env=merged)


def deploy_infra_local(args: argparse.Namespace) -> None:
    app_dir = Path(args.app_dir)
    app_dir.mkdir(parents=True, exist_ok=True)
    (app_dir / "runtime").mkdir(exist_ok=True)
    env_file = app_dir / ".env"
    if not env_file.exists():
        env_file.write_text(
            "DEEPSEEK_API_KEY=\nDSH_HOME=/var/lib/muse-dsh\nHOST=0.0.0.0\nPORT=3080\n",
            encoding="utf-8",
        )
        env_file.chmod(0o600)
        print(f"    wrote {env_file} — set DEEPSEEK_API_KEY before runtime")
    (app_dir / ".infra-ready").write_text("local\n", encoding="utf-8")
    print("Local infra: no nginx vhost. Use http://127.0.0.1:%s" % args.dsh_port)


def deploy_infra_remote(args: argparse.Namespace) -> None:
    ssh = ssh_connect(args.host, args.user, args.password)
    sftp = ssh.open_sftp()
    try:
        exit_code, out, err = upload_and_run_script(
            ssh, sftp, "remote-infra-setup.sh", remote_env(args), timeout=300
        )
        if exit_code != 0:
            print(f"\nERROR: Infrastructure setup failed with exit code {exit_code}")
            print(err[-2000:] if err else out[-2000:])
            sys.exit(1)
        seed_remote_dsh_env(sftp, args)
    finally:
        sftp.close()
        ssh.close()
    print("\nNext: ./deploy-runtime.sh")


def _env_value(path: Path, key: str) -> str:
    if not path.exists():
        return ""
    prefix = f"{key}="
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith(prefix):
            return line.split("=", 1)[1].strip().strip("'\"")
    return ""


def deploy_runtime_local(args: argparse.Namespace) -> None:
    build_image()
    app_dir = Path(args.app_dir)
    app_dir.mkdir(parents=True, exist_ok=True)
    (app_dir / "runtime").mkdir(exist_ok=True)
    compose_src = PROJECT_ROOT / "docker-compose.yml"
    compose_dst = app_dir / "runtime" / "docker-compose.yml"
    compose_dst.write_bytes(compose_src.read_bytes())
    env_file = app_dir / ".env"
    if not env_file.exists():
        print(f"ERROR: missing {env_file}; run infra first")
        sys.exit(1)
    if not _env_value(env_file, "DEEPSEEK_API_KEY"):
        print(f"ERROR: set DEEPSEEK_API_KEY in {env_file}")
        sys.exit(1)
    (app_dir / ".runtime-ready").write_text("local\n", encoding="utf-8")
    run_local(
        ["docker", "compose", "-f", str(compose_dst), "--env-file", str(env_file), "up", "-d"],
        env={
            "DSH_PORT": str(args.dsh_port),
            "MUSE_DSH_IMAGE": IMAGE_NAME,
            "DSH_ENV_FILE": str(env_file),
        },
    )


def deploy_runtime_remote(args: argparse.Namespace) -> None:
    build_image()
    with tempfile.TemporaryDirectory() as tmpdir:
        image_tar = os.path.join(tmpdir, "muse-dsh-image.tar.gz")
        save_image_tar(image_tar)
        ssh = ssh_connect(args.host, args.user, args.password)
        sftp = ssh.open_sftp()
        try:
            run_remote(ssh, f"mkdir -p {args.app_dir}/runtime {args.app_dir}/images")
            upload_file(sftp, str(PROJECT_ROOT / "docker-compose.remote.yml"), f"{args.app_dir}/runtime/docker-compose.yml")
            seed_remote_dsh_env(sftp, args)
            upload_file(sftp, image_tar, "/tmp/muse-dsh-image.tar.gz")
            env = remote_env(args, "IMAGE_TAR=/tmp/muse-dsh-image.tar.gz")
            exit_code, out, err = upload_and_run_script(
                ssh, sftp, "remote-runtime-setup.sh", env, timeout=1800
            )
            if exit_code != 0:
                print(f"\nERROR: Runtime setup failed with exit code {exit_code}")
                print(err[-2000:] if err else out[-2000:])
                sys.exit(1)
        finally:
            sftp.close()
            ssh.close()
    print("\nRuntime setup successful. Next: ./deploy.sh for image updates.")


def deploy_app_local(args: argparse.Namespace) -> None:
    deploy_runtime_local(args)


def deploy_app_remote(args: argparse.Namespace) -> None:
    build_image()
    with tempfile.TemporaryDirectory() as tmpdir:
        image_tar = os.path.join(tmpdir, "muse-dsh-image.tar.gz")
        save_image_tar(image_tar)
        ssh = ssh_connect(args.host, args.user, args.password)
        sftp = ssh.open_sftp()
        try:
            upload_file(sftp, image_tar, "/tmp/muse-dsh-image.tar.gz")
            env = remote_env(args, "IMAGE_TAR=/tmp/muse-dsh-image.tar.gz")
            exit_code, out, err = upload_and_run_script(
                ssh, sftp, "remote-app-deploy.sh", env, timeout=1800
            )
            if exit_code != 0:
                print(f"\nERROR: App deployment failed with exit code {exit_code}")
                print(err[-2000:] if err else out[-2000:])
                sys.exit(1)
        finally:
            sftp.close()
            ssh.close()
    print(f"\nMuse DSH app updated. Public origin: https://{args.dsh_host}")


def deploy_infra(args: argparse.Namespace) -> None:
    if args._config.is_local:
        deploy_infra_local(args)
        return
    if not args.host:
        print("ERROR: DEPLOY_HOST is empty; set a remote profile or use --env local")
        sys.exit(1)
    deploy_infra_remote(args)


def deploy_runtime(args: argparse.Namespace) -> None:
    if args._config.is_local:
        deploy_runtime_local(args)
        return
    deploy_runtime_remote(args)


def deploy_app(args: argparse.Namespace) -> None:
    if args._config.is_local:
        deploy_app_local(args)
        return
    deploy_app_remote(args)


def deploy_all(args: argparse.Namespace) -> None:
    deploy_infra(args)
    deploy_runtime(args)
    deploy_app(args)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Deploy Muse Remote DSH")
    add_env_argument(parser)
    parser.add_argument("--host", default=None)
    parser.add_argument("--user", default=None)
    parser.add_argument("--password", default=None)
    parser.add_argument("--dsh-host", default=None)
    parser.add_argument("--dsh-port", default=None)
    parser.add_argument("--app-dir", default=None)
    parser.add_argument("--web-origin", default=None)
    parser.add_argument("--base-domain", default=None)
    parser.add_argument("--platform", default=None)
    subparsers = parser.add_subparsers(dest="command")
    subparsers.add_parser("app", help="Rebuild and replace DSH image (default)")
    subparsers.add_parser("runtime", help="Load image and start compose (first time)")
    subparsers.add_parser("infra", help="Remote nginx vhost + .env skeleton")
    subparsers.add_parser("all", help="infra + runtime + app")
    return parser


def main() -> None:
    parser = build_parser()
    args = resolve_args(parser.parse_args())
    command = args.command or "app"
    handlers = {
        "app": deploy_app,
        "runtime": deploy_runtime,
        "infra": deploy_infra,
        "all": deploy_all,
    }
    handler = handlers.get(command)
    if not handler:
        parser.print_help()
        sys.exit(1)
    handler(args)


if __name__ == "__main__":
    main()
