//! 01_function_raster_size — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_raster_size"]
fn rs_wh() {
    // 默认页 / 公开入口: 标准 10×7.5in → 栅格 w/h 与 Java fixture 相同
    unimplemented!("spec: 01_function_raster_size RS-WH");
}

#[test]
#[ignore = "spec: 01_function_raster_size"]
fn rs_0() {
    // 坏尺寸 / 失败: 空 sldSz → 错误不 panic 成 Inf RMSE
    unimplemented!("spec: 01_function_raster_size RS-0");
}
