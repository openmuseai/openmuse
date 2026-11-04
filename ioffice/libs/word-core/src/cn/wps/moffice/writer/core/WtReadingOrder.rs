// java public enum WtReadingOrder
// rust: ★M-STAGE2 P1-D1 / T7 —— **补全不完整移植**。原为空枚举，而 Java 原文有 2 个常量
//   （`wtReadingOrderLtr`, `wtReadingOrderRtl`）⇒ 忠实补齐（空枚举会使一切取值无解）。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum WtReadingOrder {
    // java wtReadingOrderLtr
    #[default]
    wtReadingOrderLtr,
    // java wtReadingOrderRtl
    wtReadingOrderRtl,
}

// java public enum WtReadingOrder
impl WtReadingOrder {
}
