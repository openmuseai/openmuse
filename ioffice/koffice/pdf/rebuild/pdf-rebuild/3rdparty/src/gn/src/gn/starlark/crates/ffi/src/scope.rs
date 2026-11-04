// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::pin::Pin;

use starlark::values::{Heap, Value as StarlarkValue};
use types::intern_string;

use crate::{bridge::Value, Immutable, OwnedSlice, Scope};

impl Scope {
    fn new<'b>(
        parent: &Self,
        keys: &[&str],
    ) -> (cxx::UniquePtr<Self>, OwnedSlice<Pin<&'b mut Value>>) {
        let mut nested_scope = cxx::UniquePtr::<Self>::null();
        let values = crate::bridge::NewScope(parent, keys, &mut nested_scope);
        (nested_scope, values.into())
    }

    pub(crate) fn new_struct<'b>(
        settings: &crate::Settings,
        keys: &[&str],
    ) -> (cxx::UniquePtr<Self>, OwnedSlice<Pin<&'b mut Value>>) {
        let mut nested_scope = cxx::UniquePtr::<Self>::null();
        let values = crate::bridge::NewStruct(settings, keys, &mut nested_scope);
        (nested_scope, values.into())
    }

    /// Returns the settings for the given scope.
    pub fn settings(&self) -> &crate::Settings {
        // Safety: Settings pointer is always valid and non-null.
        unsafe { self.settings_cxx().as_ref() }.unwrap()
    }

    /// Returns the items currently in the scope (not including parent scopes).
    pub fn items(&self) -> Immutable<OwnedSlice<crate::bridge::KeyValue<'_>>> {
        let slice = crate::bridge::GetScopeItems(self);
        Immutable::from(crate::OwnedSlice::<crate::bridge::KeyValue>::from(slice))
    }

    /// Converts Scope items to Starlark key-value pairs.
    pub fn get_kv<'v>(&self, heap: &Heap<'v>) -> Vec<(&str, StarlarkValue<'v>)> {
        let owned = self.items();
        let mut items = Vec::new();
        for pair in owned.as_slice() {
            items.push((pair.key, pair.value.to_rust(heap)));
        }
        items
    }
}

pub struct OwnedScope(pub cxx::UniquePtr<Scope>);

impl types::Scope for OwnedScope {
    fn copy_with<'a, 'v>(&self, kv: impl Iterator<Item = (&'a str, StarlarkValue<'v>)>) -> Self {
        let parent = self.0.as_ref().unwrap();
        // Scope stores a map from string_view to value. Since we don't know the
        // lifetime of the string we were given, we must intern it in order to
        // guarantee it can be safely dereferenced.
        let (keys, vals): (Vec<&str>, Vec<StarlarkValue<'v>>) =
            kv.map(|(s, v)| (intern_string(s), v)).unzip();
        let (mut child_scope, mut placeholders) = Scope::new(parent, &keys);

        let child_pin = child_scope.as_mut().unwrap();
        for (placeholder, val) in placeholders.as_slice_mut().iter_mut().zip(vals) {
            placeholder
                .as_mut()
                .assign(val, child_pin.settings(), Default::default());
        }

        Self(child_scope)
    }

    fn get<'v>(&self, key: &str, heap: &Heap<'v>) -> Option<StarlarkValue<'v>> {
        let parent = self.0.as_ref().unwrap();
        let val_ptr = crate::bridge::GetValue(parent, key);
        // Safety: val_ptr is either null or a valid pointer backed by the parent scope
        // lifetime.
        unsafe { val_ptr.as_ref() }.map(|val| val.to_rust(heap))
    }
}

#[cfg(test)]
mod tests {
    use types::Scope as _;

    use super::*;
    use crate::TestWithScope;

    #[test]
    fn test_owned_scope_methods() {
        let mut setup = TestWithScope::new();
        let parent_scope = setup.scope();

        let (child_ptr, _) = Scope::new(&*parent_scope, &[]);
        let owned_scope = OwnedScope(child_ptr);

        starlark::environment::Module::with_temp_heap(|module| {
            let heap = module.heap();

            let val_int = heap.alloc(42);
            let val_str = heap.alloc("hello");

            let grandchild =
                owned_scope.copy_with(vec![("foo", val_int), ("bar", val_str)].into_iter());

            assert_eq!(grandchild.get("foo", &heap).unwrap().unpack_i32(), Some(42));
            assert_eq!(
                grandchild.get("bar", &heap).unwrap().unpack_str(),
                Some("hello")
            );
            assert!(grandchild.get("baz", &heap).is_none());
        });
    }
}
