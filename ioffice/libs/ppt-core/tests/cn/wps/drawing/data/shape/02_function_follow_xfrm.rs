//! 02_function_follow_xfrm — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_follow_xfrm"]
fn fl_x() {
    // 无本地 xfrm / 公开入口: parent off/ext → off/ext 相同
    unimplemented!("spec: 02_function_follow_xfrm FL-X");
}

#[test]
#[ignore = "spec: 02_function_follow_xfrm"]
fn fl_xh() {
    // hasXfrm / 关键分支: 仅 parent 有 key2 → true
    unimplemented!("spec: 02_function_follow_xfrm FL-XH");
}
