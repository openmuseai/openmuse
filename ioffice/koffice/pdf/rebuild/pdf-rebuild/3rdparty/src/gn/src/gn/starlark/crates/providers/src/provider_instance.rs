// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{fmt, fmt::Display, hash::Hasher as _};

use allocative::Allocative;
use starlark::{
    any::ProvidesStaticType,
    coerce::Coerce,
    collections::{Hashed, StarlarkHasher},
    starlark_complex_value,
    values::{
        Freeze, FreezeResult, Freezer, Heap, StarlarkValue, Trace, Value, ValueLifetimeless,
        ValueLike,
    },
};
use starlark_derive::{starlark_value, NoSerialize};

use crate::provider_type::FrozenProviderType;

/// Represents an instance of a provider.
#[derive(Clone, Trace, Coerce, ProvidesStaticType, Allocative, NoSerialize)]
#[repr(C)]
pub struct ProviderInstanceGen<V: ValueLifetimeless> {
    pub(crate) provider_type: &'static FrozenProviderType,
    pub(crate) values: Box<[Option<V>]>,
}

impl<V: ValueLifetimeless + Freeze> Freeze for ProviderInstanceGen<V>
where
    V::Frozen: ValueLifetimeless,
{
    type Frozen = ProviderInstanceGen<V::Frozen>;

    fn freeze(self, freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        Ok(ProviderInstanceGen {
            provider_type: self.provider_type,
            values: self
                .values
                .into_vec()
                .into_iter()
                .map(|v| v.freeze(freezer))
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice(),
        })
    }
}

starlark_complex_value!(pub ProviderInstance);

impl<'v, V: ValueLike<'v>> ProviderInstanceGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    pub(crate) fn ty_name(&self) -> &'static str {
        self.get_type_value_dyn().as_str()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'v str, V)> + 'a
    where
        'v: 'a,
    {
        let fields = &self.provider_type.fields;
        fields
            .iter()
            .filter_map(move |(name, &idx)| self.values[idx].map(|val| (name.as_str(), val)))
    }

    // A common API for both the Debug trait and starlark's repr.
    fn collect_repr_impl(&self, collector: &mut String) {
        use std::fmt::Write as _;
        write!(collector, "{}(", self.ty_name()).unwrap();
        for (i, (name, val)) in self.iter().enumerate() {
            if i > 0 {
                write!(collector, ", ").unwrap();
            }
            write!(collector, "{name} = ").unwrap();
            val.to_value().collect_repr(collector);
        }
        write!(collector, ")").unwrap();
    }
}

impl<'v, V: ValueLike<'v>> fmt::Debug for ProviderInstanceGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.collect_repr_impl(&mut s);
        write!(f, "{s}")
    }
}

impl<'v, V: ValueLike<'v>> Display for ProviderInstanceGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[starlark_value(type = "provider")]
impl<'v, V: ValueLike<'v>> StarlarkValue<'v> for ProviderInstanceGen<V>
where
    Self: ProvidesStaticType<'v>,
{
    type Canonical = FrozenProviderInstance;

    fn get_type_value_dyn(&self) -> starlark::values::FrozenStringValue {
        self.provider_type.data.name
    }

    fn equals(&self, other: Value<'v>) -> starlark::Result<bool> {
        let Some(other) = ProviderInstance::from_value(other) else {
            return Ok(false);
        };
        if self.provider_type.id != other.provider_type.id {
            return Ok(false);
        }
        for (v1, v2) in self.values.iter().zip(other.values.iter()) {
            match (v1, v2) {
                (Some(val1), Some(val2)) => {
                    if !val1.to_value().equals(val2.to_value())? {
                        return Ok(false);
                    }
                },
                (None, None) => {},
                _ => return Ok(false),
            }
        }
        Ok(true)
    }

    fn collect_repr(&self, collector: &mut String) {
        self.collect_repr_impl(collector);
    }

    fn collect_repr_cycle(&self, collector: &mut String) {
        use std::fmt::Write as _;
        write!(collector, "{}(...)", self.ty_name()).unwrap();
    }

    fn get_attr(&self, attribute: &str, heap: Heap<'v>) -> Option<Value<'v>> {
        self.get_attr_hashed(Hashed::new(attribute), heap)
    }

    fn get_attr_hashed(&self, attribute: Hashed<&str>, _heap: Heap<'v>) -> Option<Value<'v>> {
        let &i = self.provider_type.fields.get_hashed(attribute)?;
        self.values[i].map(|v| v.to_value())
    }

    fn write_hash(&self, hasher: &mut StarlarkHasher) -> starlark::Result<()> {
        self.provider_type.write_hash(hasher)?;
        for v in &self.values {
            if let Some(val) = *v {
                val.write_hash(hasher)?;
            } else {
                hasher.write_u8(0);
            }
        }
        Ok(())
    }

    fn dir_attr(&self) -> Vec<String> {
        let fields = &self.provider_type.fields;
        fields
            .iter()
            .filter_map(|(name, &idx)| {
                if self.values[idx].is_some() {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use starlark::values::UnpackValue as _;

    use crate::{register_providers, ProviderInstance};

    fn new_assert() -> testutils::Assert {
        let mut a = testutils::Assert::default();
        a.modify_globals(|builder| {
            register_providers(builder);
        });
        a
    }

    #[test]
    fn test_unpack_fails() {
        let mut a = new_assert();
        let val = a.pass("1");
        let err = <&ProviderInstance>::unpack_value_err(val.value()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Expected `provider`, but got `int (repr: 1)`"
        );
    }

    #[test]
    fn test_provider_instance() {
        let mut a = new_assert();

        let ty = a.pass("MyInfo = provider(fields = ['first', 'second', 'third']); MyInfo");
        a.modify_globals(move |builder| {
            builder.set("MyInfo", ty.clone());
        });

        let instance = a.pass("MyInfo(first = 'hello', third = 3)");
        let unpacked = <&ProviderInstance>::unpack_value_err(instance.value()).unwrap();
        assert_eq!(unpacked.ty_name(), "MyInfo");

        a.modify_globals(move |builder| {
            builder.set("info", instance.clone());
        });

        a.eq("type(info)", "MyInfo".to_string());

        a.eq(
            "str(info)",
            "MyInfo(first = \"hello\", third = 3)".to_string(),
        );
        a.eq(
            "repr(info)",
            "MyInfo(first = \"hello\", third = 3)".to_string(),
        );

        a.eq(
            "dir(info)",
            starlark::values::list::UnpackList {
                items: vec!["first".to_string(), "third".to_string()],
            },
        );

        // 'first' field: declared and set
        a.eq(r#"hasattr(info, "first")"#, true);
        a.eq(r#"info.first"#, "hello".to_string());

        // 'second' field: declared but unset
        a.eq(r#"hasattr(info, "second")"#, false);
        a.fail(
            "info.second",
            "Object of type `MyInfo` has no attribute `second`",
        );

        // 'nonexistent' field: undeclared and unset
        a.eq(r#"hasattr(info, "nonexistent")"#, false);
        a.fail(
            "info.nonexistent",
            "Object of type `MyInfo` has no attribute `nonexistent`",
        );

        a.eq(
            r#"
info2 = MyInfo(first = [])
info2.first.append(info2)
str(info2)
"#,
            "MyInfo(first = [MyInfo(...)])".to_string(),
        );
    }

    #[test]
    fn test_provider_instance_equality() {
        let mut a = new_assert();

        let ty = a.pass("MyInfo = provider(fields = ['first', 'second']); MyInfo");
        let other_ty = a.pass("OtherInfo = provider(fields = ['first', 'second']); OtherInfo");
        a.modify_globals(move |builder| {
            builder.set("MyInfo", ty.clone());
            builder.set("OtherInfo", other_ty.clone());
        });

        a.eq(
            "MyInfo(first = 'hello', second = 2) == MyInfo(first = 'hello', second = 2)",
            true,
        );
        a.eq(
            "MyInfo(first = 'hello', second = 2) == MyInfo(first = 'hello', second = 3)",
            false,
        );
        a.eq(
            "MyInfo(first = 'hello', second = 2) == MyInfo(first = 'hello')",
            false,
        );

        a.eq(
            "MyInfo(first = 'hello', second = 2) == OtherInfo(first = 'hello', second = 2)",
            false,
        );

        // Test equality between a frozen instance and an unfrozen instance
        let frozen_info = a.pass("MyInfo(first = 'hello', second = 2)");
        a.modify_globals(move |builder| {
            builder.set("frozen_info", frozen_info.clone());
        });
        a.eq("frozen_info == MyInfo(first = 'hello', second = 2)", true);
        a.eq("MyInfo(first = 'hello', second = 2) == frozen_info", true);
    }
}
