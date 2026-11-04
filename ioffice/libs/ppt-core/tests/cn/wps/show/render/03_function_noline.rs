//! 03_function_noline — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_noline"]
fn nl_no() {
    // noline / 公开入口: 有几何无 ln 色 → stroke None 且无宽
    unimplemented!("spec: 03_function_noline NL-NO");
}

#[test]
#[ignore = "spec: 03_function_noline"]
fn nl_empty() {
    // 空样式 / 回归: pr-07 fixture → 无描边
    unimplemented!("spec: 03_function_noline NL-EMPTY");
}
