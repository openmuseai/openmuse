// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{
    collections::StarlarkHasher,
    environment::{Methods, MethodsBuilder, MethodsStatic},
    starlark_simple_value,
    values::{
        Freeze, FreezeResult, Freezer, ProvidesStaticType, StarlarkValue, Trace, Tracer,
        ValueLike as _,
    },
};
use starlark_derive::{starlark_module, starlark_value, NoSerialize};

use crate::{LabelRef, Package, PackageRef};

/// A Label represents a target label, e.g., "//foo/bar:baz".
/// Note that unlike a regular GN label, this label does *not* include a
/// toolchain.
///
/// In Starlark, a fully qualified GN label is represented as a tuple
/// (label, toolchain). "//foo:bar(//toolchain:name)" would thus convert to
/// (Label("//foo:bar"), Label("//toolchain:name")).
#[derive(Clone, Eq, PartialEq, Hash, ProvidesStaticType, NoSerialize, allocative::Allocative)]
pub struct Label {
    /// The package part of the label.
    package: Package,
    /// The name part of the label.
    name: String,
}

starlark_simple_value!(Label);

impl Label {
    /// Creates a new `Label`.
    pub fn new(package: Package, name: String) -> Self {
        Self { package, name }
    }

    /// Returns a `LabelRef` referencing the data inside this label.
    pub fn as_ref(&self) -> LabelRef<'_> {
        LabelRef {
            package: &self.package,
            name: &self.name,
        }
    }

    /// Returns the package of this label.
    pub fn package(&self) -> &PackageRef {
        &self.package
    }

    /// Returns the name of this label.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Parses a label guaranteed to start with a "//"
    pub(crate) fn parse_absolute(s: &str) -> starlark::Result<Self> {
        let mut iter = s.split(':');
        // Verify that there is exactly one colon.
        match (iter.next(), iter.next(), iter.next()) {
            (Some(package), Some(name), None) if !name.is_empty() => Ok(Self {
                // Safety: already checked.
                package: unsafe { PackageRef::new_unchecked(package) }.to_owned(),
                name: name.to_owned(),
            }),
            // In GN, this refers to the file foo in the // directory in file
            // contexts, and //foo:foo in label contexts.
            // In Bazel, files and labels can be mixed, and thus it always
            // resolves to the label //foo:foo.
            // We explicitly ban this syntax to reduce confusion.
            // We may choose to change this later, but it's easier to allow
            // previously banned things than to ban previously allowed things.
            _ => Err(crate::Error::InvalidAbsoluteLabel(s.to_owned()).into()),
        }
    }

    /// Parses a label guaranteed to start with a ":"
    pub(crate) fn parse_relative(s: &str, relative_to: &PackageRef) -> starlark::Result<Self> {
        let name = &s[1..];
        if name.is_empty() {
            return Err(crate::Error::NotALabel(s.to_owned()).into());
        }
        if name.contains(':') {
            return Err(crate::Error::ColonInRelativeLabel(s.to_owned()).into());
        }
        Ok(Self {
            package: relative_to.to_owned(),
            name: name.to_owned(),
        })
    }

    /// Parses a label. If it is relative, it is parsed as relative to the given
    /// package.
    pub fn parse(s: &str, relative_to: &PackageRef) -> starlark::Result<Self> {
        if s.starts_with("//") {
            Self::parse_absolute(s)
        } else if s.starts_with(':') {
            Self::parse_relative(s, relative_to)
        } else {
            Err(crate::Error::NotALabel(s.to_owned()).into())
        }
    }

    /// Same as parse, but if it doesn't start with "//" or ":", returns None
    /// instead of erroring out.
    pub fn parse_maybe_label(s: &str, relative_to: &PackageRef) -> starlark::Result<Option<Self>> {
        Ok(if s.starts_with("//") {
            Some(Self::parse_absolute(s)?)
        } else if s.starts_with(':') {
            Some(Self::parse_relative(s, relative_to)?)
        } else {
            None
        })
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl std::fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.as_ref(), f)
    }
}

unsafe impl<'v> Trace<'v> for Label {
    fn trace(&mut self, _tracer: &Tracer<'v>) {}
}

impl Freeze for Label {
    type Frozen = Self;

    fn freeze(self, _freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(self)
    }
}

#[starlark_value(type = "Label")]
impl<'v> StarlarkValue<'v> for Label {
    fn get_methods() -> Option<&'static Methods>
    where
        Self: Sized,
    {
        static RES: MethodsStatic = MethodsStatic::new("Label", label_methods);
        Some(RES.methods())
    }

    fn write_hash(&self, hasher: &mut StarlarkHasher) -> starlark::Result<()> {
        use std::hash::Hash as _;
        self.hash(hasher);
        Ok(())
    }

    fn equals(&self, other: starlark::values::Value<'v>) -> starlark::Result<bool> {
        Ok(other.downcast_ref::<Self>().is_some_and(|o| self == o))
    }

    fn collect_repr(&self, collector: &mut String) {
        use std::fmt::Write as _;
        write!(collector, "{:?}", self.as_ref()).unwrap();
    }

    fn collect_str(&self, collector: &mut String) {
        use std::fmt::Write as _;
        write!(collector, "{}", self.as_ref()).unwrap();
    }
}

#[starlark_module]
fn label_methods(methods: &mut MethodsBuilder) {
    #[starlark(attribute)]
    fn package<'v>(this: &'v Label) -> starlark::Result<&'v str> {
        Ok(this.package.as_str())
    }

    #[starlark(attribute)]
    fn name<'v>(this: &'v Label) -> starlark::Result<&'v str> {
        Ok(&this.name)
    }
}

// Note: We intentionally *do not* expose a Label() constructor to starlark.
// Where the label is relative to is confusing for the average user.
// Instead, if we need this in the future, we may expose:
// * native.package_relative_label() for macros
// * ctx.package_relative_label() for rules
// * Label() for build files only.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_parsing() {
        let current_pkg = PackageRef::new("//foo/bar").unwrap();

        let lbl = Label::parse("//foo/bar:baz", PackageRef::root()).unwrap();
        assert_eq!(lbl.to_string(), "//foo/bar:baz");

        let lbl2 = Label::parse(":baz", current_pkg).unwrap();
        assert_eq!(lbl2.to_string(), "//foo/bar:baz");

        // Test non-absolute and non-colon starting fails
        assert!(Label::parse("foo", current_pkg).is_err());
        assert!(Label::parse(":", current_pkg).is_err());
        assert!(Label::parse("//foo/bar:", current_pkg).is_err());
    }

    #[test]
    fn test_label_attributes() {
        let lbl = Label::parse("//foo/bar:baz", PackageRef::root()).unwrap();
        assert_eq!(lbl.package.as_ref(), PackageRef::new("//foo/bar").unwrap());
        assert_eq!(lbl.name, "baz");
        assert_eq!(format!("{lbl:?}"), "Label(\"//foo/bar:baz\")");
    }

    #[test]
    fn test_label_starlark_api() {
        let mut a = starlark::assert::Assert::new();
        a.globals_add(move |builder| {
            builder.set(
                "my_label",
                Label::new(
                    PackageRef::new("//foo/bar").unwrap().to_owned(),
                    "baz".to_owned(),
                ),
            );
        });
        a.eq("my_label.package", "\"//foo/bar\"");
        a.eq("my_label.name", "\"baz\"");
        a.eq("str(my_label)", "\"//foo/bar:baz\"");
        a.eq("repr(my_label)", "'Label(\"//foo/bar:baz\")'");
    }
}
