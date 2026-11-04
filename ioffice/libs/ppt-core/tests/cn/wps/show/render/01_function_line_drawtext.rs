//! 01_function_line_drawtext — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_line_drawtext"]
fn td_1() {
    // 单行 / 公开入口: Hello → DrawText 数==1 且 xy 在 textRect
    unimplemented!("spec: 01_function_line_drawtext TD-1");
}

#[test]
#[ignore = "spec: 01_function_line_drawtext"]
fn td_v() {
    // 回归 vert / 回归: pr-07:02 → 仍竖排
    unimplemented!("spec: 01_function_line_drawtext TD-V");
}
