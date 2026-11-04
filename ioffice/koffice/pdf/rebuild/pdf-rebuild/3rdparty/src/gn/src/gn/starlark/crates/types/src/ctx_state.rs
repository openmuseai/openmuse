// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::collections::SmallSet;

use crate::{File, TargetRef};

/// The state associated with the `ctx` object during the rule implementation
/// function.
#[derive(Clone, Debug, allocative::Allocative)]
pub struct CtxState<T: TargetRef> {
    /// Reference to the underlying target.
    pub target: T,
    /// All files declared by ctx.actions.declare_file that were never
    /// generated.
    pub unused_declared_outputs: SmallSet<File>,
    /// A list of phonies declared during this execution step.
    pub phonies: Vec<(File, Vec<File>)>,
}

impl<T: TargetRef> CtxState<T> {
    /// Creates a new `CtxState` for the given target.
    pub fn new(target: T) -> Self {
        Self {
            target,
            unused_declared_outputs: SmallSet::new(),
            phonies: Vec::new(),
        }
    }

    /// Declares a new phony build step in the target's build state.
    pub fn new_phony(&mut self, deps: Vec<File>) -> File {
        let count = self.phonies.len();
        let mut path = self.target.target_out_dir("phony/", "", "/:");
        path.push('_');
        path.push_str(&count.to_string());
        let phony = File::intern(&path);

        self.phonies.push((phony.clone(), deps));
        phony
    }

    /// Declares a new output file relative to the target's output directory.
    pub fn declare_file(&mut self, name: &str) -> File {
        let mut path = self.target.target_out_dir("", "obj/", "/");
        path.push_str(name);
        let file = File::intern(&path);
        self.unused_declared_outputs.insert(file.clone());
        file
    }
}
