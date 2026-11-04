// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::rc::Rc;

use starlark::{
    environment::{FrozenModule, GlobalsBuilder},
    values::UnpackValue,
};
use types::{EvalContext, EvaluatorContextExt, Label, PathResolver, UnpackedOwnedValue};

use crate::{register_globals, FakeEvalContext, FakeSession};

type GlobalsConfig = Box<dyn Fn(&mut GlobalsBuilder)>;

/// A simple wrapper around starlark::Assert that provides fake evaluation
/// contexts.
pub struct Assert {
    assert: starlark::assert::Assert<'static>,
    context: Box<FakeEvalContext>,
    globals_configs: Vec<GlobalsConfig>,
}

impl Default for Assert {
    fn default() -> Self {
        Self::new(FakeEvalContext::default_rule_impl(Rc::new(
            FakeSession::default(),
        )))
    }
}

impl Assert {
    /// Creates a new `Assert` helper instance with the given context.
    pub fn new(context: FakeEvalContext) -> Self {
        let mut assert = starlark::assert::Assert::new();
        // By default, starlark runs the code 3 times (with always GC, auto GC, and
        // disabled GC) to check for GC bugs. Because the EvalContext can be
        // mutated by the evaluator, running 3 times could cause state mutations
        // to happen three times, messing with tests. Calling `always_gc()`
        // forces the framework to run the code only once (specifically,
        // under the "always GC" configuration), ensuring state is mutated only once.
        assert.always_gc();
        let context = Box::new(context);
        let context_ptr = &*context as *const FakeEvalContext;

        assert.setup_eval(move |eval| {
            // Safety: The context is owned by Assert, which outlives the evaluator run.
            // Since all evaluation methods on Assert require `&mut self`, this guarantees
            // exclusive access to `context` when the evaluator runs, so dereferencing
            // this pointer is safe and does not alias.
            eval.set_context(unsafe { &*context_ptr });
        });

        let mut s = Self {
            assert,
            context,
            globals_configs: vec![],
        };
        s.modify_globals(register_globals);
        s
    }

    /// Creates a new `Assert` helper instance with rule evaluation globals and
    /// builtins registered.
    pub fn new_rule_assert() -> Self {
        let mut assert = Self::default();
        assert.modify_globals(|builder| {
            rule::register_rule_globals!(builder, FakeEvalContext);
            providers::register_providers(builder);

            fn make_attr_schema<'v>(
                kind: attr::AttrKind,
                args: attr::AttrSpecArgs<'v>,
                eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
            ) -> starlark::Result<starlark::values::Value<'v>> {
                let context: &FakeEvalContext = eval.context();
                attr::AttrSchema::create(
                    kind,
                    args,
                    context.current_package(),
                    &context.path_resolver,
                    &eval.heap(),
                )
            }
            builder.set("attr", attr::AttrModule { make_attr_schema });
        });
        let builtins = rule::register_builtin_rules::<FakeEvalContext>();
        assert.module_add(builtins);
        assert
    }

    /// Adds a modifier to globals.
    /// This modifier is applied after all existing modifiers.
    pub fn modify_globals(&mut self, f: impl Fn(&mut GlobalsBuilder) + 'static) {
        self.globals_configs.push(Box::new(f));
        // globals_add overwrites all previous calls to globals_add.
        self.assert.globals_add(|builder| {
            for config in &self.globals_configs {
                config(builder);
            }
        });
    }

    pub fn module_add(&mut self, module: FrozenModule) {
        let name = module.frozen_heap().name().unwrap().to_string();
        self.assert.module_add(&name, module);
    }

    pub fn load_file(&self, label: &str) -> String {
        let loader = PathResolver::new_for_testing();
        let parsed = Label::parse(label, types::PackageRef::root()).unwrap();
        let path = loader.absolute_path(parsed.package(), parsed.name());
        std::fs::read_to_string(path).unwrap()
    }

    pub fn load_module(&mut self, label: &str) -> FrozenModule {
        let content = self.load_file(label);
        self.assert.module(label, &content)
    }

    /// Returns a read-only reference to the fake evaluation context.
    pub fn context(&self) -> &FakeEvalContext {
        &self.context
    }

    /// Returns the current session.
    pub fn session(&self) -> Rc<FakeSession> {
        self.context.session.clone()
    }

    /// Asserts that the result of evaluating code is equal to expected.
    #[track_caller]
    pub fn eq<T>(&mut self, code: &str, expected: T)
    where
        T: PartialEq + std::fmt::Debug + for<'v> UnpackValue<'v> + 'static,
    {
        assert_eq!(*self.eval::<T>(code), expected);
    }

    /// Evaluates code and unpacks it to a given type.
    #[track_caller]
    pub fn eval<T>(&mut self, code: &str) -> UnpackedOwnedValue<T>
    where
        T: for<'v> UnpackValue<'v> + 'static,
    {
        let owned_val = self.assert.pass(code);
        UnpackedOwnedValue::<T>::try_from(owned_val).unwrap()
    }

    /// Asserts that the two pieces of code produce something equivalent.
    #[track_caller]
    pub fn equivalent(&mut self, lhs_code: &str, rhs_code: &str) {
        let lhs_val = self.assert.pass(lhs_code);
        let rhs_val = self.assert.pass(rhs_code);
        assert_eq!(lhs_val.value(), rhs_val.value());
    }

    // We explicitly implement `pass`, `fail`, and `fails` with `&mut self`
    // signatures. The inherited methods on `starlark::assert::Assert` only
    // take `&self`, which would bypass the borrow checker and allow running
    // the evaluator while holding an active context borrow (leading to UB).
    // Exposing them as `&mut self` methods on the wrapper statically prevents this.

    /// Evaluates code and returns the Starlark value.
    #[track_caller]
    pub fn pass(&mut self, code: &str) -> starlark::values::OwnedFrozenValue {
        self.assert.pass(code)
    }

    /// Asserts that freezing the evaluated module fails with the expected
    /// error.
    #[track_caller]
    pub fn fail_to_freeze(&mut self, code: &str, expected_error: &str) -> starlark::Error {
        self.assert.fail_to_freeze(code, expected_error)
    }

    /// Asserts that the code fails to evaluate with the expected error.
    #[track_caller]
    pub fn fail(&mut self, code: &str, expected_error: &str) -> starlark::Error {
        self.assert.fail(code, expected_error)
    }

    /// Asserts that the code fails to evaluate with any of the expected errors.
    #[track_caller]
    pub fn fails(&mut self, code: &str, expected_errors: &[&str]) -> starlark::Error {
        self.assert.fails(code, expected_errors)
    }
}
