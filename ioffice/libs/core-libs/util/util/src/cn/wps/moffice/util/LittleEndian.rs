// Java: public class LittleEndian
pub struct LittleEndian;

// Java: public class LittleEndian
impl LittleEndian {
    // Java: LittleEndian()
    pub fn new() -> Self {
        // Java: super();
        LittleEndian
    }

    // Java: static byte getByte(_data: &[u8], int offset)
    pub fn getByte(data: &[u8], offset: usize) -> u8 {
        // Java: return (byte)(data[offset] & 255);
        data[offset] & 0xFF
    }

    // Java: static int getInt(_data: &[u8], int offset)
    pub fn getInt(data: &[u8], offset: usize) -> i32 {
        // Java: int b0 = data[offset++] & 255;
        let b0 = (data[offset] & 0xFF) as i32;
        // Java: int b1 = data[offset++] & 255;
        let b1 = (data[offset + 1] & 0xFF) as i32;
        // Java: int b2 = data[offset++] & 255;
        let b2 = (data[offset + 2] & 0xFF) as i32;
        // Java: int b3 = data[offset++] & 255;
        let b3 = (data[offset + 3] & 0xFF) as i32;
        // Java: return (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: static int getIntByIndex(_data: &[u8], int index)
    pub fn getIntByIndex(data: &[u8], index: usize) -> i32 {
        // Java: return getInt(data, index << 2);
        Self::getInt(data, index << 2)
    }

    // Java: static short getShort(_data: &[u8])
    pub fn getShort_no_offset(data: &[u8]) -> i16 {
        // Java: return getShort(data, 0);
        Self::getShort(data, 0)
    }

    // Java: static short getShort(_data: &[u8], int offset)
    pub fn getShort(data: &[u8], offset: usize) -> i16 {
        // Java: int b0 = data[offset] & 255;
        let b0 = (data[offset] & 0xFF) as i32;
        // Java: int b1 = data[offset + 1] & 255;
        let b1 = (data[offset + 1] & 0xFF) as i32;
        // Java: return (short)((b1 << 8) + (b0 << 0));
        ((b1 << 8) + b0) as i16
    }

    // Java: static long getUInt(_data: &[u8], int offset)
    pub fn getUInt(data: &[u8], offset: usize) -> i64 {
        // Java: long retNum = (long)getInt(data, offset);
        let retNum = Self::getInt(data, offset) as i64;
        // Java: return retNum & -1L;
        retNum & 0xFFFFFFFFi64
    }

    // Java: static int getUShort(_data: &[u8])
    pub fn getUShort_no_offset(data: &[u8]) -> i32 {
        // Java: return getUShort(data, 0);
        Self::getUShort(data, 0)
    }

    // Java: static int getUShort(_data: &[u8], int offset)
    pub fn getUShort(data: &[u8], offset: usize) -> i32 {
        // Java: int b0 = data[offset] & 255;
        let b0 = (data[offset] & 0xFF) as i32;
        // Java: int b1 = data[offset + 1] & 255;
        let b1 = (data[offset + 1] & 0xFF) as i32;
        // Java: return (b1 << 8) + (b0 << 0);
        (b1 << 8) + b0
    }

    // Java: static void memset_int(_int_p: &[u8], int index, int count, int value)
    pub fn memset_int(int_p: &mut [u8], index: usize, count: usize, value: i32) {
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: putInt(int_p, index + i << 2, value);
            Self::putInt(int_p, (index + i) << 2, value);
        }
    }

    // Java: static void putByte(_data: &[u8], int offset, int value)
    pub fn putByte(data: &mut [u8], offset: usize, value: i32) {
        // Java: data[offset] = (byte)value;
        data[offset] = value as u8;
    }

    // Java: static void putInt(_data: &[u8], int offset, int value)
    pub fn putInt(data: &mut [u8], offset: usize, value: i32) {
        // Java: int i = offset + 1;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset] = ((value as u32) & 0xFF) as u8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[offset + 1] = (((value as u32) >> 8) & 0xFF) as u8;
        // Java: data[i++] = (byte)(value >>> 16 & 255);
        data[offset + 2] = (((value as u32) >> 16) & 0xFF) as u8;
        // Java: data[i++] = (byte)(value >>> 24 & 255);
        data[offset + 3] = (((value as u32) >> 24) & 0xFF) as u8;
    }

    // Java: static void putIntByIndex(_data: &[u8], int index, int value)
    pub fn putIntByIndex(data: &mut [u8], index: usize, value: i32) {
        // Java: putInt(data, index << 2, value);
        Self::putInt(data, index << 2, value);
    }

    // Java: static void putShort(_data: &[u8], int offset, short value)
    pub fn putShort(data: &mut [u8], offset: usize, value: i16) {
        // Java: int i = offset + 1;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset] = ((value as u16) & 0xFF) as u8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[offset + 1] = (((value as u16) >> 8) & 0xFF) as u8;
    }

    // Java: static void putShort(_data: &[u8], short value)
    pub fn putShort_no_offset(data: &mut [u8], value: i16) {
        // Java: putShort(data, 0, value);
        Self::putShort(data, 0, value);
    }

    // Java: static void putUShort(_data: &[u8], int offset, int value)
    pub fn putUShort(data: &mut [u8], offset: usize, value: i32) {
        // Java: int i = offset + 1;
        // Java: data[offset] = (byte)(value >>> 0 & 255);
        data[offset] = ((value as u32) & 0xFF) as u8;
        // Java: data[i++] = (byte)(value >>> 8 & 255);
        data[offset + 1] = (((value as u32) >> 8) & 0xFF) as u8;
    }
}
