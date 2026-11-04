// java public enum WdCompatibilityMode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WdCompatibilityMode {
    // java wdCurrent(65535)
    wdCurrent,
    // java wdWord2003(11)
    wdWord2003,
    // java wdWord2007(12)
    wdWord2007,
    // java wdWord2010(14)
    wdWord2010,
    // java wdWord2013(15)
    wdWord2013,
}

// java public enum WdCompatibilityMode
impl WdCompatibilityMode {
    // java public static WdCompatibilityMode Default = wdWord2010;
    pub const Default: Self = Self::wdWord2010;

    // java public String StringValue()
    pub fn StringValue(self) -> String {
      // java return String.valueOf(this.mValue);
      self.mValue().to_string()
    }

    pub fn mValue(self) -> i32 {
      match self {
        Self::wdCurrent => 65535,
        Self::wdWord2003 => 11,
        Self::wdWord2007 => 12,
        Self::wdWord2010 => 14,
        Self::wdWord2013 => 15,
      }
    }

    // java public boolean isBefore2013Mode()
    pub fn isBefore2013Mode(self) -> bool {
      // java return this.mValue < 15;
      self.mValue() < 15
    }

    // java public boolean isTableIndentedByOldStyle()
    pub fn isTableIndentedByOldStyle(self) -> bool {
      // java return this.isBefore2013Mode();
      self.isBefore2013Mode()
    }

    // java public static WdCompatibilityMode convertValue(String strVal)
    pub fn convertValue(strVal: &str) -> Self {
      // java int v = Integer.parseInt(strVal);
      let Ok(v) = strVal.trim().parse::<i32>() else {
        // java catch (NumberFormatException) mode = Default;
        return Self::Default;
      };
      // java switch (v)
      match v {
        // java case 11: mode = wdWord2003;
        11 => Self::wdWord2003,
        // java case 12: mode = wdWord2007;
        12 => Self::wdWord2007,
        // java case 15: mode = wdWord2013;
        15 => Self::wdWord2013,
        // java case 65535: mode = wdCurrent;
        65535 => Self::wdCurrent,
        // java default: mode = wdWord2010;
        _ => Self::wdWord2010,
      }
    }
}
