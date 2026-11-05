"""Load frontend/client/brand/config.yaml (tiny subset of YAML, stdlib only)."""
from __future__ import annotations

import argparse
import re
import shlex
from dataclasses import dataclass, field
from pathlib import Path

_LIB_DIR = Path(__file__).resolve().parent
BRAND_CONFIG_PATH = _LIB_DIR.parent.parent / "brand" / "config.yaml"


@dataclass(frozen=True)
class BrandConfig:
    name_en: str
    name_zh: str
    binary_windows: str
    binary_linux: str
    macos_app: str
    data_dir: str
    data_dir_linux: str
    artifact_prefix: str
    company_name: str
    legacy_display: tuple[str, ...] = field(default_factory=tuple)

    @property
    def windows_exe(self) -> str:
        return f"{self.binary_windows}.exe"

    @property
    def macos_app_bundle(self) -> str:
        return f"{self.macos_app}.app"

    def display_for_locale(self, locale: str) -> str:
        code = locale.replace("_", "-").lower()
        if code.startswith("zh"):
            return self.name_zh
        return self.name_en


def _parse_simple_yaml(text: str) -> dict:
    """Maps, nested maps, and string lists. No anchors or multiline values."""
    root: dict = {}
    stack: list[tuple[int, dict | list]] = [(-1, root)]
    lines = text.splitlines()
    i = 0
    while i < len(lines):
        raw = lines[i]
        stripped = raw.split("#", 1)[0].rstrip()
        if not stripped.strip():
            i += 1
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        while stack and indent <= stack[-1][0]:
            stack.pop()
        parent = stack[-1][1]
        line = stripped.strip()
        if line.startswith("- "):
            if not isinstance(parent, list):
                raise ValueError(f"list item without list parent: {line}")
            parent.append(_unquote(line[2:].strip()))
            i += 1
            continue
        key, sep, rest = line.partition(":")
        if not sep:
            raise ValueError(f"cannot parse: {line}")
        if not isinstance(parent, dict):
            raise ValueError(f"map key inside list: {line}")
        key = key.strip()
        rest = rest.strip()
        if rest == "":
            j = i + 1
            while j < len(lines) and not lines[j].split("#", 1)[0].strip():
                j += 1
            is_list = False
            if j < len(lines):
                nxt = lines[j]
                nindent = len(nxt) - len(nxt.lstrip(" "))
                nline = nxt.split("#", 1)[0].strip()
                if nindent > indent and nline.startswith("- "):
                    is_list = True
            child: dict | list = [] if is_list else {}
            parent[key] = child
            stack.append((indent, child))
            i += 1
            continue
        parent[key] = _unquote(rest)
        i += 1
    return root


def _unquote(value: str) -> str:
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
        return value[1:-1]
    return value


def load_brand_config(path: Path | None = None) -> BrandConfig:
    config_path = path or BRAND_CONFIG_PATH
    data = _parse_simple_yaml(config_path.read_text(encoding="utf-8"))
    display = data.get("display") or {}
    binary = data.get("binary") or {}
    name_en = str(display.get("en") or "").strip()
    name_zh = str(display.get("zh") or "").strip()
    if not name_en or not name_zh:
        raise ValueError(f"{config_path} must set display.en and display.zh")
    data_dir = str(data.get("data_dir") or name_en).strip()
    data_dir_linux = str(data.get("data_dir_linux") or data_dir.lower()).strip()
    macos_app = str(binary.get("macos") or name_en).strip()
    legacy = data.get("legacy_display") or []
    if not isinstance(legacy, list):
        legacy = []
    return BrandConfig(
        name_en=name_en,
        name_zh=name_zh,
        binary_windows=str(binary.get("windows") or re.sub(r"\s+", "", name_en)).strip(),
        binary_linux=str(binary.get("linux") or data_dir_linux).strip(),
        macos_app=macos_app,
        data_dir=data_dir,
        data_dir_linux=data_dir_linux,
        artifact_prefix=str(data.get("artifact_prefix") or macos_app).strip(),
        company_name=str(data.get("company_name") or name_en).strip(),
        legacy_display=tuple(str(item) for item in legacy if str(item).strip()),
    )


def export_env(brand: BrandConfig | None = None) -> str:
    brand = brand or load_brand_config()
    pairs = {
        "BRAND_NAME_EN": brand.name_en,
        "BRAND_NAME_ZH": brand.name_zh,
        "BRAND_MACOS_APP": brand.macos_app,
        "BRAND_MACOS_BUNDLE": brand.macos_app_bundle,
        "BRAND_BINARY_WINDOWS": brand.binary_windows,
        "BRAND_WINDOWS_EXE": brand.windows_exe,
        "BRAND_BINARY_LINUX": brand.binary_linux,
        "BRAND_DATA_DIR": brand.data_dir,
        "BRAND_DATA_DIR_LINUX": brand.data_dir_linux,
        "BRAND_ARTIFACT_PREFIX": brand.artifact_prefix,
        "BRAND_COMPANY": brand.company_name,
    }
    return "".join(f"export {key}={shlex.quote(value)}\n" for key, value in pairs.items())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--export-env", action="store_true")
    parser.add_argument("--config", type=Path, default=None)
    args = parser.parse_args()
    brand = load_brand_config(args.config)
    if args.export_env:
        print(export_env(brand), end="")
        return 0
    print(brand)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
