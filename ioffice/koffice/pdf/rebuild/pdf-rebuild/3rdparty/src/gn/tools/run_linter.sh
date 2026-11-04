#!/bin/bash -eu
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

# Resolve root directory
cd "$(dirname "$(dirname "$0")")"

if command -v cargo >/dev/null 2>&1; then
  (cd src/gn/starlark && cargo clippy --workspace --all-targets --all-features -- -D warnings)
else
  echo "cargo is not installed, skipping Rust linting."
fi
