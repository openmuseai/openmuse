// Java: import org.apache.poi.util.HexDump

fn hex_byte(value: i8) -> String {
    const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
    let b = value as u8;
    let mut s = String::with_capacity(2);
    s.push(HEX_CHARS[(b >> 4) as usize] as char);
    s.push(HEX_CHARS[(b & 0x0F) as usize] as char);
    s
}

// Java: public class ClassID
#[derive(Clone)]
pub struct ClassID {
    // Java field: byte[] _bytes
    _bytes: Vec<u8>,
}

// Java field: int LENGTH
pub const LENGTH: i32 = 16;

// Java: public class ClassID
impl ClassID {
    // Java: ClassID(byte[] src, int offset)
    pub fn new_from_bytes(src: &[u8], offset: i32) -> Self {
        // Java: super();
        let off = offset as usize;
        let mut _bytes = vec![0u8; 16];
        // Java: this._bytes[0] = src[3 + offset];
        _bytes[0] = src[3 + off];
        // Java: this._bytes[1] = src[2 + offset];
        _bytes[1] = src[2 + off];
        // Java: this._bytes[2] = src[1 + offset];
        _bytes[2] = src[1 + off];
        // Java: this._bytes[3] = src[0 + offset];
        _bytes[3] = src[0 + off];
        // Java: this._bytes[4] = src[5 + offset];
        _bytes[4] = src[5 + off];
        // Java: this._bytes[5] = src[4 + offset];
        _bytes[5] = src[4 + off];
        // Java: this._bytes[6] = src[7 + offset];
        _bytes[6] = src[7 + off];
        // Java: this._bytes[7] = src[6 + offset];
        _bytes[7] = src[6 + off];
        // Java: System.arraycopy(src, 8 + offset, this._bytes, 8, 8);
        _bytes[8..16].copy_from_slice(&src[8 + off..16 + off]);
        ClassID { _bytes }
    }

    // Java: ClassID()
    pub fn new() -> Self {
        // Java: super();
        ClassID {
            _bytes: vec![0u8; 16],
        }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return 16;
        16
    }

    // Java: byte[] getBytes()
    pub fn getBytes(&self) -> &[u8] {
        // Java: return this._bytes;
        &self._bytes
    }

    // Java: byte[] getBytesLittleEndian()
    pub fn getBytesLittleEndian(&self) -> Vec<u8> {
        // Java: byte[] buff = new byte[16];
        let mut buff = vec![0u8; 16];
        // Java: this.write(buff, 0);
        self.write(&mut buff, 0);
        // Java: return buff;
        buff
    }

    // Java: void setBytes(byte[] bytes)
    pub fn setBytes(&mut self, bytes: &[u8]) {
        // Java: System.arraycopy(bytes, 0, this._bytes, 0, 16);
        self._bytes[..16].copy_from_slice(&bytes[..16]);
    }

    // Java: void write(byte[] dst, int offset)
    pub fn write(&self, dst: &mut [u8], offset: i32) {
        let off = offset as usize;
        // Java: if (dst.length < 16)
        if dst.len() < 16 {
            // Java: throw new ArrayStoreException("Destination byte[] must have room for at least 16 bytes, but has a length of only " + dst.length + ".");
            panic!("Destination byte[] must have room for at least 16 bytes, but has a length of only {}.", dst.len());
        }
        // Java: else
        else {
            // Java: dst[0 + offset] = this._bytes[3];
            dst[0 + off] = self._bytes[3];
            // Java: dst[1 + offset] = this._bytes[2];
            dst[1 + off] = self._bytes[2];
            // Java: dst[2 + offset] = this._bytes[1];
            dst[2 + off] = self._bytes[1];
            // Java: dst[3 + offset] = this._bytes[0];
            dst[3 + off] = self._bytes[0];
            // Java: dst[4 + offset] = this._bytes[5];
            dst[4 + off] = self._bytes[5];
            // Java: dst[5 + offset] = this._bytes[4];
            dst[5 + off] = self._bytes[4];
            // Java: dst[6 + offset] = this._bytes[7];
            dst[6 + off] = self._bytes[7];
            // Java: dst[7 + offset] = this._bytes[6];
            dst[7 + off] = self._bytes[6];
            // Java: for(int i = 8; i < 16; ++i)
            for i in 8..16 {
                // Java: dst[i + offset] = this._bytes[i];
                dst[i + off] = self._bytes[i];
            }
            // Java: System.arraycopy(this._bytes, 8, dst, 8 + offset, 8);
            dst[8 + off..16 + off].copy_from_slice(&self._bytes[8..16]);
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&ClassID>) -> bool {
        // Java: if (this == o)
        if let Some(other) = o {
            if std::ptr::eq(self, other) {
                // Java: return true;
                return true;
            }
        }
        // Java: else if (!(o instanceof ClassID))
        else {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: byte[] bytes = ((ClassID)o)._bytes;
            let bytes = &o.unwrap()._bytes;
            // Java: for(int i = 0; i < 16; ++i)
            for i in 0..16 {
                // Java: if (bytes[i] != this._bytes[i])
                if bytes[i] != self._bytes[i] {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            return true;
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return Arrays.hashCode(this._bytes);
        let mut result: i32 = 1;
        for &b in &self._bytes {
            result = result.wrapping_mul(31).wrapping_add(b as i8 as i32);
        }
        result
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer sbClassId = new StringBuffer(38);
        let mut sb = String::with_capacity(38);
        // Java: sbClassId.append('{');
        sb.push('{');
        // Java: for(int i = 0; i < 16; ++i)
        for i in 0..16usize {
            // Java: sbClassId.append(HexDump.toHex(this._bytes[i]));
            sb.push_str(&hex_byte(self._bytes[i] as i8));
            // Java: if (i == 3 || i == 5 || i == 7 || i == 9)
            if i == 3 || i == 5 || i == 7 || i == 9 {
                // Java: sbClassId.append('-');
                sb.push('-');
            }
        }
        // Java: sbClassId.append('}');
        sb.push('}');
        // Java: return sbClassId.toString();
        sb
    }

    // Java: static byte[] toBytes(String guid)
    pub fn toBytes(guid: &str) -> Vec<u8> {
        // Java: byte[] id = new byte[16];
        let mut id = vec![0u8; 16];
        // Java: String tmp = null;
        // Java: int i = 1;
        let mut i: usize = 1;
        // Java: int j = 0;
        let mut j: usize = 0;
        // Java: int len = guid.length();
        let len = guid.len();
        // Java: while(i < len - 1 && j < 16)
        while i < len - 1 && j < 16 {
            // Java: if (i != 9 && i != 14 && i != 19 && i != 24)
            if i != 9 && i != 14 && i != 19 && i != 24 {
                // Java: int var10001 = i;
                let var10001 = i;
                // Java: i += 2;
                i += 2;
                // Java: tmp = guid.substring(var10001, i);
                let tmp = &guid[var10001..i];
                // Java: id[j] = (byte)Integer.parseInt(tmp, 16);
                id[j] = u8::from_str_radix(tmp, 16).unwrap();
                // Java: ++j;
                j += 1;
            }
            // Java: else
            else {
                // Java: ++i;
                i += 1;
            }
        }
        // Java: return id;
        id
    }

    // Java: static byte[] toBytes2(String guid)
    pub fn toBytes2(guid: &str) -> Vec<u8> {
        // Java: ClassID classID = new ClassID();
        let mut classID = ClassID::new();
        // Java: byte[] dstClassID = new byte[16];
        let mut dstClassID = vec![0u8; 16];
        // Java: classID.setBytes(toBytes(guid));
        let bytes = ClassID::toBytes(guid);
        classID.setBytes(&bytes);
        // Java: classID.write(dstClassID, 0);
        classID.write(&mut dstClassID, 0);
        // Java: return dstClassID;
        dstClassID
    }
}
