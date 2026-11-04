// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use attr::{Attr, LabelOrFile};
use rule::FrozenRule;
use starlark::{environment::FrozenModule, values::FrozenValueTyped};
use testutils::{Assert, FakeEvalContext, FakeTarget};
use types::{Label, OutputType, PackageRef, Session};

fn new_assert() -> Assert {
    Assert::new_rule_assert()
}

#[test]
fn test_pure_rule_inheritance() {
    let mut assert = new_assert();
    let native = assert.load_module("//rules:native.scl");
    let pure = assert.load_module("//rules:pure.scl");

    let rule = |module: &FrozenModule, name: &str| {
        let val = module.get(name).unwrap().value().unpack_frozen().unwrap();
        let typed = FrozenValueTyped::<FrozenRule<FakeEvalContext>>::new(val).unwrap();
        Some(typed.as_ref())
    };

    assert.pass(
        r#"
load("//rules:pure.scl", "child_rule", "parent_rule")
load("//rules:native.scl", "custom_shared_library", "static_library")

custom_shared_library(
    name = "shared_library",
    mandatory = "mandatory_val",
    optional = "optional_val",
    unknown = "unknown",
)

static_library(
   name = "static_library",
   optional = "optional_val",
   unknown = "unknown"
)

parent_rule(
    name = "parent_defaulted",
    parent_only = "p",
)

child_rule(
    name = "child_defaulted",
    parent_only = "p",
    child_only = "c",
)

child_rule(
    name = "child_override",
    parent_only = "parent_val",
    child_only = "child_val",
    override = "//:custom_val",
)
"#,
    );

    let heap = starlark::values::FrozenHeap::new();
    let mut unknown_attrs = HashMap::new();
    unknown_attrs.insert(
        "unknown".to_owned(),
        starlark::values::Value::new_frozen(heap.alloc("unknown")),
    );

    let context = assert.context();
    let load = |name: &str| {
        let label = Label::new(PackageRef::root().to_owned(), name.to_owned());
        context
            .session
            .get_target(label.as_ref(), context.session.default_toolchain.as_ref())
    };

    assert_eq!(
        *load("shared_library"),
        FakeTarget {
            label: Label::new(PackageRef::root().to_owned(), "shared_library".to_owned()),
            toolchain: context.session.default_toolchain.clone(),
            outputs: vec![],
            attrs: vec![
                Attr::String("optional_val".to_owned()),
                Attr::String("mandatory_val".to_owned()),
            ],
            output_type: Some(OutputType::SharedLibrary),
            rule: rule(&native, "custom_shared_library"),
            cxx_attrs: unknown_attrs.clone(),
            dependencies: Mutex::new(HashSet::new()),
        }
    );

    assert_eq!(
        *load("static_library"),
        FakeTarget {
            label: Label::new(PackageRef::root().to_owned(), "static_library".to_owned()),
            toolchain: context.session.default_toolchain.clone(),
            outputs: vec![],
            attrs: vec![Attr::String("optional_val".to_owned())],
            output_type: Some(OutputType::StaticLibrary),
            rule: rule(&native, "static_library"),
            cxx_attrs: unknown_attrs.clone(),
            dependencies: Mutex::new(HashSet::new()),
        }
    );

    let toolchain = Label::new(
        PackageRef::root().to_owned(),
        "default_toolchain".to_owned(),
    );

    assert_eq!(
        *load("parent_defaulted"),
        FakeTarget {
            label: Label::new(PackageRef::root().to_owned(), "parent_defaulted".to_owned()),
            toolchain: toolchain.clone(),
            outputs: vec![],
            attrs: vec![
                Attr::String("p".to_owned()),
                Attr::Label(Some(LabelOrFile::Label(Label::new(
                    PackageRef::root().to_owned(),
                    "parent".to_owned()
                )))),
            ],
            output_type: None,
            rule: rule(&pure, "parent_rule"),
            cxx_attrs: HashMap::new(),
            dependencies: Mutex::new(HashSet::from([(
                Label::new(PackageRef::root().to_owned(), "parent".to_owned()),
                toolchain.clone(),
            )])),
        }
    );

    assert_eq!(
        *load("child_defaulted"),
        FakeTarget {
            label: Label::new(PackageRef::root().to_owned(), "child_defaulted".to_owned()),
            toolchain: toolchain.clone(),
            outputs: vec![],
            attrs: vec![
                Attr::String("p".to_owned()),
                Attr::Label(Some(LabelOrFile::Label(Label::new(
                    PackageRef::root().to_owned(),
                    "child".to_owned()
                )))),
                Attr::String("c".to_owned()),
            ],
            output_type: None,
            rule: rule(&pure, "child_rule"),
            cxx_attrs: HashMap::new(),
            dependencies: Mutex::new(HashSet::from([(
                Label::new(PackageRef::root().to_owned(), "child".to_owned()),
                toolchain.clone(),
            )])),
        }
    );

    assert_eq!(
        *load("child_override"),
        FakeTarget {
            label: Label::new(PackageRef::root().to_owned(), "child_override".to_owned()),
            toolchain: toolchain.clone(),
            outputs: vec![],
            attrs: vec![
                Attr::String("parent_val".to_owned()),
                Attr::Label(Some(LabelOrFile::Label(Label::new(
                    PackageRef::root().to_owned(),
                    "custom_val".to_owned()
                )))),
                Attr::String("child_val".to_owned()),
            ],
            output_type: None,
            rule: rule(&pure, "child_rule"),
            cxx_attrs: HashMap::new(),
            dependencies: Mutex::new(HashSet::from([(
                Label::new(PackageRef::root().to_owned(), "custom_val".to_owned()),
                toolchain.clone(),
            )])),
        }
    );
}
