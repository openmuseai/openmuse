//! 02_function_multi_run_style — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_multi_run_style"]
fn mr_2() {
    // 两 run / 公开入口: AB 分色 → 2 条 DrawText 色不同
    unimplemented!("spec: 02_function_multi_run_style MR-2");
}

#[test]
#[ignore = "spec: 02_function_multi_run_style"]
fn mr_1() {
    // 单 run / 对照: 一字 → 1 条
    unimplemented!("spec: 02_function_multi_run_style MR-1");
}
