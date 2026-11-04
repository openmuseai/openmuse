// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    cell::OnceCell,
    fmt,
    fmt::{Debug, Display},
};

use allocative::Allocative;
use starlark::{
    any::ProvidesStaticType,
    collections::SmallMap,
    eval::{Arguments, Evaluator, ParametersSpec, ParametersSpecParam},
    starlark_simple_value,
    values::{
        typing::TypeInstanceId, Freeze, FreezeResult, Freezer, FrozenValue, FrozenValueTyped,
        StarlarkValue, Trace, Value,
    },
};
use starlark_derive::{starlark_value, NoSerialize};

use crate::{Error, ProviderInstance};

#[derive(Debug, Clone, Trace, Allocative)]
// Contains all the information we cannot know about a provider type until we
// actually know the name of it.
pub(crate) struct ProviderTypeData {
    pub(crate) name: starlark::values::FrozenStringValue,
    pub(crate) parameter_spec: ParametersSpec<FrozenValue>,
}

/// Represents the provider type constructor.
#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Trace)]
pub struct ProviderType {
    /// The unique type identifier.
    pub(crate) id: TypeInstanceId,
    /// The configured provider fields. This is set when starlark calls
    /// `export_as` when you assign the provider to a variable.
    /// If this is not set, you cannot construct the provider.
    pub(crate) data: OnceCell<ProviderTypeData>,
    /// A mapping from field name to index.
    /// This is akin to python's `__slots__`.
    pub(crate) fields: SmallMap<String, usize>,
}

/// Represents the frozen provider type constructor.
#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Trace)]
pub struct FrozenProviderType {
    /// The unique type identifier.
    pub(crate) id: TypeInstanceId,
    /// The configured provider fields.
    pub(crate) data: ProviderTypeData,
    /// A mapping from field name to index.
    /// This is akin to python's `__slots__`.
    pub(crate) fields: SmallMap<String, usize>,
}

starlark_simple_value!(FrozenProviderType);

impl Display for ProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<provider>")
    }
}

impl Display for FrozenProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<provider>")
    }
}

#[starlark_value(type = "provider")]
impl<'v> StarlarkValue<'v> for ProviderType {
    type Canonical = FrozenProviderType;

    // Being unable to invoke non-frozen provider types makes ProviderInstance code
    // much simpler. It isn't really a problems since you should be making
    // providers inside rule implementations. We can choose to add this feature
    // later if we'd like.
    fn invoke(
        &self,
        _me: Value<'v>,
        _args: &Arguments<'v, '_>,
        _eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        Err(Error::ProviderNotFrozen.into())
    }

    fn export_as(&self, name: &str, eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<()> {
        if self.data.get().is_some() {
            return Ok(());
        }

        if !name.ends_with("Info") {
            return Err(Error::InvalidProviderName(name.to_owned()).into());
        }

        self.data.get_or_init(|| ProviderTypeData {
            name: eval.frozen_heap().alloc_str(name),
            parameter_spec: ParametersSpec::new_named_only(
                name,
                self.fields
                    .keys()
                    .map(|f| (f.as_str(), ParametersSpecParam::Optional)),
            ),
        });
        Ok(())
    }
}

#[starlark_value(type = "provider")]
impl<'v> StarlarkValue<'v> for FrozenProviderType {
    type Canonical = Self;

    fn invoke(
        &self,
        me: Value<'v>,
        args: &Arguments<'v, '_>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<Value<'v>> {
        // Safety: `me` is the receiver of type `FrozenProviderType`, which is
        // guaranteed to be frozen.
        let provider_type: FrozenValueTyped<'static, Self> =
            unsafe { FrozenValueTyped::new_unchecked(me.unpack_frozen().unwrap_unchecked()) };

        let data = &self.data;
        data.parameter_spec
            .parser(args, eval, |param_parser, eval| {
                let values: Box<[Option<Value<'v>>]> = (0..self.fields.len())
                    .map(|_| param_parser.next_opt::<Value<'v>>())
                    .collect::<starlark::Result<_>>()?;
                Ok(eval.heap().alloc_complex(ProviderInstance {
                    provider_type: provider_type.as_ref(),
                    values,
                }))
            })
    }
}

impl ProviderType {
    /// Creates a new provider type with the provided fields.
    /// This provider is not yet configured, and is unusable until `export_as`
    /// is called.
    pub fn new(fields: Vec<String>) -> starlark::Result<Self> {
        let mut field_map = SmallMap::with_capacity(fields.len());
        for (idx, field) in fields.into_iter().enumerate() {
            if field_map.insert(field.clone(), idx).is_some() {
                return Err(Error::DuplicateFieldName(field).into());
            }
        }
        Ok(Self {
            id: TypeInstanceId::r#gen(),
            data: OnceCell::new(),
            fields: field_map,
        })
    }
}

impl FrozenProviderType {
    /// Creates a new builtin provider type with a custom stable TypeInstanceId.
    /// Unlike regular providers, these providers may have either Defaulted or
    /// Required parameters.
    pub fn new(
        id: TypeInstanceId,
        name: &'static str,
        fields: &[(
            &'static str,
            starlark::eval::ParametersSpecParam<FrozenValue>,
        )],
        heap: &starlark::values::FrozenHeap,
    ) -> Self {
        let field_map = fields
            .iter()
            .enumerate()
            .map(|(idx, (name, _))| (name.to_string(), idx))
            .collect();
        let name_frozen = heap.alloc_str(name);
        Self {
            id,
            data: ProviderTypeData {
                name: name_frozen,
                parameter_spec: ParametersSpec::new_named_only(name, fields.to_vec()),
            },
            fields: field_map,
        }
    }
}

impl Freeze for ProviderType {
    type Frozen = FrozenProviderType;

    fn freeze(self, _freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        let data = self.data.into_inner().ok_or(Error::ProviderNotExported)?;
        Ok(FrozenProviderType {
            id: self.id,
            data,
            fields: self.fields,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::globals::register_providers;

    fn new_assert() -> testutils::Assert {
        let mut a = testutils::Assert::default();
        a.modify_globals(|builder| {
            register_providers(builder);
        });
        a
    }

    #[test]
    fn test_provider_type() {
        let mut a = new_assert();
        a.fail(
            "provider()",
            "Missing named-only parameter `fields` for call to `provider`",
        );

        a.fail(
            "provider(fields=['a'])(a=1)",
            "Cannot construct values of non-frozen provider type",
        );

        a.fail(
            r#"
MyInfo = provider(fields=['a'])
MyInfo(a=1, b=2)
"#,
            "Cannot construct values of non-frozen provider type",
        );
        a.fail(
            "provider(fields = 1)",
            "Provider fields must be an iterable",
        );
        a.fail("provider(fields = ['a', 'a'])", "Duplicate field name: a");
        a.fail(
            r#"
p = provider(fields=['a'])
"#,
            "Provider name must end with 'Info' (got 'p')",
        );
    }

    #[test]
    fn test_provider_aliasing() {
        let mut a = new_assert();

        let p_info = a.pass("MyInfo = provider(fields = ['a']); foo = MyInfo; foo");
        a.modify_globals(move |builder| {
            builder.set("foo", p_info.clone());
        });

        let alias = a.pass("alias = foo; alias");
        a.modify_globals(move |builder| {
            builder.set("alias", alias.clone());
        });

        // The constructor should retain its canonical name "MyInfo"
        a.eq("str(alias(a = 1))", "MyInfo(a = 1)".to_string());
    }

    #[test]
    fn test_unexported_provider_fails_to_call() {
        let mut a = new_assert();
        a.fail_to_freeze(
            "x = [provider(fields=['a'])]; x",
            "The result of provider() must be assigned to a variable",
        );
    }
}
