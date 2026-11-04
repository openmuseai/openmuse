//! 01_function_stroke_width_px — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_stroke_width_px"]
fn sw_2pt() {
    // 25400 / 公开入口: w=25400 → 指令宽==fromEmu(25400)
    unimplemented!("spec: 01_function_stroke_width_px SW-2PT");
}

#[test]
#[ignore = "spec: 01_function_stroke_width_px"]
fn sw_min() {
    // min 12700 / 边界: w<12700 但有线 → 按 Java min
    unimplemented!("spec: 01_function_stroke_width_px SW-MIN");
}
