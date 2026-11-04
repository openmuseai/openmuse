// java public static int countMonthDays(int nYear, int nMonth) {
// java Calendar c = new GregorianCalendar(nYear, nMonth, 0);
// java return c.getActualMaximum(5);
// java }
pub struct Accrint;impl Accrint{pub fn countMonthDays(y:i32,m:i32)->i32{let mut c=crate::java_runtime::Calendar::getInstance();c.set(y,m,0,0,0,0);let y=c.get(1);let m=c.get(2)+1;let next=chrono::NaiveDate::from_ymd_opt(y+(m==12)as i32,if m==12{1}else{m as u32+1},1).expect("Calendar range");use chrono::Datelike;(next-chrono::Duration::days(1)).day()as i32}}
