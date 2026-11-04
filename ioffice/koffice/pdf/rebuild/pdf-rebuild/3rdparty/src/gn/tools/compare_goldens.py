#!/usr/bin/env python3
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

import argparse
import difflib
import os
from pathlib import Path
import re
import shlex
import shutil
import sys

COLOR_GREEN = '\033[32m'
COLOR_RED = '\033[31m'
COLOR_CYAN = '\033[36m'
COLOR_RESET = '\033[0m'

# These two lines are platform-dependent, since they contain the GN binary itself.
STRIP_REGEN_COMMAND = re.compile(
    r'^(rule gn|build build.ninja.stamp.*)\n(?:[ \t]+.+\n)*', flags=re.MULTILINE
)


def get_ninja_files(directory: Path) -> set[Path]:
  ninja_files = set()
  for root, _, files in os.walk(directory):
    for f in files:
      if f.endswith('.ninja'):
        rel = os.path.relpath(os.path.join(root, f), directory)
        ninja_files.add(Path(rel))
  return ninja_files


def compare_files(want: Path, got: Path, update) -> bool:
  got_text = STRIP_REGEN_COMMAND.sub(
      '', got.read_text(encoding='utf-8', errors='replace')
  )
  want_text = want.read_text(encoding='utf-8', errors='replace')

  diff = list(
      difflib.unified_diff(
          want_text.splitlines(keepends=True),
          got_text.splitlines(keepends=True),
          fromfile=str(want),
          tofile=str(got),
          lineterm='\n',
      )
  )

  if not diff:
    return False

  if update:
    want.write_text(got_text)
    return False

  for line in diff:
    if line.startswith('+') and not line.startswith('+++'):
      sys.stdout.write(COLOR_GREEN + line + COLOR_RESET)
    elif line.startswith('-') and not line.startswith('---'):
      sys.stdout.write(COLOR_RED + line + COLOR_RESET)
    elif line.startswith('@@'):
      sys.stdout.write(COLOR_CYAN + line + COLOR_RESET)
    else:
      sys.stdout.write(line)
  return True


def main():
  parser = argparse.ArgumentParser(
      description='Compare or update ninja output files.'
  )
  parser.add_argument(
      '--update', action='store_true', help='Update the golden files'
  )
  parser.add_argument('generated_dir', help='Path to generated files directory')
  parser.add_argument('golden_dir', help='Path to golden files directory')
  args = parser.parse_args()

  want_dir = Path(args.golden_dir).resolve()
  got_dir = Path(args.generated_dir).resolve()

  want_files = get_ninja_files(want_dir)
  got_files = get_ninja_files(got_dir)

  return_code = 0
  all_files = want_files | got_files

  for f in sorted(all_files):
    want = want_dir / f
    got = got_dir / f

    if f not in got_files:
      if args.update:
        want.unlink()
      else:
        print(f'Error: Missing generated file: {f}', file=sys.stderr)
        return_code = 1
    elif f not in want_files:
      if args.update:
        shutil.copy2(got, want)
      else:
        print(f'Error: Unexpected generated file: {f}', file=sys.stderr)
        return_code = 1
    else:
      if compare_files(want, got, args.update):
        return_code = 1

  if return_code:
    cmd = ' '.join([
        shlex.quote(sys.executable),
        shlex.quote(str(Path(__file__).resolve())),
        shlex.quote(str(got_dir)),
        shlex.quote(str(want_dir)),
        '--update',
    ])
    print(
        f'Run `{cmd}` to update the golden files if the changes are'
        ' intentional.'
    )
  sys.exit(return_code)


if __name__ == '__main__':
  main()
