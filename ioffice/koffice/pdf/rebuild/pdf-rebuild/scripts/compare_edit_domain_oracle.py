#!/usr/bin/env python3
"""Gate CE1.2c topology/display/oriented geometry against original SO logs."""

import math
import re
import sys
from pathlib import Path


NAMES = ("multicolumn", "rotated", "cropbox", "form-xobject")


def parse(path: str, prefix: str):
    text = Path(path).read_text(errors="replace")
    result = {}
    for name in NAMES:
        count_match = re.search(
            rf"{prefix}DomainCorpus\[{re.escape(name)}\]\.recognized=true,count=(\d+)", text)
        display_match = re.search(
            rf"{prefix}DomainCorpus\[{re.escape(name)}\]\.display=0,m=([-0-9.Ee+,]+)", text)
        if not count_match or not display_match:
            raise ValueError(f"missing {prefix} {name} summary in {path}")
        geometries = []
        pattern = re.compile(
            rf"{prefix}DomainCorpus\[{re.escape(name)}\]\[(\d+)\]=true,"
            rf"RectF\(([-0-9.Ee]+), ([-0-9.Ee]+), ([-0-9.Ee]+), ([-0-9.Ee]+)\),"
            rf"m=([-0-9.Ee+,]+)")
        for match in pattern.finditer(text):
            geometries.append((int(match.group(1)),
                tuple(float(match.group(i)) for i in range(2, 6)),
                tuple(float(v) for v in match.group(6).split(","))))
        geometries.sort()
        result[name] = {
            "count": int(count_match.group(1)),
            "display": tuple(float(v) for v in display_match.group(1).split(",")),
            "geometry": geometries,
        }
    return result


def world_aabb(rect, matrix):
    left, top, right, bottom = rect
    points = []
    for x, y in ((left, top), (right, top), (right, bottom), (left, bottom)):
        points.append((matrix[0] * x + matrix[1] * y + matrix[2],
                       matrix[3] * x + matrix[4] * y + matrix[5]))
    return (min(x for x, _ in points), min(y for _, y in points),
            max(x for x, _ in points), max(y for _, y in points))


def angle(matrix):
    return math.atan2(matrix[3], matrix[0])


def angle_delta(a, b):
    delta = abs(a - b) % math.pi
    return min(delta, math.pi - delta)


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: compare_edit_domain_oracle.py ORIGINAL_LOG REBUILT_LOG", file=sys.stderr)
        return 64
    original = parse(sys.argv[1], "original")
    rebuilt = parse(sys.argv[2], "rebuilt")
    failures = []
    checks = 0
    for name in NAMES:
        before, after = original[name], rebuilt[name]
        checks += 1
        if before["count"] != after["count"]:
            failures.append(f"{name}: count {before['count']} != {after['count']}")
        checks += 1
        if len(before["display"]) != 9 or any(
                abs(a - b) > 1e-4 for a, b in zip(before["display"], after["display"])):
            failures.append(f"{name}: display matrix mismatch")
        if len(before["geometry"]) != len(after["geometry"]):
            failures.append(f"{name}: geometry list length mismatch")
            continue
        for (oi, ore, om), (ri, rre, rm) in zip(before["geometry"], after["geometry"]):
            checks += 3
            if oi != ri:
                failures.append(f"{name}: domain order mismatch {oi} != {ri}")
            if angle_delta(angle(om), angle(rm)) > math.radians(0.1):
                failures.append(f"{name}[{oi}]: orientation mismatch")
            old_box, new_box = world_aabb(ore, om), world_aabb(rre, rm)
            if abs(old_box[0] - new_box[0]) > 3 or abs(old_box[1] - new_box[1]) > 3:
                failures.append(f"{name}[{oi}]: anchor mismatch {old_box} vs {new_box}")
            if (new_box[0] > old_box[0] + 3 or new_box[1] > old_box[1] + 3 or
                    new_box[2] < old_box[2] - 3 or new_box[3] < old_box[3] - 3):
                failures.append(f"{name}[{oi}]: rebuilt bounds clip original {old_box} vs {new_box}")
    if failures:
        for failure in failures:
            print("FAIL", failure)
        return 1
    print(f"EDIT_DOMAIN_ORACLE PASS={checks} FAIL=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
