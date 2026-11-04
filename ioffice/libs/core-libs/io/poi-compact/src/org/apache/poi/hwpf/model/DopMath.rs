// Java: import cn.wps.base.assertion.IOAssert
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8; data[offset + 2] = (value >> 16) as u8; data[offset + 3] = (value >> 24) as u8;
}
fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8;
}

struct BitField { mask: i32, shift: i32 }
impl BitField {
    fn new(mask: i32) -> Self {
        let mut shift = 0;
        if mask != 0 { let mut m = mask; while (m & 1) == 0 { m >>= 1; shift += 1; } }
        BitField { mask, shift }
    }
    fn getValue(&self, holder: i32) -> i32 { (holder & self.mask) >> self.shift }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self.mask) | ((value << self.shift) & self.mask) }
    fn isSet(&self, holder: i32) -> bool { (holder & self.mask) != 0 }
    fn setBoolean(&self, holder: i32, flag: bool) -> i32 { if flag { holder | self.mask } else { holder & !self.mask } }
}

const MTHBRK: BitField = BitField { mask: 3, shift: 0 };
const MTHBRK_SUB: BitField = BitField { mask: 12, shift: 2 };
const MTHBPJC: BitField = BitField { mask: 112, shift: 4 };
const RESERVED1_BF: BitField = BitField { mask: 128, shift: 7 };
const F_MATH_SMALL_FRAC: BitField = BitField { mask: 256, shift: 8 };
const F_MATH_INT_LIM_UND_OVR: BitField = BitField { mask: 512, shift: 9 };
const F_MATH_NARY_LIM_UND_OVR: BitField = BitField { mask: 1024, shift: 10 };
const F_MATH_WRAP_ALIGN_LEFT: BitField = BitField { mask: 2048, shift: 11 };
const F_MATH_USE_DISP_DEFAULTS: BitField = BitField { mask: 4096, shift: 12 };
const RESERVED2_BF: BitField = BitField { mask: -8192i32, shift: 13 };

// Java: public class DopMath
pub struct DopMath {
    // Java field: int DOPMATH_SIZE
    // Java field: int field_reserved1
    pub(crate) field_reserved1: i32,
    // Java field: BitField mthbrk
    // Java field: BitField mthbrkSub
    // Java field: BitField mthbpjc
    // Java field: BitField reserved1
    // Java field: BitField fMathSmallFrac
    // Java field: BitField fMathIntLimUndOvr
    // Java field: BitField fMathNaryLimUndOvr
    // Java field: BitField fMathWrapAlignLeft
    // Java field: BitField fMathUseDispDefaults
    // Java field: BitField reserved2
    // Java field: short field_ftcMath
    pub(crate) field_ftcMath: i16,
    // Java field: int field_dxaLeftMargin
    pub(crate) field_dxaLeftMargin: i32,
    // Java field: int field_dxaRightMargin
    pub(crate) field_dxaRightMargin: i32,
    // Java field: int field_empty1
    pub(crate) field_empty1: i32,
    // Java field: int field_empty2
    pub(crate) field_empty2: i32,
    // Java field: int field_empty3
    pub(crate) field_empty3: i32,
    // Java field: int field_empty4
    pub(crate) field_empty4: i32,
    // Java field: int field_dxaIndentWrapped
    pub(crate) field_dxaIndentWrapped: i32,
    // Java field: byte[] field_math
    pub(crate) field_math: Option<Vec<u8>>,
}

pub const DOPMATH_SIZE: i32 = 34;

// Java: public class DopMath
impl DopMath {
    // Java: DopMath()
    pub fn new() -> Self {
        // Java: super();
        DopMath { field_reserved1: 0, field_ftcMath: 0, field_dxaLeftMargin: 0, field_dxaRightMargin: 0, field_empty1: 0, field_empty2: 0, field_empty3: 0, field_empty4: 0, field_dxaIndentWrapped: 0, field_math: None }
    }

    // Java: DopMath(byte[] data)
    pub fn new_with_data(data: Vec<u8>) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("data should not be null", data);
        // Java: this.field_math = data;
        DopMath { field_reserved1: 0, field_ftcMath: 0, field_dxaLeftMargin: 0, field_dxaRightMargin: 0, field_empty1: 0, field_empty2: 0, field_empty3: 0, field_empty4: 0, field_dxaIndentWrapped: 0, field_math: Some(data) }
    }

    // Java: void fillFields()
    pub fn fillFields(&mut self) {
        let m = self.field_math.as_ref().unwrap();
        // Java: this.field_reserved1 = LittleEndian.getInt(this.field_math, 0);
        self.field_reserved1 = le_get_int(m, 0);
        // Java: this.field_ftcMath = LittleEndian.getShort(this.field_math, 4);
        self.field_ftcMath = le_get_short(m, 4);
        // Java: this.field_dxaLeftMargin = LittleEndian.getShort(this.field_math, 6);
        self.field_dxaLeftMargin = le_get_short(m, 6) as i32;
        // Java: this.field_dxaRightMargin = LittleEndian.getShort(this.field_math, 10);
        self.field_dxaRightMargin = le_get_short(m, 10) as i32;
        // Java: this.field_empty1 = LittleEndian.getShort(this.field_math, 14);
        self.field_empty1 = le_get_short(m, 14) as i32;
        // Java: this.field_empty2 = LittleEndian.getShort(this.field_math, 18);
        self.field_empty2 = le_get_short(m, 18) as i32;
        // Java: this.field_empty3 = LittleEndian.getShort(this.field_math, 22);
        self.field_empty3 = le_get_short(m, 22) as i32;
        // Java: this.field_empty4 = LittleEndian.getShort(this.field_math, 26);
        self.field_empty4 = le_get_short(m, 26) as i32;
        // Java: this.field_dxaIndentWrapped = LittleEndian.getShort(this.field_math, 30);
        self.field_dxaIndentWrapped = le_get_short(m, 30) as i32;
    }

    // Java: void serialize(byte[] dopData, int offset)
    pub fn serialize(&self, dopData: &mut [u8], offset: i32) {
        let off = offset as usize;
        // Java: IOAssert.assertNotNull("dopData should not be null", dopData);
        // Java: LittleEndian.putInt(dopData, offset, this.field_reserved1);
        le_put_int(dopData, off, self.field_reserved1);
        // Java: LittleEndian.putShort(dopData, offset + 4, this.field_ftcMath);
        le_put_short(dopData, off + 4, self.field_ftcMath);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 4, this.field_dxaLeftMargin);
        le_put_int(dopData, off + 6, self.field_dxaLeftMargin);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 8, this.field_dxaRightMargin);
        le_put_int(dopData, off + 10, self.field_dxaRightMargin);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 12, this.field_empty1);
        le_put_int(dopData, off + 14, self.field_empty1);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 16, this.field_empty2);
        le_put_int(dopData, off + 18, self.field_empty2);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 20, this.field_empty3);
        le_put_int(dopData, off + 22, self.field_empty3);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 24, this.field_empty4);
        le_put_int(dopData, off + 26, self.field_empty4);
        // Java: LittleEndian.putInt(dopData, offset + 2 + 28, this.field_dxaIndentWrapped);
        le_put_int(dopData, off + 30, self.field_dxaIndentWrapped);
    }

    // Java: void setMthbrk(int value)
    pub fn setMthbrk(&mut self, value: i32) {
        // Java: this.field_reserved1 = mthbrk.setValue(this.field_reserved1, value);
        self.field_reserved1 = MTHBRK.setValue(self.field_reserved1, value);
    }
    // Java: int getMthbrk()
    pub fn getMthbrk(&self) -> i32 {
        // Java: return mthbrk.getValue(this.field_reserved1);
        MTHBRK.getValue(self.field_reserved1)
    }
    // Java: void setMthbrkSub(int value)
    pub fn setMthbrkSub(&mut self, value: i32) {
        // Java: this.field_reserved1 = mthbrkSub.setValue(this.field_reserved1, value);
        self.field_reserved1 = MTHBRK_SUB.setValue(self.field_reserved1, value);
    }
    // Java: int getMthbrkSub()
    pub fn getMthbrkSub(&self) -> i32 {
        // Java: return mthbrkSub.getValue(this.field_reserved1);
        MTHBRK_SUB.getValue(self.field_reserved1)
    }
    // Java: void setMthbpjc(int value)
    pub fn setMthbpjc(&mut self, value: i32) {
        // Java: this.field_reserved1 = mthbpjc.setValue(this.field_reserved1, value);
        self.field_reserved1 = MTHBPJC.setValue(self.field_reserved1, value);
    }
    // Java: int getMthbpjc()
    pub fn getMthbpjc(&self) -> i32 {
        // Java: return mthbpjc.getValue(this.field_reserved1);
        MTHBPJC.getValue(self.field_reserved1)
    }
    // Java: void setReserved1(boolean value)
    pub fn setReserved1(&mut self, value: bool) {
        // Java: this.field_reserved1 = reserved1.setBoolean(this.field_reserved1, value);
        self.field_reserved1 = RESERVED1_BF.setBoolean(self.field_reserved1, value);
    }
    // Java: boolean isReserved1()
    pub fn isReserved1(&self) -> bool {
        // Java: return reserved1.isSet(this.field_reserved1);
        RESERVED1_BF.isSet(self.field_reserved1)
    }
    // Java: void setFMathSmallFrac(boolean value)
    pub fn setFMathSmallFrac(&mut self, value: bool) {
        // Java: this.field_reserved1 = fMathSmallFrac.setBoolean(this.field_reserved1, value);
        self.field_reserved1 = F_MATH_SMALL_FRAC.setBoolean(self.field_reserved1, value);
    }
    // Java: boolean isFMathSmallFrac()
    pub fn isFMathSmallFrac(&self) -> bool {
        // Java: return fMathSmallFrac.isSet(this.field_reserved1);
        F_MATH_SMALL_FRAC.isSet(self.field_reserved1)
    }
    // Java: void setFMathIntLimUndOvr(int value)
    pub fn setFMathIntLimUndOvr(&mut self, value: i32) {
        // Java: this.field_reserved1 = fMathIntLimUndOvr.setValue(this.field_reserved1, value);
        self.field_reserved1 = F_MATH_INT_LIM_UND_OVR.setValue(self.field_reserved1, value);
    }
    // Java: int getFMathIntLimUndOvr()
    pub fn getFMathIntLimUndOvr(&self) -> i32 {
        // Java: return fMathIntLimUndOvr.getValue(this.field_reserved1);
        F_MATH_INT_LIM_UND_OVR.getValue(self.field_reserved1)
    }
    // Java: void setFMathNaryLimUndOvr(int value)
    pub fn setFMathNaryLimUndOvr(&mut self, value: i32) {
        // Java: this.field_reserved1 = fMathNaryLimUndOvr.setValue(this.field_reserved1, value);
        self.field_reserved1 = F_MATH_NARY_LIM_UND_OVR.setValue(self.field_reserved1, value);
    }
    // Java: int getFMathNaryLimUndOvr()
    pub fn getFMathNaryLimUndOvr(&self) -> i32 {
        // Java: return fMathNaryLimUndOvr.getValue(this.field_reserved1);
        F_MATH_NARY_LIM_UND_OVR.getValue(self.field_reserved1)
    }
    // Java: void setFMathWrapAlignLeft(boolean value)
    pub fn setFMathWrapAlignLeft(&mut self, value: bool) {
        // Java: this.field_reserved1 = fMathWrapAlignLeft.setBoolean(this.field_reserved1, value);
        self.field_reserved1 = F_MATH_WRAP_ALIGN_LEFT.setBoolean(self.field_reserved1, value);
    }
    // Java: boolean isFMathWrapAlignLeft()
    pub fn isFMathWrapAlignLeft(&self) -> bool {
        // Java: return fMathWrapAlignLeft.isSet(this.field_reserved1);
        F_MATH_WRAP_ALIGN_LEFT.isSet(self.field_reserved1)
    }
    // Java: void setFMathUseDispDefaults(boolean value)
    pub fn setFMathUseDispDefaults(&mut self, value: bool) {
        // Java: this.field_reserved1 = fMathUseDispDefaults.setBoolean(this.field_reserved1, value);
        self.field_reserved1 = F_MATH_USE_DISP_DEFAULTS.setBoolean(self.field_reserved1, value);
    }
    // Java: boolean isFMathUseDispDefaults()
    pub fn isFMathUseDispDefaults(&self) -> bool {
        // Java: return fMathUseDispDefaults.isSet(this.field_reserved1);
        F_MATH_USE_DISP_DEFAULTS.isSet(self.field_reserved1)
    }
    // Java: void setReserved2(int value)
    pub fn setReserved2(&mut self, value: i32) {
        // Java: this.field_reserved1 = reserved2.setValue(this.field_reserved1, value);
        self.field_reserved1 = RESERVED2_BF.setValue(self.field_reserved1, value);
    }
    // Java: int getReserved2()
    pub fn getReserved2(&self) -> i32 {
        // Java: return reserved2.getValue(this.field_reserved1);
        RESERVED2_BF.getValue(self.field_reserved1)
    }
    // Java: void setFtcMath(int value)
    pub fn setFtcMath(&mut self, value: i32) {
        // Java: this.field_ftcMath = (short)value;
        self.field_ftcMath = value as i16;
    }
    // Java: int getFtcMath()
    pub fn getFtcMath(&self) -> i32 {
        // Java: return this.field_ftcMath;
        self.field_ftcMath as i32
    }
    // Java: void setDxaLeftMargin(int value)
    pub fn setDxaLeftMargin(&mut self, value: i32) {
        // Java: this.field_dxaLeftMargin = value;
        self.field_dxaLeftMargin = value;
    }
    // Java: int getDxaLeftMargin()
    pub fn getDxaLeftMargin(&self) -> i32 {
        // Java: return this.field_dxaLeftMargin;
        self.field_dxaLeftMargin
    }
    // Java: void setDxaRightMargin(int value)
    pub fn setDxaRightMargin(&mut self, value: i32) {
        // Java: this.field_dxaRightMargin = value;
        self.field_dxaRightMargin = value;
    }
    // Java: int getDxaRightMargin()
    pub fn getDxaRightMargin(&self) -> i32 {
        // Java: return this.field_dxaRightMargin;
        self.field_dxaRightMargin
    }
    // Java: void setEmpty1(int value)
    pub fn setEmpty1(&mut self, value: i32) {
        // Java: this.field_empty1 = value;
        self.field_empty1 = value;
    }
    // Java: int getEmpty1()
    pub fn getEmpty1(&self) -> i32 {
        // Java: return this.field_empty1;
        self.field_empty1
    }
    // Java: void setEmpty2(int value)
    pub fn setEmpty2(&mut self, value: i32) {
        // Java: this.field_empty2 = value;
        self.field_empty2 = value;
    }
    // Java: int getEmpty2()
    pub fn getEmpty2(&self) -> i32 {
        // Java: return this.field_empty2;
        self.field_empty2
    }
    // Java: void setEmpty3(int value)
    pub fn setEmpty3(&mut self, value: i32) {
        // Java: this.field_empty3 = value;
        self.field_empty3 = value;
    }
    // Java: int getEmpty3()
    pub fn getEmpty3(&self) -> i32 {
        // Java: return this.field_empty3;
        self.field_empty3
    }
    // Java: void setEmpty4(int value)
    pub fn setEmpty4(&mut self, value: i32) {
        // Java: this.field_empty4 = value;
        self.field_empty4 = value;
    }
    // Java: int getEmpty4()
    pub fn getEmpty4(&self) -> i32 {
        // Java: return this.field_empty4;
        self.field_empty4
    }
    // Java: void setDxaIndentWrapped(int value)
    pub fn setDxaIndentWrapped(&mut self, value: i32) {
        // Java: this.field_dxaIndentWrapped = value;
        self.field_dxaIndentWrapped = value;
    }
    // Java: int getDxaIndentWrapped()
    pub fn getDxaIndentWrapped(&self) -> i32 {
        // Java: return this.field_dxaIndentWrapped;
        self.field_dxaIndentWrapped
    }
}
