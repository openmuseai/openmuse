// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = /* TODO */;
// Java field: int COLOR_AUTO
pub(crate) const COLOR_AUTO: i32 = /* TODO */;

// Java: public class ShadingDescriptorEx implements Cloneable
pub struct ShadingDescriptorEx {
    // Java field: int _cvFore
    pub(crate) _cvFore: i32,
    // Java field: int _cvBack
    pub(crate) _cvBack: i32,
    // Java field: short _ipat
    pub(crate) _ipat: i16,
}

// Java: public class ShadingDescriptorEx implements Cloneable
impl ShadingDescriptorEx {
    // Java: ShadingDescriptorEx()
    pub fn ShadingDescriptorEx(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: ShadingDescriptorEx(byte[] buf, int offset)
    pub fn ShadingDescriptorEx(&self, byte[] buf, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: int cvFore = LittleEndian.getInt(buf, offset);
      // TODO_RUST_CODE
      // Java: int cvBack = LittleEndian.getInt(buf, offset + 4);
      // TODO_RUST_CODE
      // Java: this._cvFore = reverse(cvFore);
      // TODO_RUST_CODE
      // Java: this._cvBack = reverse(cvBack);
      // TODO_RUST_CODE
      // Java: this._ipat = LittleEndian.getShort(buf, offset + 8);
      // TODO_RUST_CODE
    }

    // Java: int getCvFore()
    pub fn getCvFore(&self) -> i32 {
      // Java: return this._cvFore;
      // TODO_RUST_CODE
    }

    // Java: int getCvBack()
    pub fn getCvBack(&self) -> i32 {
      // Java: return this._cvBack;
      // TODO_RUST_CODE
    }

    // Java: short getIpat()
    pub fn getIpat(&self) -> i16 {
      // Java: return this._ipat;
      // TODO_RUST_CODE
    }

    // Java: void setCvFore(int cvFore)
    pub fn setCvFore(&self, int cvFore) {
      // Java: this._cvFore = cvFore;
      // TODO_RUST_CODE
    }

    // Java: void setCvBack(int cvBack)
    pub fn setCvBack(&self, int cvBack) {
      // Java: this._cvBack = cvBack;
      // TODO_RUST_CODE
    }

    // Java: void setIpat(short ipat)
    pub fn setIpat(&self, short ipat) {
      // Java: this._ipat = ipat;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: if (this._cvFore == -1)
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putInt(buf, offset, -16777216);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putInt(buf, offset, reverse(this._cvFore));
         // TODO_RUST_CODE
      }
      // Java: if (this._cvBack == -1)
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putInt(buf, offset + 4, -16777216);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putInt(buf, offset + 4, reverse(this._cvBack));
         // TODO_RUST_CODE
      }
      // Java: LittleEndian.putShort(buf, offset + 8, this._ipat);
      // TODO_RUST_CODE
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

    // Java: static int reverse(int x)
    pub fn reverse(&self, int x) -> i32 {
      // Java: byte[] data = int2bytes(x);
      // TODO_RUST_CODE
      // Java: int b1 = data[3] & 255;
      // TODO_RUST_CODE
      // Java: int b2 = data[2] & 255;
      // TODO_RUST_CODE
      // Java: int b3 = data[1] & 255;
      // TODO_RUST_CODE
      // Java: int b4 = data[0] & 255;
      // TODO_RUST_CODE
      // Java: return b4 == 255 && b1 == 0 && b2 == 0 && b3 == 0 ? -16777216 : (b1 << 16) + (b2 << 8) + (b3 << 0) + (b4 << 0);
      // TODO_RUST_CODE
    }

    // Java: static byte[] int2bytes(int num)
    pub fn int2bytes(&self, int num) -> byte[] {
      // Java: byte[] b = new byte[4];
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < 4; ++i)
        // TODO_RUST_CODE
      {
         // Java: b[i] = (byte)(num >>> 24 - i * 8);
         // TODO_RUST_CODE
      }
      // Java: return b;
      // TODO_RUST_CODE
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      // TODO_RUST_CODE
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, Object o) -> bool {
      // Java: ShadingDescriptorEx shdEx = (ShadingDescriptorEx)o;
      // TODO_RUST_CODE
      // Java: if (shdEx == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return this._cvFore == shdEx._cvFore && this._cvBack == shdEx._cvBack && this._ipat == shdEx._ipat;
         // TODO_RUST_CODE
      }
    }

}

impl Cloneable for ShadingDescriptorEx {
}
