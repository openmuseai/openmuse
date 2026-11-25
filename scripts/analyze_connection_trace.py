#!/usr/bin/env python3
"""Summarize OMTRACE JSONL without mixing clocks or summing nested spans."""
import argparse
import json
import math
from collections import defaultdict
from pathlib import Path


def summarize(lines):
    samples = defaultdict(list)
    failed = defaultdict(int)
    spans = {}
    for line in lines:
        if 'OMTRACE ' not in line:
            continue
        try:
            event = json.loads(line.split('OMTRACE ', 1)[1])
            key = (event['traceId'], event['spanId'])
            stage, phase, elapsed = event['stage'], event['phase'], event['elapsedUs']
            if event.get('v') != 1 or not isinstance(elapsed, (int, float)) or elapsed < 0:
                continue
        except (ValueError, TypeError, KeyError):
            continue
        phases = spans.setdefault(key, {})
        if phase in phases:  # merged log duplicate
            continue
        phases[phase] = elapsed
        if phase == 'end':
            samples[stage + '/total'].append(elapsed / 1000)
            failed[stage + '/total'] += event.get('ok') is False
        elif phase != 'start':
            samples[stage + '/' + phase].append(elapsed / 1000)
    result = []
    for stage, values in sorted(samples.items()):
        values.sort()
        percentile = lambda p: round(values[max(0, math.ceil(len(values) * p) - 1)], 3)
        result.append(dict(stage=stage, n=len(values), failures=failed[stage],
                           p50_ms=percentile(.50), p95_ms=percentile(.95), max_ms=round(values[-1], 3)))
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('logs', nargs='+', type=Path)
    parser.add_argument('--json', action='store_true')
    args = parser.parse_args()
    rows = summarize(line for path in args.logs for line in path.read_text().splitlines())
    if args.json:
        print(json.dumps(rows, indent=2))
    else:
        print(f"{'stage / cumulative phase':56} {'N':>5} {'fail':>5} {'P50 ms':>10} {'P95 ms':>10} {'max ms':>10}")
        for row in rows:
            print(f"{row['stage']:56} {row['n']:5} {row['failures']:5} {row['p50_ms']:10.3f} {row['p95_ms']:10.3f} {row['max_ms']:10.3f}")
        print('Phases are cumulative within each span. Nested spans overlap; do not add totals.')


if __name__ == '__main__':
    main()
