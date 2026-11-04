//! 03_function_textrect_inset — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_textrect_inset"]
fn in_d() {
    // 默认 inset / 公开入口: 有字 sp → text.x > shape.left
    unimplemented!("spec: 03_function_textrect_inset IN-D");
}

#[test]
#[ignore = "spec: 03_function_textrect_inset"]
fn in_c() {
    // 自定义 lIns / 关键分支: 显式 inset → x 更大
    unimplemented!("spec: 03_function_textrect_inset IN-C");
}
