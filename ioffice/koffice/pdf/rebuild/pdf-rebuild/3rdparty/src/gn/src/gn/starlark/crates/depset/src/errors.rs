// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::{Kind, Order};

/// Errors returned by depset validation and iteration operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
    #[error("conflicting orders: depset has order {order}, but transitive child has order {child_order}")]
    ConflictingOrders { order: Order, child_order: Order },
    #[error("depset elements must be of the same type, expected {expected}, got {got}")]
    DepsetTypeMismatch { expected: Kind, got: Kind },
    #[error("expected a file depset")]
    ExpectedFileDepset,
}

impl From<Error> for starlark::Error {
    fn from(err: Error) -> Self {
        Self::new_other(err)
    }
}

impl starlark::values::UnpackValueError for Error {
    fn into_error(this: Self) -> starlark::Error {
        starlark::Error::new_other(this)
    }
}
