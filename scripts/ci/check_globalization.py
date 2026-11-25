#!/usr/bin/env python3
"""Reject new host/locale literals while tracking the existing migration debt.

This inventory gate deliberately covers business sources only. The trusted
deployment assets, protocol constants, tests and generated l10n are separate.
It supplements type-safe SDK calls; it is not a full Dart/TS AST linter.
"""
import argparse
from collections import Counter
import json
from pathlib import Path
import re
import sys
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BASELINE = ROOT / "contracts/runtime-config/v1/hardcode-baseline.json"
SOURCE_ROOTS = ("app", "packages", "plugins", "web", "services/config-control/src", "services/config-control/public")
SUFFIXES = {".dart", ".js", ".ts", ".tsx", ".jsx", ".html", ".mjs", ".cjs"}
SKIP_PARTS = {"test", "tests", "integration_test", "tool", "build", "node_modules",
              ".dart_tool", "third_party", "vendors", "l10n", "assets"}
RULES = {
    "locale-branch": re.compile(r"\.languageCode\s*(?:==|!=)\s*[\'\"](?P<value>[a-z]{2,3})[\'\"]"),
    "absolute-url": re.compile(r"(?P<quote>['\"`])(?P<value>(?:https?|wss?)://[^'\"`\s<>]+)(?P=quote)"),
    "locale-literal": re.compile(r"\bLocale\(\s*['\"](?P<value>[a-z]{2,3}(?:[-_][A-Za-z0-9]+)*)['\"]"),
    "build-language": re.compile(r"String\.fromEnvironment\(\s*['\"](?P<value>OPENMUSE_DEFAULT_LANGUAGE)['\"]"),
}


def inventory():
    counts = Counter()
    locations = {}
    files = subprocess.run(["rg", "--files", *SOURCE_ROOTS], cwd=ROOT, check=True, capture_output=True, text=True).stdout.splitlines()
    for name in sorted(files):
        file = ROOT / name
        if not file.is_file() or file.suffix not in SUFFIXES:
            continue
        relative = file.relative_to(ROOT)
        if set(relative.parts) & SKIP_PARTS:
            continue
        if file.name.endswith((".g.dart", ".freezed.dart", ".min.js", ".test.js", ".test.ts", ".spec.ts", "_test.dart")):
            continue
        # DSH's checked-in build output is generated from src/main.js.
        if relative.as_posix() in {"app/openmuse_web/web/dsh-pane.js"}:
            continue
        source = file.read_text(encoding="utf-8")
        for rule, pattern in RULES.items():
            for match in pattern.finditer(source):
                key = (relative.as_posix(), rule, match.group("value"))
                counts[key] += 1
                locations.setdefault(key, source.count("\n", 0, match.start()) + 1)
    return counts, locations


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--update-baseline", action="store_true",
                        help="Explicitly refresh reviewed migration debt; never used by CI")
    args = parser.parse_args()
    actual, locations = inventory()
    if args.update_baseline:
        data = []
        for key, count in sorted(actual.items()):
            protocol = (key[1] == "absolute-url" and (
                key[2].startswith(("http://", "https://surface.openmuse.invalid"))
                or key[2].startswith("https://a2ui.org/specification/")))
            data.append({"file": key[0], "rule": key[1], "value": key[2], "count": count,
                         "classification": "protocol" if protocol else "migration",
                         "owner": "client-platform",
                         "reason": ("Local transport, virtual origin, or protocol identifier; keep under code review."
                                    if protocol else "Existing build/locale branch; migrate to the shared locale context.")})
        BASELINE.write_text(json.dumps({"schemaVersion": 1, "findings": data}, ensure_ascii=False, indent=2) + "\n")
        print(f"Recorded {sum(actual.values())} existing literals in {len(data)} groups.")
        return 0
    baseline = json.loads(BASELINE.read_text())
    allowed = Counter({(v["file"], v["rule"], v["value"]): v["count"]
                       for v in baseline["findings"]})
    new = actual - allowed
    for key, count in sorted(new.items()):
        print(f"{key[0]}:{locations[key]}: {key[1]}: {key[2]} (+{count})", file=sys.stderr)
    if new:
        print("Use the runtime configuration SDK or a reviewed protocol exception.", file=sys.stderr)
        return 1
    print(f"Globalization gate passed; {sum(actual.values())} reviewed matches remain (including protocol constants).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
