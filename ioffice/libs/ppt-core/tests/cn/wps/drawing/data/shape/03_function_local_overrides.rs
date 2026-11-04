//! 03_function_local_overrides — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_local_overrides"]
fn ov_f() {
    // 本地 fill / 公开入口: 两边都有 fill → 仍本地色
    unimplemented!("spec: 03_function_local_overrides OV-F");
}

#[test]
#[ignore = "spec: 03_function_local_overrides"]
fn ov_x() {
    // 本地 xfrm / 对照: 本地框 → 仍本地框
    unimplemented!("spec: 03_function_local_overrides OV-X");
}

#[test]
#[ignore = "spec: 03_function_local_overrides"]
fn ov_mix() {
    // 只覆盖 fill / 状态: parent 有 xfrm → fill 本地 + xfrm parent
    unimplemented!("spec: 03_function_local_overrides OV-MIX");
}
