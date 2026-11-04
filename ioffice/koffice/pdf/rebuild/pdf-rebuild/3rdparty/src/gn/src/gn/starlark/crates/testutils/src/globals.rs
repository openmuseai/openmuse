// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{environment::GlobalsBuilder, eval::Evaluator, values::Value};
use starlark_derive::starlark_module;
use types::File;

#[starlark_module]
pub fn register_globals(builder: &mut GlobalsBuilder) {
    fn make_file<'v>(
        path: String,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        Ok(eval.heap().alloc(File::intern(&path)))
    }
}
