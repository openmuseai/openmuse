#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// Java field: int JP__ERA_M
pub(crate) const JP__ERA_M: i32 = 0;
// Java field: int JP_ERA_T
pub(crate) const JP_ERA_T: i32 = 1;
// Java field: int JP_ERA_S
pub(crate) const JP_ERA_S: i32 = 2;
// Java field: int JP_ERA_H
pub(crate) const JP_ERA_H: i32 = 3;
// Java field: byte NUMERAL_DEFAULT
pub(crate) const NUMERAL_DEFAULT: i8 = 0;
// Java field: byte NUMERAL_Western
pub(crate) const NUMERAL_Western: i8 = 1;
// Java field: byte NUMERAL_Arabic_Indic
pub(crate) const NUMERAL_Arabic_Indic: i8 = 2;
// Java field: byte NUMERAL_Extended_Arabic_Indic
pub(crate) const NUMERAL_Extended_Arabic_Indic: i8 = 3;
// Java field: byte NUMERAL_Devanagari
pub(crate) const NUMERAL_Devanagari: i8 = 4;
// Java field: byte NUMERAL_Bengali
pub(crate) const NUMERAL_Bengali: i8 = 5;
// Java field: byte NUMERAL_Gurmukhi
pub(crate) const NUMERAL_Gurmukhi: i8 = 6;
// Java field: byte NUMERAL_Gujarati
pub(crate) const NUMERAL_Gujarati: i8 = 7;
// Java field: byte NUMERAL_Oriya
pub(crate) const NUMERAL_Oriya: i8 = 8;
// Java field: byte NUMERAL_Tamil
pub(crate) const NUMERAL_Tamil: i8 = 9;
// Java field: byte NUMERAL_Telugu
pub(crate) const NUMERAL_Telugu: i8 = 10;
// Java field: byte NUMERAL_Kannada
pub(crate) const NUMERAL_Kannada: i8 = 11;
// Java field: byte NUMERAL_Malayalam
pub(crate) const NUMERAL_Malayalam: i8 = 12;
// Java field: byte NUMERAL_Thai
pub(crate) const NUMERAL_Thai: i8 = 13;
// Java field: byte NUMERAL_Lao
pub(crate) const NUMERAL_Lao: i8 = 14;
// Java field: byte NUMERAL_Tibetan
pub(crate) const NUMERAL_Tibetan: i8 = 15;
// Java field: byte NUMERAL_Myanmar
pub(crate) const NUMERAL_Myanmar: i8 = 16;
// Java field: byte NUMERAL_Ethiopic
pub(crate) const NUMERAL_Ethiopic: i8 = 17;
// Java field: byte NUMERAL_Khmer
pub(crate) const NUMERAL_Khmer: i8 = 18;
// Java field: byte NUMERAL_Mongolian
pub(crate) const NUMERAL_Mongolian: i8 = 19;
// Java field: byte NUMERAL_Japanese1
pub(crate) const NUMERAL_Japanese1: i8 = 27;
// Java field: byte NUMERAL_Japanese2
pub(crate) const NUMERAL_Japanese2: i8 = 28;
// Java field: byte NUMERAL_Japanese3
pub(crate) const NUMERAL_Japanese3: i8 = 29;
// Java field: byte NUMERAL_Simplified_Chinese1
pub(crate) const NUMERAL_Simplified_Chinese1: i8 = 30;
// Java field: byte NUMERAL_Simplified_Chinese2
pub(crate) const NUMERAL_Simplified_Chinese2: i8 = 31;
// Java field: byte NUMERAL_Simplified_Chinese3
pub(crate) const NUMERAL_Simplified_Chinese3: i8 = 32;
// Java field: byte NUMERAL_Traditional_Chinese1
pub(crate) const NUMERAL_Traditional_Chinese1: i8 = 33;
// Java field: byte NUMERAL_Traditional_Chinese2
pub(crate) const NUMERAL_Traditional_Chinese2: i8 = 34;
// Java field: byte NUMERAL_Traditional_Chinese3
pub(crate) const NUMERAL_Traditional_Chinese3: i8 = 35;
// Java field: byte NUMERAL_Korean1
pub(crate) const NUMERAL_Korean1: i8 = 36;
// Java field: byte NUMERAL_Korean2
pub(crate) const NUMERAL_Korean2: i8 = 37;
// Java field: byte NUMERAL_Korean3
pub(crate) const NUMERAL_Korean3: i8 = 38;
// Java field: byte NUMERAL_Korean4
pub(crate) const NUMERAL_Korean4: i8 = 39;
// Java field: byte CALENDAR_DEFAULT
pub(crate) const CALENDAR_DEFAULT: i8 = 0;
// Java field: byte CALENDAR_LOCALE
pub(crate) const CALENDAR_LOCALE: i8 = 1;
// Java field: byte CALENDAR_US
pub(crate) const CALENDAR_US: i8 = 2;
// Java field: byte CALENDAR_JP
pub(crate) const CALENDAR_JP: i8 = 3;
// Java field: byte CALENDAR_TW
pub(crate) const CALENDAR_TW: i8 = 4;
// Java field: byte CALENDAR_KOREAN
pub(crate) const CALENDAR_KOREAN: i8 = 5;
// Java field: byte CALENDAR_HIJRI
pub(crate) const CALENDAR_HIJRI: i8 = 6;
// Java field: byte CALENDAR_THAI
pub(crate) const CALENDAR_THAI: i8 = 7;
// Java field: byte CALENDAR_HEBREW
pub(crate) const CALENDAR_HEBREW: i8 = 8;
// Java field: byte CALENDAR_MEF
pub(crate) const CALENDAR_MEF: i8 = 9;
// Java field: byte CALENDAR_ARAB_QURAN
pub(crate) const CALENDAR_ARAB_QURAN: i8 = 23;
// Java field: byte CALENDAR_ARABIC
pub(crate) const CALENDAR_ARABIC: i8 = 10;
// Java field: byte CALENDAR_TRANS_EN
pub(crate) const CALENDAR_TRANS_EN: i8 = 11;
// Java field: byte CALENDAR_LUNAR
pub(crate) const CALENDAR_LUNAR: i8 = 19;
// Java field: short LCID_DEFAULT
pub(crate) const LCID_DEFAULT: i16 = 0;
// Java field: short LOCALE_PRIMARY_MASK
pub(crate) const LOCALE_PRIMARY_MASK: i16 = 1023;

thread_local! {
    static FORMAT_CODE_AREA: std::cell::RefCell<NCLAccesser> = std::cell::RefCell::new(NCLAccesser::new_default());
}

// Java: public class NCLAccesser
pub struct NCLAccesser {
    // Java field: int mValue
    pub(crate) mValue: i32,
    // Java field: byte mNumeralShape
    pub(crate) mNumeralShape: i8,
    // Java field: byte mCalendarType
    pub(crate) mCalendarType: i8,
    // Java field: short mLocaleCodeID
    pub(crate) mLocaleCodeID: i16,
}

// Java: public class NCLAccesser
impl NCLAccesser {
    // Java: static NCLAccesser shareFormatCodeArea(String value)
    pub fn shareFormatCodeArea(value: &str) -> NCLAccesser {
      // Java: NCLAccesser codeArea = (NCLAccesser)FORMAT_CODE_AREA.get();
      // Java: codeArea.reset(value);
      let mut accesser = NCLAccesser::new_default();
      accesser.reset_str(value);
      // Java: return codeArea;
      accesser
    }

    // Java: static boolean isCaseShape(byte numeralShape)
    pub fn isCaseShape(numeralShape: i8) -> bool {
      // Java: return numeralShape >= 27 && numeralShape <= 39;
      numeralShape >= 27 && numeralShape <= 39
    }

    // Java: static boolean isChinaCaseShape(byte numeralShape)
    pub fn isChinaCaseShape(numeralShape: i8) -> bool {
      // Java: return numeralShape >= 30 && numeralShape <= 35;
      numeralShape >= 30 && numeralShape <= 35
    }

    // Java: static boolean isGeneralShape(byte numeralShape)
    pub fn isGeneralShape(numeralShape: i8) -> bool {
      // Java: return numeralShape == 0 || numeralShape == 1;
      numeralShape == 0 || numeralShape == 1
    }

    // Java: NCLAccesser()
    fn new_default() -> Self {
      // Java: super();
      NCLAccesser {
          mValue: 0,
          mNumeralShape: 0,
          mCalendarType: 0,
          mLocaleCodeID: 0,
      }
    }

    // Java: NCLAccesser(String value)
    pub fn new_from_str(value: &str) -> Self {
      // Java: super();
      let mut s = NCLAccesser::new_default();
      // Java: this.reset(value);
      s.reset_str(value);
      s
    }

    // Java: NCLAccesser(int value)
    pub fn new_from_int(value: i32) -> Self {
      // Java: super();
      let mut s = NCLAccesser::new_default();
      // Java: this.reset(value);
      s.reset_int(value);
      s
    }

    // Java: void reset(String value)
    pub fn reset_str(&mut self, value: &str) {
      // Java: try
      {
         // Java: this.mValue = value != null && value.length() > 0 ? Integer.parseInt(value, 16) : 0;
         self.mValue = if !value.is_empty() {
             i32::from_str_radix(value, 16).unwrap_or(1033)
         } else {
             0
         };
      }
      // Java: catch (NumberFormatException var3)
      // handled by unwrap_or above
      {
         // Java: this.mValue = 1033;
      }
      // Java: this.reset(this.mValue);
      self.reset_int(self.mValue);
    }

    // Java: void reset(int value)
    pub fn reset_int(&mut self, value: i32) {
      // Java: this.mNumeralShape = (byte)(value >>> 24);
      self.mNumeralShape = ((value as u32) >> 24) as i8;
      // Java: this.mCalendarType = (byte)(value >> 16 & 255);
      self.mCalendarType = ((value >> 16) & 255) as i8;
      // Java: this.mLocaleCodeID = (short)(value & '\uffff');
      self.mLocaleCodeID = (value & 0xFFFF) as i16;
    }

    // Java: byte getNumberalShape()
    pub fn getNumberalShape(&self) -> i8 {
      // Java: return this.mNumeralShape;
      self.mNumeralShape
    }

    // Java: byte getCalendarType()
    pub fn getCalendarType(&self) -> i8 {
      // Java: return this.mCalendarType;
      self.mCalendarType
    }

    // Java: short getLCID()
    pub fn getLCID(&self) -> i16 {
      // Java: return this.mLocaleCodeID;
      self.mLocaleCodeID
    }

    // Java: String getHexLCID()
    pub fn getHexLCID(&self) -> String {
      // Java: return Integer.toHexString(this.mLocaleCodeID);
      format!("{:x}", self.mLocaleCodeID as u16)
    }

    // Java: short getLocalPrimaryValue()
    pub fn getLocalPrimaryValue(&self) -> i16 {
      // Java: return (short)(this.mLocaleCodeID & 1023);
      self.mLocaleCodeID & 1023
    }

    // Java: static String genNCLCode(byte numeralShape, byte calendar, short lcid)
    pub fn genNCLCode(numeralShape: i8, calendar: i8, lcid: i16) -> String {
      // Java: return Integer.toHexString(numeralShape << 24 | calendar << 16 | lcid);
      let val = ((numeralShape as i32) << 24) | ((calendar as i32) << 16) | (lcid as u16 as i32);
      format!("{:x}", val)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &NCLAccesser) -> bool {
      // Java: if (o instanceof NCLAccesser)
      {
         // Java: NCLAccesser area = (NCLAccesser)o;
         // Java: return this.mValue == area.mValue;
         return self.mValue == o.mValue;
      }
      // Java: else
      // In Rust, type check is guaranteed at compile time
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int prime = 31;
      let _prime = 31;
      // Java: int result = 1;
      let mut result = 1;
      // Java: result = 31 * result + this.mValue;
      result = 31 * result + self.mValue;
      // Java: return result;
      result
    }

    // Java: static short getCalenderDefaultLCID(byte calenderType, short localeID)
    pub fn getCalenderDefaultLCID(calenderType: i8, localeID: i16) -> i16 {
      // Java: switch (calenderType)
      match calenderType {
         // Java: case 2:
         2 => {
            // Java: return 1033;
            1033
         }
         // Java: case 3:
         // Java: case 5:
         3 | 5 => {
            // Java: return GlobalEnv.getDefaultLCID();
            super::GlobalEnv::GlobalEnv::getDefaultLCID()
         }
         // Java: case 4:
         4 => {
            // Java: return 1028;
            1028
         }
         // Java: case 6:
         // Java: case 8:
         // Java: case 10:
         // Java: default:
         6 | 8 | 10 => {
            // Java: return localeID;
            localeID
         }
         // Java: case 7:
         7 => {
            // Java: return 1054;
            1054
         }
         // Java: case 9:
         9 => {
            // Java: return 1036;
            1036
         }
         // Java: case 11:
         11 => {
            // Java: return 1025;
            1025
         }
         _ => {
            // Java: return localeID;
            localeID
         }
      }
    }

    // Java: static String getLoadCalendarFile(byte arabicCalendar, short loadID)
    pub fn getLoadCalendarFile(arabicCalendar: i8, _loadID: i16) -> String {
      // Java: switch (arabicCalendar)
      match arabicCalendar {
         // Java: case 6:
         // Java: case 23:
         6 | 23 => {
            // Java: short supported = GlobalEnv.matchLCID(loadID);
            // Java: if (supported == 1025)
            // Java: return "calendar_arabia_hijriLocale";
            // Java: return "calendar_arabia_hijri";
            "calendar_arabia_hijri".to_string()
         }
         // Java: case 10:
         10 => {
            // Java: return "calendar_arabia_0A";
            "calendar_arabia_0A".to_string()
         }
         // Java: default:
         _ => {
            // Java: return "";
            String::new()
         }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder builder = new StringBuilder();
      let mut builder = String::new();
      // Java: builder.append("NumeralShape:0x");
      builder.push_str("NumeralShape:0x");
      // Java: builder.append(Integer.toHexString(this.mNumeralShape));
      builder.push_str(&format!("{:x}", self.mNumeralShape));
      // Java: builder.append("; CalendarType:0x");
      builder.push_str("; CalendarType:0x");
      // Java: builder.append(Integer.toHexString(this.mCalendarType));
      builder.push_str(&format!("{:x}", self.mCalendarType));
      // Java: builder.append("; Locale-Language:0x");
      builder.push_str("; Locale-Language:0x");
      // Java: builder.append(Integer.toHexString(this.mLocaleCodeID));
      builder.push_str(&format!("{:x}", self.mLocaleCodeID as u16));
      // Java: return builder.toString();
      builder
    }

}
