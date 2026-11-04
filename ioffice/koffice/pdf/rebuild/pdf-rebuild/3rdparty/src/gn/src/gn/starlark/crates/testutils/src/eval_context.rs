// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
use std::{cell::UnsafeCell, collections::HashMap, rc::Rc};

use attr::{Attr, EvalContext as AttrEvalContext, EvalContextAttrExt, Session as AttrSession};
use starlark::{
    values::{FrozenValue, FrozenValueTyped, Heap, ProvidesStaticType, Value},
    Result,
};
use types::{
    CtxState, Label, LabelRef, OutputType, Package, PackageRef, PathResolver, Scope, Session,
    TargetRef,
};

use crate::{FakeSession, FakeTarget, FakeTargetRef};

#[derive(Clone, Default, Debug)]
pub struct FakeScope(HashMap<String, Value<'static>>);

impl Scope for FakeScope {
    fn copy_with<'a, 'v>(&self, kv: impl Iterator<Item = (&'a str, Value<'v>)>) -> Self {
        let mut values = self.0.clone();
        for (k, v) in kv {
            // Safety: Transmuting 'v to 'static is safe because this mock scope
            // is only used in tests, where the Starlark heap outlives the evaluation
            // context.
            let static_val = unsafe { std::mem::transmute::<Value<'v>, Value<'static>>(v) };
            values.insert(k.to_owned(), static_val);
        }
        Self(values)
    }

    fn get<'v>(&self, key: &str, _heap: &Heap<'v>) -> Option<Value<'v>> {
        self.0.get(key).map(|v| {
            // Safety: Shortening the lifetime is always safe.
            unsafe { std::mem::transmute::<Value<'static>, Value<'v>>(*v) }
        })
    }
}

/// A simple implementation of the evaluation context used in Starlark unit
/// tests.
#[derive(allocative::Allocative)]
pub struct FakeEvalContext {
    /// The current package being processed.
    pub package: Package,
    /// The current toolchain.
    pub current_toolchain: Label,
    /// The fake starlark session.
    #[allocative(skip)]
    pub session: Rc<FakeSession>,
    /// The fake path resolver.
    #[allocative(skip)]
    pub path_resolver: PathResolver,
    /// The fake rule state.
    #[allocative(skip)]
    pub rule_state: UnsafeCell<CtxState<FakeTargetRef>>,
    /// The fake scope.
    #[allocative(skip)]
    pub scope: FakeScope,
}

unsafe impl<'v> ProvidesStaticType<'v> for FakeEvalContext {
    type StaticType = Self;
}

impl FakeEvalContext {
    /// Creates a new eval context for a given session.
    pub fn default_rule_impl(session: Rc<FakeSession>) -> Self {
        Self::rule_impl(session.clone(), session.default_target())
    }

    /// Creates a new `FakeEvalContext` for a given package name.
    pub fn new(package: &PackageRef, name: &str) -> Self {
        let session = Rc::new(FakeSession::default());
        let dummy_target = session.insert_empty_target(package, name);
        Self::rule_impl(session, dummy_target)
    }

    pub fn rule_impl(session: Rc<FakeSession>, target: FakeTargetRef) -> Self {
        Self {
            package: target.label().package().to_owned(),
            current_toolchain: target.toolchain().to_owned(),
            session,
            path_resolver: PathResolver::new_for_testing(),
            rule_state: CtxState::new(target).into(),
            scope: FakeScope::default(),
        }
    }
}

impl AttrEvalContext for FakeEvalContext {
    type Scope = FakeScope;
    type Session = FakeSession;

    fn session(&self) -> &Self::Session {
        &self.session
    }

    fn current_package(&self) -> &PackageRef {
        &self.package
    }

    fn path_resolver(&self) -> &PathResolver {
        &self.path_resolver
    }

    fn current_toolchain(&self) -> LabelRef<'_> {
        self.current_toolchain.as_ref()
    }

    fn require_macro(&self) -> Result<&FakeScope> {
        Ok(&self.scope)
    }

    fn require_bzl(&self) -> Result<()> {
        Ok(())
    }

    fn require_rule_impl(&self) -> Result<&mut CtxState<<Self::Session as Session>::TargetRef>> {
        // Safety: The eval context is single-threaded.
        Ok(unsafe { &mut (*self.rule_state.get()) })
    }
}

impl EvalContextAttrExt for FakeEvalContext {
    fn create_target(
        &self,
        target_type: Option<OutputType>,
        target_name: &str,
        scope: &FakeScope,
        rule: FrozenValue,
        attrs: Vec<Attr>,
    ) -> Result<<Self::Session as AttrSession>::TargetRef> {
        let label = Label::new(self.package.clone(), target_name.to_owned());
        let toolchain = self.current_toolchain().to_owned();
        Ok(self.session.insert_target(FakeTarget {
            label,
            toolchain,
            output_type: target_type,
            rule: if rule.is_none() {
                None
            } else {
                let typed =
                    FrozenValueTyped::<rule::FrozenRule<FakeEvalContext>>::new(rule).unwrap();
                Some(typed.as_ref())
            },
            cxx_attrs: scope.0.clone(),
            outputs: vec![],
            attrs,
            dependencies: Default::default(),
        }))
    }
}

rule::impl_ctx_methods!(FakeEvalContext);
