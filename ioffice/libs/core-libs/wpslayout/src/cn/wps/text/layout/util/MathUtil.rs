// Java: public class MathUtil
pub struct MathUtil;

// Java: public class MathUtil
impl MathUtil {
    // Java: MathUtil()
    pub fn new() -> Self {
      // Java: super();
      MathUtil
    }

    // Java: static short abs(short value)
    pub fn abs(value: i16) -> i16 {
      // Java: boolean isPositive = isGreatEqualThanZero(value);
      let is_positive = Self::isGreatEqualThanZero(value);
      // Java: return isPositive ? value : (short)(65536 - (value & '\uffff'));
      if is_positive { value } else { (65536i32 - (value as u16 as i32)) as i16 }
    }

    // Java: static boolean isGreatEqualThanZero(short value)
    pub fn isGreatEqualThanZero(value: i16) -> bool {
      // Java: return (value & '耀') == 0;
      (value as u16 & 0x8000) == 0
    }

    // Java: static boolean isGreatThanZero(short value)
    pub fn isGreatThanZero(value: i16) -> bool {
      // Java: return (value & '耀') == 0 && (value & 32767) != 0;
      (value as u16 & 0x8000) == 0 && (value & 32767) != 0
    }

}
