// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Like std::mem::transmute, but can only affect lifetime.
///
/// # Safety
///
/// The caller must ensure that the returned reference is not used after the
/// underlying data is dropped.
pub unsafe fn extend_lifetime<'to, T: ?Sized>(val: &T) -> &'to T {
    // Safety: Transmuting lifetime of reference is unsafe, safety is guaranteed by
    // the caller.
    unsafe { std::mem::transmute(val) }
}
