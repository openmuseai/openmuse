// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class PieceDescriptor
pub struct PieceDescriptor {
    // Java field: short descriptor
    pub(crate) descriptor: i16,
    // Java field: BitField fNoParaLast
    // Java field: BitField fPaphNil
    // Java field: BitField fCopied
    // Java field: int fc
    pub(crate) fc: i32,
    // Java field: short prm
    pub(crate) prm: i16,
    // Java field: Prm0 prm0
    pub(crate) prm0: Option<Prm0>,
    // Java field: Prm1 prm1
    pub(crate) prm1: Option<Prm1>,
    // Java field: int PRM_TYPE_0
    // Java field: int PRM_TYPE_1
    // Java field: boolean unicode
    pub(crate) unicode: bool,
}

pub const PRM_TYPE_0: i32 = 0;
pub const PRM_TYPE_1: i32 = 1;

// Java: public class Prm0
pub struct Prm0 {
    // Java field: byte isprm
    pub isprm: i8,
    // Java field: byte val
    pub val: i8,
}

// Java: public class Prm1
pub struct Prm1 {
    // Java field: short data
    data: i16,
}

// Java: public class PieceDescriptor
impl PieceDescriptor {
    // Java: PieceDescriptor(byte[] buf, int offset)
    pub fn new(buf: &[u8], mut offset: i32) -> Self {
        // Java: super();
        // Java: this.descriptor = LittleEndian.getShort(buf, offset);
        let descriptor = le_get_short(buf, offset as usize);
        // Java: offset += 2;
        offset += 2;
        // Java: this.fc = LittleEndian.getInt(buf, offset);
        let mut fc = le_get_int(buf, offset as usize);
        // Java: offset += 4;
        offset += 4;
        // Java: this.prm = LittleEndian.getShort(buf, offset);
        let prm = le_get_short(buf, offset as usize);
        let unicode;
        // Java: if ((this.fc & 1073741824) == 0)
        if (fc & 1073741824) == 0 {
            // Java: this.unicode = true;
            unicode = true;
        }
        // Java: else
        else {
            // Java: this.unicode = false;
            unicode = false;
            // Java: this.fc &= -1073741825;
            fc &= -1073741825i32;
            // Java: this.fc /= 2;
            fc /= 2;
        }
        PieceDescriptor {
            descriptor,
            fc,
            prm,
            prm0: None,
            prm1: None,
            unicode,
        }
    }

    // Java: int getFilePosition()
    pub fn getFilePosition(&self) -> i32 {
        // Java: return this.fc;
        self.fc
    }

    // Java: void setFilePosition(int pos)
    pub fn setFilePosition(&mut self, pos: i32) {
        // Java: this.fc = pos;
        self.fc = pos;
    }

    // Java: void forceUnicode()
    pub fn forceUnicode(&mut self) {
        // Java: if (!this.unicode)
        if !self.unicode {
            // Java: this.unicode = true;
            self.unicode = true;
            // Java: this.fc *= 2;
            self.fc *= 2;
            // Java: this.fc |= 1073741824;
            self.fc |= 1073741824;
        }
    }

    // Java: boolean isUnicode()
    pub fn isUnicode(&self) -> bool {
        // Java: return this.unicode;
        self.unicode
    }

    // Java: short getPrm()
    pub fn getPrm(&self) -> i16 {
        // Java: return this.prm;
        self.prm
    }

    // Java: int getPrmType()
    pub fn getPrmType(&self) -> i32 {
        // Java: return (this.prm & 1) == 1 ? 1 : 0;
        if (self.prm & 1) == 1 { 1 } else { 0 }
    }

    // Java: Prm0 getPrm0()
    pub fn getPrm0(&mut self) -> &Prm0 {
        // Java: if (this.prm0 == null)
        if self.prm0.is_none() {
            // Java: this.prm0 = new Prm0(this.prm);
            self.prm0 = Some(Prm0::new(self.prm));
        }
        // Java: return this.prm0;
        self.prm0.as_ref().unwrap()
    }

    // Java: Prm1 getPrm1()
    pub fn getPrm1(&mut self) -> &Prm1 {
        // Java: if (this.prm1 == null)
        if self.prm1.is_none() {
            // Java: this.prm1 = new Prm1(this.prm);
            self.prm1 = Some(Prm1::new(self.prm));
        }
        // Java: return this.prm1;
        self.prm1.as_ref().unwrap()
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
        // Java: int tempFc = this.fc;
        let mut tempFc = self.fc;
        // Java: if (!this.unicode)
        if !self.unicode {
            // Java: tempFc *= 2;
            tempFc *= 2;
            // Java: tempFc |= 1073741824;
            tempFc |= 1073741824;
        }
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: byte[] buf = new byte[8];
        let mut buf = vec![0u8; 8];
        // Java: LittleEndian.putShort(buf, offset, this.descriptor);
        le_put_short(&mut buf, offset, self.descriptor);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putInt(buf, offset, tempFc);
        le_put_int(&mut buf, offset, tempFc);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putShort(buf, offset, this.prm);
        le_put_short(&mut buf, offset, self.prm);
        // Java: return buf;
        buf
    }

    // Java: static int getSizeInBytes()
    pub fn getSizeInBytes() -> i32 {
        // Java: return 8;
        8
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const _ as usize;
        ptr as i32
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&PieceDescriptor>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: PieceDescriptor pd = (PieceDescriptor)o;
            let pd = o.unwrap();
            // Java: return this.descriptor == pd.descriptor && this.prm == pd.prm && this.unicode == pd.unicode;
            return self.descriptor == pd.descriptor && self.prm == pd.prm && self.unicode == pd.unicode;
        }
    }
}

impl Prm0 {
    // Java: Prm0(short value)
    pub fn new(value: i16) -> Self {
        // Java: super();
        // Java: if ((value & 1) != 0)
        if (value & 1) != 0 {
            // Java: throw new IllegalStateException("This is not Prm0 Struct");
            panic!("This is not Prm0 Struct");
        }
        // Java: else
        // Java: this.isprm = (byte)(value >> 1 & 127);
        let isprm = ((value >> 1) & 127) as i8;
        // Java: this.val = (byte)(value >> 8 & 255);
        let val = ((value >> 8) & 255) as i8;
        Prm0 { isprm, val }
    }
}

impl Prm1 {
    // Java: Prm1(short value)
    pub fn new(value: i16) -> Self {
        // Java: super();
        // Java: if ((value & 1) != 1)
        if (value & 1) != 1 {
            // Java: throw new IllegalStateException("This is not Prm0 Struct");
            panic!("This is not Prm0 Struct");
        }
        // Java: else
        // Java: this.data = (short)(value >> 1 & 127);
        let data = ((value >> 1) & 127) as i16;
        Prm1 { data }
    }

    // Java: short getData()
    pub fn getData(&self) -> i16 {
        // Java: return this.data;
        self.data
    }

    // Java: int getIGrpprl()
    pub fn getIGrpprl(&self) -> i32 {
        // Java: return this.data & '\uffff';
        (self.data as i32) & 0xFFFF
    }
}

impl Clone for Prm0 {
    fn clone(&self) -> Self { Prm0 { isprm: self.isprm, val: self.val } }
}

impl Clone for Prm1 {
    fn clone(&self) -> Self { Prm1 { data: self.data } }
}

impl Clone for PieceDescriptor {
    fn clone(&self) -> Self {
        PieceDescriptor {
            descriptor: self.descriptor,
            fc: self.fc,
            prm: self.prm,
            prm0: self.prm0.clone(),
            prm1: self.prm1.clone(),
            unicode: self.unicode,
        }
    }
}
