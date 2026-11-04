// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::ops::Deref;

use starlark::values::{FrozenHeapRef, OwnedFrozenValue, UnpackValue};

/// A type-safe wrapper that holds an unpacked Starlark value and the
/// FrozenHeapRef that keeps the underlying memory alive.
pub struct UnpackedOwnedValue<T: 'static> {
    value: T,
    #[allow(dead_code)] // Keeps the heap alive.
    heap: FrozenHeapRef,
}

impl<T: 'static> Deref for UnpackedOwnedValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> TryFrom<OwnedFrozenValue> for UnpackedOwnedValue<T>
where
    T: for<'v> UnpackValue<'v> + 'static,
{
    type Error = starlark::Error;

    fn try_from(val: OwnedFrozenValue) -> Result<Self, Self::Error> {
        let heap = val.owner().clone();
        // Safety: We extract the raw FrozenValue from OwnedFrozenValue. This is safe
        // because we clone the heap ref and store it in UnpackedOwnedValue,
        // ensuring the heap outlives T.
        let value = unsafe { val.unchecked_frozen_value() }.to_value();
        Ok(Self {
            value: T::unpack_value_err(value)?,
            heap,
        })
    }
}
