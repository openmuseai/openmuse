#!/usr/bin/env python3
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

"""A cross-platform rm -rf"""

import os
import shutil
import sys

if os.path.exists(sys.argv[1]):
  shutil.rmtree(sys.argv[1])
