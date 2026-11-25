#!/usr/bin/env python3
"""Publish one immutable macOS canary artifact to the existing /downloads/ alias.

The artifact is uploaded and verified before the small channel manifest is
atomically replaced. The public website catalog is intentionally untouched.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import plistlib
import re
import shlex
import shutil
import subprocess
import sys
import zipfile
from datetime import datetime, timezone
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
WORKSPACE = REPO.parent
sys.path.insert(0, str(WORKSPACE))


def _mac_version(archive: Path) -> tuple[str, str]:
    with zipfile.ZipFile(archive) as source:
        name = "OpenMuse.app/Contents/Info.plist"
        if name not in source.namelist():
            raise ValueError("macOS archive does not contain OpenMuse.app/Contents/Info.plist")
        info = plistlib.loads(source.read(name))
    if info.get("CFBundleIdentifier") != "com.openmuseai.office":
        raise ValueError("archive bundle identifier does not match OpenMuse")
    return str(info["CFBundleShortVersionString"]), str(info["CFBundleVersion"])


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def _remote_sha256(ssh, path: str) -> str | None:
    command = f"if test -f {shlex.quote(path)}; then sha256sum {shlex.quote(path)}; fi"
    _, stdout, stderr = ssh.exec_command(command, timeout=60)
    status = stdout.channel.recv_exit_status()
    if status != 0:
        raise RuntimeError(f"remote SHA-256 failed: {stderr.read().decode().strip()}")
    line = stdout.read().decode().strip()
    return line.split()[0] if line else None


def _mkdirs(sftp, path: str) -> None:
    current = ""
    for part in path.split("/"):
        if not part:
            continue
        current += "/" + part
        try:
            sftp.stat(current)
        except FileNotFoundError:
            sftp.mkdir(current)


def _deploy(artifact: Path, manifest: bytes, filename: str, *, domain: str) -> None:
    import paramiko
    from scripts.deploy_config import load_deploy_config

    config = load_deploy_config("production")
    if not config.host or not config.password:
        raise RuntimeError("production deployment credentials are unavailable")
    remote_dir = f"{config.downloads_dir.rstrip('/')}/updates/v1/canary"
    remote_artifact = f"{remote_dir}/{filename}"
    remote_manifest = f"{remote_dir}/macos-universal.json"
    expected = _sha256(artifact)
    ssh = paramiko.SSHClient()
    ssh.load_system_host_keys()
    # Match the existing Muse-WebSite deploy transport. Production should pin
    # this host key as part of the release hardening milestone.
    ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    ssh.connect(
        config.host,
        username=config.user,
        password=config.password,
        timeout=30,
        banner_timeout=30,
        auth_timeout=30,
    )
    try:
        sftp = ssh.open_sftp()
        try:
            _mkdirs(sftp, remote_dir)
            existing = _remote_sha256(ssh, remote_artifact)
            if existing is not None and existing != expected:
                raise RuntimeError("immutable artifact filename already has different bytes")
            if existing is None:
                pending = f"{remote_artifact}.upload-{os.getpid()}"
                print(f"Uploading {artifact.name} ({artifact.stat().st_size // 1048576} MiB)")
                sshpass = shutil.which('sshpass')
                if sshpass is None:
                    raise RuntimeError('sshpass is required for resumable release uploads')
                transfer = subprocess.run(
                    [
                        sshpass, '-e', 'rsync', '-a', '--append', '--inplace',
                        '--partial', '--timeout=60', '-e',
                        'ssh -o StrictHostKeyChecking=accept-new',
                        str(artifact), f'{config.user}@{config.host}:{pending}',
                    ],
                    env={**os.environ, 'SSHPASS': config.password},
                    capture_output=True,
                    text=True,
                    timeout=900,
                )
                if transfer.returncode != 0:
                    raise RuntimeError(f'rsync upload failed: {transfer.stderr.strip()}')
                if _remote_sha256(ssh, pending) != expected:
                    raise RuntimeError("uploaded artifact digest differs from local build")
                sftp.rename(pending, remote_artifact)
            pending_manifest = f"{remote_manifest}.upload-{os.getpid()}"
            with sftp.open(pending_manifest, "wb") as output:
                output.write(manifest)
            # OpenSSH's POSIX rename replaces an existing manifest atomically.
            sftp.posix_rename(pending_manifest, remote_manifest)
        finally:
            sftp.close()
    finally:
        ssh.close()
    print(f"Published https://{domain}/downloads/updates/v1/canary/macos-universal.json")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--artifact", type=Path, required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--build", type=int, required=True)
    parser.add_argument("--notes-zh", required=True)
    parser.add_argument("--notes-en", required=True)
    parser.add_argument("--domain", default="openmuseai.com")
    parser.add_argument("--deploy", action="store_true")
    args = parser.parse_args()
    if not re.fullmatch(r"\d+\.\d+\.\d+", args.version) or args.build < 1:
        parser.error("version must be x.y.z and build must be positive")
    if not re.fullmatch(r"[a-z0-9.-]+", args.domain):
        parser.error("invalid domain")
    archive = args.artifact.resolve(strict=True)
    actual_version, actual_build = _mac_version(archive)
    if (actual_version, actual_build) != (args.version, str(args.build)):
        raise ValueError(
            f"archive says {actual_version}+{actual_build}, requested {args.version}+{args.build}"
        )
    filename = f"OpenMuse-{args.version}-{args.build}-macos-universal.zip"
    output_dir = REPO / "dist" / "updates" / "v1" / "canary"
    output_dir.mkdir(parents=True, exist_ok=True)
    staged = output_dir / filename
    if staged != archive:
        import shutil

        shutil.copy2(archive, staged)
    manifest = {
        "schema": "openmuse.app-update/v1",
        "channel": "canary",
        "target": "macos-universal",
        "version": args.version,
        "build": args.build,
        "publishedAt": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "downloadUrl": f"https://{args.domain}/downloads/updates/v1/canary/{filename}",
        "sha256": _sha256(staged),
        "sizeBytes": staged.stat().st_size,
        "installKind": "macos-zip",
        "mandatory": False,
        "notes": {"zh": args.notes_zh, "en": args.notes_en},
    }
    encoded = (json.dumps(manifest, ensure_ascii=False, indent=2) + "\n").encode()
    manifest_path = output_dir / "macos-universal.json"
    manifest_path.write_bytes(encoded)
    print(f"Prepared {manifest_path}")
    print(f"SHA-256 {manifest['sha256']}")
    if args.deploy:
        _deploy(staged, encoded, filename, domain=args.domain)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
