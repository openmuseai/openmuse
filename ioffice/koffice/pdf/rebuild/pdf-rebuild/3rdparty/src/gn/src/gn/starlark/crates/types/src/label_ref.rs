// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::{Label, PackageRef};

/// A borrowed reference to a `Label`.
///
/// We implement this instead of using &Label, because this allows us to
/// convert C++ labels to rust labels without any copying.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LabelRef<'a> {
    /// The package part of the label reference.
    pub(crate) package: &'a PackageRef,
    /// The name part of the label reference.
    pub(crate) name: &'a str,
}

impl<'a> LabelRef<'a> {
    /// Creates a new `LabelRef`.
    pub fn new(package: &'a PackageRef, name: &'a str) -> Self {
        Self { package, name }
    }

    /// Returns the package of this label reference.
    pub fn package(&self) -> &PackageRef {
        self.package
    }

    /// Returns the name of this label reference.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Converts this reference into an owned `Label`.
    pub fn to_owned(&self) -> Label {
        Label::new(self.package.to_owned(), self.name.to_owned())
    }
}

impl std::fmt::Display for LabelRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.package, self.name)
    }
}

impl std::fmt::Debug for LabelRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Label(\"{}:{}\")", self.package, self.name)
    }
}
