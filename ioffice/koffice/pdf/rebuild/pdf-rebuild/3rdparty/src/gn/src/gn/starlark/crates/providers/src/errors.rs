// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Errors returned by provider validation, unpacking, and extraction.
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("The elements of provider fields must be strings")]
    FieldsMustBeStrings,
    #[error("Provider fields must be an iterable")]
    FieldsMustBeIterable,
    #[error("Provider name must end with 'Info' (got '{0}')")]
    InvalidProviderName(String),
    #[error("Duplicate field name: {0}")]
    DuplicateFieldName(String),
    #[error("The result of provider() must be assigned to a variable")]
    ProviderNotExported,
    #[error("Cannot construct values of non-frozen provider type")]
    ProviderNotFrozen,
    #[error("Duplicate provider: {0}")]
    DuplicateProvider(String),
    #[error("DefaultInfo.files must be a depset of files, got {0}")]
    DefaultInfoFilesMustBeFileDepset(String),
    #[error("GnInputsInfo.files must be a depset of files, got {0}")]
    GnInputsInfoFilesMustBeFileDepset(String),
    #[error("GnSubstitutionsInfo.substitutions must be a struct, got {0}")]
    GnSubstitutionsInfoSubstitutionsMustBeStruct(String),
}

impl From<Error> for starlark::values::FreezeError {
    fn from(err: Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<Error> for starlark::Error {
    fn from(err: Error) -> Self {
        Self::new_other(err)
    }
}
