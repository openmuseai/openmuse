use super::BaseHash::{BaseHash, BaseHashExt};
use std::sync::OnceLock;
use std::sync::Mutex;

// Java field: String MD4_HASH
pub(crate) const MD4_HASH: &str = "md4";
// Java field: int DIGEST_LENGTH
pub(crate) const DIGEST_LENGTH: i32 = 16;
// Java field: int BLOCK_LENGTH
pub(crate) const BLOCK_LENGTH: i32 = 64;
// Java field: int A
pub(crate) const A: i32 = 1732584193;
// Java field: int B
pub(crate) const B: i32 = -271733879;
// Java field: int C
pub(crate) const C: i32 = -1732584194;
// Java field: int D
pub(crate) const D: i32 = 271733878;
// Java field: String DIGEST0
pub(crate) const DIGEST0: &str = "31D6CFE0D16AE931B73C59D7E0C089C0";
// Java field: Boolean valid
static VALID: OnceLock<Mutex<Option<bool>>> = OnceLock::new();

fn read_le_i32(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32 & 0xFF)
        | ((data[offset + 1] as i32 & 0xFF) << 8)
        | ((data[offset + 2] as i32 & 0xFF) << 16)
        | ((data[offset + 3] as i32) << 24)
}

// Java: public class MD4 extends BaseHash implements Cloneable
pub struct MD4 {
    pub base: BaseHash,
    // Java field: int a
    a: i32,
    // Java field: int b
    b: i32,
    // Java field: int c
    c: i32,
    // Java field: int d
    d: i32,
}

// Java: public class MD4 extends BaseHash implements Cloneable
impl MD4 {
    // Java: MD4()
    pub fn new() -> Self {
        // Java: super(MD4_HASH, 16, 64);
        let mut md4 = MD4 {
            base: BaseHash::new(MD4_HASH, DIGEST_LENGTH, BLOCK_LENGTH),
            a: A,
            b: B,
            c: C,
            d: D,
        };
        md4
    }

    // Java: MD4(MD4 that)
    fn new_from(that: &MD4) -> Self {
        // Java: this();
        let mut md4 = MD4::new();
        // Java: this.a = that.a;
        md4.a = that.a;
        // Java: this.b = that.b;
        md4.b = that.b;
        // Java: this.c = that.c;
        md4.c = that.c;
        // Java: this.d = that.d;
        md4.d = that.d;
        // Java: this.count = that.count;
        md4.base.count = that.base.count;
        // Java: this.buffer = (byte[])that.buffer.clone();
        md4.base.buffer = that.base.buffer.clone();
        md4
    }

    // Java: Object clone()
    pub fn clone_md4(&self) -> MD4 {
        // Java: return new MD4(this);
        MD4::new_from(self)
    }

    // Java: byte[] getResult()
    pub fn getResult(&self) -> Vec<u8> {
        // Java: byte[] digest = new byte[]{(byte)this.a, (byte)(this.a >>> 8), (byte)(this.a >>> 16), (byte)(this.a >>> 24), (byte)this.b, (byte)(this.b >>> 8), (byte)(this.b >>> 16), (byte)(this.b >>> 24), (byte)this.c, (byte)(this.c >>> 8), (byte)(this.c >>> 16), (byte)(this.c >>> 24), (byte)this.d, (byte)(this.d >>> 8), (byte)(this.d >>> 16), (byte)(this.d >>> 24)};
        // Java: return digest;
        vec![
            self.a as u8,
            (self.a as u32 >> 8) as u8,
            (self.a as u32 >> 16) as u8,
            (self.a as u32 >> 24) as u8,
            self.b as u8,
            (self.b as u32 >> 8) as u8,
            (self.b as u32 >> 16) as u8,
            (self.b as u32 >> 24) as u8,
            self.c as u8,
            (self.c as u32 >> 8) as u8,
            (self.c as u32 >> 16) as u8,
            (self.c as u32 >> 24) as u8,
            self.d as u8,
            (self.d as u32 >> 8) as u8,
            (self.d as u32 >> 16) as u8,
            (self.d as u32 >> 24) as u8,
        ]
    }

    // Java: void resetContext()
    pub fn resetContext(&mut self) {
        // Java: this.a = 1732584193;
        self.a = A;
        // Java: this.b = -271733879;
        self.b = B;
        // Java: this.c = -1732584194;
        self.c = C;
        // Java: this.d = 271733878;
        self.d = D;
    }

    // Java: boolean selfTest()
    pub fn selfTest(&self) -> bool {
        // Java: if (valid == null)
        let valid_lock = VALID.get_or_init(|| Mutex::new(None));
        let mut valid = valid_lock.lock().unwrap();
        if valid.is_none() {
            // Java: valid = new Boolean("31D6CFE0D16AE931B73C59D7E0C089C0".equals(StringUtil.toString((new MD4()).digest())));
            let mut md4 = MD4::new();
            let digest = md4.digest();
            let hex = digest.iter().map(|b| format!("{:02X}", b)).collect::<String>();
            *valid = Some(DIGEST0 == hex);
        }
        // Java: return valid;
        valid.unwrap_or(false)
    }

    // Java: byte[] padBuffer()
    pub fn padBuffer(&self) -> Vec<u8> {
        // Java: int n = (int)(this.count % 64L);
        let n = (self.base.count % 64) as usize;
        // Java: int padding = n < 56 ? 56 - n : 120 - n;
        let padding = if n < 56 { 56 - n } else { 120 - n };
        // Java: byte[] pad = new byte[padding + 8];
        let mut pad = vec![0u8; padding + 8];
        // Java: pad[0] = -128;
        pad[0] = 0x80;
        // Java: long bits = this.count << 3;
        let bits = self.base.count << 3;
        // Java: pad[padding++] = (byte)((int)bits);
        pad[padding] = bits as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 8));
        pad[padding + 1] = (bits as u64 >> 8) as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 16));
        pad[padding + 2] = (bits as u64 >> 16) as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 24));
        pad[padding + 3] = (bits as u64 >> 24) as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 32));
        pad[padding + 4] = (bits as u64 >> 32) as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 40));
        pad[padding + 5] = (bits as u64 >> 40) as u8;
        // Java: pad[padding++] = (byte)((int)(bits >>> 48));
        pad[padding + 6] = (bits as u64 >> 48) as u8;
        // Java: pad[padding] = (byte)((int)(bits >>> 56));
        pad[padding + 7] = (bits as u64 >> 56) as u8;
        // Java: return pad;
        pad
    }

    pub fn digest(&mut self) -> Vec<u8> {
        let tail = self.padBuffer();
        let tail_len = tail.len();
        self.update_range(&tail, 0, tail_len);
        let result = self.getResult();
        self.reset();
        result
    }

    pub fn reset(&mut self) {
        self.base.count = 0;
        for i in 0..self.base.block_size as usize {
            self.base.buffer[i] = 0;
        }
        self.resetContext();
    }

    pub fn update_range(&mut self, b: &[u8], offset: usize, len: usize) {
        let mut n = (self.base.count % self.base.block_size as i64) as usize;
        self.base.count += len as i64;
        let part_len = self.base.block_size as usize - n;
        let mut i: usize = 0;
        if len >= part_len {
            self.base.buffer[n..n + part_len].copy_from_slice(&b[offset..offset + part_len]);
            let buf_copy = self.base.buffer.clone();
            self.transform(&buf_copy, 0);
            i = part_len;
            while i + self.base.block_size as usize - 1 < len {
                self.transform(b, offset + i);
                i += self.base.block_size as usize;
            }
            n = 0;
        }
        if i < len {
            let remaining = len - i;
            self.base.buffer[n..n + remaining].copy_from_slice(&b[offset + i..offset + i + remaining]);
        }
    }

    // Java: void transform(byte[] in, int i)
    pub fn transform(&mut self, input: &[u8], offset: usize) {
        // Java: int X0 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x0  = read_le_i32(input, offset);
        // Java: int X1 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x1  = read_le_i32(input, offset + 4);
        // Java: int X2 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x2  = read_le_i32(input, offset + 8);
        // Java: int X3 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x3  = read_le_i32(input, offset + 12);
        // Java: int X4 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x4  = read_le_i32(input, offset + 16);
        // Java: int X5 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x5  = read_le_i32(input, offset + 20);
        // Java: int X6 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x6  = read_le_i32(input, offset + 24);
        // Java: int X7 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x7  = read_le_i32(input, offset + 28);
        // Java: int X8 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x8  = read_le_i32(input, offset + 32);
        // Java: int X9 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x9  = read_le_i32(input, offset + 36);
        // Java: int X10 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x10 = read_le_i32(input, offset + 40);
        // Java: int X11 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x11 = read_le_i32(input, offset + 44);
        // Java: int X12 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x12 = read_le_i32(input, offset + 48);
        // Java: int X13 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x13 = read_le_i32(input, offset + 52);
        // Java: int X14 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i++] << 24;
        let x14 = read_le_i32(input, offset + 56);
        // Java: int X15 = in[i++] & 255 | (in[i++] & 255) << 8 | (in[i++] & 255) << 16 | in[i] << 24;
        let x15 = read_le_i32(input, offset + 60);
        // Java: int aa = this.a;
        let mut aa = self.a;
        // Java: int bb = this.b;
        let mut bb = self.b;
        // Java: int cc = this.c;
        let mut cc = self.c;
        // Java: int dd = this.d;
        let mut dd = self.d;
        // Java: aa += (bb & cc | ~bb & dd) + X0;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & cc | !bb & dd).wrapping_add(x0));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & bb | ~aa & cc) + X1;
        // Java: dd = dd << 7 | dd >>> -7;
        dd = dd.wrapping_add((aa & bb | !aa & cc).wrapping_add(x1));
        dd = (dd << 7) | ((dd as u32) >> 25) as i32;
        // Java: cc += (dd & aa | ~dd & bb) + X2;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd & aa | !dd & bb).wrapping_add(x2));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc & dd | ~cc & aa) + X3;
        // Java: bb = bb << 19 | bb >>> -19;
        bb = bb.wrapping_add((cc & dd | !cc & aa).wrapping_add(x3));
        bb = (bb << 19) | ((bb as u32) >> 13) as i32;
        // Java: aa += (bb & cc | ~bb & dd) + X4;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & cc | !bb & dd).wrapping_add(x4));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & bb | ~aa & cc) + X5;
        // Java: dd = dd << 7 | dd >>> -7;
        dd = dd.wrapping_add((aa & bb | !aa & cc).wrapping_add(x5));
        dd = (dd << 7) | ((dd as u32) >> 25) as i32;
        // Java: cc += (dd & aa | ~dd & bb) + X6;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd & aa | !dd & bb).wrapping_add(x6));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc & dd | ~cc & aa) + X7;
        // Java: bb = bb << 19 | bb >>> -19;
        bb = bb.wrapping_add((cc & dd | !cc & aa).wrapping_add(x7));
        bb = (bb << 19) | ((bb as u32) >> 13) as i32;
        // Java: aa += (bb & cc | ~bb & dd) + X8;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & cc | !bb & dd).wrapping_add(x8));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & bb | ~aa & cc) + X9;
        // Java: dd = dd << 7 | dd >>> -7;
        dd = dd.wrapping_add((aa & bb | !aa & cc).wrapping_add(x9));
        dd = (dd << 7) | ((dd as u32) >> 25) as i32;
        // Java: cc += (dd & aa | ~dd & bb) + X10;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd & aa | !dd & bb).wrapping_add(x10));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc & dd | ~cc & aa) + X11;
        // Java: bb = bb << 19 | bb >>> -19;
        bb = bb.wrapping_add((cc & dd | !cc & aa).wrapping_add(x11));
        bb = (bb << 19) | ((bb as u32) >> 13) as i32;
        // Java: aa += (bb & cc | ~bb & dd) + X12;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & cc | !bb & dd).wrapping_add(x12));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & bb | ~aa & cc) + X13;
        // Java: dd = dd << 7 | dd >>> -7;
        dd = dd.wrapping_add((aa & bb | !aa & cc).wrapping_add(x13));
        dd = (dd << 7) | ((dd as u32) >> 25) as i32;
        // Java: cc += (dd & aa | ~dd & bb) + X14;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd & aa | !dd & bb).wrapping_add(x14));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc & dd | ~cc & aa) + X15;
        // Java: bb = bb << 19 | bb >>> -19;
        bb = bb.wrapping_add((cc & dd | !cc & aa).wrapping_add(x15));
        bb = (bb << 19) | ((bb as u32) >> 13) as i32;
        // Java: aa += (bb & (cc | dd) | cc & dd) + X0 + 1518500249;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & (cc | dd) | cc & dd).wrapping_add(x0).wrapping_add(1518500249));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & (bb | cc) | bb & cc) + X4 + 1518500249;
        // Java: dd = dd << 5 | dd >>> -5;
        dd = dd.wrapping_add((aa & (bb | cc) | bb & cc).wrapping_add(x4).wrapping_add(1518500249));
        dd = (dd << 5) | ((dd as u32) >> 27) as i32;
        // Java: cc += (dd & (aa | bb) | aa & bb) + X8 + 1518500249;
        // Java: cc = cc << 9 | cc >>> -9;
        cc = cc.wrapping_add((dd & (aa | bb) | aa & bb).wrapping_add(x8).wrapping_add(1518500249));
        cc = (cc << 9) | ((cc as u32) >> 23) as i32;
        // Java: bb += (cc & (dd | aa) | dd & aa) + X12 + 1518500249;
        // Java: bb = bb << 13 | bb >>> -13;
        bb = bb.wrapping_add((cc & (dd | aa) | dd & aa).wrapping_add(x12).wrapping_add(1518500249));
        bb = (bb << 13) | ((bb as u32) >> 19) as i32;
        // Java: aa += (bb & (cc | dd) | cc & dd) + X1 + 1518500249;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & (cc | dd) | cc & dd).wrapping_add(x1).wrapping_add(1518500249));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & (bb | cc) | bb & cc) + X5 + 1518500249;
        // Java: dd = dd << 5 | dd >>> -5;
        dd = dd.wrapping_add((aa & (bb | cc) | bb & cc).wrapping_add(x5).wrapping_add(1518500249));
        dd = (dd << 5) | ((dd as u32) >> 27) as i32;
        // Java: cc += (dd & (aa | bb) | aa & bb) + X9 + 1518500249;
        // Java: cc = cc << 9 | cc >>> -9;
        cc = cc.wrapping_add((dd & (aa | bb) | aa & bb).wrapping_add(x9).wrapping_add(1518500249));
        cc = (cc << 9) | ((cc as u32) >> 23) as i32;
        // Java: bb += (cc & (dd | aa) | dd & aa) + X13 + 1518500249;
        // Java: bb = bb << 13 | bb >>> -13;
        bb = bb.wrapping_add((cc & (dd | aa) | dd & aa).wrapping_add(x13).wrapping_add(1518500249));
        bb = (bb << 13) | ((bb as u32) >> 19) as i32;
        // Java: aa += (bb & (cc | dd) | cc & dd) + X2 + 1518500249;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & (cc | dd) | cc & dd).wrapping_add(x2).wrapping_add(1518500249));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & (bb | cc) | bb & cc) + X6 + 1518500249;
        // Java: dd = dd << 5 | dd >>> -5;
        dd = dd.wrapping_add((aa & (bb | cc) | bb & cc).wrapping_add(x6).wrapping_add(1518500249));
        dd = (dd << 5) | ((dd as u32) >> 27) as i32;
        // Java: cc += (dd & (aa | bb) | aa & bb) + X10 + 1518500249;
        // Java: cc = cc << 9 | cc >>> -9;
        cc = cc.wrapping_add((dd & (aa | bb) | aa & bb).wrapping_add(x10).wrapping_add(1518500249));
        cc = (cc << 9) | ((cc as u32) >> 23) as i32;
        // Java: bb += (cc & (dd | aa) | dd & aa) + X14 + 1518500249;
        // Java: bb = bb << 13 | bb >>> -13;
        bb = bb.wrapping_add((cc & (dd | aa) | dd & aa).wrapping_add(x14).wrapping_add(1518500249));
        bb = (bb << 13) | ((bb as u32) >> 19) as i32;
        // Java: aa += (bb & (cc | dd) | cc & dd) + X3 + 1518500249;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb & (cc | dd) | cc & dd).wrapping_add(x3).wrapping_add(1518500249));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa & (bb | cc) | bb & cc) + X7 + 1518500249;
        // Java: dd = dd << 5 | dd >>> -5;
        dd = dd.wrapping_add((aa & (bb | cc) | bb & cc).wrapping_add(x7).wrapping_add(1518500249));
        dd = (dd << 5) | ((dd as u32) >> 27) as i32;
        // Java: cc += (dd & (aa | bb) | aa & bb) + X11 + 1518500249;
        // Java: cc = cc << 9 | cc >>> -9;
        cc = cc.wrapping_add((dd & (aa | bb) | aa & bb).wrapping_add(x11).wrapping_add(1518500249));
        cc = (cc << 9) | ((cc as u32) >> 23) as i32;
        // Java: bb += (cc & (dd | aa) | dd & aa) + X15 + 1518500249;
        // Java: bb = bb << 13 | bb >>> -13;
        bb = bb.wrapping_add((cc & (dd | aa) | dd & aa).wrapping_add(x15).wrapping_add(1518500249));
        bb = (bb << 13) | ((bb as u32) >> 19) as i32;
        // Java: aa += (bb ^ cc ^ dd) + X0 + 1859775393;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb ^ cc ^ dd).wrapping_add(x0).wrapping_add(1859775393));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa ^ bb ^ cc) + X8 + 1859775393;
        // Java: dd = dd << 9 | dd >>> -9;
        dd = dd.wrapping_add((aa ^ bb ^ cc).wrapping_add(x8).wrapping_add(1859775393));
        dd = (dd << 9) | ((dd as u32) >> 23) as i32;
        // Java: cc += (dd ^ aa ^ bb) + X4 + 1859775393;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd ^ aa ^ bb).wrapping_add(x4).wrapping_add(1859775393));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc ^ dd ^ aa) + X12 + 1859775393;
        // Java: bb = bb << 15 | bb >>> -15;
        bb = bb.wrapping_add((cc ^ dd ^ aa).wrapping_add(x12).wrapping_add(1859775393));
        bb = (bb << 15) | ((bb as u32) >> 17) as i32;
        // Java: aa += (bb ^ cc ^ dd) + X2 + 1859775393;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb ^ cc ^ dd).wrapping_add(x2).wrapping_add(1859775393));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa ^ bb ^ cc) + X10 + 1859775393;
        // Java: dd = dd << 9 | dd >>> -9;
        dd = dd.wrapping_add((aa ^ bb ^ cc).wrapping_add(x10).wrapping_add(1859775393));
        dd = (dd << 9) | ((dd as u32) >> 23) as i32;
        // Java: cc += (dd ^ aa ^ bb) + X6 + 1859775393;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd ^ aa ^ bb).wrapping_add(x6).wrapping_add(1859775393));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc ^ dd ^ aa) + X14 + 1859775393;
        // Java: bb = bb << 15 | bb >>> -15;
        bb = bb.wrapping_add((cc ^ dd ^ aa).wrapping_add(x14).wrapping_add(1859775393));
        bb = (bb << 15) | ((bb as u32) >> 17) as i32;
        // Java: aa += (bb ^ cc ^ dd) + X1 + 1859775393;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb ^ cc ^ dd).wrapping_add(x1).wrapping_add(1859775393));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa ^ bb ^ cc) + X9 + 1859775393;
        // Java: dd = dd << 9 | dd >>> -9;
        dd = dd.wrapping_add((aa ^ bb ^ cc).wrapping_add(x9).wrapping_add(1859775393));
        dd = (dd << 9) | ((dd as u32) >> 23) as i32;
        // Java: cc += (dd ^ aa ^ bb) + X5 + 1859775393;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd ^ aa ^ bb).wrapping_add(x5).wrapping_add(1859775393));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc ^ dd ^ aa) + X13 + 1859775393;
        // Java: bb = bb << 15 | bb >>> -15;
        bb = bb.wrapping_add((cc ^ dd ^ aa).wrapping_add(x13).wrapping_add(1859775393));
        bb = (bb << 15) | ((bb as u32) >> 17) as i32;
        // Java: aa += (bb ^ cc ^ dd) + X3 + 1859775393;
        // Java: aa = aa << 3 | aa >>> -3;
        aa = aa.wrapping_add((bb ^ cc ^ dd).wrapping_add(x3).wrapping_add(1859775393));
        aa = (aa << 3) | ((aa as u32) >> 29) as i32;
        // Java: dd += (aa ^ bb ^ cc) + X11 + 1859775393;
        // Java: dd = dd << 9 | dd >>> -9;
        dd = dd.wrapping_add((aa ^ bb ^ cc).wrapping_add(x11).wrapping_add(1859775393));
        dd = (dd << 9) | ((dd as u32) >> 23) as i32;
        // Java: cc += (dd ^ aa ^ bb) + X7 + 1859775393;
        // Java: cc = cc << 11 | cc >>> -11;
        cc = cc.wrapping_add((dd ^ aa ^ bb).wrapping_add(x7).wrapping_add(1859775393));
        cc = (cc << 11) | ((cc as u32) >> 21) as i32;
        // Java: bb += (cc ^ dd ^ aa) + X15 + 1859775393;
        // Java: bb = bb << 15 | bb >>> -15;
        bb = bb.wrapping_add((cc ^ dd ^ aa).wrapping_add(x15).wrapping_add(1859775393));
        bb = (bb << 15) | ((bb as u32) >> 17) as i32;
        // Java: this.a += aa;
        self.a = self.a.wrapping_add(aa);
        // Java: this.b += bb;
        self.b = self.b.wrapping_add(bb);
        // Java: this.c += cc;
        self.c = self.c.wrapping_add(cc);
        // Java: this.d += dd;
        self.d = self.d.wrapping_add(dd);
    }
}
