// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::OutputFile;

impl OutputFile {
    /// Converts a GN OutputFile to a starlark File.
    pub fn to_rust(&self) -> types::File {
        types::File::intern(self.value())
    }
}
