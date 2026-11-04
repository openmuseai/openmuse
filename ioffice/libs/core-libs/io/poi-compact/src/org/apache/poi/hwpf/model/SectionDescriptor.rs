// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
    data[offset + 2] = bytes[2];
    data[offset + 3] = bytes[3];
}

// Java: public class SectionDescriptor
pub struct SectionDescriptor {
    // Java field: short fn
    _fn: i16,
    // Java field: int fc
    fc: i32,
    // Java field: short fnMpr
    fnMpr: i16,
    // Java field: int fcMpr
    fcMpr: i32,
}

// Java: public class SectionDescriptor
impl SectionDescriptor {
    // Java: SectionDescriptor()
    pub fn new() -> Self {
        // Java: super();
        SectionDescriptor {
            _fn: 0,
            fc: 0,
            fnMpr: 0,
            fcMpr: 0,
        }
    }

    // Java: SectionDescriptor(byte[] buf, int offset)
    pub fn new_from_buf(buf: &[u8], mut offset: i32) -> Self {
        // Java: super();
        // Java: this.fn = LittleEndian.getShort(buf, offset);
        let _fn = le_get_short(buf, offset as usize);
        // Java: offset += 2;
        offset += 2;
        // Java: this.fc = LittleEndian.getInt(buf, offset);
        let fc = le_get_int(buf, offset as usize);
        // Java: offset += 4;
        offset += 4;
        // Java: this.fnMpr = LittleEndian.getShort(buf, offset);
        let fnMpr = le_get_short(buf, offset as usize);
        // Java: offset += 2;
        offset += 2;
        // Java: this.fcMpr = LittleEndian.getInt(buf, offset);
        let fcMpr = le_get_int(buf, offset as usize);
        SectionDescriptor { _fn, fc, fnMpr, fcMpr }
    }

    // Java: int getFc()
    pub fn getFc(&self) -> i32 {
        // Java: return this.fc;
        self.fc
    }

    // Java: void setFc(int fc)
    pub fn setFc(&mut self, fc: i32) {
        // Java: this.fc = fc;
        self.fc = fc;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const Self as usize;
        ptr as i32
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&SectionDescriptor>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: SectionDescriptor sed = (SectionDescriptor)o;
            let sed = o.unwrap();
            // Java: return sed.fn == this.fn && sed.fnMpr == this.fnMpr;
            return sed._fn == self._fn && sed.fnMpr == self.fnMpr;
        }
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: byte[] buf = new byte[12];
        let mut buf = vec![0u8; 12];
        // Java: LittleEndian.putShort(buf, offset, this.fn);
        le_put_short(&mut buf, offset, self._fn);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putInt(buf, offset, this.fc);
        le_put_int(&mut buf, offset, self.fc);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putShort(buf, offset, this.fnMpr);
        le_put_short(&mut buf, offset, self.fnMpr);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putInt(buf, offset, this.fcMpr);
        le_put_int(&mut buf, offset, self.fcMpr);
        // Java: return buf;
        buf
    }
}
