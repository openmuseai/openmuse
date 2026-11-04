// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use attr::{traits::EvalContextAttrExt, TargetAttrExt, TargetRef};
use starlark::{environment::Module, eval::Evaluator, values::OwnedFrozenValue};
use types::{EvaluatorContextExt, Session};

use crate::{Ctx, FrozenRule};

/// Runs the rule implementation function for a given target.
/// This matches the execution phase where rule implementation is run
/// synchronously.
pub fn run<C: EvalContextAttrExt + crate::CtxMethods>(
    target: &<C::Session as Session>::TargetRef,
    create_context: impl FnOnce(&<C::Session as Session>::TargetRef) -> C,
) -> starlark::Result<OwnedFrozenValue>
where
    <C::Session as Session>::TargetRef: TargetAttrExt<Rule = FrozenRule<C>>,
{
    // Safety: rule is always a rule for custom rule-built targets.
    let rule = target.rule().unwrap();

    Module::with_temp_heap(|module| {
        let rule_context = create_context(target);

        // When the module is frozen, only things transitively required by extra_value
        // are kept.
        module.set_extra_value({
            let mut eval = Evaluator::new(&module);
            let ctx = eval.heap().alloc(Ctx::<C>::new(
                rule.schema.create_ctx_fields(
                    target.attrs(),
                    rule_context.session(),
                    &rule_context.current_toolchain(),
                    rule.builtin,
                    target.builtin_attrs(&eval.heap()),
                    &eval.heap(),
                )?,
                rule,
            ));

            eval.set_context(&rule_context);
            eval.eval_function(rule.implementation.to_value(), &[ctx], &[])?
        });

        let frozen = module.freeze()?;
        Ok(frozen.owned_extra_value().unwrap())
    })
}
