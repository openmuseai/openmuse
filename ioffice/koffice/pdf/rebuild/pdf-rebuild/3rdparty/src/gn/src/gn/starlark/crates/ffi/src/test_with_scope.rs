// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// A test fixture wrapper for a C++ GN TestWithScope.
pub struct TestWithScope {
    inner: cxx::UniquePtr<crate::bridge::TestWithScope>,
}

impl Default for TestWithScope {
    fn default() -> Self {
        Self::new()
    }
}

impl TestWithScope {
    /// Creates a new GN TestWithScope fixture.
    pub fn new() -> Self {
        Self {
            inner: crate::bridge::NewTestWithScope(),
        }
    }

    /// Accesses the scope owned by the test fixture.
    pub fn scope(&mut self) -> &mut crate::Scope {
        // Safety: Scope pointer from TestWithScope is always valid and non-null.
        unsafe { self.inner.pin_mut().scope_cxx().as_mut() }.unwrap()
    }
}
