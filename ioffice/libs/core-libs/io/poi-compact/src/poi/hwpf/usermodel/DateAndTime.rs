// Java: import java.util.Calendar
// TODO_RUST_CODE use ???; // java.util.Calendar
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = 4;
// Java field: BitField _minute
static _minute: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _hour
static _hour: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _dom
static _dom: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _month
static _month: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _year
static _year: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _weekday
static _weekday: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class DateAndTime implements Cloneable
pub struct DateAndTime {
    // Java field: short _info
    pub(crate) _info: i16,
    // Java field: short _info2
    _info2: i16,
}

// Java: public class DateAndTime implements Cloneable
impl DateAndTime {
    // Java: DateAndTime()
    pub fn DateAndTime(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: DateAndTime(byte[] buf, int offset)
    pub fn DateAndTime(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = LittleEndian.getShort(buf, offset);
      // TODO_RUST_CODE
      // Java: this._info2 = LittleEndian.getShort(buf, offset + 2);
      // TODO_RUST_CODE
    }

    // Java: DateAndTime(DocumentInputStream buf, int offset)
    pub fn DateAndTime(&self, DocumentInputStream buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: buf.seek((long)offset);
      // TODO_RUST_CODE
      // Java: this._info = buf.readShort();
      // TODO_RUST_CODE
      // Java: this._info2 = buf.readShort();
      // TODO_RUST_CODE
    }

    // Java: DateAndTime(int minute, int hour, int day, int month, int year, int weekday)
    pub fn DateAndTime(&self, int minute, int hour, int day, int month, int year, int weekday) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = _minute.setShortValue(this._info, (short)minute);
      // TODO_RUST_CODE
      // Java: this._info = _hour.setShortValue(this._info, (short)hour);
      // TODO_RUST_CODE
      // Java: this._info = _dom.setShortValue(this._info, (short)day);
      // TODO_RUST_CODE
      // Java: this._info2 = _month.setShortValue(this._info2, (short)month);
      // TODO_RUST_CODE
      // Java: this._info2 = _year.setShortValue(this._info2, (short)year);
      // TODO_RUST_CODE
      // Java: this._info2 = _weekday.setShortValue(this._info2, (short)weekday);
      // TODO_RUST_CODE
    }

    // Java: Calendar getDate()
    pub fn getDate(&self) -> Calendar {
      // Java: Calendar cal = Calendar.getInstance();
      // TODO_RUST_CODE
      // Java: cal.set(_year.getValue(this._info2) + 1900, _month.getValue(this._info2) - 1, _dom.getValue(this._info), _hour.getValue(this._info), _minute.getValue(this._info), 0);
      // TODO_RUST_CODE
      // Java: cal.set(14, 0);
      // TODO_RUST_CODE
      // Java: return cal;
      // TODO_RUST_CODE
    }

    // Java: int getMinute()
    pub fn getMinute(&self) -> i32 {
      // Java: return _minute.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setMinute(int minute)
    pub fn setMinute(&self, int minute) {
      // Java: this._info = _minute.setShortValue(this._info, (short)minute);
      // TODO_RUST_CODE
    }

    // Java: int getHour()
    pub fn getHour(&self) -> i32 {
      // Java: return _hour.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setHour(int hour)
    pub fn setHour(&self, int hour) {
      // Java: this._info = _hour.setShortValue(this._info, (short)hour);
      // TODO_RUST_CODE
    }

    // Java: int getDay()
    pub fn getDay(&self) -> i32 {
      // Java: return _dom.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: void setDay(int day)
    pub fn setDay(&self, int day) {
      // Java: this._info = _dom.setShortValue(this._info, (short)day);
      // TODO_RUST_CODE
    }

    // Java: int getMonth()
    pub fn getMonth(&self) -> i32 {
      // Java: return _month.getShortValue(this._info2);
      // TODO_RUST_CODE
    }

    // Java: void setMonth(int month)
    pub fn setMonth(&self, int month) {
      // Java: this._info2 = _month.setShortValue(this._info2, (short)month);
      // TODO_RUST_CODE
    }

    // Java: int getYear()
    pub fn getYear(&self) -> i32 {
      // Java: return _year.getShortValue(this._info2);
      // TODO_RUST_CODE
    }

    // Java: void setYear(int year)
    pub fn setYear(&self, int year) {
      // Java: this._info2 = _year.setShortValue(this._info2, (short)year);
      // TODO_RUST_CODE
    }

    // Java: int getWeekday()
    pub fn getWeekday(&self) -> i32 {
      // Java: return _weekday.getShortValue(this._info2);
      // TODO_RUST_CODE
    }

    // Java: void setWeekday(int weekday)
    pub fn setWeekday(&self, int weekday) {
      // Java: this._info2 = _weekday.setShortValue(this._info2, (short)weekday);
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._info);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, offset + 2, this._info2);
      // TODO_RUST_CODE
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      // TODO_RUST_CODE
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, Object o) -> bool {
      // Java: if (o == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: DateAndTime dttm = (DateAndTime)o;
         // TODO_RUST_CODE
         // Java: return this._info == dttm._info && this._info2 == dttm._info2;
         // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

}

impl Cloneable for DateAndTime {
}
