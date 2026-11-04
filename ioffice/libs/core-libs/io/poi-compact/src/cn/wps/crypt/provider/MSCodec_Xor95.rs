// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.crypt.CryptHelper
// Java: import org.apache.poi.util.LittleEndian

fn le_get_byte_array(data: &[u8], offset: usize, len: usize) -> Vec<u8> {
    let end = std::cmp::min(offset + len, data.len());
    data[offset..end].to_vec()
}
fn le_put_short(data: &mut [u8], value: i16) {
    data[0] = value as u8;
    data[1] = (value >> 8) as u8;
}

fn lclRotateLeft_byte(value: u8, nBits: u8) -> u8 {
    let v = value as u16;
    ((v << nBits) | (v >> (8 - nBits))) as u8
}

fn lclRotateLeft_short(value: i16, nBits: u8) -> i16 {
    let v = value as u16;
    ((v << nBits) | (v >> (16 - nBits))) as i16
}

fn lclRotateLeft_short_width(value: i16, nBits: u8, nWidth: u8) -> i16 {
    let v = value as u16;
    let mask = ((1u16 << (nWidth as u16)) - 1) as u16;
    (((v << nBits) | ((v & mask) >> (nWidth - nBits))) & mask) as i16
}

fn lclGetLen(pnPassData: &[u8], nBufferSize: usize) -> usize {
    let mut nLen = 0;
    while nLen < nBufferSize && pnPassData[nLen] != 0 {
        nLen += 1;
    }
    nLen
}

fn lclGetHash(pnPassData: &[u8], nBufferSize: usize) -> i16 {
    let nLen = lclGetLen(pnPassData, nBufferSize);
    let mut nHash: i16 = nLen as i16;
    if nLen != 0 {
        nHash = nHash ^ 0xCE4Bu16 as i16;
    }
    let mut nOffset: usize = 0;
    let mut nIndex: usize = 0;
    while nIndex < nLen {
        let cChar = pnPassData[nOffset] as i16;
        let nRot = ((nIndex + 1) % 15) as u8;
        let cChar = lclRotateLeft_short_width(cChar, nRot, 15);
        nHash ^= cChar;
        nIndex += 1;
        nOffset += 1;
    }
    nHash
}

fn lclGetKey(pnPassData: &[u8], nBufferSize: usize) -> i16 {
    let nLen = lclGetLen(pnPassData, nBufferSize);
    if nLen == 0 {
        return 0;
    }
    let mut nKey: i16 = 0;
    let mut nKeyBase: i16 = i16::MIN;
    let mut nKeyEnd: i16 = -1;
    let mut nOffset = nLen as i32 - 1;
    let mut nIndex = 0;
    while nIndex < nLen {
        let mut cChar = (pnPassData[nOffset as usize] & 0x7F) as u8;
        for _nBit in 0..8u8 {
            nKeyBase = lclRotateLeft_short(nKeyBase, 1);
            if (nKeyBase & 1) != 0 {
                nKeyBase = nKeyBase ^ 4128;
            }
            if (cChar & 1) != 0 {
                nKey ^= nKeyBase;
            }
            cChar = cChar >> 1;
            nKeyEnd = lclRotateLeft_short(nKeyEnd, 1);
            if (nKeyEnd & 1) != 0 {
                nKeyEnd = nKeyEnd ^ 4128;
            }
        }
        nIndex += 1;
        nOffset -= 1;
    }
    nKey ^ nKeyEnd
}

// Java field: short[] g_more_key
const G_MORE_KEY: [i16; 61] = [259, -7696, 7439, -13156, -31552, 4364, 3600, -3634, 12606, 29208, -7879, -11249, -31495, 10252, -22166, 20163, 0, -20740, 19929, -25678, 10053, 20106, -25324, 10761, 31585, -2366, -603, -5269, -14601, -25137, 11199, 17763, -30010, 1453, 2906, 5812, 11624, 23248, 885, 1770, 3540, 7080, 14160, 28320, -8896, -10167, -24397, 20807, -23874, 21821, -21894, 17621, 28485, -8566, -21195, 19019, -27498, 14605, 29210, -5341, -14745];

// Java: public class MSCodec_Xor95
pub struct MSCodec_Xor95 {
    // Java field: byte[] mKey
    pub(crate) mKey: Vec<u8>,
    // Java field: int mnOffset
    pub(crate) mnOffset: i32,
    // Java field: int mnRotateDistance
    pub(crate) mnRotateDistance: i32,
    // Java field: short mnKey
    pub(crate) mnKey: i16,
    // Java field: short mnHash
    pub(crate) mnHash: i16,
}

// Java: public class MSCodec_Xor95
impl MSCodec_Xor95 {
    // Java: MSCodec_Xor95(int nRotateDistance)
    pub fn new(nRotateDistance: i32) -> Self {
        // Java: super();
        // Java: this.mnRotateDistance = nRotateDistance;
        MSCodec_Xor95 {
            mKey: Vec::new(),
            mnOffset: 0,
            mnRotateDistance: nRotateDistance,
            mnKey: 0,
            mnHash: 0,
        }
    }

    // Java: void initKey(byte[] passData, short nKey, short nHash)
    pub fn initKey(&mut self, passData: &[u8], nKey: i16, nHash: i16) {
        // Java: IOAssert.logTrue("passData should not be null!", passData != null);
        // Java: IOAssert.logTrue("passData.length should  be 16!", passData.length == 16);
        // Java: this.mnKey = nKey;
        self.mnKey = nKey;
        // Java: this.mnHash = nHash;
        self.mnHash = nHash;
        // Java: this.mKey = LittleEndian.getByteArray(passData, 0, 16);
        self.mKey = le_get_byte_array(passData, 0, 16);
        // Java: byte[] spnFillChars = new byte[]{-69, -1, -1, -70, -1, -1, -71, -128, 0, -66, 15, 0, -65, 15, 0};
        let spnFillChars: [u8; 15] = [0xBBu8, 0xFF, 0xFF, 0xBA, 0xFF, 0xFF, 0xB9, 0x80, 0x00, 0xBE, 0x0F, 0x00, 0xBF, 0x0F, 0x00];
        // Java: int nLen = CryptHelper.lclGetLen(passData, 16);
        let nLen = lclGetLen(passData, 16);
        // Java: int nIndex = nLen;
        let mut nIndex = nLen;
        // Java: for(int i = 0; nIndex < this.mKey.length; ++i)
        let mut i = 0;
        while nIndex < self.mKey.len() {
            // Java: this.mKey[nIndex] = spnFillChars[i];
            self.mKey[nIndex] = spnFillChars[i];
            i += 1;
            // Java: ++nIndex;
            nIndex += 1;
        }
        // Java: byte[] pnOrigKey = new byte[2];
        let mut pnOrigKey = [0u8; 2];
        // Java: LittleEndian.putShort(pnOrigKey, this.mnKey);
        le_put_short(&mut pnOrigKey, self.mnKey);
        // Java: for(int var9 = 0; var9 < this.mKey.length; ++var9)
        for var9 in 0..self.mKey.len() {
            // Java: this.mKey[var9] = (byte)((this.mKey[var9] & 255 ^ pnOrigKey[var9 & 1] & 255) & 255);
            self.mKey[var9] = ((self.mKey[var9] as i32 & 0xFF) ^ (pnOrigKey[var9 & 1] as i32 & 0xFF)) as u8;
            // Java: this.mKey[var9] = CryptHelper.lclRotateLeft(this.mKey[var9], (byte)this.mnRotateDistance);
            self.mKey[var9] = lclRotateLeft_byte(self.mKey[var9], self.mnRotateDistance as u8);
        }
    }

    // Java: boolean verifyKey(short nKey, short nHash)
    pub fn verifyKey(&self, nKey: i16, nHash: i16) -> bool {
        // Java: return nKey == this.mnKey && nHash == this.mnHash;
        nKey == self.mnKey && nHash == self.mnHash
    }

    // Java: void initCipher()
    pub fn initCipher(&mut self) {
        // Java: this.mnOffset = 0;
        self.mnOffset = 0;
    }

    // Java: abstract byte[] decode(byte[] var1, int var2, int var3)
    // Java: abstract byte[] encode(byte[] var1, int var2, int var3)

    // Java: void skip(int nBytes)
    pub fn skip(&mut self, nBytes: i32) {
        // Java: this.mnOffset = this.mnOffset + nBytes & 15;
        self.mnOffset = (self.mnOffset + nBytes) & 15;
    }

    // Java: short getHash(byte[] passData, int nSize)
    // rust: ★M-STAGE1 P2 —— 字段 `mnKey`/`mnHash` 在 Java 中为**包内可见**，
    //   `OfficePasswordUtil.generateKey` 直接读 `info.m_pXOR.mnKey/mnHash`。
    //   Rust 的 `pub(crate)` 无法跨 crate ⇒ 补公开只读访问器，语义等价
    //   （Java 侧也从不写这两个字段，只读）。
    // Java: int key = info.m_pXOR.mnKey;
    pub fn getMnKey(&self) -> i16 {
        // Java: return this.mnKey;
        return self.mnKey;
    }

    // Java: int hash = info.m_pXOR.mnHash & '\uffff';
    pub fn getMnHash(&self) -> i16 {
        // Java: return this.mnHash;
        return self.mnHash;
    }

    pub fn getHash_static(passData: &[u8], nSize: usize) -> i16 {
        // Java: return CryptHelper.lclGetHash(passData, nSize);
        lclGetHash(passData, nSize)
    }

    // Java: short GetKey(byte[] passData, int nSize)
    pub fn GetKey(passData: &[u8], nSize: i32) -> i16 {
        // Java: return nSize >= 16 && nSize <= 60 ? g_more_key[nSize] : CryptHelper.lclGetKey(passData, nSize);
        if nSize >= 16 && nSize <= 60 {
            G_MORE_KEY[nSize as usize]
        } else {
            lclGetKey(passData, nSize as usize)
        }
    }

    pub fn decode(&mut self, _data: &mut [u8], _nOffset: i32, _nBytes: i32) -> Vec<u8> {
        Vec::new()
    }

    pub fn encode(&mut self, _data: &mut [u8], _nOffset: i32, _nBytes: i32) -> Vec<u8> {
        Vec::new()
    }
}
