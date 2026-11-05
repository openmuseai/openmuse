"""Load frontend/web/brand/config.yaml (tiny subset of YAML, stdlib only)."""
from __future__ import annotations

import argparse
import shlex
from dataclasses import dataclass, field
from pathlib import Path

_LIB_DIR = Path(__file__).resolve().parent
BRAND_CONFIG_PATH = _LIB_DIR.parent.parent / "brand" / "config.yaml"


@dataclass(frozen=True)
class BrandConfig:
    name_en: str
    name_zh: str
    site_url: str
    app_path: str
    terms_url: str
    privacy_url: str
    download_url: str
    github_url: str
    twitter_handle: str
    discord_url: str
    docs_url: str
    whats_new_url: str
    templates_url: str
    forum_url: str
    reddit_url: str
    desktop_scheme: str
    legacy_desktop_scheme: str
    description: str
    legacy_display: tuple[str, ...] = field(default_factory=tuple)

    @property
    def github_issues_url(self) -> str:
        base = self.github_url.rstrip("/")
        if not base:
            return ""
        return f"{base}/issues/new/choose"

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


def _str(data: dict, key: str, default: str = "") -> str:
    value = data.get(key, default)
    if value is None:
        return default
    return str(value).strip()


def load_brand_config(path: Path | None = None) -> BrandConfig:
    config_path = path or BRAND_CONFIG_PATH
    data = _parse_simple_yaml(config_path.read_text(encoding="utf-8"))
    display = data.get("display") or {}
    name_en = str(display.get("en") or "").strip()
    name_zh = str(display.get("zh") or "").strip()
    if not name_en or not name_zh:
        raise ValueError(f"{config_path} must set display.en and display.zh")
    legacy = data.get("legacy_display") or []
    if not isinstance(legacy, list):
        legacy = []
    return BrandConfig(
        name_en=name_en,
        name_zh=name_zh,
        site_url=_str(data, "site_url", "https://openmuseai.com"),
        app_path=_str(data, "app_path", "/app") or "/app",
        terms_url=_str(data, "terms_url"),
        privacy_url=_str(data, "privacy_url"),
        download_url=_str(data, "download_url"),
        github_url=_str(data, "github_url"),
        twitter_handle=_str(data, "twitter_handle").lstrip("@"),
        discord_url=_str(data, "discord_url"),
        docs_url=_str(data, "docs_url"),
        whats_new_url=_str(data, "whats_new_url"),
        templates_url=_str(data, "templates_url"),
        forum_url=_str(data, "forum_url"),
        reddit_url=_str(data, "reddit_url"),
        desktop_scheme=_str(data, "desktop_scheme", "openmuse://") or "openmuse://",
        legacy_desktop_scheme=_str(data, "legacy_desktop_scheme", "appflowy-flutter://"),
        description=_str(
            data,
            "description",
            f"{name_en} is an AI-native office suite. Work locally, keep your data, and let AI help you write.",
        ),
        legacy_display=tuple(str(item) for item in legacy if str(item).strip()),
    )


def export_env(brand: BrandConfig | None = None) -> str:
    brand = brand or load_brand_config()
    pairs = {
        "BRAND_NAME_EN": brand.name_en,
        "BRAND_NAME_ZH": brand.name_zh,
        "BRAND_SITE_URL": brand.site_url,
        "BRAND_DOWNLOAD_URL": brand.download_url,
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
