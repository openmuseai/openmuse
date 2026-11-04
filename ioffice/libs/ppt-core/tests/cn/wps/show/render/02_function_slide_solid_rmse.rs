//! 02_function_slide_solid_rmse — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_slide_solid_rmse"]
fn rm_thr() {
    // RMSE / 公开入口: 白+红矩形 → RMSE<2.0
    unimplemented!("spec: 02_function_slide_solid_rmse RM-THR");
}

#[test]
#[ignore = "spec: 02_function_slide_solid_rmse"]
fn rm_diff() {
    // 失败打印 / 失败: 阈值外 → 非静默 fail + diff 路径
    unimplemented!("spec: 02_function_slide_solid_rmse RM-DIFF");
}
