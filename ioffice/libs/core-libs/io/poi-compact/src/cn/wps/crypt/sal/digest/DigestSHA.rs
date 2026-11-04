// Java: import org.apache.poi.util.IntegerField
// Java: import org.apache.poi.util.LittleEndian

use super::DigestError;
use super::IDigest::IDigest;
use std::cell::Cell;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32)
        | ((data[offset + 1] as i32) << 8)
        | ((data[offset + 2] as i32) << 16)
        | ((data[offset + 3] as i32) << 24)
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

fn le_put_byte(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class DigestSHA implements IDigest
pub struct DigestSHA {
    // Java field: int DIGEST_LENGTH_SHA
    // Java field: int DIGEST_CBLOCK_SHA
    // Java field: int DIGEST_LBLOCK_SHA
    // Java field: int K_00_19
    // Java field: int K_20_39
    // Java field: int K_40_59
    // Java field: int K_60_79
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: int mLength
    mLength: i32,
    // Java field: int mDataLen
    mDataLen: i32,
    // Java field: byte[] mData
    mData: Vec<u8>,
    // Java field: int mA
    mA: i32,
    // Java field: int mB
    mB: i32,
    // Java field: int mC
    mC: i32,
    // Java field: int mD
    mD: i32,
    // Java field: int mE
    mE: i32,
    // Java field: int mL
    mL: i32,
    // Java field: int mH
    mH: i32,
}

const DIGEST_LENGTH_SHA: i32 = 20;
const DIGEST_CBLOCK_SHA: i32 = 64;
const DIGEST_LBLOCK_SHA: i32 = 16;
const K_00_19: i32 = 1518500249;
const K_20_39: i32 = 1859775393;
const K_40_59: i32 = -1894007588i32;
const K_60_79: i32 = -899497514i32;

// Java: public class DigestSHA implements IDigest
impl DigestSHA {
    // Java: DigestSHA()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mData = new byte[DIGEST_LBLOCK_SHA * 4];
        let mut s = DigestSHA {
            mAlgorithm: 0,
            mLength: 0,
            mDataLen: 0,
            mData: vec![0u8; (DIGEST_LBLOCK_SHA * 4) as usize],
            mA: 0,
            mB: 0,
            mC: 0,
            mD: 0,
            mE: 0,
            mL: 0,
            mH: 0,
        };
        // Java: this.init();
        s.init_state();
        s
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: this.init();
        self.init_state();
    }

    // Java: int get(byte[] buffer)
    pub fn get_impl(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        // Java: return DigestError.Digest_E_Argument;
        // Java: else if (this.mAlgorithm != 2 && this.mAlgorithm != 3)
        if self.mAlgorithm != 2 && self.mAlgorithm != 3 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else if (this.mLength > buffer.length)
        else if self.mLength > buffer.len() as i32 {
            // Java: return DigestError.Digest_E_BufferSize;
            return DigestError::DIGEST_E_BUFFER_SIZE;
        }
        // Java: else
        else {
            // Java: this.end();
            self.end();
            // Java: LittleEndian.putByte(buffer, 0, this.mA);
            le_put_byte(buffer, 0, self.mA);
            // Java: LittleEndian.putByte(buffer, 4, this.mB);
            le_put_byte(buffer, 4, self.mB);
            // Java: LittleEndian.putByte(buffer, 8, this.mC);
            le_put_byte(buffer, 8, self.mC);
            // Java: LittleEndian.putByte(buffer, 12, this.mD);
            le_put_byte(buffer, 12, self.mD);
            // Java: LittleEndian.putByte(buffer, 16, this.mE);
            le_put_byte(buffer, 16, self.mE);
            // Java: this.init();
            self.init_state();
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int get(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn get_with_params_impl(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: this.init();
        self.init_state();
        // Java: int result = this.update(data, offset, nLen);
        let mut result = self.update_data(data, offset, nLen);
        // Java: if (result == DigestError.Digest_E_None)
        if result == DigestError::DIGEST_E_NONE {
            // Java: result = this.get(buffer);
            result = self.get_impl(buffer);
        }
        // Java: return result;
        result
    }

    // Java: int init(byte[] data, int offset, int nLen)
    pub fn init_with_data(&mut self, _data: &[u8], _offset: i32, _nLen: i32) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int raw(byte[] buffer)
    pub fn raw_impl(&mut self, _buffer: &mut [u8]) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int update(byte[] data, int offset, int nLen)
    pub fn update_data(&mut self, data: &[u8], mut offset: i32, mut nLen: i32) -> i32 {
        // Java: if (data == null)
        // Java: return DigestError.Digest_E_Argument;
        // Java: else if (this.mAlgorithm != 2 && this.mAlgorithm != 3)
        if self.mAlgorithm != 2 && self.mAlgorithm != 3 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else if (nLen == 0)
        else if nLen == 0 {
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
        // Java: else
        else {
            // Java: int len = this.mL + (nLen << 3);
            let len = self.mL.wrapping_add(nLen << 3);
            // Java: if (len < this.mL)
            if len < self.mL {
                // Java: ++this.mH;
                self.mH += 1;
            }
            // Java: this.mH += nLen >> 29;
            self.mH += nLen >> 29;
            // Java: this.mL = len;
            self.mL = len;
            // Java: if (this.mDataLen > 0)
            if self.mDataLen > 0 {
                // Java: int n = DIGEST_CBLOCK_SHA - this.mDataLen;
                let n = DIGEST_CBLOCK_SHA - self.mDataLen;
                // Java: if (nLen < n)
                if nLen < n {
                    // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, nLen);
                    let dst = self.mDataLen as usize;
                    let src = offset as usize;
                    let l = nLen as usize;
                    self.mData[dst..dst + l].copy_from_slice(&data[src..src + l]);
                    // Java: this.mDataLen += nLen;
                    self.mDataLen += nLen;
                    // Java: return DigestError.Digest_E_None;
                    return DigestError::DIGEST_E_NONE;
                }
                // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, n);
                let dst = self.mDataLen as usize;
                let src = offset as usize;
                let l = n as usize;
                self.mData[dst..dst + l].copy_from_slice(&data[src..src + l]);
                // Java: offset += n;
                offset += n;
                // Java: nLen -= n;
                nLen -= n;
                // Java: this.update();
                self.update_block();
                // Java: this.mDataLen = 0;
                self.mDataLen = 0;
            }
            // Java: while(nLen >= DIGEST_CBLOCK_SHA)
            while nLen >= DIGEST_CBLOCK_SHA {
                // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, DIGEST_CBLOCK_SHA);
                let dst = self.mDataLen as usize;
                let src = offset as usize;
                let l = DIGEST_CBLOCK_SHA as usize;
                self.mData[dst..dst + l].copy_from_slice(&data[src..src + l]);
                // Java: offset += DIGEST_CBLOCK_SHA;
                offset += DIGEST_CBLOCK_SHA;
                // Java: nLen -= DIGEST_CBLOCK_SHA;
                nLen -= DIGEST_CBLOCK_SHA;
                // Java: this.update();
                self.update_block();
            }
            // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, nLen);
            let dst = self.mDataLen as usize;
            let src = offset as usize;
            let l = nLen as usize;
            self.mData[dst..dst + l].copy_from_slice(&data[src..src + l]);
            // Java: this.mDataLen = nLen;
            self.mDataLen = nLen;
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return DIGEST_LENGTH_SHA;
        DIGEST_LENGTH_SHA
    }

    // Java: void init()
    fn init_state(&mut self) {
        // Java: this.mAlgorithm = 2;
        self.mAlgorithm = 2;
        // Java: this.mLength = DIGEST_LENGTH_SHA;
        self.mLength = DIGEST_LENGTH_SHA;
        // Java: this.mDataLen = 0;
        self.mDataLen = 0;
        // Java: this.mA = 1732584193;
        self.mA = 1732584193;
        // Java: this.mB = -271733879;
        self.mB = -271733879i32;
        // Java: this.mC = -1732584194;
        self.mC = -1732584194i32;
        // Java: this.mD = 271733878;
        self.mD = 271733878;
        // Java: this.mE = -1009589776;
        self.mE = -1009589776i32;
        // Java: this.mL = 0;
        self.mL = 0;
        // Java: this.mH = 0;
        self.mH = 0;
        // Java: Arrays.fill(this.mData, (byte)0);
        self.mData.fill(0);
    }

    // Java: void update()
    fn update_block(&mut self) {
        // Java: IntegerField A = new IntegerField(0, this.mA);
        let a = Cell::new(self.mA);
        // Java: IntegerField B = new IntegerField(0, this.mB);
        let b = Cell::new(self.mB);
        // Java: IntegerField C = new IntegerField(0, this.mC);
        let c = Cell::new(self.mC);
        // Java: IntegerField D = new IntegerField(0, this.mD);
        let d = Cell::new(self.mD);
        // Java: IntegerField E = new IntegerField(0, this.mE);
        let e = Cell::new(self.mE);
        // Java: IntegerField T = new IntegerField(0, 0);
        let t = Cell::new(0i32);
        // Java: this.BODY_00_15(0, A, B, C, D, E, T);
        self.body_00_15(0, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_00_15(1, T, A, B, C, D, E);
        self.body_00_15(1, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_00_15(2, E, T, A, B, C, D);
        self.body_00_15(2, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_00_15(3, D, E, T, A, B, C);
        self.body_00_15(3, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_00_15(4, C, D, E, T, A, B);
        self.body_00_15(4, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_00_15(5, B, C, D, E, T, A);
        self.body_00_15(5, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_00_15(6, A, B, C, D, E, T);
        self.body_00_15(6, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_00_15(7, T, A, B, C, D, E);
        self.body_00_15(7, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_00_15(8, E, T, A, B, C, D);
        self.body_00_15(8, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_00_15(9, D, E, T, A, B, C);
        self.body_00_15(9, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_00_15(10, C, D, E, T, A, B);
        self.body_00_15(10, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_00_15(11, B, C, D, E, T, A);
        self.body_00_15(11, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_00_15(12, A, B, C, D, E, T);
        self.body_00_15(12, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_00_15(13, T, A, B, C, D, E);
        self.body_00_15(13, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_00_15(14, E, T, A, B, C, D);
        self.body_00_15(14, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_00_15(15, D, E, T, A, B, C);
        self.body_00_15(15, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_16_19(16, C, D, E, T, A, B);
        self.body_16_19(16, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_16_19(17, B, C, D, E, T, A);
        self.body_16_19(17, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_16_19(18, A, B, C, D, E, T);
        self.body_16_19(18, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_16_19(19, T, A, B, C, D, E);
        self.body_16_19(19, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_20_39(20, E, T, A, B, C, D);
        self.body_20_39(20, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_20_39(21, D, E, T, A, B, C);
        self.body_20_39(21, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_20_39(22, C, D, E, T, A, B);
        self.body_20_39(22, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_20_39(23, B, C, D, E, T, A);
        self.body_20_39(23, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_20_39(24, A, B, C, D, E, T);
        self.body_20_39(24, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_20_39(25, T, A, B, C, D, E);
        self.body_20_39(25, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_20_39(26, E, T, A, B, C, D);
        self.body_20_39(26, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_20_39(27, D, E, T, A, B, C);
        self.body_20_39(27, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_20_39(28, C, D, E, T, A, B);
        self.body_20_39(28, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_20_39(29, B, C, D, E, T, A);
        self.body_20_39(29, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_20_39(30, A, B, C, D, E, T);
        self.body_20_39(30, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_20_39(31, T, A, B, C, D, E);
        self.body_20_39(31, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_20_39(32, E, T, A, B, C, D);
        self.body_20_39(32, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_20_39(33, D, E, T, A, B, C);
        self.body_20_39(33, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_20_39(34, C, D, E, T, A, B);
        self.body_20_39(34, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_20_39(35, B, C, D, E, T, A);
        self.body_20_39(35, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_20_39(36, A, B, C, D, E, T);
        self.body_20_39(36, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_20_39(37, T, A, B, C, D, E);
        self.body_20_39(37, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_20_39(38, E, T, A, B, C, D);
        self.body_20_39(38, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_20_39(39, D, E, T, A, B, C);
        self.body_20_39(39, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_40_59(40, C, D, E, T, A, B);
        self.body_40_59(40, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_40_59(41, B, C, D, E, T, A);
        self.body_40_59(41, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_40_59(42, A, B, C, D, E, T);
        self.body_40_59(42, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_40_59(43, T, A, B, C, D, E);
        self.body_40_59(43, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_40_59(44, E, T, A, B, C, D);
        self.body_40_59(44, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_40_59(45, D, E, T, A, B, C);
        self.body_40_59(45, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_40_59(46, C, D, E, T, A, B);
        self.body_40_59(46, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_40_59(47, B, C, D, E, T, A);
        self.body_40_59(47, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_40_59(48, A, B, C, D, E, T);
        self.body_40_59(48, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_40_59(49, T, A, B, C, D, E);
        self.body_40_59(49, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_40_59(50, E, T, A, B, C, D);
        self.body_40_59(50, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_40_59(51, D, E, T, A, B, C);
        self.body_40_59(51, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_40_59(52, C, D, E, T, A, B);
        self.body_40_59(52, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_40_59(53, B, C, D, E, T, A);
        self.body_40_59(53, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_40_59(54, A, B, C, D, E, T);
        self.body_40_59(54, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_40_59(55, T, A, B, C, D, E);
        self.body_40_59(55, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_40_59(56, E, T, A, B, C, D);
        self.body_40_59(56, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_40_59(57, D, E, T, A, B, C);
        self.body_40_59(57, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_40_59(58, C, D, E, T, A, B);
        self.body_40_59(58, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_40_59(59, B, C, D, E, T, A);
        self.body_40_59(59, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_60_79(60, A, B, C, D, E, T);
        self.body_60_79(60, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_60_79(61, T, A, B, C, D, E);
        self.body_60_79(61, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_60_79(62, E, T, A, B, C, D);
        self.body_60_79(62, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_60_79(63, D, E, T, A, B, C);
        self.body_60_79(63, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_60_79(64, C, D, E, T, A, B);
        self.body_60_79(64, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_60_79(65, B, C, D, E, T, A);
        self.body_60_79(65, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_60_79(66, A, B, C, D, E, T);
        self.body_60_79(66, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_60_79(67, T, A, B, C, D, E);
        self.body_60_79(67, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_60_79(68, E, T, A, B, C, D);
        self.body_60_79(68, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_60_79(69, D, E, T, A, B, C);
        self.body_60_79(69, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_60_79(70, C, D, E, T, A, B);
        self.body_60_79(70, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_60_79(71, B, C, D, E, T, A);
        self.body_60_79(71, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_60_79(72, A, B, C, D, E, T);
        self.body_60_79(72, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_60_79(73, T, A, B, C, D, E);
        self.body_60_79(73, &t, &a, &b, &c, &d, &e);
        // Java: this.BODY_60_79(74, E, T, A, B, C, D);
        self.body_60_79(74, &e, &t, &a, &b, &c, &d);
        // Java: this.BODY_60_79(75, D, E, T, A, B, C);
        self.body_60_79(75, &d, &e, &t, &a, &b, &c);
        // Java: this.BODY_60_79(76, C, D, E, T, A, B);
        self.body_60_79(76, &c, &d, &e, &t, &a, &b);
        // Java: this.BODY_60_79(77, B, C, D, E, T, A);
        self.body_60_79(77, &b, &c, &d, &e, &t, &a);
        // Java: this.BODY_60_79(78, A, B, C, D, E, T);
        self.body_60_79(78, &a, &b, &c, &d, &e, &t);
        // Java: this.BODY_60_79(79, T, A, B, C, D, E);
        self.body_60_79(79, &t, &a, &b, &c, &d, &e);
        // Java: this.mA = (int)(((long)this.mA & 4294967295L) + ((long)E.get() & 4294967295L) & -1L);
        self.mA = (self.mA as u32).wrapping_add(e.get() as u32) as i32;
        // Java: this.mB = (int)(((long)this.mB & 4294967295L) + ((long)T.get() & 4294967295L) & -1L);
        self.mB = (self.mB as u32).wrapping_add(t.get() as u32) as i32;
        // Java: this.mC = (int)(((long)this.mC & 4294967295L) + ((long)A.get() & 4294967295L) & -1L);
        self.mC = (self.mC as u32).wrapping_add(a.get() as u32) as i32;
        // Java: this.mD = (int)(((long)this.mD & 4294967295L) + ((long)B.get() & 4294967295L) & -1L);
        self.mD = (self.mD as u32).wrapping_add(b.get() as u32) as i32;
        // Java: this.mE = (int)(((long)this.mD & 4294967295L) + ((long)C.get() & 4294967295L) & -1L);
        self.mE = (self.mD as u32).wrapping_add(c.get() as u32) as i32;
    }

    // Java: int BODY_X(int i)
    fn body_x(&self, i: i32) -> i32 {
        // Java: return this.mData[i & 15] ^ this.mData[i + 2 & 15] ^ this.mData[i + 8 & 15] ^ this.mData[i + 13 & 15];
        let d = |idx: i32| -> i32 { le_get_int(&self.mData, ((idx & 15) * 4) as usize) };
        d(i) ^ d(i + 2) ^ d(i + 8) ^ d(i + 13)
    }

    // Java: void BODY_00_15(int i, IntegerField a, IntegerField b, IntegerField c, IntegerField d, IntegerField e, IntegerField f)
    fn body_00_15(&self, i: i32, a: &Cell<i32>, b: &Cell<i32>, c: &Cell<i32>, d: &Cell<i32>, e: &Cell<i32>, f: &Cell<i32>) {
        // Java: f.set(LittleEndian.getInt(this.mData, i * 4));
        f.set(le_get_int(&self.mData, (i * 4) as usize));
        // Java: f.set((int)(((long)f.get() & 4294967295L) + ((long)e.get() & 4294967295L) + (long)K_00_19 + (long)this.RTL_DIGEST_ROTL(a.get(), 5) + (long)this.F_00_19(b.get(), c.get(), d.get())));
        let val = (f.get() as u32 as i64)
            .wrapping_add(e.get() as u32 as i64)
            .wrapping_add(K_00_19 as i64)
            .wrapping_add(self.rtl_digest_rotl(a.get(), 5) as i64)
            .wrapping_add(self.f_00_19(b.get(), c.get(), d.get()) as i64);
        f.set(val as i32);
        // Java: b.set(this.RTL_DIGEST_ROTL(b.get(), 30));
        b.set(self.rtl_digest_rotl(b.get(), 30));
    }

    // Java: void BODY_16_19(int i, IntegerField a, IntegerField b, IntegerField c, IntegerField d, IntegerField e, IntegerField f)
    fn body_16_19(&mut self, i: i32, a: &Cell<i32>, b: &Cell<i32>, c: &Cell<i32>, d: &Cell<i32>, e: &Cell<i32>, f: &Cell<i32>) {
        // Java: f.set(this.BODY_X(i));
        f.set(self.body_x(i));
        // Java: f.set(this.update(f.get()));
        f.set(self.update_val(f.get()));
        // Java: LittleEndian.putInt(this.mData, (i & 15) * 4, f.get());
        le_put_int(&mut self.mData, ((i & 15) * 4) as usize, f.get());
        // Java: f.set((int)(((long)f.get() & 4294967295L) + ((long)e.get() & 4294967295L) + (long)K_00_19 + (long)this.RTL_DIGEST_ROTL(a.get(), 5) + (long)this.F_00_19(b.get(), c.get(), d.get())));
        let val = (f.get() as u32 as i64)
            .wrapping_add(e.get() as u32 as i64)
            .wrapping_add(K_00_19 as i64)
            .wrapping_add(self.rtl_digest_rotl(a.get(), 5) as i64)
            .wrapping_add(self.f_00_19(b.get(), c.get(), d.get()) as i64);
        f.set(val as i32);
        // Java: b.set(this.RTL_DIGEST_ROTL(b.get(), 30));
        b.set(self.rtl_digest_rotl(b.get(), 30));
    }

    // Java: void BODY_20_39(int i, IntegerField a, IntegerField b, IntegerField c, IntegerField d, IntegerField e, IntegerField f)
    fn body_20_39(&mut self, i: i32, a: &Cell<i32>, b: &Cell<i32>, c: &Cell<i32>, d: &Cell<i32>, e: &Cell<i32>, f: &Cell<i32>) {
        // Java: f.set(this.BODY_X(i));
        f.set(self.body_x(i));
        // Java: f.set(this.update(f.get()));
        f.set(self.update_val(f.get()));
        // Java: LittleEndian.putInt(this.mData, (i & 15) * 4, f.get());
        le_put_int(&mut self.mData, ((i & 15) * 4) as usize, f.get());
        // Java: f.set((int)(((long)f.get() & 4294967295L) + ((long)e.get() & 4294967295L) + (long)K_20_39 + (long)this.RTL_DIGEST_ROTL(a.get(), 5) + (long)this.F_20_39(b.get(), c.get(), d.get())));
        let val = (f.get() as u32 as i64)
            .wrapping_add(e.get() as u32 as i64)
            .wrapping_add(K_20_39 as i64)
            .wrapping_add(self.rtl_digest_rotl(a.get(), 5) as i64)
            .wrapping_add(self.f_20_39(b.get(), c.get(), d.get()) as i64);
        f.set(val as i32);
        // Java: b.set(this.RTL_DIGEST_ROTL(b.get(), 30));
        b.set(self.rtl_digest_rotl(b.get(), 30));
    }

    // Java: void BODY_40_59(int i, IntegerField a, IntegerField b, IntegerField c, IntegerField d, IntegerField e, IntegerField f)
    fn body_40_59(&mut self, i: i32, a: &Cell<i32>, b: &Cell<i32>, c: &Cell<i32>, d: &Cell<i32>, e: &Cell<i32>, f: &Cell<i32>) {
        // Java: f.set(this.BODY_X(i));
        f.set(self.body_x(i));
        // Java: f.set(this.update(f.get()));
        f.set(self.update_val(f.get()));
        // Java: LittleEndian.putInt(this.mData, (i & 15) * 4, f.get());
        le_put_int(&mut self.mData, ((i & 15) * 4) as usize, f.get());
        // Java: f.set((int)(((long)f.get() & 4294967295L) + ((long)e.get() & 4294967295L) + (long)K_40_59 + (long)this.RTL_DIGEST_ROTL(a.get(), 5) + (long)this.F_40_59(b.get(), c.get(), d.get())));
        let val = (f.get() as u32 as i64)
            .wrapping_add(e.get() as u32 as i64)
            .wrapping_add(K_40_59 as i64)
            .wrapping_add(self.rtl_digest_rotl(a.get(), 5) as i64)
            .wrapping_add(self.f_40_59(b.get(), c.get(), d.get()) as i64);
        f.set(val as i32);
        // Java: b.set(this.RTL_DIGEST_ROTL(b.get(), 30));
        b.set(self.rtl_digest_rotl(b.get(), 30));
    }

    // Java: void BODY_60_79(int i, IntegerField a, IntegerField b, IntegerField c, IntegerField d, IntegerField e, IntegerField f)
    fn body_60_79(&mut self, i: i32, a: &Cell<i32>, b: &Cell<i32>, c: &Cell<i32>, d: &Cell<i32>, e: &Cell<i32>, f: &Cell<i32>) {
        // Java: f.set(this.BODY_X(i));
        f.set(self.body_x(i));
        // Java: f.set(this.update(f.get()));
        f.set(self.update_val(f.get()));
        // Java: LittleEndian.putInt(this.mData, (i & 15) * 4, f.get());
        le_put_int(&mut self.mData, ((i & 15) * 4) as usize, f.get());
        // Java: f.set((int)(((long)f.get() & 4294967295L) + ((long)e.get() & 4294967295L) + (long)K_60_79 + (long)this.RTL_DIGEST_ROTL(a.get(), 5) + (long)this.F_60_79(b.get(), c.get(), d.get())));
        let val = (f.get() as u32 as i64)
            .wrapping_add(e.get() as u32 as i64)
            .wrapping_add(K_60_79 as i64)
            .wrapping_add(self.rtl_digest_rotl(a.get(), 5) as i64)
            .wrapping_add(self.f_60_79(b.get(), c.get(), d.get()) as i64);
        f.set(val as i32);
        // Java: b.set(this.RTL_DIGEST_ROTL(b.get(), 30));
        b.set(self.rtl_digest_rotl(b.get(), 30));
    }

    // Java: int update(int x)
    fn update_val(&self, x: i32) -> i32 {
        // Java: return x;
        x
    }

    // Java: int RTL_DIGEST_ROTL(int a, int n)
    fn rtl_digest_rotl(&self, a: i32, n: i32) -> i32 {
        // Java: return (int)(((long)a & 4294967295L) << n | ((long)a & 4294967295L) >> 32 - n);
        let ua = a as u32;
        (ua << n | ua >> (32 - n)) as i32
    }

    // Java: int F_00_19(int b, int c, int d)
    fn f_00_19(&self, b: i32, c: i32, d: i32) -> i32 {
        // Java: return (int)(((long)c & 4294967295L ^ (long)d & 4294967295L) & (long)b & 4294967295L ^ (long)d & 4294967295L);
        let ub = b as u32;
        let uc = c as u32;
        let ud = d as u32;
        ((uc ^ ud) & ub ^ ud) as i32
    }

    // Java: int F_20_39(int b, int c, int d)
    fn f_20_39(&self, b: i32, c: i32, d: i32) -> i32 {
        // Java: return (int)((long)b & 4294967295L ^ (long)c & 4294967295L ^ (long)d & 4294967295L);
        let ub = b as u32;
        let uc = c as u32;
        let ud = d as u32;
        (ub ^ uc ^ ud) as i32
    }

    // Java: int F_40_59(int b, int c, int d)
    fn f_40_59(&self, b: i32, c: i32, d: i32) -> i32 {
        // Java: return (int)((long)b & 4294967295L & (long)c & 4294967295L | (long)b & 4294967295L & (long)d & 4294967295L | (long)c & 4294967295L & (long)d & 4294967295L);
        let ub = b as u32;
        let uc = c as u32;
        let ud = d as u32;
        ((ub & uc) | (ub & ud) | (uc & ud)) as i32
    }

    // Java: int F_60_79(int b, int c, int d)
    fn f_60_79(&self, b: i32, c: i32, d: i32) -> i32 {
        // Java: return this.F_20_39(b, c, d);
        self.f_20_39(b, c, d)
    }

    // Java: void end()
    fn end(&mut self) {
        // Java: short[] end = new short[]{128, 0, 0, 0};
        let end: [i16; 4] = [128, 0, 0, 0];
        // Java: int i = this.mDataLen >> 2;
        let mut i = self.mDataLen >> 2;
        // Java: switch (this.mDataLen & 3)
        match self.mDataLen & 3 {
            // Java: case 1:
            1 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & 255);
                let v = le_get_int(&self.mData, (i * 4) as usize) & 255;
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: break;
            }
            // Java: case 2:
            2 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & '\uffff');
                let v = le_get_int(&self.mData, (i * 4) as usize) & 0xFFFF;
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & 16777215);
                let v = le_get_int(&self.mData, (i * 4) as usize) & 16777215;
                le_put_int(&mut self.mData, (i * 4) as usize, v);
            }
            _ => {}
        }
        // Java: switch (this.mDataLen & 3)
        match self.mDataLen & 3 {
            // Java: case 0:
            0 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, end[0] << 0);
                le_put_int(&mut self.mData, (i * 4) as usize, (end[0] as i32) << 0);
                // Java: case 1:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[1] << 8);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[1] as i32) << 8);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: case 2:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[2] << 16);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[2] as i32) << 16);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: case 3:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[3] << 24);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[3] as i32) << 24);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
            }
            // Java: case 1:
            1 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[1] << 8);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[1] as i32) << 8);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: case 2:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[2] << 16);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[2] as i32) << 16);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: case 3:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[3] << 24);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[3] as i32) << 24);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
            }
            // Java: case 2:
            2 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[2] << 16);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[2] as i32) << 16);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
                // Java: case 3:
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[3] << 24);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[3] as i32) << 24);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
            }
            // Java: case 3:
            3 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[3] << 24);
                let v = le_get_int(&self.mData, (i * 4) as usize) | ((end[3] as i32) << 24);
                le_put_int(&mut self.mData, (i * 4) as usize, v);
            }
            _ => {}
        }
        // Java: swapLong(this.mData, i + 4);
        DigestSHA::swap_long(&mut self.mData, i + 4);
        // Java: i += 4;
        i += 4;
        // Java: if (i >= DIGEST_CBLOCK_SHA - 8)
        if i >= DIGEST_CBLOCK_SHA - 8 {
            // Java: while(i < DIGEST_CBLOCK_SHA)
            while i < DIGEST_CBLOCK_SHA {
                // Java: this.mData[i] = 0;
                self.mData[i as usize] = 0;
                // Java: ++i;
                i += 1;
            }
            // Java: this.update();
            self.update_block();
            // Java: i = 0;
            i = 0;
        }
        // Java: while(i < DIGEST_CBLOCK_SHA - 8)
        while i < DIGEST_CBLOCK_SHA - 8 {
            // Java: this.mData[i] = 0;
            self.mData[i as usize] = 0;
            // Java: ++i;
            i += 1;
        }
        // Java: LittleEndian.putInt(this.mData, DIGEST_CBLOCK_SHA - 8, this.mL);
        le_put_int(&mut self.mData, (DIGEST_CBLOCK_SHA - 8) as usize, self.mL);
        // Java: LittleEndian.putInt(this.mData, DIGEST_CBLOCK_SHA - 4, this.mH);
        le_put_int(&mut self.mData, (DIGEST_CBLOCK_SHA - 4) as usize, self.mH);
        // Java: this.update();
        self.update_block();
    }

    // Java: static void swapLong(byte[] pData, int nDatLen)
    fn swap_long(pData: &mut [u8], nDatLen: i32) {
        // Java: for(int i = 4; i <= nDatLen; i += 4)
        let mut i = 4;
        while i <= nDatLen {
            // Java: byte b = pData[i - 4];
            let b = pData[(i - 4) as usize];
            // Java: pData[i - 4] = pData[i - 2];
            pData[(i - 4) as usize] = pData[(i - 2) as usize];
            // Java: pData[i - 2] = b;
            pData[(i - 2) as usize] = b;
            // Java: b = pData[i - 3];
            let b = pData[(i - 3) as usize];
            // Java: pData[i - 3] = pData[i - 1];
            pData[(i - 3) as usize] = pData[(i - 1) as usize];
            // Java: pData[i - 1] = b;
            pData[(i - 1) as usize] = b;
            i += 4;
        }
    }
}

impl IDigest for DigestSHA {
    fn init(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32 {
        self.init_with_data(var1, var2, var3)
    }

    fn update(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32 {
        self.update_data(var1, var2, var3)
    }

    fn get(&mut self, var1: &mut [u8]) -> i32 {
        self.get_impl(var1)
    }

    fn get_with_params(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.get_with_params_impl(var1, var2, var3, var4)
    }

    fn raw(&mut self, var1: &mut [u8]) -> i32 {
        self.raw_impl(var1)
    }

    fn get_length(&self) -> i32 {
        self.getLength()
    }

    fn dispose(&mut self) {
        self.dispose_impl()
    }
}
