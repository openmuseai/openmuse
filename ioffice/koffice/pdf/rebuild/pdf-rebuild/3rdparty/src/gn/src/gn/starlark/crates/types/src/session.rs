// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::{LabelRef, TargetRef};

/// An interface for the starlark session.
pub trait Session {
    /// The target type managed by this session.
    type TargetRef: TargetRef;

    /// Look up a target in the session by its label and current toolchain.
    fn get_target(&self, label: LabelRef<'_>, toolchain: LabelRef<'_>) -> Self::TargetRef;

    /// Registers a dependency from `source` to (label, toolchain).
    fn register_dependency<'a>(
        &self,
        source: Self::TargetRef,
        label: LabelRef<'a>,
        toolchain: LabelRef<'a>,
    );
}
