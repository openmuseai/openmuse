#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut, unused_assignments)]

// Java: import cn.wps.kfc.numfmt.format.NumFmtSection
use crate::cn::wps::kfc::numfmt::format::NumFmtSection;
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

use std::sync::OnceLock;

// Java field: char numberSign
const numberSign: char = '#';
// Java field: char zeroSign
const zeroSign: char = '0';
// Java field: Pattern roundNumberPattern
static roundNumberPattern: OnceLock<String> = OnceLock::new();
// Java field: Pattern groupNumberPattern
static groupNumberPattern: OnceLock<String> = OnceLock::new();
// Java field: Pattern fractionNumberPattern
static fractionNumberPattern: OnceLock<String> = OnceLock::new();
// Java field: Pattern thaiNumberPattern
static thaiNumberPattern: OnceLock<String> = OnceLock::new();
// Java field: String invalidNumberSignString
// java for(int i = 0; i < 255; ++i) { buf.append('#'); }
// java invalidNumberSignString = buf.toString();
pub static invalidNumberSignString: &str = "###############################################################################################################################################################################################################################################################";
// Java field: String jpFontStyle1
static jpFontStyle1: &str = "ＭＳ Ｐゴシック";

// Java: public class NumFmtUtil
pub struct NumFmtUtil {
}

// Java: public class NumFmtUtil
impl NumFmtUtil {
    // Java: NumFmtUtil()
    pub fn new() -> Self {
        // Java: super();
        NumFmtUtil {}
    }

    // Java: static String modifyFormatByFontStyle(String format, String fontName)
    pub fn modifyFormatByFontStyle(format: &str, fontName: &str) -> String {
        // Java: if (format != null && fontName != null)
        // LLM Rust Null-Safety: &str is never null in Rust
        if !format.is_empty() && !fontName.is_empty() {
            let mut format = format.to_string();
            // Java: if (fontName.equals("ＭＳ Ｐゴシック"))
            if fontName == "ＭＳ Ｐゴシック" {
                // Java: format = format.replaceAll("\\\\", "¥");
                format = format.replace("\\", "¥");
            }
            // Java: return format;
            return format;
        }
        // Java: else
        else {
            // Java: return format;
            return format.to_string();
        }
    }

    // Java: static void getNumberSign(int count, StringBuffer buffer)
    pub fn getNumberSign(count: i32, buffer: &mut String) {
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: buffer.append('#');
            buffer.push('#');
        }
    }

    // Java: static char[] fillChar(char c, int count)
    pub fn fillChar(c: char, count: i32) -> Vec<char> {
        let mut count = count;
        // Java: if (count < 0)
        if count < 0 {
            // Java: count = 0;
            count = 0;
        }
        // Java: char[] fills = new char[count];
        let mut fills = vec!['\0'; count as usize];
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count as usize {
            // Java: fills[i] = c;
            fills[i] = c;
        }
        // Java: return fills;
        fills
    }

    // Java: static CharSequence fillCharEx(char c, int count)
    pub fn fillCharEx(c: char, count: i32) -> String {
        // Java: StringBuffer buffer = new StringBuffer(count);
        let mut buffer = String::with_capacity(count as usize);
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: buffer.append(c);
            buffer.push(c);
        }
        // Java: return buffer.toString();
        buffer
    }

    // Java: static String filterInvalidChar(String formatCode)
    pub fn filterInvalidChar(formatCode: &str) -> Option<String> {
        // Java: if (formatCode == null)
        // LLM Rust Null-Safety: using Option return type for null case
        if formatCode.is_empty() {
            // Java: return null;
            return None;
        }
        // Java: else
        else {
            // Java: boolean isInSquare = false;
            let mut isInSquare = false;
            // Java: boolean isInRef = false;
            let mut isInRef = false;
            // Java: boolean isEscape = false;
            let mut isEscape = false;
            // Java: StringBuffer stringBuffer = new StringBuffer();
            let mut stringBuffer = String::new();
            // Java: int length = formatCode.length();
            let length = formatCode.len();
            // Java: stringBuffer.setLength(0);
            stringBuffer.clear();
            // Java: for(int i = 0; i < length; ++i)
            for (i, c_byte) in formatCode.chars().enumerate() {
                // Java: char c = formatCode.charAt(i);
                let c = c_byte;
                // Java: boolean isMetaChar = !isInRef && !isEscape && !isInSquare;
                let isMetaChar = !isInRef && !isEscape && !isInSquare;
                // Java: switch (c)
                match c {
                    // Java: case '!':
                    // LLM Rust match: combined with case '\\' into '!' | '\\' arm
                    // Java: case '\\':
                    '!' | '\\' => {
                        // Java: isEscape = isMetaChar;
                        isEscape = isMetaChar;
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '"':
                    '"' => {
                        // Java: if (!isEscape)
                        if !isEscape {
                            // Java: isInRef = !isInRef;
                            isInRef = !isInRef;
                        }
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '[':
                    '[' => {
                        // Java: isInSquare = !isEscape && !isInRef;
                        isInSquare = !isEscape && !isInRef;
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case ']':
                    ']' => {
                        // Java: if (!isEscape && !isInRef)
                        if !isEscape && !isInRef {
                            // Java: isInSquare = false;
                            isInSquare = false;
                        }
                    }
                    // Java: default:
                    _ => {}
                }
                // Java: if (isMetaChar)
                if isMetaChar {
                    // Java: stringBuffer.append(c);
                    stringBuffer.push(c);
                }
                // Java: if (c != '!' && c != '\\')
                if c != '!' && c != '\\' {
                    // Java: isEscape = false;
                    isEscape = false;
                }
            }
            // Java: return stringBuffer.toString();
            return Some(stringBuffer);
        }
    }

    // Java: static String changeAllDecimalDigits(String fullFormatCode, int offset)
    pub fn changeAllDecimalDigits(fullFormatCode: &str, offset: i32) -> String {
        // Java: String[] codeParts = new String[4];
        let mut codeParts: [Option<String>; 4] = [None, None, None, None];
        // Java: NumFmtSection.splitGroup(fullFormatCode, codeParts);
        // LLM Java Platform API: NumFmtSection.splitGroup not yet available
        // Java: StringBuffer buffer = new StringBuffer();
        let mut buffer = String::new();
        // Java: for(String part : codeParts)
        for part in &codeParts {
            // Java: if (part != null)
            if let Some(part) = part {
                // Java: buffer.append(changeDecimalDigits(part, offset));
                buffer.push_str(&Self::changeDecimalDigits(part, offset));
                // Java: buffer.append(';');
                buffer.push(';');
            }
        }
        // Java: if (buffer.length() > 0)
        if buffer.len() > 0 {
            // Java: buffer.deleteCharAt(buffer.length() - 1);
            buffer.pop();
            // Java: return buffer.toString();
            return buffer;
        }
        // Java: else
        else {
            // Java: return fullFormatCode;
            return fullFormatCode.to_string();
        }
    }

    // Java: static String changeDecimalDigits(String formatCode, int offset)
    pub fn changeDecimalDigits(formatCode: &str, offset: i32) -> String {
        // Java: if (offset == 0)
        if offset == 0 {
            // Java: return formatCode;
            return formatCode.to_string();
        }
        // Java: else
        else {
            let mut formatCode = formatCode.to_string();
            // Java: if (formatCode.equals("General"))
            if formatCode == "General" {
                // Java: formatCode = "0";
                formatCode = "0".to_string();
            }
            // Java: if (fractionNumberPattern.matcher(formatCode).find())
            // LLM Java Platform API: Pattern matching via regex, using todo!() for regex operations
            let fractionMatch: bool = todo!("fractionNumberPattern.matcher(formatCode).find()");
            if fractionMatch {
                // Java: return formatCode;
                return formatCode;
            }
            // Java: else
            else {
                // Java: Matcher m = groupNumberPattern.matcher(formatCode);
                // LLM Java Platform API: Pattern/Matcher replaced by todo!() regex operations below
                let m: bool = true;
                // Java: if (!m.find())
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: m = thaiNumberPattern.matcher(formatCode);
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: if (!m.find())
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: m = roundNumberPattern.matcher(formatCode);
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: if (!m.find())
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: return formatCode;
                // LLM JavaLine Aggregation: merged into fallback regex matching chain below
                // Java: String decimalCode = m.group(2);
                let decimalCode: Option<String> = todo!("regex matcher group(2)");
                // Java: int decCodeLen = decimalCode != null ? decimalCode.length() : 0;
                let decCodeLen = decimalCode.as_ref().map_or(0, |s| s.len() as i32);
                // Java: StringBuffer stringBuffer = new StringBuffer();
                let mut stringBuffer = String::new();
                // Java: stringBuffer.setLength(0);
                stringBuffer.clear();
                // Java: stringBuffer.append(formatCode);
                stringBuffer.push_str(&formatCode);
                // Java: String newDecimal = "";
                let mut newDecimal = String::new();
                // Java: if (offset > 0)
                if offset > 0 {
                    // Java: newDecimal = fillCharEx('0', offset + decCodeLen).toString();
                    newDecimal = Self::fillCharEx('0', offset + decCodeLen);
                    // Java: if (decimalCode != null)
                    if decimalCode.is_some() {
                        // Java: stringBuffer.replace(m.start(2), m.end(2), newDecimal);
                        todo!("stringBuffer.replace using matcher positions");
                    }
                    // Java: else
                    else {
                        // Java: stringBuffer.insert(m.end(1), '.');
                        // LLM Java Platform API: insert at matcher end position
                        let _insert_pos: i32 = todo!("m.end(1)");
                        // Java: stringBuffer.insert(m.end(1) + 1, newDecimal);
                        todo!("stringBuffer.insert using matcher positions");
                    }
                }
                // Java: else if (decCodeLen > 0)
                else if decCodeLen > 0 {
                    // Java: int newDecimalCount = decCodeLen + offset > -1 ? decCodeLen + offset : 0;
                    let newDecimalCount = if decCodeLen + offset > -1 { decCodeLen + offset } else { 0 };
                    // Java: newDecimal = fillCharEx('0', newDecimalCount).toString();
                    newDecimal = Self::fillCharEx('0', newDecimalCount);
                    // Java: stringBuffer.replace(newDecimalCount != 0 ? m.start(2) : m.start(2) - 1, m.end(2), newDecimal);
                    todo!("stringBuffer.replace using matcher positions");
                }
                // Java: return stringBuffer.toString();
                return stringBuffer;
            }
        }
    }

    // Java: static String getDeciaml(int decimalCount)
    pub fn getDeciaml(decimalCount: i32) -> String {
        // Java: if (decimalCount <= 0)
        if decimalCount <= 0 {
            // Java: return "";
            return String::new();
        }
        // Java: else
        else {
            // Java: StringBuffer buffer = new StringBuffer();
            let mut buffer = String::new();
            // Java: buffer.append(".");
            buffer.push('.');
            // Java: for(int i = 0; i < decimalCount; ++i)
            for i in 0..decimalCount {
                // Java: buffer.append('0');
                buffer.push('0');
            }
            // Java: return buffer.toString();
            return buffer;
        }
    }
}
