//! 01_function_anchor_1_2 — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_anchor_1_2"]
fn an_0() {
    // 顶 / 对照: 短文+高框 → 首行 y≈textRect.top
    unimplemented!("spec: 01_function_anchor_1_2 AN-0");
}

#[test]
#[ignore = "spec: 01_function_anchor_1_2"]
fn an_1() {
    // 居中 / 公开入口: 短文+高框 → extra≈(avail-h)/2
    unimplemented!("spec: 01_function_anchor_1_2 AN-1");
}

#[test]
#[ignore = "spec: 01_function_anchor_1_2"]
fn an_2() {
    // 底 / 公开入口: 短文+高框 → extra≈avail-h
    unimplemented!("spec: 01_function_anchor_1_2 AN-2");
}

#[test]
#[ignore = "spec: 01_function_anchor_1_2"]
fn an_eq() {
    // 等高 / 边界: 填满 → extra≈0
    unimplemented!("spec: 01_function_anchor_1_2 AN-EQ");
}
