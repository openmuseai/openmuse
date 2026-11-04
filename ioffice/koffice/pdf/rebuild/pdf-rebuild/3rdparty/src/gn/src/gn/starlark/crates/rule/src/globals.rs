// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use attr::{traits::EvalContextAttrExt, TargetAttrExt};
use starlark::{
    environment::{FrozenModule, Module},
    values::FrozenHeapName,
};
use strum::IntoEnumIterator;

use crate::{FrozenRule, OutputType};

/// Registers and returns the built-in target rules as a frozen module.
/// Usage:
/// load("//builtins:rules.scl", "static_library")
///
/// static_library(
///   name = "foo",
///   sources = [...],
///   ...
/// )
pub fn register_builtin_rules<C: EvalContextAttrExt>() -> FrozenModule
where
    <C::Session as types::Session>::TargetRef: TargetAttrExt,
{
    Module::with_temp_heap(|module| {
        for output_type in OutputType::iter() {
            let name = output_type.to_string();
            let frozen_rule = FrozenRule::<C>::new_builtin(output_type, module.frozen_heap());
            let frozen_value = module.frozen_heap().alloc(frozen_rule);
            module.set(&name, frozen_value.to_value());
        }

        module
            .freeze_named(FrozenHeapName::User(Box::new(
                "//builtins:rules.scl".to_owned(),
            )))
            .unwrap()
    })
}

/// Registers standard target rules in the session.
#[macro_export]
macro_rules! register_rule_globals {
    ($builder:expr, $ctx_type:ty) => {
        #[starlark_derive::starlark_module]
        fn register_rule_globals(builder: &mut starlark::environment::GlobalsBuilder) {
            fn rule<'v>(
                #[starlark(require = named)] implementation: starlark::values::Value<'v>,
                #[starlark(require = named)] parent: Option<starlark::values::Value<'v>>,
                #[starlark(require = named)] attrs: Option<
                    starlark::collections::SmallMap<&str, &attr::AttrSchema>,
                >,
                eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
            ) -> starlark::Result<starlark::values::Value<'v>> {
                // Rules can only be created while loading scl files, not in macros or rule
                // implementations.
                use types::{EvalContext, EvaluatorContextExt};
                eval.context::<$ctx_type>().require_bzl()?;

                let attrs = attrs.unwrap_or_default();
                let attrs = attrs
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect();
                let rule = <$crate::Rule<'v, $ctx_type>>::new(
                    attrs,
                    None,
                    parent,
                    implementation,
                    eval.frozen_heap(),
                )?;
                Ok(eval.heap().alloc(rule))
            }
        }
        register_rule_globals($builder);
    };
}

#[cfg(test)]
pub(crate) mod tests {
    use starlark::values::list::UnpackList;

    pub(crate) fn new_assert() -> testutils::Assert {
        testutils::Assert::new_rule_assert()
    }

    #[test]
    fn test_rule_creation() {
        let mut a = new_assert();
        let rule_value = a.pass(
            r#"
my_rule = rule(implementation = None)
my_rule
"#,
        );
        let my_rule = rule_value.value().unpack_frozen().unwrap();

        a.modify_globals(move |b| {
            b.set("my_rule", my_rule);
        });

        a.eq(
            r#"
my_rule2 = rule(implementation = None)
[str(my_rule), str(my_rule2), str(rule(implementation = None))]
"#,
            UnpackList {
                items: vec![
                    "<rule: my_rule>".to_owned(),
                    "<rule: my_rule2>".to_owned(),
                    "<anonymous rule>".to_owned(),
                ],
            },
        );

        a.fail_to_freeze(
            r#"
x = [rule(implementation = None)]
"#,
            "Rule must be assigned to a global variable to be used",
        );

        a.fail(
            "rule()",
            "Missing named-only parameter `implementation` for call to `rule`",
        );

        a.fail(
            r#"rule(implementation = lambda ctx: None, attrs = {"name": attr.string()})"#,
            "Attribute 'name' is reserved",
        );
    }
}
