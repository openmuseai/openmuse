//! 02_function_ph_type_default — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_ph_type_default"]
fn ph8_def() {
    // 无 type / 公开入口: hasType=false parent=null → 8
    unimplemented!("spec: 02_function_ph_type_default PH8-DEF");
}

#[test]
#[ignore = "spec: 02_function_ph_type_default"]
fn ph8_has() {
    // 有 type / 对照: body=1 → 1
    unimplemented!("spec: 02_function_ph_type_default PH8-HAS");
}

#[test]
#[ignore = "spec: 02_function_ph_type_default"]
fn ph8_mrg() {
    // phType 末路 / 关键分支: 无 layout/master → 8
    unimplemented!("spec: 02_function_ph_type_default PH8-MRG");
}
