// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use allocative::Allocative;
use starlark::{
    typing::Ty,
    values::{type_repr::StarlarkTypeRepr, UnpackValue, Value},
};

/// The rust type for the starlark value passed to attr.label(cfg = ...)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Allocative)]
pub enum AttrCfg {
    CurrentToolchain,
}

impl StarlarkTypeRepr for AttrCfg {
    type Canonical = String;

    fn starlark_type_repr() -> Ty {
        String::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for AttrCfg {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        match value.unpack_str() {
            Some("target") => Ok(Some(Self::CurrentToolchain)),
            Some(s) => Err(crate::Error::ConfigTransitionNotImplemented(s.to_owned()).into()),
            None => Ok(None),
        }
    }
}
