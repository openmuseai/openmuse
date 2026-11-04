// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::values::{Heap, Value};

/// This trait is an abstraction over GN's Scope object.
///
/// We intentionally expose, rather than methods that C++ scope objects support,
/// the API we actually wish to use from rust, which may be an abstraction
/// over that.
pub trait Scope {
    /// Creates a copy of the scope with some additional values set.
    fn copy_with<'a, 'v>(&self, kv: impl Iterator<Item = (&'a str, Value<'v>)>) -> Self;

    /// Retrieves a value from the key-value store.
    /// May allocate the value it retrieves on the heap.
    fn get<'v>(&self, key: &str, heap: &Heap<'v>) -> Option<Value<'v>>;
}
