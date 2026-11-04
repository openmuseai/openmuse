// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: int UNIT_UNUSED
pub(crate) const UNIT_UNUSED: i32 = -1;
// Java field: int UNIT_DEFAULT
pub(crate) const UNIT_DEFAULT: i32 = 0;
// Java field: int UNIT_INCH
pub(crate) const UNIT_INCH: i32 = 1;
// Java field: int UNIT_CENTIMETER
pub(crate) const UNIT_CENTIMETER: i32 = 2;
// Java field: int UNIT_DEGREE
pub(crate) const UNIT_DEGREE: i32 = 3;
// Java field: int UNIT_RADIAN
pub(crate) const UNIT_RADIAN: i32 = 4;
// Java field: int UNIT_SECOND
pub(crate) const UNIT_SECOND: i32 = 5;
// Java field: int UNIT_POUND
pub(crate) const UNIT_POUND: i32 = 6;
// Java field: int UNIT_GRAM
pub(crate) const UNIT_GRAM: i32 = 7;
// Java field: String STR_UNIT_UNUSED
pub(crate) const STR_UNIT_UNUSED: &str = "UNIT_UNUSED";
// Java field: String STR_UNIT_DEFAULT
pub(crate) const STR_UNIT_DEFAULT: &str = "UNIT_DEFAULT";
// Java field: String STR_UNIT_INCH
pub(crate) const STR_UNIT_INCH: &str = "UNIT_INCH";
// Java field: String STR_UNIT_CENTIMETER
pub(crate) const STR_UNIT_CENTIMETER: &str = "UNIT_CENTIMETER";
// Java field: String STR_UNIT_DEGREE
pub(crate) const STR_UNIT_DEGREE: &str = "UNIT_DEGREE";
// Java field: String STR_UNIT_RADIAN
pub(crate) const STR_UNIT_RADIAN: &str = "UNIT_RADIAN";
// Java field: String STR_UNIT_SECOND
pub(crate) const STR_UNIT_SECOND: &str = "UNIT_SECOND";
// Java field: String STR_UNIT_POUND
pub(crate) const STR_UNIT_POUND: &str = "UNIT_POUND";
// Java field: String STR_UNIT_GRAM
pub(crate) const STR_UNIT_GRAM: &str = "UNIT_GRAM";

// Java field: HashMap<Integer, String> sInteger2StringMap
fn get_sInteger2StringMap() -> &'static HashMap<i32, &'static str> {
    static MAP: OnceLock<HashMap<i32, &str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(UNIT_UNUSED, "UNIT_UNUSED");
        m.insert(UNIT_DEFAULT, "UNIT_DEFAULT");
        m.insert(UNIT_INCH, "UNIT_INCH");
        m.insert(UNIT_CENTIMETER, "UNIT_CENTIMETER");
        m.insert(UNIT_DEGREE, "UNIT_DEGREE");
        m.insert(UNIT_RADIAN, "UNIT_RADIAN");
        m.insert(UNIT_SECOND, "UNIT_SECOND");
        m.insert(UNIT_POUND, "UNIT_POUND");
        m.insert(UNIT_GRAM, "UNIT_GRAM");
        m
    })
}

// Java: public class MetricScale
pub struct MetricScale;

// Java: public class MetricScale
impl MetricScale {
    // Java: MetricScale()
    pub fn MetricScale() -> Self {
        // Java: super();
        MetricScale
    }

    // Java: static String toName(int id)
    pub fn toName(id: i32) -> String {
        // Java: return (String)sInteger2StringMap.get(id);
        get_sInteger2StringMap()
            .get(&id)
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
}
