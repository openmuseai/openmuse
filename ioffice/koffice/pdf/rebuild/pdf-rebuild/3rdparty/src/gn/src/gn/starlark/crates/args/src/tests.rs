// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use starlark::{environment::GlobalsBuilder, eval::Evaluator, values::Value};
use starlark_derive::starlark_module;

use crate::args::Args;

#[starlark_module]
pub(crate) fn register_args_test_globals(builder: &mut GlobalsBuilder) {
    fn args<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<Value<'v>> {
        Ok(eval.heap().alloc(Args::default()))
    }

    fn identity<'v>(x: Value<'v>) -> starlark::Result<Value<'v>> {
        Ok(x)
    }
}

struct ArgsAssert {
    assert: testutils::Assert,
}

impl ArgsAssert {
    fn new() -> Self {
        let mut a = testutils::Assert::default();
        a.modify_globals(|builder| {
            register_args_test_globals(builder);
            depset::depset_globals!(builder, testutils::FakeEvalContext);
        });
        Self { assert: a }
    }

    #[track_caller]
    fn assert_expands_to(&mut self, code: &str, expected: Result<&[&str], &str>) {
        use starlark::values::UnpackValue as _;

        use crate::unpack::FrozenArgsSequence;

        let val = self.assert.pass(code);

        let res = starlark::environment::Module::with_temp_heap(|module| {
            let mut eval = starlark::eval::Evaluator::new(&module);
            let val_ref = eval.heap().access_owned_frozen_value(&val);
            let seq = FrozenArgsSequence::unpack_value_err(val_ref).unwrap();
            seq.expand(&mut eval)
        });

        match expected {
            Ok(expected_cmd) => {
                assert_eq!(res.unwrap(), expected_cmd);
            },
            Err(expected_err) => {
                let err_str = res.unwrap_err().to_string();
                assert!(
                    err_str.contains(expected_err),
                    "Expected error containing: {expected_err}\nGot error: {err_str}"
                );
            },
        }
    }

    #[track_caller]
    fn fail(&mut self, code: &str, expected_error: &str) {
        self.assert.fail(code, expected_error);
    }
}

#[test]
fn test_combine_string_and_args() {
    let mut a = ArgsAssert::new();
    a.assert_expands_to(
        r#"["foo", args().add("bar"), "baz"]"#,
        Ok(&["foo", "bar", "baz"]),
    );
}

#[test]
fn test_args_add() {
    let mut a = ArgsAssert::new();
    a.assert_expands_to(r#"[args().add("--foo")]"#, Ok(&["--foo"]));

    a.assert_expands_to(r#"[args().add(make_file("a/b.cc"))]"#, Ok(&["a/b.cc"]));

    a.assert_expands_to(r#"[args().add("--bar", "baz")]"#, Ok(&["--bar", "baz"]));

    a.assert_expands_to(r#"[args().add("--qux", None)]"#, Ok(&[]));

    a.assert_expands_to(
        r#"[args().add("--val", 1, format="before %s after")]"#,
        Ok(&["--val", "before 1 after"]),
    );

    a.fail(
        r#"[args().add("--val", 1, format="no percent s")]"#,
        "Format string must contain exactly one '%s'",
    );

    a.fail(
        r#"[args().add("--val", 1, format="two %s %s")]"#,
        "Format string must contain exactly one '%s'",
    );
}

#[test]
fn test_args_add_all() {
    let mut a = ArgsAssert::new();
    a.assert_expands_to(r#"[args().add_all([1, 2])]"#, Ok(&["1", "2"]));

    a.assert_expands_to(
        r#"[args().add_all("--flag", ["3", "4"], before_each = "-b")]"#,
        Ok(&["--flag", "-b", "3", "-b", "4"]),
    );

    a.assert_expands_to(
        r#"[args().add_all(["x", "y"], before_each="-b", format_each=".%s.", terminate_with="--end")]"#,
        Ok(&["-b", ".x.", "-b", ".y.", "--end"]),
    );

    a.assert_expands_to(
        r#"[args().add_all([make_file("x.cc"), make_file("y.cc")])]"#,
        Ok(&["x.cc", "y.cc"]),
    );

    a.assert_expands_to(
        r#"[args().add_all(["a", None])]"#,
        Err("None is not allowed unless mapped by map_each"),
    );
}

#[test]
fn test_args_add_joined() {
    let mut a = ArgsAssert::new();
    a.assert_expands_to(
        r#"[args().add_joined([1, 2], join_with = ',')]"#,
        Ok(&["1,2"]),
    );

    a.assert_expands_to(
        r#"[args().add_joined("--flag", ["c", "d"], join_with=":")]"#,
        Ok(&["--flag", "c:d"]),
    );

    a.assert_expands_to(
        r#"[args().add_joined(depset(["a", "b"]), join_with = ",")]"#,
        Ok(&["a,b"]),
    );

    a.assert_expands_to(
        r#"
[
  args()
    .add_joined(
      [make_file("a/b.cc"), make_file("c/d.cc")],
      join_with = ":",
    )
]
"#,
        Ok(&["a/b.cc:c/d.cc"]),
    );

    a.assert_expands_to(
        r#"[args().add_joined([1, 2], join_with=",", format_each=".%s.", format_joined="list=%s")]"#,
        Ok(&["list=.1.,.2."]),
    );
}

#[test]
fn test_args_omit_if_empty() {
    let mut a = ArgsAssert::new();

    a.assert_expands_to(
        r#"[args().add_all("--flag", [], terminate_with = "after")]"#,
        Ok(&[]),
    );

    a.assert_expands_to(
        r#"[args().add_joined("--flag", [], join_with=",", omit_if_empty=False)]"#,
        Ok(&["--flag", ""]),
    );

    a.assert_expands_to(
        r#"[args().add_all("--flag", [None], map_each=identity, allow_closure=False, omit_if_empty=True)]"#,
        Ok(&[]),
    );
}

#[test]
fn test_args_map_each() {
    let mut a = ArgsAssert::new();

    a.assert_expands_to(
        r#"[args().add_all(["abc", ["def", "ghi"], None], map_each=identity, allow_closure=False)]"#,
        Ok(&["abc", "def", "ghi"]),
    );

    a.assert_expands_to(
        r#"[args().add_all([[1]], map_each=identity, allow_closure=False)]"#,
        Err("map_each must return a list[str], str, or None"),
    );

    a.assert_expands_to(
        r#"[args().add_all([1], map_each=identity, allow_closure=False)]"#,
        Err("map_each must return a list[str], str, or None"),
    );

    a.fail(
        r#"[args().add_all([1], map_each=identity)]"#,
        "map_each was specified without allow_closure",
    );
    a.assert_expands_to(
        r#"[args().add_all([1], map_each=fail, allow_closure=False)]"#,
        Err("fail: 1"),
    );
}

#[test]
fn test_args_uniquify() {
    let mut a = ArgsAssert::new();

    a.assert_expands_to(
        r#"[args().add_all(["a", "b", "a", "c"])]"#,
        Ok(&["a", "b", "a", "c"]),
    );

    a.assert_expands_to(
        r#"[args().add_all(["a", "b", "a", "c"], uniquify=True)]"#,
        Ok(&["a", "b", "c"]),
    );

    a.assert_expands_to(
        r#"[args().add_joined(["a", "b", "a", "c"], join_with=",", uniquify=True)]"#,
        Ok(&["a,b,c"]),
    );

    a.assert_expands_to(
        r#"[args().add_all(["a", "b", "a", "c"], before_each="-b", uniquify=True)]"#,
        Ok(&["-b", "a", "-b", "b", "-b", "c"]),
    );

    a.assert_expands_to(
        r#"
def map_to_constant(x):
  return "constant"

[args().add_joined(["a", "b", "c"], map_each=map_to_constant, allow_closure=False, uniquify=True, join_with=",")]
"#,
        Ok(&["constant"]),
    );
}

#[test]
fn test_args_chaining() {
    let mut a = ArgsAssert::new();
    a.assert_expands_to(
        r#"
[
  args()
    .add("--foo")
    .add_all(["bar", "baz"])
    .add_joined(["x", "y"], join_with=":")
]
"#,
        Ok(&["--foo", "bar", "baz", "x:y"]),
    );
}

#[test]
fn test_mutate_frozen_args_fails() {
    let mut a = ArgsAssert::new();
    a.assert.modify_globals(|builder| {
        builder.set("frozen_args", crate::args::FrozenArgs::default());
    });
    a.fail(
        "frozen_args.add('foo')",
        "trying to mutate a frozen Args value",
    );
}
