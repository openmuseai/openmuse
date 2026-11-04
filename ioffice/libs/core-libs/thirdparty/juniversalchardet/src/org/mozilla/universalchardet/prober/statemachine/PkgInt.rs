// Java field: int INDEX_SHIFT_4BITS
pub(crate) const INDEX_SHIFT_4BITS: i32 = 3;
// Java field: int INDEX_SHIFT_8BITS
pub(crate) const INDEX_SHIFT_8BITS: i32 = 2;
// Java field: int INDEX_SHIFT_16BITS
pub(crate) const INDEX_SHIFT_16BITS: i32 = 1;
// Java field: int SHIFT_MASK_4BITS
pub(crate) const SHIFT_MASK_4BITS: i32 = 7;
// Java field: int SHIFT_MASK_8BITS
pub(crate) const SHIFT_MASK_8BITS: i32 = 3;
// Java field: int SHIFT_MASK_16BITS
pub(crate) const SHIFT_MASK_16BITS: i32 = 1;
// Java field: int BIT_SHIFT_4BITS
pub(crate) const BIT_SHIFT_4BITS: i32 = 2;
// Java field: int BIT_SHIFT_8BITS
pub(crate) const BIT_SHIFT_8BITS: i32 = 3;
// Java field: int BIT_SHIFT_16BITS
pub(crate) const BIT_SHIFT_16BITS: i32 = 4;
// Java field: int UNIT_MASK_4BITS
pub(crate) const UNIT_MASK_4BITS: i32 = 15;
// Java field: int UNIT_MASK_8BITS
pub(crate) const UNIT_MASK_8BITS: i32 = 255;
// Java field: int UNIT_MASK_16BITS
pub(crate) const UNIT_MASK_16BITS: i32 = 65535;

// Java: public class PkgInt
pub struct PkgInt {
    // Java field: int indexShift
    index_shift: i32,
    // Java field: int shiftMask
    shift_mask: i32,
    // Java field: int bitShift
    bit_shift: i32,
    // Java field: int unitMask
    unit_mask: i32,
    // Java field: int[] data
    pub(crate) data: &'static [i32],
}

// Java: public class PkgInt
impl PkgInt {
    // Java: PkgInt(int indexShift, int shiftMask, int bitShift, int unitMask, int[] data)
    pub fn new(index_shift: i32, shift_mask: i32, bit_shift: i32, unit_mask: i32, data: &'static [i32]) -> Self {
        // Java: super();
        // Java: this.indexShift = indexShift;
        // Java: this.shiftMask = shiftMask;
        // Java: this.bitShift = bitShift;
        // Java: this.unitMask = unitMask;
        // Java: this.data = data;
        Self {
            index_shift,
            shift_mask,
            bit_shift,
            unit_mask,
            data,
        }
    }

    // Java: static int pack16bits(int a, int b)
    pub fn pack16bits(a: i32, b: i32) -> i32 {
        // Java: return b << 16 | a;
        b << 16 | a
    }

    // Java: static int pack8bits(int a, int b, int c, int d)
    pub fn pack8bits(a: i32, b: i32, c: i32, d: i32) -> i32 {
        // Java: return pack16bits(b << 8 | a, d << 8 | c);
        Self::pack16bits(b << 8 | a, d << 8 | c)
    }

    // Java: static int pack4bits(int a, int b, int c, int d, int e, int f, int g, int h)
    pub fn pack4bits(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        // Java: return pack8bits(b << 4 | a, d << 4 | c, f << 4 | e, h << 4 | g);
        Self::pack8bits(b << 4 | a, d << 4 | c, f << 4 | e, h << 4 | g)
    }

    // Java: int unpack(int i)
    pub fn unpack(&self, i: i32) -> i32 {
        // Java: return this.data[i >> this.indexShift] >> ((i & this.shiftMask) << this.bitShift) & this.unitMask;
        self.data[(i >> self.index_shift) as usize] >> ((i & self.shift_mask) << self.bit_shift) & self.unit_mask
    }
}
