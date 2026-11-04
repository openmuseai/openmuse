#!/usr/bin/env python3
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""A cross-platform `touch` command"""

import pathlib
import sys

for arg in sys.argv[1:]:
  pathlib.Path(arg).touch()
