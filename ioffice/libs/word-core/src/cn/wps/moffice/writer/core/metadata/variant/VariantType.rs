// java public enum VariantType
//
// rust: ★T7-7 真实化 —— 原为空枚举桩，使 `VariantBase(VariantType)` /
//   `VariantWStr`（`super(VariantType.var_wstr)`）等构造不可用。
//   按 `VariantType.java` 逐个复原常量。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariantType {
    var_int,
    var_double,
    var_bool,
    var_date,
    var_wstr,
    var_vector,
}

// java public enum VariantType
impl VariantType {
}
