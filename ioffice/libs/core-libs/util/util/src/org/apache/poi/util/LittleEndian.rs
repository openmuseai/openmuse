#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_mut)]

use std::io::{self, Read};
use super::LittleEndianConsts::LittleEndianConsts;

// Java: public class LittleEndian implements LittleEndianConsts
pub struct LittleEndian;

// Java: public class BufferUnderrunException
pub struct BufferUnderrunException;

// Java: public class BufferUnderrunException
impl BufferUnderrunException {
    // Java: BufferUnderrunException()
    pub fn new() -> io::Error {
        // Java: super("buffer underrun");
        io::Error::new(io::ErrorKind::UnexpectedEof, "buffer underrun")
    }
}

// Java: public class LittleEndian implements LittleEndianConsts
impl LittleEndian {
    // Java: LittleEndian()
    fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static byte getByte(_data: &[u8], int offset)
    pub fn getByte(data: &[i8], offset: i32) -> i8 {
        // Java: return (byte)(data[offset] & 255);
        data[offset as usize]
    }

    // Java: static short getUByte(_data: &[u8], int offset)
    pub fn getUByte(data: &[i8], offset: i32) -> i16 {
        // Java: return (short)(data[offset] & 255);
        (data[offset as usize] as i16) & 255
    }

    // Java: static short getShort(_data: &[u8], int offset)
    pub fn getShort(data: &[i8], offset: i32) -> i16 {
        // Java: int b0 = data[offset] & 255;
        let b0 = (data[offset as usize] as i32) & 255;
        // Java: int b1 = data[offset + 1] & 255;
        let b1 = (data[(offset + 1) as usize] as i32) & 255;
        // Java: return (short)((b1 << 8) + (b0 << 0));
        ((b1 << 8) + (b0 << 0)) as i16
    }

    // Java: static int getUShort(_data: &[u8], int offset)
    pub fn getUShort(data: &[i8], offset: i32) -> i32 {
        // Java: int b0 = data[offset] & 255;
        let b0 = (data[offset as usize] as i32) & 255;
        // Java: int b1 = data[offset + 1] & 255;
        let b1 = (data[(offset + 1) as usize] as i32) & 255;
        // Java: return (b1 << 8) + (b0 << 0);
        (b1 << 8) + (b0 << 0)
    }

    // Java: static short getShort(_data: &[u8])
    pub fn getShort_nooffset(data: &[i8]) -> i16 {
        // Java: return getShort(data, 0);
        Self::getShort(data, 0)
    }

    // Java: static int getUShort(_data: &[u8])
    pub fn getUShort_nooffset(data: &[i8]) -> i32 {
        // Java: return getUShort(data, 0);
        Self::getUShort(data, 0)
    }

    // Java: static int getInt(_data: &[u8], int offset)
    pub fn getInt(data: &[i8], offset: i32) -> i32 {
        // Java: int b0 = data[offset++] & 255;
        let mut offset = offset as usize;
        let b0 = (data[offset] as i32) & 255; offset += 1;
        // Java: int b1 = data[offset++] & 255;
        let b1 = (data[offset] as i32) & 255; offset += 1;
        // Java: int b2 = data[offset++] & 255;
        let b2 = (data[offset] as i32) & 255; offset += 1;
        // Java: int b3 = data[offset++] & 255;
        let b3 = (data[offset] as i32) & 255;
        // Java: return (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0)
    }

    // Java: static int getInt(_data: &[u8])
    pub fn getInt_nooffset(data: &[i8]) -> i32 {
        // Java: return getInt(data, 0);
        Self::getInt(data, 0)
    }

    // Java: static long getUInt(_data: &[u8], int offset)
    pub fn getUInt(data: &[i8], offset: i32) -> i64 {
        // Java: long retNum = (long)getInt(data, offset);
        let ret_num = Self::getInt(data, offset) as i64;
        // Java: return retNum & -1L;
        ret_num & 0xFFFFFFFF_i64
    }

    // Java: static long getUInt(_data: &[u8])
    pub fn getUInt_nooffset(data: &[i8]) -> i64 {
        // Java: return getUInt(data, 0);
        Self::getUInt(data, 0)
    }

    // Java: static long getLong(_data: &[u8], int offset)
    pub fn getLong(data: &[i8], offset: i32) -> i64 {
        let mut offset = offset as usize;
        // Java: long b0 = (long)(data[offset++] & 255);
        let b0 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b1 = (long)(data[offset++] & 255);
        let b1 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b2 = (long)(data[offset++] & 255);
        let b2 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b3 = (long)(data[offset++] & 255);
        let b3 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b4 = (long)(data[offset++] & 255);
        let b4 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b5 = (long)(data[offset++] & 255);
        let b5 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b6 = (long)(data[offset++] & 255);
        let b6 = (data[offset] as i64) & 255; offset += 1;
        // Java: long b7 = (long)(data[offset++] & 255);
        let b7 = (data[offset] as i64) & 255;
        // Java: return (b7 << 56) + (b6 << 48) + (b5 << 40) + (b4 << 32) + (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b7 << 56) + (b6 << 48) + (b5 << 40) + (b4 << 32) + (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0)
    }

    // Java: static double getDouble(_data: &[u8], int offset)
    pub fn getDouble(data: &[i8], offset: i32) -> f64 {
        // Java: return Double.longBitsToDouble(getLong(data, offset));
        f64::from_bits(Self::getLong(data, offset) as u64)
    }

    // Java: static void putShort(_data: &[u8], int offset, short value)
    pub fn putShort(data: &mut [i8], offset: i32, value: i16) {
        // Java: int i = offset + 1;
        let mut i = (offset + 1) as usize;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset as usize] = ((value as u16 >> 0) & 255) as i8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[i] = ((value as u16 >> 8) & 255) as i8; i += 1;
    }

    // Java: static void putByte(_data: &[u8], int offset, int value)
    pub fn putByte(data: &mut [i8], offset: i32, value: i32) {
        // Java: data[offset] = (byte)value;
        data[offset as usize] = value as i8;
    }

    // Java: static void putUShort(_data: &[u8], int offset, int value)
    pub fn putUShort(data: &mut [i8], offset: i32, value: i32) {
        // Java: int i = offset + 1;
        let mut i = (offset + 1) as usize;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset as usize] = ((value as u32 >> 0) & 255) as i8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[i] = ((value as u32 >> 8) & 255) as i8; i += 1;
    }

    // Java: static void putShort(_data: &[u8], short value)
    pub fn putShort_nooffset(data: &mut [i8], value: i16) {
        // Java: putShort(data, 0, value);
        Self::putShort(data, 0, value);
    }

    // Java: static void putInt(_data: &[u8], int offset, int value)
    pub fn putInt(data: &mut [i8], offset: i32, value: i32) {
        // Java: int i = offset + 1;
        let mut i = (offset + 1) as usize;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset as usize] = ((value as u32 >> 0) & 255) as i8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[i] = ((value as u32 >> 8) & 255) as i8; i += 1;
        // Java: data[i++] = (byte)(value >>> 16 & 255);
        data[i] = ((value as u32 >> 16) & 255) as i8; i += 1;
        // Java: data[i++] = (byte)(value >>> 24 & 255);
        data[i] = ((value as u32 >> 24) & 255) as i8; i += 1;
    }

    // Java: static void putInt(_data: &[u8], int value)
    pub fn putInt_nooffset(data: &mut [i8], value: i32) {
        // Java: putInt(data, 0, value);
        Self::putInt(data, 0, value);
    }

    // Java: static void putLong(_data: &[u8], int offset, long value)
    pub fn putLong(data: &mut [i8], offset: i32, value: i64) {
        // Java: int limit = 8 + offset;
        let limit = (8 + offset) as usize;
        // Java: long v = value;
        let mut v = value;
        // Java: for(int j = offset; j < limit; ++j)
        for j in (offset as usize)..limit {
            // Java: data[j] = (byte)((int)(v & 255L));
            data[j] = (v & 255) as i8;
            // Java: v >>= 8;
            v >>= 8;
        }
    }

    // Java: static void putDouble(_data: &[u8], int offset, double value)
    pub fn putDouble(data: &mut [i8], offset: i32, value: f64) {
        // Java: putLong(data, offset, Double.doubleToLongBits(value));
        Self::putLong(data, offset, value.to_bits() as i64);
    }

    // Java: static short readShort(InputStream stream)
    pub fn readShort(stream: &mut dyn Read) -> io::Result<i16> {
        // Java: return (short)readUShort(stream);
        Ok(Self::readUShort(stream)? as i16)
    }

    // Java: static int readUShort(InputStream stream)
    pub fn readUShort(stream: &mut dyn Read) -> io::Result<i32> {
        // Java: int ch1 = stream.read();
        let mut buf = [0u8; 1];
        let ch1 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch2 = stream.read();
        let ch2 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: if ((ch1 | ch2) < 0)
        if (ch1 | ch2) < 0 {
            // Java: throw new BufferUnderrunException();
            return Err(BufferUnderrunException::new());
        // Java: else
        } else {
            // Java: return (ch2 << 8) + (ch1 << 0);
            Ok((ch2 << 8) + (ch1 << 0))
        }
    }

    // Java: static int readInt(InputStream stream)
    pub fn readInt(stream: &mut dyn Read) -> io::Result<i32> {
        // Java: int ch1 = stream.read();
        let mut buf = [0u8; 1];
        let ch1 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch2 = stream.read();
        let ch2 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch3 = stream.read();
        let ch3 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch4 = stream.read();
        let ch4 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: if ((ch1 | ch2 | ch3 | ch4) < 0)
        if (ch1 | ch2 | ch3 | ch4) < 0 {
            // Java: throw new BufferUnderrunException();
            return Err(BufferUnderrunException::new());
        // Java: else
        } else {
            // Java: return (ch4 << 24) + (ch3 << 16) + (ch2 << 8) + (ch1 << 0);
            Ok((ch4 << 24) + (ch3 << 16) + (ch2 << 8) + (ch1 << 0))
        }
    }

    // Java: static long readLong(InputStream stream)
    pub fn readLong(stream: &mut dyn Read) -> io::Result<i64> {
        // Java: int ch1 = stream.read();
        let mut buf = [0u8; 1];
        let ch1 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch2 = stream.read();
        let ch2 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch3 = stream.read();
        let ch3 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch4 = stream.read();
        let ch4 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch5 = stream.read();
        let ch5 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch6 = stream.read();
        let ch6 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch7 = stream.read();
        let ch7 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: int ch8 = stream.read();
        let ch8 = if stream.read(&mut buf)? > 0 { buf[0] as i32 } else { -1 };
        // Java: if ((ch1 | ch2 | ch3 | ch4 | ch5 | ch6 | ch7 | ch8) < 0)
        if (ch1 | ch2 | ch3 | ch4 | ch5 | ch6 | ch7 | ch8) < 0 {
            // Java: throw new BufferUnderrunException();
            return Err(BufferUnderrunException::new());
        // Java: else
        } else {
            // Java: return ((long)ch8 << 56) + ((long)ch7 << 48) + ((long)ch6 << 40) + ((long)ch5 << 32) + ((long)ch4 << 24) + (long)(ch3 << 16) + (long)(ch2 << 8) + (long)(ch1 << 0);
            Ok(((ch8 as i64) << 56) + ((ch7 as i64) << 48) + ((ch6 as i64) << 40) + ((ch5 as i64) << 32) + ((ch4 as i64) << 24) + ((ch3 << 16) as i64) + ((ch2 << 8) as i64) + ((ch1 << 0) as i64))
        }
    }

    // Java: static int ubyteToInt(byte b)
    pub fn ubyteToInt(b: i8) -> i32 {
        // Java: return b & 255;
        (b as i32) & 255
    }

    // Java: static int getUnsignedByte(_data: &[u8], int offset)
    pub fn getUnsignedByte(data: &[i8], offset: i32) -> i32 {
        // Java: return data[offset] & 255;
        (data[offset as usize] as i32) & 255
    }

    // Java: static _getByteArray: &[u8](_data: &[u8], int offset, int size)
    pub fn getByteArray(data: &[i8], offset: i32, size: i32) -> Vec<i8> {
        // Java: _copy: &[u8] = new byte[size];
        let mut copy = vec![0i8; size as usize];
        // Java: if (offset + size > data.length)
        if (offset + size) as usize > data.len() {
            // Java: System.arraycopy(data, offset, copy, 0, data.length - offset);
            let len = data.len() - offset as usize;
            copy[..len].copy_from_slice(&data[offset as usize..]);
        // Java: else
        } else {
            // Java: System.arraycopy(data, offset, copy, 0, size);
            copy.copy_from_slice(&data[offset as usize..(offset + size) as usize]);
        }
        // Java: return copy;
        copy
    }

    // Java: static boolean isLegalRange_Int(int dataLength, int offset)
    pub fn isLegalRange_Int(data_length: i32, offset: i32) -> bool {
        // Java: return dataLength >= offset + 4;
        data_length >= offset + 4
    }
}

impl LittleEndianConsts for LittleEndian {}
