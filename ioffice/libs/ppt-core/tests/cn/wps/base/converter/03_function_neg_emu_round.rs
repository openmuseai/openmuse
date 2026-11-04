//! 03_function_neg_emu_round — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 03_function_neg_emu_round"]
fn nr_pos() {
    // 正半数 / 对照: 1.5 → 2（两边同）
    unimplemented!("spec: 03_function_neg_emu_round NR-POS");
}

#[test]
#[ignore = "spec: 03_function_neg_emu_round"]
fn nr_neg() {
    // 负半数 / 公开入口: -1.5 → Java Math.round → -1
    unimplemented!("spec: 03_function_neg_emu_round NR-NEG");
}

#[test]
#[ignore = "spec: 03_function_neg_emu_round"]
fn nr_n2() {
    // 负过半 / 关键分支: -2.5 → Java -2
    unimplemented!("spec: 03_function_neg_emu_round NR-N2");
}
