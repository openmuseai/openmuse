//! 03_function_convert_bullet — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_convert_bullet"]
fn bu_none() {
    // buNone / 公开入口: phType=1 → type=-1 且不 panic
    unimplemented!("spec: 03_function_convert_bullet BU-NONE");
}

#[test]
#[ignore = "spec: 03_function_convert_bullet"]
fn bu_char() {
    // buChar / 关键分支: • → type=1 visible
    unimplemented!("spec: 03_function_convert_bullet BU-CHAR");
}

#[test]
#[ignore = "spec: 03_function_convert_bullet"]
fn bu_num() {
    // buAutoNum / 关键分支: autonum → type=2 不 panic
    unimplemented!("spec: 03_function_convert_bullet BU-NUM");
}

#[test]
#[ignore = "spec: 03_function_convert_bullet"]
fn bu_ph10() {
    // phType 10 / 边界: phType=10 → -1
    unimplemented!("spec: 03_function_convert_bullet BU-PH10");
}

#[test]
#[ignore = "spec: 03_function_convert_bullet"]
fn bu_ex() {
    // exToInternal / 状态转换: fixture → 不 panic
    unimplemented!("spec: 03_function_convert_bullet BU-EX");
}
