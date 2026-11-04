// Java: package cn.wps.moffice.writer.data.value;
// Java: public class DateTime implements Cloneable
#[derive(Default, Clone, Debug, PartialEq)]
pub struct DateTime {
    // java private int minute;
    pub minute: i32,
    // java private int hour;
    pub hour: i32,
    // java private int day;
    pub day: i32,
    // java private int month;
    pub month: i32,
    // java private int year;
    pub year: i32,
    // java private int weekday;
    pub weekday: i32,
}

impl DateTime {
    // java public DateTime()
    pub fn DateTime() -> Self {
      // java this(0, 0, 1, 1, 1900, 0);
      Self::DateTime__int_int_int_int_int_int(0, 0, 1, 1, 1900, 0)
    }

    // java public DateTime(int minute, int hour, int day, int month, int year, int weekday)
    pub fn DateTime__int_int_int_int_int_int(minute: i32, hour: i32, day: i32, month: i32, year: i32, weekday: i32) -> Self {
      // java this.minute = minute;
      // java this.hour = hour;
      // java this.day = day;
      // java this.month = month;
      // java this.year = year;
      // java this.weekday = weekday;
      Self { minute, hour, day, month, year, weekday }
    }

    // java public int getMinute()
    pub fn getMinute(&self) -> i32 {
      // java return this.minute;
      return self.minute;
    }

    // java public void setMinute(int minute)
    pub fn setMinute(&mut self, minute: i32) {
      // java this.minute = minute;
      self.minute = minute;
    }

    // java public int getHour()
    pub fn getHour(&self) -> i32 {
      // java return this.hour;
      return self.hour;
    }

    // java public void setHour(int hour)
    pub fn setHour(&mut self, hour: i32) {
      // java this.hour = hour;
      self.hour = hour;
    }

    // java public int getDay()
    pub fn getDay(&self) -> i32 {
      // java return this.day;
      return self.day;
    }

    // java public void setDay(int day)
    pub fn setDay(&mut self, day: i32) {
      // java this.day = day;
      self.day = day;
    }

    // java public int getMonth()
    pub fn getMonth(&self) -> i32 {
      // java return this.month;
      return self.month;
    }

    // java public void setMonth(int month)
    pub fn setMonth(&mut self, month: i32) {
      // java this.month = month;
      self.month = month;
    }

    // java public int getYear()
    pub fn getYear(&self) -> i32 {
      // java return this.year;
      return self.year;
    }

    // java public void setYear(int year)
    pub fn setYear(&mut self, year: i32) {
      // java this.year = year;
      self.year = year;
    }

    // java public int getWeekday()
    pub fn getWeekday(&self) -> i32 {
      // java return this.weekday;
      return self.weekday;
    }

    // java public void setWeekday(int weekday)
    pub fn setWeekday(&mut self, weekday: i32) {
      // java this.weekday = weekday;
      self.weekday = weekday;
    }

    // java public boolean equals(Object object)
    pub fn equals(&self, object: Option<&DateTime>) -> bool {
      // java if (object != null && object instanceof DateTime)
      if let Some(dateTime) = object {
        // java DateTime dateTime = (DateTime)object;
        // java return this.minute == dateTime.minute && this.hour == dateTime.hour && this.day == dateTime.day && this.month == dateTime.month && this.year == dateTime.year && this.weekday == dateTime.weekday;
        return self.minute == dateTime.minute && self.hour == dateTime.hour && self.day == dateTime.day && self.month == dateTime.month && self.year == dateTime.year && self.weekday == dateTime.weekday;
      }
      // java else
        // java return false;
        return false;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.minute + this.hour + this.day + this.month + this.year + this.weekday;
      return self.minute + self.hour + self.day + self.month + self.year + self.weekday;
    }

    // java public DateTime clone() throws CloneNotSupportedException
    pub fn clone_(&self) -> DateTime {
      // java DateTime dateTime = (DateTime)super.clone();
      let mut dateTime = self.clone();
      // java dateTime.day = this.day;
      dateTime.day = self.day;
      // java dateTime.hour = this.hour;
      dateTime.hour = self.hour;
      // java dateTime.minute = this.minute;
      dateTime.minute = self.minute;
      // java dateTime.month = this.weekday;
      // rust: ★忠实复现 Java 原文缺陷（`month` 赋成了 `weekday`）——
      //   「移植不是'改进 Java'，是'复现 Java'。原文的缺陷属于规格的一部分。」
      dateTime.month = self.weekday;
      // java dateTime.weekday = this.day;
      // rust: ★同上，忠实复现原文缺陷（`weekday` 赋成了 `day`）。
      dateTime.weekday = self.day;
      // java dateTime.year = this.year;
      dateTime.year = self.year;
      // java return dateTime;
      return dateTime;
    }

    // java public Boolean isBefore(DateTime compareDateTime)
    pub fn isBefore(&self, compareDateTime: &DateTime) -> bool {
      // java if (this.year < compareDateTime.year)
      if self.year < compareDateTime.year {
        // java return true;
        return true;
      }
      // java else if (this.year > compareDateTime.year)
      else if self.year > compareDateTime.year {
        // java return false;
        return false;
      }
      // java else if (this.month < compareDateTime.month)
      else if self.month < compareDateTime.month {
        // java return true;
        return true;
      }
      // java else if (this.month > compareDateTime.month)
      else if self.month > compareDateTime.month {
        // java return false;
        return false;
      }
      // java else if (this.day < compareDateTime.day)
      else if self.day < compareDateTime.day {
        // java return true;
        return true;
      }
      // java else if (this.day > compareDateTime.day)
      else if self.day > compareDateTime.day {
        // java return false;
        return false;
      }
      // java else if (this.hour < compareDateTime.hour)
      else if self.hour < compareDateTime.hour {
        // java return true;
        return true;
      }
      // java else if (this.hour > compareDateTime.hour)
      else if self.hour > compareDateTime.hour {
        // java return false;
        return false;
      }
      // java else if (this.minute < compareDateTime.minute)
      else if self.minute < compareDateTime.minute {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return this.minute > compareDateTime.minute ? false : false;
        // rust: ★忠实复现 Java 原文：三元两个分支都是 `false`（原文缺陷）。
        return if self.minute > compareDateTime.minute { false } else { false };
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java return String.format("%d-%d-%d %d:%d", this.year, this.month, this.day, this.hour, this.minute);
      return format!("{}-{}-{} {}:{}", self.year, self.month, self.day, self.hour, self.minute);
    }
}
