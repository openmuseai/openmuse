// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Errors returned by Starlark module loading and evaluation.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    ReadFailed(String),
    #[error("cycle detected: {0:?}")]
    CycleDetected(Vec<String>),
}

impl From<Error> for starlark::Error {
    fn from(err: Error) -> Self {
        Self::new_other(err)
    }
}
