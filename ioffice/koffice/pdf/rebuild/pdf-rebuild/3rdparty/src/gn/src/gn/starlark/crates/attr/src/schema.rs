// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    fmt::{Display, Formatter},
};

use allocative::Allocative;
use starlark::{
    eval::ParametersSpecParam,
    starlark_simple_value,
    values::{
        none::NoneOr, Freeze, FreezeResult, Freezer, FrozenValue, Heap, ProvidesStaticType,
        StarlarkValue, Trace, Value,
    },
};
use starlark_derive::{starlark_value, NoSerialize};
use types::{PackageRef, PathResolver};

use crate::{allow_files::AllowFiles, cfg::AttrCfg, globals::AttrSpecArgs, Attr};

/// The underlying data type of a target attribute (e.g. Bool, String,
/// LabelList).
#[derive(Debug, Clone, PartialEq, Eq, Allocative)]
pub enum AttrKind {
    Bool,
    // Allowed is typically *very* small, so we use a Vec.
    Int { allowed: Option<Vec<i32>> },
    IntList,
    Label,
    LabelKeyedStringDict,
    LabelList,
    LabelListDict,
    // Allowed is typically *very* small, so we use a Vec.
    String { allowed: Option<Vec<String>> },
    StringDict,
    StringKeyedLabelDict,
    StringList,
    StringListDict,
}

/// Schema specifying what files (single or multiple) are allowed on a
/// label-like attribute.
#[derive(Debug, Clone, PartialEq, Eq, Allocative)]
pub enum AllowFilesSchema {
    None,
    Single(AllowFiles),
    Many(AllowFiles),
}

/// Represents an attr.foo(...) parameter.
/// Eg. attr.label(allow_single_file = True)
#[derive(Debug, Clone, PartialEq, Eq, Trace, NoSerialize, Allocative)]
pub struct AttrSchema {
    pub(crate) kind: AttrKind,
    pub(crate) default: Option<Attr>,
    pub(crate) disallow_empty: bool,
    pub(crate) allow_files: AllowFilesSchema,
    pub(crate) cfg: AttrCfg,
    pub(crate) doc: String,
}

// Safety: AttrSchema does not contain lifetime parameters, so it satisfies the
// lifetime requirement of StaticType.
unsafe impl ProvidesStaticType<'_> for AttrSchema {
    type StaticType = Self;
}

starlark_simple_value!(AttrSchema);

impl Freeze for AttrSchema {
    type Frozen = Self;

    fn freeze(self, _freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(self)
    }
}

impl Display for AttrSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[starlark_value(type = "AttrSchema")]
// Clippy recommends eliding the 'v lifetime, but the #[starlark_value] macro
// requires it to be explicitly declared.
#[allow(clippy::elidable_lifetime_names)]
impl<'v> StarlarkValue<'v> for AttrSchema {
    fn collect_repr(&self, collector: &mut String) {
        use std::fmt::Write as _;
        write!(collector, "{self:?}").unwrap();
    }
}

impl AttrSchema {
    /// Creates an `AttrSchema` from validation attributes and registers it.
    pub fn create<'v>(
        kind: AttrKind,
        args: AttrSpecArgs<'v>,
        package: &PackageRef,
        path_resolver: &PathResolver,
        heap: &Heap<'v>,
    ) -> starlark::Result<Value<'v>> {
        let mut schema = Self {
            kind,
            default: None,
            disallow_empty: match args.allow_empty {
                None => false,
                Some(b) => !b,
            },
            allow_files: match (
                args.allow_single_file.unwrap_or(AllowFiles::None),
                args.allow_files.unwrap_or(AllowFiles::None),
            ) {
                (AllowFiles::None, AllowFiles::None) => AllowFilesSchema::None,
                (af, AllowFiles::None) => AllowFilesSchema::Single(af),
                (AllowFiles::None, af) => AllowFilesSchema::Many(af),
                _ => return Err(crate::Error::AllowFilesMutuallyExclusive.into()),
            },
            cfg: args.cfg.unwrap_or(AttrCfg::CurrentToolchain),
            doc: match args.doc {
                None | Some(NoneOr::None) => String::new(),
                Some(NoneOr::Other(s)) => s,
            },
        };

        // Unify explicit and implicit none specifically for labels.
        // attr.label is the only type for which None is a valid value.
        let default = match args.default {
            None => None,
            Some(x) => {
                if schema.kind == AttrKind::Label && x.is_none() {
                    None
                } else {
                    Some(x)
                }
            },
        };

        let mandatory = args.mandatory.unwrap_or(false);
        if schema.disallow_empty && !mandatory && default.is_none() {
            return Err(crate::Error::AllowEmptyRequiresMandatoryOrDefault.into());
        }

        schema.default = match (mandatory, default) {
            (true, None) => None,
            (false, None) => Some(match &schema.kind {
                AttrKind::Bool => Attr::Bool(false),
                // If I create an `attr.int(values = [1])` and don't provide a value to bazel, it
                // sets the default value 0. This is wierd, but by providing this,
                // we maintain consistency with bazel.
                AttrKind::Int { .. } => Attr::Int(0),
                AttrKind::String { .. } => Attr::String(Default::default()),
                AttrKind::IntList => Attr::IntList(Default::default()),
                AttrKind::StringList => Attr::StringList(Default::default()),
                AttrKind::LabelList => Attr::LabelList(Default::default()),
                AttrKind::StringListDict => Attr::StringListDict(Default::default()),
                AttrKind::StringDict => Attr::StringDict(Default::default()),
                AttrKind::LabelKeyedStringDict => Attr::LabelKeyedStringDict(Default::default()),
                AttrKind::StringKeyedLabelDict => Attr::StringKeyedLabelDict(Default::default()),
                AttrKind::LabelListDict => Attr::LabelListDict(Default::default()),
                AttrKind::Label => Attr::Label(None),
            }),
            (false, Some(v)) => {
                // We provide an empty param name because you can write the following code:
                // p = attr.string(default = 1)
                // rule(..., attrs = {"foo": p})
                // This means that at the time you call attr.string, no parameter name has
                // been set.
                Some(Attr::create_without_defaults(
                    &schema,
                    v,
                    package,
                    path_resolver,
                )?)
            },
            (true, Some(_)) => {
                return Err(crate::Error::MandatoryAndDefaultMutuallyExclusive.into());
            },
        };

        Ok(heap.alloc(schema))
    }

    /// Verifies if a child attribute schema can override this attribute schema.
    pub fn check_override(&self, name: &str, child: &Self) -> starlark::Result<()> {
        if !matches!(self.kind, AttrKind::Label | AttrKind::LabelList) {
            return Err(crate::Error::OnlyLabelTypesMayBeOverridden(name.to_owned()).into());
        }
        if self.kind != child.kind
            || self.disallow_empty != child.disallow_empty
            || self.allow_files != child.allow_files
            || self.cfg != child.cfg
        {
            return Err(crate::Error::AttributeTypeMismatch(name.to_owned()).into());
        }
        Ok(())
    }

    /// Returns the default value of this attribute, if any.
    pub fn default(&self) -> Option<&Attr> {
        self.default.as_ref()
    }

    /// Returns the file matcher if this attribute schema allows files.
    pub fn file_matcher(&self) -> Option<&AllowFiles> {
        match &self.allow_files {
            AllowFilesSchema::Single(s) => Some(s),
            AllowFilesSchema::Many(s) => Some(s),
            AllowFilesSchema::None => None,
        }
    }

    // Converts the attribute to a parameter spec.
    // Note that we intentionally do not use Defaulted(T) as it only supports
    // Defaulted(starlark::Value), while we want Defaulted(Attr).
    pub fn as_param_spec(&self) -> ParametersSpecParam<FrozenValue> {
        match &self.default {
            None => ParametersSpecParam::Required,
            Some(_) => ParametersSpecParam::Optional,
        }
    }
}

#[cfg(test)]
mod tests {
    use types::Label;

    use super::*;
    use crate::{globals::tests::new_attr_assert, LabelOrFile};

    #[track_caller]
    fn assert_eq_schema(
        a: &mut starlark::assert::Assert<'static>,
        code: &str,
        expected: &AttrSchema,
    ) {
        let val = a.pass(code);
        let unpacked: &AttrSchema =
            starlark::values::UnpackValue::unpack_value_err(val.value()).unwrap();
        assert_eq!(unpacked, expected);
    }

    #[test]
    fn test_schema_bool() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.bool()",
            &AttrSchema {
                kind: AttrKind::Bool,
                default: Some(Attr::Bool(false)),
                disallow_empty: false,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );

        a.fail("attr.bool(default=None)", "Expected `bool`");
    }

    #[test]
    fn test_schema_int() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.int(default=42, doc='An integer')",
            &AttrSchema {
                kind: AttrKind::Int { allowed: None },
                default: Some(Attr::Int(42)),
                disallow_empty: false,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: "An integer".to_string(),
            },
        );

        a.fail(
            "attr.int(values=[1, 2], default=3)",
            "value 3 is not in allowed set",
        );
    }

    #[test]
    fn test_schema_string() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.string(values=['foo', 'bar'], mandatory=True)",
            &AttrSchema {
                kind: AttrKind::String {
                    allowed: Some(vec!["foo".to_string(), "bar".to_string()]),
                },
                default: None,
                disallow_empty: false,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );

        a.fail(
            "attr.string(values=['a', 'b'], default='c')",
            "value \"c\" is not in allowed set",
        );

        a.fail("attr.string('hello')", "Found 1 extra positional argument");
    }

    #[test]
    fn test_schema_label() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.label(allow_files=True, default=':foo')",
            &AttrSchema {
                kind: AttrKind::Label,
                default: Some(Attr::Label(Some(LabelOrFile::Label(Label::new(
                    PackageRef::root().to_owned(),
                    "foo".to_owned(),
                ))))),
                disallow_empty: false,
                allow_files: AllowFilesSchema::Many(AllowFiles::All),
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );

        assert_eq_schema(
            &mut a,
            "attr.label(default=None)",
            &AttrSchema {
                kind: AttrKind::Label,
                default: Some(Attr::Label(None)),
                disallow_empty: false,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );
    }

    #[test]
    fn test_schema_string_list() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.string_list(allow_empty=False, mandatory=True)",
            &AttrSchema {
                kind: AttrKind::StringList,
                default: None,
                disallow_empty: true,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );

        a.fail(
            "attr.string_list(allow_empty=False)",
            "allow_empty = False requires the attribute to be mandatory or have a non-empty default value",
        );
        a.fail(
            "attr.string_list(allow_empty=False, default=[])",
            "value cannot be empty",
        );
    }

    #[test]
    fn test_schema_string_default() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.string(default='hello')",
            &AttrSchema {
                kind: AttrKind::String { allowed: None },
                default: Some(Attr::String("hello".to_string())),
                disallow_empty: false,
                allow_files: AllowFilesSchema::None,
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );
    }

    #[test]
    fn test_schema_string_dict() {
        let a = new_attr_assert();

        a.fail(
            "attr.string_dict(allow_empty=False, default={})",
            "value cannot be empty",
        );
    }

    #[test]
    fn test_schema_allow_files_error() {
        let a = new_attr_assert();

        a.fail(
            "attr.label(allow_files=True, allow_single_file=True)",
            "allow_files and allow_single_file are mutually exclusive",
        );
    }

    #[test]
    fn test_schema_mandatory_default_error() {
        let a = new_attr_assert();

        a.fail(
            "attr.bool(mandatory=True, default=True)",
            "mandatory and default are mutually exclusive",
        );
    }

    #[test]
    fn test_schema_label_keyed_string_dict() {
        let mut a = new_attr_assert();

        assert_eq_schema(
            &mut a,
            "attr.label_keyed_string_dict(allow_files=True, mandatory=True)",
            &AttrSchema {
                kind: AttrKind::LabelKeyedStringDict,
                default: None,
                disallow_empty: false,
                allow_files: AllowFilesSchema::Many(AllowFiles::All),
                cfg: AttrCfg::CurrentToolchain,
                doc: String::new(),
            },
        );
    }
}
