#!/bin/bash
# Copyright 2026 The Chromium Authors. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

set -eu -o pipefail

# Use nightly rustfmt if available to get advanced formatting (import sorting, etc.)
if rustup run nightly rustfmt --version >/dev/null 2>&1; then
  rustup run nightly rustfmt --edition=2021 --config-path=src/gn/starlark/rustfmt-nightly.toml
else
  # Otherwise fallback to stable rustfmt with standard stable formatting rules
  rustfmt --edition=2021 --config-path=src/gn/starlark/rustfmt.toml
fi
