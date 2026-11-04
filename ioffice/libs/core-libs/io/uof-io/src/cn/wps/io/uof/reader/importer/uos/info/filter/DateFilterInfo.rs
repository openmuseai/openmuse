// Java: public class DateFilterInfo
pub struct DateFilterInfo {
    // Java field: int year
    pub year: i32,
    // Java field: int month
    pub month: i32,
    // Java field: int day
    pub day: i32,
    // Java field: int hour
    pub hour: i32,
    // Java field: int minute
    pub minute: i32,
    // Java field: int second
    pub second: i32,
    // Java field: String group
    pub group: Option<String>,
}

// Java: public class DateFilterInfo
impl DateFilterInfo {
    // Java: DateFilterInfo()
    pub fn DateFilterInfo() -> Self {
      // Java: super();
      DateFilterInfo {
          year: 0,
          month: 0,
          day: 0,
          hour: 0,
          minute: 0,
          second: 0,
          group: None,
      }
    }

}
