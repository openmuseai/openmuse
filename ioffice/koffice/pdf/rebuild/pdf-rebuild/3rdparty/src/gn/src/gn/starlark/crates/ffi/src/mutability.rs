// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ops::Deref;

/// A wrapper that enforces compile-time immutability for its inner type.
///
/// It only implements `Deref`, meaning that once wrapped, callers can only
/// access read-only methods taking `&self`.
pub struct Immutable<T>(T);

impl<T> From<T> for Immutable<T> {
    #[inline(always)]
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
