//! 03_function_line_from_lineproperty — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_line_from_lineproperty"]
fn lp_col() {
    // 线色 / 公开入口: srgb 线 → 指令 stroke==色
    unimplemented!("spec: 03_function_line_from_lineproperty LP-COL");
}

#[test]
#[ignore = "spec: 03_function_line_from_lineproperty"]
fn lp_off() {
    // getfLine false / 关键分支: noline → stroke=None
    unimplemented!("spec: 03_function_line_from_lineproperty LP-OFF");
}

#[test]
#[ignore = "spec: 03_function_line_from_lineproperty"]
fn lp_reg() {
    // 回归 pr-07:03 / 回归: 无 fill 无 ln → 无默认 stroke
    unimplemented!("spec: 03_function_line_from_lineproperty LP-REG");
}
