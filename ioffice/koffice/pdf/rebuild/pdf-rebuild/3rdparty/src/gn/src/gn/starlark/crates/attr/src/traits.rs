// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Re-export the traits from types so caller crates can access them seamlessly.
pub use types::{EvalContext, OutputType, Session, TargetRef};

/// Represents a target with attributes that can be executed by custom Starlark
/// rules.
pub trait TargetAttrExt: TargetRef {
    /// Returns the resolved custom attributes of the target.
    fn attrs(&self) -> &[crate::Attr];
}

/// Extension trait for EvalContext to support target creation.
pub trait EvalContextAttrExt: types::EvalContext {
    fn create_target(
        &self,
        target_type: Option<OutputType>,
        target_name: &str,
        scope: &Self::Scope,
        rule: starlark::values::FrozenValue,
        attrs: Vec<crate::Attr>,
    ) -> starlark::Result<<Self::Session as types::Session>::TargetRef>;
}
