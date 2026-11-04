#![allow(
    non_snake_case,
    non_camel_case_types,
    unused_variables,
    unused_imports,
    dead_code,
    unreachable_code,
    unused_mut
)]

// Java: import cn.wps.kfc.numfmt.global.GlobalEnv
use crate::cn::wps::kfc::numfmt::global::GlobalEnv;
// Java: import cn.wps.kfc.numfmt.global.NCLAccesser
use crate::cn::wps::kfc::numfmt::global::NCLAccesser;
// Java: import cn.wps.kfc.numfmt.resource.ResourceDef
use crate::cn::wps::kfc::numfmt::resource::ResourceDef;
// Java: import cn.wps.kfc.numfmt.resource.ResourceLoader
use crate::cn::wps::kfc::numfmt::resource::ResourceLoader;
// Java: import java.util.List
// Java: import java.util.Properties

use std::sync::{Mutex, OnceLock};

// Java field: short NO_CASE
pub(crate) const NO_CASE: i16 = 0;
// Java field: short PLAIN_CASE
pub(crate) const PLAIN_CASE: i16 = 1;
// Java field: short COMPLEX_CASE
pub(crate) const COMPLEX_CASE: i16 = 4096;
// Java field: short COMPLEX_CASE1
pub(crate) const COMPLEX_CASE1: i16 = 4097;
// Java field: short COMPLEX_CASE2
pub(crate) const COMPLEX_CASE2: i16 = 4098;
// Java field: short COMPLEX_CASE3
pub(crate) const COMPLEX_CASE3: i16 = 4100;
// Java field: short COMPLEX_CASE_RMB
pub(crate) const COMPLEX_CASE_RMB: i16 = 4104;
// Java field: int RES_DIGIT
pub(crate) const RES_DIGIT: i32 = 1;
// Java field: int RES_UNITS
pub(crate) const RES_UNITS: i32 = 2;
// Java field: int RES_ALL
pub(crate) const RES_ALL: i32 = 3;

// Java: public class NumericCase
pub struct NumericCase {
    // Java field: String[] mDigits
    mDigits: Vec<String>,
    // Java field: String[] mMinUnits
    mMinUnits: Vec<String>,
    // Java field: String[] mMainUnits
    mMainUnits: Vec<String>,
    // Java field: String[] mMinusDot
    mMinusDot: Vec<String>,
    // Java field: String[] mMoneyUnits
    mMoneyUnits: Vec<String>,
    // Java field: byte mLatestShape
    mLatestShape: i8,
    // Java field: int mResState
    mResState: i32,
    // Java field: StringBuffer mBuffer
    mBuffer: String,
}

// Java field: NumericCase instance
static INSTANCE: OnceLock<Mutex<NumericCase>> = OnceLock::new();

// Java: public class NumericCase
impl NumericCase {
    // Java: NumericCase()

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:31
    // java public NumericCase() {
    // java super();
    // java }
    pub fn new() -> Self {
        // Java: super();
        NumericCase {
            mDigits: vec![String::new(); 10],
            mMinUnits: vec![String::new(); 3],
            mMainUnits: vec![String::new(); 3],
            mMinusDot: vec![String::new(); 2],
            mMoneyUnits: vec![String::new(); 4],
            mLatestShape: 0,
            mResState: 0,
            mBuffer: String::new(),
        }
    }

    // Java: static NumericCase getInstance()

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:35
    // java public static synchronized NumericCase getInstance() {
    // java if (instance == null) {
    // java instance = new NumericCase();
    // java }
    // java
    // java return instance;
    // java }
    pub fn getInstance() -> &'static Mutex<NumericCase> {
        // Java: if (instance == null)
        {
            // Java: instance = new NumericCase();
        }
        // Java: return instance;
        INSTANCE.get_or_init(|| Mutex::new(NumericCase::new()))
    }

    // Java: static short getCaseType(short dbNum, byte numeralShape, boolean isRMBScale)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:43
    // java public static short getCaseType(short dbNum, byte numeralShape, boolean isRMBScale) {
    // java short type = 0;
    // java if (NCLAccesser.isGeneralShape(numeralShape)) {
    // java type = 0;
    // java } else if (NCLAccesser.isCaseShape(numeralShape)) {
    // java type = 4096;
    // java } else {
    // java type = 1;
    // java }
    // java
    // java if (type == 4096) {
    // java if (isRMBScale && numeralShape == 31) {
    // java type = (short)(type | 4104);
    // java } else if (numeralShape != 27 && numeralShape != 29) {
    // java if (dbNum == 20) {
    // java type = (short)(type | 4097);
    // java } else {
    // java type = (short)(type | 4097);
    // java }
    // java } else {
    // java type = (short)(type | 4098);
    // java }
    // java
    // java if (!isRMBType(type) && !NCLAccesser.isChinaCaseShape(numeralShape)) {
    // java type = (short)(type | 4100);
    // java }
    // java }
    // java
    // java return type;
    // java }
    pub fn getCaseType(db_num: i16, numeral_shape: i8, is_rmb_scale: bool) -> i16 {
        let mut ty =
            if crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser::isGeneralShape(
                numeral_shape,
            ) {
                0
            } else if crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser::isCaseShape(
                numeral_shape,
            ) {
                4096
            } else {
                1
            };
        if ty == 4096 {
            if is_rmb_scale && numeral_shape == 31 {
                ty |= 4104;
            } else if numeral_shape != 27 && numeral_shape != 29 {
                ty |= 4097;
            } else {
                ty |= 4098;
            }
            if !Self::isRMBType(ty)
                && !crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser::isChinaCaseShape(
                    numeral_shape,
                )
            {
                ty |= 4100;
            }
        }
        ty
    }

    // Java: static short getTimeCaseType(byte numeralShape, boolean isElapsed)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:74
    // java public static short getTimeCaseType(byte numeralShape, boolean isElapsed) {
    // java if (NCLAccesser.isGeneralShape(numeralShape)) {
    // java return 0;
    // java } else if (!NCLAccesser.isCaseShape(numeralShape)) {
    // java return 1;
    // java } else {
    // java short type = 0;
    // java if (!isElapsed) {
    // java switch (numeralShape) {
    // java case 27:
    // java case 30:
    // java case 33:
    // java case 36:
    // java type = (short)(type | 4098);
    // java break;
    // java case 28:
    // java case 31:
    // java case 34:
    // java case 37:
    // java default:
    // java type = (short)(type | 4097);
    // java break;
    // java case 29:
    // java case 32:
    // java case 35:
    // java case 38:
    // java type = 1;
    // java }
    // java } else {
    // java type = 4097;
    // java }
    // java
    // java return type;
    // java }
    // java }
    pub fn getTimeCaseType(numeral_shape: i8, is_elapsed: bool) -> i16 {
        use crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser;
        if NCLAccesser::isGeneralShape(numeral_shape) {
            0
        } else if !NCLAccesser::isCaseShape(numeral_shape) {
            1
        } else if is_elapsed {
            4097
        } else {
            match numeral_shape {
                27 | 30 | 33 | 36 => 4098,
                29 | 32 | 35 | 38 => 1,
                _ => 4097,
            }
        }
    }

    // Java: static boolean isRMBType(short type)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:110
    // java public static final boolean isRMBType(short type) {
    // java return (type & 4104) == 4104;
    // java }
    pub fn isRMBType(ty: i16) -> bool {
        (ty & 4104) == 4104
    }

    // Java: String deform(byte numeralShape, short type, String number)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:114
    // java public String deform(byte numeralShape, short type, String number) {
    // java if (number != null && number.length() != 0 && type != 0) {
    // java this.mBuffer.setLength(0);
    // java int resState = type == 1 ? 1 : 3;
    // java this.loadRes(numeralShape, resState);
    // java if ((this.mResState & 1) != 1) {
    // java return number;
    // java } else {
    // java return type != 1 && (this.mResState & 2) == 2 ? this.performCase(number, type) : this.replaceDigits(number);
    // java }
    // java } else {
    // java return number;
    // java }
    // java }
    pub fn deform(&mut self, numeral_shape: i8, ty: i16, number: &str) -> String {
        if number.is_empty() || ty == 0 {
            return number.into();
        }
        self.mBuffer.clear();
        let res = if ty == 1 { 1 } else { 3 };
        self.loadRes(numeral_shape, res);
        if self.mResState & 1 != 1 {
            number.into()
        } else if ty != 1 && self.mResState & 2 == 2 {
            self.performCase(number, ty)
        } else {
            self.replaceDigits(number)
        }
    }

    // Java: String replaceDigits(String str)
    // java private String replaceDigits(String str) {
    // java for(int i = 0; i < str.length(); ++i) {
    // java char c = str.charAt(i);
    // java if (c >= '0' && c <= '9') {
    // java this.mBuffer.append(this.mDigits[c - 48]);
    // java }
    // java }
    // java
    // java return this.mBuffer.toString();
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:129
    // java private String replaceDigits(String str) {
    // java for(int i = 0; i < str.length(); ++i) {
    // java char c = str.charAt(i);
    // java if (c >= '0' && c <= '9') {
    // java this.mBuffer.append(this.mDigits[c - 48]);
    // java }
    // java }
    // java
    // java return this.mBuffer.toString();
    // java }
    pub fn replaceDigits(&mut self, text: &str) -> String {
        for c in text.encode_utf16() {
            if (48..=57).contains(&c) {
                self.mBuffer.push_str(&self.mDigits[(c - 48) as usize]);
            }
        }
        self.mBuffer.clone()
    }

    // Java: String transDigits(String digits, int start, int end, byte numeralShape)
    // java String transDigits(String digits, int start, int end, byte numeralShape) {
    // java this.mBuffer.setLength(0);
    // java this.mBuffer.append(digits, start > -1 ? start : 0, end);
    // java this.transDigits((StringBuffer)this.mBuffer, 0, this.mBuffer.length(), numeralShape);
    // java return this.mBuffer.toString();
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:140
    // java String transDigits(String digits, int start, int end, byte numeralShape) {
    // java this.mBuffer.setLength(0);
    // java this.mBuffer.append(digits, start > -1 ? start : 0, end);
    // java this.transDigits((StringBuffer)this.mBuffer, 0, this.mBuffer.length(), numeralShape);
    // java return this.mBuffer.toString();
    // java }
    pub fn transDigits_str(
        &mut self,
        digits: &str,
        start: i32,
        end: i32,
        numeral_shape: i8,
    ) -> String {
        let units: Vec<u16> = digits.encode_utf16().collect();
        self.mBuffer = String::from_utf16(&units[start.max(0) as usize..end as usize])
            .expect("unpaired Java UTF16");
        let mut buffer = std::mem::take(&mut self.mBuffer);
        let len = buffer.encode_utf16().count() as i32;
        self.transDigits(&mut buffer, 0, len, numeral_shape);
        self.mBuffer = buffer;
        self.mBuffer.clone()
    }

    // Java: void transDigits(StringBuffer digits, int start, int end, byte numeralShape)
    // java void transDigits(StringBuffer digits, int start, int end, byte numeralShape) {
    // java int resState = 1;
    // java this.loadRes(numeralShape, resState);
    // java if ((this.mResState & 1) == 1) {
    // java for(int i = start; i < end; ++i) {
    // java char c = digits.charAt(i);
    // java if (c >= '0' && c <= '9') {
    // java digits.setCharAt(i, this.mDigits[c - 48].charAt(0));
    // java }
    // java }
    // java
    // java }
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:147
    // java void transDigits(StringBuffer digits, int start, int end, byte numeralShape) {
    // java int resState = 1;
    // java this.loadRes(numeralShape, resState);
    // java if ((this.mResState & 1) == 1) {
    // java for(int i = start; i < end; ++i) {
    // java char c = digits.charAt(i);
    // java if (c >= '0' && c <= '9') {
    // java digits.setCharAt(i, this.mDigits[c - 48].charAt(0));
    // java }
    // java }
    // java
    // java }
    // java }
    pub fn transDigits(&mut self, digits: &mut String, start: i32, end: i32, numeral_shape: i8) {
        self.loadRes(numeral_shape, 1);
        if self.mResState & 1 == 1 {
            let mut units: Vec<u16> = digits.encode_utf16().collect();
            for i in start..end {
                let c = units[i as usize];
                if (48..=57).contains(&c) {
                    units[i as usize] = self.mDigits[(c - 48) as usize]
                        .encode_utf16()
                        .next()
                        .unwrap();
                }
            }
            *digits = String::from_utf16(&units).expect("unpaired Java UTF16");
        }
    }

    // Java: String performCase(String formatStr, short type)
    // java private String performCase(String formatStr, short type) {
    // java boolean isRMBType = isRMBType(type);
    // java int minus = formatStr.indexOf("-");
    // java if (minus > -1) {
    // java this.mBuffer.append(this.mMinusDot[0]);
    // java }
    // java
    // java String decimal = "";
    // java int decimalIdx = formatStr.indexOf(".");
    // java if (decimalIdx > -1) {
    // java decimal = formatStr.substring(decimalIdx + 1);
    // java } else {
    // java decimalIdx = formatStr.length();
    // java }
    // java
    // java String integer = formatStr.substring(minus > -1 ? 1 : 0, decimalIdx);
    // java if (isRMBType && integer.length() > 16) {
    // java int step = 12;
    // java int len = integer.length() - 12;
    // java int group = len / 12;
    // java int index = len % 12;
    // java if (index != 0) {
    // java this.processInteger(this.mBuffer, integer.substring(0, index), false, type);
    // java }
    // java
    // java while(group > 0) {
    // java this.processInteger(this.mBuffer, integer.substring(index, index + 16), false, type);
    // java index += 12;
    // java --group;
    // java }
    // java
    // java this.mBuffer.append(this.mMainUnits[2]);
    // java integer = integer.substring(index);
    // java if (integer.matches("0+")) {
    // java integer = "";
    // java }
    // java }
    // java
    // java this.processInteger(this.mBuffer, integer, decimal.length() > 0, type);
    // java if (isRMBType && this.mBuffer.length() > 0) {
    // java this.mBuffer.append(this.mMoneyUnits[0]);
    // java }
    // java
    // java if (decimal.length() > 0) {
    // java this.processDecimal(this.mBuffer, decimal, type);
    // java } else if (isRMBType) {
    // java this.mBuffer.append(this.mMoneyUnits[3]);
    // java }
    // java
    // java return this.mBuffer.toString();
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:161
    // java private String performCase(String formatStr, short type) {
    // java boolean isRMBType = isRMBType(type);
    // java int minus = formatStr.indexOf("-");
    // java if (minus > -1) {
    // java this.mBuffer.append(this.mMinusDot[0]);
    // java }
    // java
    // java String decimal = "";
    // java int decimalIdx = formatStr.indexOf(".");
    // java if (decimalIdx > -1) {
    // java decimal = formatStr.substring(decimalIdx + 1);
    // java } else {
    // java decimalIdx = formatStr.length();
    // java }
    // java
    // java String integer = formatStr.substring(minus > -1 ? 1 : 0, decimalIdx);
    // java if (isRMBType && integer.length() > 16) {
    // java int step = 12;
    // java int len = integer.length() - 12;
    // java int group = len / 12;
    // java int index = len % 12;
    // java if (index != 0) {
    // java this.processInteger(this.mBuffer, integer.substring(0, index), false, type);
    // java }
    // java
    // java while(group > 0) {
    // java this.processInteger(this.mBuffer, integer.substring(index, index + 16), false, type);
    // java index += 12;
    // java --group;
    // java }
    // java
    // java this.mBuffer.append(this.mMainUnits[2]);
    // java integer = integer.substring(index);
    // java if (integer.matches("0+")) {
    // java integer = "";
    // java }
    // java }
    // java
    // java this.processInteger(this.mBuffer, integer, decimal.length() > 0, type);
    // java if (isRMBType && this.mBuffer.length() > 0) {
    // java this.mBuffer.append(this.mMoneyUnits[0]);
    // java }
    // java
    // java if (decimal.length() > 0) {
    // java this.processDecimal(this.mBuffer, decimal, type);
    // java } else if (isRMBType) {
    // java this.mBuffer.append(this.mMoneyUnits[3]);
    // java }
    // java
    // java return this.mBuffer.toString();
    // java }
    pub fn performCase(&mut self, format_str: &str, ty: i16) -> String {
        let is_rmb = Self::isRMBType(ty);
        let minus = format_str.find('-');
        let mut buffer = std::mem::take(&mut self.mBuffer);
        if minus.is_some() {
            buffer.push_str(&self.mMinusDot[0]);
        }
        let (decimal_index, decimal) = format_str
            .find('.')
            .map_or((format_str.len(), ""), |i| (i, &format_str[i + 1..]));
        let mut integer = &format_str[if minus.is_some() { 1 } else { 0 }..decimal_index];
        if is_rmb && integer.len() > 16 {
            let len = integer.len() - 12;
            let mut group = len / 12;
            let mut index = len % 12;
            if index != 0 {
                self.processInteger(&mut buffer, &integer[..index], false, ty);
            }
            while group > 0 {
                self.processInteger(&mut buffer, &integer[index..index + 16], false, ty);
                index += 12;
                group -= 1;
            }
            buffer.push_str(&self.mMainUnits[2]);
            integer = &integer[index..];
            if zero_string(integer) {
                integer = "";
            }
        }
        self.processInteger(&mut buffer, integer, !decimal.is_empty(), ty);
        if is_rmb && !buffer.is_empty() {
            buffer.push_str(&self.mMoneyUnits[0]);
        }
        if !decimal.is_empty() {
            self.processDecimal(&mut buffer, decimal, ty);
        } else if is_rmb {
            buffer.push_str(&self.mMoneyUnits[3]);
        }
        self.mBuffer = buffer;
        self.mBuffer.clone()
    }

    // Java: void processInteger(StringBuffer buffer, String integerStr, boolean hasDecimal, short type)
    // java private void processInteger(StringBuffer buffer, String integerStr, boolean hasDecimal, short type) {
    // java if (!integerStr.matches("0+")) {
    // java int intStrLen = integerStr.length();
    // java int scale = intStrLen / 4;
    // java int mod = intStrLen % 4;
    // java if (scale >= 4) {
    // java mod = intStrLen - 12;
    // java } else if (mod == 0) {
    // java mod = 4;
    // java } else {
    // java ++scale;
    // java }
    // java
    // java if (scale > 4) {
    // java scale = 4;
    // java }
    // java
    // java String item = null;
    // java
    // java for(int i = 0; i < intStrLen; mod = 4) {
    // java --scale;
    // java item = integerStr.substring(i, i + mod);
    // java if (!item.matches("0+")) {
    // java this.formatString(item, scale, type, buffer);
    // java }
    // java
    // java i += mod;
    // java }
    // java
    // java } else {
    // java if (!isRMBType(type) || !hasDecimal) {
    // java buffer.append(this.mDigits[0]);
    // java }
    // java
    // java }
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:213
    // java private void processInteger(StringBuffer buffer, String integerStr, boolean hasDecimal, short type) {
    // java if (!integerStr.matches("0+")) {
    // java int intStrLen = integerStr.length();
    // java int scale = intStrLen / 4;
    // java int mod = intStrLen % 4;
    // java if (scale >= 4) {
    // java mod = intStrLen - 12;
    // java } else if (mod == 0) {
    // java mod = 4;
    // java } else {
    // java ++scale;
    // java }
    // java
    // java if (scale > 4) {
    // java scale = 4;
    // java }
    // java
    // java String item = null;
    // java
    // java for(int i = 0; i < intStrLen; mod = 4) {
    // java --scale;
    // java item = integerStr.substring(i, i + mod);
    // java if (!item.matches("0+")) {
    // java this.formatString(item, scale, type, buffer);
    // java }
    // java
    // java i += mod;
    // java }
    // java
    // java } else {
    // java if (!isRMBType(type) || !hasDecimal) {
    // java buffer.append(this.mDigits[0]);
    // java }
    // java
    // java }
    // java }
    pub fn processInteger(
        &mut self,
        buffer: &mut String,
        integer: &str,
        has_decimal: bool,
        ty: i16,
    ) {
        if !zero_string(integer) {
            let len = integer.len();
            let mut scale = len / 4;
            let mut size = len % 4;
            if scale >= 4 {
                size = len - 12;
            } else if size == 0 {
                size = 4;
            } else {
                scale += 1;
            }
            if scale > 4 {
                scale = 4;
            }
            let mut i = 0;
            while i < len {
                scale -= 1;
                let item = &integer[i..i + size];
                if !zero_string(item) {
                    self.formatString(item, scale as i32, ty, buffer);
                }
                i += size;
                size = 4;
            }
        } else if !Self::isRMBType(ty) || !has_decimal {
            buffer.push_str(&self.mDigits[0]);
        }
    }

    // Java: void processDecimal(StringBuffer buffer, String decimalStr, short type)
    // java private void processDecimal(StringBuffer buffer, String decimalStr, short type) {
    // java if (isRMBType(type)) {
    // java int num1 = decimalStr.charAt(0) - 48;
    // java int num2 = 0;
    // java if (decimalStr.length() > 1) {
    // java num2 = decimalStr.charAt(1) - 48;
    // java }
    // java
    // java if (num1 != 0) {
    // java buffer.append(this.mDigits[num1]);
    // java buffer.append(this.mMoneyUnits[1]);
    // java }
    // java
    // java if (num2 != 0) {
    // java buffer.append(this.mDigits[num2]);
    // java buffer.append(this.mMoneyUnits[2]);
    // java } else {
    // java buffer.append(this.mMoneyUnits[3]);
    // java }
    // java } else {
    // java buffer.append(this.mMinusDot[1]);
    // java
    // java for(int i = 0; i < decimalStr.length(); ++i) {
    // java int num = decimalStr.charAt(i) - 48;
    // java buffer.append(this.mDigits[num]);
    // java }
    // java }
    // java
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:250
    // java private void processDecimal(StringBuffer buffer, String decimalStr, short type) {
    // java if (isRMBType(type)) {
    // java int num1 = decimalStr.charAt(0) - 48;
    // java int num2 = 0;
    // java if (decimalStr.length() > 1) {
    // java num2 = decimalStr.charAt(1) - 48;
    // java }
    // java
    // java if (num1 != 0) {
    // java buffer.append(this.mDigits[num1]);
    // java buffer.append(this.mMoneyUnits[1]);
    // java }
    // java
    // java if (num2 != 0) {
    // java buffer.append(this.mDigits[num2]);
    // java buffer.append(this.mMoneyUnits[2]);
    // java } else {
    // java buffer.append(this.mMoneyUnits[3]);
    // java }
    // java } else {
    // java buffer.append(this.mMinusDot[1]);
    // java
    // java for(int i = 0; i < decimalStr.length(); ++i) {
    // java int num = decimalStr.charAt(i) - 48;
    // java buffer.append(this.mDigits[num]);
    // java }
    // java }
    // java
    // java }
    pub fn processDecimal(&self, buffer: &mut String, decimal: &str, ty: i16) {
        if Self::isRMBType(ty) {
            let num1 = (decimal.as_bytes()[0] - 48) as usize;
            let num2 = if decimal.len() > 1 {
                (decimal.as_bytes()[1] - 48) as usize
            } else {
                0
            };
            if num1 != 0 {
                buffer.push_str(&self.mDigits[num1]);
                buffer.push_str(&self.mMoneyUnits[1]);
            }
            if num2 != 0 {
                buffer.push_str(&self.mDigits[num2]);
                buffer.push_str(&self.mMoneyUnits[2]);
            } else {
                buffer.push_str(&self.mMoneyUnits[3]);
            }
        } else {
            buffer.push_str(&self.mMinusDot[1]);
            for c in decimal.bytes() {
                buffer.push_str(&self.mDigits[(c - 48) as usize]);
            }
        }
    }

    // Java: void formatString(String str, int majorScale, short type, StringBuffer buffer)
    // java private void formatString(String str, int majorScale, short type, StringBuffer buffer) {
    // java int length = str.length();
    // java int minorScale = 0;
    // java int num = 0;
    // java boolean preIsZero = false;
    // java if (length > 4) {
    // java for(int i = 0; i < length; ++i) {
    // java num = str.charAt(i) - 48;
    // java buffer.append(this.mDigits[num]);
    // java }
    // java } else {
    // java for(int i = 0; i < length; ++i) {
    // java minorScale = length - i - 1;
    // java num = str.charAt(i) - 48;
    // java if (preIsZero && num == 0) {
    // java if (minorScale == 0) {
    // java buffer.deleteCharAt(buffer.length() - 1);
    // java }
    // java } else if (num != 0 || (type & 4100) != 4100) {
    // java if ((minorScale == 0 || num != 1 || (type & 4098) != 4098) && (minorScale != 0 || num != 0)) {
    // java buffer.append(this.mDigits[num]);
    // java }
    // java
    // java if (num != 0) {
    // java buffer.append(this.formatMinorScale(minorScale));
    // java }
    // java
    // java preIsZero = num == 0;
    // java }
    // java }
    // java }
    // java
    // java buffer.append(this.formatMajorScale(majorScale));
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:280
    // java private void formatString(String str, int majorScale, short type, StringBuffer buffer) {
    // java int length = str.length();
    // java int minorScale = 0;
    // java int num = 0;
    // java boolean preIsZero = false;
    // java if (length > 4) {
    // java for(int i = 0; i < length; ++i) {
    // java num = str.charAt(i) - 48;
    // java buffer.append(this.mDigits[num]);
    // java }
    // java } else {
    // java for(int i = 0; i < length; ++i) {
    // java minorScale = length - i - 1;
    // java num = str.charAt(i) - 48;
    // java if (preIsZero && num == 0) {
    // java if (minorScale == 0) {
    // java buffer.deleteCharAt(buffer.length() - 1);
    // java }
    // java } else if (num != 0 || (type & 4100) != 4100) {
    // java if ((minorScale == 0 || num != 1 || (type & 4098) != 4098) && (minorScale != 0 || num != 0)) {
    // java buffer.append(this.mDigits[num]);
    // java }
    // java
    // java if (num != 0) {
    // java buffer.append(this.formatMinorScale(minorScale));
    // java }
    // java
    // java preIsZero = num == 0;
    // java }
    // java }
    // java }
    // java
    // java buffer.append(this.formatMajorScale(majorScale));
    // java }
    pub fn formatString(&self, text: &str, major_scale: i32, ty: i16, buffer: &mut String) {
        let length = text.len();
        let mut pre_is_zero = false;
        if length > 4 {
            for c in text.bytes() {
                buffer.push_str(&self.mDigits[(c - 48) as usize]);
            }
        } else {
            for (i, c) in text.bytes().enumerate() {
                let minor_scale = length - i - 1;
                let num = (c - 48) as usize;
                if pre_is_zero && num == 0 {
                    if minor_scale == 0 {
                        let mut units: Vec<u16> = buffer.encode_utf16().collect();
                        units.pop().expect("StringIndexOutOfBoundsException");
                        *buffer = String::from_utf16(&units).expect("unpaired Java UTF16");
                    }
                } else if num != 0 || (ty & 4100) != 4100 {
                    if (minor_scale == 0 || num != 1 || (ty & 4098) != 4098)
                        && (minor_scale != 0 || num != 0)
                    {
                        buffer.push_str(&self.mDigits[num]);
                    }
                    if num != 0 {
                        buffer.push_str(&self.formatMinorScale(minor_scale as i32));
                    }
                    pre_is_zero = num == 0;
                }
            }
        }
        buffer.push_str(&self.formatMajorScale(major_scale));
    }

    // Java: String formatMajorScale(int majorScale)
    // java private String formatMajorScale(int majorScale) {
    // java assert majorScale < 4 && majorScale >= 0;
    // java
    // java return majorScale > 0 ? this.mMainUnits[majorScale - 1] : "";
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:315
    // java private String formatMajorScale(int majorScale) {
    // java assert majorScale < 4 && majorScale >= 0;
    // java
    // java return majorScale > 0 ? this.mMainUnits[majorScale - 1] : "";
    // java }
    pub fn formatMajorScale(&self, scale: i32) -> String {
        debug_assert!((0..4).contains(&scale));
        if scale > 0 {
            self.mMainUnits[scale as usize - 1].clone()
        } else {
            String::new()
        }
    }

    // Java: String formatMinorScale(int minorScale)
    // java private String formatMinorScale(int minorScale) {
    // java assert minorScale < 4 && minorScale >= 0;
    // java
    // java return minorScale > 0 ? this.mMinUnits[minorScale - 1] : "";
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:321
    // java private String formatMinorScale(int minorScale) {
    // java assert minorScale < 4 && minorScale >= 0;
    // java
    // java return minorScale > 0 ? this.mMinUnits[minorScale - 1] : "";
    // java }
    pub fn formatMinorScale(&self, scale: i32) -> String {
        debug_assert!((0..4).contains(&scale));
        if scale > 0 {
            self.mMinUnits[scale as usize - 1].clone()
        } else {
            String::new()
        }
    }

    // Java: void loadRes(byte numeralShape, int resState)
    // java private synchronized void loadRes(byte numeralShape, int resState) {
    // java if (this.mLatestShape != numeralShape || (this.mResState & resState) != resState) {
    // java if (this.mLatestShape != numeralShape) {
    // java this.mResState = 0;
    // java }
    // java
    // java Properties shapeCache = GlobalEnv.localeStrManager.getNumShapeCache();
    // java if ((resState & 1) != 0 && (this.mResState & 1) == 0) {
    // java List<String> digits = ResourceLoader.getArray(shapeCache, ResourceDef.getNumShapeName(numeralShape));
    // java if (digits != null) {
    // java digits.toArray(this.mDigits);
    // java this.mResState |= 1;
    // java this.mLatestShape = numeralShape;
    // java }
    // java }
    // java
    // java if ((this.mResState & 2) != 2 && (resState & 2) != 0) {
    // java List<String> units = ResourceLoader.getArray(shapeCache, ResourceDef.getNumShapeUnitName(numeralShape));
    // java if (units != null) {
    // java int size = units.size();
    // java int index = 0;
    // java
    // java for(int i = 0; i < this.mMinUnits.length; ++i) {
    // java this.mMinUnits[i] = (String)units.get(index);
    // java ++index;
    // java }
    // java
    // java for(int i = 0; i < this.mMainUnits.length; ++i) {
    // java this.mMainUnits[i] = (String)units.get(index);
    // java ++index;
    // java }
    // java
    // java this.mMinusDot[0] = index < size ? (String)units.get(index) : "-";
    // java ++index;
    // java this.mMinusDot[1] = index < size ? (String)units.get(index) : ".";
    // java ++index;
    // java
    // java for(int i = 0; i < this.mMoneyUnits.length; ++i) {
    // java this.mMoneyUnits[i] = index < size ? (String)units.get(index) : "";
    // java ++index;
    // java }
    // java
    // java this.mResState |= 2;
    // java this.mLatestShape = numeralShape;
    // java }
    // java }
    // java }
    // java }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumericCase.java:327
    // java private synchronized void loadRes(byte numeralShape, int resState) {
    // java if (this.mLatestShape != numeralShape || (this.mResState & resState) != resState) {
    // java if (this.mLatestShape != numeralShape) {
    // java this.mResState = 0;
    // java }
    // java
    // java Properties shapeCache = GlobalEnv.localeStrManager.getNumShapeCache();
    // java if ((resState & 1) != 0 && (this.mResState & 1) == 0) {
    // java List<String> digits = ResourceLoader.getArray(shapeCache, ResourceDef.getNumShapeName(numeralShape));
    // java if (digits != null) {
    // java digits.toArray(this.mDigits);
    // java this.mResState |= 1;
    // java this.mLatestShape = numeralShape;
    // java }
    // java }
    // java
    // java if ((this.mResState & 2) != 2 && (resState & 2) != 0) {
    // java List<String> units = ResourceLoader.getArray(shapeCache, ResourceDef.getNumShapeUnitName(numeralShape));
    // java if (units != null) {
    // java int size = units.size();
    // java int index = 0;
    // java
    // java for(int i = 0; i < this.mMinUnits.length; ++i) {
    // java this.mMinUnits[i] = (String)units.get(index);
    // java ++index;
    // java }
    // java
    // java for(int i = 0; i < this.mMainUnits.length; ++i) {
    // java this.mMainUnits[i] = (String)units.get(index);
    // java ++index;
    // java }
    // java
    // java this.mMinusDot[0] = index < size ? (String)units.get(index) : "-";
    // java ++index;
    // java this.mMinusDot[1] = index < size ? (String)units.get(index) : ".";
    // java ++index;
    // java
    // java for(int i = 0; i < this.mMoneyUnits.length; ++i) {
    // java this.mMoneyUnits[i] = index < size ? (String)units.get(index) : "";
    // java ++index;
    // java }
    // java
    // java this.mResState |= 2;
    // java this.mLatestShape = numeralShape;
    // java }
    // java }
    // java }
    // java }
    pub fn loadRes(&mut self, shape: i8, state: i32) {
        if self.mLatestShape != shape || (self.mResState & state) != state {
            if self.mLatestShape != shape {
                self.mResState = 0;
            }
            let cache = GlobalEnv::localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getNumShapeCache();
            if state & 1 != 0 && self.mResState & 1 == 0 {
                if let Some(digits) = ResourceLoader::ResourceLoader::getArray_from_properties(
                    &cache,
                    &ResourceDef::ResourceDef::getNumShapeName(shape),
                ) {
                    for (i, digit) in digits.iter().take(self.mDigits.len()).enumerate() {
                        self.mDigits[i] = digit.clone();
                    }
                    self.mResState |= 1;
                    self.mLatestShape = shape;
                }
            }
            if self.mResState & 2 != 2 && state & 2 != 0 {
                if let Some(units) = ResourceLoader::ResourceLoader::getArray_from_properties(
                    &cache,
                    &ResourceDef::ResourceDef::getNumShapeUnitName(shape),
                ) {
                    let mut index = 0;
                    for unit in &mut self.mMinUnits {
                        *unit = units[index].clone();
                        index += 1;
                    }
                    for unit in &mut self.mMainUnits {
                        *unit = units[index].clone();
                        index += 1;
                    }
                    self.mMinusDot[0] = units.get(index).cloned().unwrap_or_else(|| "-".into());
                    index += 1;
                    self.mMinusDot[1] = units.get(index).cloned().unwrap_or_else(|| ".".into());
                    index += 1;
                    for unit in &mut self.mMoneyUnits {
                        *unit = units.get(index).cloned().unwrap_or_default();
                        index += 1;
                    }
                    self.mResState |= 2;
                    self.mLatestShape = shape;
                }
            }
        }
    }
}

// rust: Java String.matches("0+") requires at least one zero.
fn zero_string(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c == b'0')
}
