// Java: import cn.wps.base.assertion.IOAssert
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

use super::DopMath::DopMath;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8; data[offset + 1] = (value >> 8) as u8; data[offset + 2] = (value >> 16) as u8; data[offset + 3] = (value >> 24) as u8;
}

struct BitField { mask: i32, shift: i32 }
impl BitField {
    const fn new(mask: i32) -> Self {
        let mut shift = 0;
        if mask != 0 {
            let mut m = mask;
            while (m & 1) == 0 { m >>= 1; shift += 1; }
        }
        BitField { mask, shift }
    }
    fn getValue(&self, holder: i32) -> i32 { (holder & self.mask) >> self.shift }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self.mask) | ((value << self.shift) & self.mask) }
    fn isSet(&self, holder: i32) -> bool { (holder & self.mask) != 0 }
    fn setBoolean(&self, holder: i32, flag: bool) -> i32 { if flag { holder | self.mask } else { holder & !self.mask } }
}

// Java field: int DOP2007_SIZE
pub(crate) const DOP2007_SIZE: i32 = 58;

static BF_FRMTRACKFORMATTING: BitField = BitField { mask: 1, shift: 0 };
static BF_FRMTRACKMOVES: BitField = BitField { mask: 2, shift: 1 };
static BF_RESERVED2: BitField = BitField { mask: 4, shift: 2 };
static BF_EMPTY1: BitField = BitField { mask: 8, shift: 3 };
static BF_EMPTY2: BitField = BitField { mask: 16, shift: 4 };
static BF_SSM: BitField = BitField { mask: 480, shift: 5 };
static BF_FREADINGMODEINKLOCKDOWNACTUALPAGE: BitField = BitField { mask: 512, shift: 9 };
static BF_FAUTOCOMPRESSPICTURES: BitField = BitField { mask: 1024, shift: 10 };
static BF_RESERVED3: BitField = BitField { mask: -2048, shift: 11 };

// Java: public class Dop2007
pub struct Dop2007 {
    // Java field: int field_reserved1
    pub(crate) field_reserved1: i32,
    // Java field: int field_reserved2
    pub(crate) field_reserved2: i32,
    // Java field: BitField fRMTrackFormatting
    // Java field: BitField fRMTrackMoves
    // Java field: BitField reserved2
    // Java field: BitField empty1
    // Java field: BitField empty2
    // Java field: BitField ssm
    // Java field: BitField fReadingModeInkLockDownActualPage
    // Java field: BitField fAutoCompressPictures
    // Java field: BitField reserved3
    // Java field: int field_empty3
    pub(crate) field_empty3: i32,
    // Java field: int field_empty4
    pub(crate) field_empty4: i32,
    // Java field: int field_empty5
    pub(crate) field_empty5: i32,
    // Java field: int field_empty6
    pub(crate) field_empty6: i32,
    // Java field: DopMath field_DopMath
    pub(crate) field_DopMath: Option<DopMath>,
    // Java field: byte[] mDop07
    pub(crate) mDop07: Vec<u8>,
}

// Java: public class Dop2007
impl Dop2007 {
    // Java: Dop2007(byte[] data)
    pub fn new(data: Vec<u8>) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("data should not be null", data);
        // Java: this.mDop07 = data;
        Dop2007 {
            field_reserved1: 0,
            field_reserved2: 0,
            field_empty3: 0,
            field_empty4: 0,
            field_empty5: 0,
            field_empty6: 0,
            field_DopMath: None,
            mDop07: data,
        }
    }

    // Java: Dop2007(byte[] data, DopMath dopMath)
    pub fn new_with_dop_math(data: Vec<u8>, dopMath: DopMath) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("data should not be null", data);
        // Java: IOAssert.assertNotNull("dopMath should not be null", dopMath);
        // Java: this.mDop07 = data;
        // Java: this.field_DopMath = dopMath;
        Dop2007 {
            field_reserved1: 0,
            field_reserved2: 0,
            field_empty3: 0,
            field_empty4: 0,
            field_empty5: 0,
            field_empty6: 0,
            field_DopMath: Some(dopMath),
            mDop07: data,
        }
    }

    // Java: void fillFields()
    pub fn fillFields(&mut self) {
        // Java: IOAssert.assertNotNull("mDop07 should not be null", this.mDop07);
        // Java: if (this.mDop07.length >= 58)
        if self.mDop07.len() >= 58 {
            // Java: this.field_reserved1 = LittleEndian.getInt(this.mDop07, 0);
            self.field_reserved1 = le_get_int(&self.mDop07, 0);
            // Java: this.field_reserved2 = LittleEndian.getInt(this.mDop07, 4);
            self.field_reserved2 = le_get_int(&self.mDop07, 4);
            // Java: this.field_empty3 = LittleEndian.getInt(this.mDop07, 8);
            self.field_empty3 = le_get_int(&self.mDop07, 8);
            // Java: this.field_empty4 = LittleEndian.getInt(this.mDop07, 12);
            self.field_empty4 = le_get_int(&self.mDop07, 12);
            // Java: this.field_empty5 = LittleEndian.getInt(this.mDop07, 16);
            self.field_empty5 = le_get_int(&self.mDop07, 16);
            // Java: this.field_empty6 = LittleEndian.getInt(this.mDop07, 20);
            self.field_empty6 = le_get_int(&self.mDop07, 20);
            // Java: byte[] mathData = new byte[34];
            let mut mathData = vec![0u8; 34];
            // Java: System.arraycopy(this.mDop07, 24, mathData, 0, 34);
            mathData.copy_from_slice(&self.mDop07[24..58]);
            // Java: this.field_DopMath = new DopMath(mathData);
            let mut dopMath = DopMath::new_with_data(mathData);
            // Java: this.field_DopMath.fillFields();
            dopMath.fillFields();
            self.field_DopMath = Some(dopMath);
        }
    }

    // Java: void serialize(byte[] dopData, int offset)
    pub fn serialize(&mut self, dopData: &mut [u8], offset: i32) {
        // Java: IOAssert.assertNotNull("dopData should not be null", dopData);
        // Java: IOAssert.assertNotNull("mDop07 should not be null", this.mDop07);
        // Java: if (this.mDop07.length >= 58)
        if self.mDop07.len() >= 58 {
            // Java: LittleEndian.putInt(this.mDop07, 0, this.field_reserved1);
            le_put_int(&mut self.mDop07, 0, self.field_reserved1);
            // Java: LittleEndian.putInt(this.mDop07, 4, this.field_reserved2);
            le_put_int(&mut self.mDop07, 4, self.field_reserved2);
            // Java: LittleEndian.putInt(this.mDop07, 8, this.field_empty3);
            le_put_int(&mut self.mDop07, 8, self.field_empty3);
            // Java: LittleEndian.putInt(this.mDop07, 12, this.field_empty4);
            le_put_int(&mut self.mDop07, 12, self.field_empty4);
            // Java: LittleEndian.putInt(this.mDop07, 16, this.field_empty5);
            le_put_int(&mut self.mDop07, 16, self.field_empty5);
            // Java: LittleEndian.putInt(this.mDop07, 20, this.field_empty6);
            le_put_int(&mut self.mDop07, 20, self.field_empty6);
            // Java: this.field_DopMath.serialize(this.mDop07, 24);
            if let Some(ref mut dm) = self.field_DopMath {
                dm.serialize(&mut self.mDop07, 24);
            }
        }
        // Java: System.arraycopy(this.mDop07, 0, dopData, offset, this.mDop07.length);
        let off = offset as usize;
        let len = self.mDop07.len();
        dopData[off..off + len].copy_from_slice(&self.mDop07);
    }

    // Java: void setFRMTrackFormatting(boolean value)
    pub fn setFRMTrackFormatting(&mut self, value: bool) {
        // Java: if (this.mDop07.length > 4)
        if self.mDop07.len() > 4 {
            // Java: this.field_reserved2 = fRMTrackFormatting.setBoolean(this.field_reserved2, value);
            self.field_reserved2 = BF_FRMTRACKFORMATTING.setBoolean(self.field_reserved2, value);
        }
    }

    // Java: boolean isFRMTrackFormatting()
    pub fn isFRMTrackFormatting(&self) -> bool {
        // Java: return this.mDop07.length > 4 ? fRMTrackFormatting.isSet(this.field_reserved2) : false;
        if self.mDop07.len() > 4 { BF_FRMTRACKFORMATTING.isSet(self.field_reserved2) } else { false }
    }

    // Java: void setFRMTrackMoves(boolean value)
    pub fn setFRMTrackMoves(&mut self, value: bool) {
        // Java: if (this.mDop07.length > 4)
        if self.mDop07.len() > 4 {
            // Java: this.field_reserved2 = fRMTrackMoves.setBoolean(this.field_reserved2, value);
            self.field_reserved2 = BF_FRMTRACKMOVES.setBoolean(self.field_reserved2, value);
        }
    }

    // Java: boolean isFRMTrackMoves()
    pub fn isFRMTrackMoves(&self) -> bool {
        // Java: return this.mDop07.length > 4 ? fRMTrackMoves.isSet(this.field_reserved2) : false;
        if self.mDop07.len() > 4 { BF_FRMTRACKMOVES.isSet(self.field_reserved2) } else { false }
    }

    // Java: void setSsm(byte value)
    pub fn setSsm(&mut self, value: i8) {
        // Java: if (this.mDop07.length > 4)
        if self.mDop07.len() > 4 {
            // Java: this.field_reserved2 = ssm.setValue(this.field_reserved2, value);
            self.field_reserved2 = BF_SSM.setValue(self.field_reserved2, value as i32);
        }
    }

    // Java: byte getSsm()
    pub fn getSsm(&self) -> i8 {
        // Java: return this.mDop07.length > 4 ? (byte)ssm.getValue(this.field_reserved2) : 1;
        if self.mDop07.len() > 4 { BF_SSM.getValue(self.field_reserved2) as i8 } else { 1 }
    }

    // Java: DopMath getDopMath()
    pub fn getDopMath(&self) -> Option<&DopMath> {
        // Java: return this.field_DopMath;
        self.field_DopMath.as_ref()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.mDop07.length;
        self.mDop07.len() as i32
    }
}
