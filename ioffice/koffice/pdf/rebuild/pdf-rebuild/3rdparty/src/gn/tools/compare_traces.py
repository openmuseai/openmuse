#!/usr/bin/env python3
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.
"""compare_traces.py - Performance Impact Analysis & Trace Differ for GN.

Loads traces from two sets of `gn gen --tracelog=...` invocations, with each
set potentially containing multiple different tracelog files, and attempts to
aggregate, evaluate, and render the difference in performance between them in a
human-readable format.
"""

import argparse
import collections
import dataclasses
import json
import math
import pathlib
import statistics
from typing import Dict, List, Optional, Tuple

DEFAULT_MIN_DELTA_MS = 5.0
DEFAULT_ALPHA = 0.05
REGRESSION_WEIGHT = 1.5
SUBSYSTEMS_TABLE_LIMIT = 10
BUILD_FILE_TABLE_LIMIT = 15

CATEGORY_DESCRIPTIONS = {
    'file_write_ninja': 'Ninja File Emission',
    'file_exec_template': 'Template Invocations',
    'file_exec': 'BUILD.gn Execution',
    'define': 'Target Definitions',
    'onresolved': 'Target Resolution',
    'import_load': 'Import Loading',
    'script_exec': 'External Scripts',
    'parse': 'AST Parsing',
    'load': 'File Loading',
    'import_block': 'Import Block',
    'file_write_generated': 'Generated File Emission',
    'walk_metadata': 'Metadata Walks',
    'setup': 'Initialization & Setup',
}


class Stat:
  """Maintains sample statistics (mean, sample stddev, variance) for a metric."""

  def __init__(self, values: List[float]):
    self.values = values
    self.n = len(values)
    self.mean = statistics.mean(values) if values else 0.0
    self.variance = statistics.variance(values) if self.n > 1 else 0.0
    self.stddev = statistics.stdev(values) if self.n > 1 else 0.0

  def format_int(self, show_stddev: bool = True) -> str:
    """Formats mean and standard deviation rounded to integer milliseconds."""
    if self.n > 1 and show_stddev and round(self.stddev) > 0:
      return f'{round(self.mean):,d} ± {round(self.stddev):,d}'
    return f'{round(self.mean):,d}'


@dataclasses.dataclass
class SingleTraceMetrics:
  """Extracted metrics from a single Chrome trace JSON run."""

  wall_clock_ms: float
  total_cpu_ms: float
  total_events: int
  categories: Dict[str, dict]
  subsystems: Dict[str, dict]
  file_execs: Dict[str, dict]
  script_execs: Dict[str, dict]


@dataclasses.dataclass
class AggregatedTrace:
  """Aggregated statistics across multiple trace runs for a single revision."""

  run_count: int
  wall_clock: Stat
  cpu_time: Stat
  events: Stat
  categories: Dict[str, Dict[str, Stat]]
  subsystems: Dict[str, Dict[str, Stat]]
  file_execs: Dict[str, Dict[str, Stat]]
  script_execs: Dict[str, Dict[str, Stat]]


@dataclasses.dataclass
class ChangedItem:
  name: str
  before_ms: float
  after_ms: float
  delta_ms: float
  delta_pct: float
  p_val: Optional[float]
  relevance_score: float


def _betacf(a: float, b: float, x: float, max_iter: int = 200) -> float:
  """Continued fraction for regularized incomplete beta function (Lentz method)."""
  qab = a + b
  qap = a + 1.0
  qam = a - 1.0
  c, d = 1.0, 1.0 - qab * x / qap
  if abs(d) < 1e-30:
    d = 1e-30
  d = 1.0 / d
  h = d
  for m in range(1, max_iter):
    m2 = 2 * m
    aa = m * (b - m) * x / ((qam + m2) * (a + m2))
    d = 1.0 + aa * d
    if abs(d) < 1e-30:
      d = 1e-30
    c = 1.0 + aa / c
    if abs(c) < 1e-30:
      c = 1e-30
    d = 1.0 / d
    h *= d * c
    aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
    d = 1.0 + aa * d
    if abs(d) < 1e-30:
      d = 1e-30
    c = 1.0 + aa / c
    if abs(c) < 1e-30:
      c = 1e-30
    d = 1.0 / d
    del_h = d * c
    h *= del_h
    if abs(del_h - 1.0) < 1e-12:
      break
  return h


def betainc(a: float, b: float, x: float) -> float:
  """Regularized incomplete beta function I_x(a, b)."""
  if x <= 0:
    return 0.0
  if x >= 1:
    return 1.0
  front = math.exp(
      math.lgamma(a + b)
      - math.lgamma(a)
      - math.lgamma(b)
      + a * math.log(x)
      + b * math.log(1.0 - x)
  )
  if x < (a + 1.0) / (a + b + 2.0):
    return front * _betacf(a, b, x) / a
  else:
    return 1.0 - front * _betacf(b, a, 1.0 - x) / b


def compute_p_value(b_stat: Stat, a_stat: Stat) -> Optional[float]:
  """Computes two-tailed p-value using Welch's t-test with Welch-Satterthwaite df."""
  if b_stat.n < 2 or a_stat.n < 2:
    return None

  v1 = b_stat.variance / b_stat.n
  v2 = a_stat.variance / a_stat.n
  denom = math.sqrt(v1 + v2)
  if denom == 0:
    return 1.0 if b_stat.mean == a_stat.mean else 0.0

  t = (a_stat.mean - b_stat.mean) / denom
  df_num = (v1 + v2) ** 2
  df_den = (v1**2) / (b_stat.n - 1) + (v2**2) / (a_stat.n - 1)
  df = df_num / df_den if df_den > 0 else 1.0

  x = df / (df + t * t)
  return betainc(0.5 * df, 0.5, x)


def benjamini_hochberg_adjust(p_values: List[float]) -> List[float]:
  """Controls False Discovery Rate (FDR) across thousands of parallel comparisons.

  When evaluating thousands of files, a standard alpha=0.05 cutoff produces ~5%
  false positives by pure chance. The Benjamini-Hochberg procedure adjusts
  p-values so the global expected false discovery rate remains below alpha.
  """
  m = len(p_values)
  if m == 0:
    return []

  indexed_p = sorted(enumerate(p_values), key=lambda x: x[1])
  adjusted = [0.0] * m

  running_min = 1.0
  for rank_rev, (orig_idx, p) in enumerate(reversed(indexed_p)):
    rank = m - rank_rev
    adj_p = min(1.0, (m / rank) * p)
    running_min = min(running_min, adj_p)
    adjusted[orig_idx] = running_min

  return adjusted


def compute_relevance_score(delta_ms: float, p_val: Optional[float]) -> float:
  """Ranks changes by actionability: magnitude * certainty * regression priority.

  Regressions receive a 1.5x weight over speedups so problematic files appear
  at the top of the report.
  """
  abs_delta = abs(delta_ms)
  direction_weight = REGRESSION_WEIGHT if delta_ms > 0 else 1.0

  if p_val is not None:
    # Scale certainty by -log10(p): p=0.001 -> 3.0, p=0.01 -> 2.0, p=0.05 -> 1.3
    certainty = -math.log10(max(p_val, 1e-4))
  else:
    certainty = 1.0

  return abs_delta * certainty * direction_weight


def load_trace_events(trace_path: pathlib.Path) -> List[dict]:
  """Reads Chrome trace JSON file and returns the list of traceEvents."""
  with open(trace_path, 'r', encoding='utf-8') as f:
    data = json.load(f)
  return data.get('traceEvents', [])


def _get_subsystem(name: str) -> str:
  """Extracts the top-level subsystem directory (e.g. //foo/bar)."""
  package = name.lstrip('/').split('/')[:-1]
  # Eg. //foo/bar/baz/BUILD.gn -> //foo/bar
  return '//' + '/'.join(package[:2])


def extract_single_trace_metrics(events: List[dict]) -> SingleTraceMetrics:
  """Processes raw events for one run into categories, subsystems, and files."""
  no_event = lambda: {'dur_us': 0, 'count': 0}
  categories = collections.defaultdict(no_event)
  by_category = collections.defaultdict(
      lambda: collections.defaultdict(no_event)
  )
  subsystems = collections.defaultdict(no_event)
  by_thread_intervals = collections.defaultdict(list)

  def add_event(collection: dict, key: str, dur_us: int):
    entry = collection[key]
    entry['dur_us'] += dur_us
    entry['count'] += 1

  min_ts = None
  max_ts = None
  total_events = 0

  for event in events:
    # Only complete events ('ph': 'X') represent measured function execution
    if event.get('ph') != 'X':
      continue

    total_events += 1
    cat = event.get('cat', 'unknown')
    dur = event.get('dur', 0)
    ts = event.get('ts', 0)
    name = event.get('name', '')
    tid = event.get('tid', 0)

    min_ts = ts if min_ts is None else min(min_ts, ts)
    max_ts = ts + dur if max_ts is None else max(max_ts, ts + dur)

    add_event(categories, cat, dur)
    add_event(by_category[cat], name, dur)
    by_thread_intervals[tid].append((ts, ts + dur))

    if cat in ('file_exec', 'parse'):
      add_event(subsystems, _get_subsystem(name), dur)

  wall_clock_ms = (
      ((max_ts - min_ts) / 1000.0)
      if (min_ts is not None and max_ts is not None)
      else 0.0
  )

  # Merge overlapping intervals per thread to eliminate nested event overcounting
  total_cpu_us = 0
  for intervals in by_thread_intervals.values():
    intervals.sort()
    last = float('-inf')
    for start, end in intervals:
      start = max(start, last)
      total_cpu_us += max(end - start, 0)
      last = max(last, end)

  return SingleTraceMetrics(
      wall_clock_ms=wall_clock_ms,
      total_cpu_ms=total_cpu_us / 1000,
      total_events=total_events,
      categories=categories,
      subsystems=subsystems,
      file_execs=by_category['file_exec'],
      script_execs=by_category['script_exec'],
  )


def _aggregate_metric_map(
    runs_metrics: List[Dict[str, dict]],
) -> Dict[str, Dict[str, Stat]]:
  """Aggregates a metric map (categories, files, etc.) across multiple runs into Stats."""
  all_keys = set()
  for m in runs_metrics:
    all_keys.update(m.keys())

  aggregated = {}
  for key in all_keys:
    durs = [m.get(key, {}).get('dur_us', 0) / 1000.0 for m in runs_metrics]
    counts = [m.get(key, {}).get('count', 0) for m in runs_metrics]
    aggregated[key] = {
        'duration_ms': Stat(durs),
        'count': Stat(counts),
    }
  return aggregated


def aggregate_multiple_traces(
    trace_paths: List[pathlib.Path],
) -> AggregatedTrace:
  """Loads multiple trace files for a revision and returns averaged Stats."""
  runs = [
      extract_single_trace_metrics(load_trace_events(p)) for p in trace_paths
  ]

  return AggregatedTrace(
      run_count=len(runs),
      wall_clock=Stat([r.wall_clock_ms for r in runs]),
      cpu_time=Stat([r.total_cpu_ms for r in runs]),
      events=Stat([r.total_events for r in runs]),
      categories=_aggregate_metric_map([r.categories for r in runs]),
      subsystems=_aggregate_metric_map([r.subsystems for r in runs]),
      file_execs=_aggregate_metric_map([r.file_execs for r in runs]),
      script_execs=_aggregate_metric_map([r.script_execs for r in runs]),
  )


def find_significant_changes(
    b_map: Dict[str, Dict[str, Stat]],
    a_map: Dict[str, Dict[str, Stat]],
    b_count: int,
    a_count: int,
    min_delta_ms: float,
    alpha: float,
    has_multi_run: bool,
) -> List[ChangedItem]:
  """Identifies items with significant deltas, applies FDR correction, and ranks by relevance."""
  all_keys = set(b_map.keys()) | set(a_map.keys())
  candidates = []

  for key in all_keys:
    b_stat = b_map.get(key, {}).get('duration_ms', Stat([0.0] * b_count))
    a_stat = a_map.get(key, {}).get('duration_ms', Stat([0.0] * a_count))
    d_ms = a_stat.mean - b_stat.mean

    if abs(d_ms) >= min_delta_ms:
      p_val = compute_p_value(b_stat, a_stat) if has_multi_run else None
      candidates.append((key, b_stat.mean, a_stat.mean, d_ms, p_val))

  if has_multi_run:
    testable = [c for c in candidates if c[4] is not None]
    adj_p_values = benjamini_hochberg_adjust([c[4] for c in testable])
    significant_items = [
        ChangedItem(
            key,
            b_ms,
            a_ms,
            d_ms,
            (d_ms / b_ms * 100.0) if b_ms > 0 else 0.0,
            adj_p,
            compute_relevance_score(d_ms, adj_p),
        )
        for (key, b_ms, a_ms, d_ms, _), adj_p in zip(testable, adj_p_values)
        if adj_p < alpha
    ]
  else:
    significant_items = [
        ChangedItem(
            key,
            b_ms,
            a_ms,
            d_ms,
            (d_ms / b_ms * 100.0) if b_ms > 0 else 0.0,
            None,
            compute_relevance_score(d_ms, None),
        )
        for key, b_ms, a_ms, d_ms, _ in candidates
    ]

  # Rank by Relevance Score (regressions first, then largest speedups)
  significant_items.sort(key=lambda item: item.relevance_score, reverse=True)
  return significant_items


def print_executive_verdict(
    b: AggregatedTrace, a: AggregatedTrace, alpha: float
) -> None:
  """Prints high-level conclusion on whether performance changed."""
  wall_delta = a.wall_clock.mean - b.wall_clock.mean
  wall_pct = (
      (wall_delta / b.wall_clock.mean * 100.0) if b.wall_clock.mean > 0 else 0.0
  )
  wall_p = compute_p_value(b.wall_clock, a.wall_clock)
  has_multi_run = b.run_count > 1 and a.run_count > 1

  print('\nEXECUTIVE VERDICT')
  print('-' * 86)
  if not has_multi_run:
    sign = '+' if round(wall_delta) > 0 else ''
    print(
        '  Single-run comparison: Wall-clock delta'
        f' {sign}{round(wall_delta):,d} ms ({sign}{wall_pct:.1f}%)'
    )
    print(
        '  (Tip: Pass multiple --before and --after runs for statistical'
        ' confidence testing)'
    )
  elif wall_p is not None and wall_p < alpha and abs(wall_pct) >= 1.0:
    if wall_delta > 0:
      print(
          f'  ⚠️  REGRESSION DETECTED: Slower by +{round(wall_delta):,d} ms'
          f' (+{wall_pct:.1f}%)'
      )
    else:
      print(
          f'  🚀 SPEEDUP DETECTED: Faster by -{abs(round(wall_delta)):,d} ms'
          f' ({wall_pct:.1f}%)'
      )
  else:
    print(
        '  ✓ NO MEASURABLE OVERALL IMPACT DETECTED (Wall-clock delta'
        f' {round(wall_delta):+d} ms / {wall_pct:+.1f}%)'
    )

  b_wc = f'{b.wall_clock.format_int()} ms'
  a_wc = f'{a.wall_clock.format_int()} ms'
  b_cpu = f'{b.cpu_time.format_int()} ms'
  a_cpu = f'{a.cpu_time.format_int()} ms'
  b_ev = f'{round(b.events.mean):,d}'
  a_ev = f'{round(a.events.mean):,d}'
  ev_delta = f'(Δ {round(a.events.mean - b.events.mean):+d})'

  print(f'\n  • Wall-clock / run:  Before: {b_wc:>17}  ->  After: {a_wc:>17}')
  print(f'  • Total CPU / run:   Before: {b_cpu:>17}  ->  After: {a_cpu:>17}')
  print(
      f'  • Events / run:      Before: {b_ev:>17}  ->  After: {a_ev:>17} '
      f' {ev_delta}'
  )
  print()


def print_changed_items_table(
    item_type: str,
    items: List[ChangedItem],
    max_items: int,
) -> None:
  """Prints a ranked table of changed items, or a clean confirmation if empty."""
  print(f'AFFECTED {item_type.upper()} (Ranked by impact)')
  print('-' * 86)
  if items:
    print(
        f'{"Delta (ms)":>11} {"Delta (%)":>10} {"Before":>11} {"After":>11}'
        f'  {"Name"}'
    )
    print('-' * 86)
    for item in items[:max_items]:
      sign = '+' if round(item.delta_ms) > 0 else ''
      pct_sign = '+' if item.delta_pct > 0 else ''
      print(
          f'{sign + f"{round(item.delta_ms):,d} ms":>11}'
          f' {pct_sign + f"{item.delta_pct:.1f}%":>9}'
          f' {round(item.before_ms):>8,d} ms {round(item.after_ms):>8,d} ms '
          f' {item.name}'
      )
  else:
    print(
        f'  ✓ Discarded all noise: No {item_type} had statistically significant'
        ' regressions or speedups.'
    )
  print()


def analyze_and_report(
    b: AggregatedTrace,
    a: AggregatedTrace,
    min_delta_ms: float = DEFAULT_MIN_DELTA_MS,
    alpha: float = DEFAULT_ALPHA,
) -> None:
  """Main coordinator: evaluates traces and produces the final cleaned report."""
  has_multi_run = b.run_count > 1 and a.run_count > 1

  print('=' * 86)
  print(' GN PERFORMANCE IMPACT REPORT')
  print('=' * 86)
  print(f'Samples:  Before: {b.run_count} run(s) | After: {a.run_count} run(s)')

  print_executive_verdict(b, a, alpha)

  phase_changes = find_significant_changes(
      b.categories,
      a.categories,
      b.run_count,
      a.run_count,
      min_delta_ms,
      alpha,
      has_multi_run,
  )
  for item in phase_changes:
    item.name = CATEGORY_DESCRIPTIONS.get(item.name, item.name)
  print_changed_items_table(
      'GN phases', phase_changes, max_items=len(CATEGORY_DESCRIPTIONS)
  )

  subsystem_changes = find_significant_changes(
      b.subsystems,
      a.subsystems,
      b.run_count,
      a.run_count,
      min_delta_ms,
      alpha,
      has_multi_run,
  )
  print_changed_items_table(
      'subsystems', subsystem_changes, max_items=SUBSYSTEMS_TABLE_LIMIT
  )

  file_changes = find_significant_changes(
      b.file_execs,
      a.file_execs,
      b.run_count,
      a.run_count,
      min_delta_ms,
      alpha,
      has_multi_run,
  )
  print_changed_items_table(
      'BUILD files', file_changes, max_items=BUILD_FILE_TABLE_LIMIT
  )


def main() -> None:
  parser = argparse.ArgumentParser(
      description=(
          'GN Trace Impact Report: Discards noise, ranks by relevance, and'
          ' highlights affected parts.'
      ),
      formatter_class=argparse.RawDescriptionHelpFormatter,
      epilog="""Examples:
  compare_traces.py --before 1.trace 2.trace 3.trace --after 4.trace 5.trace 6.trace
  compare_traces.py --before before.trace --after after.trace
""",
  )
  parser.add_argument(
      '--before',
      nargs='+',
      action='extend',
      type=pathlib.Path,
      required=True,
      help='One or more "before" trace JSON files.',
  )
  parser.add_argument(
      '--after',
      nargs='+',
      action='extend',
      type=pathlib.Path,
      required=True,
      help='One or more "after" trace JSON files.',
  )
  parser.add_argument(
      '--min-delta',
      type=float,
      default=DEFAULT_MIN_DELTA_MS,
      help=(
          'Minimum effect size in ms to consider (default:'
          f' {DEFAULT_MIN_DELTA_MS}).'
      ),
  )
  parser.add_argument(
      '--alpha',
      type=float,
      default=DEFAULT_ALPHA,
      help=f'Significance threshold FDR Q-value (default: {DEFAULT_ALPHA}).',
  )
  args = parser.parse_args()

  before = aggregate_multiple_traces(args.before)
  after = aggregate_multiple_traces(args.after)
  analyze_and_report(
      before, after, min_delta_ms=args.min_delta, alpha=args.alpha
  )


if __name__ == '__main__':
  main()
