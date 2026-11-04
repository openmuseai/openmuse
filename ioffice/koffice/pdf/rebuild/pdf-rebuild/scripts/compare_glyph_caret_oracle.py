#!/usr/bin/env python3
"""Gate CE2.2 against the original JNI state machine and PDFium glyph truth."""

import re
import sys
from pathlib import Path


def lines(path: str) -> list[str]:
    return Path(path).read_text(errors="replace").splitlines()


def values(log: list[str], label: str, kind: str) -> list[int]:
    pattern = re.compile(re.escape(label) + kind + r"\[[^]]+\]=(-?\d+)")
    return [int(match.group(1)) for line in log if (match := pattern.search(line))]


passed = 0
failed = 0


def check(condition: bool, name: str) -> None:
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"FAIL {name}")


if len(sys.argv) != 3:
    raise SystemExit("usage: compare_glyph_caret_oracle.py ORIGINAL_LOG REBUILT_LOG")

original = lines(sys.argv[1])
rebuilt = lines(sys.argv[2])

for mode, log in (("original", original), ("rebuilt", rebuilt)):
    for shape, length in (("Normal", 10), ("Rotated", 12)):
        label = mode + "Glyph" + shape
        check(any(label + "Focus=true,type=1" in line for line in log), label + " focus")
        hits = values(log, label, "Hit")
        check(bool(hits) and hits == sorted(hits), label + " monotonic hit")
        check(bool(hits) and hits[0] == 0 and hits[-1] == length, label + " endpoint clamp")
        caret_lines = [line for line in log if label + "Caret[" in line]
        check(len(caret_lines) == 5, label + " caret samples")
        check(all("=true,get=" in line and "points=<null>" not in line
                  for line in caret_lines), label + " caret geometry")

check(values(original, "originalGlyphNormal", "Hit") == [0, 1, 3, 5, 7, 10, 10, 10],
      "original normal command oracle")
check(values(original, "originalGlyphRotated", "Hit") == [0, 1, 3, 5, 9, 12, 12, 12],
      "original rotated command oracle")
check(values(rebuilt, "rebuiltGlyphNormal", "Hit") == [0, 0, 1, 2, 3, 6, 9, 10],
      "rebuilt proportional glyph truth")
check(values(rebuilt, "rebuiltGlyphRotated", "Hit") == [0, 0, 1, 2, 4, 7, 11, 12],
      "rebuilt rotated glyph truth")
check(any("rebuiltGlyphNormalFocus=true,type=1,text=Wim il1 MW,length=10" in line
          for line in rebuilt), "rebuilt text UTF-16 state")
check(any("rebuiltGlyphRotatedFocus=true,type=1,text=Rotate caret,length=12" in line
          for line in rebuilt), "rebuilt rotated text state")

print(f"GLYPH_CARET_ORACLE PASS={passed} FAIL={failed}")
raise SystemExit(1 if failed else 0)
