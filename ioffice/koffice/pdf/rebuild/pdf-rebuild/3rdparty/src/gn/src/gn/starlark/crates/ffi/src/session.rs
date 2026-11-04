// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::pin::Pin;

use loader::FileLoader;
use starlark::environment::{FrozenModule, Globals};
use types::{
    util::extend_lifetime, Label, LabelRef, PackageRef, PathResolver, Session as TypesSession,
};

use crate::errors::Error;

/// Represents a Starlark evaluation session exposed to C++ via FFI.
pub struct Session {
    loader: FileLoader,
    pub(crate) path_resolver: PathResolver,
    globals: Globals,
}

fn make_attr_schema<'v>(
    kind: attr::AttrKind,
    args: attr::AttrSpecArgs<'v>,
    eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
) -> starlark::Result<starlark::values::Value<'v>> {
    use types::{EvalContext as _, EvaluatorContextExt as _};
    let ctx = eval.context::<crate::eval_context::EvalContext>();
    attr::AttrSchema::create(
        kind,
        args,
        ctx.current_package(),
        ctx.path_resolver(),
        &eval.heap(),
    )
}

fn build_globals() -> Globals {
    let mut builder = starlark::environment::GlobalsBuilder::standard();
    builder.set("attr", attr::AttrModule { make_attr_schema });
    providers::globals::register_providers(&mut builder);
    depset::depset_globals!(&mut builder, crate::eval_context::EvalContext);
    rule::register_rule_globals!(&mut builder, crate::eval_context::EvalContext);
    builder.build()
}

impl Session {
    /// Creates a new `Session`.
    pub fn from_resolver(path_resolver: PathResolver) -> Self {
        Self {
            loader: FileLoader::default(),
            path_resolver,
            globals: build_globals(),
        }
    }

    /// Associated function for C++ constructor.
    pub fn new(source_root: &str, source_root_rel: &str) -> Box<Self> {
        Box::new(Self::from_resolver(PathResolver::new(
            std::path::PathBuf::from(source_root),
            source_root_rel.to_owned(),
        )))
    }

    /// Associated function for C++ constructor.
    pub fn new_for_testing() -> Box<Self> {
        Box::new(Self::from_resolver(PathResolver::new_for_testing()))
    }

    fn load(&'static self, label: LabelRef<'_>) -> starlark::Result<FrozenModule> {
        self.loader
            .load(label, &self.path_resolver, &self.globals, &|pkg| {
                // Safety: The package reference is guaranteed to live as long as the
                // eval context.
                crate::eval_context::EvalContext::new_bzl_file(self, unsafe {
                    extend_lifetime(pkg)
                })
            })
    }

    /// Loads a Starlark module and populates a scope with values by key.
    pub fn load_values(
        &'static self,
        label: &str,
        relative_to: &str,
        keys: &[&str],
        mut scope: Pin<&mut crate::bridge::Scope>,
        settings: &crate::Settings,
        origin: crate::bridge::ParseNodePtr,
        mut err: Pin<&mut crate::bridge::Err>,
    ) {
        err.as_mut().handle((|| -> starlark::Result<()> {
            let label = Label::parse(label, PackageRef::new(relative_to)?)?;
            let module = self.load(label.as_ref())?;

            for key in keys {
                let value = module
                    .get(key)
                    .map_err(|_| Error::KeyNotFound(key.to_string(), label.clone()))?;
                let mut cxx_value = crate::bridge::SetValue(scope.as_mut(), key, origin);
                cxx_value.as_mut().assign(value.value(), settings, origin);
            }
            Ok(())
        })());
    }
}

impl TypesSession for Session {
    type TargetRef = crate::target_ref::TargetRef;

    fn get_target(&self, _label: LabelRef<'_>, _toolchain: LabelRef<'_>) -> Self::TargetRef {
        todo!()
    }

    fn register_dependency<'a>(
        &self,
        _source: Self::TargetRef,
        _label: LabelRef<'a>,
        _toolchain: LabelRef<'a>,
    ) {
        todo!()
    }
}
