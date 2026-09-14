#!/usr/bin/env python3
"""Fill missing keys from gateway defaults without clobbering tenant edits.

Existing mappings win. Nested dicts are filled key-by-key so a tenant that
already has `llm-pi-ai.providers.foo` still receives `opencode-custom` when
that route is absent. Lists and scalars in the destination are left untouched.
"""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Any


def fill_missing(dest: Any, defaults: Any) -> Any:
    if not isinstance(defaults, dict):
        return dest
    if dest is None:
        return defaults
    if not isinstance(dest, dict):
        return dest
    merged = dict(dest)
    for key, value in defaults.items():
        if key not in merged:
            merged[key] = value
        else:
            merged[key] = fill_missing(merged[key], value)
    return merged


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print("usage: merge-default-settings.py <dest-settings.yaml> <defaults.yaml>", file=sys.stderr)
        return 2
    dest_path = Path(argv[1])
    defaults_path = Path(argv[2])
    try:
        import yaml
    except ImportError:
        print("merge-default-settings: skip (PyYAML not installed)")
        return 0
    defaults = yaml.safe_load(defaults_path.read_text(encoding="utf-8")) or {}
    dest = yaml.safe_load(dest_path.read_text(encoding="utf-8")) or {}
    merged = fill_missing(dest, defaults)
    if merged == dest:
        print(f"merge-default-settings: already complete {dest_path}")
        return 0
    dest_path.write_text(
        yaml.safe_dump(merged, sort_keys=False, allow_unicode=True),
        encoding="utf-8",
    )
    dest_path.chmod(0o600)
    print(f"merge-default-settings: filled missing keys in {dest_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
