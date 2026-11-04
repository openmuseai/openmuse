//! 01_function_hidden_skipped — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_hidden_skipped"]
fn hd_skip() {
    // hidden / 公开入口: isHidden=true → 0 条该 shape 指令
    unimplemented!("spec: 01_function_hidden_skipped HD-SKIP");
}

#[test]
#[ignore = "spec: 01_function_hidden_skipped"]
fn hd_show() {
    // 可见 / 对照: 普通 sp → 有几何指令
    unimplemented!("spec: 01_function_hidden_skipped HD-SHOW");
}
