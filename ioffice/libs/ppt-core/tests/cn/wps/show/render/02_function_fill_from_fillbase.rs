//! 02_function_fill_from_fillbase — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_fill_from_fillbase"]
fn ff_sol() {
    // 实色 / 公开入口: srgb 红 → 指令 fill==ARGB
    unimplemented!("spec: 02_function_fill_from_fillbase FF-SOL");
}

#[test]
#[ignore = "spec: 02_function_fill_from_fillbase"]
fn ff_no() {
    // noFill / 关键分支: a:noFill → fill=None
    unimplemented!("spec: 02_function_fill_from_fillbase FF-NO");
}

#[test]
#[ignore = "spec: 02_function_fill_from_fillbase"]
fn ff_bg() {
    // Background 层 / 边界: 页底实色 → 仍发 Background fill（回归 pp-17）
    unimplemented!("spec: 02_function_fill_from_fillbase FF-BG");
}
