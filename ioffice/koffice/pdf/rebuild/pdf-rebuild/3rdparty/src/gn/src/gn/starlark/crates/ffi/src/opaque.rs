// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::pin::Pin;

/// Trait for types that are opaque, but that we need to know the size of.
///
/// This is usually because we intend to iterate over arrays of them.
pub trait OpaqueSized {
    /// Returns the size of the type in bytes.
    fn size() -> usize;
}

impl OpaqueSized for crate::bridge::Value {
    #[inline(always)]
    fn size() -> usize {
        crate::bridge::ValueSize()
    }
}

/// Marker trait indicating that a type is not an opaque type.
pub trait NonOpaque {}

impl<'a> NonOpaque for crate::bridge::KeyValue<'a> {}
impl<T> NonOpaque for *mut T {}
impl<T> NonOpaque for Pin<&mut T> {}
