// Java: import java.nio.ByteOrder

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

// Java: public class Memory
pub struct Memory {
}

// Java: public class Memory
impl Memory {
    // Java: Memory()
    fn new() -> Self {
        // Java: super();
        Memory {}
    }

    // Java: static int peekInt(_src: &[u8], int offset, ByteOrder order)
    pub fn peekInt(src: &[u8], offset: usize, order: ByteOrder) -> i32 {
        // Java: return order == ByteOrder.BIG_ENDIAN ? (src[offset++] & 255) << 24 | (src[offset++] & 255) << 16 | (src[offset++] & 255) << 8 | (src[offset] & 255) << 0 : (src[offset++] & 255) << 0 | (src[offset++] & 255) << 8 | (src[offset++] & 255) << 16 | (src[offset] & 255) << 24;
        if order == ByteOrder::BigEndian {
            ((src[offset] as i32 & 255) << 24)
                | ((src[offset + 1] as i32 & 255) << 16)
                | ((src[offset + 2] as i32 & 255) << 8)
                | (src[offset + 3] as i32 & 255)
        } else {
            (src[offset] as i32 & 255)
                | ((src[offset + 1] as i32 & 255) << 8)
                | ((src[offset + 2] as i32 & 255) << 16)
                | ((src[offset + 3] as i32 & 255) << 24)
        }
    }

    // Java: static long peekLong(_src: &[u8], int offset, ByteOrder order)
    pub fn peekLong(src: &[u8], offset: usize, order: ByteOrder) -> i64 {
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BigEndian
        {
            // Java: int h = (src[offset++] & 255) << 24 | (src[offset++] & 255) << 16 | (src[offset++] & 255) << 8 | (src[offset++] & 255) << 0;
            let h = ((src[offset] as i32 & 255) << 24)
                | ((src[offset + 1] as i32 & 255) << 16)
                | ((src[offset + 2] as i32 & 255) << 8)
                | (src[offset + 3] as i32 & 255);
            // Java: int l = (src[offset++] & 255) << 24 | (src[offset++] & 255) << 16 | (src[offset++] & 255) << 8 | (src[offset] & 255) << 0;
            let l = ((src[offset + 4] as i32 & 255) << 24)
                | ((src[offset + 5] as i32 & 255) << 16)
                | ((src[offset + 6] as i32 & 255) << 8)
                | (src[offset + 7] as i32 & 255);
            // Java: return (long)h << 32 | (long)l & 4294967295L;
            ((h as i64) << 32) | ((l as i64) & 4294967295i64)
        }
        // Java: else
        else
        {
            // Java: int l = (src[offset++] & 255) << 0 | (src[offset++] & 255) << 8 | (src[offset++] & 255) << 16 | (src[offset++] & 255) << 24;
            let l = (src[offset] as i32 & 255)
                | ((src[offset + 1] as i32 & 255) << 8)
                | ((src[offset + 2] as i32 & 255) << 16)
                | ((src[offset + 3] as i32 & 255) << 24);
            // Java: int h = (src[offset++] & 255) << 0 | (src[offset++] & 255) << 8 | (src[offset++] & 255) << 16 | (src[offset] & 255) << 24;
            let h = (src[offset + 4] as i32 & 255)
                | ((src[offset + 5] as i32 & 255) << 8)
                | ((src[offset + 6] as i32 & 255) << 16)
                | ((src[offset + 7] as i32 & 255) << 24);
            // Java: return (long)h << 32 | (long)l & 4294967295L;
            ((h as i64) << 32) | ((l as i64) & 4294967295i64)
        }
    }

    // Java: static short peekShort(_src: &[u8], int offset, ByteOrder order)
    pub fn peekShort(src: &[u8], offset: usize, order: ByteOrder) -> i16 {
        // Java: return order == ByteOrder.BIG_ENDIAN ? (short)(src[offset] << 8 | src[offset + 1] & 255) : (short)(src[offset + 1] << 8 | src[offset] & 255);
        if order == ByteOrder::BigEndian {
            (((src[offset] as i32) << 8) | (src[offset + 1] as i32 & 255)) as i16
        } else {
            (((src[offset + 1] as i32) << 8) | (src[offset] as i32 & 255)) as i16
        }
    }

    // Java: static void pokeInt(_dst: &[u8], int offset, int value, ByteOrder order)
    pub fn pokeInt(dst: &mut [u8], offset: usize, value: i32, order: ByteOrder) {
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BigEndian
        {
            // Java: dst[offset++] = (byte)(value >> 24 & 255);
            dst[offset] = ((value >> 24) & 255) as u8;
            // Java: dst[offset++] = (byte)(value >> 16 & 255);
            dst[offset + 1] = ((value >> 16) & 255) as u8;
            // Java: dst[offset++] = (byte)(value >> 8 & 255);
            dst[offset + 2] = ((value >> 8) & 255) as u8;
            // Java: dst[offset] = (byte)(value >> 0 & 255);
            dst[offset + 3] = (value & 255) as u8;
        }
        // Java: else
        else
        {
            // Java: dst[offset++] = (byte)(value >> 0 & 255);
            dst[offset] = (value & 255) as u8;
            // Java: dst[offset++] = (byte)(value >> 8 & 255);
            dst[offset + 1] = ((value >> 8) & 255) as u8;
            // Java: dst[offset++] = (byte)(value >> 16 & 255);
            dst[offset + 2] = ((value >> 16) & 255) as u8;
            // Java: dst[offset] = (byte)(value >> 24 & 255);
            dst[offset + 3] = ((value >> 24) & 255) as u8;
        }
    }

    // Java: static void pokeLong(_dst: &[u8], int offset, long value, ByteOrder order)
    pub fn pokeLong(dst: &mut [u8], offset: usize, value: i64, order: ByteOrder) {
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BigEndian
        {
            // Java: int i = (int)(value >> 32);
            let mut i = (value >> 32) as i32;
            // Java: dst[offset++] = (byte)(i >> 24 & 255);
            dst[offset] = ((i >> 24) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 16 & 255);
            dst[offset + 1] = ((i >> 16) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 8 & 255);
            dst[offset + 2] = ((i >> 8) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 0 & 255);
            dst[offset + 3] = (i & 255) as u8;
            // Java: i = (int)value;
            i = value as i32;
            // Java: dst[offset++] = (byte)(i >> 24 & 255);
            dst[offset + 4] = ((i >> 24) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 16 & 255);
            dst[offset + 5] = ((i >> 16) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 8 & 255);
            dst[offset + 6] = ((i >> 8) & 255) as u8;
            // Java: dst[offset] = (byte)(i >> 0 & 255);
            dst[offset + 7] = (i & 255) as u8;
        }
        // Java: else
        else
        {
            // Java: int i = (int)value;
            let mut i = value as i32;
            // Java: dst[offset++] = (byte)(i >> 0 & 255);
            dst[offset] = (i & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 8 & 255);
            dst[offset + 1] = ((i >> 8) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 16 & 255);
            dst[offset + 2] = ((i >> 16) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 24 & 255);
            dst[offset + 3] = ((i >> 24) & 255) as u8;
            // Java: i = (int)(value >> 32);
            i = (value >> 32) as i32;
            // Java: dst[offset++] = (byte)(i >> 0 & 255);
            dst[offset + 4] = (i & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 8 & 255);
            dst[offset + 5] = ((i >> 8) & 255) as u8;
            // Java: dst[offset++] = (byte)(i >> 16 & 255);
            dst[offset + 6] = ((i >> 16) & 255) as u8;
            // Java: dst[offset] = (byte)(i >> 24 & 255);
            dst[offset + 7] = ((i >> 24) & 255) as u8;
        }
    }

    // Java: static void pokeShort(_dst: &[u8], int offset, short value, ByteOrder order)
    pub fn pokeShort(dst: &mut [u8], offset: usize, value: i16, order: ByteOrder) {
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BigEndian
        {
            // Java: dst[offset++] = (byte)(value >> 8 & 255);
            dst[offset] = ((value >> 8) & 255) as u8;
            // Java: dst[offset] = (byte)(value >> 0 & 255);
            dst[offset + 1] = (value & 255) as u8;
        }
        // Java: else
        else
        {
            // Java: dst[offset++] = (byte)(value >> 0 & 255);
            dst[offset] = (value & 255) as u8;
            // Java: dst[offset] = (byte)(value >> 8 & 255);
            dst[offset + 1] = ((value >> 8) & 255) as u8;
        }
    }
}
