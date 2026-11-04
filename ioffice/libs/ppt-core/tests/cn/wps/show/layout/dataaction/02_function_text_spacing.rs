//! 02_function_text_spacing — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 02_function_text_spacing"]
fn sp_pts() {
    // spcPts / 公开入口: pts=100 → point100ToLayoutUnits(100)
    unimplemented!("spec: 02_function_text_spacing SP-PTS");
}

#[test]
#[ignore = "spec: 02_function_text_spacing"]
fn sp_pct() {
    // spcPct / 关键分支: pct + font 12 → 12*pct*1.2
    unimplemented!("spec: 02_function_text_spacing SP-PCT");
}

#[test]
#[ignore = "spec: 02_function_text_spacing"]
fn sp_0() {
    // 皆无 / 边界: 空 spacing → 0.0
    unimplemented!("spec: 02_function_text_spacing SP-0");
}

#[test]
#[ignore = "spec: 02_function_text_spacing"]
fn sp_max() {
    // 多 span 字号 / 边界: max fontsize → 用 max
    unimplemented!("spec: 02_function_text_spacing SP-MAX");
}
