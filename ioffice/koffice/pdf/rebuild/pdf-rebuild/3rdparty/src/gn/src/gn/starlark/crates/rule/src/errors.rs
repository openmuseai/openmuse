// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Errors returned by the GN Starlark rule system.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Rule must be assigned to a global variable to be used")]
    RuleMustBeNamed,
    #[error("Parent must be a rule")]
    ParentMustBeARule,
    #[error("Attribute '{0}' is reserved")]
    ReservedAttribute(String),
    #[error("Rule does not have a parent rule")]
    NoParentRule,
    #[error(
        "The '{0}' object is ephemeral and cannot be stored in providers or other long-lived \
         structures."
    )]
    ObjectUnfreezable(&'static str),
}

impl From<Error> for starlark::Error {
    fn from(err: Error) -> Self {
        starlark::Error::new_other(err)
    }
}

impl From<Error> for starlark::values::FreezeError {
    fn from(err: Error) -> Self {
        Self::new(err.to_string())
    }
}
