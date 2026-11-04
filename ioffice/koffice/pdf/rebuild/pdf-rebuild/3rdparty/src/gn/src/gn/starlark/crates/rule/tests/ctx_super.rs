// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use rule::run;
use starlark::values::list::ListRef;
use testutils::Assert;
use types::{Label, PackageRef, Session};

#[test]
fn test_ctx_super() {
    let mut assert = Assert::new_rule_assert();
    assert.load_module("//rules:pure.scl");
    assert.pass(
        r#"
load("//rules:pure.scl", "child_rule", "parent_rule")

parent_rule(
    name = "child",
    parent_only = "child_parent_val",
)

child_rule(
    name = "target",
    parent_only = "parent_val",
    child_only = "child_val",
)
"#,
    );

    let context = assert.context();
    let label = Label::new(PackageRef::root().to_owned(), "target".to_owned());
    let target = context
        .session
        .get_target(label.as_ref(), context.session.default_toolchain.as_ref());

    let session = assert.session();
    let res = run(&target, move |t: &testutils::FakeTargetRef| {
        testutils::FakeEvalContext::rule_impl(session.clone(), t.clone())
    })
    .unwrap();

    let list = ListRef::from_value(res.value()).unwrap();
    let items: Vec<starlark::values::Value<'_>> = list.iter().collect();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].to_repr(), r#"ParentInfo(parent = "parent_val")"#);
    assert_eq!(items[1].to_repr(), r#"ChildInfo(child = "child_val")"#);
}
