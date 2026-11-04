// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::PackageRef;

/// A package, conceptually, is a directory in the source tree.
/// Always prefer &PackageRef over Package to avoid copies.
///
/// It represents the same concept as a SourceDir, but differs in its internal
/// representation: unlike SourceDir, it *never* ends with a '/'.
///
/// A package *always* starts with //, and may be either the root package "//",
/// or a subdirectory "//foo/bar".
#[derive(Debug, Clone, Eq, PartialEq, Hash, allocative::Allocative)]
pub struct Package(pub(crate) String);

impl std::ops::Deref for Package {
    type Target = PackageRef;

    fn deref(&self) -> &Self::Target {
        // Safety: already validated
        unsafe { PackageRef::new_unchecked(&self.0) }
    }
}

impl AsRef<PackageRef> for Package {
    fn as_ref(&self) -> &PackageRef {
        self
    }
}

impl std::borrow::Borrow<PackageRef> for Package {
    fn borrow(&self) -> &PackageRef {
        self
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
