//! 01_function_follow_fill — spec 骨架。矩阵见 FUNCTION.md。Loop 接线后去掉 ignore。
//! 禁止 assert!(true)。ignore 仅表示实现未接线。

#[test]
#[ignore = "spec: 01_function_follow_fill"]
fn fl_f() {
    // 无本地 fill / 公开入口: slide 空 fill → 色=parent
    unimplemented!("spec: 01_function_follow_fill FL-F");
}

#[test]
#[ignore = "spec: 01_function_follow_fill"]
fn fl_live() {
    // 改 parent / 状态转换: follow 后改 parent → 再读 fill 新色
    unimplemented!("spec: 01_function_follow_fill FL-LIVE");
}
