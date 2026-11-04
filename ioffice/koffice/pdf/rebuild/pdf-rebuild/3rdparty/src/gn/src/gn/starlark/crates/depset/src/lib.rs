// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod depset;
pub mod errors;
mod globals;
mod iter;
mod unpack;

pub use depset::{Depset, DepsetGen, FrozenDepset, Kind, Order};
pub use errors::Error;
pub use globals::{__private, depset_constructor};
pub use unpack::{UnpackDepset, UnpackFileDepset};
