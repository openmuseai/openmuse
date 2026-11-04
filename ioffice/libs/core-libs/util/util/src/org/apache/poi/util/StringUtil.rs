#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unreachable_code)]

// Java: import java.io.UnsupportedEncodingException
// Java: import java.text.FieldPosition
// Java: import java.text.NumberFormat
// Java: import java.util.Iterator

use super::LittleEndianInput::LittleEndianInput;
use super::LittleEndianOutput::LittleEndianOutput;

// Java field: String ENCODING_ISO_8859_1
const ENCODING_ISO_8859_1: &str = "ISO-8859-1";

// Java: public class StringUtil
pub struct StringUtil;

// Java: public class StringsIterator
pub struct StringsIterator {
    // Java field: String[] strings
    strings: Vec<String>,
    // Java field: int position
    position: usize,
}

// Java: public class StringUtil
impl StringUtil {
    // Java: StringUtil()
    fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: no super() needed in Rust
        Self
    }

    // Java: static String getFromUnicodeLE(byte[] string, int offset, int len)
    pub fn getFromUnicodeLE(string: &[u8], offset: usize, len: usize) -> Result<String, String> {
        // Java: if (offset >= 0 && (offset < string.length || offset <= 0))
        if offset >= 0 && (offset < string.len() || offset <= 0) {
            // Java: if (len >= 0 && (string.length - offset) / 2 >= len)
            if len >= 0 && (string.len() - offset) / 2 >= len {
                // Java: try
                // LLM Rust Equivalent: Result<> return type replaces try/catch
                let mut chars = Vec::with_capacity(len);
                for i in 0..len {
                    let idx = offset + i * 2;
                    if idx + 1 < string.len() {
                        let ch = (string[idx] as u16) | ((string[idx + 1] as u16) << 8);
                        chars.push(ch);
                    }
                }
                // Java: return new String(string, offset, len * 2, "UTF-16LE");
                // LLM JavaLine Aggregation: merged into return Ok() below
                // Java: catch (UnsupportedEncodingException e)
                // LLM Rust Equivalent: encoding always supported; catch unreachable
                // Java: throw new RuntimeException(e);
                // LLM Rust Equivalent: error propagated via Result return type
                return Ok(String::from_utf16_lossy(&chars));
            }
            // Java: else
            else {
                // Java: throw new IllegalArgumentException("Illegal length " + len);
                return Err(format!("Illegal length {}", len));
            }
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException("Illegal offset " + offset + " (String data is of length " + string.length + ")");
            return Err(format!("Illegal offset {} (String data is of length {})", offset, string.len()));
        }
    }

    // Java: static String getFromUnicodeLE(byte[] string)
    pub fn getFromUnicodeLE_bytes(string: &[u8]) -> Result<String, String> {
        // Java: return string.length == 0 ? "" : getFromUnicodeLE(string, 0, string.length / 2);
        if string.is_empty() { Ok(String::new()) } else { Self::getFromUnicodeLE(string, 0, string.len() / 2) }
    }

    // Java: static String getFromCompressedUnicode(byte[] string, int offset, int len)
    pub fn getFromCompressedUnicode(string: &[u8], offset: usize, len: usize) -> String {
        // Java: try
        // LLM Rust Equivalent: ISO-8859-1 decoding cannot fail in Rust; try/catch eliminated
        // Java: int len_to_use = Math.min(len, string.length - offset);
        let len_to_use = std::cmp::min(len, string.len() - offset);
        // Java: return new String(string, offset, len_to_use, "ISO-8859-1");
        let chars: Vec<char> = string[offset..offset + len_to_use].iter().map(|&b| b as char).collect();
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: ISO-8859-1 always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via Result return type
        return chars.into_iter().collect();
    }

    // Java: static String readCompressedUnicode(LittleEndianInput in, int nChars)
    pub fn readCompressedUnicode(input: &mut dyn LittleEndianInput, nChars: i32) -> String {
        // Java: char[] buf = new char[nChars];
        let mut buf = Vec::with_capacity(nChars as usize);
        // Java: for(int i = 0; i < buf.length; ++i)
        for _i in 0..nChars {
            // Java: buf[i] = (char)in.readUByte();
            buf.push(input.readUByte() as u8 as char);
        }
        // Java: return new String(buf);
        buf.into_iter().collect()
    }

    // Java: static String readUnicodeString(LittleEndianInput in)
    pub fn readUnicodeString(input: &mut dyn LittleEndianInput) -> String {
        // Java: int nChars = in.readUShort();
        let nChars = input.readUShort();
        // Java: if (nChars == 0 && in.available() == 0)
        if nChars == 0 && input.available() == 0 {
            // Java: return "";
            return String::new();
        }
        // Java: else
        else {
            // Java: byte flag = in.readByte();
            let flag = input.readByte();
            // Java: return (flag & 1) == 0 ? readCompressedUnicode(in, nChars) : readUnicodeLE(in, nChars);
            if (flag & 1) == 0 {
                Self::readCompressedUnicode(input, nChars)
            } else {
                Self::readUnicodeLE(input, nChars)
            }
        }
    }

    // Java: static String readUnicodeString(LittleEndianInput in, int nChars)
    pub fn readUnicodeString_nchars(input: &mut dyn LittleEndianInput, nChars: i32) -> String {
        // Java: byte is16Bit = in.readByte();
        let is16Bit = input.readByte();
        // Java: return (is16Bit & 1) == 0 ? readCompressedUnicode(in, nChars) : readUnicodeLE(in, nChars);
        if (is16Bit & 1) == 0 {
            Self::readCompressedUnicode(input, nChars)
        } else {
            Self::readUnicodeLE(input, nChars)
        }
    }

    // Java: static void writeNUnicodeString(LittleEndianOutput out, String value, int num)
    pub fn writeNUnicodeString(out: &mut dyn LittleEndianOutput, value: &str, num: i32) {
        // Java: if (num >= 1)
        if num >= 1 {
            // Java: int nChars = value.length();
            let nChars = value.len() as i32;
            // Java: if (num == 1)
            if num == 1 {
                // Java: out.writeByte(nChars);
                out.writeByte(nChars);
            }
            // Java: else if (num == 2)
            else if num == 2 {
                // Java: out.writeShort(nChars);
                out.writeShort(nChars);
            }
            // Java: else if (num == 3)
            else if num == 3 {
                // Java: out.writeShort(nChars);
                out.writeShort(nChars);
                // Java: boolean is16Bit = hasMultibyte(value);
                let is16Bit = Self::hasMultibyte(value);
                // Java: out.writeByte(is16Bit ? 1 : 0);
                out.writeByte(if is16Bit { 1 } else { 0 });
            }
            // Java: else
            else {
                // Java: out.writeShort(nChars);
                out.writeShort(nChars);
                // Java: boolean is16Bit = hasMultibyte(value);
                let is16Bit = Self::hasMultibyte(value);
                // Java: out.writeByte(is16Bit ? 1 : 0);
                out.writeByte(if is16Bit { 1 } else { 0 });
                // Java: if (is16Bit)
                if is16Bit {
                    // Java: putNUnicodeLE(value, out, num - 3);
                    Self::putNUnicodeLE(value, out, num - 3);
                }
                // Java: else
                else {
                    // Java: putCompressedNUnicode(value, out, num - 3);
                    Self::putCompressedNUnicode(value, out, num - 3);
                }
            }
        }
    }

    // Java: static void writeUnicodeString(LittleEndianOutput out, String value)
    pub fn writeUnicodeString(out: &mut dyn LittleEndianOutput, value: &str) {
        // Java: int nChars = value.length();
        let nChars = value.encode_utf16().count() as i32;
        // Java: out.writeShort(nChars);
        out.writeShort(nChars);
        // Java: boolean is16Bit = hasMultibyte(value);
        let is16Bit = Self::hasMultibyte(value);
        // Java: out.writeByte(is16Bit ? 1 : 0);
        out.writeByte(if is16Bit { 1 } else { 0 });
        // Java: if (is16Bit)
        if is16Bit {
            // Java: putUnicodeLE(value, out);
            Self::putUnicodeLE_out(value, out);
        }
        // Java: else
        else {
            // Java: putCompressedUnicode(value, out);
            Self::putCompressedUnicode_out(value, out);
        }
    }

    // Java: static void writeUnicodeStringFlagAndData(LittleEndianOutput out, String value)
    pub fn writeUnicodeStringFlagAndData(out: &mut dyn LittleEndianOutput, value: &str) {
        // Java: boolean is16Bit = hasMultibyte(value);
        let is16Bit = Self::hasMultibyte(value);
        // Java: out.writeByte(is16Bit ? 1 : 0);
        out.writeByte(if is16Bit { 1 } else { 0 });
        // Java: if (is16Bit)
        if is16Bit {
            // Java: putUnicodeLE(value, out);
            Self::putUnicodeLE_out(value, out);
        }
        // Java: else
        else {
            // Java: putCompressedUnicode(value, out);
            Self::putCompressedUnicode_out(value, out);
        }
    }

    // Java: static int getEncodedSize(String value)
    pub fn getEncodedSize(value: &str) -> i32 {
        // Java: int result = 3;
        let mut result = 3;
        // Java: result += value.length() * (hasMultibyte(value) ? 2 : 1);
        result += value.encode_utf16().count() as i32 * (if Self::hasMultibyte(value) { 2 } else { 1 });
        // Java: return result;
        result
    }

    // Java: static void putCompressedUnicode(String input, byte[] output, int offset)
    pub fn putCompressedUnicode(input: &str, output: &mut [u8], offset: usize) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: ISO-8859-1 encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("ISO-8859-1");
        let bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: ISO-8859-1 always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: System.arraycopy(bytes, 0, output, offset, bytes.length);
        // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
        output[offset..offset + bytes.len()].copy_from_slice(&bytes);
    }

    // Java: static void putCompressedNUnicode(String input, LittleEndianOutput out, int num)
    pub fn putCompressedNUnicode(input: &str, out: &mut dyn LittleEndianOutput, num: i32) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: ISO-8859-1 encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("ISO-8859-1");
        let bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: ISO-8859-1 always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: for(int i = 0; i < num; ++i)
        for i in 0..num as usize {
            // Java: out.writeByte(bytes[i]);
            out.writeByte(bytes[i] as i32);
        }
    }

    // Java: static void putCompressedUnicode(String input, LittleEndianOutput out)
    pub fn putCompressedUnicode_out(input: &str, out: &mut dyn LittleEndianOutput) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: ISO-8859-1 encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("ISO-8859-1");
        // rust: Android ISO-8859-1 replaces each unmappable UTF-16 unit;
        // a supplementary character therefore emits two '?' bytes.
        let bytes: Vec<u8> = input.encode_utf16().map(|c| if c<=255{c as u8}else{b'?'}).collect();
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: ISO-8859-1 always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: out.write(bytes);
        out.write(&bytes);
    }

    // Java: static void putUnicodeLE(String input, byte[] output, int offset)
    pub fn putUnicodeLE(input: &str, output: &mut [u8], offset: usize) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: UTF-16LE encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("UTF-16LE");
        let bytes: Vec<u8> = {
            let mut b = Vec::with_capacity(input.len() * 2);
            for ch in input.encode_utf16() {
                b.push((ch & 0xFF) as u8);
                b.push((ch >> 8) as u8);
            }
            b
        };
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: UTF-16LE always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: System.arraycopy(bytes, 0, output, offset, bytes.length);
        // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
        output[offset..offset + bytes.len()].copy_from_slice(&bytes);
    }

    // Java: static void putNUnicodeLE(String input, LittleEndianOutput out, int num)
    pub fn putNUnicodeLE(input: &str, out: &mut dyn LittleEndianOutput, num: i32) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: UTF-16LE encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("UTF-16LE");
        let bytes: Vec<u8> = {
            let mut b = Vec::with_capacity(input.len() * 2);
            for ch in input.encode_utf16() {
                b.push((ch & 0xFF) as u8);
                b.push((ch >> 8) as u8);
            }
            b
        };
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: UTF-16LE always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: for(int i = 0; i < num; ++i)
        for i in 0..num as usize {
            // Java: out.writeByte(bytes[i]);
            out.writeByte(bytes[i] as i32);
        }
    }

    // Java: static void putUnicodeLE(String input, LittleEndianOutput out)
    pub fn putUnicodeLE_out(input: &str, out: &mut dyn LittleEndianOutput) {
        // Java: byte[] bytes;
        // LLM JavaLine Aggregation: merged into let binding below
        // Java: try
        // LLM Rust Equivalent: UTF-16LE encoding cannot fail in Rust; try/catch eliminated
        // Java: bytes = input.getBytes("UTF-16LE");
        let bytes: Vec<u8> = {
            let mut b = Vec::with_capacity(input.len() * 2);
            for ch in input.encode_utf16() {
                b.push((ch & 0xFF) as u8);
                b.push((ch >> 8) as u8);
            }
            b
        };
        // Java: catch (UnsupportedEncodingException e)
        // LLM Rust Equivalent: UTF-16LE always supported; catch unreachable
        // Java: throw new RuntimeException(e);
        // LLM Rust Equivalent: error propagated via panic in Rust
        // Java: out.write(bytes);
        out.write(&bytes);
    }

    // Java: static String readUnicodeLE(LittleEndianInput in, int nChars)
    pub fn readUnicodeLE(input: &mut dyn LittleEndianInput, nChars: i32) -> String {
        // Java: char[] buf = new char[nChars];
        let mut buf = Vec::with_capacity(nChars as usize);
        // Java: for(int i = 0; i < buf.length; ++i)
        for _i in 0..nChars {
            // Java: buf[i] = (char)in.readUShort();
            buf.push(std::char::from_u32(input.readUShort() as u32).unwrap_or('\u{FFFD}'));
        }
        // Java: return new String(buf);
        buf.into_iter().collect()
    }

    // Java: static String format(String message, Object[] params)
    pub fn format(message: &str, params: &[String]) -> String {
        // Java: int currentParamNumber = 0;
        let mut currentParamNumber: usize = 0;
        // Java: StringBuffer formattedMessage = new StringBuffer();
        let mut formattedMessage = String::new();
        // Java: for(int i = 0; i < message.length(); ++i)
        let chars: Vec<char> = message.chars().collect();
        let mut i: usize = 0;
        while i < chars.len() {
            // Java: if (message.charAt(i) == '%')
            if chars[i] == '%' {
                // Java: if (currentParamNumber >= params.length)
                if currentParamNumber >= params.len() {
                    // Java: formattedMessage.append("?missing data?");
                    formattedMessage.push_str("?missing data?");
                }
                // Java: else if (params[currentParamNumber] instanceof Number && i + 1 < message.length())
                // LLM Rust Equivalent: params are all String in Rust; treating as non-Number
                else if i + 1 < chars.len() {
                    // Java: i += matchOptionalFormatting((Number)params[currentParamNumber++], message.substring(i + 1), formattedMessage);
                    formattedMessage.push_str(&params[currentParamNumber]);
                    currentParamNumber += 1;
                }
                // Java: else
                else {
                    // Java: formattedMessage.append(params[currentParamNumber++].toString());
                    formattedMessage.push_str(&params[currentParamNumber]);
                    currentParamNumber += 1;
                }
            }
            // Java: else if (message.charAt(i) == '\\' && i + 1 < message.length() && message.charAt(i + 1) == '%')
            else if chars[i] == '\\' && i + 1 < chars.len() && chars[i + 1] == '%' {
                // Java: formattedMessage.append('%');
                formattedMessage.push('%');
                // Java: ++i;
                i += 1;
            }
            // Java: else
            else {
                // Java: formattedMessage.append(message.charAt(i));
                formattedMessage.push(chars[i]);
            }
            i += 1;
        }
        // Java: return formattedMessage.toString();
        formattedMessage
    }

    // Java: static int matchOptionalFormatting(Number number, String formatting, StringBuffer outputTo)
    fn matchOptionalFormatting(number: f64, formatting: &str, outputTo: &mut String) -> usize {
        // Java: NumberFormat numberFormat = NumberFormat.getInstance();
        // LLM No Rust equivalent: NumberFormat not available; using format! macro
        let numberFormat = 0;
        let chars: Vec<char> = formatting.chars().collect();
        // Java: if (0 < formatting.length() && Character.isDigit(formatting.charAt(0)))
        if !chars.is_empty() && chars[0].is_ascii_digit() {
            // Java: numberFormat.setMinimumIntegerDigits(Integer.parseInt(formatting.charAt(0) + ""));
            // LLM Rust Equivalent: NumberFormat not available; using format! macro
            // Java: if (2 < formatting.length() && formatting.charAt(1) == '.' && Character.isDigit(formatting.charAt(2)))
            if chars.len() > 2 && chars[1] == '.' && chars[2].is_ascii_digit() {
                // Java: numberFormat.setMaximumFractionDigits(Integer.parseInt(formatting.charAt(2) + ""));
                // LLM Rust Equivalent: NumberFormat not available; using format! macro
                // Java: numberFormat.format(number, outputTo, new FieldPosition(0));
                outputTo.push_str(&format!("{}", number));
                // Java: return 3;
                return 3;
            }
            // Java: else
            else {
                // Java: numberFormat.format(number, outputTo, new FieldPosition(0));
                outputTo.push_str(&format!("{}", number));
                // Java: return 1;
                return 1;
            }
        }
        // Java: else if (0 < formatting.length() && formatting.charAt(0) == '.' && 1 < formatting.length() && Character.isDigit(formatting.charAt(1)))
        else if chars.len() > 1 && chars[0] == '.' && chars[1].is_ascii_digit() {
            // Java: numberFormat.setMaximumFractionDigits(Integer.parseInt(formatting.charAt(1) + ""));
            // LLM Rust Equivalent: NumberFormat not available; using format! macro
            // Java: numberFormat.format(number, outputTo, new FieldPosition(0));
            outputTo.push_str(&format!("{}", number));
            // Java: return 2;
            return 2;
        }
        // Java: else
        else {
            // Java: numberFormat.format(number, outputTo, new FieldPosition(0));
            outputTo.push_str(&format!("{}", number));
            // Java: return 1;
            return 1;
        }
    }

    // Java: static String getPreferredEncoding()
    pub fn getPreferredEncoding() -> &'static str {
        // Java: return "ISO-8859-1";
        "ISO-8859-1"
    }

    // Java: static boolean hasMultibyte(String value)
    pub fn hasMultibyte(value: &str) -> bool {
        // Java: if (value == null)
        // LLM Rust Null-Safety: &str cannot be null in Rust; condition always false
        if false {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < value.length(); ++i)
            for c in value.chars() {
                // Java: char c = value.charAt(i);
                // LLM JavaLine Aggregation: merged into for loop iterator
                // Java: if (c > 255)
                if c as u32 > 255 {
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            return false;
        }
    }

    // Java: static boolean isUnicodeString(String value)
    pub fn isUnicodeString(value: &str) -> bool {
        // Java: try
        // LLM Rust Equivalent: ISO-8859-1 encoding cannot fail in Rust; try/catch eliminated
        // Java: return !value.equals(new String(value.getBytes("ISO-8859-1"), "ISO-8859-1"));
        for c in value.chars() {
            if c as u32 > 255 {
                return true;
            }
        }
        // Java: catch (UnsupportedEncodingException var2)
        // LLM Rust Equivalent: ISO-8859-1 always supported; catch unreachable
        // Java: return true;
        // LLM Rust Equivalent: unreachable in Rust; encoding always supported
        return false;
    }
}

// Java: public class StringsIterator
impl StringsIterator {
    // Java: StringsIterator(String[] strings)
    pub fn new(strings: Option<Vec<String>>) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: no super() needed in Rust
        // Java: if (strings != null)
        if let Some(s) = strings {
            // Java: this.strings = strings;
            // LLM JavaLine Aggregation: merged into struct initialization
            Self { strings: s, position: 0 }
        }
        // Java: else
        else {
            // Java: this.strings = new String[0];
            // LLM JavaLine Aggregation: merged into struct initialization
            Self { strings: Vec::new(), position: 0 }
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.position < this.strings.length;
        self.position < self.strings.len()
    }

    // Java: String next()
    pub fn next(&mut self) -> &str {
        // Java: int ourPos = this.position++;
        let ourPos = self.position;
        self.position += 1;
        // Java: if (ourPos >= this.strings.length)
        if ourPos >= self.strings.len() {
            // Java: throw new ArrayIndexOutOfBoundsException(ourPos);
            panic!("ArrayIndexOutOfBoundsException: {}", ourPos);
        }
        // Java: else
        else {
            // Java: return this.strings[ourPos];
            &self.strings[ourPos]
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
    }
}
