//! 03_function_not_draw_background — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_not_draw_background"]
fn bg_off() {
    // bit8 / 公开入口: 有 bgPr 的页 → 无 Background 指令
    unimplemented!("spec: 03_function_not_draw_background BG-OFF");
}

#[test]
#[ignore = "spec: 03_function_not_draw_background"]
fn bg_on() {
    // flags=0 / 回归: bgPr 实色 → 有 Background
    unimplemented!("spec: 03_function_not_draw_background BG-ON");
}
