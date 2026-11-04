// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod allow_files;
pub mod attr;
pub mod cfg;
pub mod ctx;
pub mod errors;
pub mod globals;
pub mod schema;
pub mod traits;
pub mod value;

pub use allow_files::AllowFiles;
pub use attr::{Attr, LabelOrFile};
pub use cfg::AttrCfg;
pub use ctx::{CtxAttr, CtxAttrSchema};
pub(crate) use errors::Error;
pub use globals::{AttrModule, AttrSpecArgs};
pub use schema::{AllowFilesSchema, AttrKind, AttrSchema};
pub use traits::{EvalContext, EvalContextAttrExt, Session, TargetAttrExt, TargetRef};
