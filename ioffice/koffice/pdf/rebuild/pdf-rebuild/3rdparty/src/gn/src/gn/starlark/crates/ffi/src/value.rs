// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::pin::Pin;

use starlark::values::{list::ListRef, structs::StructRef};

use crate::{
    bridge::{SliceAny, Value, ValueType},
    Immutable, Scope, Settings, Slice,
};

impl Value {
    fn list_value(&self) -> Immutable<Slice<Self>> {
        Immutable::from(Slice::from(crate::bridge::list_value_cxx(self)))
    }

    pub fn to_rust<'v>(&self, heap: &starlark::values::Heap<'v>) -> starlark::values::Value<'v> {
        match self.kind() {
            ValueType::None => starlark::values::Value::new_none(),
            ValueType::Boolean => starlark::values::Value::new_bool(*self.boolean_value()),
            ValueType::Integer => heap.alloc(*self.int_value()),
            ValueType::String => heap.alloc(self.string_value()),
            ValueType::List => {
                let slice = self.list_value();
                let items: Vec<_> = slice.iter().map(|item| item.to_rust(heap)).collect();
                heap.alloc(items)
            },
            ValueType::Scope => {
                let scope_ptr = self.scope_value();
                // Safety: C++ Value invariants guarantee that scope_value() is never null
                // when the type is SCOPE.
                let scope = unsafe { &*scope_ptr };
                heap.alloc(starlark::values::structs::AllocStruct(scope.get_kv(heap)))
            },
            _ => unreachable!(),
        }
    }

    pub fn assign<'v>(
        mut self: Pin<&mut Self>,
        val: starlark::values::Value<'v>,
        settings: &Settings,
        origin: crate::bridge::ParseNodePtr,
    ) {
        if val.is_none() {
            crate::bridge::SetValueNone(self.as_mut(), origin);
        } else if let Some(s) = val.unpack_str() {
            crate::bridge::SetValueString(self.as_mut(), origin, s);
        } else if let Some(b) = val.unpack_bool() {
            crate::bridge::SetValueBool(self.as_mut(), origin, b);
        } else if let Some(i) = val.unpack_i32() {
            crate::bridge::SetValueInt(self.as_mut(), origin, i64::from(i));
        } else if let Some(l) = ListRef::from_value(val) {
            let mut slice: Slice<Self> = SliceAny {
                ptr: crate::bridge::SetValueList(self.as_mut(), origin, l.len()),
                len: l.len(),
            }
            .into();
            for (el_pin, src) in slice.iter_mut().zip(l.iter()) {
                el_pin.assign(src, settings, origin);
            }
        } else if let Some(s) = StructRef::from_value(val) {
            let keys: Vec<&str> = s.iter().map(|(k, _)| k.as_str()).collect();
            let (r#struct, mut values) = Scope::new_struct(settings, &keys);

            for (v_starlark, v_cxx) in s.iter().map(|(_, v)| v).zip(values.as_slice_mut()) {
                v_cxx.as_mut().assign(v_starlark, settings, origin);
            }

            crate::bridge::SetValueScope(self.as_mut(), origin, r#struct);
        } else {
            todo!("Arbitrary starlark values not (yet) supported");
        }
    }
}

#[cfg(test)]
mod tests {
    use starlark::values::{FrozenValue, Heap, ValueLike as _};

    use super::*;
    use crate::TestWithScope;

    fn back_and_forth<'v>(
        heap: &Heap<'v>,
        val: starlark::values::Value<'v>,
    ) -> starlark::values::Value<'v> {
        let mut setup = TestWithScope::new();
        let scope = setup.scope();

        let mut value = crate::bridge::NewValueForTesting();
        value.pin_mut().assign(
            val,
            scope.settings(),
            crate::bridge::ParseNodePtr {
                ptr: std::ptr::null(),
            },
        );
        value.to_rust(heap)
    }

    #[test]
    fn test_none_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            assert!(back_and_forth(&heap, FrozenValue::new_none().to_value()).is_none());
        });
    }

    #[test]
    fn test_bool_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            assert_eq!(
                back_and_forth(&heap, heap.alloc(true).to_value()).unpack_bool(),
                Some(true)
            );
            assert_eq!(
                back_and_forth(&heap, heap.alloc(false).to_value()).unpack_bool(),
                Some(false)
            );
        });
    }

    #[test]
    fn test_int_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            assert_eq!(
                back_and_forth(&heap, heap.alloc(123456789i32).to_value()).unpack_i32(),
                Some(123456789)
            );
        });
    }

    #[test]
    fn test_string_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            assert_eq!(
                back_and_forth(&heap, heap.alloc("hello world").to_value()).unpack_str(),
                Some("hello world")
            );
            assert_eq!(
                back_and_forth(
                    &heap,
                    heap.alloc("hello long string without SSO optimizations")
                        .to_value(),
                )
                .unpack_str(),
                Some("hello long string without SSO optimizations")
            );
        });
    }

    #[test]
    fn test_list_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            let list_ref = ListRef::from_value(back_and_forth(
                &heap,
                heap.alloc(vec![heap.alloc(42), heap.alloc("hello")])
                    .to_value(),
            ))
            .unwrap();
            assert_eq!(list_ref.len(), 2);
            let mut iter = list_ref.iter();
            assert_eq!(iter.next().unwrap().unpack_i32(), Some(42));
            assert_eq!(iter.next().unwrap().unpack_str(), Some("hello"));
        });
    }

    #[test]
    fn test_struct_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            let struct_ref = StructRef::from_value(back_and_forth(
                &heap,
                heap.alloc(starlark::values::structs::AllocStruct(vec![
                    ("foo", heap.alloc(100)),
                    ("bar", heap.alloc("baz")),
                ]))
                .to_value(),
            ))
            .unwrap();
            let get_field = |name: &str| {
                struct_ref
                    .iter()
                    .find(|(k, _)| k.as_str() == name)
                    .map(|(_, v)| v)
            };
            assert_eq!(get_field("foo").unwrap().unpack_i32(), Some(100));
            assert_eq!(get_field("bar").unwrap().unpack_str(), Some("baz"));
        });
    }

    #[test]
    fn test_nested_struct_conversion() {
        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();
            let inner_struct =
                starlark::values::structs::AllocStruct(vec![("inner_foo", heap.alloc(42))]);
            let struct_ref = StructRef::from_value(back_and_forth(
                &heap,
                heap.alloc(starlark::values::structs::AllocStruct(vec![
                    ("outer_foo", heap.alloc(100)),
                    ("nested", heap.alloc(inner_struct)),
                ]))
                .to_value(),
            ))
            .unwrap();

            let outer_foo = struct_ref
                .iter()
                .find(|(k, _)| k.as_str() == "outer_foo")
                .map(|(_, v)| v)
                .unwrap()
                .unpack_i32();
            assert_eq!(outer_foo, Some(100));

            let nested_val = struct_ref
                .iter()
                .find(|(k, _)| k.as_str() == "nested")
                .map(|(_, v)| v)
                .unwrap();
            let nested_ref = StructRef::from_value(nested_val).unwrap();
            let inner_foo = nested_ref
                .iter()
                .find(|(k, _)| k.as_str() == "inner_foo")
                .map(|(_, v)| v)
                .unwrap()
                .unpack_i32();
            assert_eq!(inner_foo, Some(42));
        });
    }
}
