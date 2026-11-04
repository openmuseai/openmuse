// Java: import org.apache.poi.util.BitField
use util::org::apache::poi::util::BitField;
// Java: import org.apache.poi.util.BitFieldFactory
use util::org::apache::poi::util::BitFieldFactory;
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java field: int SIZE
pub(crate) const SIZE: i32 = /* TODO */;
// Java field: int COLOR_AUTO
pub(crate) const COLOR_AUTO: i32 = /* TODO */;
// Java field: BitField _icoFore
static _icoFore: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _icoBack
static _icoBack: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField _ipat
static _ipat: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class ShadingDescriptor implements Cloneable
pub struct ShadingDescriptor {
    // Java field: short _info
    pub(crate) _info: i16,
}

// Java: public class ShadingDescriptor implements Cloneable
impl ShadingDescriptor {
    // Java: ShadingDescriptor()
    pub fn ShadingDescriptor(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: ShadingDescriptor(byte[] buf, int offset)
    pub fn ShadingDescriptor(&self, byte[] buf, int offset) {
      // Java: this(LittleEndian.getShort(buf, offset));
      // TODO_RUST_CODE
    }

    // Java: ShadingDescriptor(short info)
    pub fn ShadingDescriptor(&self, short info) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._info = info;
      // TODO_RUST_CODE
    }

    // Java: int getIcoFore()
    pub fn getIcoFore(&self) -> i32 {
      // Java: short icoFore = _icoFore.getShortValue(this._info);
      // TODO_RUST_CODE
      // Java: return this.getColor(icoFore);
      // TODO_RUST_CODE
    }

    // Java: int getIcoBack()
    pub fn getIcoBack(&self) -> i32 {
      // Java: short icoBack = _icoBack.getShortValue(this._info);
      // TODO_RUST_CODE
      // Java: return this.getColor(icoBack);
      // TODO_RUST_CODE
    }

    // Java: short getIpat()
    pub fn getIpat(&self) -> i16 {
      // Java: return _ipat.getShortValue(this._info);
      // TODO_RUST_CODE
    }

    // Java: short toShort()
    pub fn toShort(&self) -> i16 {
      // Java: return this._info;
      // TODO_RUST_CODE
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, byte[] buf, int offset) {
      // Java: LittleEndian.putShort(buf, offset, this._info);
      // TODO_RUST_CODE
    }

    // Java: int getColor(short ico)
    pub fn getColor(&self, short ico) -> i32 {
      // Java: switch (ico)
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: return -16777216;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: return 0;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: return 255;
            // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
            // Java: return 65535;
            // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
            // Java: return 65280;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
            // Java: return 16711935;
            // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: return 16711680;
            // TODO_RUST_CODE
         // Java: case 7:
         // TODO_RUST_CODE
            // Java: return 16776960;
            // TODO_RUST_CODE
         // Java: case 8:
         // TODO_RUST_CODE
            // Java: return 16777215;
            // TODO_RUST_CODE
         // Java: case 9:
         // TODO_RUST_CODE
            // Java: return 128;
            // TODO_RUST_CODE
         // Java: case 10:
         // TODO_RUST_CODE
            // Java: return 32896;
            // TODO_RUST_CODE
         // Java: case 11:
         // TODO_RUST_CODE
            // Java: return 32768;
            // TODO_RUST_CODE
         // Java: case 12:
         // TODO_RUST_CODE
            // Java: return 8388736;
            // TODO_RUST_CODE
         // Java: case 13:
         // TODO_RUST_CODE
            // Java: return 8388608;
            // TODO_RUST_CODE
         // Java: case 14:
         // TODO_RUST_CODE
            // Java: return 8421376;
            // TODO_RUST_CODE
         // Java: case 15:
         // TODO_RUST_CODE
            // Java: return 8421504;
            // TODO_RUST_CODE
         // Java: case 16:
         // TODO_RUST_CODE
            // Java: return 12632256;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: return -1;
            // TODO_RUST_CODE
      }
    }

    // Java: Object clone()
    pub fn clone(&self) -> Object {
      // Java: return super.clone();
      // TODO_RUST_CODE
    }

}

impl Cloneable for ShadingDescriptor {
}
