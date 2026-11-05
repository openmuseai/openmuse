#!/usr/bin/env python3
"""Deploy AppFlowy Web: runtime once, then artifact-only updates."""

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

_HERE = Path(__file__).resolve()
_MUSE_CLIENTS = _HERE.parents[3]
for _root in (_MUSE_CLIENTS / "local", _MUSE_CLIENTS):
    if (_root / "scripts" / "deploy_common.py").is_file():
        sys.path.insert(0, str(_root))
        break
from scripts.deploy_common import package_web_artifacts
from scripts.deploy_config import add_env_argument, load_deploy_config

try:
    import paramiko
except ImportError:
    print("Installing paramiko...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-q", "paramiko"])
    import paramiko

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCRIPTS_DIR = PROJECT_ROOT / "scripts"


def resolve_args(args: argparse.Namespace) -> argparse.Namespace:
    cfg = load_deploy_config(args.env)
    if args.host is None:
        args.host = cfg.host
    if args.user is None:
        args.user = cfg.user
    if args.password is None:
        args.password = cfg.password
    if args.domain is None:
        args.domain = cfg.app_domain
    if args.public_domain is None:
        args.public_domain = cfg.public_domain
    if args.app_dir is None:
        args.app_dir = cfg.web_app_dir
    if args.web_port is None:
        args.web_port = cfg.web_port
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


def web_urls(cfg) -> tuple[str, str, str]:
    return cfg.web_urls()


def verify_deployment(ssh: paramiko.SSHClient, args: argparse.Namespace) -> None:
    print("\n==> Verifying deployment...")
    _, out, _ = run_remote(
        ssh,
        f"curl -sk -o /dev/null -w '%{{http_code}}' https://127.0.0.1/ -H 'Host: {args.domain}'",
    )
    if "200" in out:
        print(f"    HTTPS check passed (status: {out.strip()})")
        return
    _, out2, _ = run_remote(
        ssh,
        f"curl -s -o /dev/null -w '%{{http_code}}' http://127.0.0.1:{args.web_port}/",
    )
    print(f"    Direct app check (status: {out2.strip()})")


def deploy_runtime(args: argparse.Namespace) -> None:
    remote_env = f"DOMAIN={args.domain} APP_DIR={args.app_dir} WEB_PORT={args.web_port}"
    ssh = ssh_connect(args.host, args.user, args.password)
    sftp = ssh.open_sftp()
    try:
        run_remote(ssh, f"mkdir -p {args.app_dir}/runtime {args.app_dir}/dist")
        upload_file(sftp, str(PROJECT_ROOT / "docker/nginx.conf"), f"{args.app_dir}/runtime/nginx.conf")
        print("==> Setting up runtime container (nginx)...")
        exit_code, out, err = upload_and_run_script(
            ssh, sftp, "remote-runtime-setup.sh", remote_env, timeout=600,
        )
        if exit_code != 0:
            print(f"\nERROR: Runtime setup failed with exit code {exit_code}")
            print(err[-2000:] if err else out[-2000:])
            sys.exit(1)
    finally:
        sftp.close()
        ssh.close()
    print(f"\n{'='*50}")
    print("  Runtime setup successful!")
    print(f"  Next: ./deploy.sh")
    print(f"{'='*50}")


def deploy_app(args: argparse.Namespace) -> None:
    base_url, gotrue_url, ws_url = web_urls(args._config)
    remote_env = (
        f"DOMAIN={args.domain} APP_DIR={args.app_dir} WEB_PORT={args.web_port} "
        f"ARTIFACT_TAR=/tmp/appflowy-web-artifacts.tar.gz"
    )

    with tempfile.TemporaryDirectory() as tmpdir:
        artifact_tar = args.artifact_tar if args.skip_build else os.path.join(tmpdir, "appflowy-web-artifacts.tar.gz")
        if not args.skip_build:
            package_web_artifacts(
                PROJECT_ROOT, artifact_tar,
                base_url=base_url, gotrue_url=gotrue_url, ws_url=ws_url,
                dsh_agent_url=args._config.dsh_public_url or args._config.dsh_agent_url,
            )
        elif not os.path.exists(args.artifact_tar):
            print(f"ERROR: --skip-build specified but artifact tar not found: {args.artifact_tar}")
            sys.exit(1)

        ssh = ssh_connect(args.host, args.user, args.password)
        sftp = ssh.open_sftp()
        try:
            upload_file(sftp, artifact_tar, "/tmp/appflowy-web-artifacts.tar.gz")
            print("==> Deploying build artifacts...")
            exit_code, out, err = upload_and_run_script(
                ssh, sftp, "remote-app-deploy.sh", remote_env, timeout=1800,
            )
            if exit_code != 0:
                print(f"\nERROR: App deployment failed with exit code {exit_code}")
                print(err[-2000:] if err else out[-2000:])
                sys.exit(1)
            verify_deployment(ssh, args)
        finally:
            sftp.close()
            ssh.close()

    print(f"\n{'='*50}")
    print("  AppFlowy Web deployment successful!")
    print(f"  URL: https://{args.domain}")
    print(f"{'='*50}")


def deploy_infra(args: argparse.Namespace) -> None:
    remote_env = f"DOMAIN={args.domain} APP_DIR={args.app_dir} WEB_PORT={args.web_port}"
    ssh = ssh_connect(args.host, args.user, args.password)
    sftp = ssh.open_sftp()
    try:
        exit_code, out, err = upload_and_run_script(
            ssh, sftp, "remote-infra-setup.sh", remote_env, timeout=300,
        )
        if exit_code != 0:
            print(f"\nERROR: Infrastructure setup failed with exit code {exit_code}")
            print(err[-2000:] if err else out[-2000:])
            sys.exit(1)
    finally:
        sftp.close()
        ssh.close()
    print("\nNext steps: ./deploy-runtime.sh  then  ./deploy.sh")


def deploy_all(args: argparse.Namespace) -> None:
    deploy_infra(args)
    deploy_runtime(args)
    deploy_app(args)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Deploy AppFlowy Web to server")
    add_env_argument(parser)
    parser.add_argument("--host", default=None, help="SSH host (default: from deploy profile)")
    parser.add_argument("--user", default=None, help="SSH user (default: from deploy profile)")
    parser.add_argument("--password", default=None, help="SSH password (default: from deploy profile)")
    parser.add_argument("--domain", default=None, help="App hostname for nginx (default: from deploy profile)")
    parser.add_argument("--public-domain", default=None,
                        help="Public HTTPS domain for API URLs in build (default: from deploy profile)")
    parser.add_argument("--app-dir", default=None, help="Remote app directory (default: from deploy profile)")
    parser.add_argument("--web-port", default=None, help="Web listen port (default: from deploy profile)")
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--artifact-tar", default="", help="Pre-built artifact tar for --skip-build")

    subparsers = parser.add_subparsers(dest="command")
    subparsers.add_parser("app", help="Deploy build artifacts only (default)")
    subparsers.add_parser("runtime", help="Setup nginx runtime container (first time)")
    subparsers.add_parser("infra", help="Setup .env file only")
    subparsers.add_parser("all", help="infra + runtime + app")
    return parser


def main():
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
