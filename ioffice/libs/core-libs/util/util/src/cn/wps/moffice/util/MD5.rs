// Java: public class MD5
pub struct MD5 {
    // Java field: int[] buf
    buf: Vec<i32>,
    // Java field: long bits
    bits: i64,
    // Java field: byte[] in
    input: Vec<u8>,
    // Java field: int[] inint
    inint: Vec<i32>,
}

fn f1(x: i32, y: i32, z: i32) -> i32 { z ^ (x & (y ^ z)) }
fn f2(x: i32, y: i32, z: i32) -> i32 { y ^ (z & (x ^ y)) }
fn f3(x: i32, y: i32, z: i32) -> i32 { x ^ y ^ z }
fn f4(x: i32, y: i32, z: i32) -> i32 { y ^ (x | !z) }

// Java: public class MD5
impl MD5 {
    // Java: MD5()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.buf[0] = 1732584193;
        // Java: this.buf[1] = -271733879;
        // Java: this.buf[2] = -1732584194;
        // Java: this.buf[3] = 271733878;
        // Java: this.bits = 0L;
        // Java: this.in = new byte[64];
        // Java: this.inint = new int[16];
        MD5 {
            buf: vec![1732584193, -271733879, -1732584194, 271733878],
            bits: 0,
            input: vec![0u8; 64],
            inint: vec![0i32; 16],
        }
    }

    // Java: void update(byte[] newbuf)
    pub fn update_all(&mut self, newbuf: &[u8]) {
        // Java: this.update(newbuf, 0, newbuf.length);
        let len = newbuf.len();
        self.update(newbuf, 0, len);
    }

    // Java: void update(byte[] newbuf, int length)
    pub fn update_len(&mut self, newbuf: &[u8], length: usize) {
        // Java: this.update(newbuf, 0, length);
        self.update(newbuf, 0, length);
    }

    // Java: void update(byte[] newbuf, int bufstart, int buflen)
    pub fn update(&mut self, newbuf: &[u8], mut bufstart: usize, buflen: usize) {
        // Java: int len = buflen;
        let mut len = buflen;
        // Java: int t = (int)this.bits;
        let mut t = self.bits as i32;
        // Java: this.bits += (long)(buflen << 3);
        self.bits += (buflen << 3) as i64;
        // Java: t = t >>> 3 & 63;
        t = ((t as u32) >> 3) as i32 & 63;
        // Java: if (t != 0)
        if t != 0 {
            // Java: int p = t;
            let p = t as usize;
            // Java: t = 64 - t;
            t = 64 - t;
            // Java: if (buflen < t)
            if buflen < t as usize {
                // Java: System.arraycopy(newbuf, bufstart, this.in, p, buflen);
                self.input[p..p + buflen].copy_from_slice(&newbuf[bufstart..bufstart + buflen]);
                // Java: return;
                return;
            }
            // Java: System.arraycopy(newbuf, bufstart, this.in, p, t);
            self.input[p..p + t as usize].copy_from_slice(&newbuf[bufstart..bufstart + t as usize]);
            // Java: this.transform();
            self.transform();
            // Java: bufstart += t;
            bufstart += t as usize;
            // Java: len = buflen - t;
            len = buflen - t as usize;
        }
        // Java: while(len >= 64)
        while len >= 64 {
            // Java: System.arraycopy(newbuf, bufstart, this.in, 0, 64);
            self.input[..64].copy_from_slice(&newbuf[bufstart..bufstart + 64]);
            // Java: this.transform();
            self.transform();
            // Java: bufstart += 64;
            bufstart += 64;
            // Java: len -= 64;
            len -= 64;
        }
        // Java: System.arraycopy(newbuf, bufstart, this.in, 0, len);
        if len > 0 {
            self.input[..len].copy_from_slice(&newbuf[bufstart..bufstart + len]);
        }
    }

    // Java: void md5final(byte[] digest)
    pub fn md5final(&mut self, digest: &mut [u8]) {
        // Java: int count = (int)(this.bits >>> 3 & 63L);
        let count = ((self.bits as u64 >> 3) & 63) as usize;
        // Java: int p = count + 1;
        let p = count + 1;
        // Java: this.in[count] = -128;
        self.input[count] = 0x80;
        // Java: count = 63 - count;
        let count = 63 - count;
        // Java: if (count < 8)
        if count < 8 {
            // Java: this.zeroByteArray(this.in, p, count);
            self.zero_byte_array_range(p, count);
            // Java: this.transform();
            self.transform();
            // Java: this.zeroByteArray(this.in, 0, 56);
            self.zero_byte_array_range(0, 56);
        }
        // Java: else
        else {
            // Java: this.zeroByteArray(this.in, p, count - 8);
            self.zero_byte_array_range(p, count - 8);
        }
        // Java: int lowbits = (int)this.bits;
        let lowbits = self.bits as i32;
        // Java: int highbits = (int)(this.bits >>> 32);
        let highbits = (self.bits as u64 >> 32) as i32;
        // Java: this.PUT_32BIT_LSB_FIRST(this.in, 56, lowbits);
        Self::put_32bit_lsb_first(&mut self.input, 56, lowbits);
        // Java: this.PUT_32BIT_LSB_FIRST(this.in, 60, highbits);
        Self::put_32bit_lsb_first(&mut self.input, 60, highbits);
        // Java: this.transform();
        self.transform();
        // Java: this.PUT_32BIT_LSB_FIRST(digest, 0, this.buf[0]);
        Self::put_32bit_lsb_first(digest, 0, self.buf[0]);
        // Java: this.PUT_32BIT_LSB_FIRST(digest, 4, this.buf[1]);
        Self::put_32bit_lsb_first(digest, 4, self.buf[1]);
        // Java: this.PUT_32BIT_LSB_FIRST(digest, 8, this.buf[2]);
        Self::put_32bit_lsb_first(digest, 8, self.buf[2]);
        // Java: this.PUT_32BIT_LSB_FIRST(digest, 12, this.buf[3]);
        Self::put_32bit_lsb_first(digest, 12, self.buf[3]);
        // Java: this.zeroByteArray(this.in);
        // Java: this.zeroIntArray(this.buf);
        // Java: this.bits = 0L;
        // Java: this.zeroIntArray(this.inint);
        self.input.fill(0);
        self.buf.fill(0);
        self.bits = 0;
        self.inint.fill(0);
    }

    // Java: static String getMD5(String input)
    pub fn getMD5(input_str: &str) -> String {
        // Java: try
        // Java: MessageDigest md = MessageDigest.getInstance("MD5");
        // Java: byte[] messageDigest = md.digest(input.getBytes());
        // Java: BigInteger number = new BigInteger(1, messageDigest);
        // Java: String hashtext;
        // Java: for(hashtext = number.toString(16); hashtext.length() < 32; hashtext = "0" + hashtext)
        // Java: return hashtext;
        // Java: catch (NoSuchAlgorithmException e)
        // Java: throw new RuntimeException(e);
        let digest = md5_digest(input_str.as_bytes());
        let hex = digest.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        hex
    }

    // Java: static String getMD52(String source)
    pub fn getMD52(source: &str) -> String {
        // Java: MD5 md = new MD5();
        let mut md = MD5::new();
        // Java: md.update(source.getBytes());
        md.update_all(source.as_bytes());
        // Java: byte[] out = new byte[16];
        let mut out = vec![0u8; 16];
        // Java: md.md5final(out);
        md.md5final(&mut out);
        // Java: BigInteger number = new BigInteger(1, out);
        // Java: String hashtext;
        // Java: for(hashtext = number.toString(16); hashtext.length() < 32; hashtext = "0" + hashtext)
        // Java: return hashtext;
        let hex = out.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        hex
    }

    // Java: static void main(String[] args)
    // Java: int errorCount = 0;
    // Java: String source = "";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "The quick brown fox jumps over the lazy dog";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "djfajlgiorgk  fijiughrijgn qefuhqrgbfjknvfdbbqe u30u4t9572y5uthn gj reughnfjvp39p4ut493  roiug3jrn fjbh0p";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "你好捏peowijgrpejbeugr扔 ufjerogjrgj9 nvejgriejfojifjeij溻 jnjrej irej 浊  rger氤后悔 irqernglrign 的朱最近的 滥用";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "1992年8月Ronald L. Rivest向IETF提交了一份重要文件，描述了这种算法的原理，由于这种算法的公开性和安全性，在90年代被广泛使用在各种程序语言中，用以确保资料传递无误等。\r\nMD5由MD4、MD3、MD2改进而来，主要增强算法复杂度和不可逆性。\n目前，MD5算法因其普遍、稳定、快速的特点，仍广泛应用于普通数据的错误检查领域。例如在一些BitTorrent下载中，软件将通过计算MD5检验下载到的文件片段的完整性。";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "MD5已经广泛使用在为文件传输提供一定的可靠性方面。例如，服务器预先提供一个MD5校验和，用户下载完文件以后，用MD5算法计算下载文件的MD5校验和，然后通过检查这两个校验和是否一致，就能判断下载的文件是否出错。MD5亦有应用于部份网上赌场以保证赌博的公平性，原理是系统先在玩家下注前已生成该局的结果，将该结果的字符串配合一组随机字符串利用MD5 加密，将该加密字符串于玩家下注前便显示给玩家，再在结果开出后将未加密的字符串显示给玩家，玩家便可利用MD5工具加密验证该字符串是否吻合。例子: 在玩家下注骰宝前，赌场便先决定该局结果，假设生成的随机结果为4、5、 6大，赌场便会先利用MD5 加密“4, 5, 6”此字符串并于玩家下注前告诉玩家；由于赌场是无法预计玩家会下什么注，所以便能确保赌场不能作弊；当玩家下注完毕后，赌场便告诉玩家该原始字符串，即“4, 5, 6”，玩家便可利用MD5工具加密该字符串是否与下注前的加密字符串吻合。该字符串一般会加上一组随机字符串 (Random string)，以防止玩家利用碰撞 (Collision) 解密字符串，但如使用超级电脑利用碰撞亦有可能从加上随机字符串的加密字符串中取得游戏结果。随机字符串的长度与碰撞的次数成正比关系，一般网上赌场使用的随机字符串是长于20字，有些网上赌场的随机字符串更长达500字，以增加解密难度。";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: source = "";
    // Java: if (!getMD5(source).equals(getMD52(source)))
    {
        // Java: ++errorCount;
    }
    // Java: System.out.println(errorCount);

    // Java: void zeroByteArray(byte[] a)
    fn zero_byte_array(&mut self) {
        // Java: this.zeroByteArray(a, 0, a.length);
        self.input.fill(0);
    }

    // Java: void zeroByteArray(byte[] a, int start, int length)
    fn zero_byte_array_range(&mut self, start: usize, length: usize) {
        // Java: this.setByteArray(a, (byte)0, start, length);
        self.set_byte_array(0, start, length);
    }

    // Java: void setByteArray(byte[] a, byte val, int start, int length)
    fn set_byte_array(&mut self, val: u8, start: usize, length: usize) {
        // Java: int end = start + length;
        let end = start + length;
        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: a[i] = val;
            self.input[i] = val;
        }
    }

    // Java: void zeroIntArray(int[] a)
    fn zero_int_array(&mut self, a: &mut Vec<i32>) {
        // Java: this.zeroIntArray(a, 0, a.length);
        let len = a.len();
        Self::zero_int_array_range(a, 0, len);
    }

    // Java: void zeroIntArray(int[] a, int start, int length)
    fn zero_int_array_range(a: &mut Vec<i32>, start: usize, length: usize) {
        // Java: this.setIntArray(a, 0, start, length);
        Self::set_int_array(a, 0, start, length);
    }

    // Java: void setIntArray(int[] a, int val, int start, int length)
    fn set_int_array(a: &mut Vec<i32>, val: i32, start: usize, length: usize) {
        // Java: int end = start + length;
        let end = start + length;
        // Java: for(int i = start; i < end; ++i)
        for i in start..end {
            // Java: a[i] = val;
            a[i] = val;
        }
    }

    // Java: int MD5STEP(Fcore f, int w, int x, int y, int z, int data, int s)
    fn md5step(f: fn(i32, i32, i32) -> i32, w: i32, x: i32, y: i32, z: i32, data: i32, s: u32) -> i32 {
        // Java: w += f.f(x, y, z) + data;
        let mut w = w.wrapping_add(f(x, y, z)).wrapping_add(data);
        // Java: w = w << s | w >>> 32 - s;
        w = (w << s) | ((w as u32) >> (32 - s)) as i32;
        // Java: w += x;
        w = w.wrapping_add(x);
        // Java: return w;
        w
    }

    // Java: void transform()
    fn transform(&mut self) {
        // Java: int[] inint = new int[16];
        let mut inint = [0i32; 16];
        // Java: for(int i = 0; i < 16; ++i)
        for i in 0..16 {
            // Java: inint[i] = this.GET_32BIT_LSB_FIRST(this.in, 4 * i);
            inint[i] = Self::get_32bit_lsb_first(&self.input, 4 * i);
        }
        // Java: int a = this.buf[0];
        let mut a = self.buf[0];
        // Java: int b = this.buf[1];
        let mut b = self.buf[1];
        // Java: int c = this.buf[2];
        let mut c = self.buf[2];
        // Java: int d = this.buf[3];
        let mut d = self.buf[3];
        // Java: a = this.MD5STEP(this.F1, a, b, c, d, inint[0] + -680876936, 7);
        a = Self::md5step(f1, a, b, c, d, inint[0].wrapping_add(-680876936), 7);
        // Java: d = this.MD5STEP(this.F1, d, a, b, c, inint[1] + -389564586, 12);
        d = Self::md5step(f1, d, a, b, c, inint[1].wrapping_add(-389564586), 12);
        // Java: c = this.MD5STEP(this.F1, c, d, a, b, inint[2] + 606105819, 17);
        c = Self::md5step(f1, c, d, a, b, inint[2].wrapping_add(606105819), 17);
        // Java: b = this.MD5STEP(this.F1, b, c, d, a, inint[3] + -1044525330, 22);
        b = Self::md5step(f1, b, c, d, a, inint[3].wrapping_add(-1044525330), 22);
        // Java: a = this.MD5STEP(this.F1, a, b, c, d, inint[4] + -176418897, 7);
        a = Self::md5step(f1, a, b, c, d, inint[4].wrapping_add(-176418897), 7);
        // Java: d = this.MD5STEP(this.F1, d, a, b, c, inint[5] + 1200080426, 12);
        d = Self::md5step(f1, d, a, b, c, inint[5].wrapping_add(1200080426), 12);
        // Java: c = this.MD5STEP(this.F1, c, d, a, b, inint[6] + -1473231341, 17);
        c = Self::md5step(f1, c, d, a, b, inint[6].wrapping_add(-1473231341), 17);
        // Java: b = this.MD5STEP(this.F1, b, c, d, a, inint[7] + -45705983, 22);
        b = Self::md5step(f1, b, c, d, a, inint[7].wrapping_add(-45705983), 22);
        // Java: a = this.MD5STEP(this.F1, a, b, c, d, inint[8] + 1770035416, 7);
        a = Self::md5step(f1, a, b, c, d, inint[8].wrapping_add(1770035416), 7);
        // Java: d = this.MD5STEP(this.F1, d, a, b, c, inint[9] + -1958414417, 12);
        d = Self::md5step(f1, d, a, b, c, inint[9].wrapping_add(-1958414417), 12);
        // Java: c = this.MD5STEP(this.F1, c, d, a, b, inint[10] + -42063, 17);
        c = Self::md5step(f1, c, d, a, b, inint[10].wrapping_add(-42063), 17);
        // Java: b = this.MD5STEP(this.F1, b, c, d, a, inint[11] + -1990404162, 22);
        b = Self::md5step(f1, b, c, d, a, inint[11].wrapping_add(-1990404162), 22);
        // Java: a = this.MD5STEP(this.F1, a, b, c, d, inint[12] + 1804603682, 7);
        a = Self::md5step(f1, a, b, c, d, inint[12].wrapping_add(1804603682), 7);
        // Java: d = this.MD5STEP(this.F1, d, a, b, c, inint[13] + -40341101, 12);
        d = Self::md5step(f1, d, a, b, c, inint[13].wrapping_add(-40341101), 12);
        // Java: c = this.MD5STEP(this.F1, c, d, a, b, inint[14] + -1502002290, 17);
        c = Self::md5step(f1, c, d, a, b, inint[14].wrapping_add(-1502002290), 17);
        // Java: b = this.MD5STEP(this.F1, b, c, d, a, inint[15] + 1236535329, 22);
        b = Self::md5step(f1, b, c, d, a, inint[15].wrapping_add(1236535329), 22);
        // Java: a = this.MD5STEP(this.F2, a, b, c, d, inint[1] + -165796510, 5);
        a = Self::md5step(f2, a, b, c, d, inint[1].wrapping_add(-165796510), 5);
        // Java: d = this.MD5STEP(this.F2, d, a, b, c, inint[6] + -1069501632, 9);
        d = Self::md5step(f2, d, a, b, c, inint[6].wrapping_add(-1069501632), 9);
        // Java: c = this.MD5STEP(this.F2, c, d, a, b, inint[11] + 643717713, 14);
        c = Self::md5step(f2, c, d, a, b, inint[11].wrapping_add(643717713), 14);
        // Java: b = this.MD5STEP(this.F2, b, c, d, a, inint[0] + -373897302, 20);
        b = Self::md5step(f2, b, c, d, a, inint[0].wrapping_add(-373897302), 20);
        // Java: a = this.MD5STEP(this.F2, a, b, c, d, inint[5] + -701558691, 5);
        a = Self::md5step(f2, a, b, c, d, inint[5].wrapping_add(-701558691), 5);
        // Java: d = this.MD5STEP(this.F2, d, a, b, c, inint[10] + 38016083, 9);
        d = Self::md5step(f2, d, a, b, c, inint[10].wrapping_add(38016083), 9);
        // Java: c = this.MD5STEP(this.F2, c, d, a, b, inint[15] + -660478335, 14);
        c = Self::md5step(f2, c, d, a, b, inint[15].wrapping_add(-660478335), 14);
        // Java: b = this.MD5STEP(this.F2, b, c, d, a, inint[4] + -405537848, 20);
        b = Self::md5step(f2, b, c, d, a, inint[4].wrapping_add(-405537848), 20);
        // Java: a = this.MD5STEP(this.F2, a, b, c, d, inint[9] + 568446438, 5);
        a = Self::md5step(f2, a, b, c, d, inint[9].wrapping_add(568446438), 5);
        // Java: d = this.MD5STEP(this.F2, d, a, b, c, inint[14] + -1019803690, 9);
        d = Self::md5step(f2, d, a, b, c, inint[14].wrapping_add(-1019803690), 9);
        // Java: c = this.MD5STEP(this.F2, c, d, a, b, inint[3] + -187363961, 14);
        c = Self::md5step(f2, c, d, a, b, inint[3].wrapping_add(-187363961), 14);
        // Java: b = this.MD5STEP(this.F2, b, c, d, a, inint[8] + 1163531501, 20);
        b = Self::md5step(f2, b, c, d, a, inint[8].wrapping_add(1163531501), 20);
        // Java: a = this.MD5STEP(this.F2, a, b, c, d, inint[13] + -1444681467, 5);
        a = Self::md5step(f2, a, b, c, d, inint[13].wrapping_add(-1444681467), 5);
        // Java: d = this.MD5STEP(this.F2, d, a, b, c, inint[2] + -51403784, 9);
        d = Self::md5step(f2, d, a, b, c, inint[2].wrapping_add(-51403784), 9);
        // Java: c = this.MD5STEP(this.F2, c, d, a, b, inint[7] + 1735328473, 14);
        c = Self::md5step(f2, c, d, a, b, inint[7].wrapping_add(1735328473), 14);
        // Java: b = this.MD5STEP(this.F2, b, c, d, a, inint[12] + -1926607734, 20);
        b = Self::md5step(f2, b, c, d, a, inint[12].wrapping_add(-1926607734), 20);
        // Java: a = this.MD5STEP(this.F3, a, b, c, d, inint[5] + -378558, 4);
        a = Self::md5step(f3, a, b, c, d, inint[5].wrapping_add(-378558), 4);
        // Java: d = this.MD5STEP(this.F3, d, a, b, c, inint[8] + -2022574463, 11);
        d = Self::md5step(f3, d, a, b, c, inint[8].wrapping_add(-2022574463), 11);
        // Java: c = this.MD5STEP(this.F3, c, d, a, b, inint[11] + 1839030562, 16);
        c = Self::md5step(f3, c, d, a, b, inint[11].wrapping_add(1839030562), 16);
        // Java: b = this.MD5STEP(this.F3, b, c, d, a, inint[14] + -35309556, 23);
        b = Self::md5step(f3, b, c, d, a, inint[14].wrapping_add(-35309556), 23);
        // Java: a = this.MD5STEP(this.F3, a, b, c, d, inint[1] + -1530992060, 4);
        a = Self::md5step(f3, a, b, c, d, inint[1].wrapping_add(-1530992060), 4);
        // Java: d = this.MD5STEP(this.F3, d, a, b, c, inint[4] + 1272893353, 11);
        d = Self::md5step(f3, d, a, b, c, inint[4].wrapping_add(1272893353), 11);
        // Java: c = this.MD5STEP(this.F3, c, d, a, b, inint[7] + -155497632, 16);
        c = Self::md5step(f3, c, d, a, b, inint[7].wrapping_add(-155497632), 16);
        // Java: b = this.MD5STEP(this.F3, b, c, d, a, inint[10] + -1094730640, 23);
        b = Self::md5step(f3, b, c, d, a, inint[10].wrapping_add(-1094730640), 23);
        // Java: a = this.MD5STEP(this.F3, a, b, c, d, inint[13] + 681279174, 4);
        a = Self::md5step(f3, a, b, c, d, inint[13].wrapping_add(681279174), 4);
        // Java: d = this.MD5STEP(this.F3, d, a, b, c, inint[0] + -358537222, 11);
        d = Self::md5step(f3, d, a, b, c, inint[0].wrapping_add(-358537222), 11);
        // Java: c = this.MD5STEP(this.F3, c, d, a, b, inint[3] + -722521979, 16);
        c = Self::md5step(f3, c, d, a, b, inint[3].wrapping_add(-722521979), 16);
        // Java: b = this.MD5STEP(this.F3, b, c, d, a, inint[6] + 76029189, 23);
        b = Self::md5step(f3, b, c, d, a, inint[6].wrapping_add(76029189), 23);
        // Java: a = this.MD5STEP(this.F3, a, b, c, d, inint[9] + -640364487, 4);
        a = Self::md5step(f3, a, b, c, d, inint[9].wrapping_add(-640364487), 4);
        // Java: d = this.MD5STEP(this.F3, d, a, b, c, inint[12] + -421815835, 11);
        d = Self::md5step(f3, d, a, b, c, inint[12].wrapping_add(-421815835), 11);
        // Java: c = this.MD5STEP(this.F3, c, d, a, b, inint[15] + 530742520, 16);
        c = Self::md5step(f3, c, d, a, b, inint[15].wrapping_add(530742520), 16);
        // Java: b = this.MD5STEP(this.F3, b, c, d, a, inint[2] + -995338651, 23);
        b = Self::md5step(f3, b, c, d, a, inint[2].wrapping_add(-995338651), 23);
        // Java: a = this.MD5STEP(this.F4, a, b, c, d, inint[0] + -198630844, 6);
        a = Self::md5step(f4, a, b, c, d, inint[0].wrapping_add(-198630844), 6);
        // Java: d = this.MD5STEP(this.F4, d, a, b, c, inint[7] + 1126891415, 10);
        d = Self::md5step(f4, d, a, b, c, inint[7].wrapping_add(1126891415), 10);
        // Java: c = this.MD5STEP(this.F4, c, d, a, b, inint[14] + -1416354905, 15);
        c = Self::md5step(f4, c, d, a, b, inint[14].wrapping_add(-1416354905), 15);
        // Java: b = this.MD5STEP(this.F4, b, c, d, a, inint[5] + -57434055, 21);
        b = Self::md5step(f4, b, c, d, a, inint[5].wrapping_add(-57434055), 21);
        // Java: a = this.MD5STEP(this.F4, a, b, c, d, inint[12] + 1700485571, 6);
        a = Self::md5step(f4, a, b, c, d, inint[12].wrapping_add(1700485571), 6);
        // Java: d = this.MD5STEP(this.F4, d, a, b, c, inint[3] + -1894986606, 10);
        d = Self::md5step(f4, d, a, b, c, inint[3].wrapping_add(-1894986606), 10);
        // Java: c = this.MD5STEP(this.F4, c, d, a, b, inint[10] + -1051523, 15);
        c = Self::md5step(f4, c, d, a, b, inint[10].wrapping_add(-1051523), 15);
        // Java: b = this.MD5STEP(this.F4, b, c, d, a, inint[1] + -2054922799, 21);
        b = Self::md5step(f4, b, c, d, a, inint[1].wrapping_add(-2054922799), 21);
        // Java: a = this.MD5STEP(this.F4, a, b, c, d, inint[8] + 1873313359, 6);
        a = Self::md5step(f4, a, b, c, d, inint[8].wrapping_add(1873313359), 6);
        // Java: d = this.MD5STEP(this.F4, d, a, b, c, inint[15] + -30611744, 10);
        d = Self::md5step(f4, d, a, b, c, inint[15].wrapping_add(-30611744), 10);
        // Java: c = this.MD5STEP(this.F4, c, d, a, b, inint[6] + -1560198380, 15);
        c = Self::md5step(f4, c, d, a, b, inint[6].wrapping_add(-1560198380), 15);
        // Java: b = this.MD5STEP(this.F4, b, c, d, a, inint[13] + 1309151649, 21);
        b = Self::md5step(f4, b, c, d, a, inint[13].wrapping_add(1309151649), 21);
        // Java: a = this.MD5STEP(this.F4, a, b, c, d, inint[4] + -145523070, 6);
        a = Self::md5step(f4, a, b, c, d, inint[4].wrapping_add(-145523070), 6);
        // Java: d = this.MD5STEP(this.F4, d, a, b, c, inint[11] + -1120210379, 10);
        d = Self::md5step(f4, d, a, b, c, inint[11].wrapping_add(-1120210379), 10);
        // Java: c = this.MD5STEP(this.F4, c, d, a, b, inint[2] + 718787259, 15);
        c = Self::md5step(f4, c, d, a, b, inint[2].wrapping_add(718787259), 15);
        // Java: b = this.MD5STEP(this.F4, b, c, d, a, inint[9] + -343485551, 21);
        b = Self::md5step(f4, b, c, d, a, inint[9].wrapping_add(-343485551), 21);
        // Java: int[] var10000 = this.buf;
        // Java: var10000[0] += a;
        self.buf[0] = self.buf[0].wrapping_add(a);
        // Java: var10000 = this.buf;
        // Java: var10000[1] += b;
        self.buf[1] = self.buf[1].wrapping_add(b);
        // Java: var10000 = this.buf;
        // Java: var10000[2] += c;
        self.buf[2] = self.buf[2].wrapping_add(c);
        // Java: var10000 = this.buf;
        // Java: var10000[3] += d;
        self.buf[3] = self.buf[3].wrapping_add(d);
    }

    // Java: int GET_32BIT_LSB_FIRST(byte[] b, int off)
    fn get_32bit_lsb_first(b: &[u8], off: usize) -> i32 {
        // Java: return b[off + 0] & 255 | (b[off + 1] & 255) << 8 | (b[off + 2] & 255) << 16 | (b[off + 3] & 255) << 24;
        (b[off] as i32 & 255)
            | ((b[off + 1] as i32 & 255) << 8)
            | ((b[off + 2] as i32 & 255) << 16)
            | ((b[off + 3] as i32 & 255) << 24)
    }

    // Java: void PUT_32BIT_LSB_FIRST(byte[] b, int off, int value)
    fn put_32bit_lsb_first(b: &mut [u8], off: usize, value: i32) {
        // Java: b[off + 0] = (byte)(value & 255);
        b[off] = (value & 255) as u8;
        // Java: b[off + 1] = (byte)(value >> 8 & 255);
        b[off + 1] = ((value >> 8) & 255) as u8;
        // Java: b[off + 2] = (byte)(value >> 16 & 255);
        b[off + 2] = ((value >> 16) & 255) as u8;
        // Java: b[off + 3] = (byte)(value >> 24 & 255);
        b[off + 3] = ((value >> 24) & 255) as u8;
    }
}

// Java: public class Fcore
// Java: Fcore()
// Java: super();
// Java: abstract int f(int var1, int var2, int var3)

fn md5_digest(data: &[u8]) -> Vec<u8> {
    let mut md5 = MD5::new();
    md5.update_all(data);
    let mut out = vec![0u8; 16];
    md5.md5final(&mut out);
    out
}
