#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_mut)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.BufferedInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.io.PrintStream
// Java: import java.text.DecimalFormat

use std::io::{self, Read, Write};

// Java field: String EOL
pub const EOL: &str = "\n";
// Java field: __hexcodes: &[char]
const HEXCODES: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
// Java field: __shifts: &[i32]
const SHIFTS: [i32; 16] = [60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0];

// Java: public class HexDump
pub struct HexDump;

// Java: public class HexDump
impl HexDump {
    // Java: HexDump()
    fn new() -> Self {
        // Java: super();
        HexDump
    }

    // Java: static void dump(_data: &[u8], long offset, OutputStream stream, int index, int length)
    pub fn dump_to_stream(data: &[i8], offset: i64, stream: &mut dyn Write, index: i32, length: i32) -> io::Result<()> {
        // Java: if (data.length == 0)
        if data.len() == 0 {
            // Java: stream.write(("No Data" + System.getProperty("line.separator")).getBytes());
            stream.write_all(format!("No Data{}", EOL).as_bytes())?;
            // Java: stream.flush();
            stream.flush()?;
        // Java: else if (index >= 0 && index < data.length)
        } else if index >= 0 && (index as usize) < data.len() {
            // Java: if (stream == null)
            // Java: throw new IllegalArgumentException("cannot write to nullstream");
            // (stream is always valid in Rust)
            // Java: else
            {
                // Java: long display_offset = offset + (long)index;
                let mut display_offset = offset + index as i64;
                // Java: StringBuffer buffer = new StringBuffer(74);
                let mut buffer = String::with_capacity(74);
                // Java: int data_length = Math.min(data.length, index + length);
                let data_length = std::cmp::min(data.len() as i32, index + length);
                // Java: for(int j = index; j < data_length; j += 16)
                let mut j = index;
                while j < data_length {
                    // Java: int chars_read = data_length - j;
                    let mut chars_read = data_length - j;
                    // Java: if (chars_read > 16)
                    if chars_read > 16 {
                        // Java: chars_read = 16;
                        chars_read = 16;
                    }
                    // Java: buffer.append(dump(display_offset)).append(' ');
                    buffer.push_str(&Self::dump_long(display_offset));
                    buffer.push(' ');
                    // Java: for(int k = 0; k < 16; ++k)
                    for k in 0..16 {
                        // Java: if (k < chars_read)
                        if k < chars_read {
                            // Java: buffer.append(dump(data[k + j]));
                            buffer.push_str(&Self::dump_byte(data[(k + j) as usize]));
                        // Java: else
                        } else {
                            // Java: buffer.append("  ");
                            buffer.push_str("  ");
                        }
                        // Java: buffer.append(' ');
                        buffer.push(' ');
                    }
                    // Java: for(int k = 0; k < chars_read; ++k)
                    for k in 0..chars_read {
                        // Java: if (data[k + j] >= 32 && data[k + j] < 127)
                        if data[(k + j) as usize] >= 32 && data[(k + j) as usize] < 127 {
                            // Java: buffer.append((char)data[k + j]);
                            buffer.push(data[(k + j) as usize] as u8 as char);
                        // Java: else
                        } else {
                            // Java: buffer.append('.');
                            buffer.push('.');
                        }
                    }
                    // Java: buffer.append(EOL);
                    buffer.push_str(EOL);
                    // Java: stream.write(buffer.toString().getBytes());
                    stream.write_all(buffer.as_bytes())?;
                    // Java: stream.flush();
                    stream.flush()?;
                    // Java: buffer.setLength(0);
                    buffer.clear();
                    // Java: display_offset += (long)chars_read;
                    display_offset += chars_read as i64;
                    j += 16;
                }
            }
        // Java: else
        } else {
            // Java: throw new ArrayIndexOutOfBoundsException("illegal index: " + index + " into array of length " + data.length);
            panic!("illegal index: {} into array of length {}", index, data.len());
        }
        Ok(())
    }

    // Java: static void dump(_data: &[u8], long offset, OutputStream stream, int index)
    pub fn dump_to_stream_default(data: &[i8], offset: i64, stream: &mut dyn Write, index: i32) -> io::Result<()> {
        // Java: dump(data, offset, stream, index, data.length - index);
        Self::dump_to_stream(data, offset, stream, index, data.len() as i32 - index)
    }

    // Java: static String dump(_data: &[u8], long offset, int index)
    pub fn dump_to_string(data: &[i8], offset: i64, index: i32) -> String {
        // Java: if (index >= 0 && index < data.length)
        if index >= 0 && (index as usize) < data.len() {
            // Java: long display_offset = offset + (long)index;
            let mut display_offset = offset + index as i64;
            // Java: StringBuffer buffer = new StringBuffer(74);
            let mut buffer = String::with_capacity(74);
            // Java: for(int j = index; j < data.length; j += 16)
            let mut j = index;
            while j < data.len() as i32 {
                // Java: int chars_read = data.length - j;
                let mut chars_read = data.len() as i32 - j;
                // Java: if (chars_read > 16)
                if chars_read > 16 {
                    // Java: chars_read = 16;
                    chars_read = 16;
                }
                // Java: buffer.append(dump(display_offset)).append(' ');
                buffer.push_str(&Self::dump_long(display_offset));
                buffer.push(' ');
                // Java: for(int k = 0; k < 16; ++k)
                for k in 0..16 {
                    // Java: if (k < chars_read)
                    if k < chars_read {
                        // Java: buffer.append(dump(data[k + j]));
                        buffer.push_str(&Self::dump_byte(data[(k + j) as usize]));
                    // Java: else
                    } else {
                        // Java: buffer.append("  ");
                        buffer.push_str("  ");
                    }
                    // Java: buffer.append(' ');
                    buffer.push(' ');
                }
                // Java: for(int k = 0; k < chars_read; ++k)
                for k in 0..chars_read {
                    // Java: if (data[k + j] >= 32 && data[k + j] < 127)
                    if data[(k + j) as usize] >= 32 && data[(k + j) as usize] < 127 {
                        // Java: buffer.append((char)data[k + j]);
                        buffer.push(data[(k + j) as usize] as u8 as char);
                    // Java: else
                    } else {
                        // Java: buffer.append('.');
                        buffer.push('.');
                    }
                }
                // Java: buffer.append(EOL);
                buffer.push_str(EOL);
                // Java: display_offset += (long)chars_read;
                display_offset += chars_read as i64;
                j += 16;
            }
            // Java: return buffer.toString();
            buffer
        // Java: else
        } else {
            // Java: throw new ArrayIndexOutOfBoundsException("illegal index: " + index + " into array of length " + data.length);
            panic!("illegal index: {} into array of length {}", index, data.len());
        }
    }

    // Java: static String dump(long value)
    fn dump_long(value: i64) -> String {
        // Java: StringBuffer buf = new StringBuffer();
        let mut buf = String::new();
        // Java: buf.setLength(0);
        buf.clear();
        // Java: for(int j = 0; j < 8; ++j)
        for j in 0..8 {
            // Java: buf.append(_hexcodes[(int)(value >> _shifts[j + _shifts.length - 8]) & 15]);
            let shift = SHIFTS[j + SHIFTS.len() - 8];
            let idx = ((value as u64 >> shift as u64) & 15) as usize;
            buf.push(HEXCODES[idx]);
        }
        // Java: return buf.toString();
        buf
    }

    // Java: static String dump(byte value)
    fn dump_byte(value: i8) -> String {
        // Java: StringBuffer buf = new StringBuffer();
        let mut buf = String::new();
        // Java: buf.setLength(0);
        buf.clear();
        // Java: for(int j = 0; j < 2; ++j)
        for j in 0..2 {
            // Java: buf.append(_hexcodes[value >> _shifts[j + 6] & 15]);
            let shift = SHIFTS[j + 6];
            let idx = ((value as i32 >> shift) & 15) as usize;
            buf.push(HEXCODES[idx]);
        }
        // Java: return buf.toString();
        buf
    }

    // Java: static String toHex(_value: &[u8])
    pub fn toHex_bytes(value: &[i8]) -> String {
        // Java: StringBuffer retVal = new StringBuffer();
        let mut ret_val = String::new();
        // Java: retVal.append('[');
        ret_val.push('[');
        // Java: for(int x = 0; x < value.length; ++x)
        for x in 0..value.len() {
            // Java: if (x > 0)
            if x > 0 {
                // Java: retVal.append(", ");
                ret_val.push_str(", ");
            }
            // Java: retVal.append(toHex(value[x]));
            ret_val.push_str(&Self::toHex_byte(value[x]));
        }
        // Java: retVal.append(']');
        ret_val.push(']');
        // Java: return retVal.toString();
        ret_val
    }

    // Java: static String toHex(short[] value)
    pub fn toHex_shorts(value: &[i16]) -> String {
        // Java: StringBuffer retVal = new StringBuffer();
        let mut ret_val = String::new();
        // Java: retVal.append('[');
        ret_val.push('[');
        // Java: for(int x = 0; x < value.length; ++x)
        for x in 0..value.len() {
            // Java: retVal.append(toHex(value[x]));
            ret_val.push_str(&Self::toHex_short(value[x]));
            // Java: retVal.append(", ");
            ret_val.push_str(", ");
        }
        // Java: retVal.append(']');
        ret_val.push(']');
        // Java: return retVal.toString();
        ret_val
    }

    // Java: static String toHex(_value: &[u8], int bytesPerLine)
    pub fn toHex_bytes_per_line(value: &[i8], bytes_per_line: i32) -> String {
        // Java: int digits = (int)Math.round(Math.log((double)value.length) / Math.log((double)10.0F) + (double)0.5F);
        let digits = ((value.len() as f64).ln() / (10.0f64).ln() + 0.5).round() as usize;
        // Java: StringBuffer formatString = new StringBuffer();
        let mut format_string = String::new();
        // Java: for(int i = 0; i < digits; ++i)
        for _ in 0..digits {
            // Java: formatString.append('0');
            format_string.push('0');
        }
        // Java: formatString.append(": ");
        format_string.push_str(": ");
        // Java: DecimalFormat format = new DecimalFormat(formatString.toString());
        let format_digits = digits;
        // Java: StringBuffer retVal = new StringBuffer();
        let mut ret_val = String::new();
        // Java: retVal.append(format.format(0L));
        ret_val.push_str(&format!("{:0>width$}: ", 0, width = format_digits));
        // Java: int i = -1;
        let mut i: i32 = -1;
        // Java: for(int x = 0; x < value.length; ++x)
        for x in 0..value.len() {
            // Java: ++i;
            i += 1;
            // Java: if (i == bytesPerLine)
            if i == bytes_per_line {
                // Java: retVal.append('\n');
                ret_val.push('\n');
                // Java: retVal.append(format.format((long)x));
                ret_val.push_str(&format!("{:0>width$}: ", x, width = format_digits));
                // Java: i = 0;
                i = 0;
            }
            // Java: retVal.append(toHex(value[x]));
            ret_val.push_str(&Self::toHex_byte(value[x]));
            // Java: retVal.append(", ");
            ret_val.push_str(", ");
        }
        // Java: return retVal.toString();
        ret_val
    }

    // Java: static String toHex(short value)
    pub fn toHex_short(value: i16) -> String {
        // Java: return toHex((long)value, 4);
        Self::toHex_long_digits(value as i64, 4)
    }

    // Java: static String toHex(byte value)
    pub fn toHex_byte(value: i8) -> String {
        // Java: return toHex((long)value, 2);
        Self::toHex_long_digits(value as i64, 2)
    }

    // Java: static String toHex(int value)
    pub fn toHex_int(value: i32) -> String {
        // Java: return toHex((long)value, 8);
        Self::toHex_long_digits(value as i64, 8)
    }

    // Java: static String toHex(long value)
    pub fn toHex_long(value: i64) -> String {
        // Java: return toHex(value, 16);
        Self::toHex_long_digits(value, 16)
    }

    // Java: static String toHex(long value, int digits)
    fn toHex_long_digits(value: i64, digits: i32) -> String {
        // Java: StringBuffer result = new StringBuffer(digits);
        let mut result = String::with_capacity(digits as usize);
        // Java: for(int j = 0; j < digits; ++j)
        for j in 0..digits as usize {
            // Java: result.append(_hexcodes[(int)(value >> _shifts[j + (16 - digits)] & 15L)]);
            let shift = SHIFTS[j + (16 - digits as usize)];
            let idx = ((value as u64 >> shift as u64) & 15) as usize;
            result.push(HEXCODES[idx]);
        }
        // Java: return result.toString();
        result
    }

    // Java: static void dump(InputStream in, PrintStream out, int start, int bytesToDump)
    pub fn dump_input_stream(input: &mut dyn Read, out: &mut dyn Write, start: i32, bytes_to_dump: i32) -> io::Result<()> {
        // Java: ByteArrayOutputStream buf = new ByteArrayOutputStream();
        let mut buf: Vec<u8> = Vec::new();
        // Java: if (bytesToDump == -1)
        if bytes_to_dump == -1 {
            // Java: for(int c = in.read(); c != -1; c = in.read())
            let mut byte_buf = [0u8; 1];
            loop {
                let n = input.read(&mut byte_buf)?;
                if n == 0 { break; }
                // Java: buf.write(c);
                buf.push(byte_buf[0]);
            }
        // Java: else
        } else {
            // Java: int bytesRemaining = bytesToDump;
            let mut bytes_remaining = bytes_to_dump;
            // Java: while(bytesRemaining-- > 0)
            while { let old = bytes_remaining; bytes_remaining -= 1; old > 0 } {
                // Java: int c = in.read();
                let mut byte_buf = [0u8; 1];
                let n = input.read(&mut byte_buf)?;
                // Java: if (c == -1)
                if n == 0 {
                    // Java: break;
                    break;
                }
                // Java: buf.write(c);
                buf.push(byte_buf[0]);
            }
        }
        // Java: _data: &[u8] = buf.toByteArray();
        let data: Vec<i8> = buf.iter().map(|&b| b as i8).collect();
        // Java: dump(data, 0L, out, start, data.length);
        Self::dump_to_stream(&data, 0, out, start, data.len() as i32)
    }

    // Java: static _toHexChars: &[char](long pValue, int nBytes)
    pub fn toHexChars(p_value: i64, n_bytes: i32) -> Vec<char> {
        // Java: int charPos = 2 + nBytes * 2;
        let mut char_pos = (2 + n_bytes * 2) as usize;
        // Java: _result: &[char] = new char[charPos];
        let mut result = vec!['\0'; char_pos];
        // Java: long value = pValue;
        let mut value = p_value as u64;
        // Java: do
        loop {
            // Java: --charPos;
            char_pos -= 1;
            // Java: result[charPos] = _hexcodes[(int)(value & 15L)];
            result[char_pos] = HEXCODES[(value & 15) as usize];
            // Java: value >>>= 4;
            value >>= 4;
            if char_pos <= 1 { break; }
        }
        // Java: result[0] = '0';
        result[0] = '0';
        // Java: result[1] = 'x';
        result[1] = 'x';
        // Java: return result;
        result
    }

    // Java: static _longToHex: &[char](long value)
    pub fn longToHex(value: i64) -> Vec<char> {
        // Java: return toHexChars(value, 8);
        Self::toHexChars(value, 8)
    }

    // Java: static _intToHex: &[char](int value)
    pub fn intToHex(value: i32) -> Vec<char> {
        // Java: return toHexChars((long)value, 4);
        Self::toHexChars(value as i64, 4)
    }

    // Java: static _shortToHex: &[char](int value)
    pub fn shortToHex(value: i32) -> Vec<char> {
        // Java: return toHexChars((long)value, 2);
        Self::toHexChars(value as i64, 2)
    }

    // Java: static _byteToHex: &[char](int value)
    pub fn byteToHex(value: i32) -> Vec<char> {
        // Java: return toHexChars((long)value, 1);
        Self::toHexChars(value as i64, 1)
    }

    // Java: static void main(_args: &[String])
    pub fn main(args: &[String]) {
        // Java: File file = new File(args[0]);
        // Java: InputStream in = new BufferedInputStream(new FileInputStream(file));
        // Java: _b: &[u8] = new byte[(int)file.length()];
        // Java: in.read(b);
        // Java: System.out.println(dump(b, 0L, 0));
        // Java: in.close();
        todo!()
    }
}
