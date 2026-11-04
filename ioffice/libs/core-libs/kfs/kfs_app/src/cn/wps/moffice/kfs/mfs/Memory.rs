#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java: public final class Memory
#[derive(Default)]
pub struct Memory {}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum ByteOrder {
    #[default]
    BigEndian,
    LittleEndian,
}

impl ByteOrder {
    pub const BIG_ENDIAN: ByteOrder = ByteOrder::BigEndian;
    pub const LITTLE_ENDIAN: ByteOrder = ByteOrder::LittleEndian;
}

// Java: public final class Memory
impl Memory {
    // Java: private Memory()
    pub fn new() -> Memory { Self {} }

    // Java: public static int peekInt(byte[] src, int offset, ByteOrder order)
    pub fn peekInt(src: &[u8], offset: i32, order: ByteOrder) -> i32 {
        let mut off = offset as usize;
        // Java: return order == ByteOrder.BIG_ENDIAN ? (src[offset++] & 255) << 24 | (src[offset++] & 255) << 16 | (src[offset++] & 255) << 8 | (src[offset] & 255) << 0 : ...
        if order == ByteOrder::BIG_ENDIAN {
            let b0 = (src[off] as i32) & 255; off += 1;
            let b1 = (src[off] as i32) & 255; off += 1;
            let b2 = (src[off] as i32) & 255; off += 1;
            let b3 = (src[off] as i32) & 255;
            (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
        } else {
            let b0 = (src[off] as i32) & 255; off += 1;
            let b1 = (src[off] as i32) & 255; off += 1;
            let b2 = (src[off] as i32) & 255; off += 1;
            let b3 = (src[off] as i32) & 255;
            b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
        }
    }

    // Java: public static long peekLong(byte[] src, int offset, ByteOrder order)
    pub fn peekLong(src: &[u8], offset: i32, order: ByteOrder) -> i64 {
        let mut off = offset as usize;
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BIG_ENDIAN {
            // Java: int h = (src[offset++] & 255) << 24 | ...
            let b0 = (src[off] as i32) & 255; off += 1;
            let b1 = (src[off] as i32) & 255; off += 1;
            let b2 = (src[off] as i32) & 255; off += 1;
            let b3 = (src[off] as i32) & 255; off += 1;
            let h = (b0 << 24) | (b1 << 16) | (b2 << 8) | b3;
            // Java: int l = (src[offset++] & 255) << 24 | ...
            let b4 = (src[off] as i32) & 255; off += 1;
            let b5 = (src[off] as i32) & 255; off += 1;
            let b6 = (src[off] as i32) & 255; off += 1;
            let b7 = (src[off] as i32) & 255;
            let l = (b4 << 24) | (b5 << 16) | (b6 << 8) | b7;
            // Java: return (long)h << 32 | (long)l & 4294967295L;
            ((h as i64) << 32) | ((l as i64) & 4294967295i64)
        }
        // Java: else
        else {
            // Java: int l = (src[offset++] & 255) << 0 | ...
            let b0 = (src[off] as i32) & 255; off += 1;
            let b1 = (src[off] as i32) & 255; off += 1;
            let b2 = (src[off] as i32) & 255; off += 1;
            let b3 = (src[off] as i32) & 255; off += 1;
            let l = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
            // Java: int h = (src[offset++] & 255) << 0 | ...
            let b4 = (src[off] as i32) & 255; off += 1;
            let b5 = (src[off] as i32) & 255; off += 1;
            let b6 = (src[off] as i32) & 255; off += 1;
            let b7 = (src[off] as i32) & 255;
            let h = b4 | (b5 << 8) | (b6 << 16) | (b7 << 24);
            // Java: return (long)h << 32 | (long)l & 4294967295L;
            ((h as i64) << 32) | ((l as i64) & 4294967295i64)
        }
    }

    // Java: public static short peekShort(byte[] src, int offset, ByteOrder order)
    pub fn peekShort(src: &[u8], offset: i32, order: ByteOrder) -> i16 {
        let off = offset as usize;
        // Java: return order == ByteOrder.BIG_ENDIAN ? (short)(src[offset] << 8 | src[offset + 1] & 255) : (short)(src[offset + 1] << 8 | src[offset] & 255);
        if order == ByteOrder::BIG_ENDIAN {
            (((src[off] as i32) << 8) | ((src[off + 1] as i32) & 255)) as i16
        } else {
            (((src[off + 1] as i32) << 8) | ((src[off] as i32) & 255)) as i16
        }
    }

    // Java: public static void pokeInt(byte[] dst, int offset, int value, ByteOrder order)
    pub fn pokeInt(dst: &mut [u8], offset: i32, value: i32, order: ByteOrder) {
        let mut off = offset as usize;
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BIG_ENDIAN {
            // Java: dst[offset++] = (byte)(value >> 24 & 255);
            dst[off] = ((value >> 24) & 255) as u8; off += 1;
            dst[off] = ((value >> 16) & 255) as u8; off += 1;
            dst[off] = ((value >> 8) & 255) as u8; off += 1;
            dst[off] = (value & 255) as u8;
        }
        // Java: else
        else {
            dst[off] = (value & 255) as u8; off += 1;
            dst[off] = ((value >> 8) & 255) as u8; off += 1;
            dst[off] = ((value >> 16) & 255) as u8; off += 1;
            dst[off] = ((value >> 24) & 255) as u8;
        }
    }

    // Java: public static void pokeLong(byte[] dst, int offset, long value, ByteOrder order)
    pub fn pokeLong(dst: &mut [u8], offset: i32, value: i64, order: ByteOrder) {
        let mut off = offset as usize;
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BIG_ENDIAN {
            // Java: int i = (int)(value >> 32);
            let mut i = (value >> 32) as i32;
            dst[off] = ((i >> 24) & 255) as u8; off += 1;
            dst[off] = ((i >> 16) & 255) as u8; off += 1;
            dst[off] = ((i >> 8) & 255) as u8; off += 1;
            dst[off] = (i & 255) as u8; off += 1;
            // Java: i = (int)value;
            i = value as i32;
            dst[off] = ((i >> 24) & 255) as u8; off += 1;
            dst[off] = ((i >> 16) & 255) as u8; off += 1;
            dst[off] = ((i >> 8) & 255) as u8; off += 1;
            dst[off] = (i & 255) as u8;
        }
        // Java: else
        else {
            // Java: int i = (int)value;
            let mut i = value as i32;
            dst[off] = (i & 255) as u8; off += 1;
            dst[off] = ((i >> 8) & 255) as u8; off += 1;
            dst[off] = ((i >> 16) & 255) as u8; off += 1;
            dst[off] = ((i >> 24) & 255) as u8; off += 1;
            // Java: i = (int)(value >> 32);
            i = (value >> 32) as i32;
            dst[off] = (i & 255) as u8; off += 1;
            dst[off] = ((i >> 8) & 255) as u8; off += 1;
            dst[off] = ((i >> 16) & 255) as u8; off += 1;
            dst[off] = ((i >> 24) & 255) as u8;
        }
    }

    // Java: public static void pokeShort(byte[] dst, int offset, short value, ByteOrder order)
    pub fn pokeShort(dst: &mut [u8], offset: i32, value: i16, order: ByteOrder) {
        let mut off = offset as usize;
        // Java: if (order == ByteOrder.BIG_ENDIAN)
        if order == ByteOrder::BIG_ENDIAN {
            dst[off] = ((value >> 8) & 255) as u8; off += 1;
            dst[off] = (value & 255) as u8;
        }
        // Java: else
        else {
            dst[off] = (value & 255) as u8; off += 1;
            dst[off] = ((value >> 8) & 255) as u8;
        }
    }
}
