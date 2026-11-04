//! 02_function_ph_hint_flag — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_ph_hint_flag"]
fn ph_off() {
    // bit16 / 关键分支: flags|=16 空 ph → 无提示 DrawText
    unimplemented!("spec: 02_function_ph_hint_flag PH-OFF");
}

#[test]
#[ignore = "spec: 02_function_ph_hint_flag"]
fn ph_on() {
    // 允许提示 / 对照: 编辑态 flags → 可有提示字
    unimplemented!("spec: 02_function_ph_hint_flag PH-ON");
}

#[test]
#[ignore = "spec: 02_function_ph_hint_flag"]
fn ph_body() {
    // 真正文 / 边界: ph 但有用户字 → 仍发射正文
    unimplemented!("spec: 02_function_ph_hint_flag PH-BODY");
}
