// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use super::BorderCode::BorderCode;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}
fn le_get_ushort(data: &[u8], offset: usize) -> u16 {
    (data[offset] as u16) | ((data[offset + 1] as u16) << 8)
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8; data[offset + 2] = (value >> 16) as u8; data[offset + 3] = (value >> 24) as u8;
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}
fn le_put_ushort(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}

struct BitField { mask: i32, shift: i32 }
impl BitField {
    fn getValue(&self, holder: i32) -> i32 { (holder & self.mask) >> self.shift }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self.mask) | ((value << self.shift) & self.mask) }
    fn isSet(&self, holder: i32) -> bool { (holder & self.mask) != 0 }
    fn setBoolean(&self, holder: i32, flag: bool) -> i32 { if flag { holder | self.mask } else { holder & !self.mask } }
}

// Java field: int SIZE
pub static PICF_SIZE: i32 = 68;

// Java: public class PICF
pub struct PICF {
    // Java field: int lcb
    pub(crate) lcb: i32,
    // Java field: short cbHeader
    pub(crate) cbHeader: i16,
    // Java field: METAFILEPICT16 mfp
    pub(crate) mfp: METAFILEPICT16,
    // Java field: BITMAP16 bm
    pub(crate) bm: BITMAP16,
    // Java field: RECT16 rcWinMF
    pub(crate) rcWinMF: RECT16,
    // Java field: short dxaGoal
    pub(crate) dxaGoal: i16,
    // Java field: short dyaGoal
    pub(crate) dyaGoal: i16,
    // Java field: int mx
    pub(crate) mx: i32,
    // Java field: int my
    pub(crate) my: i32,
    // Java field: short dxaCropLeft
    pub(crate) dxaCropLeft: i16,
    // Java field: short dyaCropTop
    pub(crate) dyaCropTop: i16,
    // Java field: short dxaCropRight
    pub(crate) dxaCropRight: i16,
    // Java field: short dyaCropBottom
    pub(crate) dyaCropBottom: i16,
    // Java field: short info
    pub(crate) info: i16,
    // Java field: BitField brcl
    // Java field: BitField fFrameEmpty
    // Java field: BitField fBitmap
    // Java field: BitField fDrawHatch
    // Java field: BitField fError
    // Java field: BitField bpp
    // Java field: BorderCode[] brc
    pub(crate) brc: Vec<BorderCode>,
    // Java field: short dxaOrigin
    pub(crate) dxaOrigin: i16,
    // Java field: short dyaOrigin
    pub(crate) dyaOrigin: i16,
    // Java field: short cProps
    pub(crate) cProps: i16,
}

// Java: public class BITMAP16
pub struct BITMAP16 {
    // Java field: short bmType
    pub(crate) bmType: i16,
    // Java field: short bmWidth
    pub(crate) bmWidth: i16,
    // Java field: short bmHeight
    pub(crate) bmHeight: i16,
    // Java field: short bmWidthBytes
    pub(crate) bmWidthBytes: i16,
    // Java field: short bmPlanes
    pub(crate) bmPlanes: i16,
    // Java field: short bmBitsPixel
    pub(crate) bmBitsPixel: i16,
    // Java field: short bmBits
    pub(crate) bmBits: i16,
}

// Java: public class METAFILEPICT16
pub struct METAFILEPICT16 {
    // Java field: short mm
    pub(crate) mm: i16,
    // Java field: short xExt
    pub(crate) xExt: i16,
    // Java field: short yExt
    pub(crate) yExt: i16,
    // Java field: short hMF
    pub(crate) hMF: i16,
}

// Java: public class RECT16
pub struct RECT16 {
    // Java field: short top
    pub(crate) top: i16,
    // Java field: short left
    pub(crate) left: i16,
    // Java field: short right
    pub(crate) right: i16,
    // Java field: short bottom
    pub(crate) bottom: i16,
}

// Java: public class PICF
impl PICF {
    // Java: PICF()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mfp = new METAFILEPICT16();
        let mfp = METAFILEPICT16::new();
        // Java: this.bm = new BITMAP16();
        let bm = BITMAP16::new();
        // Java: this.rcWinMF = new RECT16();
        let rcWinMF = RECT16::new();
        let mut brc = Vec::with_capacity(4);
        // Java: for(int i = 0; i < this.brc.length; ++i)
        for _i in 0..4 {
            // Java: this.brc[i] = new BorderCode();
            brc.push(BorderCode::new());
        }
        PICF {
            lcb: 0, cbHeader: 0, mfp, bm, rcWinMF,
            dxaGoal: 0, dyaGoal: 0, mx: 0, my: 0,
            dxaCropLeft: 0, dyaCropTop: 0, dxaCropRight: 0, dyaCropBottom: 0,
            info: 0, brc, dxaOrigin: 0, dyaOrigin: 0, cProps: 0,
        }
    }

    // Java: PICF(byte[] buf, int offset)
    pub fn new_from_buf(buf: &[u8], offset: i32) -> Self {
        // Java: super();
        let mut off = offset as usize;
        // Java: this.lcb = LittleEndian.getInt(buf, offset);
        let lcb = le_get_int(buf, off);
        // Java: offset += 4;
        off += 4;
        // Java: this.cbHeader = LittleEndian.getShort(buf, offset);
        let cbHeader = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.mfp = new METAFILEPICT16(buf, offset);
        let mfp = METAFILEPICT16::new_from_buf(buf, off as i32);
        // Java: offset += 8;
        off += 8;
        // Java: this.bm = new BITMAP16(buf, offset);
        let bm = BITMAP16::new_from_buf(buf, off as i32);
        // Java: offset += 14;
        off += 14;
        // Java: this.dxaGoal = LittleEndian.getShort(buf, offset);
        let dxaGoal = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.dyaGoal = LittleEndian.getShort(buf, offset);
        let dyaGoal = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.mx = LittleEndian.getUShort(buf, offset);
        let mx = le_get_ushort(buf, off) as i32;
        // Java: offset += 2;
        off += 2;
        // Java: this.my = LittleEndian.getUShort(buf, offset);
        let my = le_get_ushort(buf, off) as i32;
        // Java: offset += 2;
        off += 2;
        // Java: this.dxaCropLeft = LittleEndian.getShort(buf, offset);
        let dxaCropLeft = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.dyaCropTop = LittleEndian.getShort(buf, offset);
        let dyaCropTop = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.dxaCropRight = LittleEndian.getShort(buf, offset);
        let dxaCropRight = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.dyaCropBottom = LittleEndian.getShort(buf, offset);
        let dyaCropBottom = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.info = LittleEndian.getShort(buf, offset);
        let info = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        let mut brc = Vec::with_capacity(4);
        // Java: for(int i = 0; i < this.brc.length; ++i)
        for _i in 0..4 {
            // Java: this.brc[i] = new BorderCode(buf, offset);
            brc.push(BorderCode::new_with_buf(buf, off as i32));
            // Java: offset += 4;
            off += 4;
        }
        // Java: this.dxaOrigin = LittleEndian.getShort(buf, offset);
        let dxaOrigin = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.dyaOrigin = LittleEndian.getShort(buf, offset);
        let dyaOrigin = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.cProps = LittleEndian.getShort(buf, offset);
        let cProps = le_get_short(buf, off);
        // Java: offset += 2;
        PICF {
            lcb, cbHeader, mfp, bm, rcWinMF: RECT16::new(),
            dxaGoal, dyaGoal, mx, my,
            dxaCropLeft, dyaCropTop, dxaCropRight, dyaCropBottom,
            info, brc, dxaOrigin, dyaOrigin, cProps,
        }
    }

    // Java: byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
        // Java: byte[] retval = new byte[this.size()];
        let mut retval = vec![0u8; self.size() as usize];
        // Java: int offset = 0;
        let mut off: usize = 0;
        // Java: LittleEndian.putInt(retval, offset, this.lcb);
        le_put_int(&mut retval, off, self.lcb);
        // Java: offset += 4;
        off += 4;
        // Java: LittleEndian.putUShort(retval, offset, this.cbHeader);
        le_put_ushort(&mut retval, off, self.cbHeader as i32);
        // Java: offset += 2;
        off += 2;
        // Java: System.arraycopy(this.mfp.serialize(), 0, retval, offset, 8);
        let mfp_data = self.mfp.serialize();
        retval[off..off + 8].copy_from_slice(&mfp_data[..8]);
        // Java: offset += 8;
        off += 8;
        // Java: System.arraycopy(this.bm.serialize(), 0, retval, offset, 14);
        let bm_data = self.bm.serialize();
        retval[off..off + 14].copy_from_slice(&bm_data[..14]);
        // Java: offset += 14;
        off += 14;
        // Java: LittleEndian.putShort(retval, offset, this.dxaGoal);
        le_put_short(&mut retval, off, self.dxaGoal);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dyaGoal);
        le_put_short(&mut retval, off, self.dyaGoal);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putUShort(retval, offset, this.mx);
        le_put_ushort(&mut retval, off, self.mx);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putUShort(retval, offset, this.my);
        le_put_ushort(&mut retval, off, self.my);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dxaCropLeft);
        le_put_short(&mut retval, off, self.dxaCropLeft);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dyaCropTop);
        le_put_short(&mut retval, off, self.dyaCropTop);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dxaCropRight);
        le_put_short(&mut retval, off, self.dxaCropRight);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dyaCropBottom);
        le_put_short(&mut retval, off, self.dyaCropBottom);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.info);
        le_put_short(&mut retval, off, self.info);
        // Java: offset += 2;
        off += 2;
        // Java: for(int i = 0; i < this.brc.length; ++i)
        for i in 0..self.brc.len() {
            // Java: this.brc[i].serialize(retval, offset);
            self.brc[i].serialize(&mut retval, off as i32);
            // Java: offset += 4;
            off += 4;
        }
        // Java: LittleEndian.putShort(retval, offset, this.dxaOrigin);
        le_put_short(&mut retval, off, self.dxaOrigin);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.dyaOrigin);
        le_put_short(&mut retval, off, self.dyaOrigin);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.cProps);
        le_put_short(&mut retval, off, self.cProps);
        // Java: offset += 2;
        // Java: return retval;
        retval
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return SIZE;
        PICF_SIZE
    }
}

// Java: public class BITMAP16
impl BITMAP16 {
    // Java: BITMAP16()
    pub fn new() -> Self {
        // Java: super();
        BITMAP16 { bmType: 0, bmWidth: 0, bmHeight: 0, bmWidthBytes: 0, bmPlanes: 0, bmBitsPixel: 0, bmBits: 0 }
    }

    // Java: BITMAP16(byte[] buf, int offset)
    pub fn new_from_buf(buf: &[u8], offset: i32) -> Self {
        // Java: super();
        let mut off = offset as usize;
        // Java: this.bmType = LittleEndian.getShort(buf, offset);
        let bmType = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmWidth = LittleEndian.getShort(buf, offset);
        let bmWidth = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmHeight = LittleEndian.getShort(buf, offset);
        let bmHeight = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmWidthBytes = LittleEndian.getShort(buf, offset);
        let bmWidthBytes = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmPlanes = LittleEndian.getShort(buf, offset);
        let bmPlanes = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmBitsPixel = LittleEndian.getShort(buf, offset);
        let bmBitsPixel = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.bmBits = LittleEndian.getShort(buf, offset);
        let bmBits = le_get_short(buf, off);
        BITMAP16 { bmType, bmWidth, bmHeight, bmWidthBytes, bmPlanes, bmBitsPixel, bmBits }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 14;
        14
    }

    // Java: byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
        // Java: byte[] retval = new byte[this.size()];
        let mut retval = vec![0u8; self.size() as usize];
        // Java: int offset = 0;
        let mut off: usize = 0;
        // Java: LittleEndian.putShort(retval, offset, this.bmType);
        le_put_short(&mut retval, off, self.bmType);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmWidth);
        le_put_short(&mut retval, off, self.bmWidth);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmHeight);
        le_put_short(&mut retval, off, self.bmHeight);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmWidthBytes);
        le_put_short(&mut retval, off, self.bmWidthBytes);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmPlanes);
        le_put_short(&mut retval, off, self.bmPlanes);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmBitsPixel);
        le_put_short(&mut retval, off, self.bmBitsPixel);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bmBits);
        le_put_short(&mut retval, off, self.bmBits);
        // Java: offset += 2;
        // Java: return retval;
        retval
    }
}

// Java: public class METAFILEPICT16
impl METAFILEPICT16 {
    // Java: METAFILEPICT16()
    pub fn new() -> Self {
        // Java: super();
        METAFILEPICT16 { mm: 0, xExt: 0, yExt: 0, hMF: 0 }
    }

    // Java: METAFILEPICT16(byte[] buf, int offset)
    pub fn new_from_buf(buf: &[u8], offset: i32) -> Self {
        // Java: super();
        let mut off = offset as usize;
        // Java: this.mm = LittleEndian.getShort(buf, offset);
        let mm = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.xExt = LittleEndian.getShort(buf, offset);
        let xExt = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.yExt = LittleEndian.getShort(buf, offset);
        let yExt = le_get_short(buf, off);
        // Java: offset += 2;
        off += 2;
        // Java: this.hMF = LittleEndian.getShort(buf, offset);
        let hMF = le_get_short(buf, off);
        METAFILEPICT16 { mm, xExt, yExt, hMF }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 8;
        8
    }

    // Java: byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
        // Java: byte[] retval = new byte[this.size()];
        let mut retval = vec![0u8; self.size() as usize];
        // Java: int offset = 0;
        let mut off: usize = 0;
        // Java: LittleEndian.putShort(retval, offset, this.mm);
        le_put_short(&mut retval, off, self.mm);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.xExt);
        le_put_short(&mut retval, off, self.xExt);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.yExt);
        le_put_short(&mut retval, off, self.yExt);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.hMF);
        le_put_short(&mut retval, off, self.hMF);
        // Java: offset += 2;
        // Java: return retval;
        retval
    }
}

// Java: public class RECT16
impl RECT16 {
    // Java: RECT16()
    pub fn new() -> Self {
        // Java: super();
        RECT16 { top: 0, left: 0, right: 0, bottom: 0 }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 8;
        8
    }

    // Java: byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
        // Java: byte[] retval = new byte[this.size()];
        let mut retval = vec![0u8; self.size() as usize];
        // Java: int offset = 0;
        let mut off: usize = 0;
        // Java: LittleEndian.putShort(retval, offset, this.top);
        le_put_short(&mut retval, off, self.top);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.left);
        le_put_short(&mut retval, off, self.left);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.right);
        le_put_short(&mut retval, off, self.right);
        // Java: offset += 2;
        off += 2;
        // Java: LittleEndian.putShort(retval, offset, this.bottom);
        le_put_short(&mut retval, off, self.bottom);
        // Java: offset += 2;
        // Java: return retval;
        retval
    }
}
