// Java: import cn.wps.base.assertion.KSAssert
// Java: import java.util.ArrayList
use super::BytesReader::BytesReader;

// Java: public class Multibytecoding
pub struct Multibytecoding;

// Java: public class Multibytecoding
impl Multibytecoding {
    // Java: Multibytecoding()
    pub fn Multibytecoding() -> Self {
        // Java: super();
        Multibytecoding
    }

    // Java: static long decodeUInt(BytesReader reader)
    pub fn decodeUInt(reader: &mut BytesReader) -> i64 {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: byte count = 0;
        let mut count: i32 = 0;
        // Java: long value = 0L;
        let mut value: i64 = 0;
        // Java: byte flag;
        let mut flag: i32;
        // Java: do
        loop {
            // Java: byte bb = (byte)reader.getByte();
            let bb = reader.getByte() as i8;
            // Java: flag = (byte)(bb & 128);
            flag = (bb as i32) & 128;
            // Java: value |= (long)((bb & 127) << count);
            value |= ((bb as i64 & 127) << count) as i64;
            // Java: count = (byte)(count + 7);
            count += 7;
            if flag == 0 {
                break;
            }
        }
        // Java: return value;
        value
    }

    // Java: static byte[] encodeInt(long value)
    pub fn encodeInt(mut value: i64) -> Vec<u8> {
        // Java: boolean isNegative = value < 0L;
        let is_negative = value < 0;
        // Java: if (isNegative)
        if is_negative {
            // Java: value *= -1L;
            value *= -1;
        }
        // Java: value <<= 1;
        value <<= 1;
        // Java: ArrayList<Byte> result = _encodeUInt(value);
        let mut result = Self::_encodeUInt(value);
        // Java: if (isNegative)
        if is_negative {
            // Java: result.add(0, (byte)((Byte)result.get(0) | 1));
            result[0] |= 1;
        }
        // Java: byte[] ret = new byte[result.size()];
        // Java: for(int i = 0; i < result.size(); ++i)
        // Java: ret[i] = (Byte)result.get(i);
        // Java: return ret;
        result
    }

    // Java: static ArrayList<Byte> _encodeUInt(long value)
    fn _encodeUInt(mut value: i64) -> Vec<u8> {
        // Java: ArrayList<Byte> result = new ArrayList();
        let mut result: Vec<u8> = Vec::new();
        // Java: byte flag;
        let mut flag: u8;
        // Java: do
        loop {
            // Java: byte bb = (byte)((int)(value & 127L));
            let bb = (value & 127) as u8;
            // Java: value >>= 7;
            value >>= 7;
            // Java: flag = (byte)(0L == value ? 0 : 128);
            flag = if value == 0 { 0 } else { 128 };
            // Java: result.add((byte)(bb | flag));
            result.push(bb | flag);
            if flag == 0 {
                break;
            }
        }
        // Java: return result;
        result
    }

    // Java: static byte[] encodeUInt(long value)
    pub fn encodeUInt(value: i64) -> Vec<u8> {
        // Java: ArrayList<Byte> result = _encodeUInt(value);
        let result = Self::_encodeUInt(value);
        // Java: byte[] ret = new byte[result.size()];
        // Java: for(int i = 0; i < result.size(); ++i)
        // Java: ret[i] = (Byte)result.get(i);
        // Java: return ret;
        result
    }

    // Java: static long decodeInt(BytesReader reader)
    pub fn decodeInt(reader: &mut BytesReader) -> i64 {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: boolean isNegative = false;
        let mut is_negative = false;
        // Java: long value = decodeUInt(reader);
        let mut value = Self::decodeUInt(reader);
        // Java: if (0L != (value & 1L))
        if (value & 1) != 0 {
            // Java: isNegative = true;
            is_negative = true;
        }
        // Java: value >>= 1;
        value >>= 1;
        // Java: if (isNegative)
        if is_negative {
            // Java: value *= -1L;
            value *= -1;
        }
        // Java: return value;
        value
    }

    // Java: static float decodeFloat(BytesReader reader)
    pub fn decodeFloat(reader: &mut BytesReader) -> f32 {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: int index = 0;
        // Java: byte[] data = new byte[4];
        let mut data = [0u8; 4];
        // Java: index = 0;
        let mut index = 0;
        // Java: do
        loop {
            // Java: data[index++] = (byte)reader.getByte();
            data[index] = reader.getByte() as u8;
            index += 1;
            if index > 3 {
                break;
            }
        }
        // Java: return byte2float(data, 0);
        Self::byte2float(&data, 0)
    }

    // Java: static byte[] encodeFloat(float value)
    pub fn encodeFloat(value: f32) -> Vec<u8> {
        // Java: return float2byte(value);
        Self::float2byte(value)
    }

    // Java: static float byte2float(byte[] b, int index)
    fn byte2float(b: &[u8], index: usize) -> f32 {
        // Java: int l = b[index + 0];
        let mut l = b[index] as i32;
        // Java: l &= 255;
        l &= 255;
        // Java: l = (int)((long)l | (long)b[index + 1] << 8);
        l = (l as i64 | (b[index + 1] as i64) << 8) as i32;
        // Java: l &= 65535;
        l &= 65535;
        // Java: l = (int)((long)l | (long)b[index + 2] << 16);
        l = (l as i64 | (b[index + 2] as i64) << 16) as i32;
        // Java: l &= 16777215;
        l &= 16777215;
        // Java: l = (int)((long)l | (long)b[index + 3] << 24);
        l = (l as i64 | (b[index + 3] as i64) << 24) as i32;
        // Java: return Float.intBitsToFloat(l);
        f32::from_bits(l as u32)
    }

    // Java: static byte[] float2byte(float f)
    fn float2byte(f: f32) -> Vec<u8> {
        // Java: int fbit = Float.floatToIntBits(f);
        let fbit = f.to_bits() as i32;
        // Java: byte[] b = new byte[4];
        let mut b = [0u8; 4];
        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..4 {
            // Java: b[i] = (byte)(fbit >> 24 - i * 8);
            b[i] = (fbit >> (24 - i * 8)) as u8;
        }
        // Java: int len = b.length;
        let len = b.len();
        // Java: byte[] dest = new byte[len];
        // Java: System.arraycopy(b, 0, dest, 0, len);
        let mut dest = b.to_vec();
        // Java: for(int i = 0; i < len / 2; ++i)
        for i in 0..len / 2 {
            // Java: byte temp = dest[i];
            // Java: dest[i] = dest[len - i - 1];
            // Java: dest[len - i - 1] = temp;
            dest.swap(i, len - i - 1);
        }
        // Java: return dest;
        dest
    }
}
