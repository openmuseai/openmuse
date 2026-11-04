#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut, unused_parens)]

// Java: import android.util.Log

// Java field: String TAG
const TAG: &str = "FaxDecoder";
const table1: [i32; 9] = [0, 1, 3, 7, 15, 31, 63, 127, 255];
const table2: [i32; 9] = [0, 128, 192, 224, 240, 248, 252, 254, 255];
const flipTable: [i8; 256] = [0, -128, 64, -64, 32, -96, 96, -32, 16, -112, 80, -48, 48, -80, 112, -16, 8, -120, 72, -56, 40, -88, 104, -24, 24, -104, 88, -40, 56, -72, 120, -8, 4, -124, 68, -60, 36, -92, 100, -28, 20, -108, 84, -44, 52, -76, 116, -12, 12, -116, 76, -52, 44, -84, 108, -20, 28, -100, 92, -36, 60, -68, 124, -4, 2, -126, 66, -62, 34, -94, 98, -30, 18, -110, 82, -46, 50, -78, 114, -14, 10, -118, 74, -54, 42, -86, 106, -22, 26, -102, 90, -38, 58, -70, 122, -6, 6, -122, 70, -58, 38, -90, 102, -26, 22, -106, 86, -42, 54, -74, 118, -10, 14, -114, 78, -50, 46, -82, 110, -18, 30, -98, 94, -34, 62, -66, 126, -2, 1, -127, 65, -63, 33, -95, 97, -31, 17, -111, 81, -47, 49, -79, 113, -15, 9, -119, 73, -55, 41, -87, 105, -23, 25, -103, 89, -39, 57, -71, 121, -7, 5, -123, 69, -59, 37, -91, 101, -27, 21, -107, 85, -43, 53, -75, 117, -11, 13, -115, 77, -51, 45, -83, 109, -19, 29, -99, 93, -35, 61, -67, 125, -3, 3, -125, 67, -61, 35, -93, 99, -29, 19, -109, 83, -45, 51, -77, 115, -13, 11, -117, 75, -53, 43, -85, 107, -21, 27, -101, 91, -37, 59, -69, 123, -5, 7, -121, 71, -57, 39, -89, 103, -25, 23, -105, 87, -41, 55, -73, 119, -9, 15, -113, 79, -49, 47, -81, 111, -17, 31, -97, 95, -33, 63, -65, 127, -1];
// Java field: short[] white
const white: [i16; 1024] = [6430, 6400, 6400, 6400, 3225, 3225, 3225, 3225, 944, 944, 944, 944, 976, 976, 976, 976, 1456, 1456, 1456, 1456, 1488, 1488, 1488, 1488, 718, 718, 718, 718, 718, 718, 718, 718, 750, 750, 750, 750, 750, 750, 750, 750, 1520, 1520, 1520, 1520, 1552, 1552, 1552, 1552, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 428, 654, 654, 654, 654, 654, 654, 654, 654, 1072, 1072, 1072, 1072, 1104, 1104, 1104, 1104, 1136, 1136, 1136, 1136, 1168, 1168, 1168, 1168, 1200, 1200, 1200, 1200, 1232, 1232, 1232, 1232, 622, 622, 622, 622, 622, 622, 622, 622, 1008, 1008, 1008, 1008, 1040, 1040, 1040, 1040, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 1712, 1712, 1712, 1712, 1744, 1744, 1744, 1744, 846, 846, 846, 846, 846, 846, 846, 846, 1264, 1264, 1264, 1264, 1296, 1296, 1296, 1296, 1328, 1328, 1328, 1328, 1360, 1360, 1360, 1360, 1392, 1392, 1392, 1392, 1424, 1424, 1424, 1424, 686, 686, 686, 686, 686, 686, 686, 686, 910, 910, 910, 910, 910, 910, 910, 910, 1968, 1968, 1968, 1968, 2000, 2000, 2000, 2000, 2032, 2032, 2032, 2032, 16, 16, 16, 16, 10257, 10257, 10257, 10257, 12305, 12305, 12305, 12305, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 362, 878, 878, 878, 878, 878, 878, 878, 878, 1904, 1904, 1904, 1904, 1936, 1936, 1936, 1936, -18413, -18413, -16365, -16365, -14317, -14317, -10221, -10221, 590, 590, 590, 590, 590, 590, 590, 590, 782, 782, 782, 782, 782, 782, 782, 782, 1584, 1584, 1584, 1584, 1616, 1616, 1616, 1616, 1648, 1648, 1648, 1648, 1680, 1680, 1680, 1680, 814, 814, 814, 814, 814, 814, 814, 814, 1776, 1776, 1776, 1776, 1808, 1808, 1808, 1808, 1840, 1840, 1840, 1840, 1872, 1872, 1872, 1872, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, 6157, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, -12275, 14353, 14353, 14353, 14353, 16401, 16401, 16401, 16401, 22547, 22547, 24595, 24595, 20497, 20497, 20497, 20497, 18449, 18449, 18449, 18449, 26643, 26643, 28691, 28691, 30739, 30739, -32749, -32749, -30701, -30701, -28653, -28653, -26605, -26605, -24557, -24557, -22509, -22509, -20461, -20461, 8207, 8207, 8207, 8207, 8207, 8207, 8207, 8207, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 104, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 4107, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 266, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 298, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 524, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 168, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 460, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 492, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 2059, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232, 232];
// Java field: short[] additionalMakeup
const additionalMakeup: [i16; 16] = [28679, 28679, 31752, -32759, -31735, -30711, -29687, -28663, 29703, 29703, 30727, 30727, -27639, -26615, -25591, -24567];
// Java field: short[] initBlack
const initBlack: [i16; 16] = [3226, 6412, 200, 168, 38, 38, 134, 134, 100, 100, 100, 100, 68, 68, 68, 68];
// Java field: short[] twoBitBlack
const twoBitBlack: [i16; 4] = [292, 260, 226, 226];
// Java field: short[] black
const black: [i16; 512] = [62, 62, 30, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 3225, 588, 588, 588, 588, 588, 588, 588, 588, 1680, 1680, 20499, 22547, 24595, 26643, 1776, 1776, 1808, 1808, -24557, -22509, -20461, -18413, 1904, 1904, 1936, 1936, -16365, -14317, 782, 782, 782, 782, 814, 814, 814, 814, -12269, -10221, 10257, 10257, 12305, 12305, 14353, 14353, 16403, 18451, 1712, 1712, 1744, 1744, 28691, 30739, -32749, -30701, -28653, -26605, 2061, 2061, 2061, 2061, 2061, 2061, 2061, 2061, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 424, 750, 750, 750, 750, 1616, 1616, 1648, 1648, 1424, 1424, 1456, 1456, 1488, 1488, 1520, 1520, 1840, 1840, 1872, 1872, 1968, 1968, 8209, 8209, 524, 524, 524, 524, 524, 524, 524, 524, 556, 556, 556, 556, 556, 556, 556, 556, 1552, 1552, 1584, 1584, 2000, 2000, 2032, 2032, 976, 976, 1008, 1008, 1040, 1040, 1072, 1072, 1296, 1296, 1328, 1328, 718, 718, 718, 718, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 456, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 326, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 358, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 490, 4113, 4113, 6161, 6161, 848, 848, 880, 880, 912, 912, 944, 944, 622, 622, 622, 622, 654, 654, 654, 654, 1104, 1104, 1136, 1136, 1168, 1168, 1200, 1200, 1232, 1232, 1264, 1264, 686, 686, 686, 686, 1360, 1360, 1392, 1392, 12, 12, 12, 12, 12, 12, 12, 12, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390, 390];
// Java field: byte[] twoDCodes
const twoDCodes: [i8; 128] = [80, 88, 23, 71, 30, 30, 62, 62, 4, 4, 4, 4, 4, 4, 4, 4, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 51, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41];

/// Java's `>>>` (unsigned right shift) for i32
fn ushr(val: i32, shift: i32) -> i32 {
    ((val as u32) >> (shift as u32)) as i32
}

// Java: public class FaxDecoder
pub struct FaxDecoder {
    // Java field: int bitPointer
    bitPointer: i32,
    // Java field: int bytePointer
    bytePointer: i32,
    // Java field: byte[] data
    data: Vec<u8>,
    // Java field: int w
    w: i32,
    // Java field: int h
    h: i32,
    // Java field: int fillOrder
    fillOrder: i32,
    // Java field: int changingElemSize
    changingElemSize: i32,
    // Java field: int[] prevChangingElems
    prevChangingElems: Vec<i32>,
    // Java field: int[] currChangingElems
    currChangingElems: Vec<i32>,
    // Java field: int lastChangingElement
    lastChangingElement: i32,
    // Java field: int compression
    compression: i32,
    // Java field: int uncompressedMode
    uncompressedMode: i32,
    // Java field: int fillBits
    fillBits: i32,
    // Java field: int oneD
    oneD: i32,
    // Java field: int[] table1
    // Java field: int[] table2
    // Java field: byte[] flipTable
    // Java field: short[] white
    // Java field: short[] additionalMakeup
    // Java field: short[] initBlack
    // Java field: short[] twoBitBlack
    // Java field: short[] black
    // Java field: byte[] twoDCodes
}

// Java: public class FaxDecoder
impl FaxDecoder {
    // Java: FaxDecoder(int fillOrder, int w, int h)
    pub fn new(fillOrder: i32, w: i32, h: i32) -> Self {
        // Java: super();
        // Java: this.fillOrder = fillOrder;
        // Java: this.w = w;
        // Java: this.h = h;
        // Java: this.bitPointer = 0;
        // Java: this.bytePointer = 0;
        // Java: this.prevChangingElems = new int[w];
        // Java: this.currChangingElems = new int[w];
        FaxDecoder {
            fillOrder,
            w,
            h,
            bitPointer: 0,
            bytePointer: 0,
            prevChangingElems: vec![0i32; w as usize],
            currChangingElems: vec![0i32; w as usize],
            data: Vec::new(),
            changingElemSize: 0,
            lastChangingElement: 0,
            compression: 2,
            uncompressedMode: 0,
            fillBits: 0,
            oneD: 0,
        }
    }

    // Java: void decode1D(byte[] buffer, int offset, byte[] compData, int startX, int height)
    pub fn decode1D(&mut self, buffer: &mut [u8], offset: i32, compData: &[u8], startX: i32, height: i32) {
        // Java: this.data = compData;
        self.data = compData.to_vec();
        // Java: int lineOffset = 0;
        let mut lineOffset = 0i32;
        // Java: int scanlineStride = (this.w + 7) / 8;
        let scanlineStride = (self.w + 7) / 8;
        // Java: this.bitPointer = 0;
        self.bitPointer = 0;
        // Java: this.bytePointer = offset;
        self.bytePointer = offset;
        // Java: for(int i = 0; i < height; ++i)
        for _i in 0..height {
            // Java: this.decodeNextScanline(buffer, lineOffset, startX);
            self.decodeNextScanline(buffer, lineOffset, startX);
            // Java: lineOffset += scanlineStride;
            lineOffset += scanlineStride;
        }
    }

    // Java: void decodeNextScanline(byte[] buffer, int lineOffset, int bitOffset)
    pub fn decodeNextScanline(&mut self, buffer: &mut [u8], lineOffset: i32, mut bitOffset: i32) {
        // Java: int bits = 0;
        let mut bits: i32 = 0;
        // Java: int code = 0;
        let mut code: i32 = 0;
        // Java: int isT = 0;
        let mut isT: i32 = 0;
        // Java: boolean isWhite = true;
        let mut isWhite = true;
        // Java: this.changingElemSize = 0;
        self.changingElemSize = 0;
        // Java: while(bitOffset < this.w)
        while bitOffset < self.w {
            // Java: while(isWhite)
            while isWhite {
                // Java: int current = this.nextNBits(10);
                let mut current = self.nextNBits(10);
                // Java: int entry = white[current];
                let mut entry = white[current as usize] as i32;
                // Java: isT = entry & 1;
                isT = entry & 1;
                // Java: bits = entry >>> 1 & 15;
                bits = ushr(entry, 1) & 15;
                // Java: if (bits == 12)
                if bits == 12 {
                    // Java: int twoBits = this.nextLesserThan8Bits(2);
                    let twoBits = self.nextLesserThan8Bits(2);
                    // Java: current = current << 2 & 12 | twoBits;
                    current = (current << 2) & 12 | twoBits;
                    // Java: entry = additionalMakeup[current];
                    entry = additionalMakeup[current as usize] as i32;
                    // Java: bits = entry >>> 1 & 7;
                    bits = ushr(entry, 1) & 7;
                    // Java: code = entry >>> 4 & 4095;
                    code = ushr(entry, 4) & 4095;
                    // Java: bitOffset += code;
                    bitOffset += code;
                    // Java: this.updatePointer(4 - bits);
                    self.updatePointer(4 - bits);
                }
                // Java: else
                else {
                    // Java: if (bits == 0)
                    if bits == 0 {
                        // Java: Log.e(TAG, "TIFFFaxDecoder0: error: bits = 0");
                        eprintln!("{}: TIFFFaxDecoder0: error: bits = 0", TAG);
                        // Java: throw new RuntimeException("TIFFFaxDecoder0");
                        panic!("TIFFFaxDecoder0");
                    }
                    // Java: if (bits == 15)
                    if bits == 15 {
                        // Java: Log.e(TAG, "TIFFFaxDecoder1: error: bits = 15");
                        eprintln!("{}: TIFFFaxDecoder1: error: bits = 15", TAG);
                        // Java: throw new RuntimeException("TIFFFaxDecoder1");
                        panic!("TIFFFaxDecoder1");
                    }
                    // Java: code = entry >>> 5 & 2047;
                    code = ushr(entry, 5) & 2047;
                    // Java: bitOffset += code;
                    bitOffset += code;
                    // Java: this.updatePointer(10 - bits);
                    self.updatePointer(10 - bits);
                    // Java: if (isT == 0)
                    if isT == 0 {
                        // Java: isWhite = false;
                        isWhite = false;
                        // Java: this.currChangingElems[this.changingElemSize++] = bitOffset;
                        self.currChangingElems[self.changingElemSize as usize] = bitOffset;
                        self.changingElemSize += 1;
                    }
                }
            }
            // Java: if (bitOffset == this.w)
            if bitOffset == self.w {
                // Java: if (this.compression == 2)
                if self.compression == 2 {
                    // Java: this.advancePointer();
                    self.advancePointer();
                }
                // Java: break;
                break;
            }
            // Java: while(!isWhite)
            while !isWhite {
                // Java: int current = this.nextLesserThan8Bits(4);
                let mut current = self.nextLesserThan8Bits(4);
                // Java: int entry = initBlack[current];
                let mut entry = initBlack[current as usize] as i32;
                // Java: isT = entry & 1;
                isT = entry & 1;
                // Java: bits = entry >>> 1 & 15;
                bits = ushr(entry, 1) & 15;
                // Java: code = entry >>> 5 & 2047;
                code = ushr(entry, 5) & 2047;
                // Java: if (code == 100)
                if code == 100 {
                    // Java: current = this.nextNBits(9);
                    current = self.nextNBits(9);
                    // Java: entry = black[current];
                    entry = black[current as usize] as i32;
                    // Java: isT = entry & 1;
                    isT = entry & 1;
                    // Java: bits = entry >>> 1 & 15;
                    bits = ushr(entry, 1) & 15;
                    // Java: code = entry >>> 5 & 2047;
                    code = ushr(entry, 5) & 2047;
                    // Java: if (bits == 12)
                    if bits == 12 {
                        // Java: this.updatePointer(5);
                        self.updatePointer(5);
                        // Java: current = this.nextLesserThan8Bits(4);
                        current = self.nextLesserThan8Bits(4);
                        // Java: entry = additionalMakeup[current];
                        entry = additionalMakeup[current as usize] as i32;
                        // Java: bits = entry >>> 1 & 7;
                        bits = ushr(entry, 1) & 7;
                        // Java: code = entry >>> 4 & 4095;
                        code = ushr(entry, 4) & 4095;
                        // Java: this.setToBlack(buffer, lineOffset, bitOffset, code);
                        self.setToBlack(buffer, lineOffset, bitOffset, code);
                        // Java: bitOffset += code;
                        bitOffset += code;
                        // Java: this.updatePointer(4 - bits);
                        self.updatePointer(4 - bits);
                    }
                    // Java: else
                    else {
                        // Java: if (bits == 15)
                        if bits == 15 {
                            // Java: Log.e(TAG, "TIFFFaxDecoder2: error: bits = 15");
                            eprintln!("{}: TIFFFaxDecoder2: error: bits = 15", TAG);
                            // Java: throw new RuntimeException("TIFFFaxDecoder2");
                            panic!("TIFFFaxDecoder2");
                        }
                        // Java: this.setToBlack(buffer, lineOffset, bitOffset, code);
                        self.setToBlack(buffer, lineOffset, bitOffset, code);
                        // Java: bitOffset += code;
                        bitOffset += code;
                        // Java: this.updatePointer(9 - bits);
                        self.updatePointer(9 - bits);
                        // Java: if (isT == 0)
                        if isT == 0 {
                            // Java: isWhite = true;
                            isWhite = true;
                            // Java: this.currChangingElems[this.changingElemSize++] = bitOffset;
                            self.currChangingElems[self.changingElemSize as usize] = bitOffset;
                            self.changingElemSize += 1;
                        }
                    }
                }
                // Java: else if (code == 200)
                else if code == 200 {
                    // Java: current = this.nextLesserThan8Bits(2);
                    current = self.nextLesserThan8Bits(2);
                    // Java: entry = twoBitBlack[current];
                    entry = twoBitBlack[current as usize] as i32;
                    // Java: code = entry >>> 5 & 2047;
                    code = ushr(entry, 5) & 2047;
                    // Java: bits = entry >>> 1 & 15;
                    bits = ushr(entry, 1) & 15;
                    // Java: this.setToBlack(buffer, lineOffset, bitOffset, code);
                    self.setToBlack(buffer, lineOffset, bitOffset, code);
                    // Java: bitOffset += code;
                    bitOffset += code;
                    // Java: this.updatePointer(2 - bits);
                    self.updatePointer(2 - bits);
                    // Java: isWhite = true;
                    isWhite = true;
                    // Java: this.currChangingElems[this.changingElemSize++] = bitOffset;
                    self.currChangingElems[self.changingElemSize as usize] = bitOffset;
                    self.changingElemSize += 1;
                }
                // Java: else
                else {
                    // Java: this.setToBlack(buffer, lineOffset, bitOffset, code);
                    self.setToBlack(buffer, lineOffset, bitOffset, code);
                    // Java: bitOffset += code;
                    bitOffset += code;
                    // Java: this.updatePointer(4 - bits);
                    self.updatePointer(4 - bits);
                    // Java: isWhite = true;
                    isWhite = true;
                    // Java: this.currChangingElems[this.changingElemSize++] = bitOffset;
                    self.currChangingElems[self.changingElemSize as usize] = bitOffset;
                    self.changingElemSize += 1;
                }
            }
            // Java: if (bitOffset == this.w)
            if bitOffset == self.w {
                // Java: if (this.compression == 2)
                if self.compression == 2 {
                    // Java: this.advancePointer();
                    self.advancePointer();
                }
                // Java: break;
                break;
            }
        }
        // Java: this.currChangingElems[this.changingElemSize++] = bitOffset;
        self.currChangingElems[self.changingElemSize as usize] = bitOffset;
        self.changingElemSize += 1;
    }

    // Java: void decode2D(byte[] buffer, int offset, byte[] compData, int startX, int height, long tiffT4Options)
    pub fn decode2D(&mut self, buffer: &mut [u8], offset: i32, compData: &[u8], startX: i32, height: i32, tiffT4Options: i64) {
        // Java: this.data = compData;
        self.data = compData.to_vec();
        // Java: this.compression = 3;
        self.compression = 3;
        // Java: this.bitPointer = 0;
        self.bitPointer = 0;
        // Java: this.bytePointer = offset;
        self.bytePointer = offset;
        // Java: int scanlineStride = (this.w + 7) / 8;
        let scanlineStride = (self.w + 7) / 8;
        // Java: int[] b = new int[2];
        let mut b = [0i32; 2];
        // Java: int currIndex = 0;
        let mut currIndex: i32;
        // Java: this.oneD = (int)(tiffT4Options & 1L);
        self.oneD = (tiffT4Options & 1) as i32;
        // Java: this.uncompressedMode = (int)((tiffT4Options & 2L) >> 1);
        self.uncompressedMode = ((tiffT4Options & 2) >> 1) as i32;
        // Java: this.fillBits = (int)((tiffT4Options & 4L) >> 2);
        self.fillBits = ((tiffT4Options & 4) >> 2) as i32;
        // Java: if (this.readEOL() != 1)
        if self.readEOL() != 1 {
            // Java: Log.e(TAG, "TIFFFaxDecoder3: error: readEOL() != 1");
            eprintln!("{}: TIFFFaxDecoder3: error: readEOL() != 1", TAG);
            // Java: throw new RuntimeException("TIFFFaxDecoder3");
            panic!("TIFFFaxDecoder3");
        }
        // Java: else
        {
            // Java: int lineOffset = 0;
            let mut lineOffset = 0;
            // Java: this.decodeNextScanline(buffer, lineOffset, startX);
            self.decodeNextScanline(buffer, lineOffset, startX);
            // Java: lineOffset += scanlineStride;
            lineOffset += scanlineStride;
            // Java: for(int lines = 1; lines < height; ++lines)
            for _lines in 1..height {
                // Java: if (this.readEOL() != 0)
                if self.readEOL() != 0 {
                    // Java: this.decodeNextScanline(buffer, lineOffset, startX);
                    self.decodeNextScanline(buffer, lineOffset, startX);
                }
                // Java: else
                else {
                    // Java: int[] temp = this.prevChangingElems;
                    // Java: this.prevChangingElems = this.currChangingElems;
                    // Java: this.currChangingElems = temp;
                    std::mem::swap(&mut self.prevChangingElems, &mut self.currChangingElems);
                    // Java: currIndex = 0;
                    currIndex = 0;
                    // Java: int a0 = -1;
                    let mut a0 = -1;
                    // Java: boolean isWhite = true;
                    let mut isWhite = true;
                    // Java: int bitOffset = startX;
                    let mut bitOffset = startX;
                    // Java: this.lastChangingElement = 0;
                    self.lastChangingElement = 0;
                    // Java: while(bitOffset < this.w)
                    while bitOffset < self.w {
                        // Java: this.getNextChangingElement(a0, isWhite, b);
                        self.getNextChangingElement(a0, isWhite, &mut b);
                        // Java: int b1 = b[0];
                        let b1 = b[0];
                        // Java: int b2 = b[1];
                        let b2 = b[1];
                        // Java: int entry = this.nextLesserThan8Bits(7);
                        let mut entry = self.nextLesserThan8Bits(7);
                        // Java: entry = twoDCodes[entry] & 255;
                        entry = (twoDCodes[entry as usize] as i32) & 255;
                        // Java: int code = (entry & 120) >>> 3;
                        let code = ushr(entry & 120, 3);
                        // Java: int bits = entry & 7;
                        let bits = entry & 7;
                        // Java: if (code == 0)
                        if code == 0 {
                            // Java: if (!isWhite)
                            if !isWhite {
                                // Java: this.setToBlack(buffer, lineOffset, bitOffset, b2 - bitOffset);
                                self.setToBlack(buffer, lineOffset, bitOffset, b2 - bitOffset);
                            }
                            // Java: a0 = b2;
                            a0 = b2;
                            // Java: bitOffset = b2;
                            bitOffset = b2;
                            // Java: this.updatePointer(7 - bits);
                            self.updatePointer(7 - bits);
                        }
                        // Java: else if (code == 1)
                        else if code == 1 {
                            // Java: this.updatePointer(7 - bits);
                            self.updatePointer(7 - bits);
                            // Java: if (isWhite)
                            if isWhite {
                                // Java: int number = this.decodeWhiteCodeWord();
                                let mut number = self.decodeWhiteCodeWord();
                                // Java: int var30 = bitOffset + number;
                                let var30 = bitOffset + number;
                                // Java: this.currChangingElems[currIndex++] = var30;
                                self.currChangingElems[currIndex as usize] = var30;
                                currIndex += 1;
                                // Java: number = this.decodeBlackCodeWord();
                                number = self.decodeBlackCodeWord();
                                // Java: this.setToBlack(buffer, lineOffset, var30, number);
                                self.setToBlack(buffer, lineOffset, var30, number);
                                // Java: bitOffset = var30 + number;
                                bitOffset = var30 + number;
                                // Java: this.currChangingElems[currIndex++] = bitOffset;
                                self.currChangingElems[currIndex as usize] = bitOffset;
                                currIndex += 1;
                            }
                            // Java: else
                            else {
                                // Java: int number = this.decodeBlackCodeWord();
                                let mut number = self.decodeBlackCodeWord();
                                // Java: this.setToBlack(buffer, lineOffset, bitOffset, number);
                                self.setToBlack(buffer, lineOffset, bitOffset, number);
                                // Java: int var31 = bitOffset + number;
                                let var31 = bitOffset + number;
                                // Java: this.currChangingElems[currIndex++] = var31;
                                self.currChangingElems[currIndex as usize] = var31;
                                currIndex += 1;
                                // Java: number = this.decodeWhiteCodeWord();
                                number = self.decodeWhiteCodeWord();
                                // Java: bitOffset = var31 + number;
                                bitOffset = var31 + number;
                                // Java: this.currChangingElems[currIndex++] = bitOffset;
                                self.currChangingElems[currIndex as usize] = bitOffset;
                                currIndex += 1;
                            }
                            // Java: a0 = bitOffset;
                            a0 = bitOffset;
                        }
                        // Java: else
                        else {
                            // Java: if (code > 8)
                            if code > 8 {
                                // Java: Log.e(TAG, "TIFFFaxDecoder4: error: code:" + code);
                                eprintln!("{}: TIFFFaxDecoder4: error: code:{}", TAG, code);
                                // Java: throw new RuntimeException("TIFFFaxDecoder4");
                                panic!("TIFFFaxDecoder4");
                            }
                            // Java: int a1 = b1 + (code - 5);
                            let a1 = b1 + (code - 5);
                            // Java: this.currChangingElems[currIndex++] = a1;
                            self.currChangingElems[currIndex as usize] = a1;
                            currIndex += 1;
                            // Java: if (!isWhite)
                            if !isWhite {
                                // Java: this.setToBlack(buffer, lineOffset, bitOffset, a1 - bitOffset);
                                self.setToBlack(buffer, lineOffset, bitOffset, a1 - bitOffset);
                            }
                            // Java: a0 = a1;
                            a0 = a1;
                            // Java: bitOffset = a1;
                            bitOffset = a1;
                            // Java: isWhite = !isWhite;
                            isWhite = !isWhite;
                            // Java: this.updatePointer(7 - bits);
                            self.updatePointer(7 - bits);
                        }
                    }
                    // Java: this.currChangingElems[currIndex++] = bitOffset;
                    self.currChangingElems[currIndex as usize] = bitOffset;
                    currIndex += 1;
                    // Java: this.changingElemSize = currIndex;
                    self.changingElemSize = currIndex;
                }
                // Java: lineOffset += scanlineStride;
                lineOffset += scanlineStride;
            }
        }
    }

    // Java: void decodeT6(byte[] buffer, int offset, byte[] compData, int startX, int height, long tiffT6Options)
    pub fn decodeT6(&mut self, buffer: &mut [u8], offset: i32, compData: &[u8], startX: i32, height: i32, tiffT6Options: i64) {
        // Java: this.data = compData;
        self.data = compData.to_vec();
        // Java: this.compression = 4;
        self.compression = 4;
        // Java: this.bitPointer = 0;
        self.bitPointer = 0;
        // Java: this.bytePointer = offset;
        self.bytePointer = offset;
        // Java: int scanlineStride = (this.w + 7) / 8;
        let scanlineStride = (self.w + 7) / 8;
        // Java: int[] b = new int[2];
        let mut b = [0i32; 2];
        // Java: this.uncompressedMode = (int)((tiffT6Options & 2L) >> 1);
        self.uncompressedMode = ((tiffT6Options & 2) >> 1) as i32;
        // Java: int[] cce = this.currChangingElems;
        // Java: this.changingElemSize = 0;
        self.changingElemSize = 0;
        // Java: cce[this.changingElemSize++] = this.w;
        self.currChangingElems[self.changingElemSize as usize] = self.w;
        self.changingElemSize += 1;
        // Java: cce[this.changingElemSize++] = this.w;
        self.currChangingElems[self.changingElemSize as usize] = self.w;
        self.changingElemSize += 1;
        // Java: int lineOffset = 0;
        let mut lineOffset = 0;
        // Java: for(int lines = 0; lines < height; ++lines)
        for _lines in 0..height {
            // Java: int a0 = -1;
            let mut a0 = -1;
            // Java: boolean isWhite = true;
            let mut isWhite = true;
            // Java: int[] temp = this.prevChangingElems;
            // Java: this.prevChangingElems = this.currChangingElems;
            // Java: cce = this.currChangingElems = temp;
            std::mem::swap(&mut self.prevChangingElems, &mut self.currChangingElems);
            // Java: int currIndex = 0;
            let mut currIndex: i32 = 0;
            // Java: int bitOffset = startX;
            let mut bitOffset = startX;
            // Java: this.lastChangingElement = 0;
            self.lastChangingElement = 0;
            // Java: while(bitOffset < this.w)
            while bitOffset < self.w {
                // Java: this.getNextChangingElement(a0, isWhite, b);
                self.getNextChangingElement(a0, isWhite, &mut b);
                // Java: int b1 = b[0];
                let b1 = b[0];
                // Java: int b2 = b[1];
                let b2 = b[1];
                // Java: int entry = this.nextLesserThan8Bits(7);
                let mut entry = self.nextLesserThan8Bits(7);
                // Java: entry = twoDCodes[entry] & 255;
                entry = (twoDCodes[entry as usize] as i32) & 255;
                // Java: int code = (entry & 120) >>> 3;
                let code = ushr(entry & 120, 3);
                // Java: int bits = entry & 7;
                let bits = entry & 7;
                // Java: if (code == 0)
                if code == 0 {
                    // Java: if (!isWhite)
                    if !isWhite {
                        // Java: this.setToBlack(buffer, lineOffset, bitOffset, b2 - bitOffset);
                        self.setToBlack(buffer, lineOffset, bitOffset, b2 - bitOffset);
                    }
                    // Java: a0 = b2;
                    a0 = b2;
                    // Java: bitOffset = b2;
                    bitOffset = b2;
                    // Java: this.updatePointer(7 - bits);
                    self.updatePointer(7 - bits);
                }
                // Java: else if (code == 1)
                else if code == 1 {
                    // Java: this.updatePointer(7 - bits);
                    self.updatePointer(7 - bits);
                    // Java: if (isWhite)
                    if isWhite {
                        // Java: int number = this.decodeWhiteCodeWord();
                        let mut number = self.decodeWhiteCodeWord();
                        // Java: int var31 = bitOffset + number;
                        let var31 = bitOffset + number;
                        // Java: cce[currIndex++] = var31;
                        self.currChangingElems[currIndex as usize] = var31;
                        currIndex += 1;
                        // Java: number = this.decodeBlackCodeWord();
                        number = self.decodeBlackCodeWord();
                        // Java: this.setToBlack(buffer, lineOffset, var31, number);
                        self.setToBlack(buffer, lineOffset, var31, number);
                        // Java: bitOffset = var31 + number;
                        bitOffset = var31 + number;
                        // Java: cce[currIndex++] = bitOffset;
                        self.currChangingElems[currIndex as usize] = bitOffset;
                        currIndex += 1;
                    }
                    // Java: else
                    else {
                        // Java: int number = this.decodeBlackCodeWord();
                        let mut number = self.decodeBlackCodeWord();
                        // Java: this.setToBlack(buffer, lineOffset, bitOffset, number);
                        self.setToBlack(buffer, lineOffset, bitOffset, number);
                        // Java: int var32 = bitOffset + number;
                        let var32 = bitOffset + number;
                        // Java: cce[currIndex++] = var32;
                        self.currChangingElems[currIndex as usize] = var32;
                        currIndex += 1;
                        // Java: number = this.decodeWhiteCodeWord();
                        number = self.decodeWhiteCodeWord();
                        // Java: bitOffset = var32 + number;
                        bitOffset = var32 + number;
                        // Java: cce[currIndex++] = bitOffset;
                        self.currChangingElems[currIndex as usize] = bitOffset;
                        currIndex += 1;
                    }
                    // Java: a0 = bitOffset;
                    a0 = bitOffset;
                }
                // Java: else if (code <= 8)
                else if code <= 8 {
                    // Java: int a1 = b1 + (code - 5);
                    let a1 = b1 + (code - 5);
                    // Java: cce[currIndex++] = a1;
                    self.currChangingElems[currIndex as usize] = a1;
                    currIndex += 1;
                    // Java: if (!isWhite)
                    if !isWhite {
                        // Java: this.setToBlack(buffer, lineOffset, bitOffset, a1 - bitOffset);
                        self.setToBlack(buffer, lineOffset, bitOffset, a1 - bitOffset);
                    }
                    // Java: a0 = a1;
                    a0 = a1;
                    // Java: bitOffset = a1;
                    bitOffset = a1;
                    // Java: isWhite = !isWhite;
                    isWhite = !isWhite;
                    // Java: this.updatePointer(7 - bits);
                    self.updatePointer(7 - bits);
                }
                // Java: else
                else {
                    // Java: if (code != 11)
                    if code != 11 {
                        // Java: Log.w(TAG, "TIFFFaxDecoder5: code error: " + code);
                        eprintln!("{}: TIFFFaxDecoder5: code error: {}", TAG, code);
                        // Java: throw new RuntimeException("TIFFFaxDecoder5");
                        panic!("TIFFFaxDecoder5");
                    }
                    // Java: if (this.nextLesserThan8Bits(3) != 7)
                    if self.nextLesserThan8Bits(3) != 7 {
                        // Java: Log.w(TAG, "TIFFFaxDecoder5: nextLesserThan8Bits(3) != 7, nextLesserThan8Bits(3): " + this.nextLesserThan8Bits(3));
                        eprintln!("{}: TIFFFaxDecoder5: nextLesserThan8Bits(3) != 7, nextLesserThan8Bits(3): {}", TAG, self.nextLesserThan8Bits(3));
                        // Java: throw new RuntimeException("TIFFFaxDecoder5");
                        panic!("TIFFFaxDecoder5");
                    }
                    // Java: int zeros = 0;
                    let mut zeros = 0;
                    // Java: boolean exit = false;
                    let mut exit = false;
                    // Java: while(!exit)
                    while !exit {
                        // Java: while(this.nextLesserThan8Bits(1) != 1)
                        while self.nextLesserThan8Bits(1) != 1 {
                            // Java: ++zeros;
                            zeros += 1;
                        }
                        // Java: if (zeros > 5)
                        if zeros > 5 {
                            // Java: zeros -= 6;
                            zeros -= 6;
                            // Java: if (!isWhite && zeros > 0)
                            if !isWhite && zeros > 0 {
                                // Java: cce[currIndex++] = bitOffset;
                                self.currChangingElems[currIndex as usize] = bitOffset;
                                currIndex += 1;
                            }
                            // Java: bitOffset += zeros;
                            bitOffset += zeros;
                            // Java: if (zeros > 0)
                            if zeros > 0 {
                                // Java: isWhite = true;
                                isWhite = true;
                            }
                            // Java: if (this.nextLesserThan8Bits(1) == 0)
                            if self.nextLesserThan8Bits(1) == 0 {
                                // Java: if (!isWhite)
                                if !isWhite {
                                    // Java: cce[currIndex++] = bitOffset;
                                    self.currChangingElems[currIndex as usize] = bitOffset;
                                    currIndex += 1;
                                }
                                // Java: isWhite = true;
                                isWhite = true;
                            }
                            // Java: else
                            else {
                                // Java: if (isWhite)
                                if isWhite {
                                    // Java: cce[currIndex++] = bitOffset;
                                    self.currChangingElems[currIndex as usize] = bitOffset;
                                    currIndex += 1;
                                }
                                // Java: isWhite = false;
                                isWhite = false;
                            }
                            // Java: exit = true;
                            exit = true;
                        }
                        // Java: if (zeros == 5)
                        if zeros == 5 {
                            // Java: if (!isWhite)
                            if !isWhite {
                                // Java: cce[currIndex++] = bitOffset;
                                self.currChangingElems[currIndex as usize] = bitOffset;
                                currIndex += 1;
                            }
                            // Java: bitOffset += zeros;
                            bitOffset += zeros;
                            // Java: isWhite = true;
                            isWhite = true;
                        }
                        // Java: else
                        else {
                            // Java: bitOffset += zeros;
                            bitOffset += zeros;
                            // Java: cce[currIndex++] = bitOffset;
                            self.currChangingElems[currIndex as usize] = bitOffset;
                            currIndex += 1;
                            // Java: this.setToBlack(buffer, lineOffset, bitOffset, 1);
                            self.setToBlack(buffer, lineOffset, bitOffset, 1);
                            // Java: ++bitOffset;
                            bitOffset += 1;
                            // Java: isWhite = false;
                            isWhite = false;
                        }
                    }
                }
            }
            // Java: if (currIndex <= this.w)
            if currIndex <= self.w {
                // Java: cce[currIndex++] = bitOffset;
                self.currChangingElems[currIndex as usize] = bitOffset;
                currIndex += 1;
            }
            // Java: this.changingElemSize = currIndex;
            self.changingElemSize = currIndex;
            // Java: lineOffset += scanlineStride;
            lineOffset += scanlineStride;
        }
    }

    // Java: void setToBlack(byte[] buffer, int lineOffset, int bitOffset, int numBits)
    fn setToBlack(&self, buffer: &mut [u8], lineOffset: i32, bitOffset: i32, numBits: i32) {
        // Java: int bitNum = 8 * lineOffset + bitOffset;
        let mut bitNum = 8 * lineOffset + bitOffset;
        // Java: int lastBit = bitNum + numBits;
        let lastBit = bitNum + numBits;
        // Java: int byteNum = bitNum >> 3;
        let mut byteNum = (bitNum >> 3) as usize;
        // Java: int shift = bitNum & 7;
        let shift = bitNum & 7;
        // Java: if (shift > 0)
        if shift > 0 {
            // Java: int maskVal = 1 << 7 - shift;
            let mut maskVal = 1i32 << (7 - shift);
            // Java: byte val;
            // Java: for(val = buffer[byteNum]; maskVal > 0 && bitNum < lastBit; ++bitNum)
            let mut val = buffer[byteNum] as i32;
            while maskVal > 0 && bitNum < lastBit {
                // Java: val = (byte)(val | maskVal);
                val |= maskVal;
                // Java: maskVal >>= 1;
                maskVal >>= 1;
                bitNum += 1;
            }
            // Java: buffer[byteNum] = val;
            buffer[byteNum] = val as u8;
        }
        // Java: for(int var11 = bitNum >> 3; bitNum < lastBit - 7; bitNum += 8)
        let mut var11 = (bitNum >> 3) as usize;
        while bitNum < lastBit - 7 {
            // Java: buffer[var11++] = -1;
            buffer[var11] = 0xFFu8;
            var11 += 1;
            bitNum += 8;
        }
        // Java: while(bitNum < lastBit)
        while bitNum < lastBit {
            // Java: byteNum = bitNum >> 3;
            byteNum = (bitNum >> 3) as usize;
            // Java: buffer[byteNum] = (byte)(buffer[byteNum] | 1 << 7 - (bitNum & 7));
            buffer[byteNum] |= 1u8 << (7 - (bitNum & 7));
            // Java: ++bitNum;
            bitNum += 1;
        }
    }

    // Java: int decodeWhiteCodeWord()
    fn decodeWhiteCodeWord(&mut self) -> i32 {
        // Java: int code = -1;
        let mut code: i32 = -1;
        // Java: int runLength = 0;
        let mut runLength: i32 = 0;
        // Java: boolean isWhite = true;
        let mut isWhite = true;
        // Java: while(isWhite)
        while isWhite {
            // Java: int current = this.nextNBits(10);
            let mut current = self.nextNBits(10);
            // Java: int entry = white[current];
            let mut entry = white[current as usize] as i32;
            // Java: int isT = entry & 1;
            let isT = entry & 1;
            // Java: int bits = entry >>> 1 & 15;
            let mut bits = ushr(entry, 1) & 15;
            // Java: if (bits == 12)
            if bits == 12 {
                // Java: int twoBits = this.nextLesserThan8Bits(2);
                let twoBits = self.nextLesserThan8Bits(2);
                // Java: current = current << 2 & 12 | twoBits;
                current = (current << 2) & 12 | twoBits;
                // Java: entry = additionalMakeup[current];
                entry = additionalMakeup[current as usize] as i32;
                // Java: bits = entry >>> 1 & 7;
                bits = ushr(entry, 1) & 7;
                // Java: code = entry >>> 4 & 4095;
                code = ushr(entry, 4) & 4095;
                // Java: runLength += code;
                runLength += code;
                // Java: this.updatePointer(4 - bits);
                self.updatePointer(4 - bits);
            }
            // Java: else
            else {
                // Java: if (bits == 0)
                if bits == 0 {
                    // Java: Log.e(TAG, "TIFFFaxDecoder0: code error: " + code);
                    eprintln!("{}: TIFFFaxDecoder0: code error: {}", TAG, code);
                    // Java: throw new RuntimeException("TIFFFaxDecoder0");
                    panic!("TIFFFaxDecoder0");
                }
                // Java: if (bits == 15)
                if bits == 15 {
                    // Java: Log.e(TAG, "TIFFFaxDecoder1: code error: " + code);
                    eprintln!("{}: TIFFFaxDecoder1: code error: {}", TAG, code);
                    // Java: throw new RuntimeException("TIFFFaxDecoder1");
                    panic!("TIFFFaxDecoder1");
                }
                // Java: code = entry >>> 5 & 2047;
                code = ushr(entry, 5) & 2047;
                // Java: runLength += code;
                runLength += code;
                // Java: this.updatePointer(10 - bits);
                self.updatePointer(10 - bits);
                // Java: if (isT == 0)
                if isT == 0 {
                    // Java: isWhite = false;
                    isWhite = false;
                }
            }
        }
        // Java: return runLength;
        runLength
    }

    // Java: int decodeBlackCodeWord()
    fn decodeBlackCodeWord(&mut self) -> i32 {
        // Java: int code = -1;
        let mut code: i32 = -1;
        // Java: int runLength = 0;
        let mut runLength: i32 = 0;
        // Java: boolean isWhite = false;
        let mut isWhite = false;
        // Java: while(!isWhite)
        while !isWhite {
            // Java: int current = this.nextLesserThan8Bits(4);
            let mut current = self.nextLesserThan8Bits(4);
            // Java: int entry = initBlack[current];
            let mut entry = initBlack[current as usize] as i32;
            // Java: int isT = entry & 1;
            let mut isT = entry & 1;
            // Java: int bits = entry >>> 1 & 15;
            let mut bits = ushr(entry, 1) & 15;
            // Java: code = entry >>> 5 & 2047;
            code = ushr(entry, 5) & 2047;
            // Java: if (code == 100)
            if code == 100 {
                // Java: current = this.nextNBits(9);
                current = self.nextNBits(9);
                // Java: entry = black[current];
                entry = black[current as usize] as i32;
                // Java: isT = entry & 1;
                isT = entry & 1;
                // Java: bits = entry >>> 1 & 15;
                bits = ushr(entry, 1) & 15;
                // Java: code = entry >>> 5 & 2047;
                code = ushr(entry, 5) & 2047;
                // Java: if (bits == 12)
                if bits == 12 {
                    // Java: this.updatePointer(5);
                    self.updatePointer(5);
                    // Java: current = this.nextLesserThan8Bits(4);
                    current = self.nextLesserThan8Bits(4);
                    // Java: entry = additionalMakeup[current];
                    entry = additionalMakeup[current as usize] as i32;
                    // Java: bits = entry >>> 1 & 7;
                    bits = ushr(entry, 1) & 7;
                    // Java: code = entry >>> 4 & 4095;
                    code = ushr(entry, 4) & 4095;
                    // Java: runLength += code;
                    runLength += code;
                    // Java: this.updatePointer(4 - bits);
                    self.updatePointer(4 - bits);
                }
                // Java: else
                else {
                    // Java: if (bits == 15)
                    if bits == 15 {
                        // Java: Log.e(TAG, "TIFFFaxDecoder2: code error: " + code);
                        eprintln!("{}: TIFFFaxDecoder2: code error: {}", TAG, code);
                        // Java: throw new RuntimeException("TIFFFaxDecoder2");
                        panic!("TIFFFaxDecoder2");
                    }
                    // Java: runLength += code;
                    runLength += code;
                    // Java: this.updatePointer(9 - bits);
                    self.updatePointer(9 - bits);
                    // Java: if (isT == 0)
                    if isT == 0 {
                        // Java: isWhite = true;
                        isWhite = true;
                    }
                }
            }
            // Java: else if (code == 200)
            else if code == 200 {
                // Java: current = this.nextLesserThan8Bits(2);
                current = self.nextLesserThan8Bits(2);
                // Java: entry = twoBitBlack[current];
                entry = twoBitBlack[current as usize] as i32;
                // Java: code = entry >>> 5 & 2047;
                code = ushr(entry, 5) & 2047;
                // Java: runLength += code;
                runLength += code;
                // Java: bits = entry >>> 1 & 15;
                bits = ushr(entry, 1) & 15;
                // Java: this.updatePointer(2 - bits);
                self.updatePointer(2 - bits);
                // Java: isWhite = true;
                isWhite = true;
            }
            // Java: else
            else {
                // Java: runLength += code;
                runLength += code;
                // Java: this.updatePointer(4 - bits);
                self.updatePointer(4 - bits);
                // Java: isWhite = true;
                isWhite = true;
            }
        }
        // Java: return runLength;
        runLength
    }

    // Java: int readEOL()
    fn readEOL(&mut self) -> i32 {
        // Java: if (this.fillBits == 0)
        if self.fillBits == 0 {
            // Java: int checkn;
            // Java: if ((checkn = this.nextNBits(12)) != 1)
            let checkn = self.nextNBits(12);
            if checkn != 1 {
                // Java: Log.e(TAG, "TIFFFaxDecoder6: error: 1 != " + checkn);
                eprintln!("{}: TIFFFaxDecoder6: error: 1 != {}", TAG, checkn);
                // Java: throw new RuntimeException("TIFFFaxDecoder6");
                panic!("TIFFFaxDecoder6");
            }
        }
        // Java: else if (this.fillBits == 1)
        else if self.fillBits == 1 {
            // Java: int bitsLeft = 8 - this.bitPointer;
            let bitsLeft = 8 - self.bitPointer;
            // Java: int checkn;
            // Java: if ((checkn = this.nextNBits(bitsLeft)) != 0)
            let mut checkn = self.nextNBits(bitsLeft);
            if checkn != 0 {
                // Java: Log.e(TAG, "TIFFFaxDecoder8: error: 0 != " + checkn);
                eprintln!("{}: TIFFFaxDecoder8: error: 0 != {}", TAG, checkn);
                // Java: throw new RuntimeException("TIFFFaxDecoder8");
                panic!("TIFFFaxDecoder8");
            }
            // Java: if (bitsLeft < 4 && (checkn = this.nextNBits(8)) != 0)
            if bitsLeft < 4 {
                checkn = self.nextNBits(8);
                if checkn != 0 {
                    // Java: Log.e(TAG, "TIFFFaxDecoder8: error: 0 != " + checkn);
                    eprintln!("{}: TIFFFaxDecoder8: error: 0 != {}", TAG, checkn);
                    // Java: throw new RuntimeException("TIFFFaxDecoder8");
                    panic!("TIFFFaxDecoder8");
                }
            }
            // Java: while((checkn = this.nextNBits(8)) != 1)
            loop {
                checkn = self.nextNBits(8);
                if checkn == 1 { break; }
                // Java: if (checkn != 0)
                if checkn != 0 {
                    // Java: Log.e(TAG, "TIFFFaxDecoder8: error: 0 !=" + checkn);
                    eprintln!("{}: TIFFFaxDecoder8: error: 0 !={}", TAG, checkn);
                    // Java: throw new RuntimeException("TIFFFaxDecoder8");
                    panic!("TIFFFaxDecoder8");
                }
            }
        }
        // Java: return this.oneD == 0 ? 1 : this.nextLesserThan8Bits(1);
        if self.oneD == 0 { 1 } else { self.nextLesserThan8Bits(1) }
    }

    // Java: void getNextChangingElement(int a0, boolean isWhite, int[] ret)
    fn getNextChangingElement(&mut self, a0: i32, isWhite: bool, ret: &mut [i32; 2]) {
        // Java: int[] pce = this.prevChangingElems;
        // Java: int ces = this.changingElemSize;
        let ces = self.changingElemSize;
        // Java: int start = this.lastChangingElement > 0 ? this.lastChangingElement - 1 : 0;
        let mut start = if self.lastChangingElement > 0 { self.lastChangingElement - 1 } else { 0 };
        // Java: if (isWhite)
        if isWhite {
            // Java: start &= -2;
            start &= !1;
        }
        // Java: else
        else {
            // Java: start |= 1;
            start |= 1;
        }
        // Java: int i;
        let mut i = start;
        // Java: for(i = start; i < ces; i += 2)
        while i < ces {
            // Java: int temp = pce[i];
            let temp = self.prevChangingElems[i as usize];
            // Java: if (temp > a0)
            if temp > a0 {
                // Java: this.lastChangingElement = i;
                self.lastChangingElement = i;
                // Java: ret[0] = temp;
                ret[0] = temp;
                // Java: break;
                break;
            }
            i += 2;
        }
        // Java: if (i + 1 < ces)
        if i + 1 < ces {
            // Java: ret[1] = pce[i + 1];
            ret[1] = self.prevChangingElems[(i + 1) as usize];
        }
    }

    // Java: int nextNBits(int bitsToGet)
    fn nextNBits(&mut self, bitsToGet: i32) -> i32 {
        // Java: int l = this.data.length - 1;
        let l = self.data.len() as i32 - 1;
        // Java: int bp = this.bytePointer;
        let bp = self.bytePointer as usize;
        // Java: byte b;
        let b: i32;
        // Java: byte next;
        let next: i32;
        // Java: byte next2next;
        let next2next: i32;
        // Java: if (this.fillOrder == 1)
        if self.fillOrder == 1 {
            // Java: b = this.data[bp];
            b = self.data[bp] as i8 as i32;
            // Java: if (bp == l)
            if self.bytePointer == l {
                // Java: next = 0;
                next = 0;
                // Java: next2next = 0;
                next2next = 0;
            }
            // Java: else if (bp + 1 == l)
            else if self.bytePointer + 1 == l {
                // Java: next = this.data[bp + 1];
                next = self.data[bp + 1] as i8 as i32;
                // Java: next2next = 0;
                next2next = 0;
            }
            // Java: else
            else {
                // Java: next = this.data[bp + 1];
                next = self.data[bp + 1] as i8 as i32;
                // Java: next2next = this.data[bp + 2];
                next2next = self.data[bp + 2] as i8 as i32;
            }
        }
        // Java: else
        else {
            // Java: if (this.fillOrder != 2)
            if self.fillOrder != 2 {
                // Java: Log.e(TAG, "TIFFFaxDecoder7: error: fillOrder:" + this.fillOrder);
                eprintln!("{}: TIFFFaxDecoder7: error: fillOrder:{}", TAG, self.fillOrder);
                // Java: throw new RuntimeException("TIFFFaxDecoder7");
                panic!("TIFFFaxDecoder7");
            }
            // Java: b = flipTable[this.data[bp] & 255];
            b = flipTable[(self.data[bp] & 255) as usize] as i32;
            // Java: if (bp == l)
            if self.bytePointer == l {
                // Java: next = 0;
                next = 0;
                // Java: next2next = 0;
                next2next = 0;
            }
            // Java: else if (bp + 1 == l)
            else if self.bytePointer + 1 == l {
                // Java: next = flipTable[this.data[bp + 1] & 255];
                next = flipTable[(self.data[bp + 1] & 255) as usize] as i32;
                // Java: next2next = 0;
                next2next = 0;
            }
            // Java: else
            else {
                // Java: next = flipTable[this.data[bp + 1] & 255];
                next = flipTable[(self.data[bp + 1] & 255) as usize] as i32;
                // Java: next2next = flipTable[this.data[bp + 2] & 255];
                next2next = flipTable[(self.data[bp + 2] & 255) as usize] as i32;
            }
        }
        // Java: int bitsLeft = 8 - this.bitPointer;
        let bitsLeft = 8 - self.bitPointer;
        // Java: int bitsFromNextByte = bitsToGet - bitsLeft;
        let mut bitsFromNextByte = bitsToGet - bitsLeft;
        // Java: int bitsFromNext2NextByte = 0;
        let mut bitsFromNext2NextByte = 0;
        // Java: if (bitsFromNextByte > 8)
        if bitsFromNextByte > 8 {
            // Java: bitsFromNext2NextByte = bitsFromNextByte - 8;
            bitsFromNext2NextByte = bitsFromNextByte - 8;
            // Java: bitsFromNextByte = 8;
            bitsFromNextByte = 8;
        }
        // Java: ++this.bytePointer;
        self.bytePointer += 1;
        // Java: int i1 = (b & table1[bitsLeft]) << bitsToGet - bitsLeft;
        let i1 = (b & table1[bitsLeft as usize]) << (bitsToGet - bitsLeft);
        // Java: int i2 = (next & table2[bitsFromNextByte]) >>> 8 - bitsFromNextByte;
        let mut i2 = ushr(next & table2[bitsFromNextByte as usize], 8 - bitsFromNextByte);
        // Java: int i3 = 0;
        // Java: if (bitsFromNext2NextByte != 0)
        if bitsFromNext2NextByte != 0 {
            // Java: i2 <<= bitsFromNext2NextByte;
            i2 <<= bitsFromNext2NextByte;
            // Java: i3 = (next2next & table2[bitsFromNext2NextByte]) >>> 8 - bitsFromNext2NextByte;
            let i3 = ushr(next2next & table2[bitsFromNext2NextByte as usize], 8 - bitsFromNext2NextByte);
            // Java: i2 |= i3;
            i2 |= i3;
            // Java: ++this.bytePointer;
            self.bytePointer += 1;
            // Java: this.bitPointer = bitsFromNext2NextByte;
            self.bitPointer = bitsFromNext2NextByte;
        }
        // Java: else if (bitsFromNextByte == 8)
        else if bitsFromNextByte == 8 {
            // Java: this.bitPointer = 0;
            self.bitPointer = 0;
            // Java: ++this.bytePointer;
            self.bytePointer += 1;
        }
        // Java: else
        else {
            // Java: this.bitPointer = bitsFromNextByte;
            self.bitPointer = bitsFromNextByte;
        }
        // Java: int i = i1 | i2;
        // Java: return i;
        i1 | i2
    }

    // Java: int nextLesserThan8Bits(int bitsToGet)
    fn nextLesserThan8Bits(&mut self, bitsToGet: i32) -> i32 {
        // Java: int l = this.data.length - 1;
        let l = self.data.len() as i32 - 1;
        // Java: int bp = this.bytePointer;
        let bp = self.bytePointer as usize;
        // Java: byte b;
        let b: i32;
        // Java: byte next;
        let next: i32;
        // Java: if (this.fillOrder == 1)
        if self.fillOrder == 1 {
            // Java: b = this.data[bp];
            b = self.data[bp] as i8 as i32;
            // Java: if (bp == l)
            if self.bytePointer == l {
                // Java: next = 0;
                next = 0;
            }
            // Java: else
            else {
                // Java: next = this.data[bp + 1];
                next = self.data[bp + 1] as i8 as i32;
            }
        }
        // Java: else
        else {
            // Java: if (this.fillOrder != 2)
            if self.fillOrder != 2 {
                // Java: Log.e(TAG, "TIFFFaxDecoder7: error: fillOrder:" + this.fillOrder);
                eprintln!("{}: TIFFFaxDecoder7: error: fillOrder:{}", TAG, self.fillOrder);
                // Java: throw new RuntimeException("TIFFFaxDecoder7");
                panic!("TIFFFaxDecoder7");
            }
            // Java: b = flipTable[this.data[bp] & 255];
            b = flipTable[(self.data[bp] & 255) as usize] as i32;
            // Java: if (bp == l)
            if self.bytePointer == l {
                // Java: next = 0;
                next = 0;
            }
            // Java: else
            else {
                // Java: next = flipTable[this.data[bp + 1] & 255];
                next = flipTable[(self.data[bp + 1] & 255) as usize] as i32;
            }
        }
        // Java: int bitsLeft = 8 - this.bitPointer;
        let bitsLeft = 8 - self.bitPointer;
        // Java: int bitsFromNextByte = bitsToGet - bitsLeft;
        let bitsFromNextByte = bitsToGet - bitsLeft;
        // Java: int shift = bitsLeft - bitsToGet;
        let shift = bitsLeft - bitsToGet;
        // Java: int i1;
        let i1: i32;
        // Java: if (shift >= 0)
        if shift >= 0 {
            // Java: i1 = (b & table1[bitsLeft]) >>> shift;
            i1 = ushr(b & table1[bitsLeft as usize], shift);
            // Java: this.bitPointer += bitsToGet;
            self.bitPointer += bitsToGet;
            // Java: if (this.bitPointer == 8)
            if self.bitPointer == 8 {
                // Java: this.bitPointer = 0;
                self.bitPointer = 0;
                // Java: ++this.bytePointer;
                self.bytePointer += 1;
            }
        }
        // Java: else
        else {
            // Java: i1 = (b & table1[bitsLeft]) << -shift;
            let mut i1_val = (b & table1[bitsLeft as usize]) << (-shift);
            // Java: int i2 = (next & table2[bitsFromNextByte]) >>> 8 - bitsFromNextByte;
            let i2 = ushr(next & table2[bitsFromNextByte as usize], 8 - bitsFromNextByte);
            // Java: i1 |= i2;
            i1_val |= i2;
            i1 = i1_val;
            // Java: ++this.bytePointer;
            self.bytePointer += 1;
            // Java: this.bitPointer = bitsFromNextByte;
            self.bitPointer = bitsFromNextByte;
        }
        // Java: return i1;
        i1
    }

    // Java: void updatePointer(int bitsToMoveBack)
    fn updatePointer(&mut self, bitsToMoveBack: i32) {
        // Java: int i = this.bitPointer - bitsToMoveBack;
        let i = self.bitPointer - bitsToMoveBack;
        // Java: if (i < 0)
        if i < 0 {
            // Java: --this.bytePointer;
            self.bytePointer -= 1;
            // Java: this.bitPointer = 8 + i;
            self.bitPointer = 8 + i;
        }
        // Java: else
        else {
            // Java: this.bitPointer = i;
            self.bitPointer = i;
        }
    }

    // Java: boolean advancePointer()
    fn advancePointer(&mut self) -> bool {
        // Java: if (this.bitPointer != 0)
        if self.bitPointer != 0 {
            // Java: ++this.bytePointer;
            self.bytePointer += 1;
            // Java: this.bitPointer = 0;
            self.bitPointer = 0;
        }
        // Java: return true;
        true
    }
}
