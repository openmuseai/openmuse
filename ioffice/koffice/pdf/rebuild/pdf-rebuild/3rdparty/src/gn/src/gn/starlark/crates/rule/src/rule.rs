// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{cell::OnceCell, fmt, marker::PhantomData};

use allocative::Allocative;
use attr::{traits::EvalContextAttrExt, AttrSchema, CtxAttrSchema};
use starlark::{
    any::ProvidesStaticType,
    collections::SmallMap,
    eval::{Arguments, Evaluator, ParametersSpec, ParametersSpecParam},
    values::{
        Freeze, FreezeResult, Freezer, FrozenHeap, FrozenValue, FrozenValueTyped, StarlarkValue,
        Value, ValueLike,
    },
};
use starlark_derive::{starlark_value, NoSerialize};
use types::EvaluatorContextExt;
pub use types::OutputType;

use crate::frozen_rule::FrozenRule;

/// Representation of a Starlark rule object.
///
/// Rules represent target definitions in GN (e.g., `executable`, `source_set`,
/// or custom rules declared via `rule()`).
///
/// Note that a rule is unusable - it must be frozen before being used.
///
/// The generic parameter `C` represents the execution context. This is required
/// because although we don't store it in the rule object itself, `invoke`
/// requires a concrete execution context.
#[derive(NoSerialize, Allocative)]
pub struct Rule<'v, C: EvalContextAttrExt> {
    schema: CtxAttrSchema,
    // Filled after `export_as` is called.
    // Contains the name of the rule, and the signature required to call it.
    once_named: OnceCell<(String, ParametersSpec<FrozenValue>)>,
    builtin: Option<OutputType>,
    implementation: Value<'v>,
    parent: Option<Value<'v>>,
    _phantom: PhantomData<C>,
}

// Safety: Rule does not automatically derive Send and Sync because of C.
// But it only contains C inside PhantomData, so Send and Sync are actually
// safe.
unsafe impl<'v, C: EvalContextAttrExt> Send for Rule<'v, C> {}
unsafe impl<'v, C: EvalContextAttrExt> Sync for Rule<'v, C> {}

unsafe impl<'v, C: EvalContextAttrExt> ProvidesStaticType<'v> for Rule<'v, C> {
    type StaticType = Rule<'static, C>;
}

unsafe impl<'v, C: EvalContextAttrExt> starlark::values::Trace<'v> for Rule<'v, C> {
    fn trace(&mut self, tracer: &starlark::values::Tracer<'v>) {
        self.implementation.trace(tracer);
        self.parent.trace(tracer);
    }
}

impl<'v, C: EvalContextAttrExt> fmt::Debug for Rule<'v, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Some reserved attributes are currently in use (eg. name).
/// Others are reserved for future use, so we disallow them in case we want to
/// support them in the future.
const RESERVED_ATTRS: &[&str] = &[
    "deps",
    "name",
    "public",
    "public_deps",
    "sources",
    "testonly",
    "visibility",
];

fn merge_attrs(
    parent_attrs: &SmallMap<String, AttrSchema>,
    mut child_attrs: SmallMap<String, AttrSchema>,
) -> Result<SmallMap<String, AttrSchema>, starlark::Error> {
    let mut merged = SmallMap::with_capacity(parent_attrs.len() + child_attrs.len());

    // Order matters. Parent attributes need to be added first. This allows the
    // static_library builtin, for example, to declare attrs = `{"public": ...,
    // "sources": ...}`, then it can simply set `values = [target.public(),
    // target.sources(), starlark parameters]`
    for (name, parent_schema) in parent_attrs {
        if let Some(child_schema) = child_attrs.shift_remove(name) {
            parent_schema.check_override(name, &child_schema)?;
            merged.insert(name.clone(), child_schema);
        } else {
            merged.insert(name.clone(), parent_schema.clone());
        }
    }

    for (name, child_schema) in child_attrs {
        merged.insert(name, child_schema);
    }

    Ok(merged)
}

impl<'v, C: EvalContextAttrExt> Rule<'v, C> {
    /// Creates a new `Rule` with the given attributes, implementation, and
    /// parent.
    pub fn new(
        attrs: SmallMap<String, AttrSchema>,
        builtin: Option<OutputType>,
        parent: Option<Value<'v>>,
        implementation: Value<'v>,
        frozen_heap: &FrozenHeap,
    ) -> Result<Self, starlark::Error> {
        for name in attrs.keys() {
            if RESERVED_ATTRS.contains(&name.as_str()) {
                return Err(crate::Error::ReservedAttribute(name.clone()).into());
            }
        }

        let mut builtin = builtin;
        let parent_schema = match parent {
            Some(parent_val) => {
                if let Some(rule) = parent_val.downcast_ref::<FrozenRule<C>>() {
                    if builtin.is_none() {
                        builtin = rule.builtin;
                    }
                    Some(&rule.schema)
                } else if let Some(rule) = parent_val.downcast_ref::<Rule<'v, C>>() {
                    if builtin.is_none() {
                        builtin = rule.builtin;
                    }
                    Some(&rule.schema)
                } else {
                    return Err(crate::Error::ParentMustBeARule.into());
                }
            },
            None => None,
        };

        let attrs = match parent_schema {
            Some(parent_schema) => merge_attrs(parent_schema.attrs(), attrs)?,
            None => attrs,
        };

        let schema = CtxAttrSchema::new(attrs, builtin, frozen_heap);
        Ok(Self {
            schema,
            once_named: Default::default(),
            builtin,
            implementation,
            parent,
            _phantom: PhantomData,
        })
    }
}

pub(crate) fn build_signature(
    name: &str,
    schema: &CtxAttrSchema,
    is_builtin: bool,
) -> ParametersSpec<FrozenValue> {
    let named_only: Vec<_> = std::iter::once(("name", ParametersSpecParam::Required))
        .chain(
            schema
                .attrs()
                .iter()
                .map(|(k, attr)| (k.as_str(), attr.as_param_spec())),
        )
        .collect();

    ParametersSpec::new_parts(name, vec![], vec![], false, named_only, is_builtin)
}

#[starlark_value(type = "rule")]
impl<'v, C: EvalContextAttrExt> StarlarkValue<'v> for Rule<'v, C>
where
    Self: ProvidesStaticType<'v>,
{
    type Canonical = FrozenRule<C>;

    fn export_as(
        &self,
        variable_name: &str,
        _eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<()> {
        let signature = build_signature(variable_name, &self.schema, self.builtin.is_some());
        let _ = self.once_named.set((variable_name.to_owned(), signature));
        Ok(())
    }

    fn invoke(
        &self,
        _me: Value<'v>,
        _args: &Arguments<'v, '_>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        eval.context::<C>().require_macro()?;
        // Rules can only be created in bzl files, so if we are in a macro, the rule
        // must already be frozen. In that case, we would be calling
        // FrozenRule::invoke instead.
        unreachable!();
    }
}

impl<'v, C: EvalContextAttrExt> Freeze for Rule<'v, C> {
    type Frozen = FrozenRule<C>;

    fn freeze(self, freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        let (name, signature) = self
            .once_named
            .into_inner()
            .ok_or(crate::Error::RuleMustBeNamed)?;
        Ok(FrozenRule {
            schema: self.schema,
            builtin: self.builtin,
            implementation: self.implementation.freeze(freezer)?,
            name,
            signature,
            parent: match self.parent {
                Some(p) => {
                    let frozen_parent = p.freeze(freezer)?;
                    let typed = FrozenValueTyped::<FrozenRule<C>>::new(frozen_parent).unwrap();
                    Some(typed.as_ref())
                },
                None => None,
            },
            _phantom: PhantomData,
        })
    }
}

impl<'v, C: EvalContextAttrExt> starlark::values::AllocValue<'v> for Rule<'v, C> {
    #[inline]
    fn alloc_value(self, heap: starlark::values::Heap<'v>) -> starlark::values::Value<'v> {
        heap.alloc_complex(self)
    }
}

impl<'v, C: EvalContextAttrExt> fmt::Display for Rule<'v, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((name, _)) = self.once_named.get() {
            write!(f, "<rule: {name}>")
        } else {
            write!(f, "<anonymous rule>")
        }
    }
}
