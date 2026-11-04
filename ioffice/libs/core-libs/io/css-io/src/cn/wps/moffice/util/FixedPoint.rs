// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.property.PropBaseSerializable
use util::cn::wps::moffice::property::PropBaseSerializable::PropBaseSerializableTrait as PropBaseSerializable;
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
use std::io::{Read, Write, Result};

// Java field: char DOT
pub(crate) const DOT: char = '.';
// Java field: char ZERO
pub(crate) const ZERO: char = '0';
// Java field: int MAX_PRECISION
pub(crate) const MAX_PRECISION: i32 = 5;
// Java field: int MAX_INTEGRAL_LEN
pub(crate) const MAX_INTEGRAL_LEN: i32 = 5;
// Java field: int MAX_VALUE_UNSIGNED_SHORT
pub(crate) const MAX_VALUE_UNSIGNED_SHORT: i32 = 65535;

// Java: public class FixedPoint implements PropBaseSerializable
pub struct FixedPoint {
    // Java field: boolean mNegative
    pub(crate) mNegative: bool,
    // Java field: short mIntegral
    pub(crate) mIntegral: i16,
    // Java field: int mFractional
    pub(crate) mFractional: i32,
    // Java field: int mPrecision
    pub(crate) mPrecision: i32,
}

// Java: public class FixedPoint implements PropBaseSerializable
impl FixedPoint {
    // Java: FixedPoint()
    pub fn new() -> Self {
      // Java: super();
      FixedPoint {
          mNegative: false,
          mIntegral: 0,
          mFractional: 0,
          mPrecision: 0,
      }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&FixedPoint>) -> bool {
      // Java: if (null == o)
      if o.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: Assert.assertTrue("o should be an instance of FixedPoint.", o instanceof FixedPoint);
         let fp = o.unwrap();
         // Java: FixedPoint fp = (FixedPoint)o;
         // Java: return fp.mNegative == this.mNegative && fp.mIntegral == this.mIntegral && fp.mFractional == this.mFractional && fp.mPrecision == this.mPrecision;
         return fp.mNegative == self.mNegative && fp.mIntegral == self.mIntegral && fp.mFractional == self.mFractional && fp.mPrecision == self.mPrecision;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return (this.mNegative ? 1 : 0) + this.mIntegral + this.mFractional + this.mPrecision;
      (if self.mNegative { 1i32 } else { 0 }) + self.mIntegral as i32 + self.mFractional + self.mPrecision
    }

    // Java: FixedPoint(int val)
    pub fn new_from_int(val: i32) -> Self {
      // Java: this((short)((val & -65536) >> 16), val & '\uffff');
      Self::new_from_parts(((val & -65536i32) >> 16) as i16, val & 0xFFFF)
    }

    // Java: FixedPoint(short integral, int fractional)
    pub fn new_from_parts(integral: i16, fractional: i32) -> Self {
      // Java: super();
      // Java: this.mIntegral = integral;
      // Java: this.mNegative = integral < 0;
      // Java: if (0 != (fractional & -65536))
      if 0 != (fractional & -65536i32)
      {
         // Java: throw new NumberFormatException("The fractional part in the val String is greater than the max value of unsigned short.");
         panic!("The fractional part in the val String is greater than the max value of unsigned short.");
      }
      // Java: else
      else
      {
         // Java: this.mFractional = fractional & '\uffff';
         // Java: this.mPrecision = 5;
         FixedPoint {
             mIntegral: integral,
             mNegative: integral < 0,
             mFractional: fractional & 0xFFFF,
             mPrecision: 5,
         }
      }
    }

    // Java: FixedPoint(String val, int precision)
    pub fn new_from_str_precision(val: &str, precision: i32) -> Self {
      // Java: super();
      let mut fp = Self::new();
      // Java: this.initFromStr(val, precision);
      fp.initFromStr(val, precision);
      fp
    }

    // Java: FixedPoint(String val)
    pub fn new_from_str(val: &str) -> Self {
      // Java: super();
      // Java: StringBuilder builder = new StringBuilder();
      let mut builder = String::new();
      // Java: int len = val.length();
      let len = val.len();
      // Java: int iDot = len;
      let mut iDot = len;
      // Java: for(int i = 0; i < len; ++i)
      for ch in val.chars()
      {
         // Java: char ch = val.charAt(i);
         // Java: if ('.' == ch)
         if '.' == ch
         {
            // Java: iDot = builder.length();
            iDot = builder.len();
         }
         // Java: else
         else
         {
            // Java: builder.append(ch);
            builder.push(ch);
         }
      }
      // Java: this.initFromStr(builder.toString(), builder.length() - iDot);
      let precision = builder.len() as i32 - iDot as i32;
      let mut fp = Self::new();
      fp.initFromStr(&builder, precision);
      fp
    }

    // Java: void initFromStr(String val, int precision)
    pub fn initFromStr(&mut self, val: &str, precision: i32) {
      // Java: if (precision < 0)
      if precision < 0
      {
         // Java: throw new NumberFormatException("The min and max precision are 0 and 5");
         panic!("The min and max precision are 0 and 5");
      }
      // Java: else
      else
      {
         // Java: if (precision > 5)
         let mut precision = precision;
         if precision > 5
         {
            // Java: precision = 5;
            precision = 5;
         }
         // Java: int integralLen = val.length() - precision;
         let integralLen = val.len() as i32 - precision;
         // Java: if (integralLen > 0)
         if integralLen > 0
         {
            // Java: char firstDigit = val.charAt(0);
            let firstDigit = val.as_bytes()[0] as char;
            // Java: if (firstDigit == '-')
            if firstDigit == '-'
            {
               // Java: this.mNegative = true;
               self.mNegative = true;
            }
            // Java: int integralLenWithoutNegativeSign = integralLen - (this.mNegative ? 1 : 0);
            let mut integralLenWithoutNegativeSign = integralLen - (if self.mNegative { 1 } else { 0 });
            // Java: int tmp = integralLenWithoutNegativeSign;
            let tmp = integralLenWithoutNegativeSign;
            // Java: for(int i = 0; i < tmp && val.charAt((this.mNegative ? 1 : 0) + i) == '0'; ++i)
            let offset = if self.mNegative { 1 } else { 0 };
            for i in 0..tmp
            {
               if val.as_bytes()[(offset + i) as usize] as char != '0' {
                   break;
               }
               // Java: --integralLenWithoutNegativeSign;
               integralLenWithoutNegativeSign -= 1;
            }
            // Java: if (integralLenWithoutNegativeSign > 5)
            if integralLenWithoutNegativeSign > 5
            {
               // Java: throw new NumberFormatException("The max length of integral part is 5");
               panic!("The max length of integral part is 5");
            }
         }
         // Java: String strIntegral = null;
         let str_integral: &str;
         // Java: String strFractional = null;
         let str_fractional: &str;
         // Java: if (integralLen > 0)
         if integralLen > 0
         {
            // Java: if (integralLen == 1 && this.mNegative)
            if integralLen == 1 && self.mNegative
            {
               // Java: strIntegral = "0";
               str_integral = "0";
            }
            // Java: else
            else
            {
               // Java: strIntegral = val.substring(0, integralLen);
               str_integral = &val[..integralLen as usize];
            }
            // Java: strFractional = val.substring(integralLen);
            str_fractional = &val[integralLen as usize..];
         }
         // Java: else
         else
         {
            // Java: strIntegral = "0";
            str_integral = "0";
            // Java: strFractional = val;
            str_fractional = val;
         }
         // Java: this.mIntegral = Short.valueOf(strIntegral);
         self.mIntegral = str_integral.parse::<i16>().unwrap();
         // Java: this.mPrecision = precision;
         self.mPrecision = precision;
         // Java: this.mFractional = strFractional.length() == 0 ? 0 : Integer.valueOf(strFractional);
         self.mFractional = if str_fractional.is_empty() { 0 } else { str_fractional.parse::<i32>().unwrap() };
         // Java: int leadZero = Integer.numberOfLeadingZeros(this.mFractional) - 16;
         let leadZero = (self.mFractional as u32).leading_zeros() as i32 - 16;
         // Java: if (leadZero > 0)
         if leadZero > 0
         {
            // Java: this.mFractional <<= leadZero;
            self.mFractional <<= leadZero;
         }
         // Java: if (0 != (this.mFractional & -65536))
         if 0 != (self.mFractional & -65536i32)
         {
            // Java: throw new NumberFormatException("The fractional part in the val String is greater than the max value of unsigned short.");
            panic!("The fractional part in the val String is greater than the max value of unsigned short.");
         }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder builder = new StringBuilder();
      let mut builder = String::new();
      // Java: if (this.mIntegral == 0 && this.mNegative)
      if self.mIntegral == 0 && self.mNegative
      {
         // Java: builder.append("-0");
         builder.push_str("-0");
      }
      // Java: else
      else
      {
         // Java: builder.append(this.mIntegral);
         builder.push_str(&self.mIntegral.to_string());
      }
      // Java: if (this.mPrecision == 0)
      if self.mPrecision == 0
      {
         // Java: return builder.toString();
         return builder;
      }
      // Java: else
      else
      {
         // Java: builder.append('.');
         builder.push('.');
         // Java: String strFrac = Integer.valueOf(this.mFractional).toString();
         let str_frac = self.mFractional.to_string();
         // Java: int leadZeros = 5 - strFrac.length();
         let lead_zeros = 5i32 - str_frac.len() as i32;
         // Java: Assert.assertTrue("d >= 0 should be true", leadZeros >= 0);
         Assert::assertTrue_msg("d >= 0 should be true", lead_zeros >= 0);
         // Java: if (leadZeros >= 0)
         if lead_zeros >= 0
         {
            // Java: for(int i = 0; i < leadZeros; ++i)
            for _i in 0..lead_zeros
            {
               // Java: builder.append('0');
               builder.push('0');
            }
            // Java: for(int i = 0; i < this.mPrecision - leadZeros; ++i)
            for i in 0..(self.mPrecision - lead_zeros)
            {
               // Java: builder.append(strFrac.charAt(i));
               builder.push(str_frac.as_bytes()[i as usize] as char);
            }
         }
         // Java: return builder.toString();
         return builder;
      }
    }

    // Java: float floatValue()
    pub fn floatValue(&self) -> f32 {
      // Java: Assert.assertTrue("We should only use the lower 16bits of mFractional.", 0 == (this.mFractional & -65536));
      Assert::assertTrue_msg("We should only use the lower 16bits of mFractional.", 0 == (self.mFractional & -65536i32));
      // Java: float ret = (float)Math.abs(this.mIntegral) + (float)this.mFractional / 65536.0F;
      let mut ret = (self.mIntegral.unsigned_abs() as f32) + (self.mFractional as f32) / 65536.0f32;
      // Java: if (this.mNegative)
      if self.mNegative
      {
         // Java: ret = -ret;
         ret = -ret;
      }
      // Java: return ret;
      ret
    }

    // Java: int getMSOFixedPoint()
    pub fn getMSOFixedPoint(&self) -> i32 {
      // Java: Assert.assertTrue("We should only use the lower 16bits of mFractional.", 0 == (this.mFractional & -65536));
      Assert::assertTrue_msg("We should only use the lower 16bits of mFractional.", 0 == (self.mFractional & -65536i32));
      // Java: return this.mIntegral << 16 | this.mFractional & '\uffff';
      ((self.mIntegral as i32) << 16) | (self.mFractional & 0xFFFF)
    }

    // Java: short getIntegral()
    pub fn getIntegral(&self) -> i16 {
      // Java: return this.mIntegral;
      self.mIntegral
    }

    // Java: int getFractional()
    pub fn getFractional(&self) -> i32 {
      // Java: Assert.assertTrue("We should only use the lower 16bits of mFractional.", 0 == (this.mFractional & -65536));
      Assert::assertTrue_msg("We should only use the lower 16bits of mFractional.", 0 == (self.mFractional & -65536i32));
      // Java: return this.mFractional;
      self.mFractional
    }

    // Java: boolean isNegative()
    pub fn isNegative(&self) -> bool {
      // Java: return this.mNegative;
      self.mNegative
    }

    // Java: int getPrecision()
    pub fn getPrecision(&self) -> i32 {
      // Java: return this.mPrecision;
      self.mPrecision
    }

    // Java: void readExternal(ObjectInput in)
    pub fn readExternal(&mut self, input: &mut dyn Read) -> Result<()> {
      let mut buf1 = [0u8; 1];
      let mut buf2 = [0u8; 2];
      let mut buf4 = [0u8; 4];
      // Java: this.mNegative = in.readBoolean();
      input.read_exact(&mut buf1)?;
      self.mNegative = buf1[0] != 0;
      // Java: this.mIntegral = in.readShort();
      input.read_exact(&mut buf2)?;
      self.mIntegral = i16::from_be_bytes(buf2);
      // Java: this.mFractional = in.readInt();
      input.read_exact(&mut buf4)?;
      self.mFractional = i32::from_be_bytes(buf4);
      // Java: this.mPrecision = in.readInt();
      input.read_exact(&mut buf4)?;
      self.mPrecision = i32::from_be_bytes(buf4);
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput out)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
      // Java: out.writeBoolean(this.mNegative);
      output.write_all(&[if self.mNegative { 1u8 } else { 0u8 }])?;
      // Java: out.writeShort(this.mIntegral);
      output.write_all(&self.mIntegral.to_be_bytes())?;
      // Java: out.writeInt(this.mFractional);
      output.write_all(&self.mFractional.to_be_bytes())?;
      // Java: out.writeInt(this.mPrecision);
      output.write_all(&self.mPrecision.to_be_bytes())?;
      Ok(())
    }

}

impl PropBaseSerializable for FixedPoint {
}
