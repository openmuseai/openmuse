// Java: public class Date 
//
// rust: ★T7-7 真实化 —— Java `java.util.Date` 承载自 epoch 起的毫秒值。
//   此前本桩为空结构体（仅 `new()`），使 `PLCField.initFields()` 的
//   `new Date((Long)updateVal)`（`PLCField.java:35`）与 `new Date()`
//   （`PLCField.java:40`）无法区分"给定时刻"与"当前时刻"；
//   域代码 `TIME` / `DATE` / `CREATEDATE` / `SAVEDATE` / `PRINTDATE` 的
//   取值都依赖该毫秒值（经 `field::parser::datetime::DateTimeParser`）。
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Date {
    // Java: private transient long fastTime;
    fastTime: i64,
}

// Java: public class Date 
impl Date {
    pub fn new() -> Self { Self::default() }

    // Java: public Date(long date) { this.fastTime = date; }
    // rust: 重载门面沿用仓库 `name__argtypes` 约定。
    pub fn new__long(date: i64) -> Self {
        Self { fastTime: date }
    }

    // Java: public long getTime() { return fastTime; }
    pub fn getTime(&self) -> i64 {
        self.fastTime
    }
}
