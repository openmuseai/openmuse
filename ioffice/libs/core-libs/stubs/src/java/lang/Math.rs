#![allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct Math;
impl Math {
    pub fn new() -> Self { Self }
    // rust: ★M-STAGE2 P1-D1 / T7-2 —— **破损桩修复**。
    //   Java `Math.min`/`max`/`abs` 对 `int`/`long`/`float`/`double` 有多重载，
    //   原桩只实现 `i32` ⇒ 一切 `float` 调用点（`DefaultRunProperties` 等）编译失败。
    //   Rust 无重载 ⇒ 以泛型忠实覆盖 Java 的全部数值重载，且调用点无需改名。
    pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T { if a < b { a } else { b } }
    pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T { if a > b { a } else { b } }
    // Java: static int abs(int a)
    pub fn abs(a: i32) -> i32 { a.abs() }
    // rust: Java `Math.abs` 的 float/double 重载（模板命名约定 `name__<类型>`）。
    pub fn abs__float(a: f32) -> f32 { a.abs() }
    pub fn abs__double(a: f64) -> f64 { a.abs() }
}
