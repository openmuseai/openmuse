// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::Settings;

impl Settings {
    /// Returns the toolchain label for the given settings.
    pub fn toolchain(&self) -> types::LabelRef<'_> {
        self.toolchain_label().as_ref()
    }
}
