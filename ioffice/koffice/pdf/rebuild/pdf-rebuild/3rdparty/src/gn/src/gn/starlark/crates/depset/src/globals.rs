// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{
    collections::SmallSet,
    values::{list::UnpackList, Heap, UnpackValue as _, Value, ValueLike as _},
};
use types::File;

use crate::{Depset, Kind, Order, UnpackDepset};

/// Inner implementation of the Starlark `depset(...)` constructor.
/// This checks type matching, detects order conflicts, and creates the depset
/// value.
pub fn depset_constructor<'v, C: types::EvalContext>(
    direct: Option<UnpackList<Value<'v>>>,
    transitive: Option<UnpackList<UnpackDepset<'v>>>,
    mut order: Order,
    heap: &Heap<'v>,
    ctx: &C,
) -> starlark::Result<Value<'v>> {
    let mut kind = Kind::Empty;
    let mut set_kind = |k: Kind| -> Result<(), crate::Error> {
        if k == Kind::Empty {
            return Ok(());
        }
        if kind == Kind::Empty {
            kind = k;
            Ok(())
        } else if kind != k {
            Err(crate::Error::DepsetTypeMismatch {
                expected: kind,
                got: k,
            })
        } else {
            Ok(())
        }
    };

    let mut direct_vec = Vec::new();
    if let Some(direct_list) = direct {
        let mut direct_set = SmallSet::with_capacity(direct_list.items.len());
        direct_vec = Vec::with_capacity(direct_list.items.len());
        for elem in direct_list.items {
            if elem.downcast_ref::<File>().is_some() {
                set_kind(Kind::File)?;
            } else {
                set_kind(Kind::Unknown)?;
            }
            if direct_set.insert_hashed(elem.get_hashed()?) {
                direct_vec.push(elem);
            }
        }
    }

    let transitive_vec = match transitive {
        Some(transitive_list) => transitive_list
            .items
            .into_iter()
            .filter(|child_depset| !child_depset.is_empty())
            .map(|child_depset| {
                let child_order = child_depset.order();
                if child_order != Order::Unspecified {
                    if order == Order::Unspecified {
                        order = child_order;
                    } else if order != child_order {
                        return Err(crate::Error::ConflictingOrders { order, child_order });
                    }
                }
                set_kind(*child_depset.kind())?;
                Ok(child_depset.value())
            })
            .collect::<Result<Vec<_>, crate::Error>>()?,
        None => Vec::new(),
    };

    if direct_vec.is_empty() && transitive_vec.len() == 1 {
        // Safe to unwrap because we validated it's a depset when pushing to
        // transitive_vec.
        let child_depset = Depset::from_value(transitive_vec[0]).unwrap();
        if order == child_depset.order() || order == Order::Unspecified {
            // Just reuse the child depset object.
            Ok(transitive_vec[0])
        } else {
            // We cannot reuse the child because the wrapper specifies a different order,
            // so we create a new equivalent depset wrapping it.
            Ok(heap.alloc(Depset {
                order,
                direct: child_depset.direct.clone(),
                transitive: child_depset.transitive.clone(),
                kind: child_depset.kind,
                phony: child_depset.phony.clone(),
            }))
        }
    } else {
        let phony = if kind == Kind::File {
            if direct_vec.len() == 1 && transitive_vec.is_empty() {
                // If the depset contains only a single element, its phony is actually just the
                // real object. We don't need to do this for transitive, because
                // we optimize a depset with 1 transitive and no direct to not even create a
                // depset object.
                direct_vec[0].downcast_ref::<File>().cloned()
            } else if !direct_vec.is_empty() || !transitive_vec.is_empty() {
                let mut deps = vec![];
                for v in &direct_vec {
                    // Safety: `v` is guaranteed to be a `File` because we validated all direct
                    // elements when setting the depset kind to `Kind::File`.
                    deps.push(v.downcast_ref::<File>().unwrap().clone());
                }
                for v in &transitive_vec {
                    // Safety:
                    // 1. `v` is guaranteed to be a `Depset` because we successfully unpacked it and
                    //    validated its kind during transition loop validation.
                    // 2. The child depset is guaranteed to have a `phony` File because it is a
                    //    non-empty `Kind::File` depset, which always has a phony file.
                    let child_dep = UnpackDepset::unpack_value(*v).unwrap().unwrap();
                    deps.push(child_dep.phony().as_ref().unwrap().clone());
                }
                let state = ctx.require_rule_impl()?;
                Some(state.new_phony(deps))
            } else {
                None
            }
        } else {
            None
        };

        Ok(heap.alloc(Depset {
            order,
            direct: direct_vec,
            transitive: transitive_vec,
            kind,
            phony,
        }))
    }
}

#[doc(hidden)]
pub mod __private {
    pub use starlark;
    pub use starlark_derive::starlark_module;
    pub use types::EvaluatorContextExt;
}

/// Helper macro to register the `depset` function.
/// Required because this module isn't aware of the real EvaluatorContext type.
#[macro_export]
macro_rules! depset_globals {
    ($builder:expr, $ctx_type:ty) => {{
        // starlark_module requires that the function returns something named
        // "starlark::Result".
        use $crate::__private::starlark;

        #[$crate::__private::starlark_module]
        fn register_depset_globals(builder: &mut starlark::environment::GlobalsBuilder) {
            fn depset<'v>(
                direct: Option<starlark::values::list::UnpackList<starlark::values::Value<'v>>>,
                transitive: Option<starlark::values::list::UnpackList<$crate::UnpackDepset<'v>>>,
                #[starlark(default = $crate::Order::Unspecified)] order: $crate::Order,
                eval: &mut starlark::eval::Evaluator<'v, '_, '_>,
            ) -> starlark::Result<starlark::values::Value<'v>> {
                use $crate::__private::EvaluatorContextExt;

                $crate::depset_constructor::<$ctx_type>(
                    direct,
                    transitive,
                    order,
                    &eval.heap(),
                    eval.context(),
                )
            }
        }

        register_depset_globals($builder);
    }};
}

#[cfg(test)]
pub(crate) mod tests {
    use starlark::{environment::GlobalsBuilder, eval::Evaluator};
    use starlark_derive::starlark_module;
    use types::EvaluatorContextExt as _;

    use super::*;

    #[starlark_module]
    pub(crate) fn test_globals(builder: &mut GlobalsBuilder) {
        fn new_file_depset<'v>(
            files: UnpackList<&File>,
            eval: &mut Evaluator<'v, '_, '_>,
        ) -> starlark::Result<Depset<'v>> {
            let heap = eval.heap();
            let ctx = eval.context::<testutils::eval_context::FakeEvalContext>();
            let direct = files.items.into_iter().cloned().collect();
            Depset::new_file_depset(direct, &heap, ctx)
        }
    }

    pub(crate) fn new_assert() -> testutils::Assert {
        let mut a = testutils::Assert::default();
        a.modify_globals(|builder| {
            test_globals(builder);
            depset_globals!(builder, testutils::FakeEvalContext);
        });
        a
    }
}
