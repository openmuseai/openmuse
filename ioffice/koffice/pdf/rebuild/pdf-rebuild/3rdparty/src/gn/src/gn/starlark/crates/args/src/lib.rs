// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod args;
pub mod errors;
pub mod expand;
pub mod formatter;
#[cfg(test)]
mod tests;
pub mod unpack;
pub use args::{args_methods, Args, FrozenArgs};
pub(crate) use errors::Error;
pub use formatter::Formatter;
pub use unpack::FrozenArgsSequence;
