// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{
    typing::Ty,
    values::{type_repr::StarlarkTypeRepr, FrozenValueTyped, UnpackValue, Value, ValueTyped},
};
use types::File;

use crate::depset::{Depset, FrozenDepset};

/// Helper type to unpack a Starlark `Value` as either a mutable or frozen
/// `Depset`.
pub struct UnpackDepset<'v> {
    depset: &'v Depset<'v>,
    value: Value<'v>,
}

impl<'v> StarlarkTypeRepr for UnpackDepset<'v> {
    type Canonical = <&'v Depset<'v> as StarlarkTypeRepr>::Canonical;

    fn starlark_type_repr() -> Ty {
        <&'v Depset<'v> as StarlarkTypeRepr>::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for UnpackDepset<'v> {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        if let Some(frozen) = value
            .unpack_frozen()
            .and_then(FrozenValueTyped::<FrozenDepset>::new)
        {
            let depset: &'v Depset<'v> = starlark::coerce::coerce(frozen.as_ref());
            Ok(Some(UnpackDepset { depset, value }))
        } else {
            let mutable = ValueTyped::<Depset<'v>>::new_err(value)?;
            let depset: &'v Depset<'v> = mutable.as_ref();
            Ok(Some(UnpackDepset { depset, value }))
        }
    }
}

use std::ops::Deref;

impl<'v> Deref for UnpackDepset<'v> {
    type Target = Depset<'v>;

    fn deref(&self) -> &Self::Target {
        self.depset
    }
}
impl<'v> UnpackDepset<'v> {
    /// Returns a reference to the unpacked `Depset`.
    pub fn depset(&self) -> &'v Depset<'v> {
        self.depset
    }

    /// Returns the raw `Value` that was unpacked.
    pub fn value(&self) -> Value<'v> {
        self.value
    }
}

/// Helper to unpack a depset and retrieve its single `File` if it represents a
/// phony file depset.
#[derive(Debug, PartialEq, Eq)]
pub struct UnpackFileDepset(pub Option<File>);

impl StarlarkTypeRepr for UnpackFileDepset {
    type Canonical = Self;

    fn starlark_type_repr() -> Ty {
        Ty::any()
    }
}

impl<'v> UnpackValue<'v> for UnpackFileDepset {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        let depset = UnpackDepset::unpack_value_err(value)?;
        if depset.phony().is_some() || depset.is_empty() {
            Ok(Some(Self(depset.depset.phony().clone())))
        } else {
            Err(crate::errors::Error::ExpectedFileDepset.into())
        }
    }
}
