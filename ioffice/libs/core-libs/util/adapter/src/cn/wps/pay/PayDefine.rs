// Java field: String PAY_JAPAN_CODE
static PAY_JAPAN_CODE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String PAY_KNOX_CODE
static PAY_KNOX_CODE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String PAY_ENT_CODE
static PAY_ENT_CODE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: int PAY_TRY_DAYS_WEEK
static PAY_TRY_DAYS_WEEK: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int PAY_TRY_DAYS_MONTH
static PAY_TRY_DAYS_MONTH: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int PAY_TRY_DAYS_HALF_YEAR
static PAY_TRY_DAYS_HALF_YEAR: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int PAY_TRY_DAYS_HALFYEAR
static PAY_TRY_DAYS_HALFYEAR: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

fn get_PAY_JAPAN_CODE() -> &'static std::sync::Mutex<String> {
    PAY_JAPAN_CODE.get_or_init(|| std::sync::Mutex::new("000004".to_string()))
}
fn get_PAY_KNOX_CODE() -> &'static std::sync::Mutex<String> {
    PAY_KNOX_CODE.get_or_init(|| std::sync::Mutex::new("000005".to_string()))
}
fn get_PAY_ENT_CODE() -> &'static std::sync::Mutex<String> {
    PAY_ENT_CODE.get_or_init(|| std::sync::Mutex::new("000006".to_string()))
}
fn get_PAY_TRY_DAYS_WEEK() -> &'static std::sync::Mutex<i32> {
    PAY_TRY_DAYS_WEEK.get_or_init(|| std::sync::Mutex::new(7))
}
fn get_PAY_TRY_DAYS_MONTH() -> &'static std::sync::Mutex<i32> {
    PAY_TRY_DAYS_MONTH.get_or_init(|| std::sync::Mutex::new(30))
}
fn get_PAY_TRY_DAYS_HALF_YEAR() -> &'static std::sync::Mutex<i32> {
    PAY_TRY_DAYS_HALF_YEAR.get_or_init(|| std::sync::Mutex::new(90))
}
fn get_PAY_TRY_DAYS_HALFYEAR() -> &'static std::sync::Mutex<i32> {
    PAY_TRY_DAYS_HALFYEAR.get_or_init(|| std::sync::Mutex::new(180))
}

// Java: public class PayDefine
pub struct PayDefine {
}

// Java: public class PayDefine
impl PayDefine {
    // Java: PayDefine()
    pub fn new() -> Self {
      // Java: super();
      PayDefine {}
    }

}
