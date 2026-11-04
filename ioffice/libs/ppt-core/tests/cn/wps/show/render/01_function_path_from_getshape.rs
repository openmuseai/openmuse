//! 01_function_path_from_getshape — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_path_from_getshape"]
fn gp_rect() {
    // rect path / 公开入口: prst rect → 指令框==property(733)
    unimplemented!("spec: 01_function_path_from_getshape GP-RECT");
}

#[test]
#[ignore = "spec: 01_function_path_from_getshape"]
fn gp_ell() {
    // ellipse / 关键分支: prstGeom=ellipse → DrawPath 点数与 GraphicsPath 一致
    unimplemented!("spec: 01_function_path_from_getshape GP-ELL");
}

#[test]
#[ignore = "spec: 01_function_path_from_getshape"]
fn gp_miss() {
    // getShape None / 失败: 无法 convert → 0 条几何或显式 skip，不 panic
    unimplemented!("spec: 01_function_path_from_getshape GP-MISS");
}
