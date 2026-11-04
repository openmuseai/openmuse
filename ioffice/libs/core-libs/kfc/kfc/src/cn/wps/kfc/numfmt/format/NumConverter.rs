// Java field: int GENERAL_NUM_LENGTH
pub(crate) const GENERAL_NUM_LENGTH: i32 = 11;

// Java: public class NumConverter
pub struct NumConverter;

// Java: public class NumConverter
impl NumConverter {
    // Java: NumConverter()

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:11
    // java public NumConverter() {
    // java super();
    // java }
    pub fn new() -> Self {
        // Java: super();
        NumConverter
    }

    // Java: static String convertGeneral(byte numeralShape, double value, int maxWidth)
    #[allow(unreachable_code, unused_variables)]

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:15
    // java public static String convertGeneral(byte numeralShape, double value, int maxWidth) {
    // java StringBuffer buffer = new StringBuffer();
    // java int sign = value < (double)0.0F ? 1 : 0;
    // java int width = maxWidth <= 11 ? maxWidth - sign : 11;
    // java double absValue = Math.abs(value);
    // java int expontent = (int)Math.log10(absValue);
    // java int integerCount = expontent < 1 ? 1 : expontent + 1;
    // java int sciPartLen = Math.abs(expontent) + 1 >= 100 ? 7 : 6;
    // java if (expontent < 1) {
    // java if (absValue == (double)0.0F) {
    // java buffer.append(0);
    // java } else if (absValue < 1.0E-4 && width >= 5) {
    // java String format = NumberToStr.double2StandardE(absValue, 14);
    // java int epos = format.indexOf("E");
    // java int decimalCount = epos - 1 + Integer.parseInt(format.substring(epos + 2)) - 1;
    // java if (decimalCount + 1 >= width) {
    // java String[] science = new String[4];
    // java NumberToStr.double2Science(absValue, width - sciPartLen, 1, science);
    // java scienceToBuffer(science, buffer);
    // java } else {
    // java NumberToStr.double2String(absValue, width - 2, buffer);
    // java }
    // java } else if (absValue < 0.001 && width == 5) {
    // java String[] science = new String[4];
    // java NumberToStr.double2Science(absValue, 0, 1, science);
    // java scienceToBuffer(science, buffer);
    // java } else {
    // java NumberToStr.double2String(absValue, width - 2, buffer);
    // java }
    // java } else if (integerCount > 5 && integerCount > width) {
    // java String[] science = new String[4];
    // java NumberToStr.double2Science(absValue, width - sciPartLen, 1, science);
    // java scienceToBuffer(science, buffer);
    // java } else {
    // java NumberToStr.double2String(absValue, width - integerCount - 1, buffer);
    // java }
    // java
    // java if (sign > 0) {
    // java buffer.insert(0, '-');
    // java }
    // java
    // java if (buffer.length() > maxWidth) {
    // java buffer.setLength(0);
    // java NumFmtUtil.getNumberSign(maxWidth, buffer);
    // java } else {
    // java transDigits((StringBuffer)buffer, 0, buffer.length(), numeralShape);
    // java }
    // java
    // java return buffer.toString();
    // java }
    pub fn convertGeneral(numeral_shape: i8, value: f64, max_width: i32) -> String {
        use crate::cn::wps::kfc::numfmt::{
            global::GlobalEnv::localeStrManager, util::NumberToStr::NumberToStr,
        };
        // rust: int sign=value<0?1:0;int width=maxWidth<=11?maxWidth-sign:11;double absValue=Math.abs(value);int expontent=(int)Math.log10(absValue);int integerCount=expontent<1?1:expontent+1;int sciPartLen=Math.abs(expontent)+1>=100?7:6;
        let mut buffer = String::new();
        let sign = if value < 0.0 { 1 } else { 0 };
        let width = if max_width <= 11 {
            max_width - sign
        } else {
            11
        };
        let abs_value = value.abs();
        let exponent = abs_value.log10() as i32;
        let integer_count = if exponent < 1 {
            1
        } else {
            exponent.wrapping_add(1)
        };
        let sci_len = if exponent.wrapping_abs().wrapping_add(1) >= 100 {
            7
        } else {
            6
        };
        let science = |decimals: i32, buffer: &mut String| {
            let mut sci = std::array::from_fn(|_| String::new());
            NumberToStr::double2Science(abs_value, decimals, 1, &mut sci);
            Self::scienceToBuffer(&sci, buffer);
        };
        if exponent < 1 {
            if abs_value == 0.0 {
                buffer.push('0');
            } else if abs_value < 1.0e-4 && width >= 5 {
                // java decimalCount=epos-1+Integer.parseInt(format.substring(epos+2))-1;
                let standard = NumberToStr::double2StandardE(abs_value, 14);
                let epos = standard.find('E').unwrap();
                let decimal_count =
                    epos as i32 - 1 + standard[epos + 2..].parse::<i32>().unwrap() - 1;
                if decimal_count + 1 >= width {
                    science(width - sci_len, &mut buffer);
                } else {
                    NumberToStr::double2String_to_buffer(abs_value, width - 2, &mut buffer);
                }
            } else if abs_value < 0.001 && width == 5 {
                science(0, &mut buffer);
            } else {
                NumberToStr::double2String_to_buffer(abs_value, width - 2, &mut buffer);
            }
        } else if integer_count > 5 && integer_count > width {
            science(width - sci_len, &mut buffer);
        } else {
            NumberToStr::double2String_to_buffer(abs_value, width - integer_count - 1, &mut buffer);
        }
        if sign > 0 {
            buffer.insert(0, '-');
        }
        // rust: if(buffer.length()>maxWidth){buffer.setLength(0);NumFmtUtil.getNumberSign(maxWidth,buffer);}else transDigits(...);
        if buffer.encode_utf16().count() as i32 > max_width {
            buffer = "#".repeat(max_width.max(0) as usize);
        } else {
            let end = buffer.len();
            Self::transDigits_buffer(&mut buffer, 0, end, numeral_shape);
        }
        buffer
    }

    // Java: static void scienceToBuffer(String[] science, StringBuffer buffer)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:66
    // java private static void scienceToBuffer(String[] science, StringBuffer buffer) {
    // java buffer.append(science[0]);
    // java if (science[1].length() > 0) {
    // java buffer.append(GlobalEnv.localeStrManager.getDecimalPoint());
    // java buffer.append(science[1]);
    // java }
    // java
    // java buffer.append('E');
    // java buffer.append(science[2]);
    // java if (science[3].length() == 1) {
    // java buffer.append('0');
    // java }
    // java
    // java buffer.append(science[3]);
    // java }
    fn scienceToBuffer(science: &[String; 4], buffer: &mut String) {
        // Java: buffer.append(science[0]);
        buffer.push_str(&science[0]);
        // Java: if (science[1].length() > 0)
        if !science[1].is_empty() {
            // Java: buffer.append(GlobalEnv.localeStrManager.getDecimalPoint());
            buffer.push_str(
                &crate::cn::wps::kfc::numfmt::global::GlobalEnv::localeStrManager
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .getDecimalPoint(),
            );
            // Java: buffer.append(science[1]);
            buffer.push_str(&science[1]);
        }
        // Java: buffer.append('E');
        buffer.push('E');
        // Java: buffer.append(science[2]);
        buffer.push_str(&science[2]);
        // Java: if (science[3].length() == 1)
        if science[3].len() == 1 {
            // Java: buffer.append('0');
            buffer.push('0');
        }
        // Java: buffer.append(science[3]);
        buffer.push_str(&science[3]);
    }

    // Java: static String convertTimeCase(byte numeralShape, String time, boolean isElapsed)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:82
    // java public static String convertTimeCase(byte numeralShape, String time, boolean isElapsed) {
    // java short type = NumericCase.getTimeCaseType(numeralShape, isElapsed);
    // java if (type == 0) {
    // java return time;
    // java } else {
    // java int len = time.length();
    // java
    // java int index;
    // java for(index = -1; index < len - 1 && time.charAt(index + 1) == '0'; ++index) {
    // java }
    // java
    // java String zeros = "";
    // java String numbers = time;
    // java if (index > -1 && index <= len - 1) {
    // java zeros = transDigits((String)time, 0, index + 1, numeralShape);
    // java numbers = time.substring(index + 1, time.length());
    // java }
    // java
    // java return zeros + NumericCase.getInstance().deform(numeralShape, type, numbers);
    // java }
    // java }
    pub fn convertTimeCase(numeral_shape: i8, time: &str, is_elapsed: bool) -> String {
        let ty = super::NumericCase::NumericCase::getTimeCaseType(numeral_shape, is_elapsed);
        if ty == 0 {
            return time.into();
        }
        let zeros = time.bytes().take_while(|c| *c == b'0').count();
        let leading = if zeros > 0 {
            Self::transDigits(time, 0, zeros, numeral_shape)
        } else {
            String::new()
        };
        leading
            + &super::NumericCase::NumericCase::getInstance()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .deform(numeral_shape, ty, &time[zeros..])
    }

    // Java: static String convertNumeric(short dbNum, byte numeralShape, boolean isRMBScale, double value)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:104
    // java public static String convertNumeric(short dbNum, byte numeralShape, boolean isRMBScale, double value) {
    // java short type = NumericCase.getCaseType(dbNum, numeralShape, isRMBScale);
    // java String number = "";
    // java if (!NumericCase.isRMBType(type)) {
    // java number = NumberToStr.double2String(value, 15);
    // java } else {
    // java number = NumberToStr.double2String(value, 2, false);
    // java }
    // java
    // java return NumericCase.getInstance().deform(numeralShape, type, number);
    // java }
    pub fn convertNumeric(
        db_num: i16,
        numeral_shape: i8,
        is_rmb_scale: bool,
        value: f64,
    ) -> String {
        let ty = super::NumericCase::NumericCase::getCaseType(db_num, numeral_shape, is_rmb_scale);
        let number = if !super::NumericCase::NumericCase::isRMBType(ty) {
            crate::cn::wps::kfc::numfmt::util::NumberToStr::NumberToStr::double2String_precision(
                value, 15,
            )
        } else {
            crate::cn::wps::kfc::numfmt::util::NumberToStr::NumberToStr::double2String_percent(
                value, 2, false,
            )
        };
        super::NumericCase::NumericCase::getInstance()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .deform(numeral_shape, ty, &number)
    }

    // Java: static String transDigits(String digits, int start, int end, byte numeralShape)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:116
    // java public static String transDigits(String digits, int start, int end, byte numeralShape) {
    // java assert start >= 0 && end <= digits.length();
    // java
    // java return NCLAccesser.isGeneralShape(numeralShape) ? digits.substring(start, end) : NumericCase.getInstance().transDigits(digits, start, end, numeralShape);
    // java }
    pub fn transDigits(digits: &str, start: usize, end: usize, numeral_shape: i8) -> String {
        // Java: assert start >= 0 && end <= digits.length();
        debug_assert!(start <= end && end <= digits.encode_utf16().count());
        // Java: return NCLAccesser.isGeneralShape(numeralShape) ? digits.substring(start, end) : NumericCase.getInstance().transDigits(digits, start, end, numeralShape);
        if crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser::isGeneralShape(
            numeral_shape,
        ) {
            String::from_utf16(&digits.encode_utf16().collect::<Vec<_>>()[start..end])
                .expect("unpaired Java UTF16")
        } else {
            super::NumericCase::NumericCase::getInstance()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .transDigits_str(digits, start as i32, end as i32, numeral_shape)
        }
    }

    // Java: static void transDigits(StringBuffer digits, int start, int end, byte numeralShape)

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/format/NumConverter.java:122
    // java public static void transDigits(StringBuffer digits, int start, int end, byte numeralShape) {
    // java if (!NCLAccesser.isGeneralShape(numeralShape)) {
    // java NumericCase.getInstance().transDigits(digits, start, end, numeralShape);
    // java }
    // java }
    pub fn transDigits_buffer(digits: &mut String, start: usize, end: usize, numeral_shape: i8) {
        // Java: if (!NCLAccesser.isGeneralShape(numeralShape))
        if !crate::cn::wps::kfc::numfmt::global::NCLAccesser::NCLAccesser::isGeneralShape(
            numeral_shape,
        ) {
            // Java: NumericCase.getInstance().transDigits(digits, start, end, numeralShape);
            super::NumericCase::NumericCase::getInstance()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .transDigits(digits, start as i32, end as i32, numeral_shape)
        }
    }

    // Java: static String generalConvertOld(byte numeralShape, double value, int maxWidth)
    #[allow(unreachable_code, unused_variables, unused_mut)]
    pub fn generalConvertOld(_numeral_shape: i8, value: f64, max_width: i32) -> String {
        // Java: StringBuffer buffer = new StringBuffer();
        let mut buffer = String::new();
        // Java: String[] science = new String[4];
        let mut _science = [String::new(), String::new(), String::new(), String::new()];
        // Java: int sign = value < (double)0.0F ? 1 : 0;
        let sign: i32 = if value < 0.0 { 1 } else { 0 };
        // Java: int width = maxWidth <= 11 ? maxWidth - sign : 11;
        let _width: i32 = if max_width <= 11 {
            max_width - sign
        } else {
            11
        };
        // Java: double absValue = Math.abs(value);
        let abs_value: f64 = value.abs();
        // Java: int expontent = (int)Math.log10(absValue);
        let expontent: i32 = if abs_value > 0.0 {
            abs_value.log10() as i32
        } else {
            0
        };
        // Java: int integerCount = expontent < 1 ? 1 : expontent + 1;
        let integer_count: i32 = if expontent < 1 { 1 } else { expontent + 1 };
        // Java: int sciPartLen = Math.abs(expontent) + 1 >= 100 ? 7 : 6;
        let _sci_part_len: i32 = if expontent.abs() + 1 >= 100 { 7 } else { 6 };
        // Java: if (expontent < 1)
        if expontent < 1 {
            // Java: if (absValue < 1.0E-4)
            if abs_value < 1.0e-4 {
                // Java: String format = NumberToStr.double2StandardE(absValue, 14);
                // Java: int epos = format.indexOf("E");
                // Java: int decimalCount = epos - 1 + Integer.parseInt(format.substring(epos + 2)) - 1;
                // Java: if (decimalCount <= 9 && decimalCount <= width)
                {
                    // Java: NumberToStr.double2String(absValue, width - 2, false, buffer);
                }
                // Java: else if (width >= 5)
                {
                    // Java: NumberToStr.double2Science(absValue, width - sciPartLen, 1, science);
                    // Java: scienceToBuffer(science, buffer);
                }
                // Java: else
                {
                    // Java: buffer.append('0');
                }
                todo!("generalConvertOld: NumberToStr not yet available")
            }
            // Java: else if (absValue < 0.001)
            else if abs_value < 0.001 {
                // Java: if (width < 6)
                {
                    // Java: if (width == 5)
                    {
                        // Java: NumberToStr.double2Science(absValue, 0, 1, science);
                        // Java: scienceToBuffer(science, buffer);
                    }
                    // Java: else
                    {
                        // Java: buffer.append(0);
                    }
                }
                // Java: else
                {
                    // Java: NumberToStr.double2String(absValue, width - 2, false, buffer);
                }
                todo!("generalConvertOld: NumberToStr not yet available")
            }
            // Java: else
            else {
                // Java: NumberToStr.double2String(absValue, width - 2, false, buffer);
                todo!("generalConvertOld: NumberToStr not yet available")
            }
        }
        // Java: else if (integerCount > 11)
        else if integer_count > 11 {
            // Java: NumberToStr.double2Science(absValue, width - sciPartLen, 1, science);
            // Java: scienceToBuffer(science, buffer);
            todo!("generalConvertOld: NumberToStr not yet available")
        }
        // Java: else if (integerCount > 5)
        else if integer_count > 5 {
            // Java: if (integerCount > width)
            if integer_count > _width {
                // Java: NumberToStr.double2Science(absValue, width - sciPartLen, 1, science);
                // Java: scienceToBuffer(science, buffer);
                todo!("generalConvertOld: NumberToStr not yet available")
            }
            // Java: else
            else {
                // Java: NumberToStr.double2String(absValue, width - integerCount - 1, false, buffer);
                todo!("generalConvertOld: NumberToStr not yet available")
            }
        }
        // Java: else
        else {
            // Java: NumberToStr.double2String(absValue, width - integerCount - 1, false, buffer);
            todo!("generalConvertOld: NumberToStr not yet available")
        }
        // Java: if (sign > 0)
        if sign > 0 {
            // Java: buffer.insert(0, '-');
            buffer.insert(0, '-');
        }
        // Java: if (buffer.length() > maxWidth)
        if buffer.len() as i32 > max_width {
            // Java: buffer.setLength(0);
            buffer.clear();
            // Java: NumFmtUtil.getNumberSign(maxWidth, buffer);
            todo!("generalConvertOld: NumFmtUtil not yet available")
        }
        // Java: return buffer.toString();
        buffer
    }
}
