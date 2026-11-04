// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt::{self, Display, Formatter};

use allocative::{Allocative, Visitor};
use starlark::{
    environment::{Methods, MethodsBuilder, MethodsStatic},
    eval::Evaluator,
    values::{list::UnpackList, none::NoneOr, ProvidesStaticType, StarlarkValue, Value},
};
use starlark_derive::{starlark_module, starlark_value, NoSerialize};

use crate::{allow_files::AllowFiles, cfg::AttrCfg, AttrKind};

/// The type that all parameters of attr.* get converted to.
/// Mostly used because rust doesn't have default parameters,
/// so we just ..Default::default() the fields that aren't used.
#[derive(Default)]
pub struct AttrSpecArgs<'v> {
    pub(crate) default: Option<Value<'v>>,
    pub(crate) mandatory: Option<bool>,
    pub(crate) allow_empty: Option<bool>,
    pub(crate) allow_files: Option<AllowFiles>,
    pub(crate) allow_single_file: Option<AllowFiles>,
    pub(crate) cfg: Option<AttrCfg>,
    pub(crate) doc: Option<NoneOr<String>>,
}

/// The Starlark `attr` module containing functions to declare rule attributes.
#[derive(Debug, ProvidesStaticType, NoSerialize)]
pub struct AttrModule {
    pub make_attr_schema: for<'v, 'a, 'e> fn(
        AttrKind,
        AttrSpecArgs<'v>,
        &mut Evaluator<'v, 'a, 'e>,
    ) -> starlark::Result<Value<'v>>,
}

impl Allocative for AttrModule {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        let visitor = visitor.enter_self_sized::<Self>();
        visitor.exit();
    }
}

starlark::starlark_simple_value!(AttrModule);

impl Display for AttrModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "attr")
    }
}

#[starlark_value(type = "attr")]
impl<'v> StarlarkValue<'v> for AttrModule {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new("attr", attr_methods);
        Some(RES.methods())
    }
}

#[starlark_module]
pub fn attr_methods(builder: &mut MethodsBuilder) {
    fn bool<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::Bool,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                ..Default::default()
            },
            eval,
        )
    }

    fn int<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        values: Option<UnpackList<i32>>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::Int {
                allowed: values.map(|v| v.into_iter().collect()),
            },
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                ..Default::default()
            },
            eval,
        )
    }

    fn int_list<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::IntList,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                ..Default::default()
            },
            eval,
        )
    }

    fn label<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_files: Option<AllowFiles>,
        allow_single_file: Option<AllowFiles>,
        cfg: Option<AttrCfg>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::Label,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_files,
                allow_single_file,
                cfg,
                ..Default::default()
            },
            eval,
        )
    }

    fn label_keyed_string_dict<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        allow_files: Option<AllowFiles>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::LabelKeyedStringDict,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                allow_files,
                ..Default::default()
            },
            eval,
        )
    }

    fn label_list<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        allow_files: Option<AllowFiles>,
        cfg: Option<AttrCfg>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::LabelList,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                allow_files,
                cfg,
                ..Default::default()
            },
            eval,
        )
    }

    fn label_list_dict<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        allow_files: Option<AllowFiles>,
        cfg: Option<AttrCfg>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::LabelListDict,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                allow_files,
                cfg,
                ..Default::default()
            },
            eval,
        )
    }

    fn string<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        values: Option<UnpackList<String>>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::String {
                allowed: values.map(|v| v.into_iter().collect()),
            },
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                ..Default::default()
            },
            eval,
        )
    }

    fn string_dict<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::StringDict,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                ..Default::default()
            },
            eval,
        )
    }

    fn string_keyed_label_dict<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        allow_files: Option<AllowFiles>,
        cfg: Option<AttrCfg>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::StringKeyedLabelDict,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                allow_files,
                cfg,
                ..Default::default()
            },
            eval,
        )
    }

    fn string_list<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::StringList,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                ..Default::default()
            },
            eval,
        )
    }

    fn string_list_dict<'v>(
        #[starlark(this)] this: &AttrModule,
        #[starlark(require = named)] default: Option<Value<'v>>,
        doc: Option<NoneOr<String>>,
        mandatory: Option<bool>,
        allow_empty: Option<bool>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        (this.make_attr_schema)(
            AttrKind::StringListDict,
            AttrSpecArgs {
                default,
                doc,
                mandatory,
                allow_empty,
                ..Default::default()
            },
            eval,
        )
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use starlark::assert::Assert;

    use super::*;
    use crate::AttrSchema;

    fn make_attr_schema<'v>(
        kind: crate::AttrKind,
        args: AttrSpecArgs<'v>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<starlark::values::Value<'v>> {
        let package = types::PackageRef::root();
        AttrSchema::create(
            kind,
            args,
            package,
            &types::PathResolver::new_for_testing(),
            &eval.heap(),
        )
    }

    pub fn new_attr_assert() -> Assert<'static> {
        let mut assert = Assert::new();
        assert.globals_add(|builder| {
            builder.set("attr", super::AttrModule { make_attr_schema });
        });
        assert
    }
}
