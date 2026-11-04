// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    fmt::{Display, Formatter},
};

use allocative::Allocative;
use starlark::{
    environment::{Methods, MethodsBuilder, MethodsStatic},
    starlark_complex_value,
    typing::Ty,
    values::{
        type_repr::StarlarkTypeRepr, Freeze, FreezeResult, Freezer, Heap, ProvidesStaticType,
        StarlarkValue, Trace, UnpackValue, Value, ValueLike,
    },
};
use starlark_derive::{starlark_module, starlark_value, Coerce, NoSerialize};
use types::File;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Allocative)]
/// All orderings are guaranteed to be deterministic.
pub enum Order {
    /// Our unspecified order is postorder. However, this should not be relied
    /// upon.
    /// This is named "default" in bazel, but we call it Unspecified to clarify
    /// exactly how it should work from the depset implementation side of
    /// things.
    Unspecified,
    /// Guaranteed to traverse through direct dependencies in left-to-right
    /// order, then transitive in left-to-right order.
    Preorder,
    /// Guaranteed to traverse through transitive dependencies in left-to-right
    /// order, then direct in left-to-right order.
    Postorder,
    /// Our topological order is reverse postorder. However, this should not be
    /// relied upon. Note: Topological order is much slower and less memory
    /// efficient as it requires an intermediate Vec to be created and then
    /// reversed.
    Topological,
}

impl StarlarkTypeRepr for Order {
    type Canonical = String;

    fn starlark_type_repr() -> Ty {
        String::starlark_type_repr()
    }
}

impl<'v> UnpackValue<'v> for Order {
    type Error = starlark::Error;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        match <&'v str>::unpack_value_err(value)? {
            "default" => Ok(Some(Self::Unspecified)),
            "preorder" => Ok(Some(Self::Preorder)),
            "postorder" => Ok(Some(Self::Postorder)),
            "topological" => Ok(Some(Self::Topological)),
            s => Err(crate::Error::InvalidOrder(s.to_owned()).into()),
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // For consistency with bazel, the user sees unspecified as "default".
            Self::Unspecified => write!(f, "default"),
            Self::Preorder => write!(f, "preorder"),
            Self::Postorder => write!(f, "postorder"),
            Self::Topological => write!(f, "topological"),
        }
    }
}

/// The type of elements contained in a depset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, allocative::Allocative)]
pub enum Kind {
    Empty,
    Unknown,
    File,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty"),
            Self::Unknown => write!(f, "unknown"),
            Self::File => write!(f, "File"),
        }
    }
}

/// A generic implementation of a Starlark Depset.
#[derive(Debug, Trace, Coerce, ProvidesStaticType, NoSerialize, Allocative)]
// By implementing coerce, freezing depsets is zero-cost.
// Coerce requires repr(C).
// Starlark knows that it can just do a reinterpret cast of the memory.
#[repr(C)]
pub struct DepsetGen<V> {
    /// The traversal order of this depset.
    pub(crate) order: Order,
    /// The direct elements of this depset. De-duped on creation.
    pub(crate) direct: Vec<V>,
    /// Transitive depsets. Each entry is guaranteed to be a non-empty depset.
    pub(crate) transitive: Vec<V>,
    /// The element type kind of this depset.
    pub(crate) kind: Kind,
    /// The single phony file for this depset. Set for `depset[File]` only.
    /// If the depset only has a single element, it will just be that element.
    pub(crate) phony: Option<File>,
}

impl<'v> Depset<'v> {
    /// Creates a new `Depset` containing `File` elements.
    pub fn new_file_depset<C: types::EvalContext>(
        direct: Vec<File>,
        heap: &Heap<'v>,
        ctx: &C,
    ) -> starlark::Result<Self> {
        let phony = if direct.len() == 1 {
            Some(direct[0].clone())
        } else if !direct.is_empty() {
            Some(ctx.require_rule_impl()?.new_phony(direct.clone()))
        } else {
            None
        };
        Ok(Self {
            order: Order::Unspecified,
            transitive: Vec::new(),
            kind: if direct.is_empty() {
                Kind::Empty
            } else {
                Kind::File
            },
            direct: direct.into_iter().map(|f| heap.alloc(f)).collect(),
            phony,
        })
    }
}

impl<V> Default for DepsetGen<V> {
    fn default() -> Self {
        Self {
            order: Order::Unspecified,
            direct: Vec::new(),
            transitive: Vec::new(),
            kind: Kind::Empty,
            phony: None,
        }
    }
}

impl<V> DepsetGen<V> {
    pub(crate) fn order(&self) -> Order {
        self.order
    }

    pub(crate) fn direct(&self) -> &[V] {
        &self.direct
    }

    pub(crate) fn transitive(&self) -> &[V] {
        &self.transitive
    }

    /// Returns the element kind of this depset.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns the single phony File if this is a file depset containing
    /// exactly one element.
    pub fn phony(&self) -> &Option<File> {
        &self.phony
    }

    /// Returns true if this depset has no elements (its kind is Empty).
    pub fn is_empty(&self) -> bool {
        self.kind == Kind::Empty
    }
}

starlark_complex_value!(pub Depset);

impl<'v, V: ValueLike<'v>> Display for DepsetGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Explicitly DO NOT flatten a depset implicitly, they can get massive.
        // We may consider printing some fields in the future, but for now
        // we'll leave this empty to be safe.
        if self.is_empty() {
            write!(f, "depset([])")
        } else {
            write!(f, "depset(...)")
        }
    }
}

#[starlark_value(type = "depset")]
impl<'v, V: ValueLike<'v>> StarlarkValue<'v> for DepsetGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new("depset", depset_methods);
        Some(RES.methods())
    }

    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl Freeze for Depset<'_> {
    type Frozen = FrozenDepset;

    fn freeze(self, freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(DepsetGen {
            order: self.order,
            direct: self.direct.freeze(freezer)?,
            transitive: self.transitive.freeze(freezer)?,
            kind: self.kind,
            phony: self.phony,
        })
    }
}

/// Declares the Starlark methods for the `depset` type.
#[starlark_module]
pub fn depset_methods(builder: &mut MethodsBuilder) {
    fn to_list<'v>(this: &Depset<'v>) -> starlark::Result<Vec<Value<'v>>> {
        Ok(this.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use testutils::Assert;

    use super::*;
    use crate::{globals::tests::new_assert, UnpackFileDepset};

    #[test]
    fn test_depset_deduplication() {
        let mut a = new_assert();
        let depset_val = a.pass("depset(['a', 'a'])");
        let depset = depset_val.value().downcast_ref::<FrozenDepset>().unwrap();
        assert_eq!(depset.direct().len(), 1);
    }

    #[test]
    fn test_depset_invalid_transitive() {
        let mut a = new_assert();
        a.fail(
            "depset(['c'], transitive=['not a depset'])",
            "Expected value of type `depset` but got `string (repr: \"not a depset\")`",
        );
    }

    #[test]
    fn test_depset_conflicting_orders() {
        let mut a = new_assert();
        a.fail(
            "depset(transitive=[depset(['a'], order='preorder'), depset(['b'], order='postorder')])",
            "conflicting orders: depset has order preorder, but transitive child has order postorder",
        );

        // When outer depset specifies non-default order, we can't reuse the inner
        // depset.
        a.equivalent(
            "depset(transitive=[depset(['c'], transitive=[depset(['a'])])], order='preorder').to_list()",
            "['c', 'a']",
        );
    }

    #[test]
    fn test_depset_type_validation() {
        let mut a = new_assert();
        let frozen_str_depset = a.pass("depset(['a', 'b'])");
        let frozen_file_depset = a.pass("depset([make_file('a.txt'), make_file('b.txt')])");

        a.modify_globals(move |builder: &mut starlark::environment::GlobalsBuilder| {
            builder.set("frozen_str_depset", frozen_str_depset.clone());
            builder.set("frozen_file_depset", frozen_file_depset.clone());
        });

        // Homogeneous File depset.
        a.equivalent(
            "[f.path for f in depset([make_file('a.txt'), make_file('b.txt')]).to_list()]",
            "['a.txt', 'b.txt']",
        );

        // Homogeneous string depset.
        a.equivalent("depset(['a', 'b']).to_list()", "['a', 'b']");

        // Mixing File and String in direct elements.
        a.fail(
            "depset([make_file('a.txt'), 'b'])",
            "depset elements must be of the same type, expected File, got unknown",
        );

        // Mixing String and File in direct elements.
        a.fail(
            "depset(['a', make_file('b.txt')])",
            "depset elements must be of the same type, expected unknown, got File",
        );

        // Mixing File depset and String depset in transitive elements.
        a.fail(
            "depset(transitive=[depset([make_file('a.txt')]), depset(['b'])])",
            "depset elements must be of the same type, expected File, got unknown",
        );

        // Direct File elements mixed with transitive String depset.
        a.fail(
            "depset([make_file('a.txt')], transitive=[depset(['b'])])",
            "depset elements must be of the same type, expected File, got unknown",
        );

        // Transitive depset containing a frozen depset.
        a.equivalent(
            "depset(['c'], transitive=[frozen_str_depset], order='postorder').to_list()",
            "['a', 'b', 'c']",
        );

        // Transitive depset containing a frozen File depset.
        a.equivalent(
            "[f.path for f in depset([make_file('c.txt')], transitive=[frozen_file_depset], order='postorder').to_list()]",
            "['a.txt', 'b.txt', 'c.txt']",
        );
    }

    #[test]
    fn test_depset_starlark_conversions() {
        let mut a = new_assert();
        // Truthiness checks.
        a.equivalent("bool(depset(['a']))", "True");
        a.equivalent("bool(depset())", "False");
        a.equivalent("bool(depset(transitive=[depset()]))", "False");

        a.equivalent("repr(depset())", "\"depset([])\"");
    }

    #[test]
    fn test_depset_phony() {
        let mut a = new_assert();

        // This is to ensure we don't accidentally use File::new.
        // We cannot mix File::new and File::from_rust to get the same file object.
        let f = |s: &str| File::intern(s);

        let mut upto_phony = 0;
        // Collect the phonies we've seen since last time we called new_phonies.
        let mut new_phonies = |a: &Assert| {
            use types::EvalContext as _;
            let phonies = &a.context().require_rule_impl().unwrap().phonies;
            let result = &phonies[upto_phony..];
            upto_phony = phonies.len();
            result.to_vec()
        };

        assert!(UnpackFileDepset::unpack_value_err(a.pass("depset([1])").value()).is_err());

        a.eq("depset()", UnpackFileDepset(None));
        assert_eq!(new_phonies(&a), &[]);

        // Because the outer depset only phonies to one item, we shouldn't make a phony.
        a.eq(
            "depset([make_file('a.txt')])",
            UnpackFileDepset(Some(f("a.txt"))),
        );
        assert_eq!(new_phonies(&a), &[]);

        // We should make a phony here for the inner depset.
        // Because the outer depset only phonies to one phony, it shouldn't make a
        // phony.
        a.eq(
            "depset(transitive = [depset([make_file('a.txt'), make_file('b.txt')])])",
            UnpackFileDepset(Some(f("phony/$TOOLCHAIN/$LABEL_0"))),
        );
        assert_eq!(
            new_phonies(&a),
            &[(f("phony/$TOOLCHAIN/$LABEL_0"), vec![f("a.txt"), f("b.txt")])]
        );
        a.eq(
            "depset([make_file('c.txt')], transitive=[depset([make_file('a.txt'), make_file('b.txt')])])",
            UnpackFileDepset(Some(f("phony/$TOOLCHAIN/$LABEL_2"))),
        );
        assert_eq!(
            new_phonies(&a),
            &[
                (f("phony/$TOOLCHAIN/$LABEL_1"), vec![f("a.txt"), f("b.txt")]),
                (
                    f("phony/$TOOLCHAIN/$LABEL_2"),
                    vec![f("c.txt"), f("phony/$TOOLCHAIN/$LABEL_1")]
                ),
            ]
        );
        assert_eq!(new_phonies(&a), &[]);

        a.eq(
            "new_file_depset([make_file('a.txt')])",
            UnpackFileDepset(Some(f("a.txt"))),
        );
        assert_eq!(new_phonies(&a), &[]);

        a.eq(
            "new_file_depset([make_file('a.txt'), make_file('b.txt')])",
            UnpackFileDepset(Some(f("phony/$TOOLCHAIN/$LABEL_3"))),
        );
        assert_eq!(
            new_phonies(&a),
            &[(f("phony/$TOOLCHAIN/$LABEL_3"), vec![f("a.txt"), f("b.txt")])]
        );
    }
}
