// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Low-level FFI bindings and types for interoperating between the C++
//! GN codebase and the Rust Starlark interpreter crates using `cxx`.
//!
//! **Architecture & Bridge design**
//!
//! Rather than manually compiling raw `extern "C"` FFI wrappers, all FFI
//! boundary mappings are consolidated within `bridge.rs` under a single
//! `#[cxx::bridge]` module.
//!
//! This module is then transpiled with cxxbridge into a C++ header and source
//! file.
//!
//! Safe APIs for C++ types are then exposed in the impl functions for each of
//! these types in their own files.
mod bridge;
mod err;
mod errors;
mod eval_context;
mod label;
mod mutability;
mod opaque;
mod output_file;
mod scope;
mod session;
mod settings;
mod slice;
mod target_ref;
mod test_with_scope;
mod value;

pub use bridge::{Err, KeyValue, Label, OutputFile, Scope, Settings, SourceDir, Value, ValueType};
pub use mutability::Immutable;
pub use opaque::{NonOpaque, OpaqueSized};
pub use scope::OwnedScope;
pub use session::Session;
pub use slice::{OwnedSlice, Slice};
pub use test_with_scope::TestWithScope;
