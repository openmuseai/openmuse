#!/usr/bin/env python3
"""Print the package identity fields used by China app filing.

The filing form wants a stable store signing certificate. Pass the release
keystore, the signed APK/AAB, or the iOS/Harmony certificate. Passwords stay
in the environment and are never printed.

  ./scripts/filing/app-filing-identity.sh all
  ./scripts/filing/app-filing-identity.sh android --apk dist/OpenMuse-Android-arm64.apk
  ./scripts/filing/app-filing-identity.sh android --keystore release.p12
  ./scripts/filing/app-filing-identity.sh ios --cert distribution.cer
  ./scripts/filing/app-filing-identity.sh harmony --cert harmony.cer --bundle-id com.example.app
"""

from __future__ import annotations

import argparse
import hashlib
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

from cryptography import x509
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ec, rsa
from cryptography.hazmat.primitives.serialization import pkcs12

REPO_ROOT = Path(__file__).resolve().parents[2]
MOBILE_ROOT = REPO_ROOT / "app" / "openmuse_mobile"
HARMONY_ROOT = REPO_ROOT / "app" / "openmuse_harmony"
PUBLIC_CERTS = MOBILE_ROOT / "release_materials" / "assets"


@dataclass
class PublicKeyMaterial:
    algorithm: str
    hex_value: str
    decimal_value: str
    label: str


@dataclass
class CertificateIdentity:
    subject: str
    not_before: str
    not_after: str
    md5: str
    sha1: str
    sha256: str
    public_key: PublicKeyMaterial

    @property
    def internal_alpha(self) -> bool:
        return "Internal Alpha" in self.subject


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="platform", required=True)

    android = sub.add_parser("android", help="Android 包名、公钥、MD5")
    add_material_args(android)
    android.add_argument("--package", help="override the application id")

    ios = sub.add_parser("ios", help="iOS Bundle ID、公钥、SHA1")
    add_material_args(ios)
    ios.add_argument("--bundle-id", help="override the bundle id")

    harmony = sub.add_parser("harmony", help="鸿蒙包名、公钥、MD5")
    add_material_args(harmony)
    harmony.add_argument("--bundle-id", help="override the Harmony bundle name")

    sub.add_parser("all", help="print all three platforms from the project and environment")

    args = parser.parse_args()
    if args.platform == "all":
        print_all()
        return 0
    if args.platform == "android":
        return print_android(args)
    if args.platform == "ios":
        return print_ios(args)
    return print_harmony(args)


def add_material_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--apk", type=Path, help="signed APK; its signer certificate is read")
    parser.add_argument("--keystore", type=Path, help="PKCS12 or JKS release keystore")
    parser.add_argument("--cert", type=Path, help="PEM or DER certificate")
    parser.add_argument("--p12", type=Path, help="PKCS12 certificate bundle")
    parser.add_argument("--alias", help="keystore alias; required when the store has several keys")


def print_all() -> None:
    print_section(
        "Android",
        package_label="包名",
        package_value=android_application_id(),
        identity=android_identity_from_env(),
        fingerprint_field="MD5指纹",
    )
    print()
    print_section(
        "iOS",
        package_label="Bundle ID",
        package_value=ios_bundle_id(),
        identity=certificate_from_env(
            "OPENMUSE_IOS_CERTIFICATE",
            "OPENMUSE_IOS_P12",
            "OPENMUSE_IOS_P12_PASSWORD",
            PUBLIC_CERTS / "AppleDistribution-2026.cer",
        ),
        fingerprint_field="SHA1指纹",
    )
    print()
    harmony_id, harmony_note = harmony_bundle_id()
    print_section(
        "鸿蒙",
        package_label="包名",
        package_value=harmony_id,
        identity=certificate_from_env(
            "OPENMUSE_HARMONY_CERTIFICATE",
            "OPENMUSE_HARMONY_P12",
            "OPENMUSE_HARMONY_P12_PASSWORD",
            PUBLIC_CERTS / "OpenMuse-HarmonyOS-Release-2026.cer",
        ),
        fingerprint_field="MD5指纹",
        package_note=harmony_note,
    )


def print_android(args: argparse.Namespace) -> int:
    package = args.package or package_from_apk(args.apk) or android_application_id()
    identity = identity_from_args(args, "OPENMUSE_ANDROID_STORE_PASSWORD", "OPENMUSE_ANDROID_KEY_ALIAS")
    if identity is None:
        print("缺少 Android 签名证书。传入 --apk、--keystore、--cert 或 --p12。", file=sys.stderr)
        return 2
    print_section("Android", "包名", package, identity, "MD5指纹")
    return 0


def print_ios(args: argparse.Namespace) -> int:
    bundle_id = args.bundle_id or ios_bundle_id()
    identity = identity_from_args(args, "OPENMUSE_IOS_P12_PASSWORD", None)
    if identity is None:
        print("缺少 iOS 签名证书。传入 --cert 或 --p12。", file=sys.stderr)
        return 2
    print_section("iOS", "Bundle ID", bundle_id, identity, "SHA1指纹")
    return 0


def print_harmony(args: argparse.Namespace) -> int:
    if args.bundle_id:
        bundle_id, note = args.bundle_id, ""
    else:
        bundle_id, note = harmony_bundle_id()
    identity = identity_from_args(args, "OPENMUSE_HARMONY_P12_PASSWORD", None)
    if identity is None:
        print("缺少鸿蒙签名证书。传入 --cert 或 --p12。", file=sys.stderr)
        return 2
    if not bundle_id:
        print("缺少鸿蒙包名。传入 --bundle-id，或在鸿蒙工程的 app.json5 中写 bundleName。", file=sys.stderr)
        return 2
    print_section("鸿蒙", "包名", bundle_id, identity, "MD5指纹", package_note=note)
    return 0


def print_section(
    platform: str,
    package_label: str,
    package_value: str,
    identity: CertificateIdentity | None,
    fingerprint_field: str,
    package_note: str = "",
) -> None:
    print(f"平台: {platform}")
    print(f"{package_label}: {package_value or '未配置'}")
    if package_note:
        print(f"说明: {package_note}")
    if identity is None:
        print("公钥(十六进制): 缺少上架签名证书")
        print(f"{fingerprint_field}: 缺少上架签名证书")
        return
    if identity.internal_alpha:
        print("注意: 这是内部 Alpha 证书。流水线不会保留它的私钥，不能用来备案或上架。")
    print(f"公钥(十六进制): {identity.public_key.hex_value}")
    print(f"公钥(十进制): {identity.public_key.decimal_value}")
    print(f"公钥格式: {identity.public_key.label}")
    print(f"MD5指纹: {identity.md5}")
    print(f"SHA1指纹: {identity.sha1}")
    print(f"SHA256指纹: {identity.sha256}")
    print(f"备案填写: {package_label}、公钥(十六进制)、{fingerprint_field}")
    print(f"证书主体: {identity.subject}")
    print(f"证书有效期: {identity.not_before} 至 {identity.not_after}")


def identity_from_args(
    args: argparse.Namespace,
    p12_password_env: str,
    alias_env: str | None,
) -> CertificateIdentity | None:
    if args.apk is not None:
        return certificate_from_apk(args.apk)
    if args.keystore is not None:
        alias = args.alias or (os.environ.get(alias_env) if alias_env else None)
        return certificate_from_keystore(args.keystore, alias)
    if args.cert is not None:
        return identify(load_certificate_file(args.cert))
    if args.p12 is not None:
        return certificate_from_p12(args.p12, os.environ.get(p12_password_env, ""))
    return None


def android_identity_from_env() -> CertificateIdentity | None:
    apk = os.environ.get("OPENMUSE_ANDROID_APK", "")
    if apk:
        return certificate_from_apk(Path(apk))
    keystore = os.environ.get("OPENMUSE_ANDROID_KEYSTORE", "")
    if keystore:
        return certificate_from_keystore(
            Path(keystore),
            os.environ.get("OPENMUSE_ANDROID_KEY_ALIAS"),
        )
    cert = os.environ.get("OPENMUSE_ANDROID_CERTIFICATE", "")
    if cert:
        return identify(load_certificate_file(Path(cert)))
    default_cert = PUBLIC_CERTS / "OpenMuse-Android-Release-2026.cer"
    if default_cert.is_file():
        return identify(load_certificate_file(default_cert))
    return None


def certificate_from_env(
    cert_env: str,
    p12_env: str,
    password_env: str,
    default_cert: Path | None = None,
) -> CertificateIdentity | None:
    cert = os.environ.get(cert_env, "")
    if cert:
        return identify(load_certificate_file(Path(cert)))
    p12 = os.environ.get(p12_env, "")
    if p12:
        return certificate_from_p12(Path(p12), os.environ.get(password_env, ""))
    if default_cert is not None and default_cert.is_file():
        return identify(load_certificate_file(default_cert))
    return None


def certificate_from_apk(apk: Path) -> CertificateIdentity:
    apksigner = build_tool("apksigner")
    if apksigner is None:
        raise SystemExit("找不到 apksigner。设置 ANDROID_HOME 后再从 APK 读取签名。")
    result = subprocess.run(
        [str(apksigner), "verify", "--print-certs-pem", str(apk)],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise SystemExit(result.stderr.strip() or f"apksigner 无法读取 {apk}")
    pems = re.findall(
        r"-----BEGIN CERTIFICATE-----.*?-----END CERTIFICATE-----",
        result.stdout,
        re.S,
    )
    if not pems:
        raise SystemExit(f"{apk} 没有签名证书")
    return identify(x509.load_pem_x509_certificate(pems[0].encode()))


def certificate_from_keystore(keystore: Path, alias: str | None) -> CertificateIdentity:
    password = os.environ.get("OPENMUSE_ANDROID_STORE_PASSWORD", "")
    if not password:
        raise SystemExit("读取 keystore 需要环境变量 OPENMUSE_ANDROID_STORE_PASSWORD。")
    chosen = alias or sole_alias(keystore, password)
    result = subprocess.run(
        [
            "keytool",
            "-exportcert",
            "-rfc",
            "-keystore",
            str(keystore),
            "-alias",
            chosen,
            "-storepass",
            password,
        ],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        message = result.stderr.strip() or result.stdout.strip()
        raise SystemExit(message or f"无法从 {keystore} 导出证书")
    return identify(x509.load_pem_x509_certificate(result.stdout.encode()))


def sole_alias(keystore: Path, password: str) -> str:
    result = subprocess.run(
        ["keytool", "-list", "-keystore", str(keystore), "-storepass", password],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        message = result.stderr.strip() or result.stdout.strip()
        raise SystemExit(message or f"无法读取 {keystore}")
    aliases = re.findall(r"^(.+), .+, PrivateKeyEntry", result.stdout, re.M)
    if len(aliases) != 1:
        raise SystemExit("keystore 里有多把钥匙。传入 --alias，或设置 OPENMUSE_ANDROID_KEY_ALIAS。")
    return aliases[0]


def certificate_from_p12(path: Path, password: str) -> CertificateIdentity:
    if password == "":
        raise SystemExit(f"读取 {path.name} 需要对应的口令环境变量。")
    _, certificate, _ = pkcs12.load_key_and_certificates(path.read_bytes(), password.encode())
    if certificate is None:
        raise SystemExit(f"{path} 里没有证书")
    return identify(certificate)


def load_certificate_file(path: Path) -> x509.Certificate:
    data = path.read_bytes()
    if b"BEGIN CERTIFICATE" in data:
        return x509.load_pem_x509_certificate(data)
    return x509.load_der_x509_certificate(data)


def identify(certificate: x509.Certificate) -> CertificateIdentity:
    der = certificate.public_bytes(serialization.Encoding.DER)
    return CertificateIdentity(
        subject=certificate.subject.rfc4514_string(),
        not_before=certificate.not_valid_before_utc.strftime("%Y-%m-%d"),
        not_after=certificate.not_valid_after_utc.strftime("%Y-%m-%d"),
        md5=hashlib.md5(der).hexdigest(),
        sha1=hashlib.sha1(der).hexdigest(),
        sha256=hashlib.sha256(der).hexdigest(),
        public_key=public_key_material(certificate),
    )


def public_key_material(certificate: x509.Certificate) -> PublicKeyMaterial:
    public_key = certificate.public_key()
    if isinstance(public_key, rsa.RSAPublicKey):
        modulus = public_key.public_numbers().n
        return PublicKeyMaterial("RSA", format(modulus, "x"), str(modulus), "RSA 模数")
    if isinstance(public_key, ec.EllipticCurvePublicKey):
        numbers = public_key.public_numbers()
        width = (public_key.key_size + 7) // 8
        point = b"\x04" + numbers.x.to_bytes(width, "big") + numbers.y.to_bytes(width, "big")
        return PublicKeyMaterial(
            "EC",
            point.hex(),
            str(int.from_bytes(point, "big")),
            "椭圆曲线未压缩公钥点",
        )
    raise SystemExit(f"不支持的公钥算法: {public_key.__class__.__name__}")


def package_from_apk(apk: Path | None) -> str:
    if apk is None:
        return ""
    aapt = build_tool("aapt")
    if aapt is None:
        return ""
    result = subprocess.run(
        [str(aapt), "dump", "badging", str(apk)],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        return ""
    match = re.search(r"package: name='([^']+)'", result.stdout)
    return match.group(1) if match else ""


def android_application_id() -> str:
    gradle = MOBILE_ROOT / "android" / "app" / "build.gradle.kts"
    match = re.search(r'applicationId\s*=\s*"([^"]+)"', gradle.read_text())
    if not match:
        raise SystemExit(f"无法从 {gradle} 读取 applicationId")
    return match.group(1)


def ios_bundle_id() -> str:
    project = MOBILE_ROOT / "ios" / "Runner.xcodeproj" / "project.pbxproj"
    identifiers = re.findall(r"PRODUCT_BUNDLE_IDENTIFIER = ([^;]+);", project.read_text())
    application_ids = sorted(
        {
            item.strip().strip('"')
            for item in identifiers
            if "RunnerTests" not in item and "Tests" not in item
        }
    )
    if len(application_ids) != 1:
        raise SystemExit(f"iOS Bundle ID 不唯一: {', '.join(application_ids) or '未找到'}")
    return application_ids[0]


def harmony_bundle_id() -> tuple[str, str]:
    manifest = HARMONY_ROOT / "AppScope" / "app.json5"
    if not manifest.is_file():
        return "", "仓库里还没有鸿蒙工程。有签名证书后用 --bundle-id 和 --cert 生成。"
    match = re.search(r'"bundleName"\s*:\s*"([^"]+)"', manifest.read_text())
    if not match:
        raise SystemExit(f"无法从 {manifest} 读取 bundleName")
    return match.group(1), ""


def build_tool(name: str) -> Path | None:
    sdk = os.environ.get("ANDROID_HOME") or os.environ.get("ANDROID_SDK_ROOT")
    root = Path(sdk) if sdk else Path.home() / "Library" / "Android" / "sdk"
    tools = root / "build-tools"
    if not tools.is_dir():
        return None
    revisions = sorted(tools.iterdir(), key=_revision_key)
    for revision in reversed(revisions):
        candidate = revision / name
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate
    return None


def _revision_key(path: Path) -> tuple[int, ...]:
    return tuple(int(part) if part.isdigit() else 0 for part in path.name.split("."))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BrokenPipeError:
        raise SystemExit(0)
