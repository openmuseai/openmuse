//! 01_function_para_direction — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_para_direction"]
fn pd_l() {
    // ltr / 对照: 显式 ltr → false
    unimplemented!("spec: 01_function_para_direction PD-L");
}

#[test]
#[ignore = "spec: 01_function_para_direction"]
fn pd_r() {
    // rtl / 公开入口: 显式 rtl → true
    unimplemented!("spec: 01_function_para_direction PD-R");
}

#[test]
#[ignore = "spec: 01_function_para_direction"]
fn pd_par() {
    // 无本地沿 parent / 关键分支: follow 链 → true
    unimplemented!("spec: 01_function_para_direction PD-PAR");
}
