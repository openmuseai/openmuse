// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::RefCell;

use allocative::Allocative;
use attr::{traits::EvalContextAttrExt, CtxAttr, TargetAttrExt};
use starlark::{
    any::ProvidesStaticType,
    environment::Methods,
    values::{AllocValue, Freeze, FreezeResult, Freezer, Heap, StarlarkValue, Value},
};
use starlark_derive::{starlark_value, NoSerialize};
use types::{CtxMethods, Session};

use crate::FrozenRule;

#[derive(Allocative, NoSerialize)]
pub struct Ctx<'v, C: EvalContextAttrExt> {
    /// Contains ctx.attr/files/file
    attrs: CtxAttr<'v>,
    /// The rule currently being evaluated.
    /// If you have a parent and child rule, this will start as [child], then
    /// when you call ctx.super() it will be [child, parent].
    #[allocative(skip)]
    rule_stack: RefCell<Vec<&'v FrozenRule<C>>>,
}

impl<'v, C: EvalContextAttrExt> Ctx<'v, C> {
    pub fn new(attrs: CtxAttr<'v>, rule: &'v FrozenRule<C>) -> Self {
        Self {
            attrs,
            rule_stack: RefCell::new(vec![rule]),
        }
    }

    /// Runs ctx.super()
    pub fn run_super(
        &self,
        this: Value<'v>,
        eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        let parent = {
            let rule_stack = self.rule_stack.borrow();
            let current = rule_stack.last().expect("rule_stack is never empty");
            current.parent.ok_or(crate::errors::Error::NoParentRule)?
        };
        self.rule_stack.borrow_mut().push(parent);
        let res = eval.eval_function(parent.implementation.to_value(), &[this], &[]);
        self.rule_stack.borrow_mut().pop();
        res
    }
}

impl<'v, C: EvalContextAttrExt> std::fmt::Debug for Ctx<'v, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ctx")
    }
}

impl<'v, C: EvalContextAttrExt> std::fmt::Display for Ctx<'v, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ctx")
    }
}

unsafe impl<'v, C: EvalContextAttrExt> starlark::values::Trace<'v> for Ctx<'v, C> {
    fn trace(&mut self, tracer: &starlark::values::Tracer<'v>) {
        self.attrs.attr.trace(tracer);
        self.attrs.files.trace(tracer);
        self.attrs.file.trace(tracer);
    }
}

unsafe impl<'v, EvalCtx: EvalContextAttrExt> ProvidesStaticType<'v> for Ctx<'v, EvalCtx> {
    type StaticType = Ctx<'static, EvalCtx>;
}

#[starlark_value(type = "ctx")]
impl<'v, C: EvalContextAttrExt + CtxMethods> StarlarkValue<'v> for Ctx<'v, C>
where
    <C::Session as Session>::TargetRef: TargetAttrExt,
{
    type Canonical = Self;

    fn get_methods() -> Option<&'static Methods> {
        Some(<C as CtxMethods>::methods())
    }

    fn get_attr(&self, attribute: &str, _heap: Heap<'v>) -> Option<Value<'v>> {
        match attribute {
            "attr" => Some(self.attrs.attr),
            "files" => Some(self.attrs.files),
            "file" => Some(self.attrs.file),
            _ => None,
        }
    }

    fn dir_attr(&self) -> Vec<String> {
        vec!["attr".to_owned(), "files".to_owned(), "file".to_owned()]
    }
}

impl<'v, C: EvalContextAttrExt + CtxMethods> AllocValue<'v> for Ctx<'v, C>
where
    <C::Session as Session>::TargetRef: TargetAttrExt,
{
    fn alloc_value(self, heap: Heap<'v>) -> Value<'v> {
        heap.alloc_complex(self)
    }
}

impl<'v, C: EvalContextAttrExt + CtxMethods> Freeze for Ctx<'v, C>
where
    <C::Session as Session>::TargetRef: TargetAttrExt,
{
    type Frozen = starlark::values::none::NoneType;

    fn freeze(self, _packer: &Freezer) -> FreezeResult<Self::Frozen> {
        Err(crate::errors::Error::ObjectUnfreezable("ctx").into())
    }
}

#[macro_export]
macro_rules! impl_ctx_methods {
    ($ctx_type:ty) => {
        #[starlark_derive::starlark_module]
        pub fn ctx_methods(builder: &mut starlark::environment::MethodsBuilder) {
            #[starlark(name = "super")]
            fn super_<'v>(
                this: starlark::values::Value<'v>,
                eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
            ) -> starlark::Result<starlark::values::Value<'v>> {
                use starlark::values::ValueLike as _;
                this.downcast_ref::<$crate::Ctx<'v, $ctx_type>>()
                    .unwrap()
                    .run_super(this, eval)
            }
        }

        impl $crate::CtxMethods for $ctx_type {
            fn methods() -> &'static starlark::environment::Methods {
                static RES: starlark::environment::MethodsStatic =
                    starlark::environment::MethodsStatic::new("Ctx", |builder| {
                        ctx_methods(builder);
                    });
                RES.methods()
            }
        }
    };
}
