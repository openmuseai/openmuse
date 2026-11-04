//! 04_function_canrotate_group — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 04_function_canrotate_group"]
fn cr_sp() {
    // 普通 sp / 对照: 单 sp → true
    unimplemented!("spec: 04_function_canrotate_group CR-SP");
}

#[test]
#[ignore = "spec: 04_function_canrotate_group"]
fn cr_gf() {
    // graphicFrame / 关键分支: 空 gf → false
    unimplemented!("spec: 04_function_canrotate_group CR-GF");
}

#[test]
#[ignore = "spec: 04_function_canrotate_group"]
fn cr_grp_ok() {
    // 组全可转 / 公开入口: 子皆 true → true
    unimplemented!("spec: 04_function_canrotate_group CR-GRP-OK");
}

#[test]
#[ignore = "spec: 04_function_canrotate_group"]
fn cr_grp_no() {
    // 组含 gf / 关键分支: 一子 false → 组 false
    unimplemented!("spec: 04_function_canrotate_group CR-GRP-NO");
}
