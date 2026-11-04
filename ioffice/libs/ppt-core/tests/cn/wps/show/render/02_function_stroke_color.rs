//! 02_function_stroke_color — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_stroke_color"]
fn sc_srgb() {
    // srgb / 公开入口: FF0000 → stroke==红
    unimplemented!("spec: 02_function_stroke_color SC-SRGB");
}

#[test]
#[ignore = "spec: 02_function_stroke_color"]
fn sc_sch() {
    // scheme / 关键分支: theme → 解析后非 0
    unimplemented!("spec: 02_function_stroke_color SC-SCH");
}
