#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut)]

// Java: public class PackBitDecoder
pub struct PackBitDecoder {
}

// Java: public class PackBitDecoder
impl PackBitDecoder {
    // Java: PackBitDecoder()
    pub fn new() -> Self {
        // Java: super();
        PackBitDecoder {}
    }

    // Java: static byte[] decodePackbits(byte[] data, int offset, int arraySize, byte[] dst)
    pub fn decodePackbits(data: &[u8], offset: i32, arraySize: i32, dst: Option<Vec<u8>>) -> Vec<u8> {
        // Java: if (dst == null)
        let mut dst = match dst {
            Some(d) => d,
            None => {
                // Java: dst = new byte[arraySize];
                vec![0u8; arraySize as usize]
            }
        };
        // Java: int srcCount = offset;
        let mut srcCount = offset as usize;
        // Java: int dstCount = 0;
        let mut dstCount: usize = 0;
        // Java: try
        let result: Result<Vec<u8>, ()> = (|| {
            // Java: while(dstCount < arraySize)
            while dstCount < arraySize as usize {
                // Java: byte b = data[srcCount++];
                let b = data[srcCount] as i8;
                srcCount += 1;
                // Java: if (b >= 0 && b <= 127)
                if b >= 0 && b <= 127 {
                    // Java: for(int i = 0; i < b + 1; ++i)
                    for _i in 0..(b as i32 + 1) {
                        // Java: dst[dstCount++] = data[srcCount++];
                        dst[dstCount] = data[srcCount];
                        dstCount += 1;
                        srcCount += 1;
                    }
                }
                // Java: else if (b <= -1 && b >= -127)
                else if b <= -1 && b >= -127 {
                    // Java: byte repeat = data[srcCount++];
                    let repeat = data[srcCount];
                    srcCount += 1;
                    // Java: for(int i = 0; i < -b + 1; ++i)
                    for _i in 0..(-(b as i32) + 1) {
                        // Java: dst[dstCount++] = repeat;
                        dst[dstCount] = repeat;
                        dstCount += 1;
                    }
                }
                // Java: else
                else {
                    // Java: ++srcCount;
                    srcCount += 1;
                }
            }
            // Java: return dst;
            Ok(dst.clone())
        })();
        match result {
            Ok(d) => d,
            // Java: catch (ArrayIndexOutOfBoundsException var9)
            Err(_) => {
                // Java: throw new RuntimeException("PackBitDecoder_IndexOutOfBounds");
                panic!("PackBitDecoder_IndexOutOfBounds");
            }
        }
    }

    // Java: static short[] decodePack16bits(byte[] data, int arraySize, short[] dst)
    pub fn decodePack16bits(data: &[u8], arraySize: i32, dst: Option<Vec<i16>>) -> Vec<i16> {
        // Java: if (dst == null)
        let mut dst = match dst {
            Some(d) => d,
            None => {
                // Java: dst = new short[arraySize];
                vec![0i16; arraySize as usize]
            }
        };
        // Java: int srcCount = 0;
        let mut srcCount: usize = 0;
        // Java: int dstCount = 0;
        let mut dstCount: usize = 0;
        // Java: int srcSize = data.length;
        let srcSize = data.len();
        // Java: try
        let result: Result<Vec<i16>, ()> = (|| {
            // Java: int shft = 16;
            let mut shft: i32 = 16;
            // Java: while(true)
            loop {
                // Java: shft -= 8;
                shft -= 8;
                // Java: if (shft < 0)
                if shft < 0 {
                    // Java: return dst;
                    return Ok(dst.clone());
                }
                // Java: dstCount = 0;
                dstCount = 0;
                // Java: while(dstCount < arraySize && srcCount < srcSize)
                while dstCount < arraySize as usize && srcCount < srcSize {
                    // Java: int b = data[srcCount++];
                    let b = data[srcCount] as i8 as i32;
                    srcCount += 1;
                    // Java: if (b >= 0 && b <= 127)
                    if b >= 0 && b <= 127 {
                        // Java: for(int i = 0; i < b; ++i)
                        for _i in 0..b {
                            // Java: int var15 = dstCount++;
                            let var15 = dstCount;
                            dstCount += 1;
                            // Java: dst[var15] = (short)(dst[var15] | (short)data[srcCount++] << shft);
                            dst[var15] = (dst[var15] as i32 | ((data[srcCount] as i8 as i16 as i32) << shft)) as i16;
                            srcCount += 1;
                        }
                    }
                    // Java: else if (b >= 128)
                    else if b >= 128 {
                        // Java: int repeat = (short)data[srcCount++] << shft;
                        let repeat = (data[srcCount] as i8 as i16 as i32) << shft;
                        srcCount += 1;
                        // Java: for(int i = 0; i < b - 126; ++i)
                        for _i in 0..(b - 126) {
                            // Java: int var14 = dstCount++;
                            let var14 = dstCount;
                            dstCount += 1;
                            // Java: dst[var14] = (short)(dst[var14] | repeat);
                            dst[var14] = (dst[var14] as i32 | repeat) as i16;
                        }
                    }
                    // Java: else
                    else {
                        // Java: ++srcCount;
                        srcCount += 1;
                    }
                }
            }
        })();
        match result {
            Ok(d) => d,
            // Java: catch (ArrayIndexOutOfBoundsException var10)
            Err(_) => {
                // Java: throw new RuntimeException("PackBitDecoder_IndexOutOfBounds");
                panic!("PackBitDecoder_IndexOutOfBounds");
            }
        }
    }

    // Java: static int[] decodePack32bits(byte[] data, int arraySize, int[] dst)
    pub fn decodePack32bits(data: &[u8], arraySize: i32, dst: Option<Vec<i32>>) -> Vec<i32> {
        // Java: if (dst == null)
        let mut dst = match dst {
            Some(d) => d,
            None => {
                // Java: dst = new int[arraySize];
                vec![0i32; arraySize as usize]
            }
        };
        // Java: int srcCount = 0;
        let mut srcCount: usize = 0;
        // Java: int dstCount = 0;
        let mut dstCount: usize = 0;
        // Java: int srcSize = data.length;
        let srcSize = data.len();
        // Java: try
        let result: Result<Vec<i32>, ()> = (|| {
            // Java: int shft = 32;
            let mut shft: i32 = 32;
            // Java: while(true)
            loop {
                // Java: shft -= 8;
                shft -= 8;
                // Java: if (shft < 0)
                if shft < 0 {
                    // Java: return dst;
                    return Ok(dst.clone());
                }
                // Java: dstCount = 0;
                dstCount = 0;
                // Java: while(dstCount < arraySize && srcCount < srcSize)
                while dstCount < arraySize as usize && srcCount < srcSize {
                    // Java: byte b = data[srcCount++];
                    let b = data[srcCount] as i8;
                    srcCount += 1;
                    // Java: if (b >= 0 && b <= 127)
                    if b >= 0 && b <= 127 {
                        // Java: for(int i = 0; i < b + 1 && srcCount < srcSize; ++i)
                        for _i in 0..(b as i32 + 1) {
                            if srcCount >= srcSize { break; }
                            // Java: int var15 = dstCount++;
                            let var15 = dstCount;
                            dstCount += 1;
                            // Java: dst[var15] |= data[srcCount++] << shft;
                            dst[var15] |= (data[srcCount] as i8 as i32) << shft;
                            srcCount += 1;
                        }
                    }
                    // Java: else if (b <= -1 && b >= -127)
                    else if b <= -1 && b >= -127 {
                        // Java: byte repeat = data[srcCount++];
                        let repeat = data[srcCount] as i8 as i32;
                        srcCount += 1;
                        // Java: for(int i = 0; i < -b + 1; ++i)
                        for _i in 0..(-(b as i32) + 1) {
                            // Java: int var14 = dstCount++;
                            let var14 = dstCount;
                            dstCount += 1;
                            // Java: dst[var14] |= repeat << shft;
                            dst[var14] |= repeat << shft;
                        }
                    }
                    // Java: else
                    else {
                        // Java: ++srcCount;
                        srcCount += 1;
                    }
                }
            }
        })();
        match result {
            Ok(d) => d,
            // Java: catch (ArrayIndexOutOfBoundsException var10)
            Err(_) => {
                // Java: throw new RuntimeException("PackBitDecoder_IndexOutOfBounds");
                panic!("PackBitDecoder_IndexOutOfBounds");
            }
        }
    }
}
